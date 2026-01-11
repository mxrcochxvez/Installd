<script setup lang="ts">
import { ref } from 'vue';
import TopBar from './components/TopBar.vue';
import Sidebar from './components/Sidebar.vue';
import SoftwareGrid from './components/SoftwareGrid.vue';
import FossLibrary from './components/FossLibrary.vue';
import Settings from './components/Settings.vue';

const activeTab = ref('installed');
</script>

<template>
  <div class="h-screen w-screen bg-main text-white flex flex-col overflow-hidden font-sans select-none border border-white/10 rounded-lg shadow-2xl relative">
    <!-- Grid Background -->
    <div class="absolute inset-0 bg-grid-pattern bg-[length:24px_24px] pointer-events-none opacity-20"></div>

    <TopBar title="System Manager" />
    
    <div class="flex-1 flex overflow-hidden z-10">
      <Sidebar :activeTab="activeTab" @update:activeTab="activeTab = $event" />
      
      <main class="flex-1 overflow-hidden bg-main/50 relative">
        <KeepAlive>
           <component :is="activeTab === 'installed' ? SoftwareGrid : activeTab === 'foss' ? FossLibrary : Settings" />
        </KeepAlive>
      </main>
    </div>
  </div>
</template>
