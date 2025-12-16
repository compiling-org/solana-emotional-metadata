import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Alert, AlertDescription } from './ui/alert';

/**
 * Cross-Chain Bridge Component
 * Provides UI for transferring biometric NFTs between Filecoin and Solana
 */

interface CrossChainBridgeProps {
  className?: string;
}

interface TransferResult {
  success: boolean;
  transferId?: string;
  filecoinTokenId?: string;
  solanaTokenId?: string;
  error?: string;
  direction: string;
}

interface BridgeStatistics {
  completedTransfers: number;
  failedTransfers: number;
  pendingTransfers: number;
  totalTransfers: number;
}

const CrossChainBridge: React.FC<CrossChainBridgeProps> = ({ className = '' }) => {
  const [bridge, setBridge] = useState<any>(null);
  const [isInitializing, setIsInitializing] = useState(false);
  const [isTransferring, setIsTransferring] = useState(false);
  const [bridgeInitialized, setBridgeInitialized] = useState(false);
  const [statistics, setStatistics] = useState<BridgeStatistics>({
    completedTransfers: 0,
    failedTransfers: 0,
    pendingTransfers: 0,
    totalTransfers: 0
  });
  const [transferHistory, setTransferHistory] = useState<any[]>([]);
  const [lastTransfer, setLastTransfer] = useState<TransferResult | null>(null);
  const [error, setError] = useState<string>('');

  // Initialize bridge on component mount
  useEffect(() => {
    initializeBridge();
  }, []);

  // Update statistics periodically
  useEffect(() => {
    if (bridgeInitialized && bridge) {
      const interval = setInterval(() => {
        updateBridgeStatistics();
      }, 30000); // Update every 30 seconds

      return () => clearInterval(interval);
    }
  }, [bridgeInitialized, bridge]);

  const initializeBridge = async () => {
    setIsInitializing(true);
    setError('');

    try {
      // Dynamically import the bridge to avoid SSR issues
      const CrossChainBridge = require('../utils/cross-chain-bridge');
      const { config } = require('../config/cross-chain-bridge-config');

      const bridgeInstance = new CrossChainBridge(config);
      const initSuccess = await bridgeInstance.initialize();

      if (initSuccess) {
        setBridge(bridgeInstance);
        setBridgeInitialized(true);
        updateBridgeStatistics(bridgeInstance);
      } else {
        throw new Error('Failed to initialize bridge');
      }
    } catch (err) {
      console.error('Bridge initialization failed:', err);
      setError(`Failed to initialize cross-chain bridge: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setIsInitializing(false);
    }
  };

  const updateBridgeStatistics = (bridgeInstance = bridge) => {
    if (bridgeInstance) {
      const stats = bridgeInstance.getBridgeStatistics();
      const history = bridgeInstance.getTransferHistory();
      
      setStatistics(stats);
      setTransferHistory(history);
    }
  };

  const transferFromFilecoinToSolana = async () => {
    if (!bridge) {
      setError('Bridge not initialized');
      return;
    }

    setIsTransferring(true);
    setError('');

    try {
      // For demo purposes, we'll use a mock Filecoin token ID
      // In a real implementation, this would come from user input or wallet
      const mockFilecoinTokenId = `demo_filecoin_nft_${Date.now()}`;
      const solanaRecipient = 'DemoSolanaWallet123456789';

      const result = await bridge.transferFromFilecoinToSolana(
        mockFilecoinTokenId,
        solanaRecipient
      );

      setLastTransfer({
        ...result,
        direction: 'filecoin-to-solana'
      });

      if (result.success) {
        updateBridgeStatistics();
      }
    } catch (err) {
      console.error('Filecoin to Solana transfer failed:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Transfer failed: ${errorMessage}`);
      setLastTransfer({
        success: false,
        error: errorMessage,
        direction: 'filecoin-to-solana'
      });
    } finally {
      setIsTransferring(false);
    }
  };

  const transferFromSolanaToFilecoin = async () => {
    if (!bridge) {
      setError('Bridge not initialized');
      return;
    }

    setIsTransferring(true);
    setError('');

    try {
      // For demo purposes, we'll use a mock Solana token account
      // In a real implementation, this would come from user input or wallet
      const mockSolanaTokenAccount = 'DemoSolanaTokenAccount123456789';
      const filecoinRecipient = 'f1demo123456789abcdefghijklmnopqrstuvwxyz';

      const result = await bridge.transferFromSolanaToFilecoin(
        mockSolanaTokenAccount,
        filecoinRecipient
      );

      setLastTransfer({
        ...result,
        direction: 'solana-to-filecoin'
      });

      if (result.success) {
        updateBridgeStatistics();
      }
    } catch (err) {
      console.error('Solana to Filecoin transfer failed:', err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Transfer failed: ${errorMessage}`);
      setLastTransfer({
        success: false,
        error: errorMessage,
        direction: 'solana-to-filecoin'
      });
    } finally {
      setIsTransferring(false);
    }
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'completed':
        return <Badge variant="default" className="bg-green-500">Completed</Badge>;
      case 'failed':
        return <Badge variant="destructive">Failed</Badge>;
      case 'pending':
        return <Badge variant="outline">Pending</Badge>;
      default:
        return <Badge variant="outline">Unknown</Badge>;
    }
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString();
  };

  return (
    <div className={`space-y-6 ${className}`}>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">Cross-Chain Bridge</CardTitle>
          <CardDescription>
            Transfer biometric NFTs between Filecoin and Solana blockchains
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <h3 className="text-lg font-semibold">Bridge Status</h3>
              <div className="space-y-1">
                <div className="flex justify-between">
                  <span>Status:</span>
                  <Badge variant={bridgeInitialized ? "default" : "destructive"}>
                    {bridgeInitialized ? "Connected" : "Disconnected"}
                  </Badge>
                </div>
                <div className="flex justify-between">
                  <span>Total Transfers:</span>
                  <span className="font-mono">{statistics.totalTransfers}</span>
                </div>
                <div className="flex justify-between">
                  <span>Completed:</span>
                  <span className="font-mono text-green-600">{statistics.completedTransfers}</span>
                </div>
                <div className="flex justify-between">
                  <span>Failed:</span>
                  <span className="font-mono text-red-600">{statistics.failedTransfers}</span>
                </div>
                <div className="flex justify-between">
                  <span>Pending:</span>
                  <span className="font-mono text-yellow-600">{statistics.pendingTransfers}</span>
                </div>
              </div>
            </div>

            <div className="space-y-2">
              <h3 className="text-lg font-semibold">Transfer Actions</h3>
              <div className="space-y-2">
                <Button
                  onClick={transferFromFilecoinToSolana}
                  disabled={!bridgeInitialized || isTransferring}
                  className="w-full"
                  variant="outline"
                >
                  {isTransferring && lastTransfer?.direction === 'filecoin-to-solana' ? (
                    "Transferring..."
                  ) : (
                    "Filecoin → Solana"
                  )}
                </Button>
                <Button
                  onClick={transferFromSolanaToFilecoin}
                  disabled={!bridgeInitialized || isTransferring}
                  className="w-full"
                  variant="outline"
                >
                  {isTransferring && lastTransfer?.direction === 'solana-to-filecoin' ? (
                    "Transferring..."
                  ) : (
                    "Solana → Filecoin"
                  )}
                </Button>
              </div>
            </div>
          </div>

          {lastTransfer && (
            <div className="border rounded-lg p-4 bg-muted/50">
              <h4 className="font-semibold mb-2">Last Transfer</h4>
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="text-muted-foreground">Direction:</span>
                  <span className="ml-2 font-mono">{lastTransfer.direction}</span>
                </div>
                <div>
                  <span className="text-muted-foreground">Status:</span>
                  <span className="ml-2">
                    {lastTransfer.success ? (
                      <Badge variant="default" className="bg-green-500">Success</Badge>
                    ) : (
                      <Badge variant="destructive">Failed</Badge>
                    )}
                  </span>
                </div>
                {lastTransfer.transferId && (
                  <div className="col-span-2">
                    <span className="text-muted-foreground">Transfer ID:</span>
                    <span className="ml-2 font-mono text-xs">{lastTransfer.transferId}</span>
                  </div>
                )}
                {lastTransfer.error && (
                  <div className="col-span-2">
                    <span className="text-muted-foreground">Error:</span>
                    <span className="ml-2 text-red-500">{lastTransfer.error}</span>
                  </div>
                )}
              </div>
            </div>
          )}

          {transferHistory.length > 0 && (
            <div className="border rounded-lg p-4">
              <h4 className="font-semibold mb-2">Recent Transfers</h4>
              <div className="space-y-2 max-h-48 overflow-y-auto">
                {transferHistory.slice(0, 5).map((transfer, index) => (
                  <div key={index} className="flex justify-between items-center py-2 border-b last:border-b-0">
                    <div className="flex items-center space-x-2">
                      {getStatusBadge(transfer.status)}
                      <span className="text-sm font-mono">
                        {transfer.transferId?.substring(0, 16)}...
                      </span>
                    </div>
                    <span className="text-xs text-muted-foreground">
                      {formatTimestamp(transfer.timestamp)}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {!bridgeInitialized && !isInitializing && (
            <div className="text-center py-4">
              <Button onClick={initializeBridge}>
                Initialize Bridge
              </Button>
            </div>
          )}

          {isInitializing && (
            <div className="text-center py-4">
              <div className="animate-pulse">Initializing cross-chain bridge...</div>
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Bridge Information</CardTitle>
          <CardDescription>
            Technical details about the cross-chain bridge implementation
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
            <div>
              <h5 className="font-semibold mb-2">Supported Networks</h5>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Filecoin (Calibration/Mainnet)</li>
                <li>• Solana (Devnet/Testnet/Mainnet)</li>
              </ul>
            </div>
            <div>
              <h5 className="font-semibold mb-2">Transfer Features</h5>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Biometric NFT data transfer</li>
                <li>• Emotion data conversion</li>
                <li>• Quality score validation</li>
                <li>• Cross-chain verification</li>
              </ul>
            </div>
            <div>
              <h5 className="font-semibold mb-2">Security Features</h5>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Transfer verification</li>
                <li>• Quality score validation (min 0.7)</li>
                <li>• Transfer history tracking</li>
                <li>• Error handling and retry logic</li>
              </ul>
            </div>
            <div>
              <h5 className="font-semibold mb-2">Data Conversion</h5>
              <ul className="space-y-1 text-muted-foreground">
                <li>• Filecoin: Single emotion score → Solana: 6D vector</li>
                <li>• Solana: 6D emotion vector → Filecoin: Single score</li>
                <li>• Biometric hash preservation</li>
                <li>• Cross-chain ID mapping</li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default CrossChainBridge;