<script setup lang="ts">
import { LayoutGrid, Package, Settings } from 'lucide-vue-next';

defineProps<{
  activeTab: string;
}>();

defineEmits<{
  (e: 'update:activeTab', tab: string): void;
}>();
</script>

<template>
  <aside class="w-64 border-r border-white/10 bg-surface/30 flex flex-col backdrop-blur-md">
    <!-- Header -->
    <div class="h-14 border-b border-white/10 flex items-center px-6 gap-3">
      <div class="w-3 h-3 bg-acid rounded-sm"></div>
      <span class="font-mono font-bold tracking-widest text-white uppercase">Installd_</span>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 p-4 space-y-1">
      <div class="px-3 py-2 text-xs font-mono text-dim font-bold mb-2">DASHBOARD</div>
      
      <button
        @click="$emit('update:activeTab', 'installed')"
        class="w-full flex items-center gap-3 px-3 py-2 text-sm font-mono transition-all group border border-transparent"
        :class="activeTab === 'installed' ? 'bg-acid/10 text-acid border-acid/20' : 'text-dim hover:bg-white/5 hover:text-white'"
      >
        <LayoutGrid class="w-4 h-4" />
        <span>INSTALLED</span>
      </button>

      <button
        @click="$emit('update:activeTab', 'foss')"
        class="w-full flex items-center gap-3 px-3 py-2 text-sm font-mono transition-all group border border-transparent"
        :class="activeTab === 'foss' ? 'bg-acid/10 text-acid border-acid/20' : 'text-dim hover:bg-white/5 hover:text-white'"
      >
        <Package class="w-4 h-4" />
        <span>LIBRARY</span>
      </button>
      
      <div class="px-3 py-2 text-xs font-mono text-dim font-bold mt-6 mb-2">SYSTEM</div>
      
      <button
        @click="$emit('update:activeTab', 'settings')"
        class="w-full flex items-center gap-3 px-3 py-2 text-sm font-mono transition-all group border border-transparent"
        :class="activeTab === 'settings' ? 'bg-acid/10 text-acid border-acid/20' : 'text-dim hover:bg-white/5 hover:text-white'"
      >
        <Settings class="w-4 h-4" />
        <span>SETTINGS</span>
      </button>
    </nav>

    <!-- Footer Status -->
    <div class="p-4 border-t border-white/10">
      <div class="flex items-center gap-2 text-[10px] font-mono text-dim">
        <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
        <span>SYSTEM.READY</span>
      </div>
      <div class="mt-1 text-[10px] font-mono text-dim opacity-50">
        v0.1.0-alpha
      </div>
    </div>
  </aside>
</template>
