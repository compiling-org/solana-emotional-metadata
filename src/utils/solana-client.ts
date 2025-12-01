import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';
import { Program, AnchorProvider, web3, BN } from '@project-serum/anchor';
import { createHash } from 'crypto';

// IDL definition inline to avoid import issues
const idl = {
  "version": "0.1.0",
  "name": "biometric_nft",
  "instructions": [
    {
      "name": "initializeNft",
      "accounts": [
        {
          "name": "nftAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "emotionData",
          "type": {
            "defined": "EmotionData"
          }
        },
        {
          "name": "qualityScore",
          "type": "f64"
        },
        {
          "name": "biometricHash",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "NftAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "emotionData",
            "type": {
              "defined": "EmotionData"
            }
          },
          {
            "name": "qualityScore",
            "type": "f64"
          },
          {
            "name": "biometricHash",
            "type": "string"
          },
          {
            "name": "isVerified",
            "type": "bool"
          },
          {
            "name": "createdAt",
            "type": "i64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "EmotionData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "valence",
            "type": "f64"
          },
          {
            "name": "arousal",
            "type": "f64"
          },
          {
            "name": "dominance",
            "type": "f64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "LowQualityScore",
      "msg": "Quality score is too low"
    }
  ],
  "metadata": {
    "address": "3BRGASWgfiPsxwVQq2W6JKLuWvZRBRSd3gkgfeDt9yoA"
  }
};

const PROGRAM_ID = new PublicKey('3BRGASWgfiPsxwVQq2W6JKLuWvZRBRSd3gkgfeDt9yoA');

interface EmotionData {
  valence: number;
  arousal: number;
  dominance: number;
  timestamp?: number;
}

interface NFTAccount {
  owner: PublicKey;
  emotionData: EmotionData;
  qualityScore: number;
  biometricHash: string;
  isVerified: boolean;
  createdAt: BN;
  emotionHistory: EmotionData[];
}

export class BiometricNFTClient {
  private program: Program;
  private connection: Connection;

  constructor(connection: Connection, provider: AnchorProvider) {
    this.connection = connection;
    this.program = new Program(idl as any, PROGRAM_ID, provider);
  }

  // Initialize a new biometric NFT
  async initializeNFT(
    payer: PublicKey,
    emotionData: EmotionData,
    qualityScore: number,
    biometricHash: string
  ): Promise<{ nftAccount: PublicKey; transactionSignature: string }> {
    try {
      // Generate a new NFT account address
      const nftAccount = web3.Keypair.generate();
      
      // Add timestamp if not provided
      const emotionDataWithTimestamp = {
        ...emotionData,
        timestamp: emotionData.timestamp || Date.now()
      };

      // Create the transaction
      const tx = await this.program.methods
        .initializeNft(emotionDataWithTimestamp, qualityScore, biometricHash)
        .accounts({
          nftAccount: nftAccount.publicKey,
          payer: payer,
          systemProgram: SystemProgram.programId,
        })
        .signers([nftAccount])
        .rpc();

      return {
        nftAccount: nftAccount.publicKey,
        transactionSignature: tx
      };
    } catch (error) {
      console.error('Error initializing NFT:', error);
      throw error;
    }
  }

  // Verify biometric data
  async verifyBiometric(
    nftAccount: PublicKey,
    verifier: PublicKey,
    biometricData: string
  ): Promise<string> {
    try {
      const tx = await this.program.methods
        .verifyBiometric(biometricData)
        .accounts({
          nftAccount: nftAccount,
          verifier: verifier,
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Error verifying biometric:', error);
      throw error;
    }
  }

  // Update emotion data
  async updateEmotion(
    nftAccount: PublicKey,
    owner: PublicKey,
    newEmotionData: EmotionData
  ): Promise<string> {
    try {
      const emotionDataWithTimestamp = {
        ...newEmotionData,
        timestamp: newEmotionData.timestamp || Date.now()
      };

      const tx = await this.program.methods
        .updateEmotion(emotionDataWithTimestamp)
        .accounts({
          nftAccount: nftAccount,
          owner: owner,
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Error updating emotion:', error);
      throw error;
    }
  }

  // Fetch NFT account data
  async getNFTAccount(nftAccount: PublicKey): Promise<NFTAccount | null> {
    try {
      const account = await this.program.account.nftAccount.fetch(nftAccount);
      return account as unknown as NFTAccount;
    } catch (error) {
      console.error('Error fetching NFT account:', error);
      return null;
    }
  }

  // Get all NFTs for a specific owner
  async getNFTsByOwner(owner: PublicKey): Promise<PublicKey[]> {
    try {
      const accounts = await this.connection.getProgramAccounts(PROGRAM_ID, {
        filters: [
          {
            memcmp: {
              offset: 8, // Skip discriminator
              bytes: owner.toBase58(),
            },
          },
        ],
      });

      return accounts.map(account => account.pubkey);
    } catch (error) {
      console.error('Error fetching NFTs by owner:', error);
      return [];
    }
  }

  // Calculate emotion quality score
  calculateQualityScore(emotionData: EmotionData): number {
    // Simple quality calculation based on emotion parameters
    const valenceScore = Math.abs(emotionData.valence - 0.5) * 2; // 0-1 range
    const arousalScore = emotionData.arousal; // 0-1 range
    const dominanceScore = emotionData.dominance; // 0-1 range
    
    // Weighted average
    const qualityScore = (valenceScore * 0.4 + arousalScore * 0.3 + dominanceScore * 0.3);
    
    return Math.min(qualityScore, 1.0); // Cap at 1.0
  }

  // Generate biometric hash from emotion data using SHA-256
  generateBiometricHash(emotionData: EmotionData): string {
    // Create a deterministic hash from emotion data and timestamp
    const dataString = `${emotionData.valence}-${emotionData.arousal}-${emotionData.dominance}-${Date.now()}`;
    
    // Use SHA-256 for cryptographic hash
    const hash = createHash('sha256').update(dataString).digest('hex');
    
    return hash;
  }

  // Upload metadata to Arweave/IPFS (mock implementation)
  async uploadMetadata(metadata: any): Promise<string> {
    // In a real implementation, this would upload to Arweave or IPFS
    // For now, we'll return a mock URL
    console.log('Uploading metadata:', metadata);
    
    // Simulate upload delay
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    return `https://arweave.net/mock-transaction-${Date.now()}`;
  }
}

// Helper function to create AnchorProvider
export function createAnchorProvider(connection: Connection, wallet: any): AnchorProvider {
  const provider = new AnchorProvider(
    connection,
    wallet,
    AnchorProvider.defaultOptions()
  );
  return provider;
}

export default BiometricNFTClient;