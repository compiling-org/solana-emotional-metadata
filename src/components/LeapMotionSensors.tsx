import React, { useEffect, useRef, useState } from 'react';

interface LeapMotionSensorsProps {
  host?: string;
  className?: string;
  onMetrics?: (metrics: { hands: number; gestures: number; latestGesture?: any }) => void;
}

const LeapMotionSensors: React.FC<LeapMotionSensorsProps> = ({ host = 'ws://localhost:6437', className, onMetrics }) => {
  const [connected, setConnected] = useState(false);
  const [handCount, setHandCount] = useState(0);
  const [gestureCount, setGestureCount] = useState(0);
  const [latestGesture, setLatestGesture] = useState<any>(null);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    let ws: WebSocket | null = null;
    let running = true;
    try {
      ws = new WebSocket(`${host}/v6.json`);
      wsRef.current = ws;
      ws.onopen = () => {
        if (!running) return;
        setConnected(true);
        const enable = JSON.stringify({ enableGestures: true, background: false });
        ws?.send(enable);
      };
      ws.onmessage = (evt) => {
        if (!running) return;
        try {
          const data = JSON.parse(evt.data as string);
          const hands = data.hands ? data.hands.length : 0;
          const gestures = data.gestures ? data.gestures.length : 0;
          setHandCount(hands);
          setGestureCount(gestures);
          if (gestures > 0) {
            setLatestGesture(data.gestures[data.gestures.length - 1]);
          }
          onMetrics?.({ hands, gestures, latestGesture });
        } catch {}
      };
      ws.onerror = () => {
        if (!running) return;
        setConnected(false);
      };
      ws.onclose = () => {
        if (!running) return;
        setConnected(false);
      };
    } catch {
      setConnected(false);
    }
    return () => {
      running = false;
      try {
        ws?.close();
      } catch {}
    };
  }, [host]);

  return (
    <div className={`bg-gray-800 rounded-lg p-4 border border-gray-700 ${className || ''}`}>
      <div className="flex justify-between items-center mb-3">
        <h3 className="text-white font-semibold">Leap Motion Sensors</h3>
        <div className={`text-xs ${connected ? 'text-green-400' : 'text-red-400'}`}>
          {connected ? 'Connected' : 'Disconnected'}
        </div>
      </div>
      <div className="grid grid-cols-3 gap-3 mb-4">
        <div className="text-center">
          <div className="text-gray-400 text-xs">Hands</div>
          <div className="text-purple-400 text-xl font-mono">{handCount}</div>
        </div>
        <div className="text-center">
          <div className="text-gray-400 text-xs">Gestures</div>
          <div className="text-blue-400 text-xl font-mono">{gestureCount}</div>
        </div>
        <div className="text-center">
          <div className="text-gray-400 text-xs">Status</div>
          <div className="text-gray-300 text-sm">{connected ? 'Live' : 'Idle'}</div>
        </div>
      </div>
      <div className="bg-gray-900 rounded p-3">
        <div className="text-xs text-gray-400 mb-2">Latest Gesture</div>
        <pre className="text-xs text-gray-200 overflow-auto max-h-40">
          {latestGesture ? JSON.stringify(latestGesture, null, 2) : 'No gestures'}
        </pre>
      </div>
      <div className="mt-3 text-xs text-gray-500">
        Requires Leap Motion service at {host}
      </div>
    </div>
  );
};

export default LeapMotionSensors;
