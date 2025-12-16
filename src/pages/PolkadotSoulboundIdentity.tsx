import React, { useState, useEffect } from 'react';
import { KeyringPair } from '@polkadot/keyring/types';
import { PolkadotSoulboundClient, EmotionData, Identity, emotionToString } from '../utils/polkadot-client';
import { generateBiometricHash } from '../utils/biometric-utils';

interface PolkadotSoulboundIdentityProps {
  contractAddress?: string;
}

const PolkadotSoulboundIdentity: React.FC<PolkadotSoulboundIdentityProps> = ({ 
  contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' 
}) => {
  const [client, setClient] = useState<PolkadotSoulboundClient | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [keypair, setKeypair] = useState<KeyringPair | null>(null);
  const [identity, setIdentity] = useState<Identity | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Form states
  const [identityName, setIdentityName] = useState('');
  const [emotionData, setEmotionData] = useState<EmotionData>({
    valence: 0.5,
    arousal: 0.5,
    dominance: 0.5,
    confidence: 85,
    timestamp: Date.now()
  });
  const [metadataUri, setMetadataUri] = useState('');
  const [biometricData, setBiometricData] = useState('');

  useEffect(() => {
    const initClient = async () => {
      try {
        const soulboundClient = new PolkadotSoulboundClient(contractAddress);
        await soulboundClient.connect();
        setClient(soulboundClient);
        setIsConnected(true);
      } catch (err) {
        setError('Failed to connect to Polkadot network');
        console.error('Connection error:', err);
      }
    };

    initClient();

    return () => {
      if (client) {
        client.disconnect();
      }
    };
  }, [contractAddress]);

  const handleCreateIdentity = async () => {
    if (!client || !keypair || !identityName || !metadataUri) {
      setError('Please fill in all required fields and connect wallet');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      // Generate biometric hash from input data
      const biometricHash = await generateBiometricHash(biometricData || 'default-biometric-data');
      
      const result = await client.createIdentity(
        keypair,
        identityName,
        biometricHash,
        emotionData,
        metadataUri
      );

      // Fetch the newly created identity
      const newIdentity = await client.getIdentity(result.identityId);
      setIdentity(newIdentity);
      
      alert(`Identity created successfully! ID: ${result.identityId}, Transaction: ${result.transactionHash}`);
    } catch (err) {
      setError(`Failed to create identity: ${err instanceof Error ? err.message : 'Unknown error'}`);
      console.error('Create identity error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleConnectWallet = async () => {
    try {
      // In a real implementation, this would connect to a Polkadot wallet extension
      // For now, we'll create a mock keypair
      const { Keyring } = await import('@polkadot/keyring');
      const { cryptoWaitReady } = await import('@polkadot/util-crypto');
      
      await cryptoWaitReady();
      const keyring = new Keyring({ type: 'sr25519' });
      const mockKeypair = keyring.addFromUri('//Alice'); // In production, use actual wallet
      
      setKeypair(mockKeypair);
      setError(null);
    } catch (err) {
      setError('Failed to connect wallet');
      console.error('Wallet connection error:', err);
    }
  };

  const handleEmotionChange = (type: keyof Omit<EmotionData, 'timestamp' | 'confidence'>, value: number) => {
    setEmotionData(prev => ({
      ...prev,
      [type]: value,
      timestamp: Date.now()
    }));
  };

  const handleBiometricDataChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setBiometricData(e.target.value);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 p-6">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-2">
            Polkadot Soulbound Identity
          </h1>
          <p className="text-blue-200">
            Create your unique, non-transferable digital identity with biometric verification
          </p>
        </div>

        {/* Connection Status */}
        <div className="bg-black bg-opacity-30 rounded-lg p-4 mb-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className={`w-3 h-3 rounded-full ${isConnected ? 'bg-green-400' : 'bg-red-400'}`} />
              <span className="text-white">
                {isConnected ? 'Connected to Polkadot' : 'Connecting to Polkadot...'}
              </span>
            </div>
            <button
              onClick={handleConnectWallet}
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors"
            >
              {keypair ? 'Wallet Connected' : 'Connect Wallet'}
            </button>
          </div>
        </div>

        {error && (
          <div className="bg-red-600 bg-opacity-20 border border-red-500 rounded-lg p-4 mb-6">
            <p className="text-red-200">{error}</p>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Identity Creation Form */}
          <div className="bg-black bg-opacity-30 rounded-lg p-6">
            <h2 className="text-2xl font-semibold text-white mb-4">
              Create Soulbound Identity
            </h2>
            
            <div className="space-y-4">
              <div>
                <label className="block text-blue-200 text-sm font-medium mb-2">
                  Identity Name
                </label>
                <input
                  type="text"
                  value={identityName}
                  onChange={(e) => setIdentityName(e.target.value)}
                  className="w-full bg-gray-800 border border-gray-600 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter your identity name"
                />
              </div>

              <div>
                <label className="block text-blue-200 text-sm font-medium mb-2">
                  Biometric Data
                </label>
                <textarea
                  value={biometricData}
                  onChange={handleBiometricDataChange}
                  className="w-full bg-gray-800 border border-gray-600 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter biometric data (EEG, heart rate, etc.)"
                  rows={3}
                />
              </div>

              <div>
                <label className="block text-blue-200 text-sm font-medium mb-2">
                  Metadata URI (IPFS)
                </label>
                <input
                  type="text"
                  value={metadataUri}
                  onChange={(e) => setMetadataUri(e.target.value)}
                  className="w-full bg-gray-800 border border-gray-600 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="ipfs://QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco"
                />
              </div>

              {/* Emotion Data Controls */}
              <div className="space-y-3">
                <h3 className="text-lg font-medium text-white">Emotional Parameters</h3>
                
                <div>
                  <label className="block text-blue-200 text-sm font-medium mb-1">
                    Valence (Positive/Negative): {emotionData.valence.toFixed(2)}
                  </label>
                  <input
                    type="range"
                    min="-1"
                    max="1"
                    step="0.01"
                    value={emotionData.valence}
                    onChange={(e) => handleEmotionChange('valence', parseFloat(e.target.value))}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-gray-400">
                    <span>Negative</span>
                    <span>Neutral</span>
                    <span>Positive</span>
                  </div>
                </div>

                <div>
                  <label className="block text-blue-200 text-sm font-medium mb-1">
                    Arousal (Energy Level): {emotionData.arousal.toFixed(2)}
                  </label>
                  <input
                    type="range"
                    min="-1"
                    max="1"
                    step="0.01"
                    value={emotionData.arousal}
                    onChange={(e) => handleEmotionChange('arousal', parseFloat(e.target.value))}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-gray-400">
                    <span>Low Energy</span>
                    <span>Calm</span>
                    <span>High Energy</span>
                  </div>
                </div>

                <div>
                  <label className="block text-blue-200 text-sm font-medium mb-1">
                    Dominance (Control): {emotionData.dominance.toFixed(2)}
                  </label>
                  <input
                    type="range"
                    min="-1"
                    max="1"
                    step="0.01"
                    value={emotionData.dominance}
                    onChange={(e) => handleEmotionChange('dominance', parseFloat(e.target.value))}
                    className="w-full"
                  />
                  <div className="flex justify-between text-xs text-gray-400">
                    <span>Submissive</span>
                    <span>Balanced</span>
                    <span>Dominant</span>
                  </div>
                </div>

                <div className="bg-gray-800 rounded-lg p-3">
                  <p className="text-sm text-blue-200">
                    Current Emotion: {emotionToString(emotionData)}
                  </p>
                </div>
              </div>

              <button
                onClick={handleCreateIdentity}
                disabled={loading || !keypair || !identityName || !metadataUri}
                className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold py-3 px-4 rounded-lg transition-colors"
              >
                {loading ? 'Creating Identity...' : 'Create Soulbound Identity'}
              </button>
            </div>
          </div>

          {/* Identity Display */}
          <div className="bg-black bg-opacity-30 rounded-lg p-6">
            <h2 className="text-2xl font-semibold text-white mb-4">
              Your Identity
            </h2>
            
            {identity ? (
              <div className="space-y-4">
                <div className="bg-gray-800 rounded-lg p-4">
                  <h3 className="text-lg font-medium text-white mb-2">{identity.name}</h3>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-400">ID:</span>
                      <span className="text-white">#{identity.identity_id}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Owner:</span>
                      <span className="text-white text-xs">{identity.owner.slice(0, 8)}...{identity.owner.slice(-8)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Verified:</span>
                      <span className={identity.verified ? 'text-green-400' : 'text-yellow-400'}>
                        {identity.verified ? '✓ Verified' : '⏳ Pending'}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Reputation:</span>
                      <span className="text-white">{identity.reputation_score}/100</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Created:</span>
                      <span className="text-white">
                        {new Date(identity.created_at).toLocaleDateString()}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="bg-gray-800 rounded-lg p-4">
                  <h4 className="text-md font-medium text-white mb-2">Emotional State</h4>
                  <div className="space-y-1 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Valence:</span>
                      <span className="text-white">{(identity.emotion_data.valence * 127).toFixed(0)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Arousal:</span>
                      <span className="text-white">{(identity.emotion_data.arousal * 127).toFixed(0)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Dominance:</span>
                      <span className="text-white">{(identity.emotion_data.dominance * 127).toFixed(0)}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Confidence:</span>
                      <span className="text-white">{identity.emotion_data.confidence}%</span>
                    </div>
                  </div>
                  <div className="mt-2 p-2 bg-gray-700 rounded">
                    <p className="text-xs text-blue-200">
                      {emotionToString(identity.emotion_data)}
                    </p>
                  </div>
                </div>

                <div className="bg-gray-800 rounded-lg p-4">
                  <h4 className="text-md font-medium text-white mb-2">Biometric Hash</h4>
                  <div className="text-xs text-gray-400 break-all font-mono">
                    {Array.from(identity.biometric_hash)
                      .map(byte => byte.toString(16).padStart(2, '0'))
                      .join('')}
                  </div>
                </div>

                <div className="bg-gray-800 rounded-lg p-4">
                  <h4 className="text-md font-medium text-white mb-2">Metadata</h4>
                  <a 
                    href={identity.metadata_uri} 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="text-blue-400 hover:text-blue-300 text-sm break-all"
                  >
                    {identity.metadata_uri}
                  </a>
                </div>
              </div>
            ) : (
              <div className="text-center py-8">
                <div className="text-gray-400 mb-4">
                  <svg className="w-16 h-16 mx-auto mb-4 opacity-50" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clipRule="evenodd" />
                  </svg>
                  <p>No identity created yet</p>
                  <p className="text-sm">Create your soulbound identity using the form</p>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Information Panel */}
        <div className="bg-black bg-opacity-30 rounded-lg p-6 mt-6">
          <h3 className="text-xl font-semibold text-white mb-4">About Soulbound Identities</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-blue-200">
            <div>
              <h4 className="font-medium text-white mb-2">🔒 Non-Transferable</h4>
              <p>Once created, your identity cannot be transferred or sold. It's permanently bound to your account.</p>
            </div>
            <div>
              <h4 className="font-medium text-white mb-2">🧬 Biometric Verification</h4>
              <p>Your identity includes biometric data verification, ensuring authenticity and preventing fraud.</p>
            </div>
            <div>
              <h4 className="font-medium text-white mb-2">😊 Emotional State</h4>
              <p>Capture your emotional state at the time of creation using valence, arousal, and dominance parameters.</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PolkadotSoulboundIdentity;