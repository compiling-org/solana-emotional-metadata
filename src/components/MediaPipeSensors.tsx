import React, { useEffect, useRef, useState } from 'react';
import { Camera } from '@mediapipe/camera_utils';
import { Hands } from '@mediapipe/hands';
import { FaceMesh } from '@mediapipe/face_mesh';
import { Pose } from '@mediapipe/pose';

interface MediaPipeSensorsProps {
  className?: string;
  onMetrics?: (metrics: { 
    hands: number; 
    faces: number; 
    poses: number;
    features?: {
      faceVariance: number;
      handOpenness: number;
      poseStability: number;
      confidence: number;
    }
  }) => void;
}

const MediaPipeSensors: React.FC<MediaPipeSensorsProps> = ({ className, onMetrics }) => {
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [status, setStatus] = useState('Initializing');
  const [counts, setCounts] = useState({ hands: 0, faces: 0, poses: 0 });
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let camera: Camera | null = null;
    let hands: Hands | null = null;
    let faceMesh: FaceMesh | null = null;
    let pose: Pose | null = null;
    let running = true;
    let lastPose: Array<{x:number;y:number}> | null = null;

    const init = async () => {
      try {
        const video = document.createElement('video');
        video.setAttribute('playsinline', 'true');
        video.muted = true;
        video.autoplay = true;
        videoRef.current = video;
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        hands = new Hands({ locateFile: (file) => `https://cdn.jsdelivr.net/npm/@mediapipe/hands/${file}` });
        hands.setOptions({ maxNumHands: 2, modelComplexity: 1, selfieMode: true });
        hands.onResults((results: any) => {
          if (!running || !ctx || !canvas) return;
          if (results.image) {
            ctx.drawImage(results.image, 0, 0, canvas.width, canvas.height);
          }
          const handCount = results.multiHandLandmarks ? results.multiHandLandmarks.length : 0;
          // Hand openness proxy: average bounding box area normalized
          let handOpenness = 0;
          if (results.multiHandLandmarks && results.multiHandLandmarks.length > 0) {
            const areas = results.multiHandLandmarks.map((lm: any) => {
              let minX = 1, minY = 1, maxX = 0, maxY = 0;
              lm.forEach((p: any) => {
                minX = Math.min(minX, p.x);
                minY = Math.min(minY, p.y);
                maxX = Math.max(maxX, p.x);
                maxY = Math.max(maxY, p.y);
              });
              const w = Math.max(0, maxX - minX);
              const h = Math.max(0, maxY - minY);
              return w * h;
            });
            handOpenness = areas.reduce((a: number, b: number) => a + b, 0) / areas.length;
          }
          setCounts(prev => {
            const next = {
              ...prev,
              hands: handCount
            };
            onMetrics?.({
              ...next,
              features: {
                faceVariance: 0,
                handOpenness,
                poseStability: 0,
                confidence: Math.min(1, handCount * 0.3)
              }
            });
            return next;
          });
          if (results.multiHandLandmarks) {
            results.multiHandLandmarks.forEach((lm: any) => {
              // connectors
              ctx.strokeStyle = '#a78bfa55';
              ctx.lineWidth = 2;
              ctx.beginPath();
              lm.forEach((p: any, idx: number) => {
                const x = p.x * canvas.width;
                const y = p.y * canvas.height;
                if (idx === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
              });
              ctx.stroke();
              lm.forEach((p: any) => {
                ctx.fillStyle = '#a78bfa';
                ctx.beginPath();
                ctx.arc(p.x * canvas.width, p.y * canvas.height, 3, 0, Math.PI * 2);
                ctx.fill();
              });
            });
          }
        });

        faceMesh = new FaceMesh({ locateFile: (file) => `https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh/${file}` });
        faceMesh.setOptions({ refineLandmarks: true, maxNumFaces: 1 });
        faceMesh.onResults((results: any) => {
          if (!running || !ctx || !canvas) return;
          if (results.image) {
            ctx.drawImage(results.image, 0, 0, canvas.width, canvas.height);
          }
          const faceCount = results.multiFaceLandmarks ? results.multiFaceLandmarks.length : 0;
          // Face variance: mean distance to centroid
          let faceVariance = 0;
          if (results.multiFaceLandmarks && results.multiFaceLandmarks.length > 0) {
            const variances = results.multiFaceLandmarks.map((lm: any) => {
              const cx = lm.reduce((s: number, p: any) => s + p.x, 0) / lm.length;
              const cy = lm.reduce((s: number, p: any) => s + p.y, 0) / lm.length;
              const v = lm.reduce((s: number, p: any) => {
                const dx = p.x - cx; const dy = p.y - cy;
                return s + Math.sqrt(dx*dx + dy*dy);
              }, 0) / lm.length;
              return v;
            });
            faceVariance = variances.reduce((a: number, b: number) => a + b, 0) / variances.length;
          }
          setCounts(prev => {
            const next = {
              ...prev,
              faces: faceCount
            };
            onMetrics?.({
              ...next,
              features: {
                faceVariance,
                handOpenness: 0,
                poseStability: 0,
                confidence: Math.min(1, faceCount * 0.4)
              }
            });
            return next;
          });
          if (results.multiFaceLandmarks) {
            results.multiFaceLandmarks.forEach((lm: any) => {
              // connectors (subset to limit overdraw)
              ctx.strokeStyle = '#60a5fa55';
              ctx.lineWidth = 1.5;
              ctx.beginPath();
              lm.slice(0, 100).forEach((p: any, idx: number) => {
                const x = p.x * canvas.width;
                const y = p.y * canvas.height;
                if (idx === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
              });
              ctx.stroke();
              lm.forEach((p: any) => {
                ctx.fillStyle = '#60a5fa';
                ctx.beginPath();
                ctx.arc(p.x * canvas.width, p.y * canvas.height, 2, 0, Math.PI * 2);
                ctx.fill();
              });
            });
          }
        });

        pose = new Pose({ locateFile: (file) => `https://cdn.jsdelivr.net/npm/@mediapipe/pose/${file}` });
        pose.setOptions({ modelComplexity: 1 });
        pose.onResults((results: any) => {
          if (!running || !ctx || !canvas) return;
          if (results.image) {
            ctx.drawImage(results.image, 0, 0, canvas.width, canvas.height);
          }
          const hasPose = results.poseLandmarks && results.poseLandmarks.length > 0 ? 1 : 0;
          let poseStability = 0;
          if (results.poseLandmarks && results.poseLandmarks.length > 0) {
            const current = results.poseLandmarks.map((p: any) => ({ x: p.x, y: p.y }));
            if (lastPose && lastPose.length === current.length) {
              const deltas = current.map((p: { x: number; y: number }, i: number) => {
                const dx = p.x - lastPose![i].x;
                const dy = p.y - lastPose![i].y;
                return Math.sqrt(dx*dx + dy*dy);
              });
              const avgDelta = deltas.reduce((a: number, b: number) => a + b, 0) / deltas.length;
              poseStability = Math.max(0, 1 - avgDelta * 10); // normalize
            }
            lastPose = current;
          }
          setCounts(prev => {
            const next = {
              ...prev,
              poses: hasPose
            };
            onMetrics?.({
              ...next,
              features: {
                faceVariance: 0,
                handOpenness: 0,
                poseStability,
                confidence: Math.min(1, hasPose * 0.5)
              }
            });
            return next;
          });
          if (results.poseLandmarks) {
            // connectors
            ctx.strokeStyle = '#34d39966';
            ctx.lineWidth = 2;
            ctx.beginPath();
            results.poseLandmarks.forEach((p: any, idx: number) => {
              const x = p.x * canvas.width;
              const y = p.y * canvas.height;
              if (idx === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
            });
            ctx.stroke();
            results.poseLandmarks.forEach((p: any) => {
              ctx.fillStyle = '#34d399';
              ctx.beginPath();
              ctx.arc(p.x * canvas.width, p.y * canvas.height, 3, 0, Math.PI * 2);
              ctx.fill();
            });
          }
        });

        camera = new Camera(video, {
          onFrame: async () => {
            if (!video) return;
            await hands!.send({ image: video });
            await faceMesh!.send({ image: video });
            await pose!.send({ image: video });
          },
          width: 640,
          height: 480,
        });
        try {
          const stream = await navigator.mediaDevices.getUserMedia({ video: { width: 640, height: 480 } });
          video.srcObject = stream;
          await video.play();
        } catch {}
        setReady(true);
        setStatus('Running');
        camera.start();
      } catch (e) {
        setStatus('Error');
      }
    };

    init();

    return () => {
      running = false;
      try {
        camera?.stop();
      } catch {}
    };
  }, []);

  return (
    <div className={`bg-gray-800 rounded-lg p-4 border border-gray-700 ${className || ''}`}>
      <div className="flex justify-between items-center mb-3">
        <h3 className="text-white font-semibold">MediaPipe Sensors</h3>
        <div className="text-xs text-gray-400">{status}</div>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="bg-gray-900 rounded p-2">
          <canvas ref={canvasRef} width={640} height={480} className="w-full rounded" />
        </div>
        <div className="bg-gray-900 rounded p-4">
          <div className="grid grid-cols-3 gap-3">
            <div className="text-center">
              <div className="text-gray-400 text-xs">Hands</div>
              <div className="text-purple-400 text-xl font-mono">{counts.hands}</div>
            </div>
            <div className="text-center">
              <div className="text-gray-400 text-xs">Faces</div>
              <div className="text-blue-400 text-xl font-mono">{counts.faces}</div>
            </div>
            <div className="text-center">
              <div className="text-gray-400 text-xs">Pose</div>
              <div className="text-green-400 text-xl font-mono">{counts.poses}</div>
            </div>
          </div>
          <div className="mt-4 text-xs text-gray-500">
            {ready ? 'Camera active' : 'Awaiting camera permission'}
          </div>
        </div>
      </div>
    </div>
  );
};

export default MediaPipeSensors;
