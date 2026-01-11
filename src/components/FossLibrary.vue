<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Search, Package, Download, Loader2 } from 'lucide-vue-next';

interface ApiPackage {
  id: string;
  name: string;
  publisher: string;
  description: string;
  license: string;
  tags: string[];
  latest_version: string;
}

const CATEGORIES = [
  { id: "browser", label: "Browsers", icon: "🌐" },
  { id: "development", label: "Dev", icon: "💻" },
  { id: "media", label: "Media", icon: "🎬" },
  { id: "utilities", label: "Utilities", icon: "🔧" },
  { id: "productivity", label: "Office", icon: "📊" },
  { id: "security", label: "Security", icon: "🔒" },
  { id: "games", label: "Games", icon: "🎮" },
];

// State
const apiPackages = ref<ApiPackage[]>([]);
const totalPackages = ref(0);
const currentPage = ref(0);
const libraryLoading = ref(false);
const librarySearchQuery = ref("");
const selectedCategory = ref<string | null>(null);
const installingPackage = ref<string | null>(null);
const installStatus = ref("");

// Computed
const displayedLibraryPackages = computed(() => {
  let packages = apiPackages.value;
  
  if (selectedCategory.value) {
    packages = packages.filter((pkg) =>
      pkg.tags.some((tag) => tag.toLowerCase().includes(selectedCategory.value!.toLowerCase()))
    );
  }
  
  if (librarySearchQuery.value) {
    const query = librarySearchQuery.value.toLowerCase();
    packages = packages.filter(
      (pkg) =>
        pkg.name.toLowerCase().includes(query) ||
        pkg.publisher.toLowerCase().includes(query) ||
        pkg.id.toLowerCase().includes(query)
    );
  }
  
  return packages;
});

// Methods
async function loadLibraryPackages(page: number = 0) {
  libraryLoading.value = true;
  try {
    const result = await invoke<[ApiPackage[], number]>("fetch_winget_api", {
      page,
      perPage: 50,
    });
    apiPackages.value = result[0];
    totalPackages.value = result[1];
    currentPage.value = page;
  } catch (e) {
    console.error("Failed to load library packages:", e);
  }
  libraryLoading.value = false;
}

async function searchLibraryPackages() {
  if (!librarySearchQuery.value.trim()) {
    loadLibraryPackages(0);
    return;
  }
  libraryLoading.value = true;
  try {
    const results = await invoke<ApiPackage[]>("search_winget_api", {
      query: librarySearchQuery.value,
    });
    apiPackages.value = results;
    totalPackages.value = results.length;
  } catch (e) {
    console.error("Failed to search packages:", e);
  }
  libraryLoading.value = false;
}

async function installApiPackage(pkg: ApiPackage) {
  const confirmed = window.confirm(
    `Install ${pkg.name}?\n\nThis will install the software using Windows Package Manager (winget).`
  );
  
  if (!confirmed) return;
  
  installingPackage.value = pkg.id;
  installStatus.value = `Installing ${pkg.name}...`;
  
  try {
    const result = await invoke<string>("install_winget", {
      packageId: pkg.id,
    });
    installStatus.value = result;
  } catch (e) {
    installStatus.value = `Error: ${e}`;
  } finally {
    installingPackage.value = null;
    setTimeout(() => { installStatus.value = ""; }, 5000);
  }
}

function getPackageFavicon(publisher: string): string {
  if (!publisher || publisher.length < 2) return '';
  const publisherClean = publisher.toLowerCase()
    .replace(/[^a-z0-9]/g, '')
    .replace(/inc|llc|corp|ltd|gmbh|software|technologies?|studios?/gi, '');
    
  return `https://www.google.com/s2/favicons?domain=${publisherClean}.com&sz=64`;
}

onMounted(() => {
  loadLibraryPackages();
});
</script>

