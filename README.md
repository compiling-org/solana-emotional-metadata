# Solana Emotional Metadata

This repository contains the Solana Foundation grant implementation for high-performance emotional data tracking with 90%+ compression.

## Project Overview

We propose developing a high-performance Solana program for efficient on-chain storage and validation of real-time creative metadata. Using Solana's State Compression and innovative data structures, this module will record live parameters from creative performances - including emotional states, shader seeds, and performance data - enabling the tokenization of ephemeral live art that was previously impossible to capture on-chain.

## Features

- **Real-time Emotional Data Tracking**: Live recording of emotional states with 90%+ compression
- **State Compression**: Novel algorithms for affective computing data
- **High Throughput**: Thousands of parameter updates per minute
- **Solana Integration**: Native Solana program with Anchor framework
- **Practical Emotional Input**: Multiple input methods including camera-based facial expression analysis

## Getting Started

### Prerequisites

- Rust and Cargo
- Node.js and npm
- Solana CLI
- Anchor framework
- Phantom wallet (for testing)

### Installation

```bash
# Install CLI tools
./scripts/install-cli-tools.sh

# Build the project
./build-solana-grant.sh
```

### Building

```bash
# Build Solana program
cd src/solana-client
anchor build

# Run tests
anchor test
```

### Deployment

1. Deploy program to Solana devnet
2. Update program ID in test-website configuration
3. Serve test-website on a web server

## Practical Emotional Input Methods

Our system implements multiple practical approaches for collecting emotional data that work without specialized hardware:

1. **Manual Input Methods** (Primary Approach)
   - Simple slider controls for valence, arousal, and dominance
   - Works on any device with a browser
   - No special hardware required

2. **Camera-Based Analysis** (Enhancement)
   - Facial expression analysis using standard webcams
   - Local processing for privacy
   - Real-time emotional state detection

3. **Interaction Pattern Analysis** (Passive Collection)
   - Keyboard typing rhythm analysis
   - Mouse movement dynamics
   - Subtle emotional inference from user behavior

4. **Voice Tone Analysis** (Audio Input)
   - Microphone-based emotional inference
   - Local processing for privacy
   - Real-time analysis during creative sessions

5. **EEG/BMI Integration** (Future Enhancement)
   - Compatible with consumer biometric devices
   - Advanced emotional state detection
   - Research-grade precision for specialized applications

## Directory Structure

```
├── src/
│   ├── solana-client/      # Solana program and client code
│   └── rust-client/        # Core Rust library (shared dependency)
├── test-website/           # Browser-based frontend
├── scripts/                # Utility scripts
├── build-solana-grant.sh   # Build script
└── README.md              # This file
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- **Website**: https://compiling-org.netlify.app
- **GitHub**: https://github.com/compiling-org
- **Email**: kapil.bambardekar@gmail.com, vdmo@gmail.com
