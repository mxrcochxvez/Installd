<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { Release } from '../types';
import ReleaseCard from './ReleaseCard.vue';
import { Database, Loader2 } from 'lucide-vue-next';

const releases = ref<Release[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

const latestRelease = ref<Release | null>(null);
const previousReleases = ref<Release[]>([]);

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

const fetchReleases = async () => {
  try {
    const response = await fetch('https://api.github.com/repos/mxrcochxvez/Installd/releases');
    if (!response.ok) throw new Error('Failed to fetch releases');
    
    const data = await response.json();
    
    releases.value = data.map((item: any) => {
      // Find .exe asset for Windows
      const asset = item.assets.find((a: any) => a.name.endsWith('.exe'));
      
      return {
        id: item.id.toString(),
        version: item.tag_name,
        name: item.name || item.tag_name,
        description: item.body || 'No release notes available.',
        date: new Date(item.published_at).toLocaleDateString(),
        downloadUrl: asset ? asset.browser_download_url : item.html_url,
        isPrerelease: item.prerelease,
        size: asset ? formatFileSize(asset.size) : 'N/A',
        downloads_count: asset ? asset.download_count : 0
      };
    });

    if (releases.value.length > 0) {
      latestRelease.value = releases.value[0];
      previousReleases.value = releases.value.slice(1);
    }
  } catch (e) {
    error.value = 'Unable to load release manifest.';
    console.error(e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchReleases();
});
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

      <div v-if="loading" class="flex justify-center py-12">
        <Loader2 class="w-8 h-8 text-acid animate-spin" />
      </div>

      <div v-else-if="error" class="text-center py-12 text-red-400 font-mono">
        {{ error }}
      </div>

      <template v-else>
        <!-- Latest Release -->
        <div v-if="latestRelease" class="mb-16">
          <ReleaseCard :release="latestRelease" :isLatest="true" />
        </div>

        <!-- History Section -->
        <div v-if="previousReleases.length > 0" id="history" class="scroll-mt-24">
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
                HASH CHECKSUMS AVAILABLE ON GITHUB // <a href="https://github.com/mxrcochxvez/Installd" target="_blank" class="text-acid hover:underline">VIEW REPOSITORY</a>
             </p>
        </div>
      </template>
    </div>
  </section>
</template>
