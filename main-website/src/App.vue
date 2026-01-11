<script setup lang="ts">
import Navbar from './components/Navbar.vue';
import Hero from './components/Hero.vue';
import Features from './components/Features.vue';
import DownloadHub from './components/DownloadHub.vue';
import Footer from './components/Footer.vue';
</script>

<template>
  <div class="min-h-screen bg-main text-white selection:bg-acid/30">
    <Navbar />
    <main>
      <Hero />
      <Features />
      <DownloadHub />
    </main>
    <Footer />
  </div>
</template>
