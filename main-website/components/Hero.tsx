import React from 'react';
import { Download, ArrowRight, ShieldCheck, Cpu, AppWindow } from 'lucide-react';
import { MOCK_RELEASES } from '../constants';

const Hero: React.FC = () => {
  const latestRelease = MOCK_RELEASES[0];

  const scrollToDownload = () => {
    document.getElementById('download')?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <section id="hero" className="relative pt-32 pb-20 lg:pt-48 lg:pb-32 overflow-hidden border-b border-white/10">
      {/* Technical Grid Background */}
      <div className="absolute inset-0 bg-grid-pattern bg-[length:40px_40px] opacity-[0.15] pointer-events-none"></div>
      <div className="absolute inset-0 bg-gradient-to-t from-main via-transparent to-transparent pointer-events-none"></div>

      <div className="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        <div className="flex flex-col lg:flex-row items-center gap-16">
          <div className="flex-1 text-left">
            <div className="inline-flex items-center gap-3 px-4 py-2 border border-acid/30 bg-acid/5 mb-8">
              <span className="w-2 h-2 bg-acid animate-pulse"></span>
              <span className="text-acid font-mono text-xs uppercase tracking-widest">Build v{latestRelease.version} // Native Rust</span>
            </div>

            <h1 className="text-6xl md:text-8xl font-bold tracking-tighter text-white mb-6 uppercase leading-[0.9]">
              Reclaim<br/>
              <span className="text-transparent bg-clip-text bg-gradient-to-r from-acid to-white/50">Your Space.</span><br/>
              <span className="text-dim">Your Wallet.</span>
            </h1>
            
            <p className="text-lg md:text-xl text-dim max-w-xl mb-10 font-light leading-relaxed border-l-2 border-white/20 pl-6">
              A lightning-fast native Windows utility built in Rust. 
              Zero webview overhead. Uninstall apps, visualize storage, and find FOSS alternatives instantly.
            </p>

            <div className="flex flex-col sm:flex-row gap-4">
              <button
                onClick={scrollToDownload}
                className="px-8 py-4 bg-acid hover:bg-acid-hover text-black font-bold text-lg uppercase tracking-wider transition-all flex items-center justify-center gap-3 group"
              >
                <Download className="w-5 h-5" />
                <span>Download Installer</span>
              </button>
              <button 
                 onClick={() => document.getElementById('features')?.scrollIntoView({ behavior: 'smooth' })}
                 className="px-8 py-4 bg-transparent hover:bg-white/5 text-white border border-white/20 font-bold text-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2"
              >
                View Features
                <ArrowRight className="w-5 h-5" />
              </button>
            </div>

            <div className="mt-12 flex items-center gap-8 text-dim text-xs font-mono uppercase tracking-widest">
              <div className="flex items-center gap-2">
                <ShieldCheck className="w-4 h-4 text-acid" />
                <span>Verified Safe</span>
              </div>
              <div className="flex items-center gap-2">
                <Cpu className="w-4 h-4 text-acid" />
                <span>Native WinAPI</span>
              </div>
              <div className="flex items-center gap-2">
                <AppWindow className="w-4 h-4 text-acid" />
                <span>&lt; 5MB Memory</span>
              </div>
            </div>
          </div>

          {/* Native UI Mockup */}
          <div className="flex-1 w-full max-w-lg lg:max-w-full">
             <div className="relative aspect-square md:aspect-[4/3] border-2 border-white/10 bg-surface/50 p-2">
                {/* Decorative corners */}
                <div className="absolute -top-1 -left-1 w-4 h-4 border-t-2 border-l-2 border-acid"></div>
                <div className="absolute -top-1 -right-1 w-4 h-4 border-t-2 border-r-2 border-acid"></div>
                <div className="absolute -bottom-1 -left-1 w-4 h-4 border-b-2 border-l-2 border-acid"></div>
                <div className="absolute -bottom-1 -right-1 w-4 h-4 border-b-2 border-r-2 border-acid"></div>
                
                {/* Internal Window Mockup */}
                <div className="h-full border border-white/10 bg-main flex flex-col shadow-2xl overflow-hidden">
                  
                  {/* Title Bar */}
                  <div className="h-10 border-b border-white/10 bg-surface flex items-center justify-between px-4">
                    <div className="flex items-center gap-2">
                       <div className="w-3 h-3 bg-acid/80 rounded-sm"></div>
                       <span className="text-xs font-bold text-white tracking-wide">Installed</span>
                    </div>
                    <div className="flex gap-2">
                      <div className="w-3 h-3 border border-white/20"></div>
                      <div className="w-3 h-3 bg-white/20"></div>
                    </div>
                  </div>
                  
                  {/* App Content */}
                  <div className="flex-1 flex">
                     {/* Sidebar */}
                     <div className="w-16 md:w-48 border-r border-white/10 bg-surface/30 p-4 hidden sm:flex flex-col gap-4">
                        <div className="h-8 bg-acid/10 border-l-2 border-acid flex items-center px-3 text-xs font-mono text-acid font-bold">DASHBOARD</div>
                        <div className="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">PACKAGES</div>
                        <div className="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">STORAGE</div>
                        <div className="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">SETTINGS</div>
                     </div>

                     {/* Main Area */}
                     <div className="flex-1 p-6 bg-main relative">
                        <div className="flex justify-between items-center mb-6">
                           <h3 className="text-xl font-bold text-white">Installed Apps</h3>
                           <div className="px-3 py-1 bg-surface border border-white/10 text-xs text-dim font-mono">Filter: All</div>
                        </div>

                        {/* Grid of Apps */}
                        <div className="grid grid-cols-2 gap-4">
                           {[1, 2, 3, 4].map((i) => (
                              <div key={i} className="bg-surface/50 border border-white/10 p-4 hover:border-acid/50 transition-colors group cursor-pointer relative overflow-hidden">
                                 <div className="flex items-start justify-between mb-3">
                                    <div className="w-8 h-8 bg-white/10 group-hover:bg-acid/20 transition-colors"></div>
                                    <div className="w-2 h-2 rounded-full bg-green-500"></div>
                                 </div>
                                 <div className="h-2 w-20 bg-white/20 mb-2"></div>
                                 <div className="h-2 w-12 bg-white/10"></div>
                                 
                                 <div className="absolute inset-0 bg-acid/5 opacity-0 group-hover:opacity-100 transition-opacity"></div>
                              </div>
                           ))}
                        </div>

                        {/* Floating Action Button simulation */}
                        <div className="absolute bottom-6 right-6 w-12 h-12 bg-acid text-black flex items-center justify-center shadow-lg shadow-acid/20">
                           <Download className="w-5 h-5" />
                        </div>
                     </div>
                  </div>
                </div>
             </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;