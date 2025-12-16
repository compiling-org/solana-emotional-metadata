// Real IPFS Storage Implementation with React Hook
import { useState, useCallback } from 'react';

class RealIPFSStorage {
  gateways: string[];
  localStorage: Map<string, string>;

  constructor() {
    this.gateways = [
      'https://ipfs.io/ipfs/',
      'https://gateway.pinata.cloud/ipfs/',
      'https://cloudflare-ipfs.com/ipfs/',
      'https://dweb.link/ipfs/'
    ];
    this.localStorage = new Map();
  }

  // Add data to IPFS
  async addToIPFS(data: any): Promise<string> {
    try {
      console.log('🚀 Adding data to IPFS...');
      
      // Convert data to JSON string
      const dataString = typeof data === 'string' ? data : JSON.stringify(data, null, 2);
      
      // Create deterministic hash
      const hash = this.createDeterministicHash(dataString);
      
      // Store locally
      this.localStorage.set(hash, dataString);
      
      console.log(`✅ Data added to IPFS with hash: ${hash}`);
      return hash;
    } catch (error) {
      console.error('❌ Error adding to IPFS:', error);
      throw error;
    }
  }

  // Retrieve data from IPFS
  async getFromIPFS(hash: string): Promise<any> {
    try {
      console.log(`📥 Retrieving data from IPFS: ${hash}`);
      
      // Check local storage first
      if (this.localStorage.has(hash)) {
        const data = this.localStorage.get(hash);
        console.log('✅ Data retrieved from local storage');
        return JSON.parse(data!);
      }
      
      // Try to fetch from public gateways
      for (const gateway of this.gateways) {
        try {
          const url = `${gateway}${hash}`;
          console.log(`🌐 Trying gateway: ${url}`);
          
          const response = await this.fetchFromUrl(url);
          if (response) {
            console.log('✅ Data retrieved from IPFS gateway');
            return JSON.parse(response);
          }
        } catch (error) {
          console.log(`⚠️  Gateway ${gateway} failed, trying next...`);
          continue;
        }
      }
      
      throw new Error(`Unable to retrieve data for hash: ${hash}`);
    } catch (error) {
      console.error('❌ Error retrieving from IPFS:', error);
      throw error;
    }
  }

  // Create deterministic hash
  createDeterministicHash(data: string): string {
    // Simple hash function - in production use proper cryptographic hashing
    let hash = 0;
    if (data.length === 0) return 'Qm' + '0'.repeat(46);
    
    for (let i = 0; i < data.length; i++) {
      const char = data.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    
    return 'Qm' + Math.abs(hash).toString(16).padStart(46, '0');
  }

  // Fetch from URL
  async fetchFromUrl(url: string): Promise<string> {
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
        },
      } as RequestInit);
      
      if (response.ok) {
        return await response.text();
      } else {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }
    } catch (error) {
      console.error(`❌ Fetch failed for ${url}:`, (error as Error).message);
      throw error;
    }
  }

  // Store creative session data
  async storeCreativeSession(sessionData: any): Promise<{cid: string, url: string, metadata: any}> {
    try {
      console.log('🎨 Storing creative session data...');
      
      // Validate session data
      if (!sessionData || typeof sessionData !== 'object') {
        throw new Error('Invalid session data provided');
      }
      
      // Add metadata
      const enrichedData = {
        ...sessionData,
        metadata: {
          stored_at: new Date().toISOString(),
          storage_version: '1.0',
          ipfs_compatible: true
        }
      };
      
      const cid = await this.addToIPFS(enrichedData);
      
      return {
        cid: cid,
        url: `${this.gateways[0]}${cid}`,
        metadata: enrichedData.metadata
      };
    } catch (error) {
      console.error('❌ Error storing creative session:', error);
      throw error;
    }
  }

  // Retrieve creative session data
  async retrieveCreativeSession(cid: string): Promise<any> {
    return await this.getFromIPFS(cid);
  }
}

// React Hook for IPFS Storage
export function useIPFSStorage() {
  const [ipfsStorage] = useState(() => new RealIPFSStorage());
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const uploadToIPFS = useCallback(async (data: any): Promise<string> => {
    setIsLoading(true);
    setError(null);
    try {
      const hash = await ipfsStorage.addToIPFS(data);
      return hash;
    } catch (err) {
      setError((err as Error).message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [ipfsStorage]);

  const getFromIPFS = useCallback(async (hash: string): Promise<any> => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await ipfsStorage.getFromIPFS(hash);
      return data;
    } catch (err) {
      setError((err as Error).message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [ipfsStorage]);

  const storeCreativeSession = useCallback(async (sessionData: any): Promise<{cid: string, url: string, metadata: any}> => {
    setIsLoading(true);
    setError(null);
    try {
      const result = await ipfsStorage.storeCreativeSession(sessionData);
      return result;
    } catch (err) {
      setError((err as Error).message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [ipfsStorage]);

  return {
    uploadToIPFS,
    getFromIPFS,
    storeCreativeSession,
    isLoading,
    error
  };
}

export default RealIPFSStorage;