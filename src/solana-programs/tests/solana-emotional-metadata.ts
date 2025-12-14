import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { BiometricNft } from "../target/types/biometric_nft";
import { EmotionalMetadata } from "../target/types/emotional_metadata";

describe("solana-emotional-metadata", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const biometricProgram = anchor.workspace.BiometricNft as Program<BiometricNft>;
  const emotionalProgram = anchor.workspace.EmotionalMetadata as Program<EmotionalMetadata>;

  const authority = Keypair.generate();
  const owner = Keypair.generate();
  const nftMint = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await biometricProgram.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  describe("Biometric NFT Tests", () => {
    let collectionPDA: PublicKey;
    let nftPDA: PublicKey;

    it("Should initialize biometric collection", async () => {
      const [collection] = PublicKey.findProgramAddressSync(
        [Buffer.from("collection"), authority.publicKey.toBuffer()],
        biometricProgram.programId
      );
      collectionPDA = collection;

      const tx = await biometricProgram.methods
        .initializeCollection(
          "Emotional Biometric Collection",
          "EBC",
          "https://example.com/collection.json"
        )
        .accounts({
          collection: collectionPDA,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("Collection initialized:", tx);

      const collectionAccount = await biometricProgram.account.biometricCollection.fetch(collectionPDA);
      expect(collectionAccount.name).to.equal("Emotional Biometric Collection");
      expect(collectionAccount.symbol).to.equal("EBC");
      expect(collectionAccount.totalSupply.toString()).to.equal("0");
    });

    it("Should mint biometric NFT with emotion data", async () => {
      const biometricHash = Array.from({length: 32}, () => Math.floor(Math.random() * 256));
      const emotionData = {
        valence: 0.7,
        arousal: 0.6,
        dominance: 0.8,
        confidence: 0.85,
        timestamp: new anchor.BN(Math.floor(Date.now() / 1000)),
      };

      const [nft] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("nft"),
          collectionPDA.toBuffer(),
          Buffer.from([0]), // First NFT
        ],
        biometricProgram.programId
      );
      nftPDA = nft;

      const tx = await biometricProgram.methods
        .mintBiometricNft(
          biometricHash,
          emotionData,
          "https://example.com/nft/1.json"
        )
        .accounts({
          nft: nftPDA,
          collection: collectionPDA,
          owner: owner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([owner])
        .rpc();

      console.log("NFT minted:", tx);

      const nftAccount = await biometricProgram.account.biometricNft.fetch(nftPDA);
      expect(nftAccount.biometricHash).to.deep.equal(biometricHash);
      expect(nftAccount.emotionData.valence).to.equal(0.7);
      expect(nftAccount.generation.toString()).to.equal("1");
    });

    it("Should update emotion state", async () => {
      const newEmotionData = {
        valence: 0.8,
        arousal: 0.5,
        dominance: 0.9,
        confidence: 0.9,
        timestamp: new anchor.BN(Math.floor(Date.now() / 1000)),
      };

      const tx = await biometricProgram.methods
        .updateEmotionState(newEmotionData)
        .accounts({
          nft: nftPDA,
          owner: owner.publicKey,
        })
        .signers([owner])
        .rpc();

      console.log("Emotion updated:", tx);

      const nftAccount = await biometricProgram.account.biometricNft.fetch(nftPDA);
      expect(nftAccount.emotionData.valence).to.equal(0.8);
      expect(nftAccount.emotionData.arousal).to.equal(0.5);
    });

    it("Should transfer NFT with emotional validation", async () => {
      const newOwner = Keypair.generate();

      // This should succeed because arousal < 0.7
      const tx = await biometricProgram.methods
        .transferNft(newOwner.publicKey)
        .accounts({
          nft: nftPDA,
          currentOwner: owner.publicKey,
        })
        .signers([owner])
        .rpc();

      console.log("NFT transferred:", tx);

      const nftAccount = await biometricProgram.account.biometricNft.fetch(nftPDA);
      expect(nftAccount.owner.toString()).to.equal(newOwner.publicKey.toString());
    });
  });

  describe("Emotional Metadata Tests", () => {
    let registryPDA: PublicKey;
    let metadataPDA: PublicKey;

    it("Should initialize emotional registry", async () => {
      const [registry] = PublicKey.findProgramAddressSync(
        [Buffer.from("registry"), authority.publicKey.toBuffer()],
        emotionalProgram.programId
      );
      registryPDA = registry;

      const tx = await emotionalProgram.methods
        .initializeRegistry()
        .accounts({
          registry: registryPDA,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("Registry initialized:", tx);

      const registryAccount = await emotionalProgram.account.emotionalRegistry.fetch(registryPDA);
      expect(registryAccount.totalRecords.toString()).to.equal("0");
      expect(registryAccount.authority.toString()).to.equal(authority.publicKey.toString());
    });

    it("Should store emotional metadata", async () => {
      const emotionVector = {
        valence: 0.7,
        arousal: 0.6,
        dominance: 0.8,
      };
      const biometricSignature = Array.from({length: 64}, () => Math.floor(Math.random() * 256));
      const aiConfidence = 0.85;

      const [metadata] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          nftMint.publicKey.toBuffer(),
          Buffer.from([0]),
        ],
        emotionalProgram.programId
      );
      metadataPDA = metadata;

      const tx = await emotionalProgram.methods
        .storeEmotionalMetadata(
          nftMint.publicKey,
          emotionVector,
          biometricSignature,
          aiConfidence
        )
        .accounts({
          metadata: metadataPDA,
          registry: registryPDA,
          owner: owner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([owner])
        .rpc();

      console.log("Emotional metadata stored:", tx);

      const metadataAccount = await emotionalProgram.account.emotionalMetadata.fetch(metadataPDA);
      expect(metadataAccount.nftMint.toString()).to.equal(nftMint.publicKey.toString());
      expect(metadataAccount.emotionVector.valence).to.equal(0.7);
      expect(metadataAccount.aiConfidence).to.equal(0.85);
    });

    it("Should analyze emotional patterns", async () => {
      const nftMints = [nftMint.publicKey];

      const analysis = await emotionalProgram.methods
        .analyzeEmotionalPatterns(nftMints)
        .accounts({
          authority: authority.publicKey,
        })
        .view();

      console.log("Emotional analysis:", analysis);

      expect(analysis.averageValence).to.be.a('number');
      expect(analysis.averageArousal).to.be.a('number');
      expect(analysis.averageDominance).to.be.a('number');
      expect(analysis.emotionalStability).to.be.a('number');
    });
  });
});