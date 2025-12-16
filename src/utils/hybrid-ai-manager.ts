import * as tf from '@tensorflow/tfjs';

export interface BiometricData {
  timestamp: number;
  valence: number;
  arousal: number;
  dominance: number;
  attention: number;
  stress: number;
  eeg: number[];
  audio: number[];
}

export interface EmotionResult {
  valence: number;
  arousal: number;
  dominance: number;
  attention: number;
  stress: number;
  confidence: number;
}

export class HybridAIManager {
  private emotionModel: tf.LayersModel | null = null;
  private eegModel: tf.LayersModel | null = null;
  private audioModel: tf.LayersModel | null = null;
  private isInitialized = false;

  constructor() {
    this.initializeModels();
  }

  private async initializeModels() {
    try {
      // Initialize lightweight emotion detection model
      this.emotionModel = await this.createEmotionModel();
      this.eegModel = await this.createEEGModel();
      this.audioModel = await this.createAudioModel();
      this.isInitialized = true;
      console.log('Hybrid AI Manager initialized successfully');
    } catch (error) {
      console.error('Failed to initialize AI models:', error);
    }
  }

  private createEmotionModel(): tf.LayersModel {
    const model = tf.sequential({
      layers: [
        tf.layers.dense({ inputShape: [10], units: 64, activation: 'relu' }),
        tf.layers.dropout({ rate: 0.2 }),
        tf.layers.dense({ units: 32, activation: 'relu' }),
        tf.layers.dropout({ rate: 0.2 }),
        tf.layers.dense({ units: 16, activation: 'relu' }),
        tf.layers.dense({ units: 5, activation: 'sigmoid' }) // valence, arousal, dominance, attention, stress
      ]
    });

    model.compile({
      optimizer: tf.train.adam(0.001),
      loss: 'meanSquaredError',
      metrics: ['accuracy']
    });

    return model;
  }

  private createEEGModel(): tf.LayersModel {
    const model = tf.sequential({
      layers: [
        tf.layers.conv1d({ inputShape: [128, 1], filters: 32, kernelSize: 3, activation: 'relu' }),
        tf.layers.maxPooling1d({ poolSize: 2 }),
        tf.layers.conv1d({ filters: 64, kernelSize: 3, activation: 'relu' }),
        tf.layers.maxPooling1d({ poolSize: 2 }),
        tf.layers.flatten(),
        tf.layers.dense({ units: 64, activation: 'relu' }),
        tf.layers.dropout({ rate: 0.3 }),
        tf.layers.dense({ units: 10, activation: 'linear' })
      ]
    });

    model.compile({
      optimizer: tf.train.adam(0.001),
      loss: 'meanSquaredError'
    });

    return model;
  }

  private flattenArray(arr: any): number[] {
    if (Array.isArray(arr)) {
      return arr.flat(Infinity);
    }
    return [arr];
  }

  private createAudioModel(): tf.LayersModel {
    const model = tf.sequential({
      layers: [
        tf.layers.conv1d({ inputShape: [1024, 1], filters: 16, kernelSize: 3, activation: 'relu' }),
        tf.layers.maxPooling1d({ poolSize: 2 }),
        tf.layers.conv1d({ filters: 32, kernelSize: 3, activation: 'relu' }),
        tf.layers.maxPooling1d({ poolSize: 2 }),
        tf.layers.flatten(),
        tf.layers.dense({ units: 64, activation: 'relu' }),
        tf.layers.dropout({ rate: 0.3 }),
        tf.layers.dense({ units: 10, activation: 'linear' })
      ]
    });

    model.compile({
      optimizer: tf.train.adam(0.001),
      loss: 'meanSquaredError'
    });

    return model;
  }

  generateSyntheticEEG(): number[] {
    // Generate realistic synthetic EEG data
    const eegData: number[] = [];
    const baseFrequency = 10; // Alpha wave frequency
    const samplingRate = 128; // 128 Hz
    const duration = 1; // 1 second
    
    for (let i = 0; i < samplingRate * duration; i++) {
      const time = i / samplingRate;
      // Simulate alpha waves with some noise
      const alphaWave = Math.sin(2 * Math.PI * baseFrequency * time) * 0.5;
      const noise = (Math.random() - 0.5) * 0.1;
      const betaWave = Math.sin(2 * Math.PI * 20 * time) * 0.2; // Beta waves
      
      eegData.push(alphaWave + betaWave + noise);
    }
    
    return eegData;
  }

  generateSyntheticAudio(): number[] {
    // Generate realistic synthetic audio data
    const audioData: number[] = [];
    const samplingRate = 44100; // 44.1 kHz
    const duration = 0.1; // 100ms
    const frequency = 440; // A4 note
    
    for (let i = 0; i < samplingRate * duration; i++) {
      const time = i / samplingRate;
      // Generate a simple sine wave with harmonics
      const fundamental = Math.sin(2 * Math.PI * frequency * time) * 0.5;
      const harmonic2 = Math.sin(2 * Math.PI * frequency * 2 * time) * 0.3;
      const harmonic3 = Math.sin(2 * Math.PI * frequency * 3 * time) * 0.1;
      const noise = (Math.random() - 0.5) * 0.05;
      
      audioData.push(fundamental + harmonic2 + harmonic3 + noise);
    }
    
    return audioData;
  }

