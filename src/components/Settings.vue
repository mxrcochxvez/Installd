<script setup lang="ts">
</script>

<template>
  <div class="h-full p-8 overflow-y-auto">
    <div class="max-w-2xl mx-auto space-y-12">
      
      <!-- Header -->
      <div>
        <h2 class="text-3xl font-bold text-white mb-2">Settings</h2>
        <p class="text-dim font-mono text-sm">Configure application preferences and view system info.</p>
      </div>

      <!-- About Section -->
      <div class="space-y-6">
        <h3 class="text-xs font-mono font-bold text-acid uppercase tracking-wider border-b border-white/10 pb-2">About Installd</h3>
        
        <div class="bg-surface border border-white/10 p-6 rounded-sm">
           <p class="text-dim leading-relaxed mb-4">
             A modern utility to help you discover and switch to free and open-source alternatives for your installed software.
             Built with Rust and Tauri for maximum performance and native system integration.
           </p>
           
           <div class="grid grid-cols-2 gap-4 mt-6">
              <div class="flex flex-col gap-1">
                 <span class="text-xs text-dim font-mono">VERSION</span>
                 <span class="text-white font-mono">v0.1.0-alpha</span>
              </div>
              <div class="flex flex-col gap-1">
                 <span class="text-xs text-dim font-mono">BUILD</span>
                 <span class="text-white font-mono">release/stable</span>
              </div>
           </div>
        </div>
      </div>

      <!-- Features Section -->
      <div class="space-y-6">
        <h3 class="text-xs font-mono font-bold text-acid uppercase tracking-wider border-b border-white/10 pb-2">Capabilities</h3>
        
        <div class="grid md:grid-cols-2 gap-4">
           <div class="p-4 bg-surface border border-white/10 hover:border-acid/50 transition-colors group">
              <div class="text-2xl mb-2">🔍</div>
              <h4 class="text-white font-bold mb-1 group-hover:text-acid">Scan</h4>
              <p class="text-xs text-dim">Deep scan of installed Windows software and registry entries.</p>
           </div>
           
           <div class="p-4 bg-surface border border-white/10 hover:border-acid/50 transition-colors group">
              <div class="text-2xl mb-2">🔄</div>
              <h4 class="text-white font-bold mb-1 group-hover:text-acid">Alternatives</h4>
              <p class="text-xs text-dim">Intelligent matching with FOSS alternatives from our database.</p>
           </div>

           <div class="p-4 bg-surface border border-white/10 hover:border-acid/50 transition-colors group">
              <div class="text-2xl mb-2">📦</div>
              <h4 class="text-white font-bold mb-1 group-hover:text-acid">Manage</h4>
              <p class="text-xs text-dim">Silent uninstallation and bulk package management.</p>
           </div>

           <div class="p-4 bg-surface border border-white/10 hover:border-acid/50 transition-colors group">
              <div class="text-2xl mb-2">⬇️</div>
              <h4 class="text-white font-bold mb-1 group-hover:text-acid">Winget</h4>
              <p class="text-xs text-dim">Native integration with Windows Package Manager.</p>
           </div>
        </div>
      </div>

    </div>
  </div>
</template>
