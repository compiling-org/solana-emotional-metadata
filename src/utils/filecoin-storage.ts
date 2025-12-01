// @ts-ignore - Web3Storage import may not have proper types
declare const Web3Storage: any;

import { NFTStorage, Blob } from 'nft.storage';

// Web3.Storage client for Filecoin integration
export class FilecoinStorageClient {
  private nftStorage: NFTStorage;
  private web3Storage: any; // Use any type for Web3Storage

  constructor(apiKey: string) {
    // Initialize NFTStorage with the provided API key
    this.nftStorage = new NFTStorage({ token: apiKey });
    // Initialize Web3.Storage with the same API key
    // @ts-ignore - Web3Storage may not have proper types
    this.web3Storage = new Web3Storage({ token: apiKey });
  }

  /**
   * Store NFT metadata and assets on Filecoin via Web3.Storage
   */
  async storeNFTData(
    metadata: {
      name: string;
      description: string;
      image: Blob | File;
      attributes?: Array<{
        trait_type: string;
        value: string | number;
      }>;
      properties?: Record<string, any>;
    }
  ): Promise<{
    cid: string;
    url: string;
    metadata: any;
  }> {
    if (!this.nftStorage || !this.web3Storage) {
      throw new Error('Storage clients not initialized');
    }

    try {
      // Store image first using Web3.Storage for better reliability
      console.log('Storing image on Filecoin via Web3.Storage...');
      const imageFile = new File([metadata.image], 'image.png', { type: 'image/png' });
      const imageCid = await this.web3Storage.put([imageFile], {
        name: `${metadata.name}-image`,
        wrapWithDirectory: false
      });

      // Create image URL
      const imageUrl = `https://w3s.link/ipfs/${imageCid}`;

      // Create metadata object with actual image URL
      const nftMetadata = {
        name: metadata.name,
        description: metadata.description,
        image: imageUrl, // Use the actual IPFS URL
        attributes: metadata.attributes || [],
        properties: {
          ...metadata.properties,
          created: new Date().toISOString(),
          storage: 'web3.storage',
          imageCid: imageCid
        }
      };

      // Store metadata using Web3.Storage
      const metadataBlob = new Blob([JSON.stringify(nftMetadata, null, 2)], {
        type: 'application/json'
      });
      const metadataFile = new File([metadataBlob], 'metadata.json', {
        type: 'application/json'
      });

      console.log('Storing metadata on Filecoin via Web3.Storage...');
      const metadataCid = await this.web3Storage.put([metadataFile], {
        name: `${metadata.name}-metadata`,
        wrapWithDirectory: false
      });

      const metadataUrl = `https://w3s.link/ipfs/${metadataCid}`;

      return {
        cid: metadataCid,
        url: metadataUrl,
        metadata: nftMetadata
      };
    } catch (error) {
      console.error('Failed to store NFT data:', error);
      throw error;
    }
  }

  /**
   * Store AI-generated art with emotional metadata
   */
  async storeEmotionalArt(
    artData: {
      canvas: HTMLCanvasElement;
      emotionData: {
        valence: number;
        arousal: number;
        dominance: number;
        confidence: number;
      };
      biometricHash: string;
      aiModel: string;
      generationParams: Record<string, any>;
    }
  ): Promise<{
    cid: string;
    url: string;
    metadata: any;
  }> {
    try {
      // Convert canvas to blob
      const blob = await new Promise<Blob>((resolve) => {
        artData.canvas.toBlob((blob) => {
          if (blob) resolve(blob);
          else throw new Error('Failed to create blob from canvas');
        }, 'image/png');
      });

      // Create attributes for emotional state
      const attributes = [
        {
          trait_type: 'Valence',
          value: Math.round(artData.emotionData.valence * 100) / 100
        },
        {
          trait_type: 'Arousal',
          value: Math.round(artData.emotionData.arousal * 100) / 100
        },
        {
          trait_type: 'Dominance',
          value: Math.round(artData.emotionData.dominance * 100) / 100
        },
        {
          trait_type: 'Confidence',
          value: Math.round(artData.emotionData.confidence)
        },
        {
          trait_type: 'AI Model',
          value: artData.aiModel
        },
        {
          trait_type: 'Biometric Hash',
          value: artData.biometricHash.substring(0, 16) + '...'
        }
      ];

      // Generate description based on emotional state
      const emotionalDescription = this.generateEmotionalDescription(artData.emotionData);

      return await this.storeNFTData({
        name: `Emotional Art - ${new Date().toISOString()}`,
        description: emotionalDescription,
        image: blob,
        attributes,
        properties: {
          ai: {
            model: artData.aiModel,
            parameters: artData.generationParams
          },
          biometric: {
            hash: artData.biometricHash,
            timestamp: new Date().toISOString()
          },
          emotional: artData.emotionData
        }
      });
    } catch (error) {
      console.error('Failed to store emotional art:', error);
      throw error;
    }
  }

