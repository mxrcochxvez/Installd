<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Search, Loader2, Trash2, Download, X } from 'lucide-vue-next';

// Types
interface InstalledSoftware {
  name: string;
  version: string;
  publisher: string;
  install_date: string;
  install_location: string;
  uninstall_string: string;
  icon_path: string;
  estimated_size: number;
}

interface FossApp {
  name: string;
  description: string;
  website: string;
  download_url: string;
  license: string;
  category: string;
  icon: string;
  winget_id: string | null;
}

interface SoftwareWithAlternatives {
  software: InstalledSoftware;
  has_alternatives: boolean;
  alternatives: FossApp[];
}

// State
const loading = ref(true);
const installedSoftware = ref<SoftwareWithAlternatives[]>([]);
const searchQuery = ref("");
const showOnlyWithAlternatives = ref(false);
const selectedSoftware = ref<SoftwareWithAlternatives | null>(null);
const uninstallStatus = ref("");
const iconCache = ref<Map<string, string | null>>(new Map());
const wingetAvailable = ref(false);
const installingPackage = ref<string | null>(null);
const installStatus = ref("");

// Computed
const filteredSoftware = computed(() => {
  let filtered = installedSoftware.value;

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(
      (s) =>
        s.software.name.toLowerCase().includes(query) ||
        s.software.publisher.toLowerCase().includes(query)
    );
  }

  if (showOnlyWithAlternatives.value) {
    filtered = filtered.filter((s) => s.has_alternatives);
  }

  return filtered;
});



// API Calls
async function loadInstalledSoftware() {
  loading.value = true;
  try {
    installedSoftware.value = await invoke<SoftwareWithAlternatives[]>(
      "get_installed_software"
    );
    loadIconsForSoftware();
  } catch (e) {
    console.error("Failed to load installed software:", e);
  }
  loading.value = false;
}

async function loadIcon(iconPath: string): Promise<string | null> {
  if (!iconPath) return null;
  if (iconCache.value.has(iconPath)) {
    return iconCache.value.get(iconPath) || null;
  }
  try {
    const icon = await invoke<string | null>("get_app_icon", { iconPath });
    iconCache.value.set(iconPath, icon);
    return icon;
  } catch (e) {
    iconCache.value.set(iconPath, null);
    return null;
  }
}

async function loadIconsForSoftware() {
  for (const item of installedSoftware.value) {
    if (item.software.icon_path && !iconCache.value.has(item.software.icon_path)) {
      loadIcon(item.software.icon_path);
    }
  }
}

function getIcon(iconPath: string): string | undefined {
  if (!iconPath) return undefined;
  return iconCache.value.get(iconPath) ?? undefined;
}

async function checkWingetAvailable() {
  try {
    wingetAvailable.value = await invoke<boolean>("check_winget_available");
  } catch (e) {
    wingetAvailable.value = false;
  }
}

// Actions
async function uninstallSoftware(software: InstalledSoftware) {
  if (!software.uninstall_string) {
    uninstallStatus.value = "No uninstaller available for this software";
    return;
  }

  try {
    uninstallStatus.value = await invoke<string>("uninstall_software", {
      uninstallString: software.uninstall_string,
    });
    setTimeout(loadInstalledSoftware, 2000);
  } catch (e) {
    uninstallStatus.value = `Error: ${e}`;
  }
}

async function installWithWinget(app: FossApp) {
  if (!app.winget_id) return;
  
  const confirmed = window.confirm(
    `Install ${app.name}?\n\nThis will install the software using Windows Package Manager (winget).`
  );
  
  if (!confirmed) return;
  
  installingPackage.value = app.winget_id;
  installStatus.value = `Installing ${app.name}...`;
  
  try {
    const result = await invoke<string>("install_winget", {
      packageId: app.winget_id,
    });
    installStatus.value = result;
  } catch (e) {
    installStatus.value = `Error: ${e}`;
  } finally {
    installingPackage.value = null;
    setTimeout(() => { installStatus.value = ""; }, 5000);
  }
}