  async detectEmotion(eegData: number[], audioData: number[]): Promise<EmotionResult> {
    if (!this.isInitialized || !this.emotionModel || !this.eegModel || !this.audioModel) {
      // Fallback to synthetic emotion detection
      return this.generateSyntheticEmotion();
    }

    try {
      // Process EEG data
      const eegTensor = tf.tensor3d([eegData.map(val => [val])]);
      const eegFeatures = this.eegModel.predict(eegTensor) as tf.Tensor;
      const eegArray = await eegFeatures.array();

      // Process Audio data
      const audioTensor = tf.tensor3d([audioData.map(val => [val])]);
      const audioFeatures = this.audioModel.predict(audioTensor) as tf.Tensor;
      const audioArray = await audioFeatures.array();

      // Combine features for emotion detection
      const eegFlat = this.flattenArray(eegArray);
      const audioFlat = this.flattenArray(audioArray);
      const combinedFeatures = [...eegFlat, ...audioFlat];
      const emotionTensor = tf.tensor2d([combinedFeatures]);
      const emotionPrediction = this.emotionModel.predict(emotionTensor) as tf.Tensor;
      const emotionArray = await emotionPrediction.array();

      // Clean up tensors
      eegTensor.dispose();
      audioTensor.dispose();
      emotionTensor.dispose();
      eegFeatures.dispose();
      audioFeatures.dispose();
      emotionPrediction.dispose();

      // Handle the emotion array result safely
      const emotionResult = this.flattenArray(emotionArray);

      return {
        valence: Math.max(0, Math.min(1, emotionResult[0] || 0.5)),
        arousal: Math.max(0, Math.min(1, emotionResult[1] || 0.5)),
        dominance: Math.max(0, Math.min(1, emotionResult[2] || 0.5)),
        attention: Math.max(0, Math.min(1, emotionResult[3] || 0.5)),
        stress: Math.max(0, Math.min(1, emotionResult[4] || 0.5)),
        confidence: 0.8
      };
    } catch (error) {
      console.error('Emotion detection failed:', error);
      return this.generateSyntheticEmotion();
    }
  }

  private generateSyntheticEmotion(): EmotionResult {
    // Generate realistic synthetic emotion data
    const timeOfDay = (Date.now() % 86400000) / 86400000; // Normalized time of day
    const circadianFactor = Math.sin(timeOfDay * 2 * Math.PI - Math.PI / 2) * 0.3 + 0.7;
    
    return {
      valence: Math.max(0, Math.min(1, 0.5 + Math.random() * 0.4 - 0.2)),
      arousal: Math.max(0, Math.min(1, 0.6 + Math.random() * 0.3 - 0.15)),
      dominance: Math.max(0, Math.min(1, 0.7 + Math.random() * 0.2 - 0.1)),
      attention: Math.max(0, Math.min(1, 0.8 * circadianFactor + Math.random() * 0.1)),
      stress: Math.max(0, Math.min(1, 0.3 + Math.random() * 0.2 - 0.1)),
      confidence: 0.7
    };
  }

  async analyzeBiometricData(biometricData: BiometricData[]): Promise<{
    averageEmotion: EmotionResult;
    trends: string[];
    anomalies: string[];
  }> {
    if (biometricData.length === 0) {
      return {
        averageEmotion: this.generateSyntheticEmotion(),
        trends: [],
        anomalies: []
      };
    }

    const avgValence = biometricData.reduce((sum, data) => sum + data.valence, 0) / biometricData.length;
    const avgArousal = biometricData.reduce((sum, data) => sum + data.arousal, 0) / biometricData.length;
    const avgDominance = biometricData.reduce((sum, data) => sum + data.dominance, 0) / biometricData.length;
    const avgAttention = biometricData.reduce((sum, data) => sum + data.attention, 0) / biometricData.length;
    const avgStress = biometricData.reduce((sum, data) => sum + data.stress, 0) / biometricData.length;

    const averageEmotion: EmotionResult = {
      valence: avgValence,
      arousal: avgArousal,
      dominance: avgDominance,
      attention: avgAttention,
      stress: avgStress,
      confidence: 0.9
    };

    // Simple trend analysis
    const trends: string[] = [];
    if (biometricData.length > 1) {
      const firstHalf = biometricData.slice(0, Math.floor(biometricData.length / 2));
      const secondHalf = biometricData.slice(Math.floor(biometricData.length / 2));
      
      const firstAvgValence = firstHalf.reduce((sum, data) => sum + data.valence, 0) / firstHalf.length;
      const secondAvgValence = secondHalf.reduce((sum, data) => sum + data.valence, 0) / secondHalf.length;
      
      if (secondAvgValence > firstAvgValence + 0.1) {
        trends.push('Positive emotional trend detected');
      } else if (secondAvgValence < firstAvgValence - 0.1) {
        trends.push('Negative emotional trend detected');
      }

      const firstAvgStress = firstHalf.reduce((sum, data) => sum + data.stress, 0) / firstHalf.length;
      const secondAvgStress = secondHalf.reduce((sum, data) => sum + data.stress, 0) / secondHalf.length;
      
      if (secondAvgStress > firstAvgStress + 0.1) {
        trends.push('Increasing stress levels');
      } else if (secondAvgStress < firstAvgStress - 0.1) {
        trends.push('Decreasing stress levels');
      }
    }

    // Simple anomaly detection
    const anomalies: string[] = [];
    biometricData.forEach((data, index) => {
      if (data.stress > 0.8) {
        anomalies.push(`High stress detected at timestamp ${data.timestamp}`);
      }
      if (data.attention < 0.2) {
        anomalies.push(`Low attention detected at timestamp ${data.timestamp}`);
      }
    });

    return {
      averageEmotion,
      trends,
      anomalies
    };
  }

  dispose() {
    if (this.emotionModel) {
      this.emotionModel.dispose();
    }
    if (this.eegModel) {
      this.eegModel.dispose();
    }
    if (this.audioModel) {
      this.audioModel.dispose();
    }
  }
}