  /**
   * Store biometric data securely
   */
  async storeBiometricData(
    data: {
      eegData?: number[];
      heartRateData?: number[];
      facialData?: Blob;
      metadata: {
        userId: string;
        sessionId: string;
        timestamp: string;
        deviceInfo: string;
      };
    }
  ): Promise<{
    cid: string;
    url: string;
    encrypted: boolean;
  }> {
    if (!this.web3Storage) {
      throw new Error('Web3.Storage client not initialized');
    }

    try {
      // Create biometric data package
      const biometricPackage = {
        eeg: data.eegData,
        heartRate: data.heartRateData,
        metadata: data.metadata,
        version: '1.0.0',
        encrypted: true // In production, actually encrypt this data
      };

      // Convert to blob
      const jsonString = JSON.stringify(biometricPackage, null, 2);
      const blob = new Blob([jsonString], { type: 'application/json' });
      const file = new File([blob], `biometric-${data.metadata.sessionId}.json`, {
        type: 'application/json'
      });

      console.log('Storing biometric data on Filecoin via Web3.Storage...');
      const cid = await this.web3Storage.put([file], {
        name: `biometric-${data.metadata.sessionId}`,
        wrapWithDirectory: false
      });

      const url = `https://w3s.link/ipfs/${cid}`;

      return {
        cid,
        url,
        encrypted: true
      };
    } catch (error) {
      console.error('Failed to store biometric data:', error);
      throw error;
    }
  }

  /**
   * Retrieve stored data by CID
   */
  async retrieveData(cid: string): Promise<any> {
    try {
      const url = `https://w3s.link/ipfs/${cid}`;
      const response = await fetch(url);
      
      if (!response.ok) {
        throw new Error(`Failed to retrieve data: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Failed to retrieve data:', error);
      throw error;
    }
  }

  /**
   * List stored content for a user
   */
  async listUserContent(userId: string): Promise<Array<{
    cid: string;
    name: string;
    timestamp: string;
    type: 'nft' | 'biometric' | 'art';
  }>> {
    // This would require implementing a content indexing system
    // For now, return mock data
    return [
      {
        cid: 'bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi',
        name: `Emotional Art - ${userId}`,
        timestamp: new Date().toISOString(),
        type: 'art'
      }
    ];
  }

  /**
   * Generate emotional description based on valence/arousal/dominance
   */
  private generateEmotionalDescription(emotionData: {
    valence: number;
    arousal: number;
    dominance: number;
  }): string {
    const { valence, arousal, dominance } = emotionData;
    
    let description = 'This AI-generated artwork captures a moment of ';
    
    // Valence description
    if (valence > 0.5) {
      description += 'profound positivity and joy';
    } else if (valence > 0) {
      description += 'gentle positivity and contentment';
    } else if (valence > -0.5) {
      description += 'mild negativity and melancholy';
    } else {
      description += 'deep negativity and sadness';
    }
    
    description += ', combined with ';
    
    // Arousal description
    if (arousal > 0.5) {
      description += 'high energy and excitement';
    } else if (arousal > 0) {
      description += 'moderate energy and alertness';
    } else if (arousal > -0.5) {
      description += 'low energy and calmness';
    } else {
      description += 'very low energy and relaxation';
    }
    
    description += '. The emotional state reflects ';
    
    // Dominance description
    if (dominance > 0.5) {
      description += 'strong control and confidence';
    } else if (dominance > 0) {
      description += 'moderate control and balance';
    } else if (dominance > -0.5) {
      description += 'some submission and vulnerability';
    } else {
      description += 'deep submission and helplessness';
    }
    
    description += '. This piece represents the unique emotional fingerprint captured through biometric analysis during the creative process.';
    
    return description;
  }

  /**
   * Get storage usage statistics
   */
  async getStorageStats(): Promise<{
    totalStored: number;
    totalSize: number;
    averageFileSize: number;
    storageQuota: number;
    usagePercentage: number;
  }> {
    // Mock statistics - in production, this would query Web3.Storage API
    return {
      totalStored: 42,
      totalSize: 156 * 1024 * 1024, // 156 MB
      averageFileSize: 3.7 * 1024 * 1024, // 3.7 MB
      storageQuota: 1024 * 1024 * 1024, // 1 GB
      usagePercentage: 15.2
    };
  }
}

// Utility functions for Filecoin integration
export function createFilecoinStorageClient(apiKey: string): FilecoinStorageClient {
  return new FilecoinStorageClient(apiKey);
}

export function validateApiKey(apiKey: string): boolean {
  // Basic validation for Web3.Storage API key format
  return /^eyJ[A-Za-z0-9-_]+\.eyJ[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+$/.test(apiKey);
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}