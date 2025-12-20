<template>
  <div class="flex flex-col h-screen w-screen overflow-hidden bg-background text-foreground">
    <TitleBar />
    <div class="flex flex-1 overflow-hidden">
      <SideBar />
      <main class="flex-1 overflow-auto p-6 pt-12">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import TitleBar from './TitleBar.vue';
import SideBar from './SideBar.vue';
</script>
