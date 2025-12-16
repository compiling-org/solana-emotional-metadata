import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import SolanaEmotionalNFT from './pages/SolanaEmotionalNFT';

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-900">
        <header className="bg-gray-800 shadow">
          <div className="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
            <h1 className="text-3xl font-bold text-white">
              Solana Emotional Metadata
            </h1>
            <nav className="mt-4">
              <Link to="/" className="text-green-400 hover:text-green-300 mr-6">� Home</Link>
              <Link to="/solana-nft" className="text-yellow-400 hover:text-yellow-300 mr-6">� Solana NFT</Link>
            </nav>
          </div>
        </header>
        
        <main>
          <Routes>
            <Route path="/" element={<SolanaEmotionalNFT />} />
            <Route path="/solana-nft" element={<SolanaEmotionalNFT />} />
          </Routes>
        </main>
        
        <footer className="bg-gray-800">
          <div className="max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8">
            <p className="text-center text-gray-400 text-sm">
              Solana devnet wallet connection and emotional metadata studio
            </p>
          </div>
        </footer>
      </div>
    </Router>
  );
}

export default App;