async function downloadFossApp(url: string) {
  try {
    await invoke<string>("download_foss_app", { url });
  } catch (e) {
    console.error("Failed to open download page:", e);
  }
}

function selectSoftware(software: SoftwareWithAlternatives) {
  selectedSoftware.value = software;
}

function formatSize(kb: number): string {
  if (kb < 1024) return `${kb} KB`;
  if (kb < 1024 * 1024) return `${(kb / 1024).toFixed(1)} MB`;
  return `${(kb / (1024 * 1024)).toFixed(1)} GB`;
}

onMounted(() => {
  loadInstalledSoftware();
  checkWingetAvailable();
});
</script>

<template>
  <div class="h-full flex overflow-hidden">
    <!-- Main Content -->
    <div class="flex-1 flex flex-col min-w-0">
       <!-- Toolbar -->
       <div class="h-14 border-b border-white/10 flex items-center justify-between px-6 bg-main z-10">
          <h2 class="text-lg font-bold text-white tracking-tight">Installed Apps <span class="text-dim font-mono text-xs ml-2">({{ filteredSoftware.length }})</span></h2>
          
          <div class="flex items-center gap-4">
             <div class="relative group">
                <Search class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-dim group-focus-within:text-acid" />
                <input 
                  v-model="searchQuery" 
                  type="text" 
                  placeholder="Search apps..." 
                  class="bg-surface border border-white/10 h-9 pl-9 pr-4 text-sm font-mono text-white focus:outline-none focus:border-acid transition-colors w-64"
                >
             </div>
             
             <label class="flex items-center gap-2 cursor-pointer group">
                <div class="relative">
                   <input type="checkbox" v-model="showOnlyWithAlternatives" class="sr-only peer">
                   <div class="w-9 h-5 bg-surface border border-white/20 rounded-full peer-checked:bg-acid peer-checked:border-acid transition-colors"></div>
                   <div class="absolute left-1 top-1 w-3 h-3 bg-dim rounded-full transition-transform peer-checked:translate-x-4 peer-checked:bg-black"></div>
                </div>
                <span class="text-xs font-mono text-dim group-hover:text-white transition-colors">HAS ALTERNATIVES</span>
             </label>
          </div>
       </div>

       <!-- Grid -->
       <div class="flex-1 overflow-y-auto p-6">
          <div v-if="loading" class="flex flex-col items-center justify-center h-full text-dim">
             <Loader2 class="w-8 h-8 animate-spin mb-4 text-acid" />
             <p class="font-mono text-xs">SCANNING_SYSTEM...</p>
          </div>

          <div v-else-if="filteredSoftware.length === 0" class="flex flex-col items-center justify-center h-full text-dim">
             <div class="w-16 h-16 border border-white/10 flex items-center justify-center mb-4 rounded-full bg-surface">
                <Search class="w-6 h-6" />
             </div>
             <p>No software found matching filters</p>
          </div>

          <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
             <div 
               v-for="item in filteredSoftware" 
               :key="item.software.name"
               @click="selectSoftware(item)"
               class="group relative bg-surface border border-white/10 p-4 hover:border-acid/50 hover:bg-surfaceHighlight transition-all cursor-pointer h-full min-h-[8rem] flex flex-col justify-between"
               :class="{'ring-1 ring-acid border-acid': selectedSoftware?.software.name === item.software.name}"
             >
                <div class="flex items-start justify-between">
                   <div class="w-10 h-10 bg-main rounded-sm flex items-center justify-center overflow-hidden border border-white/5">
                      <img v-if="getIcon(item.software.icon_path)" :src="getIcon(item.software.icon_path) || ''" class="w-8 h-8 object-contain" />
                      <span v-else class="text-lg">📦</span>
                   </div>
                   <div v-if="item.has_alternatives" class="w-2 h-2 bg-acid rounded-full animate-pulse shadow-[0_0_8px_rgba(204,255,0,0.6)]"></div>
                </div>
                
                <div>
                   <h3 class="font-bold text-white text-sm truncate pr-2">{{ item.software.name }}</h3>
                   <div class="flex items-center gap-2 mt-1">
                      <span class="text-[10px] font-mono text-dim border border-white/10 px-1">{{ item.software.version ? 'v' + item.software.version : 'UNK' }}</span>
                      <span v-if="item.software.estimated_size" class="text-[10px] text-dim">{{ formatSize(item.software.estimated_size) }}</span>
                   </div>
                </div>
             </div>
          </div>
       </div>
    </div>

    <!-- Details Sidebar (Right) -->
    <div v-if="selectedSoftware" class="w-80 border-l border-white/10 bg-surface/50 backdrop-blur-md flex flex-col">
       <div class="p-6 border-b border-white/10 flex justify-between items-start">
          <h2 class="font-bold text-lg text-white leading-tight pr-4">{{ selectedSoftware.software.name }}</h2>
          <button @click="selectedSoftware = null" class="text-dim hover:text-white transition-colors">
             <X class="w-5 h-5" />
          </button>
       </div>
       
       <div class="flex-1 overflow-y-auto p-6 space-y-8">
          <!-- Info -->
          <div class="space-y-4">
             <h3 class="text-xs font-mono text-acid font-bold uppercase tracking-wider">Metdata</h3>
             <div class="grid grid-cols-2 gap-y-4 gap-x-2 text-sm">
                <div class="text-dim">Publisher</div>
                <div class="text-white text-right truncate">{{ selectedSoftware.software.publisher || '-' }}</div>
                
                <div class="text-dim">Install Date</div>
                <div class="text-white text-right truncate">{{ selectedSoftware.software.install_date }}</div>
                
                <div class="text-dim">Size</div>
                <div class="text-white text-right">{{ formatSize(selectedSoftware.software.estimated_size) }}</div>
             </div>
             
             <button 
               @click="uninstallSoftware(selectedSoftware.software)"
               class="w-full mt-4 flex items-center justify-center gap-2 bg-red-500/10 text-red-400 border border-red-500/20 py-2 text-xs font-bold uppercase tracking-wide hover:bg-red-500 hover:text-white transition-all"
             >
                <Trash2 class="w-3 h-3" />
                Uninstall
             </button>
             <div v-if="uninstallStatus" class="text-xs text-red-400 mt-2">{{ uninstallStatus }}</div>
          </div>

          <!-- Alternatives -->
          <div v-if="selectedSoftware.has_alternatives" class="space-y-4">
             <h3 class="text-xs font-mono text-acid font-bold uppercase tracking-wider">Recommended Alternatives</h3>
             
             <div v-for="alt in selectedSoftware.alternatives" :key="alt.name" class="bg-main border border-white/10 p-4 space-y-3 group hover:border-acid/30 transition-colors">
                 <div class="flex justify-between items-start">
                    <h4 class="font-bold text-white">{{ alt.name }}</h4>
                    <span class="text-[10px] font-mono text-acid border border-acid/20 px-1 bg-acid/5">{{ alt.license }}</span>
                 </div>
                 <p class="text-xs text-dim leading-relaxed">{{ alt.description }}</p>
                 
                 <div class="pt-2 flex gap-2">
                    <button 
                      v-if="wingetAvailable && alt.winget_id"
                      @click="installWithWinget(alt)"
                      :disabled="installingPackage === alt.winget_id"
                      class="flex-1 bg-acid text-black text-[10px] font-bold py-1.5 uppercase hover:bg-white transition-colors flex items-center justify-center gap-1"
                    >
                       <Download class="w-3 h-3" />
                       {{ installingPackage === alt.winget_id ? 'Installing...' : 'Install' }}
                    </button>
                    <button 
                      @click="downloadFossApp(alt.download_url)"
                      class="flex-1 border border-white/20 text-white text-[10px] font-bold py-1.5 uppercase hover:bg-white hover:text-black transition-colors"
                    >
                       Website
                    </button>
                 </div>
             </div>
          </div>
          
          <div v-else class="p-4 border border-dashed border-white/10 text-center">
             <p class="text-xs text-dim">No specific recommendations found for this app.</p>
          </div>
       </div>
    </div>
  </div>
</template>
