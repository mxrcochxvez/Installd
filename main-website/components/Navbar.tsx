import React, { useState, useEffect } from 'react';
import { Menu, X, AppWindow, Check, AlertCircle } from 'lucide-react';

const Navbar: React.FC = () => {
  const [isScrolled, setIsScrolled] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [systemStatus, setSystemStatus] = useState<{ msg: string; type: 'success' | 'error' } | null>(null);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const scrollToSection = (id: string) => {
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' });
      setIsMobileMenuOpen(false);
    }
  };

  const checkSystemCompatibility = () => {
    const userAgent = window.navigator.userAgent;
    // Simple check for Windows
    const isWindows = userAgent.includes('Windows');

    if (isWindows) {
      setSystemStatus({ msg: 'available for your system', type: 'success' });
    } else {
      setSystemStatus({ msg: 'not yet available for your system', type: 'error' });
    }

    // Hide after 3 seconds
    setTimeout(() => {
      setSystemStatus(null);
    }, 3000);
    
    setIsMobileMenuOpen(false);
  };

  return (
    <nav
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-200 border-b ${
        isScrolled
          ? 'bg-main/90 backdrop-blur-sm border-white/10 py-3'
          : 'bg-transparent border-transparent py-6'
      }`}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center">
          <div className="flex items-center space-x-3 cursor-pointer group" onClick={() => scrollToSection('hero')}>
            <div className="bg-white text-black p-1.5 group-hover:bg-acid transition-colors duration-300">
              <AppWindow className="w-5 h-5" />
            </div>
            <span className="text-xl font-bold tracking-tight text-white font-mono uppercase">Installed_</span>
          </div>

          {/* Desktop Nav */}
          <div className="hidden md:flex items-center space-x-12">
            <button onClick={() => scrollToSection('features')} className="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
              Features
            </button>
            <button onClick={() => scrollToSection('history')} className="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
              Index
            </button>
            <button onClick={checkSystemCompatibility} className="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
              Sys_Info
            </button>
            <button
              onClick={() => scrollToSection('download')}
              className="bg-acid hover:bg-acid-hover text-black px-6 py-2 text-sm font-bold uppercase tracking-wider transition-all transform hover:-translate-y-0.5"
            >
              Get Installer
            </button>
          </div>

          {/* Mobile Menu Button */}
          <div className="md:hidden">
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="text-white p-2 border border-white/10"
            >
              {isMobileMenuOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
            </button>
          </div>
        </div>
      </div>

      {/* Mobile Nav */}
      {isMobileMenuOpen && (
        <div className="md:hidden absolute top-full left-0 right-0 bg-main border-b border-white/10 p-4">
          <div className="flex flex-col space-y-4">
            <button onClick={() => scrollToSection('features')} className="text-left text-dim hover:text-white py-2 font-mono uppercase">
              // Features
            </button>
            <button onClick={() => scrollToSection('history')} className="text-left text-dim hover:text-white py-2 font-mono uppercase">
              // Index
            </button>
            <button onClick={checkSystemCompatibility} className="text-left text-dim hover:text-white py-2 font-mono uppercase">
              // Sys_Info
            </button>
            <button
              onClick={() => scrollToSection('download')}
              className="bg-acid text-black py-3 font-bold uppercase tracking-wider text-center"
            >
              Initialize Download
            </button>
          </div>
        </div>
      )}

      {/* Toast Notification */}
      {systemStatus && (
        <div className="absolute top-full left-0 right-0 md:left-auto md:right-8 md:top-24 md:w-auto flex justify-center pointer-events-none">
            <div className={`
                pointer-events-auto mt-4 md:mt-0 mx-4 md:mx-0
                flex items-center gap-3 px-6 py-4 
                bg-surface border-l-4 shadow-2xl animate-in fade-in slide-in-from-top-4 duration-300
                ${systemStatus.type === 'success' ? 'border-acid' : 'border-red-500'}
            `}>
                {systemStatus.type === 'success' ? (
                    <Check className="w-5 h-5 text-acid" />
                ) : (
                    <AlertCircle className="w-5 h-5 text-red-500" />
                )}
                <span className="font-mono text-sm font-bold uppercase tracking-wide text-white">
                    {systemStatus.msg}
                </span>
            </div>
        </div>
      )}
    </nav>
  );
};

export default Navbar;