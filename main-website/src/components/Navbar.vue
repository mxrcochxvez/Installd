<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { Menu, X, AppWindow, Check, AlertCircle } from 'lucide-vue-next';

interface SystemStatus {
  msg: string;
  type: 'success' | 'error';
}

const isScrolled = ref(false);
const isMobileMenuOpen = ref(false);
const systemStatus = ref<SystemStatus | null>(null);

const handleScroll = () => {
  isScrolled.value = window.scrollY > 20;
};

onMounted(() => {
  window.addEventListener('scroll', handleScroll);
});

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll);
});

const scrollToSection = (id: string) => {
  const element = document.getElementById(id);
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' });
    isMobileMenuOpen.value = false;
  }
};

const checkSystemCompatibility = () => {
  const userAgent = window.navigator.userAgent;
  // Simple check for Windows
  const isWindows = userAgent.includes('Windows');

  if (isWindows) {
    systemStatus.value = { msg: 'available for your system', type: 'success' };
  } else {
    systemStatus.value = { msg: 'not yet available for your system', type: 'error' };
  }

  // Hide after 3 seconds
  setTimeout(() => {
    systemStatus.value = null;
  }, 3000);
  
  isMobileMenuOpen.value = false;
};
</script>

<template>
  <nav
    :class="['fixed top-0 left-0 right-0 z-50 transition-all duration-200 border-b',
      isScrolled
        ? 'bg-main/90 backdrop-blur-sm border-white/10 py-3'
        : 'bg-transparent border-transparent py-6'
    ]"
  >
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center">
        <div class="flex items-center space-x-3 cursor-pointer group" @click="scrollToSection('hero')">
          <div class="bg-white text-black p-1.5 group-hover:bg-acid transition-colors duration-300">
            <AppWindow class="w-5 h-5" />
          </div>
          <span class="text-xl font-bold tracking-tight text-white font-mono uppercase">Installd_</span>
        </div>

        <!-- Desktop Nav -->
        <div class="hidden md:flex items-center space-x-12">
          <button @click="scrollToSection('features')" class="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
            Features
          </button>
          <button @click="scrollToSection('history')" class="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
            Index
          </button>
          <button @click="checkSystemCompatibility" class="text-dim hover:text-white transition-colors text-sm font-medium uppercase tracking-widest">
            Sys_Info
          </button>
          <button
            @click="scrollToSection('download')"
            class="bg-acid hover:bg-acid-hover text-black px-6 py-2 text-sm font-bold uppercase tracking-wider transition-all transform hover:-translate-y-0.5"
          >
            Get Installer
          </button>
        </div>

        <!-- Mobile Menu Button -->
        <div class="md:hidden">
          <button
            @click="isMobileMenuOpen = !isMobileMenuOpen"
            class="text-white p-2 border border-white/10"
          >
            <X v-if="isMobileMenuOpen" class="w-5 h-5" />
            <Menu v-else class="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>

    <!-- Mobile Nav -->
    <div v-if="isMobileMenuOpen" class="md:hidden absolute top-full left-0 right-0 bg-main border-b border-white/10 p-4">
      <div class="flex flex-col space-y-4">
        <button @click="scrollToSection('features')" class="text-left text-dim hover:text-white py-2 font-mono uppercase">
          // Features
        </button>
        <button @click="scrollToSection('history')" class="text-left text-dim hover:text-white py-2 font-mono uppercase">
          // Index
        </button>
        <button @click="checkSystemCompatibility" class="text-left text-dim hover:text-white py-2 font-mono uppercase">
          // Sys_Info
        </button>
        <button
          @click="scrollToSection('download')"
          class="bg-acid text-black py-3 font-bold uppercase tracking-wider text-center"
        >
          Initialize Download
        </button>
      </div>
    </div>

    <!-- Toast Notification -->
    <div v-if="systemStatus" class="absolute top-full left-0 right-0 md:left-auto md:right-8 md:top-24 md:w-auto flex justify-center pointer-events-none">
        <div
            :class="[
                'pointer-events-auto mt-4 md:mt-0 mx-4 md:mx-0 flex items-center gap-3 px-6 py-4 bg-surface border-l-4 shadow-2xl animate-in fade-in slide-in-from-top-4 duration-300',
                systemStatus.type === 'success' ? 'border-acid' : 'border-red-500'
            ]"
        >
            <Check v-if="systemStatus.type === 'success'" class="w-5 h-5 text-acid" />
            <AlertCircle v-else class="w-5 h-5 text-red-500" />
            <span class="font-mono text-sm font-bold uppercase tracking-wide text-white">
                {{ systemStatus.msg }}
            </span>
        </div>
    </div>
  </nav>
</template>
