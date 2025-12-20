<template>
  <div class="max-w-2xl">
    <h1 class="text-2xl font-bold mb-6">{{ $t('settings.title') }}</h1>

    <div class="space-y-8">
      <!-- Appearance -->
      <section class="space-y-4">
        <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.appearance.title') }}</h2>
        <div class="grid gap-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.appearance.language') }}</label>
            <select v-model="settings.language" @change="saveSetting('language', settings.language)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50">
              <option value="en">English</option>
              <option value="zh">中文</option>
            </select>
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.appearance.theme') }}</label>
             <select v-model="settings.theme" @change="saveSetting('theme', settings.theme)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50">
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>
        </div>
      </section>

      <!-- Environment -->
      <section class="space-y-4">
        <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.env.title') }}</h2>
        <div class="grid gap-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.env.git_path') }}</label>
            <input type="text" v-model="settings.git_path" @change="saveSetting('git_path', settings.git_path)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" />
          </div>
        </div>
      </section>

      <!-- System -->
      <section class="space-y-4">
         <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.system.title') }}</h2>
         <div class="flex items-center space-x-2">
             <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
                 {{ $t('settings.system.check') }}
             </button>
         </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();

const settings = ref({
    language: 'en',
    theme: 'system',
    git_path: 'git',
});

onMounted(async () => {
    try {
        const allSettings = await invoke<Record<string, string>>('get_all_settings');
        if (allSettings.language) {
            settings.value.language = allSettings.language;
            locale.value = allSettings.language;
        }
        if (allSettings.theme) settings.value.theme = allSettings.theme;
        if (allSettings.git_path) settings.value.git_path = allSettings.git_path;
    } catch (e) {
        console.error('Failed to load settings:', e);
    }
});

const saveSetting = async (key: string, value: string) => {
    try {
        await invoke('set_setting', { key, value });
        if (key === 'language') {
            locale.value = value;
        }
        console.log(`Saved ${key}: ${value}`);
    } catch (e) {
        console.error(`Failed to save ${key}:`, e);
    }
};
</script>
