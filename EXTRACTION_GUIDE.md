# Solana Creative Engine - Code Extraction Guide

## Overview
This document provides precise instructions for extracting ONLY the Solana-specific code from the main blockchain-nft-interactive project for the Solana Foundation grant repository.

## Critical Extraction Rules
1. **ONLY copy Solana-specific files** - DO NOT copy the entire main project
2. **Maintain proper directory structure** - Follow Solana project conventions
3. **Include all dependencies** - Ensure package.json includes Solana-specific packages
4. **Test functionality** - Verify all copied code works correctly

## Files to Extract

### Core Solana Integration Files
```
src/utils/solana-client.ts                    # Main Solana client
src/utils/solana-enhanced-integration.ts     # Enhanced Solana integration
src/utils/solana-client-enhanced.ts        # Advanced Solana features
```

### Solana Smart Contracts
```
src/solana-program/programs/biometric-nft/src/lib.rs      # Biometric NFT program
src/solana-program/programs/cross-chain-ai/src/lib.rs     # Cross-chain AI program
contracts/solana/biometric-nft/programs/biometric-nft/src/lib.rs # Alternative location
```

### Solana Client Libraries
```
src/solana-client/src/lib.rs               # Solana client Rust library
src/solana-client/src/neuroemotive.rs      # Neuroemotive integration
src/solana-client/src/storage_advanced.rs # Advanced storage features
```

### Supporting Files
```
src/utils/unified-ai-ml-integration.js      # AI/ML bridge (shared dependency)
src/utils/filecoin-storage.ts                # Filecoin storage (for NFT metadata)
```

### Configuration Files
```
package.json                                 # Solana-specific dependencies
Cargo.toml                                   # Rust workspace configuration
Anchor.toml                                  # Anchor framework configuration
```

## Solana-Specific Dependencies
The package.json must include these Solana-specific packages:
```json
{
  "dependencies": {
    "@solana/web3.js": "^1.87.6",
    "@solana/spl-token": "^0.3.9",
    "@project-serum/anchor": "^0.28.0",
    "@coral-xyz/anchor": "^0.28.0"
  }
}
```

## Directory Structure for Solana Grant Repository
```
solana-creative-engine/
├── src/
│   ├── utils/
│   │   ├── solana-client.ts
│   │   ├── solana-enhanced-integration.ts
│   │   ├── solana-client-enhanced.ts
│   │   ├── unified-ai-ml-integration.js
│   │   └── filecoin-storage.ts
│   ├── contracts/
│   │   └── solana/
│   │       ├── biometric-nft/
│   │       │   └── programs/
│   │       │       └── biometric-nft/
│   │       │           └── src/
│   │       │               └── lib.rs
│   │       └── cross-chain-ai/
│   │           └── programs/
│   │               └── cross-chain-ai/
│   │                   └── src/
│   │                       └── lib.rs
│   └── solana-client/
│       └── src/
│           ├── lib.rs
│           ├── neuroemotive.rs
│           └── storage_advanced.rs
├── package.json
├── Cargo.toml
├── Anchor.toml
└── README.md
```

## Testing Instructions
1. Verify Solana program compilation: `anchor build`
2. Test Solana client integration: Check wallet connection and program calls
3. Validate AI/ML integration: Ensure biometric processing works
4. Test file upload to Filecoin: Verify NFT metadata storage

## Deployment Checklist
- [ ] All Solana programs compile successfully
- [ ] Solana wallet integration works
- [ ] AI/ML biometric processing functions correctly
- [ ] Filecoin storage integration operates properly
- [ ] Cross-chain bridge functionality tested
- [ ] All dependencies properly installed

## Critical Reminders
- **DO NOT** copy non-Solana files (NEAR, Polkadot, etc.)
- **DO NOT** copy the entire main project structure
- **ONLY** extract Solana-specific functionality
- **VERIFY** all copied code is Solana-related
- **TEST** functionality after extraction