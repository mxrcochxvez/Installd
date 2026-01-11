<script setup lang="ts">
import { MOCK_RELEASES } from '../constants';
import ReleaseCard from './ReleaseCard.vue';
import { Database } from 'lucide-vue-next';

const latestRelease = MOCK_RELEASES[0];
const previousReleases = MOCK_RELEASES.slice(1);
</script>

<template>
  <section id="download" class="py-24 bg-surfaceHighlight border-b border-white/10">
    <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
      
      <div class="flex items-center justify-between mb-12 border-b border-white/10 pb-6">
        <div>
          <h2 class="text-3xl font-bold text-white uppercase tracking-tight">Release Manifest</h2>
          <p class="text-dim font-mono text-sm mt-2">Target OS: Windows 10/11 (x64)</p>
        </div>
        <Database class="w-8 h-8 text-white/20" />
      </div>

      <!-- Latest Release -->
      <div class="mb-16">
        <ReleaseCard :release="latestRelease" :isLatest="true" />
      </div>

      <!-- History Section -->
      <div id="history" class="scroll-mt-24">
        <h3 class="text-sm font-mono text-dim uppercase tracking-widest mb-6 flex items-center gap-2">
           <span class="w-2 h-2 bg-dim"></span>
           Archived Versions
        </h3>
        
        <div class="space-y-px bg-white/10 border border-white/10">
          <ReleaseCard v-for="release in previousReleases" :key="release.id" :release="release" />
        </div>
      </div>
      
      <div class="mt-12 p-4 border border-dashed border-white/20 text-center">
           <p class="text-dim font-mono text-xs">
              HASH CHECKSUMS AVAILABLE ON GITHUB // <a href="#" class="text-acid hover:underline">VIEW REPOSITORY</a>
           </p>
      </div>
    </div>
  </section>
</template>
