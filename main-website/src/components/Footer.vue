<script setup lang="ts">
import { Github, Twitter } from 'lucide-vue-next';
</script>

<template>
  <footer class="bg-main py-16">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="border-t border-white/10 pt-8 flex flex-col md:flex-row justify-between items-start md:items-center gap-8">
        
        <div>
          <div class="font-mono text-lg font-bold text-white uppercase mb-2">Installd_</div>
          <p class="text-dim text-sm max-w-xs">
            Open source distribution system.<br/>
            Optimize your workflow.
          </p>
        </div>

        <div class="flex gap-8 text-sm font-mono uppercase tracking-wider">
           <a href="#" class="text-dim hover:text-acid transition-colors">Github</a>
           <a href="#" class="text-dim hover:text-acid transition-colors">Twitter</a>
           <a href="#" class="text-dim hover:text-acid transition-colors">Discord</a>
        </div>

        <div class="text-right">
           <p class="text-dim text-xs font-mono">
             © {{ new Date().getFullYear() }} INSTALLD PROJECT
           </p>
           <p class="text-white/20 text-[10px] font-mono mt-1">
             NO TRACKERS // NO ADS
           </p>
        </div>

      </div>
    </div>
  </footer>
</template>
