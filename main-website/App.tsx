import React from 'react';
import Navbar from './components/Navbar';
import Hero from './components/Hero';
import Features from './components/Features';
import DownloadHub from './components/DownloadHub';
import Footer from './components/Footer';

function App() {
  return (
    <div className="min-h-screen bg-main text-white selection:bg-acid/30">
      <Navbar />
      <main>
        <Hero />
        <Features />
        <DownloadHub />
      </main>
      <Footer />
    </div>
  );
}

export default App;