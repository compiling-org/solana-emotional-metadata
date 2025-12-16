import React, { useState, useEffect, useCallback, useRef } from 'react';
import { LAMPORTS_PER_SOL, clusterApiUrl, PublicKey } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { ConnectionProvider, WalletProvider, useWallet, useConnection } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter, SolflareWalletAdapter, TorusWalletAdapter } from '@solana/wallet-adapter-wallets';
import { toast } from 'sonner';
import '@solana/wallet-adapter-react-ui/styles.css';
import BiometricNFTClient, { createAnchorProvider } from '../utils/solana-client';
import MediaPipeSensors from '../components/MediaPipeSensors';
import LeapMotionSensors from '../components/LeapMotionSensors';
import { RealBiometricCapture } from '../components/RealBiometricCapture';
import { HybridAIManager } from '../utils/hybrid-ai-manager';
import { useIPFSStorage } from '../utils/real-ipfs-storage.ts';

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
  properties?: Record<string, any>;
}

// Main component with wallet provider
const SolanaEmotionalNFTWrapper: React.FC = () => {
  const network = WalletAdapterNetwork.Devnet;
  const endpoint = clusterApiUrl(network);
  const wallets = [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter(),
    new TorusWalletAdapter(),
  ];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <SolanaEmotionalNFT />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

// Main NFT component with real wallet integration
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
  const { connection } = useConnection();
  const wallet = useWallet();
  const [nftClient, setNftClient] = useState<BiometricNFTClient | null>(null);
  const [devnetSignature, setDevnetSignature] = useState<string>('');
  const [recipientAddress, setRecipientAddress] = useState<string>('');
  const [sendAmount, setSendAmount] = useState<string>('0.01');
  const [storageToken, setStorageToken] = useState<string>('');
  const [metadataCid, setMetadataCid] = useState<string>('');
  const [mintedAccount, setMintedAccount] = useState<string>('');
  const [imageCid, setImageCid] = useState<string>('');
  const [selectedModel, setSelectedModel] = useState<string>('vad_synthesis_v1');
  const [myNfts, setMyNfts] = useState<Array<{ account: string; emotion: { valence: number; arousal: number; dominance: number; timestamp?: number }; quality: number; biometricHash: string; createdAt?: string }>>([]);
  const [metadataContent, setMetadataContent] = useState<any | null>(null);
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [capturing, setCapturing] = useState(false);
  const prevFrameRef = useRef<Uint8ClampedArray | null>(null);
  const [sensorsActive, setSensorsActive] = useState(false);
  const [isRecording, setIsRecording] = useState(false);
  const [sensorCounts, setSensorCounts] = useState({ hands: 0, faces: 0, poses: 0 });
  const [sensorFeatures, setSensorFeatures] = useState<{ faceVariance: number; handOpenness: number; poseStability: number; confidence: number } | null>(null);
  const aiManagerRef = useRef<HybridAIManager | null>(null);
  const [lastSensorUpdate, setLastSensorUpdate] = useState<number | null>(null);
  const emotionChartRef = useRef<HTMLCanvasElement | null>(null);
  const emotionHistoryRef = useRef<Array<{ v: number; a: number; d: number }>>([]);
  const sessionEventsRef = useRef<Array<{ t: number; type: string; data: any }>>([]);
  const [micLevel, setMicLevel] = useState(0);
  const { uploadToIPFS } = useIPFSStorage();
  const [sessionCid, setSessionCid] = useState<string>('');
  const [sessionUrl, setSessionUrl] = useState<string>('');
  const [onChainAccount, setOnChainAccount] = useState<any | null>(null);
  const [recentMemos, setRecentMemos] = useState<Array<{ sig: string; memo: string }>>([]);
  const [loadSessionCid, setLoadSessionCid] = useState<string>('');
  const [loadedSession, setLoadedSession] = useState<any | null>(null);
  const [solBalance, setSolBalance] = useState<number | null>(null);
  const [showLoadedOverlay, setShowLoadedOverlay] = useState<boolean>(true);
  const hasAutoStartedRef = useRef<boolean>(false);
  const [sortDir, setSortDir] = useState<'asc' | 'desc'>('desc');

  // Initialize NFT client when wallet is connected
  useEffect(() => {
    if (wallet.connected && wallet.publicKey) {
      try {
        toast.loading('Initializing NFT client...', {
          duration: 2000,
          position: 'top-center',
        });
        
        const provider = createAnchorProvider(connection, wallet as any);
        const client = new BiometricNFTClient(connection, provider);
        setNftClient(client);
        
        console.log('NFT client initialized with wallet:', wallet.publicKey.toString());
        
        toast.success('🎉 Wallet connected successfully!', {
          duration: 3000,
          position: 'top-center',
          description: 'Ready to mint emotional NFTs',
        });
        
      } catch (error) {
        console.error('Failed to initialize NFT client:', error);
        
        toast.error('Failed to initialize NFT client', {
          duration: 5000,
          position: 'top-center',
          description: 'Please try reconnecting your wallet',
        });
      }
    } else {
      setNftClient(null);
      
      if (wallet.connected === false) {
        toast.info('👋 Wallet disconnected', {
          duration: 3000,
          position: 'top-center',
        });
      }
    }
  }, [wallet.connected, wallet.publicKey, connection]);

  // Initialize AI manager
  useEffect(() => {
    if (!aiManagerRef.current) {
      aiManagerRef.current = new HybridAIManager();
    }
  }, []);

  // Generate AI-enhanced canvas art based on emotion data
  const generateAIArt = useCallback(async () => {
    const { valence, arousal, dominance } = emotionData;
    const canvas = document.createElement('canvas');
    canvas.width = 512;
    canvas.height = 512;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Background gradient based on valence/arousal
    const colors = (() => {
      if (valence > 0.5) {
        return arousal > 0.5 
          ? { primary: '#FF6B6B', secondary: '#FFE66D' }
          : { primary: '#4ECDC4', secondary: '#45B7D1' };
      } else if (valence > 0.0) {
        return arousal > 0.5
          ? { primary: '#FFA726', secondary: '#FF7043' }
          : { primary: '#96CEB4', secondary: '#FFEAA7' };
      } else {
        return arousal > 0.5
          ? { primary: '#8E44AD', secondary: '#E74C3C' }
          : { primary: '#34495E', secondary: '#2C3E50' };
      }
    })();

    const gradient = ctx.createLinearGradient(0, 0, canvas.width, canvas.height);
    gradient.addColorStop(0, colors.primary);
    gradient.addColorStop(1, colors.secondary);
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw AI-influenced patterns using HybridAIManager features
    const ai = aiManagerRef.current;
    let positions: Array<{ x: number; y: number }> = [];
    if (ai) {
      const eeg = ai.generateSyntheticEEG();
      const audio = ai.generateSyntheticAudio();
      const emotion = await ai.detectEmotion(eeg, audio);
      const complexity = Math.floor(20 + emotion.attention * 40);
      for (let i = 0; i < complexity; i++) {
        positions.push({
          x: Math.random(),
          y: Math.random()
        });
      }
    } else {
      positions = Array.from({ length: 20 }, () => ({ x: Math.random(), y: Math.random() }));
    }

    positions.forEach((p, i) => {
      const x = p.x * canvas.width;
      const y = p.y * canvas.height;
      const radius = (arousal * 40) + Math.random() * 25;
      ctx.fillStyle = `${colors.primary}40`;
      ctx.beginPath();
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.fill();

      if (dominance > 0.5 && i > 0) {
        ctx.strokeStyle = `${colors.secondary}20`;
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(x, y);
        const prevX = (positions[i - 1].x * canvas.width + Math.random() * 60 - 30);
        const prevY = (positions[i - 1].y * canvas.height + Math.random() * 60 - 30);
        ctx.lineTo(prevX, prevY);
        ctx.stroke();
      }
    });

    await new Promise<void>((resolve) => {
      canvas.toBlob((blob) => {
        if (blob) {
          const reader = new FileReader();
          reader.onloadend = () => {
            setAiGeneratedArt(reader.result as string);
            (async () => {
              try {
                const cid = await uploadToIPFS(reader.result as string);
                setImageCid(cid);
              } catch {}
            })();
            resolve();
          };
          reader.readAsDataURL(blob);
        } else {
          resolve();
        }
      }, 'image/png');
    });
  }, [emotionData]);

  const saveEmotionSession = async () => {
    try {
      const start = sessionEventsRef.current.length > 0 ? sessionEventsRef.current[0].t : Date.now();
      const end = Date.now();
      const sessionData = {
        version: '1.0',
        model: selectedModel,
        start,
        end,
        duration_ms: end - start,
        emotion_history: emotionHistoryRef.current,
        sensor_counts: sensorCounts,
        sensor_features: sensorFeatures,
        mic_level: micLevel,
        events: sessionEventsRef.current.slice(-500)
      };
      const json = JSON.stringify(sessionData);
      const cid = await uploadToIPFS(json);
      setSessionCid(cid);
      setSessionUrl(`https://ipfs.io/ipfs/${cid}`);
      toast.success('Session stored to IPFS', { duration: 3000, position: 'top-center' });
    } catch (e) {
      toast.error('Failed to store session', { duration: 4000, position: 'top-center' });
    }
  };
  
  const downloadSessionJson = async () => {
    if (!sessionUrl) return;
    try {
      const res = await fetch(sessionUrl);
      const text = await res.text();
      const blob = new Blob([text], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `emotion_session_${new Date().toISOString()}.json`;
      a.click();
      URL.revokeObjectURL(url);
      toast.success('Session JSON downloaded', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to download session JSON', { duration: 4000, position: 'top-center' });
    }
  };
  
  const downloadMetadataJson = async () => {
    if (!metadataCid) return;
    try {
      const res = await fetch(`https://ipfs.io/ipfs/${metadataCid}`);
      const text = await res.text();
      const blob = new Blob([text], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `metadata_${metadataCid}.json`;
      a.click();
      URL.revokeObjectURL(url);
      toast.success('Metadata JSON downloaded', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to download metadata', { duration: 4000, position: 'top-center' });
    }
  };
  
  const loadSessionByCid = async () => {
    if (!loadSessionCid) return;
    try {
      const res = await fetch(`https://ipfs.io/ipfs/${loadSessionCid}`);
      const json = await res.json();
      setLoadedSession(json);
      toast.success('Session loaded', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to load session', { duration: 4000, position: 'top-center' });
    }
  };

  const startBiometricCapture = async () => {
    if (capturing) return;
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: { width: 320, height: 240 } });
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
        try {
          (videoRef.current as HTMLVideoElement).muted = true;
          (videoRef.current as HTMLVideoElement).playsInline = true;
          await (videoRef.current as HTMLVideoElement).play();
        } catch {}
      }
      setCapturing(true);
      setSensorsActive(true);
      setIsRecording(true);
      const process = () => {
        if (!capturing) return;
        const v = videoRef.current;
        const c = canvasRef.current;
        if (v && c) {
          const w = c.width, h = c.height;
          const ctx = c.getContext('2d');
          if (ctx) {
            ctx.drawImage(v, 0, 0, w, h);
            const img = ctx.getImageData(0, 0, w, h);
            const data = img.data;
            let total = 0, motion = 0;
            const prev = prevFrameRef.current;
            for (let i = 0; i < data.length; i += 4) {
              const r = data[i], g = data[i + 1], b = data[i + 2];
              const brightness = (r + g + b) / 3;
              total += brightness;
              if (prev) {
                const pb = (prev[i] + prev[i + 1] + prev[i + 2]) / 3;
                motion += Math.abs(brightness - pb);
              }
            }
            prevFrameRef.current = data.slice();
            const avgBrightness = total / (data.length / 4);
            const val = Math.max(0, Math.min(1, avgBrightness / 255));
            const aro = Math.max(0, Math.min(1, motion / (data.length / 4) / 50));
            const dom = Math.max(0, Math.min(1, 0.5 + (val - 0.5) * 0.6));
            setEmotionData(prev => ({ ...prev, valence: val, arousal: aro, dominance: dom }));
          }
        }
        requestAnimationFrame(process);
      };
      requestAnimationFrame(process);
    } catch (e) {
      toast.error('Camera access failed', { duration: 4000, position: 'top-center' });
    }
  };

  const stopBiometricCapture = () => {
    setCapturing(false);
    const v = videoRef.current;
    if (v && v.srcObject) {
      const tracks = (v.srcObject as MediaStream).getTracks();
      tracks.forEach(t => t.stop());
      v.srcObject = null;
    }
  };

  useEffect(() => {
    if (!hasAutoStartedRef.current) {
      hasAutoStartedRef.current = true;
      setSensorsActive(true);
      setIsRecording(true);
      startBiometricCapture();
    }
  }, []);

  // Update emotion based on MediaPipe sensor metrics
  const handleMediaPipeMetrics = useCallback((metrics: { hands: number; faces: number; poses: number; features?: { faceVariance: number; handOpenness: number; poseStability: number; confidence: number } }) => {
    setSensorCounts({ hands: metrics.hands, faces: metrics.faces, poses: metrics.poses });
    if (metrics.features) setSensorFeatures(metrics.features);
    setLastSensorUpdate(Date.now());
    sessionEventsRef.current.push({ t: Date.now(), type: 'mediapipe_metrics', data: metrics });
    const activity = Math.min(1, (metrics.hands * 0.2 + metrics.faces * 0.3 + metrics.poses * 0.4));
    const featureBoost = metrics.features ? Math.min(1, (
      (metrics.features.handOpenness * 0.4) +
      (metrics.features.faceVariance * 0.3) +
      (metrics.features.poseStability * 0.5) +
      (metrics.features.confidence * 0.6)
    )) : 0;
    const val = Math.max(0, Math.min(1, 0.5 + (metrics.faces > 0 ? 0.12 : -0.08) + (metrics.features ? (metrics.features.faceVariance - 0.2) * 0.3 : 0)));
    const aro = Math.max(0, Math.min(1, activity * 0.7 + featureBoost * 0.3));
    const dom = Math.max(0, Math.min(1, 0.4 + metrics.poses * 0.15 + metrics.hands * 0.1 + (metrics.features ? metrics.features.poseStability * 0.25 : 0)));
    setEmotionData(prev => ({
      ...prev,
      valence: (prev.valence * 0.7) + (val * 0.3),
      arousal: (prev.arousal * 0.5) + (aro * 0.5),
      dominance: (prev.dominance * 0.6) + (dom * 0.4),
    }));
  }, []);

  // Handle audio-driven biometric data
  const handleBiometricData = useCallback((data: {
    heartRate: number;
    breathingRate: number;
    emotion: { valence: number; arousal: number; dominance: number };
    eegBands: { delta: number; theta: number; alpha: number; beta: number; gamma: number };
    timestamp: number;
  }) => {
    setLastSensorUpdate(Date.now());
    sessionEventsRef.current.push({ t: data.timestamp, type: 'biometric_data', data });
    const val = Math.max(0, Math.min(1, (data.emotion.valence + 1) / 2));
    const aro = Math.max(0, Math.min(1, data.emotion.arousal));
    const dom = Math.max(0, Math.min(1, data.emotion.dominance));
    setEmotionData(prev => ({
      ...prev,
      valence: (prev.valence * 0.6) + (val * 0.4),
      arousal: (prev.arousal * 0.6) + (aro * 0.4),
      dominance: (prev.dominance * 0.6) + (dom * 0.4),
    }));
  }, []);

  // Generate biometric hash
  const captureBiometricData = async () => {
    if (!nftClient) {
      toast.error('NFT client not initialized. Please connect wallet first.', {
        duration: 3000,
        position: 'top-center',
      });
      return;
    }
    
    try {
      toast.loading('Generating biometric hash...', {
        duration: 2000,
        position: 'top-center',
      });
      
      const { valence, arousal, dominance } = emotionData;
      const hash = await nftClient.generateBiometricHash({
        valence,
        arousal,
        dominance,
        timestamp: Date.now()
      });
      
      setEmotionData(prev => ({ ...prev, biometricHash: hash }));
      
      toast.success('🔐 Biometric data captured successfully!', {
        duration: 3000,
        position: 'top-center',
        description: 'Your emotional signature has been secured.',
      });
      
    } catch (error) {
      console.error('Failed to generate biometric hash:', error);
      
      // Fallback to simple hash
      const { valence, arousal, dominance } = emotionData;
      const fallbackHash = `bio_${Date.now()}_${valence}_${arousal}_${dominance}`.slice(0, 32);
      setEmotionData(prev => ({ ...prev, biometricHash: fallbackHash }));
      
      toast.warning('⚠️ Using fallback biometric hash', {
        duration: 4000,
        position: 'top-center',
        description: 'Biometric generation failed, using secure fallback.',
      });
    }
  };

  // Mint NFT with real Solana transaction (program + optional memo)
  const mintEmotionalNFT = async () => {
    if (!wallet.connected || !wallet.publicKey || !nftClient || !emotionData.biometricHash) {
      alert('Please connect wallet and capture biometric data first');
      return;
    }

    setIsMinting(true);
    try {
      const metadata: NFTMetadata = {
        name: `Emotional NFT #${Date.now()}`,
        symbol: 'EMO',
        description: `AI-generated emotional art based on biometric data. Valence: ${emotionData.valence.toFixed(2)}, Arousal: ${emotionData.arousal.toFixed(2)}, Dominance: ${emotionData.dominance.toFixed(2)}`,
        image: imageCid ? `https://ipfs.io/ipfs/${imageCid}` : aiGeneratedArt,
        attributes: [
          { trait_type: 'Valence', value: emotionData.valence.toFixed(2) },
          { trait_type: 'Arousal', value: emotionData.arousal.toFixed(2) },
          { trait_type: 'Dominance', value: emotionData.dominance.toFixed(2) },
          { trait_type: 'Biometric Hash', value: emotionData.biometricHash },
          { trait_type: 'Sensor Health', value: Math.round(((micLevel * 0.5) + ((sensorFeatures?.confidence || 0) * 0.5)) * 100) },
          { trait_type: 'Hands', value: sensorCounts.hands },
          { trait_type: 'Faces', value: sensorCounts.faces },
          { trait_type: 'Pose', value: sensorCounts.poses }
        ],
        properties: {
          session_cid: sessionCid || '',
          model: selectedModel
        }
      };

      console.log('🎨 Creating biometric NFT with metadata:', metadata);

      // Calculate quality score based on emotion data
      const baseQuality = nftClient.calculateQualityScore({
        valence: emotionData.valence,
        arousal: emotionData.arousal,
        dominance: emotionData.dominance
      });
      const sensorConfidence = Math.min(1, ((sensorFeatures?.confidence || 0) * 0.6) + (micLevel * 0.4));
      const activityBonus = Math.min(1, (sensorCounts.hands * 0.1) + (sensorCounts.faces * 0.1) + (sensorCounts.poses * 0.2));
      const qualityScore = Math.round(Math.max(70, Math.min(100, baseQuality * 0.7 + sensorConfidence * 20 + activityBonus * 10)));

      let cid = '';
      if (storageToken) {
        cid = await nftClient.uploadMetadataNFTStorage(storageToken, metadata);
        setMetadataCid(cid);
      }

      // On-chain program mint (initializeNft)
      const result = await nftClient.initializeNFT(
        wallet.publicKey,
        {
          valence: emotionData.valence,
          arousal: emotionData.arousal,
          dominance: emotionData.dominance,
          timestamp: Date.now()
        },
        qualityScore,
        emotionData.biometricHash
      );

      setTransactionSignature(result.transactionSignature);
      setMintedAccount(result.nftAccount.toString());
      setNftMinted(true);

      // Optional memo for audit trail (non-blocking)
      try {
        const memoSig = await nftClient.sendMemoWithWallet(
          wallet,
          `NFT_MINT:${emotionData.biometricHash}:${qualityScore}:${cid || 'no_cid'}:${Date.now()}`
        );
        console.log('📝 Memo signature:', memoSig);
      } catch (memoErr) {
        console.warn('Memo failed (non-blocking):', memoErr);
      }
      
      toast.success('🎨 Emotional NFT minted via program!', {
        duration: 4000,
        position: 'top-center',
        description: `Tx: ${result.transactionSignature.slice(0, 8)}...${result.transactionSignature.slice(-8)}`,
      });
      
    } catch (error) {
      console.error('❌ Failed to mint NFT:', error);
      
      let errorMessage = 'Failed to mint NFT. ';
      if (error instanceof Error) {
        if (error.message.includes('insufficient funds')) {
          errorMessage += 'Insufficient SOL balance. Please get some devnet SOL from the faucet.';
        } else if (error.message.includes('User rejected')) {
          errorMessage += 'Transaction was rejected by wallet.';
        } else if (error.message.includes('Wallet does not support')) {
          errorMessage += 'Your wallet does not support transaction signing.';
        } else {
          errorMessage += error.message;
        }
      } else {
        errorMessage += 'Unknown error occurred.';
      }
      
      toast.error(errorMessage, {
        duration: 5000,
        position: 'top-center',
      });
    } finally {
      setIsMinting(false);
    }
  };

  const sendDevnetMemo = async () => {
    if (!wallet.connected || !wallet.publicKey || !nftClient) {
      toast.error('Please connect wallet first', {
        duration: 3000,
        position: 'top-center',
      });
      return;
    }
    
    try {
      toast.loading('Requesting devnet SOL airdrop...', {
        duration: 2000,
        position: 'top-center',
      });
      
      // Request airdrop for testing
      const airdropSig = await connection.requestAirdrop(wallet.publicKey, 0.05 * LAMPORTS_PER_SOL);
      await connection.confirmTransaction(airdropSig, 'confirmed');
      
      toast.loading('Sending memo transaction...', {
        duration: 2000,
        position: 'top-center',
      });
      
      // Send memo transaction using wallet
      const sig = await nftClient.sendMemoWithWallet(wallet, 'Emotional NFT Studio memo');
      setDevnetSignature(sig);
      
      console.log('✅ Devnet memo sent:', sig);
      
      toast.success('📝 Devnet memo sent successfully!', {
        duration: 4000,
        position: 'top-center',
        description: `Transaction: ${sig.slice(0, 8)}...${sig.slice(-8)}`,
      });
      
    } catch (e) {
      console.error('Failed to send memo:', e);
      
      let errorMessage = 'Failed to send memo transaction. ';
      if (e instanceof Error) {
        if (e.message.includes('insufficient funds')) {
          errorMessage += 'Insufficient SOL balance for transaction fees.';
        } else if (e.message.includes('User rejected')) {
          errorMessage += 'Transaction was rejected by wallet.';
        } else {
          errorMessage += e.message;
        }
      } else {
        errorMessage += 'Unknown error occurred.';
      }
      
      toast.error(errorMessage, {
        duration: 5000,
        position: 'top-center',
      });
    }
  };

  const sendSolTransfer = async () => {
    if (!wallet.connected || !wallet.publicKey || !nftClient) {
      toast.error('Please connect wallet first', { duration: 3000, position: 'top-center' });
      return;
    }
    try {
      const lamports = Math.floor(parseFloat(sendAmount) * LAMPORTS_PER_SOL);
      const to = new PublicKey(recipientAddress);
      const sig = await nftClient.sendSol(wallet, to, lamports);
      toast.success('Transfer sent', { duration: 4000, position: 'top-center', description: `Signature: ${sig.slice(0,8)}...${sig.slice(-8)}` });
    } catch (e) {
      toast.error(e instanceof Error ? e.message : 'Transfer failed', { duration: 5000, position: 'top-center' });
    }
  };

  // Generate initial art
  useEffect(() => {
    generateAIArt();
    // update emotion history and chart
    const hist = emotionHistoryRef.current;
    hist.push({ v: emotionData.valence, a: emotionData.arousal, d: emotionData.dominance });
    if (hist.length > 100) hist.shift();
    const canvas = emotionChartRef.current;
    if (canvas) {
      const ctx = canvas.getContext('2d');
      if (ctx) {
        const w = canvas.width, h = canvas.height;
        ctx.clearRect(0, 0, w, h);
        const drawLine = (key: 'v' | 'a' | 'd', color: string) => {
          ctx.strokeStyle = color;
          ctx.lineWidth = 2;
          ctx.beginPath();
          hist.forEach((p, i) => {
            const x = (i / (hist.length - 1)) * w;
            const y = h - (p[key] * h);
            if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
          });
          ctx.stroke();
        };
        drawLine('v', '#a78bfa');
        drawLine('a', '#f59e0b');
        drawLine('d', '#10b981');
        if (onChainAccount?.emotionHistory?.length) {
          const chainHist = onChainAccount.emotionHistory.slice(-100);
          const drawChain = (key: 'valence' | 'arousal' | 'dominance', color: string) => {
            ctx.strokeStyle = color;
            ctx.lineWidth = 1.5;
            ctx.beginPath();
            chainHist.forEach((p: any, i: number) => {
              const x = (i / (chainHist.length - 1)) * w;
              const y = h - ((p[key] || 0) * h);
              if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
            });
            ctx.stroke();
          };
          drawChain('valence', '#a78bfa55');
          drawChain('arousal', '#f59e0b55');
          drawChain('dominance', '#10b98155');
        }
        if (showLoadedOverlay && loadedSession?.emotion_history?.length) {
          const sessHist = loadedSession.emotion_history.slice(-100);
          const drawSess = (key: 'v' | 'a' | 'd', color: string) => {
            ctx.strokeStyle = color;
            ctx.lineWidth = 1;
            ctx.setLineDash([4, 3]);
            ctx.beginPath();
            sessHist.forEach((p: any, i: number) => {
              const x = (i / (sessHist.length - 1)) * w;
              const y = h - ((p[key] || 0) * h);
              if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
            });
            ctx.stroke();
            ctx.setLineDash([]);
          };
          drawSess('v', '#a78bfa88');
          drawSess('a', '#f59e0b88');
          drawSess('d', '#10b98188');
        }
      }
    }
  }, [generateAIArt, onChainAccount, loadedSession, showLoadedOverlay]);
  
  const refreshBalance = async () => {
    if (!wallet.publicKey) return;
    try {
      const lamports = await connection.getBalance(wallet.publicKey, 'confirmed');
      setSolBalance(lamports / LAMPORTS_PER_SOL);
      toast.success('Balance refreshed', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to refresh balance', { duration: 4000, position: 'top-center' });
    }
  };
  
  const requestAirdrop = async () => {
    if (!wallet.publicKey) return;
    try {
      const sig = await connection.requestAirdrop(wallet.publicKey, Math.floor(0.5 * LAMPORTS_PER_SOL));
      await connection.confirmTransaction(sig, 'confirmed');
      await refreshBalance();
      toast.success('Airdrop requested', { duration: 2000, position: 'top-center', description: `Tx: ${sig.slice(0,8)}...${sig.slice(-8)}` });
    } catch {
      toast.error('Airdrop failed', { duration: 4000, position: 'top-center' });
    }
  };
  
  const refreshOnChainEmotion = async () => {
    if (!nftClient || !mintedAccount) return;
    try {
      const acc = await nftClient.getNFTAccount(new PublicKey(mintedAccount));
      setOnChainAccount(acc);
      toast.success('On-chain emotion refreshed', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to refresh on-chain emotion', { duration: 4000, position: 'top-center' });
    }
  };
  
  const fetchRecentMemos = async () => {
    if (!wallet.publicKey) return;
    try {
      const sigs = await connection.getSignaturesForAddress(wallet.publicKey, { limit: 20 });
      const out: Array<{ sig: string; memo: string }> = [];
      for (const s of sigs) {
        const tx = await connection.getParsedTransaction(s.signature, { maxSupportedTransactionVersion: 0 });
        if (tx && tx.transaction.message.instructions) {
          for (const inst of tx.transaction.message.instructions as any[]) {
            const pid = inst.programId?.toString?.() || inst.programId;
            const parsed = inst.parsed;
            if (parsed?.type === 'memo' && parsed?.info?.memo) {
              out.push({ sig: s.signature, memo: parsed.info.memo });
            } else if (pid === 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr') {
              const data = inst.data || '';
              try {
                const memo = typeof data === 'string' ? atob(data) : '';
                if (memo) out.push({ sig: s.signature, memo });
              } catch {}
            }
          }
        }
      }
      setRecentMemos(out.slice(0, 10));
      toast.success('Recent memos fetched', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to fetch memos', { duration: 4000, position: 'top-center' });
    }
  };
  
  const downloadOnChainHistory = () => {
    if (!onChainAccount || !onChainAccount.emotionHistory) return;
    try {
      const blob = new Blob([JSON.stringify(onChainAccount.emotionHistory, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `onchain_emotion_history_${mintedAccount || 'unknown'}.json`;
      a.click();
      URL.revokeObjectURL(url);
      toast.success('On-chain history downloaded', { duration: 2000, position: 'top-center' });
    } catch {
      toast.error('Failed to download on-chain history', { duration: 4000, position: 'top-center' });
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-violet-900 via-indigo-900 to-purple-900 text-white p-6">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-4xl font-bold mb-2 text-center">Solana Emotional NFT Studio</h1>
        <div className="flex justify-center mb-6">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-black/40 border border-violet-500/40 text-violet-200 text-sm">
            <span>Solana Program-backed Mint</span>
          </div>
        </div>
        <div className="flex flex-col md:flex-row items-center justify-center gap-4 mb-6">
          <div className="bg-gray-800 border border-gray-700 rounded-lg px-4 py-2 text-sm">
            {wallet.connected ? 'Wallet Connected' : 'Wallet Disconnected'}
          </div>
          <div className="bg-gray-800 border border-gray-700 rounded-lg px-4 py-2 text-sm flex items-center gap-3">
            <span className="text-gray-300">Sensor Health</span>
            <div className="w-40 bg-gray-700 rounded-full h-2">
              <div 
                className="bg-yellow-400 h-2 rounded-full transition-all duration-150"
                style={{ width: `${Math.round(((micLevel * 0.5) + ((sensorFeatures?.confidence || 0) * 0.5)) * 100)}%` }}
              />
            </div>
            <span className="text-gray-400">
              {Math.round(((micLevel * 0.5) + ((sensorFeatures?.confidence || 0) * 0.5)) * 100)}%
            </span>
          </div>
        </div>
        
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Emotion Controls */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-6">Emotion Controls</h2>
            
            {/* Wallet Connection */}
            <div className="mb-6">
              <WalletMultiButton className="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded-lg transition-colors" />
              {wallet.connected && wallet.publicKey && (
                <div className="bg-green-900 border border-green-700 rounded-lg p-3 mt-2">
                  <p className="text-sm font-medium text-green-300">Wallet Connected</p>
                  <p className="text-xs text-green-400 break-all">
                    {wallet.publicKey.toString().slice(0, 8)}...{wallet.publicKey.toString().slice(-8)}
                  </p>
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
            
            {/* Real Sensors */}
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-2">Real Sensors</h3>
              <div className="flex items-center gap-2 mb-3">
                <button
                  onClick={() => { setSensorsActive(true); setIsRecording(true); }}
                  disabled={sensorsActive}
                  className="bg-teal-600 hover:bg-teal-700 text-white px-3 py-2 rounded"
                >
                  Start Sensors
                </button>
                <button
                  onClick={() => { setSensorsActive(false); setIsRecording(false); }}
                  disabled={!sensorsActive}
                  className="bg-red-600 hover:bg-red-700 text-white px-3 py-2 rounded"
                >
                  Stop Sensors
                </button>
                <select
                  value={selectedModel}
                  onChange={(e) => setSelectedModel(e.target.value)}
                  className="bg-gray-700 text-white px-2 py-2 rounded"
                >
                  <option value="vad_synthesis_v1">VAD Synthesis v1</option>
                  <option value="stream_diffusion_v1">Stream Diffusion v1</option>
                </select>
              </div>
            {sensorsActive && (
              <div className="space-y-4">
                <MediaPipeSensors className="mt-2" onMetrics={handleMediaPipeMetrics} />
                <LeapMotionSensors
                  onMetrics={(m) => {
                    sessionEventsRef.current.push({ t: Date.now(), type: 'leapmotion_metrics', data: m });
                  }}
                />
                <RealBiometricCapture 
                  onBiometricData={handleBiometricData} 
                  isRecording={isRecording} 
                  onMicLevel={(lvl) => setMicLevel(lvl)}
                />
                <div className="text-xs text-gray-400">
                  Hands: {sensorCounts.hands} | Faces: {sensorCounts.faces} | Pose: {sensorCounts.poses} | Mic: {Math.round(micLevel * 100)}%
                  {lastSensorUpdate && (
                    <span className="ml-2 text-gray-500">
                      • Last update: {new Date(lastSensorUpdate).toLocaleTimeString()}
                    </span>
                  )}
                </div>
                <div className="w-full bg-gray-700 rounded-full h-2">
                  <div 
                    className="bg-yellow-400 h-2 rounded-full transition-all duration-150"
                    style={{ width: `${Math.round(micLevel * 100)}%` }}
                  />
                </div>
              </div>
            )}
          </div>
            
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2">Recipient (Send SOL)</label>
              <input
                type="text"
                value={recipientAddress}
                onChange={(e) => setRecipientAddress(e.target.value)}
                placeholder="Recipient public key"
                className="w-full h-10 px-3 bg-gray-700 rounded-lg"
              />
              <div className="flex gap-2 mt-2">
                <input
                  type="text"
                  value={sendAmount}
                  onChange={(e) => setSendAmount(e.target.value)}
                  className="w-32 h-10 px-3 bg-gray-700 rounded-lg"
                />
                <button
                  onClick={sendSolTransfer}
                  disabled={!wallet.connected || !recipientAddress}
                  className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg"
                >
                  Send SOL
                </button>
              </div>
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
                <div className="mt-3">
                  <canvas ref={emotionChartRef} width={300} height={80} className="w-full bg-gray-900 rounded" />
                  <div className="text-[10px] text-gray-500 mt-1">Emotion history (V/A/D)</div>
                </div>
              </div>
            </div>
            
            <div className="mb-4">
              <h3 className="text-lg font-medium mb-2">Biometric Capture</h3>
              <div className="flex items-center gap-2">
                <button
                  onClick={startBiometricCapture}
                  disabled={capturing}
                  className="bg-teal-600 hover:bg-teal-700 text-white px-3 py-2 rounded"
                >
                  Start Camera
                </button>
                <button
                  onClick={stopBiometricCapture}
                  disabled={!capturing}
                  className="bg-red-600 hover:bg-red-700 text-white px-3 py-2 rounded"
                >
                  Stop Camera
                </button>
                <button
                  onClick={saveEmotionSession}
                  className="bg-purple-600 hover:bg-purple-700 text-white px-3 py-2 rounded"
                >
                  Save Session to IPFS
                </button>
                <button
                  onClick={async () => {
                    if (!wallet.connected || !nftClient || !sessionCid) {
                      toast.error('Connect wallet and save session first', { duration: 3000, position: 'top-center' });
                      return;
                    }
                    try {
                      const sig = await nftClient.sendMemoWithWallet(
                        wallet,
                        `SESSION_CID:${sessionCid}:${selectedModel}:${Date.now()}`
                      );
                      toast.success('Session anchored via memo', { duration: 3000, position: 'top-center', description: `Tx: ${sig.slice(0,8)}...${sig.slice(-8)}` });
                    } catch {
                      toast.error('Failed to anchor session', { duration: 4000, position: 'top-center' });
                    }
                  }}
                  className="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-2 rounded"
                >
                  Anchor Session via Memo
                </button>
              </div>
              <div className="mt-2 grid grid-cols-1 md:grid-cols-2 gap-2">
                <video ref={videoRef} className="w-full rounded bg-black" muted playsInline />
                <canvas ref={canvasRef} width={320} height={240} className="w-full rounded bg-black" />
              </div>
              {sessionUrl && (
                <div className="mt-3 text-xs text-gray-300 break-all">
                  Session: {sessionUrl}
                  <button
                    onClick={async () => {
                      try {
                        await navigator.clipboard.writeText(sessionUrl);
                        toast.success('Session URL copied', { duration: 2000, position: 'top-center' });
                      } catch {}
                    }}
                    className="ml-2 px-2 py-1 bg-gray-700 rounded"
                  >
                    Copy
                  </button>
                  <button
                    onClick={downloadSessionJson}
                    className="ml-2 px-2 py-1 bg-gray-700 rounded"
                  >
                    Download JSON
                  </button>
                </div>
              )}
              <div className="mt-3">
                <h4 className="text-sm font-medium text-gray-200 mb-1">Session Inspector</h4>
                <pre className="text-xs text-gray-300 bg-gray-900 rounded p-2 overflow-auto max-h-40">
                  {JSON.stringify({
                    model: selectedModel,
                    sessionUrl,
                    latestEvents: sessionEventsRef.current.slice(-10),
                    lastEmotion: emotionHistoryRef.current[emotionHistoryRef.current.length - 1],
                    mic: Math.round(micLevel * 100) / 100,
                    sensors: sensorCounts
                  }, null, 2)}
                </pre>
                <div className="mt-2 flex items-center gap-2">
                  <input
                    type="text"
                    value={loadSessionCid}
                    onChange={(e) => setLoadSessionCid(e.target.value)}
                    placeholder="Load session by CID"
                    className="w-full h-8 px-2 bg-gray-700 rounded"
                  />
                  <button
                    onClick={loadSessionByCid}
                    className="px-2 py-1 bg-gray-700 rounded text-xs"
                  >
                    Load
                  </button>
                  {loadSessionCid && (
                    <a
                      href={`https://ipfs.io/ipfs/${loadSessionCid}`}
                      target="_blank"
                      rel="noreferrer"
                      className="px-2 py-1 bg-gray-700 rounded text-xs underline"
                    >
                      Open in Gateway
                    </a>
                  )}
                  <button
                    onClick={async () => {
                      if (!wallet.connected || !nftClient || !loadSessionCid) {
                        toast.error('Connect wallet and enter CID', { duration: 3000, position: 'top-center' });
                        return;
                      }
                      try {
                        const sig = await nftClient.sendMemoWithWallet(
                          wallet,
                          `SESSION_CID:${loadSessionCid}:${selectedModel}:${Date.now()}`
                        );
                        toast.success('Loaded session anchored', { duration: 3000, position: 'top-center', description: `Tx: ${sig.slice(0,8)}...${sig.slice(-8)}` });
                      } catch {
                        toast.error('Failed to anchor loaded session', { duration: 4000, position: 'top-center' });
                      }
                    }}
                    className="px-2 py-1 bg-indigo-600 rounded text-xs"
                  >
                    Anchor Loaded Session
                  </button>
                  <label className="flex items-center gap-1 text-xs">
                    <input
                      type="checkbox"
                      checked={showLoadedOverlay}
                      onChange={(e) => setShowLoadedOverlay(e.target.checked)}
                    />
                    Overlay on chart
                  </label>
                </div>
                {loadedSession && (
                  <div className="mt-2">
                    <div className="text-xs text-gray-400">Loaded Session Preview</div>
                    <pre className="text-xs text-gray-300 bg-gray-900 rounded p-2 overflow-auto max-h-40">
                      {JSON.stringify(loadedSession, null, 2)}
                    </pre>
                    <button
                      onClick={() => {
                        try {
                          const hist = loadedSession?.emotion_history;
                          if (hist && hist.length) {
                            const last = hist[hist.length - 1];
                            if (typeof last.v === 'number' && typeof last.a === 'number' && typeof last.d === 'number') {
                              setEmotionData(prev => ({ ...prev, valence: last.v, arousal: last.a, dominance: last.d }));
                              toast.success('Applied last emotion from session', { duration: 2000, position: 'top-center' });
                            }
                          }
                        } catch {
                          toast.error('Failed to apply session emotion', { duration: 4000, position: 'top-center' });
                        }
                      }}
                      className="mt-2 px-2 py-1 bg-gray-700 rounded text-xs"
                    >
                      Apply Last Emotion to Sliders
                    </button>
                  </div>
                )}
              </div>
            </div>
            
              {nftMinted && (
                <div className="bg-green-900 border border-green-700 rounded-lg p-4 mb-4">
                  <h3 className="text-lg font-medium text-green-300 mb-2">NFT Minted Successfully!</h3>
                  <p className="text-sm text-green-400 mb-2">Transaction Signature:</p>
                  <p className="text-xs text-green-300 break-all bg-green-800 p-2 rounded">{transactionSignature}</p>
                  <div className="mt-2 flex flex-wrap gap-2">
                    <button
                      onClick={async () => {
                        try {
                          await navigator.clipboard.writeText(transactionSignature);
                          toast.success('Signature copied', { duration: 2000, position: 'top-center' });
                        } catch {}
                      }}
                      className="px-2 py-1 bg-gray-700 rounded text-xs"
                    >
                      Copy Signature
                    </button>
                    {transactionSignature && (
                      <a
                        href={`https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`}
                        target="_blank"
                        rel="noreferrer"
                        className="px-2 py-1 bg-gray-700 rounded text-xs underline"
                      >
                        Open in Explorer
                      </a>
                    )}
                  </div>
                  {mintedAccount && (
                    <div className="mt-2 text-xs text-green-300 break-all">
                      NFT Account: {mintedAccount}
                      <div className="mt-2 flex flex-wrap gap-2">
                        <button
                          onClick={async () => {
                            try {
                              await navigator.clipboard.writeText(mintedAccount);
                              toast.success('Account copied', { duration: 2000, position: 'top-center' });
                            } catch {}
                          }}
                          className="px-2 py-1 bg-gray-700 rounded text-xs"
                        >
                          Copy Account
                        </button>
                        <a
                          href={`https://explorer.solana.com/address/${mintedAccount}?cluster=devnet`}
                          target="_blank"
                          rel="noreferrer"
                          className="px-2 py-1 bg-gray-700 rounded text-xs underline"
                        >
                          Open in Explorer
                        </a>
                      </div>
                    </div>
                  )}
                  {metadataCid && (
                  <div className="mt-2 text-xs text-green-300 break-all">
                    CID: {metadataCid}
                    <button
                      onClick={async () => {
                        try {
                          await navigator.clipboard.writeText(metadataCid);
                          toast.success('CID copied', { duration: 2000, position: 'top-center' });
                        } catch {}
                      }}
                      className="ml-2 px-2 py-1 bg-gray-700 rounded"
                    >
                      Copy
                    </button>
                    <button
                      onClick={async () => {
                        try {
                          const res = await fetch(`https://ipfs.io/ipfs/${metadataCid}`);
                          const json = await res.json();
                          setMetadataContent(json);
                        } catch {
                          toast.error('Failed to load metadata', { duration: 3000, position: 'top-center' });
                        }
                      }}
                      className="ml-2 px-2 py-1 bg-gray-700 rounded"
                    >
                      Load
                    </button>
                    <button
                      onClick={downloadMetadataJson}
                      className="ml-2 px-2 py-1 bg-gray-700 rounded"
                    >
                      Download JSON
                    </button>
                    <a
                      href={`https://ipfs.io/ipfs/${metadataCid}`}
                      target="_blank"
                      rel="noreferrer"
                      className="ml-2 px-2 py-1 bg-gray-700 rounded underline"
                    >
                      Open in Gateway
                    </a>
                  </div>
                  )}
                  {metadataContent && (
                    <div className="mt-2">
                      <div className="text-xs text-green-300">Metadata Preview</div>
                      <pre className="text-xs text-green-200 bg-green-800 rounded p-2 overflow-auto max-h-40">
                        {JSON.stringify(metadataContent, null, 2)}
                      </pre>
                    </div>
                  )}
                </div>
              )}
            
            <button
              onClick={mintEmotionalNFT}
              disabled={isMinting || !wallet.connected || !emotionData.biometricHash}
              className="w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 disabled:from-gray-600 disabled:to-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors"
            >
              {isMinting ? 'Minting NFT...' : 'Mint Emotional NFT'}
            </button>
            <button
              onClick={async () => {
                if (!wallet.connected || !wallet.publicKey || !nftClient || !mintedAccount) {
                  toast.error('Connect wallet and mint first', { duration: 3000, position: 'top-center' });
                  return;
                }
                try {
                  const sig = await nftClient.updateEmotion(
                    new PublicKey(mintedAccount),
                    wallet.publicKey,
                    {
                      valence: emotionData.valence,
                      arousal: emotionData.arousal,
                      dominance: emotionData.dominance,
                      timestamp: Date.now()
                    }
                  );
                  toast.success('Emotion updated on-chain', { duration: 3000, position: 'top-center', description: `Tx: ${sig.slice(0,8)}...${sig.slice(-8)}` });
                } catch (e) {
                  toast.error('Emotion update failed', { duration: 4000, position: 'top-center' });
                }
              }}
              className="mt-2 w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
            >
              Update Emotion On-Chain
            </button>
            <div className="mt-2 flex items-center gap-2">
              <button
                onClick={refreshOnChainEmotion}
                disabled={!mintedAccount}
                className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-2 rounded"
              >
                Refresh On-Chain Emotion
              </button>
            </div>
            {onChainAccount && (
              <div className="mt-2 bg-gray-800 rounded p-3 text-xs">
                <div>
                  On-Chain Emotion: V {onChainAccount.emotionData?.valence?.toFixed(3)} · A {onChainAccount.emotionData?.arousal?.toFixed(3)} · D {onChainAccount.emotionData?.dominance?.toFixed(3)}
                </div>
                {onChainAccount.emotionHistory && onChainAccount.emotionHistory.length > 0 && (
                  <div className="mt-2">
                    <div className="text-gray-400">Emotion History (last 5)</div>
                    <pre className="bg-gray-900 rounded p-2 overflow-auto max-h-40">
                      {JSON.stringify(onChainAccount.emotionHistory.slice(-5), null, 2)}
                    </pre>
                    <button
                      onClick={downloadOnChainHistory}
                      className="mt-2 px-2 py-1 bg-gray-700 rounded"
                    >
                      Download On-Chain History
                    </button>
                  </div>
                )}
              </div>
            )}
            <div className="mt-4 grid grid-cols-1 gap-2">
              <div className="mb-2">
                <input
                  type="text"
                  placeholder="Web3/NFT.Storage API token (optional)"
                  value={storageToken}
                  onChange={(e) => setStorageToken(e.target.value)}
                  className="w-full h-10 px-3 bg-gray-700 rounded-lg"
                />
              </div>
              <div className="text-xs text-gray-400">
                Devnet Faucet:&nbsp;
                {wallet.publicKey ? (
                  <a
                    href={`https://faucet.solana.com/devnet?wallet=${wallet.publicKey.toString()}`}
                    target="_blank"
                    rel="noreferrer"
                    className="text-blue-400 underline"
                  >
                    Request SOL
                  </a>
                ) : (
                  <span>Connect wallet to request SOL</span>
                )}
              </div>
              {wallet.publicKey && (
                <div className="mt-2 flex flex-wrap gap-2 text-xs">
                  <button
                    onClick={async () => {
                      try {
                        await navigator.clipboard.writeText(wallet.publicKey!.toString());
                        toast.success('Wallet address copied', { duration: 2000, position: 'top-center' });
                      } catch {}
                    }}
                    className="px-2 py-1 bg-gray-700 rounded"
                  >
                    Copy Wallet
                  </button>
                  <a
                    href={`https://explorer.solana.com/address/${wallet.publicKey.toString()}?cluster=devnet`}
                    target="_blank"
                    rel="noreferrer"
                    className="px-2 py-1 bg-gray-700 rounded underline"
                  >
                    Open Wallet in Explorer
                  </a>
                  <button
                    onClick={refreshBalance}
                    className="px-2 py-1 bg-gray-700 rounded"
                  >
                    Refresh Balance
                  </button>
                  <button
                    onClick={requestAirdrop}
                    className="px-2 py-1 bg-gray-700 rounded"
                  >
                    Airdrop 0.5 SOL
                  </button>
                  {solBalance !== null && (
                    <span className="px-2 py-1 bg-gray-800 rounded text-green-300">
                      Balance: {solBalance.toFixed(3)} SOL
                    </span>
                  )}
                </div>
              )}
              <button
                onClick={sendDevnetMemo}
                disabled={!wallet.connected}
                className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
              >
                Send Devnet Memo (Validate Wallet)
              </button>
              {devnetSignature && (
                <p className="text-xs text-green-300 break-all">Memo Signature: {devnetSignature}</p>
              )}
              <div className="mt-2">
                <div className="flex items-center gap-2">
                  <button
                    onClick={fetchRecentMemos}
                    disabled={!wallet.connected}
                    className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-2 rounded"
                  >
                    Refresh Recent Anchors (Memo)
                  </button>
                </div>
                {recentMemos.length > 0 && (
                  <div className="mt-2 text-xs">
                    <div className="text-gray-400">Recent Memos</div>
                    <div className="space-y-1">
                      {recentMemos.map((m) => (
                        <div key={m.sig} className="bg-gray-800 rounded p-2">
                          <div className="break-all text-gray-300">{m.memo}</div>
                          <div className="mt-1 flex flex-wrap gap-2">
                            <button
                              onClick={async () => {
                                try {
                                  await navigator.clipboard.writeText(m.memo);
                                  toast.success('Memo copied', { duration: 2000, position: 'top-center' });
                                } catch {}
                              }}
                              className="px-2 py-1 bg-gray-700 rounded"
                            >
                              Copy Memo
                            </button>
                            <a
                              href={`https://explorer.solana.com/tx/${m.sig}?cluster=devnet`}
                              target="_blank"
                              rel="noreferrer"
                              className="px-2 py-1 bg-gray-700 rounded underline"
                            >
                              Open Tx
                            </a>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </div>
              <div className="mt-6">
                <h3 className="text-lg font-medium mb-2">My Emotional NFTs</h3>
                <div className="flex items-center gap-2 mb-2">
                  <button
                    onClick={async () => {
                    if (!wallet.connected || !wallet.publicKey || !nftClient) {
                      toast.error('Connect wallet first', { duration: 3000, position: 'top-center' });
                      return;
                    }
                    try {
                      const pubkeys = await nftClient.getNFTsByOwner(wallet.publicKey);
                      const results: Array<{ account: string; emotion: { valence: number; arousal: number; dominance: number; timestamp?: number }; quality: number; biometricHash: string; createdAt?: string }> = [];
                      for (const pk of pubkeys) {
                        const acc = await nftClient.getNFTAccount(pk);
                        if (acc) {
                          results.push({
                            account: pk.toString(),
                            emotion: acc.emotionData,
                            quality: acc.qualityScore,
                            biometricHash: acc.biometricHash,
                            createdAt: (acc.createdAt && (acc.createdAt as any).toString) ? (acc.createdAt as any).toString() : undefined
                          });
                        }
                      }
                      setMyNfts(results);
                    } catch {
                      toast.error('Failed to load NFTs', { duration: 3000, position: 'top-center' });
                    }
                  }}
                  className="bg-gray-700 hover:bg-gray-600 text-white px-3 py-2 rounded"
                >
                  Refresh My NFTs
                </button>
                <button
                  onClick={async () => {
                    if (!wallet.connected || !wallet.publicKey || !nftClient || myNfts.length === 0) {
                      toast.error('Connect wallet and load NFTs first', { duration: 3000, position: 'top-center' });
                      return;
                    }
                    try {
                      for (const n of myNfts) {
                        await nftClient.updateEmotion(
                          new PublicKey(n.account),
                          wallet.publicKey,
                          {
                            valence: emotionData.valence,
                            arousal: emotionData.arousal,
                            dominance: emotionData.dominance,
                            timestamp: Date.now()
                          }
                        );
                      }
                      toast.success('Bulk emotion update sent', { duration: 3000, position: 'top-center' });
                      const pubkeys = await nftClient.getNFTsByOwner(wallet.publicKey);
                      const results: Array<{ account: string; emotion: { valence: number; arousal: number; dominance: number; timestamp?: number }; quality: number; biometricHash: string; createdAt?: string }> = [];
                      for (const pk of pubkeys) {
                        const acc = await nftClient.getNFTAccount(pk);
                        if (acc) {
                          results.push({
                            account: pk.toString(),
                            emotion: acc.emotionData,
                            quality: acc.qualityScore,
                            biometricHash: acc.biometricHash,
                            createdAt: (acc.createdAt && (acc.createdAt as any).toString) ? (acc.createdAt as any).toString() : undefined
                          });
                        }
                      }
                      setMyNfts(results);
                    } catch {
                      toast.error('Bulk update failed', { duration: 4000, position: 'top-center' });
                    }
                  }}
                  className="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-2 rounded"
                >
                  Bulk Update From Current Emotion
                </button>
                <div className="ml-auto flex items-center gap-2 text-xs">
                  <span className="text-gray-400">Sort by Quality:</span>
                  <button
                    onClick={() => setSortDir('desc')}
                    className={`px-2 py-1 rounded ${sortDir === 'desc' ? 'bg-gray-600' : 'bg-gray-700'}`}
                  >
                    High→Low
                  </button>
                  <button
                    onClick={() => setSortDir('asc')}
                    className={`px-2 py-1 rounded ${sortDir === 'asc' ? 'bg-gray-600' : 'bg-gray-700'}`}
                  >
                    Low→High
                  </button>
                </div>
                </div>
                {myNfts.length === 0 ? (
                  <div className="text-xs text-gray-400">No NFTs found</div>
                ) : (
                  <div className="space-y-2">
                  {[...myNfts].sort((a, b) => sortDir === 'desc' ? (b.quality - a.quality) : (a.quality - b.quality)).map((n) => (
                    <div key={n.account} className="bg-gray-800 rounded p-3 text-xs">
                      <div className="text-gray-300 break-all">Account: {n.account}</div>
                      <div className="mt-1 flex flex-wrap gap-2">
                        <button
                          onClick={async () => {
                            try {
                              await navigator.clipboard.writeText(n.account);
                              toast.success('Account copied', { duration: 2000, position: 'top-center' });
                            } catch {}
                          }}
                          className="px-2 py-1 bg-gray-700 rounded"
                        >
                          Copy
                        </button>
                        <a
                          href={`https://explorer.solana.com/address/${n.account}?cluster=devnet`}
                          target="_blank"
                          rel="noreferrer"
                          className="px-2 py-1 bg-gray-700 rounded underline"
                        >
                          Explorer
                        </a>
                        <button
                          onClick={async () => {
                            if (!nftClient || !wallet.publicKey) return;
                            try {
                              const sig = await nftClient.updateEmotion(
                                new PublicKey(n.account),
                                wallet.publicKey,
                                {
                                  valence: emotionData.valence,
                                  arousal: emotionData.arousal,
                                  dominance: emotionData.dominance,
                                  timestamp: Date.now()
                                }
                              );
                              toast.success('Emotion updated', { duration: 2000, position: 'top-center', description: `Tx: ${sig.slice(0,8)}...${sig.slice(-8)}` });
                            } catch {
                              toast.error('Update failed', { duration: 4000, position: 'top-center' });
                            }
                          }}
                          className="px-2 py-1 bg-indigo-600 rounded text-white"
                        >
                          Update From Current Emotion
                        </button>
                      </div>
                      <div className="grid grid-cols-2 gap-2 mt-2">
                        <div>Valence: {n.emotion.valence.toFixed(3)}</div>
                        <div>Arousal: {n.emotion.arousal.toFixed(3)}</div>
                        <div>Dominance: {n.emotion.dominance.toFixed(3)}</div>
                        <div>Quality: {Math.round(n.quality * 100) / 100}</div>
                      </div>
                      <div className="mt-1 text-gray-400 break-all">Biometric: {n.biometricHash}</div>
                      {n.createdAt && <div className="text-gray-500">CreatedAt: {n.createdAt}</div>}
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SolanaEmotionalNFTWrapper;
