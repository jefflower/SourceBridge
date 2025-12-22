<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { initTheme } from '@/composables/useTheme';
import { isTauri } from '@/utils/tauriMock';

const { locale } = useI18n();
const tauriEnv = isTauri();

onMounted(async () => {
  initTheme();
  
  // Initialize Language
  try {
    const lang = await invoke<string | null>('get_setting', { key: 'language' });
    if (lang) {
      locale.value = lang;
    }
  } catch (e) {
    console.error('Failed to load language setting:', e);
  }
});
</script>

<template>
  <div class="h-screen flex flex-col overflow-hidden bg-background text-foreground">
    <div v-if="!tauriEnv" class="bg-yellow-500/10 text-yellow-600 dark:text-yellow-500 text-[10px] px-2 py-0.5 text-center border-b border-yellow-500/20 z-50">
      Mock Mode (Browser Development)
    </div>
    <router-view class="flex-1 overflow-hidden" />
  </div>
</template>

<style>
#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
