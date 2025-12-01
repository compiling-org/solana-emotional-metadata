// Real Blockchain Configuration for Testnet Deployment
export const BLOCKCHAIN_CONFIG = {
  // NEAR Testnet Configuration
  near: {
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org',
    // Deployed contract addresses
    contracts: {
      soulboundNFT: 'bio-nft-1764175259.sleeplessmonk-testnet-1764175172.testnet'
    }
  },
  
  // Solana Devnet Configuration  
  solana: {
    network: 'devnet',
    rpcUrl: 'https://api.devnet.solana.com',
    webSocketUrl: 'wss://api.devnet.solana.com',
    explorerUrl: 'https://explorer.solana.com/?cluster=devnet',
    // Deployed program addresses
    programs: {
      biometricNFT: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
    }
  },
  
  // Web3.Storage Configuration
  web3Storage: {
    // Replace with real API key when available
    apiKey: process.env.WEB3_STORAGE_API_KEY || 'your-web3-storage-api-key-here',
    endpoint: 'https://api.web3.storage',
    fallbackGateway: 'https://ipfs.io/ipfs/'
  },
  
  // Wallet Configuration
  wallets: {
    near: {
      // NEAR Wallet Selector configuration
      modules: [
        '@near-wallet-selector/near-wallet',
        '@near-wallet-selector/my-near-wallet',
        '@near-wallet-selector/sender',
        '@near-wallet-selector/ledger'
      ]
    },
    solana: {
      // Solana wallet adapters
      wallets: [
        'phantom',
        'solflare',
        'torus',
        'ledger'
      ]
    }
  }
};

// Environment-specific overrides
if (process.env.NODE_ENV === 'production') {
  // Production mainnet configuration
  BLOCKCHAIN_CONFIG.near.networkId = 'mainnet';
  BLOCKCHAIN_CONFIG.near.nodeUrl = 'https://rpc.mainnet.near.org';
  BLOCKCHAIN_CONFIG.near.walletUrl = 'https://wallet.near.org';
  BLOCKCHAIN_CONFIG.near.helperUrl = 'https://helper.mainnet.near.org';
  BLOCKCHAIN_CONFIG.near.explorerUrl = 'https://explorer.near.org';
  
  BLOCKCHAIN_CONFIG.solana.network = 'mainnet-beta';
  BLOCKCHAIN_CONFIG.solana.rpcUrl = 'https://api.mainnet-beta.solana.com';
  BLOCKCHAIN_CONFIG.solana.webSocketUrl = 'wss://api.mainnet-beta.solana.com';
  BLOCKCHAIN_CONFIG.solana.explorerUrl = 'https://explorer.solana.com';
}

export default BLOCKCHAIN_CONFIG;