import React from 'react';
import { MOCK_RELEASES } from '../constants';
import ReleaseCard from './ReleaseCard';
import { Database } from 'lucide-react';

const DownloadHub: React.FC = () => {
  const latestRelease = MOCK_RELEASES[0];
  const previousReleases = MOCK_RELEASES.slice(1);

  return (
    <section id="download" className="py-24 bg-surfaceHighlight border-b border-white/10">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        
        <div className="flex items-center justify-between mb-12 border-b border-white/10 pb-6">
          <div>
            <h2 className="text-3xl font-bold text-white uppercase tracking-tight">Release Manifest</h2>
            <p className="text-dim font-mono text-sm mt-2">Target OS: Windows 10/11 (x64)</p>
          </div>
          <Database className="w-8 h-8 text-white/20" />
        </div>

        {/* Latest Release */}
        <div className="mb-16">
          <ReleaseCard release={latestRelease} isLatest={true} />
        </div>

        {/* History Section */}
        <div id="history" className="scroll-mt-24">
          <h3 className="text-sm font-mono text-dim uppercase tracking-widest mb-6 flex items-center gap-2">
             <span className="w-2 h-2 bg-dim"></span>
             Archived Versions
          </h3>
          
          <div className="space-y-px bg-white/10 border border-white/10">
            {previousReleases.map((release) => (
              <ReleaseCard key={release.id} release={release} />
            ))}
          </div>
        </div>
        
        <div className="mt-12 p-4 border border-dashed border-white/20 text-center">
             <p className="text-dim font-mono text-xs">
                HASH CHECKSUMS AVAILABLE ON GITHUB // <a href="#" className="text-acid hover:underline">VIEW REPOSITORY</a>
             </p>
        </div>
      </div>
    </section>
  );
};

export default DownloadHub;