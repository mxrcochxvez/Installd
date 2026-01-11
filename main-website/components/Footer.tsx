import React from 'react';
import { Github, Twitter } from 'lucide-react';

const Footer: React.FC = () => {
  return (
    <footer className="bg-main py-16">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="border-t border-white/10 pt-8 flex flex-col md:flex-row justify-between items-start md:items-center gap-8">
          
          <div>
            <div className="font-mono text-lg font-bold text-white uppercase mb-2">Installed_</div>
            <p className="text-dim text-sm max-w-xs">
              Open source distribution system.<br/>
              Optimize your workflow.
            </p>
          </div>

          <div className="flex gap-8 text-sm font-mono uppercase tracking-wider">
             <a href="#" className="text-dim hover:text-acid transition-colors">Github</a>
             <a href="#" className="text-dim hover:text-acid transition-colors">Twitter</a>
             <a href="#" className="text-dim hover:text-acid transition-colors">Discord</a>
          </div>

          <div className="text-right">
             <p className="text-dim text-xs font-mono">
               © {new Date().getFullYear()} INSTALLED PROJECT
             </p>
             <p className="text-white/20 text-[10px] font-mono mt-1">
               NO TRACKERS // NO ADS
             </p>
          </div>

        </div>
      </div>
    </footer>
  );
};

export default Footer;