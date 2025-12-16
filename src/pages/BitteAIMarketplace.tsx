import React, { useState, useEffect } from 'react';
import { bitteService, AIAgent } from '../services/bitteService';

interface AINFT {
  id: string;
  owner_id: string;
  metadata: {
    title: string;
    description: string;
    media: string;
    media_hash?: string;
    copies?: number;
    issued_at?: string;
    expires_at?: string;
    starts_at?: string;
    updated_at?: string;
    extra?: string;
    reference?: string;
    reference_hash?: string;
  };
  ai_data?: {
    emotion_vector: {
      valence: number;
      arousal: number;
      dominance: number;
    };
    biometric_hash: string;
    ai_model_used: string;
    generation_params: Record<string, any>;
  };
  approved_account_ids?: Record<string, number>;
  royalty?: Record<string, number>;
  split_owners?: Record<string, number>;
  minter?: string;
  loan?: string;
  composeable_stats?: {
    local_depth: number;
    cross_contract_children: number;
  };
  origin_key?: string;
}

// Use AIAgent interface from bitteService instead of local interface

interface BitteMarketplaceProps {
  className?: string;
}

const BitteAIMarketplace: React.FC<BitteMarketplaceProps> = ({ className }) => {
  const [nfts, setNfts] = useState<AINFT[]>([]);
  const [aiAgents, setAiAgents] = useState<AIAgent[]>([]);
  const [loading, setLoading] = useState(false);
  const [walletConnected, setWalletConnected] = useState(false);
  const [accountId, setAccountId] = useState<string>('');
  const [aiPrompt, setAiPrompt] = useState('');
  const [generatedArt, setGeneratedArt] = useState<string>('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [emotionData, setEmotionData] = useState<{
    valence: number;
    arousal: number;
    dominance: number;
  }>({
    valence: 0.5,
    arousal: 0.5,
    dominance: 0.5
  });

  // Bitte Protocol Configuration (for future real integration)

  // Connect to Bitte AI Wallet
  const connectBitteWallet = async () => {
    try {
      const connection = await bitteService.connectWallet();
      
      if (connection.success) {
        setAccountId(connection.accountId!);
        setWalletConnected(true);
        
        // Load AI agents and NFTs
        await loadAIAgents();
        await loadAINFTs(connection.accountId!);
      } else {
        console.error('Wallet connection failed:', connection.error);
        // Fallback to mock for demo purposes
        const mockAccountId = `bitte_user_${Math.random().toString(36).substr(2, 9)}.near`;
        setAccountId(mockAccountId);
        setWalletConnected(true);
        
        await loadAIAgents();
        await loadAINFTs(mockAccountId);
      }
    } catch (error) {
      console.error('Failed to connect Bitte wallet:', error);
      // Fallback to mock for demo purposes
      const mockAccountId = `bitte_user_${Math.random().toString(36).substr(2, 9)}.near`;
      setAccountId(mockAccountId);
      setWalletConnected(true);
      
      await loadAIAgents();
      await loadAINFTs(mockAccountId);
    }
  };

  // Load AI agents from Bitte Protocol
  const loadAIAgents = async () => {
    try {
      const agents = await bitteService.loadAIAgents();
      setAiAgents(agents);
    } catch (error) {
      console.error('Failed to load AI agents:', error);
    }
  };

  // Load AI-powered NFTs
  const loadAINFTs = async (accountId: string) => {
    setLoading(true);
    try {
      // Mock AI NFTs with biometric and emotional data
      const mockAINFTs: AINFT[] = [
        {
          id: 'ai_token_001',
          owner_id: accountId,
          metadata: {
            title: 'Emotional Biometric Portrait #1',
            description: 'AI-generated art based on biometric emotional analysis',
            media: 'https://trae-api-sg.mchost.guru/api/ide/v1/text_to_image?prompt=emotional%20biometric%20abstract%20portrait%20with%20neural%20patterns&image_size=square_hd',
            copies: 1,
            issued_at: new Date().toISOString(),
          },
          ai_data: {
            emotion_vector: { valence: 0.7, arousal: 0.6, dominance: 0.8 },
            biometric_hash: '0x1234567890abcdef',
            ai_model_used: 'emotion_analyzer_v2',
            generation_params: {
              style: 'abstract_biometric',
              color_palette: 'emotional_warm',
              complexity: 0.8
            }
          },
          royalty: { [accountId]: 750 }, // 7.5% royalty
          minter: accountId,
        },
        {
          id: 'ai_token_002',
          owner_id: accountId,
          metadata: {
            title: 'AI Soulbound Identity',
            description: 'Unique biometric identity NFT with AI verification',
            media: 'https://trae-api-sg.mchost.guru/api/ide/v1/text_to_image?prompt=soulbound%20identity%20nft%20with%20ai%20verification%20patterns&image_size=square_hd',
            copies: 1,
            issued_at: new Date().toISOString(),
          },
          ai_data: {
            emotion_vector: { valence: 0.5, arousal: 0.4, dominance: 0.9 },
            biometric_hash: '0xfedcba0987654321',
            ai_model_used: 'biometric_validator',
            generation_params: {
              style: 'identity_verification',
              security_level: 'maximum',
              uniqueness_score: 0.95
            }
          },
          royalty: { [accountId]: 1000 }, // 10% royalty
          minter: accountId,
        }
      ];
      
      setNfts(mockAINFTs);
    } catch (error) {
      console.error('Failed to load AI NFTs:', error);
    } finally {
      setLoading(false);
    }
  };

  // Generate AI art using Bitte Protocol
  const generateAIArt = async () => {
    if (!aiPrompt.trim()) return;
    
    setIsGenerating(true);
    try {
      const result = await bitteService.generateEmotionalFractal(emotionData);
      
      if (result.success && result.visualOutput) {
        // Use the visual output from the server
        setGeneratedArt(`data:image/svg+xml;base64,${btoa(result.visualOutput.svg)}`);
        
        console.log('AI Art generated with parameters:', {
          prompt: aiPrompt,
          emotion_data: emotionData,
          ai_model: 'bitte-emotion-generator',
          timestamp: new Date().toISOString(),
          fractalId: result.fractalId
        });
      } else {
        console.error('Fractal generation failed:', result.error);
        // Fallback to image generation
        const enhancedPrompt = `${aiPrompt} with emotional valence ${emotionData.valence}, arousal ${emotionData.arousal}, dominance ${emotionData.dominance}`;
        const fallbackArt = `https://trae-api-sg.mchost.guru/api/ide/v1/text_to_image?prompt=${encodeURIComponent(enhancedPrompt)}&image_size=square_hd`;
        setGeneratedArt(fallbackArt);
      }
    } catch (error) {
      console.error('Failed to generate AI art:', error);
      // Fallback to mock generation
      const enhancedPrompt = `${aiPrompt} with emotional valence ${emotionData.valence}, arousal ${emotionData.arousal}, dominance ${emotionData.dominance}`;
      const mockGeneratedArt = `https://trae-api-sg.mchost.guru/api/ide/v1/text_to_image?prompt=${encodeURIComponent(enhancedPrompt)}&image_size=square_hd`;
      setGeneratedArt(mockGeneratedArt);
    } finally {
      setIsGenerating(false);
    }
  };

  // Mint AI-generated NFT through Bitte Protocol
  const mintAINFT = async () => {
    if (!walletConnected || !generatedArt) {
      alert('Please connect wallet and generate art first');
      return;
    }

    try {
      const result = await bitteService.mintBiometricNFT(emotionData, generatedArt);
      
      if (result.success) {
        // Add the new NFT to the list
        const newAINFT: AINFT = {
          id: result.tokenId!,
          owner_id: accountId,
          metadata: {
            title: result.metadata?.title || 'AI Generated Biometric NFT',
            description: result.metadata?.description || 'AI-generated biometric NFT with emotional intelligence',
            media: generatedArt,
            copies: 1,
            issued_at: new Date().toISOString(),
          },
          ai_data: {
            emotion_vector: emotionData,
            biometric_hash: result.biometricData?.biometricHash || 'unknown',
            ai_model_used: 'bitte-emotion-generator',
            generation_params: {
              prompt: aiPrompt,
              emotion_based: true,
              generation_timestamp: new Date().toISOString(),
              blockchain_tx: result.transactionHash
            }
          },
          royalty: { [accountId]: 500 }, // 5% royalty
          minter: accountId,
        };
        
        setNfts([...nfts, newAINFT]);
        alert(`AI NFT minted successfully! Transaction: ${result.transactionHash}`);
        
        // Reset generation
        setGeneratedArt('');
        setAiPrompt('');
      } else {
        console.error('NFT minting failed:', result.error);
        alert('Failed to mint AI NFT: ' + result.error);
      }
    } catch (error) {
      console.error('Failed to mint AI NFT:', error);
      alert('Failed to mint AI NFT: ' + (error instanceof Error ? error.message : 'Unknown error'));
    }
  };

  // Execute AI-powered transaction through Bitte
  const executeAITransaction = async (action: string, params: any) => {
    if (!walletConnected) {
      alert('Please connect Bitte wallet first');
      return;
    }

    try {
      const result = await bitteService.executeAITransaction(action, {
        ...params,
        emotion_context: emotionData,
        timestamp: new Date().toISOString()
      });
      
      if (result.success) {
        console.log(`AI transaction "${action}" executed successfully:`, result);
        alert(`AI transaction "${action}" executed successfully! Transaction: ${result.transactionHash}`);
      } else {
        console.error('AI transaction failed:', result.error);
        alert('Failed to execute AI transaction: ' + result.error);
      }
    } catch (error) {
      console.error('Failed to execute AI transaction:', error);
      alert('Failed to execute AI transaction');
    }
  };

  useEffect(() => {
    // Auto-connect mock Bitte wallet for demo
    if (!walletConnected) {
      connectBitteWallet();
    }
  }, []);

  return (
    <div className={`min-h-screen bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900 ${className || ''}`}>
      <div className="container mx-auto px-4 py-8">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-4">Bitte Protocol AI Marketplace</h1>
          <p className="text-xl text-gray-300 mb-6">
            AI-powered biometric NFT marketplace with emotional intelligence
          </p>
          
          {!walletConnected ? (
            <button
              onClick={connectBitteWallet}
              className="bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 transform hover:scale-105"
            >
              Connect Bitte AI Wallet
            </button>
          ) : (
            <div className="text-white">
              <p className="mb-4">Connected to Bitte: <span className="font-mono bg-gray-800 px-2 py-1 rounded">{accountId}</span></p>
              <div className="flex justify-center space-x-4">
                <button
                  onClick={() => executeAITransaction('mint_ai_nft', { type: 'biometric' })}
                  className="bg-gradient-to-r from-green-600 to-teal-600 hover:from-green-700 hover:to-teal-700 text-white font-bold py-2 px-4 rounded-lg transition duration-200"
                >
                  AI Mint Biometric NFT
                </button>
                <button
                  onClick={() => executeAITransaction('create_agent', { capabilities: ['emotion_analysis'] })}
                  className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-bold py-2 px-4 rounded-lg transition duration-200"
                >
                  Deploy AI Agent
                </button>
              </div>
            </div>
          )}
        </div>

        {/* AI Art Generation Section */}
        <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-xl p-6 mb-8">
          <h2 className="text-2xl font-bold text-white mb-4">AI Art Generator</h2>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div>
              <textarea
                value={aiPrompt}
                onChange={(e) => setAiPrompt(e.target.value)}
                placeholder="Describe the AI art you want to create... (e.g., 'emotional landscape with biometric patterns')"
                className="w-full p-3 border border-gray-300 rounded-lg mb-4 h-24 resize-none"
              />
              
              <div className="mb-4">
                <h3 className="text-white font-semibold mb-2">Emotion Parameters</h3>
                <div className="space-y-2">
                  <div>
                    <label className="text-white text-sm">Valence: {emotionData.valence}</label>
                    <input
                      type="range"
                      min="0"
                      max="1"
                      step="0.1"
                      value={emotionData.valence}
                      onChange={(e) => setEmotionData({...emotionData, valence: parseFloat(e.target.value)})}
                      className="w-full"
                    />
                  </div>
                  <div>
                    <label className="text-white text-sm">Arousal: {emotionData.arousal}</label>
                    <input
                      type="range"
                      min="0"
                      max="1"
                      step="0.1"
                      value={emotionData.arousal}
                      onChange={(e) => setEmotionData({...emotionData, arousal: parseFloat(e.target.value)})}
                      className="w-full"
                    />
                  </div>
                  <div>
                    <label className="text-white text-sm">Dominance: {emotionData.dominance}</label>
                    <input
                      type="range"
                      min="0"
                      max="1"
                      step="0.1"
                      value={emotionData.dominance}
                      onChange={(e) => setEmotionData({...emotionData, dominance: parseFloat(e.target.value)})}
                      className="w-full"
                    />
                  </div>
                </div>
              </div>
              
              <div className="flex space-x-2">
                <button
                  onClick={generateAIArt}
                  disabled={!aiPrompt.trim() || isGenerating}
                  className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-bold py-2 px-4 rounded-lg transition duration-200"
                >
                  {isGenerating ? 'Generating...' : 'Generate AI Art'}
                </button>
                {generatedArt && (
                  <button
                    onClick={mintAINFT}
                    className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition duration-200"
                  >
                    Mint as NFT
                  </button>
                )}
              </div>
            </div>
            
            <div>
              {generatedArt ? (
                <div>
                  <h3 className="text-white font-semibold mb-2">Generated Art</h3>
                  <img
                    src={generatedArt}
                    alt="AI Generated Art"
                    className="w-full h-64 object-cover rounded-lg mb-2"
                  />
                  <p className="text-gray-300 text-sm">Generated with emotion parameters: V{emotionData.valence} A{emotionData.arousal} D{emotionData.dominance}</p>
                </div>
              ) : (
                <div className="border-2 border-dashed border-gray-400 rounded-lg h-64 flex items-center justify-center">
                  <p className="text-gray-400">AI art will appear here</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* AI Agents Section */}
        <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-xl p-6 mb-8">
          <h2 className="text-2xl font-bold text-white mb-4">Available AI Agents</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {aiAgents.map((agent) => (
              <div key={agent.agent_id} className="bg-gray-800 bg-opacity-50 rounded-lg p-4">
                <h3 className="text-white font-bold mb-2">{agent.name}</h3>
                <p className="text-gray-300 text-sm mb-2">{agent.ai_model}</p>
                <div className="mb-2">
                  {agent.capabilities.map((capability) => (
                    <span key={capability} className="inline-block bg-blue-600 text-white text-xs px-2 py-1 rounded mr-1 mb-1">
                      {capability}
                    </span>
                  ))}
                </div>
                <button
                  onClick={() => executeAITransaction('deploy_agent', { agent_id: agent.agent_id })}
                  className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-1 px-2 rounded text-sm transition duration-200"
                >
                  Deploy Agent
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* AI NFTs Marketplace */}
        {loading && (
          <div className="text-center text-white">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-white mx-auto mb-4"></div>
            <p>Loading AI NFTs...</p>
          </div>
        )}

        {!loading && nfts.length > 0 && (
          <div className="bg-white bg-opacity-10 backdrop-blur-lg rounded-xl p-6">
            <h2 className="text-2xl font-bold text-white mb-4">AI-Powered NFTs</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {nfts.map((nft) => (
                <div key={nft.id} className="bg-gray-800 bg-opacity-50 rounded-lg p-4">
                  <img
                    src={nft.metadata.media}
                    alt={nft.metadata.title}
                    className="w-full h-48 object-cover rounded-lg mb-4"
                  />
                  <h3 className="text-white font-bold mb-2">{nft.metadata.title}</h3>
                  <p className="text-gray-300 text-sm mb-3">{nft.metadata.description}</p>
                  
                  {nft.ai_data && (
                    <div className="mb-3">
                      <h4 className="text-white text-sm font-semibold mb-1">AI Data</h4>
                      <div className="text-xs text-gray-400">
                        <p>Model: {nft.ai_data.ai_model_used}</p>
                        <p>Emotion: V{nft.ai_data.emotion_vector.valence.toFixed(2)} A{nft.ai_data.emotion_vector.arousal.toFixed(2)} D{nft.ai_data.emotion_vector.dominance.toFixed(2)}</p>
                      </div>
                    </div>
                  )}
                  
                  <div className="flex space-x-2">
                    <button
                      onClick={() => executeAITransaction('transfer_nft', { nft_id: nft.id, to: 'collector.bitte.near' })}
                      className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm transition duration-200"
                    >
                      Transfer
                    </button>
                    <button
                      onClick={() => executeAITransaction('list_for_sale', { nft_id: nft.id, price: '5.5' })}
                      className="bg-green-600 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm transition duration-200"
                    >
                      Sell
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {!loading && nfts.length === 0 && walletConnected && (
          <div className="text-center text-white">
            <p className="text-xl mb-4">No AI NFTs found. Generate and mint your first AI-powered NFT!</p>
            <button
              onClick={() => setAiPrompt('biometric emotional art with neural patterns')}
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200"
            >
              Generate First AI NFT
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default BitteAIMarketplace;