<template>
  <div class="h-full flex flex-col min-w-0">
    <!-- Toolbar -->
    <div class="h-14 border-b border-white/10 flex items-center justify-between px-6 bg-main z-10 shrink-0">
      <div class="flex items-center gap-4">
         <h2 class="text-lg font-bold text-white tracking-tight">FOSS Library</h2>
         <div class="h-4 w-px bg-white/10"></div>
         <div class="flex gap-2">
            <button 
               v-for="cat in CATEGORIES" 
               :key="cat.id"
               @click="selectedCategory = selectedCategory === cat.id ? null : cat.id"
               class="text-xs px-2 py-1 rounded-sm border transition-colors flex items-center gap-1"
               :class="selectedCategory === cat.id ? 'bg-acid text-black border-acid font-bold' : 'bg-transparent border-white/10 text-dim hover:border-white/30 hover:text-white'"
            >
               <span>{{ cat.icon }}</span>
               <span class="hidden xl:inline">{{ cat.label }}</span>
            </button>
         </div>
      </div>
      
      <div class="relative group">
         <Search class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-dim group-focus-within:text-acid" />
         <input 
            v-model="librarySearchQuery"
            @keyup.enter="searchLibraryPackages"
            type="text" 
            placeholder="Search repository..." 
            class="bg-surface border border-white/10 h-9 pl-9 pr-4 text-sm font-mono text-white focus:outline-none focus:border-acid transition-colors w-64"
         >
      </div>
    </div>

    <!-- Grid -->
    <div class="flex-1 overflow-y-auto p-6">
       <div v-if="libraryLoading" class="flex flex-col items-center justify-center h-full text-dim">
          <Loader2 class="w-8 h-8 animate-spin mb-4 text-acid" />
          <p class="font-mono text-xs">FETCHING_MANIFESTS...</p>
       </div>

       <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          <div 
             v-for="pkg in displayedLibraryPackages"
             :key="pkg.id"
             class="group relative bg-surface border border-white/10 p-5 hover:border-acid/50 hover:bg-surfaceHighlight transition-all flex flex-col justify-between h-full min-h-[12rem]"
          >
             <div>
                <div class="flex items-start justify-between mb-3">
                   <div class="w-10 h-10 bg-main rounded-sm flex items-center justify-center overflow-hidden border border-white/5">
                      <img 
                        v-if="getPackageFavicon(pkg.publisher)" 
                        :src="getPackageFavicon(pkg.publisher)" 
                        @error="($event.target as HTMLImageElement).style.display = 'none'"
                        class="w-6 h-6 object-contain opacity-80 group-hover:opacity-100 transition-opacity" 
                      />
                      <Package v-else class="w-5 h-5 text-dim" />
                   </div>
                   <div class="text-[10px] font-mono text-dim border border-white/10 px-1.5 py-0.5 rounded-sm bg-main">
                      {{ pkg.latest_version }}
                   </div>
                </div>
                
                <h3 class="font-bold text-white text-sm line-clamp-1 mb-1">{{ pkg.name }}</h3>
                <p class="text-xs text-dim line-clamp-2 mb-2 min-h-[2.5em]">{{ pkg.description || 'No description available' }}</p>
                
                <div class="flex flex-wrap gap-1 mt-2">
                   <span v-for="tag in pkg.tags.slice(0, 2)" :key="tag" class="text-[9px] uppercase tracking-wider text-dim bg-white/5 px-1.5 py-0.5 rounded-sm">
                      {{ tag }}
                   </span>
                </div>
             </div>

             <button 
                @click="installApiPackage(pkg)"
                :disabled="installingPackage === pkg.id"
                class="w-full mt-3 bg-white/5 border border-white/10 text-white text-xs font-bold py-2 hover:bg-acid hover:text-black hover:border-acid transition-all flex items-center justify-center gap-2"
             >
                <Download v-if="installingPackage !== pkg.id" class="w-3 h-3" />
                <Loader2 v-else class="w-3 h-3 animate-spin" />
                {{ installingPackage === pkg.id ? 'INSTALLING...' : 'INSTALL' }}
             </button>
          </div>
       </div>
    </div>
    
    <!-- Pagination Footer -->
    <div v-if="!libraryLoading && totalPackages > 50" class="h-12 border-t border-white/10 bg-surface flex items-center justify-between px-6 shrink-0">
       <div class="text-xs text-dim font-mono">
          Showing {{ currentPage * 50 + 1 }}-{{ Math.min((currentPage + 1) * 50, totalPackages) }} of {{ totalPackages }}
       </div>
       
       <div class="flex gap-2">
          <button 
             :disabled="currentPage === 0"
             @click="loadLibraryPackages(currentPage - 1)"
             class="p-1 px-3 border border-white/10 text-dim text-xs font-mono hover:bg-white/10 disabled:opacity-30"
          >
             PREV
          </button>
          <button 
             :disabled="(currentPage + 1) * 50 >= totalPackages"
             @click="loadLibraryPackages(currentPage + 1)"
             class="p-1 px-3 border border-white/10 text-dim text-xs font-mono hover:bg-white/10 disabled:opacity-30"
          >
             NEXT
          </button>
       </div>
    </div>
  </div>
</template>
