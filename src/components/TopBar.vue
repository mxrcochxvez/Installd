<script setup lang="ts">
import { Minus, Square, X } from 'lucide-vue-next';
import { getCurrentWindow } from '@tauri-apps/api/window';

defineProps<{
  title: string;
}>();

const appWindow = getCurrentWindow();

function minimize() {
  appWindow.minimize();
}

function maximize() {
  appWindow.toggleMaximize();
}

function close() {
  appWindow.close();
}
</script>

<template>
  <div data-tauri-drag-region class="h-10 bg-main border-b border-white/10 flex justify-between items-center select-none z-50">
    <!-- Window Title -->
    <div class="px-4 flex items-center gap-2 pointer-events-none">
       <span class="text-xs font-mono text-dim uppercase tracking-wider">Installd // {{ title }}</span>
    </div>

    <!-- Window Controls -->
    <div class="flex h-full">
      <button 
        @click="minimize" 
        class="w-12 h-full flex items-center justify-center text-dim hover:text-white hover:bg-white/5 transition-colors"
      >
        <Minus class="w-4 h-4" />
      </button>
      <button 
        @click="maximize" 
        class="w-12 h-full flex items-center justify-center text-dim hover:text-white hover:bg-white/5 transition-colors"
      >
        <Square class="w-3 h-3" />
      </button>
      <button 
        @click="close" 
        class="w-12 h-full flex items-center justify-center text-dim hover:text-white hover:bg-red-500 transition-colors"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>
