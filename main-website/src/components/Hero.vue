<script setup lang="ts">
import { MOCK_RELEASES } from '../constants';
import { Download, ArrowRight, ShieldCheck, Cpu, AppWindow } from 'lucide-vue-next';

const latestRelease = MOCK_RELEASES[0];

const scrollToDownload = () => {
  document.getElementById('download')?.scrollIntoView({ behavior: 'smooth' });
};
</script>

<template>
  <section id="hero" class="relative pt-32 pb-20 lg:pt-48 lg:pb-32 overflow-hidden border-b border-white/10">
    <!-- Technical Grid Background -->
    <div class="absolute inset-0 bg-grid-pattern bg-[length:40px_40px] opacity-[0.15] pointer-events-none"></div>
    <div class="absolute inset-0 bg-gradient-to-t from-main via-transparent to-transparent pointer-events-none"></div>

    <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      
      <div class="flex flex-col lg:flex-row items-center gap-16">
        <div class="flex-1 text-left">
          <div class="inline-flex items-center gap-3 px-4 py-2 border border-acid/30 bg-acid/5 mb-8">
            <span class="w-2 h-2 bg-acid animate-pulse"></span>
            <span class="text-acid font-mono text-xs uppercase tracking-widest">Build v{{ latestRelease.version }} // Native Rust</span>
          </div>

          <h1 class="text-6xl md:text-8xl font-bold tracking-tighter text-white mb-6 uppercase leading-[0.9]">
            Reclaim<br/>
            <span class="text-transparent bg-clip-text bg-gradient-to-r from-acid to-white/50">Your Space.</span><br/>
            <span class="text-dim">Your Wallet.</span>
          </h1>
          
          <p class="text-lg md:text-xl text-dim max-w-xl mb-10 font-light leading-relaxed border-l-2 border-white/20 pl-6">
            A lightning-fast native Windows utility built in Rust. 
            Zero webview overhead. Uninstall apps, visualize storage, and find FOSS alternatives instantly.
          </p>

          <div class="flex flex-col sm:flex-row gap-4">
            <button
              @click="scrollToDownload"
              class="px-8 py-4 bg-acid hover:bg-acid-hover text-black font-bold text-lg uppercase tracking-wider transition-all flex items-center justify-center gap-3 group"
            >
              <Download class="w-5 h-5" />
              <span>Download Installer</span>
            </button>
            <button 
               @click="() => document.getElementById('features')?.scrollIntoView({ behavior: 'smooth' })"
               class="px-8 py-4 bg-transparent hover:bg-white/5 text-white border border-white/20 font-bold text-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2"
            >
              View Features
              <ArrowRight class="w-5 h-5" />
            </button>
          </div>

          <div class="mt-12 flex items-center gap-8 text-dim text-xs font-mono uppercase tracking-widest">
            <div class="flex items-center gap-2">
              <ShieldCheck class="w-4 h-4 text-acid" />
              <span>Verified Safe</span>
            </div>
            <div class="flex items-center gap-2">
              <Cpu class="w-4 h-4 text-acid" />
              <span>Native WinAPI</span>
            </div>
            <div class="flex items-center gap-2">
              <AppWindow class="w-4 h-4 text-acid" />
              <span>&lt; 5MB Memory</span>
            </div>
          </div>
        </div>

        <!-- Native UI Mockup -->
        <div class="flex-1 w-full max-w-lg lg:max-w-full">
           <div class="relative aspect-square md:aspect-[4/3] border-2 border-white/10 bg-surface/50 p-2">
              <!-- Decorative corners -->
              <div class="absolute -top-1 -left-1 w-4 h-4 border-t-2 border-l-2 border-acid"></div>
              <div class="absolute -top-1 -right-1 w-4 h-4 border-t-2 border-r-2 border-acid"></div>
              <div class="absolute -bottom-1 -left-1 w-4 h-4 border-b-2 border-l-2 border-acid"></div>
              <div class="absolute -bottom-1 -right-1 w-4 h-4 border-b-2 border-r-2 border-acid"></div>
              
              <!-- Internal Window Mockup -->
              <div class="h-full border border-white/10 bg-main flex flex-col shadow-2xl overflow-hidden">
                
                <!-- Title Bar -->
                <div class="h-10 border-b border-white/10 bg-surface flex items-center justify-between px-4">
                  <div class="flex items-center gap-2">
                     <div class="w-3 h-3 bg-acid/80 rounded-sm"></div>
                     <span class="text-xs font-bold text-white tracking-wide">Installd</span>
                  </div>
                  <div class="flex gap-2">
                    <div class="w-3 h-3 border border-white/20"></div>
                    <div class="w-3 h-3 bg-white/20"></div>
                  </div>
                </div>
                
                <!-- App Content -->
                <div class="flex-1 flex">
                   <!-- Sidebar -->
                   <div class="w-16 md:w-48 border-r border-white/10 bg-surface/30 p-4 hidden sm:flex flex-col gap-4">
                      <div class="h-8 bg-acid/10 border-l-2 border-acid flex items-center px-3 text-xs font-mono text-acid font-bold">DASHBOARD</div>
                      <div class="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">PACKAGES</div>
                      <div class="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">STORAGE</div>
                      <div class="h-8 hover:bg-white/5 flex items-center px-3 text-xs font-mono text-dim">SETTINGS</div>
                   </div>

                   <!-- Main Area -->
                   <div class="flex-1 p-6 bg-main relative">
                      <div class="flex justify-between items-center mb-6">
                         <h3 class="text-xl font-bold text-white">Installd Apps</h3>
                         <div class="px-3 py-1 bg-surface border border-white/10 text-xs text-dim font-mono">Filter: All</div>
                      </div>

                      <!-- Grid of Apps -->
                      <div class="grid grid-cols-2 gap-4">
                         <div v-for="i in 4" :key="i" class="bg-surface/50 border border-white/10 p-4 hover:border-acid/50 transition-colors group cursor-pointer relative overflow-hidden">
                            <div class="flex items-start justify-between mb-3">
                               <div class="w-8 h-8 bg-white/10 group-hover:bg-acid/20 transition-colors"></div>
                               <div class="w-2 h-2 rounded-full bg-green-500"></div>
                            </div>
                            <div class="h-2 w-20 bg-white/20 mb-2"></div>
                            <div class="h-2 w-12 bg-white/10"></div>
                            
                            <div class="absolute inset-0 bg-acid/5 opacity-0 group-hover:opacity-100 transition-opacity"></div>
                         </div>
                      </div>

                      <!-- Floating Action Button simulation -->
                      <div class="absolute bottom-6 right-6 w-12 h-12 bg-acid text-black flex items-center justify-center shadow-lg shadow-acid/20">
                         <Download class="w-5 h-5" />
                      </div>
                   </div>
                </div>
              </div>
           </div>
        </div>
      </div>
    </div>
  </section>
</template>
