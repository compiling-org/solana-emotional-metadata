import { useEffect, useRef, useState } from 'react';
// import FractalBlockchainIntegration from '../components/FractalBlockchainIntegration';

interface EmotionalState {
  valence: number;
  arousal: number;
  dominance: number;
}

interface FractalParams {
  fractalType: 'mandelbrot' | 'julia' | 'burning_ship' | 'newton' | 'phoenix';
  zoom: number;
  centerX: number;
  centerY: number;
  maxIterations: number;
  timeOffset: number;
}

export default function AIFractalStudio() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [emotionalState, setEmotionalState] = useState<EmotionalState>({
    valence: 0.5,
    arousal: 0.5,
    dominance: 0.5
  });
  const [fractalParams, setFractalParams] = useState<FractalParams>({
    fractalType: 'mandelbrot',
    zoom: 1.0,
    centerX: -0.5,
    centerY: 0.0,
    maxIterations: 100,
    timeOffset: 0.0
  });
  const [isAnimating, setIsAnimating] = useState(false);
  const [animationSpeed, setAnimationSpeed] = useState(0.01);
  // const [integrationResult, setIntegrationResult] = useState<any>(null); // eslint-disable-line @typescript-eslint/no-explicit-any

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = 800;
    canvas.height = 600;

    // Simple fractal simulation (in real implementation, this would use WASM)
    const drawFractal = () => {
      const imageData = ctx.createImageData(canvas.width, canvas.height);
      const data = imageData.data;

      for (let x = 0; x < canvas.width; x++) {
        for (let y = 0; y < canvas.height; y++) {
          const index = (y * canvas.width + x) * 4;
          
          // Simple fractal-like pattern based on emotional state
          const cx = (x - canvas.width / 2) / (canvas.width / 4) * emotionalState.valence;
          const cy = (y - canvas.height / 2) / (canvas.height / 4) * emotionalState.arousal;
          
          let zx = 0;
          let zy = 0;
          let iterations = 0;
          
          while (zx * zx + zy * zy < 4 && iterations < fractalParams.maxIterations) {
            const tmp = zx * zx - zy * zy + cx;
            zy = 2 * zx * zy + cy;
            zx = tmp;
            iterations++;
          }
          
          const color = iterations / fractalParams.maxIterations;
          const hue = (color * 360 + fractalParams.timeOffset * 50) % 360;
          const saturation = emotionalState.dominance * 100;
          const lightness = color * 70 + 15;
          
          // Convert HSL to RGB
          const c = (1 - Math.abs(2 * lightness / 100 - 1)) * saturation / 100;
          const x1 = c * (1 - Math.abs(((hue / 60) % 2) - 1));
          const m = lightness / 100 - c / 2;
          
          let r = 0, g = 0, b = 0;
          if (hue < 60) { r = c; g = x1; b = 0; }
          else if (hue < 120) { r = x1; g = c; b = 0; }
          else if (hue < 180) { r = 0; g = c; b = x1; }
          else if (hue < 240) { r = 0; g = x1; b = c; }
          else if (hue < 300) { r = x1; g = 0; b = c; }
          else { r = c; g = 0; b = x1; }
          
          data[index] = (r + m) * 255;
          data[index + 1] = (g + m) * 255;
          data[index + 2] = (b + m) * 255;
          data[index + 3] = 255;
        }
      }
      
      ctx.putImageData(imageData, 0, 0);
    };

    drawFractal();
  }, [emotionalState, fractalParams]);

  useEffect(() => {
    if (!isAnimating) return;

    const interval = setInterval(() => {
      setFractalParams(prev => ({
        ...prev,
        timeOffset: prev.timeOffset + animationSpeed
      }));
    }, 50);

    return () => clearInterval(interval);
  }, [isAnimating, animationSpeed]);

  const handleEmotionChange = (param: keyof EmotionalState, value: number) => {
    setEmotionalState(prev => ({ ...prev, [param]: value }));
  };

  const handleFractalParamChange = (param: keyof FractalParams, value: any) => {
    setFractalParams(prev => ({ ...prev, [param]: value }));
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-4xl font-bold mb-8 text-center bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent">
          AI Fractal Studio
        </h1>
        
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Emotional Controls */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">Emotional Parameters</h2>
            
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium mb-2">Valence (Pleasure)</label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  value={emotionalState.valence}
                  onChange={(e) => handleEmotionChange('valence', parseFloat(e.target.value))}
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>Negative</span>
                  <span>Neutral</span>
                  <span>Positive</span>
                </div>
                <p className="text-center mt-1">{emotionalState.valence.toFixed(2)}</p>
              </div>
              
              <div>
                <label className="block text-sm font-medium mb-2">Arousal (Energy)</label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  value={emotionalState.arousal}
                  onChange={(e) => handleEmotionChange('arousal', parseFloat(e.target.value))}
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>Calm</span>
                  <span>Neutral</span>
                  <span>Excited</span>
                </div>
                <p className="text-center mt-1">{emotionalState.arousal.toFixed(2)}</p>
              </div>
              
              <div>
                <label className="block text-sm font-medium mb-2">Dominance (Control)</label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  value={emotionalState.dominance}
                  onChange={(e) => handleEmotionChange('dominance', parseFloat(e.target.value))}
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>Submissive</span>
                  <span>Neutral</span>
                  <span>Dominant</span>
                </div>
                <p className="text-center mt-1">{emotionalState.dominance.toFixed(2)}</p>
              </div>
            </div>
          </div>
          
          {/* Fractal Display */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">Fractal Visualization</h2>
            
            <div className="bg-black rounded-lg p-4 mb-4">
              <canvas
                ref={canvasRef}
                className="w-full h-64 rounded border border-gray-600"
                style={{ maxWidth: '100%', height: 'auto' }}
              />
            </div>
            
            <div className="flex gap-4 mb-4">
              <button
                onClick={() => setIsAnimating(!isAnimating)}
                className={`flex-1 py-2 px-4 rounded-lg font-medium transition-colors ${
                  isAnimating 
                    ? 'bg-red-600 hover:bg-red-700 text-white' 
                    : 'bg-green-600 hover:bg-green-700 text-white'
                }`}
              >
                {isAnimating ? 'Stop Animation' : 'Start Animation'}
              </button>
              
              <button
                onClick={() => setAnimationSpeed(animationSpeed === 0.01 ? 0.05 : 0.01)}
                className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
              >
                Speed: {animationSpeed === 0.01 ? 'Slow' : 'Fast'}
              </button>
            </div>
          </div>
          
          {/* Fractal Controls */}
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-semibold mb-4">Fractal Parameters</h2>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">Fractal Type</label>
                <select
                  value={fractalParams.fractalType}
                  onChange={(e) => handleFractalParamChange('fractalType', e.target.value)}
                  className="w-full bg-gray-700 border border-gray-600 rounded-lg px-3 py-2 text-white"
                >
                  <option value="mandelbrot">Mandelbrot Set</option>
                  <option value="julia">Julia Set</option>
                  <option value="burning_ship">Burning Ship</option>
                  <option value="newton">Newton Fractal</option>
                  <option value="phoenix">Phoenix Fractal</option>
                </select>
              </div>
              
              <div>
                <label className="block text-sm font-medium mb-2">Zoom Level</label>
                <input
                  type="range"
                  min="0.1"
                  max="10"
                  step="0.1"
                  value={fractalParams.zoom}
                  onChange={(e) => handleFractalParamChange('zoom', parseFloat(e.target.value))}
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <p className="text-center mt-1">{fractalParams.zoom.toFixed(1)}x</p>
              </div>
              
              <div>
                <label className="block text-sm font-medium mb-2">Max Iterations</label>
                <input
                  type="range"
                  min="50"
                  max="500"
                  step="10"
                  value={fractalParams.maxIterations}
                  onChange={(e) => handleFractalParamChange('maxIterations', parseInt(e.target.value))}
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <p className="text-center mt-1">{fractalParams.maxIterations}</p>
              </div>
              
              <div className="bg-gray-700 rounded-lg p-3">
                <h3 className="font-medium mb-2">Emotional Impact</h3>
                <div className="text-sm space-y-1">
                  <div className="flex justify-between">
                    <span>Valence:</span>
                    <span className={emotionalState.valence > 0.5 ? 'text-green-400' : 'text-red-400'}>
                      {emotionalState.valence > 0.5 ? 'Positive' : 'Negative'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span>Arousal:</span>
                    <span className={emotionalState.arousal > 0.5 ? 'text-yellow-400' : 'text-blue-400'}>
                      {emotionalState.arousal > 0.5 ? 'High Energy' : 'Low Energy'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span>Dominance:</span>
                    <span className={emotionalState.dominance > 0.5 ? 'text-purple-400' : 'text-orange-400'}>
                      {emotionalState.dominance > 0.5 ? 'In Control' : 'Submissive'}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div className="mt-8 bg-gray-800 rounded-lg p-6">
          <h3 className="text-xl font-semibold mb-4">AI Integration Status</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-gray-700 rounded-lg p-4">
              <h4 className="font-medium mb-2">Emotional Computing</h4>
              <p className="text-sm text-green-400">✅ Active</p>
            </div>
            <div className="bg-gray-700 rounded-lg p-4">
              <h4 className="font-medium mb-2">Fractal Generation</h4>
              <p className="text-sm text-green-400">✅ Simulated</p>
            </div>
            <div className="bg-gray-700 rounded-lg p-4">
              <h4 className="font-medium mb-2">WASM Integration</h4>
              <p className="text-sm text-yellow-400">⚠️ Pending</p>
            </div>
          </div>
          
          <div className="mt-4 p-4 bg-yellow-900 border border-yellow-700 rounded-lg">
            <h4 className="font-medium text-yellow-300 mb-2">⚠️ Integration Note</h4>
            <p className="text-sm text-yellow-300">
              This is a simulation using Canvas 2D. For real WebGPU fractal generation, 
              integrate with the compiled WASM module from the Rust creative engine.
            </p>
          </div>
        </div>
        
        {/* AI Blockchain Integration */}
        {/* <FractalBlockchainIntegration 
          canvasRef={canvasRef}
          emotionalState={emotionalState}
          fractalParams={fractalParams}
        /> */}
      </div>
    </div>
  );
}