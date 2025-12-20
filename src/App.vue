<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { initTheme } from '@/composables/useTheme';

const { locale } = useI18n();

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
  <router-view />
</template>
