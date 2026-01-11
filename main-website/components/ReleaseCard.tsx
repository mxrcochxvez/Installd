import React from 'react';
import { Release } from '../types';
import { Download, ChevronRight } from 'lucide-react';

interface ReleaseCardProps {
  release: Release;
  isLatest?: boolean;
}

const ReleaseCard: React.FC<ReleaseCardProps> = ({ release, isLatest = false }) => {
  return (
    <div className={`group relative border ${isLatest ? 'border-acid bg-acid/5' : 'border-white/10 bg-main'} p-6 md:p-8 transition-all hover:bg-surface`}>
      
      {/* Label for Latest */}
      {isLatest && (
        <div className="absolute -top-3 left-6 bg-acid text-black text-[10px] font-bold px-2 py-1 uppercase tracking-widest">
          Current_Version
        </div>
      )}

      <div className="flex flex-col md:flex-row md:items-center justify-between gap-8">
        <div className="flex-1">
          <div className="flex items-baseline gap-4 mb-2">
            <h3 className={`text-3xl font-bold font-mono ${isLatest ? 'text-white' : 'text-dim group-hover:text-white'}`}>
              {release.version}
            </h3>
            <span className="text-xs font-mono text-dim uppercase tracking-wider border border-white/10 px-2 py-0.5">
              {release.date}
            </span>
          </div>
          
          <div className="mb-4">
             <span className="text-sm font-bold text-white uppercase tracking-wide">{release.name}</span>
          </div>

          <p className="text-dim text-sm font-mono max-w-2xl border-l border-white/10 pl-4 py-1">
            > {release.description}
          </p>
          
          <div className="mt-4 flex gap-4 text-xs font-mono text-dim">
             <span>SIZE: {release.size}</span>
             <span>|</span>
             <span>DL: {release.downloads_count}</span>
          </div>
        </div>

        <div className="flex-shrink-0">
          <a
            href={release.downloadUrl}
            className={`flex items-center justify-center gap-2 px-6 py-4 text-sm font-bold uppercase tracking-wider border transition-all ${
              isLatest
                ? 'bg-acid text-black border-acid hover:bg-white hover:border-white'
                : 'bg-transparent text-white border-white/20 hover:border-white hover:bg-white/5'
            }`}
          >
            <Download className="w-4 h-4" />
            <span>Download .EXE</span>
          </a>
        </div>
      </div>
    </div>
  );
};

export default ReleaseCard;