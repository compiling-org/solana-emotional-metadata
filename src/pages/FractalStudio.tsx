import React from 'react';
import { RealIntegratedRenderer } from '../components/RealIntegratedRenderer';

export const FractalStudio: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-900">
      <RealIntegratedRenderer 
        mode="combined"
        className="h-screen"
        onRecordingStart={() => console.log('Recording started')}
        onRecordingStop={(data) => console.log('Recording stopped:', data)}
      />
    </div>
  );
};

export default FractalStudio;