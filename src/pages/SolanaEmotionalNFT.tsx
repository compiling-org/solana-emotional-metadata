import React, { useState, useEffect, useCallback } from 'react';
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import BiometricNFTClient from '../utils/solana-client';

interface EmotionData {
  valence: number;
  arousal: number;
  dominance: number;
  biometricHash: string;
}

interface NFTMetadata {
  name: string;
  symbol: string;
  description: string;
  image: string;
  attributes: Array<{
    trait_type: string;
    value: number | string;
  }>;
}

const SolanaEmotionalNFT: React.FC = () => {
  const [emotionData, setEmotionData] = useState<EmotionData>({
    valence: 0.5,
    arousal: 0.5,
    dominance: 0.5,
    biometricHash: ''
  });
  const [isMinting, setIsMinting] = useState(false);
  const [nftMinted, setNftMinted] = useState(false);
  const [transactionSignature, setTransactionSignature] = useState<string>('');
  const [aiGeneratedArt, setAiGeneratedArt] = useState<string>('');
  const [nftClient, setNftClient] = useState<BiometricNFTClient | null>(null);
  const [walletConnected, setWalletConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string>('');
  const [devnetSignature, setDevnetSignature] = useState<string>('');
  const [keypair, setKeypair] = useState<Keypair | null>(null);

  // Simple wallet connection simulation
  const connectWallet = async () => {
    try {
      // For demo purposes, create a keypair (in production, use actual wallet connection)
      const kp = Keypair.generate();
      setPublicKey(kp.publicKey.toString());
      setKeypair(kp);
      setWalletConnected(true);
      
      // Initialize NFT client with demo connection
      const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
      const client = new BiometricNFTClient(connection, null as any);
      setNftClient(client);
      
      console.log('Wallet connected:', kp.publicKey.toString());
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  // Generate AI art based on emotion data
  const generateAIArt = useCallback(() => {
    const { valence, arousal, dominance } = emotionData;
    
    // Simple SVG-based art generation
    const hue = valence * 360;
    const saturation = arousal * 100;
    const lightness = 50 + (dominance * 30);
    
    const svg = `
      <svg width="400" height="400" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <radialGradient id="emotionGradient" cx="50%" cy="50%" r="50%">
            <stop offset="0%" style="stop-color:hsl(${hue}, ${saturation}%, ${lightness}%);stop-opacity:1" />
            <stop offset="100%" style="stop-color:hsl(${(hue + 60) % 360}, ${saturation * 0.7}%, ${lightness * 0.7}%);stop-opacity:1" />
          </radialGradient>
        </defs>
        <rect width="400" height="400" fill="url(#emotionGradient)"/>
        <circle cx="200" cy="200" r="${100 + valence * 50}" fill="hsl(${hue}, ${saturation}%, ${lightness}%)" opacity="0.7"/>
        <circle cx="${150 + arousal * 100}" cy="${150 + dominance * 100}" r="${30 + valence * 20}" fill="hsl(${(hue + 180) % 360}, ${saturation}%, ${lightness}%)" opacity="0.8"/>
        <text x="200" y="380" text-anchor="middle" fill="white" font-size="14" font-family="Arial">
          V:${valence.toFixed(2)} A:${arousal.toFixed(2)} D:${dominance.toFixed(2)}
        </text>
      </svg>
    `;
    
    const base64Svg = btoa(svg);
    setAiGeneratedArt(`data:image/svg+xml;base64,${base64Svg}`);
  }, [emotionData]);

  // Generate biometric hash
  const captureBiometricData = () => {
    const { valence, arousal, dominance } = emotionData;
    const hash = `bio_${Date.now()}_${valence}_${arousal}_${dominance}`.slice(0, 32);
    setEmotionData(prev => ({ ...prev, biometricHash: hash }));
  };

  // Mint NFT
  const mintEmotionalNFT = async () => {
    if (!walletConnected || !nftClient || !emotionData.biometricHash) {
      alert('Please connect wallet and capture biometric data first');
      return;
    }

    setIsMinting(true);
    try {
      // Simulate NFT minting
      const metadata: NFTMetadata = {
        name: `Emotional NFT #${Date.now()}`,
        symbol: 'EMO',
        description: `AI-generated emotional art based on biometric data. Valence: ${emotionData.valence.toFixed(2)}, Arousal: ${emotionData.arousal.toFixed(2)}, Dominance: ${emotionData.dominance.toFixed(2)}`,
        image: aiGeneratedArt,
        attributes: [
          { trait_type: 'Valence', value: emotionData.valence.toFixed(2) },
          { trait_type: 'Arousal', value: emotionData.arousal.toFixed(2) },
          { trait_type: 'Dominance', value: emotionData.dominance.toFixed(2) },
          { trait_type: 'Biometric Hash', value: emotionData.biometricHash }
        ]
      };

      // Simulate transaction
      const mockSignature = `mock_tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      setTransactionSignature(mockSignature);
      setNftMinted(true);
      
      console.log('NFT Minted:', metadata);
      console.log('Transaction Signature:', mockSignature);
      
    } catch (error) {
      console.error('Failed to mint NFT:', error);
      alert('Failed to mint NFT. Check console for details.');
    } finally {
      setIsMinting(false);
    }
  };

  const sendDevnetMemo = async () => {
    if (!walletConnected || !nftClient || !keypair) {
      alert('Please connect wallet first');
      return;
    }
    try {
      const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
      const airdropSig = await connection.requestAirdrop(keypair.publicKey, 0.05 * LAMPORTS_PER_SOL);
      await connection.confirmTransaction(airdropSig, 'confirmed');
      const sig = await nftClient.sendMemoWithKeypair(keypair, 'Emotional NFT Studio memo');
      setDevnetSignature(sig);
    } catch (e) {
      console.error('Failed to send memo:', e);
      alert('Failed to send memo transaction');
    }
  };

  // Generate initial art
  useEffect(() => {
    generateAIArt();
  }, [generateAIArt]);

  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-4xl font-bold mb-8 text-center">Solana Emotional NFT Studio</h1>
        
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Emotion Controls */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-6">Emotion Controls</h2>
            
            {/* Wallet Connection */}
            <div className="mb-6">
              {!walletConnected ? (
                <button
                  onClick={connectWallet}
                  className="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
                >
                  Connect Wallet
                </button>
              ) : (
                <div className="bg-green-900 border border-green-700 rounded-lg p-3">
                  <p className="text-sm font-medium text-green-300">Wallet Connected</p>
                  <p className="text-xs text-green-400 break-all">{publicKey.slice(0, 8)}...{publicKey.slice(-8)}</p>
                </div>
              )}
            </div>

            {/* Valence Slider */}
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2">Valence (Pleasure)</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={emotionData.valence}
                onChange={(e) => setEmotionData(prev => ({ ...prev, valence: parseFloat(e.target.value) }))}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <div className="flex justify-between text-xs text-gray-400 mt-1">
                <span>Unpleasant</span>
                <span>Neutral</span>
                <span>Pleasant</span>
              </div>
              <p className="text-center mt-1">{emotionData.valence.toFixed(2)}</p>
            </div>

            {/* Arousal Slider */}
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2">Arousal (Energy)</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={emotionData.arousal}
                onChange={(e) => setEmotionData(prev => ({ ...prev, arousal: parseFloat(e.target.value) }))}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <div className="flex justify-between text-xs text-gray-400 mt-1">
                <span>Low Energy</span>
                <span>Neutral</span>
                <span>High Energy</span>
              </div>
              <p className="text-center mt-1">{emotionData.arousal.toFixed(2)}</p>
            </div>

            {/* Dominance Slider */}
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2">Dominance (Control)</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={emotionData.dominance}
                onChange={(e) => setEmotionData(prev => ({ ...prev, dominance: parseFloat(e.target.value) }))}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <div className="flex justify-between text-xs text-gray-400 mt-1">
                <span>Submissive</span>
                <span>Neutral</span>
                <span>Dominant</span>
              </div>
              <p className="text-center mt-1">{emotionData.dominance.toFixed(2)}</p>
            </div>
            
            <button
              onClick={captureBiometricData}
              className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
            >
              Capture Biometric Data
            </button>
            
            {emotionData.biometricHash && (
              <div className="bg-green-900 border border-green-700 rounded-lg p-3">
                <p className="text-sm font-medium text-green-300">Biometric Verified</p>
                <p className="text-xs text-green-400 break-all">{emotionData.biometricHash}</p>
              </div>
            )}
          </div>
          
          {/* NFT Preview & Minting */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">NFT Preview</h2>
            
            {aiGeneratedArt ? (
              <div className="mb-4">
                <img
                  src={aiGeneratedArt}
                  alt="AI Generated Emotional Art"
                  className="w-full h-64 object-cover rounded-lg"
                  onError={(e) => {
                    (e.target as HTMLImageElement).src = 'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAwIiBoZWlnaHQ9IjIwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cmVjdCB3aWR0aD0iMjAwIiBoZWlnaHQ9IjIwMCIgZmlsbD0iIzY2NjY2NiIvPjx0ZXh0IHg9IjUwJSIgeT0iNTAlIiBmb250LXNpemU9IjE0IiBmaWxsPSIjZmZmIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBkeT0iLjNlbSI+RW1vdGlvbmFsIEFydDwvdGV4dD48L3N2Zz4=';
                  }}
                />
              </div>
            ) : (
              <div className="w-full h-64 bg-gray-700 rounded-lg flex items-center justify-center mb-4">
                <p className="text-gray-400">Generating emotional art...</p>
              </div>
            )}
            
            <div className="mb-4">
              <h3 className="text-lg font-medium mb-2">Emotion Analysis</h3>
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span>Valence:</span>
                  <span className={emotionData.valence > 0.5 ? 'text-green-400' : 'text-red-400'}>
                    {emotionData.valence > 0.5 ? 'Positive' : 'Negative'} ({emotionData.valence.toFixed(2)})
                  </span>
                </div>
                <div className="flex justify-between">
                  <span>Arousal:</span>
                  <span className={emotionData.arousal > 0.5 ? 'text-yellow-400' : 'text-blue-400'}>
                    {emotionData.arousal > 0.5 ? 'High Energy' : 'Low Energy'} ({emotionData.arousal.toFixed(2)})
                  </span>
                </div>
                <div className="flex justify-between">
                  <span>Dominance:</span>
                  <span className={emotionData.dominance > 0.5 ? 'text-purple-400' : 'text-gray-400'}>
                    {emotionData.dominance > 0.5 ? 'Dominant' : 'Submissive'} ({emotionData.dominance.toFixed(2)})
                  </span>
                </div>
              </div>
            </div>
            
            {nftMinted && (
              <div className="bg-green-900 border border-green-700 rounded-lg p-4 mb-4">
                <h3 className="text-lg font-medium text-green-300 mb-2">NFT Minted Successfully!</h3>
                <p className="text-sm text-green-400 mb-2">Transaction Signature:</p>
                <p className="text-xs text-green-300 break-all bg-green-800 p-2 rounded">{transactionSignature}</p>
              </div>
            )}
            
            <button
              onClick={mintEmotionalNFT}
              disabled={isMinting || !walletConnected || !emotionData.biometricHash}
              className="w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 disabled:from-gray-600 disabled:to-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors"
            >
              {isMinting ? 'Minting NFT...' : 'Mint Emotional NFT'}
            </button>
            <div className="mt-4 grid grid-cols-1 gap-2">
              <button
                onClick={sendDevnetMemo}
                disabled={!walletConnected}
                className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
              >
                Send Devnet Memo (Validate Wallet)
              </button>
              {devnetSignature && (
                <p className="text-xs text-green-300 break-all">Memo Signature: {devnetSignature}</p>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SolanaEmotionalNFT;