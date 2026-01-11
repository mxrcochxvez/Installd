<script setup lang="ts">
import { FEATURES } from '../constants';
</script>

<template>
  <section id="features" class="py-32 bg-main border-b border-white/10">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex flex-col md:flex-row justify-between items-end mb-20 gap-8">
        <div>
          <h2 class="text-acid font-mono text-sm tracking-widest uppercase mb-4">// Capabilities</h2>
          <p class="text-4xl md:text-5xl font-bold text-white uppercase tracking-tight">
            System Optimization<br/>Protocol
          </p>
        </div>
        <p class="text-dim text-lg max-w-sm border-l border-white/20 pl-6 font-light">
           Standard Windows tools leave artifacts. Installd removes them entirely with a native, zero-latency interface.
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-px bg-white/10 border border-white/10">
        <div
          v-for="(feature, index) in FEATURES"
          :key="index"
          class="bg-main p-10 hover:bg-surface transition-colors group relative"
        >
          <div class="absolute top-4 right-4 text-[10px] font-mono text-white/10 group-hover:text-acid transition-colors">
            0{{ index + 1 }}
          </div>
          <div class="w-10 h-10 border border-white/20 flex items-center justify-center mb-8 bg-surface group-hover:border-acid transition-colors">
            <component :is="feature.icon" class="w-5 h-5 text-acid" />
          </div>
          <h3 class="text-xl font-bold text-white mb-4 font-mono uppercase">{{ feature.title }}</h3>
          <p class="text-dim leading-relaxed text-sm">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>
