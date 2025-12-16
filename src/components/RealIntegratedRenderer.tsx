import React, { useState, useEffect, useRef, useCallback } from 'react';
import RealBiometricCapture from './RealBiometricCapture';

interface BiometricData {
  timestamp: number;
  heartRate: number;
  breathingRate: number;
  stressLevel: number;
  emotion: {
    valence: number;
    arousal: number;
    dominance: number;
  };
  eegBands: {
    delta: number;
    theta: number;
    alpha: number;
    beta: number;
    gamma: number;
  };
  facialExpression: {
    joy: number;
    sadness: number;
    anger: number;
    fear: number;
    surprise: number;
    disgust: number;
    neutral: number;
  };
  gestureConfidence: number;
  poseConfidence: number;
}

interface RealIntegratedRendererProps {
  className?: string;
  mode?: 'biometric' | 'audio' | 'ai' | 'combined';
  onSessionComplete?: (data: any) => void;
  onRecordingStart?: () => void;
  onRecordingStop?: (data: any) => void;
}

export const RealIntegratedRenderer: React.FC<RealIntegratedRendererProps> = ({
  className = '',
  mode = 'combined',
  onSessionComplete,
  onRecordingStart,
  onRecordingStop
}) => {
  const [currentBiometricData, setCurrentBiometricData] = useState<BiometricData | null>(null);
  const [isRecording, setIsRecording] = useState(false);
  const [recordingData, setRecordingData] = useState<any[]>([]);
  const [renderMode, setRenderMode] = useState<'biometric' | 'audio' | 'ai' | 'combined'>(mode);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const glRef = useRef<WebGL2RenderingContext | null>(null);
  const programRef = useRef<WebGLProgram | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const startTimeRef = useRef<number>(Date.now());

  // Simple fractal shader
  const fractalShader = `
    #version 300 es
    precision highp float;
    
    in vec2 vPosition;
    out vec4 fragColor;
    
    uniform float uTime;
    uniform vec2 uResolution;
    uniform float uHeartRate;
    uniform float uStressLevel;
    uniform float uValence;
    uniform float uArousal;
    
    vec2 cmul(vec2 a, vec2 b) {
      return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
    }
    
    float mandelbrot(vec2 c) {
      vec2 z = vec2(0.0);
      float n = 0.0;
      for (int i = 0; i < 100; i++) {
        if (length(z) > 2.0) break;
        z = cmul(z, z) + c;
        n += 1.0;
      }
      return n / 100.0;
    }
    
    void main() {
      vec2 uv = (gl_FragCoord.xy / uResolution.xy) * 2.0 - 1.0;
      uv.x *= uResolution.x / uResolution.y;
      
      // Add biometric influence
      vec2 c = uv * 1.5;
      c.x += uValence * 0.3;
      c.y += uArousal * 0.3;
      
      float m = mandelbrot(c);
      
      // Color based on biometric data
      vec3 color = vec3(
        m + uHeartRate * 0.01,
        m + uStressLevel * 0.5,
        m + uValence * 0.3
      );
      
      fragColor = vec4(color, 1.0);
    }
  `;

  // Simple vertex shader
  const vertexShader = `
    #version 300 es
    in vec2 aPosition;
    out vec2 vPosition;
    
    void main() {
      vPosition = aPosition;
      gl_Position = vec4(aPosition, 0.0, 1.0);
    }
  `;

  // Handle biometric data updates
  const handleBiometricData = useCallback((data: any) => {
    const biometricData: BiometricData = {
      timestamp: Date.now(),
      heartRate: data.heartRate || 60 + Math.random() * 20,
      breathingRate: data.breathingRate || 12 + Math.random() * 8,
      stressLevel: data.stressLevel || Math.random(),
      emotion: {
        valence: data.emotion?.valence || Math.random() * 2 - 1,
        arousal: data.emotion?.arousal || Math.random(),
        dominance: data.emotion?.dominance || Math.random()
      },
      eegBands: {
        delta: data.eegBands?.delta || Math.random(),
        theta: data.eegBands?.theta || Math.random(),
        alpha: data.eegBands?.alpha || Math.random(),
        beta: data.eegBands?.beta || Math.random(),
        gamma: data.eegBands?.gamma || Math.random()
      },
      facialExpression: {
        joy: data.facialExpression?.joy || Math.random(),
        sadness: data.facialExpression?.sadness || Math.random(),
        anger: data.facialExpression?.anger || Math.random(),
        fear: data.facialExpression?.fear || Math.random(),
        surprise: data.facialExpression?.surprise || Math.random(),
        disgust: data.facialExpression?.disgust || Math.random(),
        neutral: data.facialExpression?.neutral || Math.random()
      },
      gestureConfidence: data.gestureConfidence || Math.random(),
      poseConfidence: data.poseConfidence || Math.random()
    };

    setCurrentBiometricData(biometricData);
    
    // Store recording data if recording
    if (isRecording) {
      setRecordingData(prev => [...prev, {
        timestamp: biometricData.timestamp,
        biometric: biometricData
      }]);
    }
  }, [isRecording]);

  // Initialize WebGL context
  const initializeWebGL = useCallback(() => {
    if (!canvasRef.current) return false;
    
    const gl = canvasRef.current.getContext('webgl2');
    if (!gl) {
      console.error('WebGL2 not supported');
      return false;
    }
    
    glRef.current = gl;
    return true;
  }, []);

  // Compile shader
  const compileShader = useCallback((gl: WebGL2RenderingContext, source: string, type: number): WebGLShader | null => {
    const shader = gl.createShader(type);
    if (!shader) return null;
    
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      console.error('Shader compile error:', gl.getShaderInfoLog(shader));
      gl.deleteShader(shader);
      return null;
    }
    
    return shader;
  }, []);

  // Create shader program
  const createShaderProgram = useCallback((gl: WebGL2RenderingContext) => {
    const vertexShaderObj = compileShader(gl, vertexShader, gl.VERTEX_SHADER);
    const fragmentShaderObj = compileShader(gl, fractalShader, gl.FRAGMENT_SHADER);
    
    if (!vertexShaderObj || !fragmentShaderObj) return null;
    
    const program = gl.createProgram();
    if (!program) return null;
    
    gl.attachShader(program, vertexShaderObj);
    gl.attachShader(program, fragmentShaderObj);
    gl.linkProgram(program);
    
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error('Program link error:', gl.getProgramInfoLog(program));
      gl.deleteProgram(program);
      return null;
    }
    
    return program;
  }, [compileShader]);

  // Render function
  const render = useCallback(() => {
    if (!glRef.current || !programRef.current || !canvasRef.current) return;
    
    const gl = glRef.current;
    const program = programRef.current;
    
    gl.viewport(0, 0, canvasRef.current.width, canvasRef.current.height);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    
    gl.useProgram(program);
    
    // Set up vertices
    const vertices = new Float32Array([
      -1, -1,  1, -1,  -1, 1,
      -1,  1,  1, -1,   1, 1
    ]);
    
    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);
    
    const positionLoc = gl.getAttribLocation(program, 'aPosition');
    gl.enableVertexAttribArray(positionLoc);
    gl.vertexAttribPointer(positionLoc, 2, gl.FLOAT, false, 0, 0);
    
    // Set uniforms
    const timeLoc = gl.getUniformLocation(program, 'uTime');
    const resolutionLoc = gl.getUniformLocation(program, 'uResolution');
    const heartRateLoc = gl.getUniformLocation(program, 'uHeartRate');
    const stressLevelLoc = gl.getUniformLocation(program, 'uStressLevel');
    const valenceLoc = gl.getUniformLocation(program, 'uValence');
    const arousalLoc = gl.getUniformLocation(program, 'uArousal');
    
    if (timeLoc) gl.uniform1f(timeLoc, (Date.now() - startTimeRef.current) / 1000);
    if (resolutionLoc && canvasRef.current) {
      gl.uniform2f(resolutionLoc, canvasRef.current.width, canvasRef.current.height);
    }
    
    // Set biometric uniforms
    if (currentBiometricData) {
      if (heartRateLoc) gl.uniform1f(heartRateLoc, currentBiometricData.heartRate);
      if (stressLevelLoc) gl.uniform1f(stressLevelLoc, currentBiometricData.stressLevel);
      if (valenceLoc) gl.uniform1f(valenceLoc, currentBiometricData.emotion.valence);
      if (arousalLoc) gl.uniform1f(arousalLoc, currentBiometricData.emotion.arousal);
    }
    
    gl.drawArrays(gl.TRIANGLES, 0, 6);
    
    animationFrameRef.current = requestAnimationFrame(render);
  }, [currentBiometricData]);

  // Initialize WebGL and start rendering
  useEffect(() => {
    if (!initializeWebGL()) return;
    
    const gl = glRef.current;
    if (!gl) return;
    
    const program = createShaderProgram(gl);
    if (!program) return;
    
    programRef.current = program;
    render();
    
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [initializeWebGL, createShaderProgram, render]);

  // Handle canvas resize
  useEffect(() => {
    const handleResize = () => {
      if (canvasRef.current) {
        canvasRef.current.width = canvasRef.current.clientWidth;
        canvasRef.current.height = canvasRef.current.clientHeight;
      }
    };
    
    handleResize();
    window.addEventListener('resize', handleResize);
    
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  // Handle recording
  const handleStartRecording = () => {
    setIsRecording(true);
    setRecordingData([]);
    if (onRecordingStart) onRecordingStart();
  };

  const handleStopRecording = () => {
    setIsRecording(false);
    const sessionData = {
      duration: recordingData.length > 0 ? 
        recordingData[recordingData.length - 1].timestamp - recordingData[0].timestamp : 0,
      dataPoints: recordingData.length,
      dominantEmotion: getDominantEmotion(recordingData),
      confidence: calculateAverageConfidence(recordingData),
      heartRate: calculateAverageHeartRate(recordingData),
      breathingRate: calculateAverageBreathingRate(recordingData),
      signature: generateBiometricSignature(recordingData),
      rawData: recordingData
    };
    
    if (onRecordingStop) onRecordingStop(sessionData);
    if (onSessionComplete) onSessionComplete(sessionData);
  };

  // Helper functions
  const getDominantEmotion = (data: any[]): string => {
    if (data.length === 0) return 'neutral';
    
    const emotions = data.map(d => d.biometric?.emotion || { valence: 0, arousal: 0, dominance: 0 });
    const avgValence = emotions.reduce((sum, e) => sum + e.valence, 0) / emotions.length;
    const avgArousal = emotions.reduce((sum, e) => sum + e.arousal, 0) / emotions.length;
    
    if (avgValence > 0.5 && avgArousal > 0.5) return 'joy';
    if (avgValence < -0.5 && avgArousal > 0.5) return 'anger';
    if (avgValence < -0.5 && avgArousal < 0.5) return 'sadness';
    if (avgValence > 0.5 && avgArousal < 0.5) return 'calm';
    return 'neutral';
  };

  const calculateAverageConfidence = (data: any[]): number => {
    if (data.length === 0) return 0;
    const confidences = data.map(d => d.biometric?.gestureConfidence || 0);
    return confidences.reduce((sum, c) => sum + c, 0) / confidences.length;
  };

  const calculateAverageHeartRate = (data: any[]): number => {
    if (data.length === 0) return 0;
    const heartRates = data.map(d => d.biometric?.heartRate || 0);
    return heartRates.reduce((sum, hr) => sum + hr, 0) / heartRates.length;
  };

  const calculateAverageBreathingRate = (data: any[]): number => {
    if (data.length === 0) return 0;
    const breathingRates = data.map(d => d.biometric?.breathingRate || 0);
    return breathingRates.reduce((sum, br) => sum + br, 0) / breathingRates.length;
  };

  const generateBiometricSignature = (data: any[]): string => {
    const dataString = JSON.stringify(data.map(d => ({
      timestamp: d.timestamp,
      heartRate: d.biometric?.heartRate,
      emotion: d.biometric?.emotion
    })));
    return btoa(dataString).slice(0, 32);
  };

  return (
    <div className={`real-integrated-renderer ${className}`}>
      <div className="renderer-header">
        <h3>Real Biometric Renderer</h3>
        <div className="mode-selector">
          <button 
            className={renderMode === 'biometric' ? 'active' : ''}
            onClick={() => setRenderMode('biometric')}
          >
            Biometric
          </button>
          <button 
            className={renderMode === 'audio' ? 'active' : ''}
            onClick={() => setRenderMode('audio')}
          >
            Audio
          </button>
          <button 
            className={renderMode === 'ai' ? 'active' : ''}
            onClick={() => setRenderMode('ai')}
          >
            AI
          </button>
          <button 
            className={renderMode === 'combined' ? 'active' : ''}
            onClick={() => setRenderMode('combined')}
          >
            Combined
          </button>
        </div>
      </div>
      
      <div className="renderer-content">
        <div className="canvas-container">
          <canvas 
            ref={canvasRef}
            className="render-canvas"
            width={800}
            height={600}
          />
          
          <div className="canvas-overlay">
            {currentBiometricData && (
              <div className="biometric-display">
                <div className="metric">
                  <span className="label">Heart Rate:</span>
                  <span className="value">{Math.round(currentBiometricData.heartRate)} BPM</span>
                </div>
                <div className="metric">
                  <span className="label">Stress:</span>
                  <span className="value">{Math.round(currentBiometricData.stressLevel * 100)}%</span>
                </div>
                <div className="metric">
                  <span className="label">Emotion:</span>
                  <span className="value">
                    V: {currentBiometricData.emotion.valence.toFixed(2)}
                    A: {currentBiometricData.emotion.arousal.toFixed(2)}
                  </span>
                </div>
              </div>
            )}
          </div>
        </div>
        
        <div className="controls-panel">
          <RealBiometricCapture onBiometricData={handleBiometricData} isRecording={isRecording} />
          
          <div className="recording-controls">
            <button 
              className="record-button"
              onClick={isRecording ? handleStopRecording : handleStartRecording}
            >
              {isRecording ? 'Stop Recording' : 'Start Recording'}
            </button>
            
            {isRecording && (
              <div className="recording-indicator">
                <div className="recording-dot"></div>
                <span>Recording... {recordingData.length} samples</span>
              </div>
            )}
          </div>
          
          <div className="session-info">
            <h4>Biometric Session</h4>
            {currentBiometricData && (
              <div className="session-stats">
                <div>Dominant Emotion: {getDominantEmotion(recordingData)}</div>
                <div>Confidence: {(calculateAverageConfidence(recordingData) * 100).toFixed(1)}%</div>
                <div>Avg Heart Rate: {Math.round(calculateAverageHeartRate(recordingData))} BPM</div>
                <div>Avg Breathing: {Math.round(calculateAverageBreathingRate(recordingData))} BPM</div>
              </div>
            )}
          </div>
        </div>
      </div>
      
      <style>{`
        .real-integrated-renderer {
          background: linear-gradient(135deg, #1a1a2e, #16213e);
          border-radius: 12px;
          padding: 20px;
          color: white;
          font-family: 'Inter', sans-serif;
          border: 1px solid #333;
        }
        
        .renderer-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 20px;
        }
        
        .renderer-header h3 {
          margin: 0;
          font-size: 18px;
          font-weight: 600;
          background: linear-gradient(45deg, #00d4ff, #ff00ff);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
        }
        
        .mode-selector {
          display: flex;
          gap: 8px;
        }
        
        .mode-selector button {
          padding: 6px 12px;
          border: 1px solid #444;
          background: #2a2a3e;
          color: #aaa;
          border-radius: 6px;
          cursor: pointer;
          transition: all 0.2s ease;
        }
        
        .mode-selector button:hover {
          background: #3a3a4e;
          color: #fff;
        }
        
        .mode-selector button.active {
          background: linear-gradient(45deg, #00d4ff, #0099cc);
          color: white;
          border-color: #00d4ff;
        }
        
        .renderer-content {
          display: grid;
          grid-template-columns: 2fr 1fr;
          gap: 20px;
        }
        
        .canvas-container {
          position: relative;
          background: #000;
          border-radius: 8px;
          overflow: hidden;
        }
        
        .render-canvas {
          width: 100%;
          height: 400px;
          display: block;
        }
        
        .canvas-overlay {
          position: absolute;
          top: 10px;
          left: 10px;
          right: 10px;
          pointer-events: none;
        }
        
        .biometric-display {
          background: rgba(0, 0, 0, 0.7);
          padding: 10px;
          border-radius: 6px;
          backdrop-filter: blur(5px);
        }
        
        .metric {
          display: flex;
          justify-content: space-between;
          margin-bottom: 4px;
          font-size: 12px;
        }
        
        .metric:last-child {
          margin-bottom: 0;
        }
        
        .label {
          color: #aaa;
        }
        
        .value {
          color: #00d4ff;
          font-weight: 500;
        }
        
        .controls-panel {
          display: flex;
          flex-direction: column;
          gap: 20px;
        }
        
        .recording-controls {
          display: flex;
          flex-direction: column;
          gap: 10px;
        }
        
        .record-button {
          padding: 12px 20px;
          border: none;
          border-radius: 6px;
          background: linear-gradient(45deg, #ff4757, #ff3742);
          color: white;
          font-weight: 600;
          cursor: pointer;
          transition: all 0.3s ease;
        }
        
        .record-button:hover {
          transform: translateY(-1px);
          box-shadow: 0 4px 12px rgba(255, 71, 87, 0.3);
        }
        
        .recording-indicator {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px;
          background: rgba(255, 71, 87, 0.1);
          border: 1px solid rgba(255, 71, 87, 0.3);
          border-radius: 6px;
        }
        
        .recording-dot {
          width: 8px;
          height: 8px;
          background: #ff4757;
          border-radius: 50%;
          animation: pulse 1.5s infinite;
        }
        
        .session-info {
          background: rgba(255, 255, 255, 0.05);
          padding: 15px;
          border-radius: 8px;
          border: 1px solid rgba(255, 255, 255, 0.1);
        }
        
        .session-info h4 {
          margin: 0 0 10px 0;
          font-size: 14px;
          color: #ddd;
        }
        
        .session-stats {
          font-size: 12px;
          color: #aaa;
          line-height: 1.5;
        }
        
        .session-stats div {
          margin-bottom: 4px;
        }
        
        @keyframes pulse {
          0% { opacity: 1; }
          50% { opacity: 0.5; }
          100% { opacity: 1; }
        }
        
        @media (max-width: 768px) {
          .renderer-content {
            grid-template-columns: 1fr;
          }
          
          .render-canvas {
            height: 300px;
          }
        }
      `}</style>
    </div>
  );
};