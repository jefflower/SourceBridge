<template>
  <div class="max-w-2xl h-full overflow-auto p-2">
    <h1 class="text-2xl font-bold mb-6">{{ $t('settings.title') }}</h1>

    <div class="space-y-8 pb-8">
      <!-- Appearance -->
      <section class="space-y-4">
        <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.appearance.title') }}</h2>
        <div class="grid gap-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.appearance.language') }}</label>
            <select v-model="settings.language" @change="saveSetting('language', settings.language)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50">
              <option value="en">{{ $t('settings.appearance.language_options.en') }}</option>
              <option value="zh">{{ $t('settings.appearance.language_options.zh') }}</option>
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

      <!-- AI Configuration -->
      <section class="space-y-4">
        <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.ai.title') }}</h2>
        <div class="grid gap-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.ai.endpoint') }}</label>
            <input type="text" v-model="settings.ai_endpoint" @change="saveSetting('ai_endpoint', settings.ai_endpoint)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" placeholder="http://localhost:11434" />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.ai.model') }}</label>
            <input type="text" v-model="settings.ai_model" @change="saveSetting('ai_model', settings.ai_model)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" placeholder="llama3" />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.ai.api_key') }}</label>
            <input type="password" v-model="settings.ai_api_key" @change="saveSetting('ai_api_key', settings.ai_api_key)" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" placeholder="Optional" />
          </div>
          <p class="text-xs text-muted-foreground col-span-4 pl-1">
             {{ $t('settings.ai.desc') }}
          </p>
        </div>
      </section>

      <!-- Environment -->
      <section class="space-y-4">
        <h2 class="text-lg font-semibold border-b pb-2">{{ $t('settings.env.title') }}</h2>
        <div class="grid gap-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <label class="text-sm font-medium">{{ $t('settings.env.git_path') }}</label>
            <div class="col-span-3 flex gap-2">
              <input type="text" v-model="settings.git_path" @change="saveSetting('git_path', settings.git_path)" class="flex-1 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" />
              <button type="button" @click="browseGitPath" class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-3">
                {{ $t('common.browse') }}
              </button>
            </div>
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
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';
import { setTheme, type Theme } from '@/composables/useTheme';

const { t: $t } = useI18n();

const { locale } = useI18n();

const settings = ref({
    language: 'en',
    theme: 'system' as Theme,
    git_path: 'git',
    ai_endpoint: 'http://localhost:11434',
    ai_model: 'llama3',
    ai_api_key: '',
});

onMounted(async () => {
    try {
        const allSettings = await invoke<Record<string, string>>('get_all_settings');
        if (allSettings.language) {
            settings.value.language = allSettings.language;
            locale.value = allSettings.language;
        }
        if (allSettings.theme) settings.value.theme = allSettings.theme as Theme;
        if (allSettings.git_path) settings.value.git_path = allSettings.git_path;

        if (allSettings.ai_endpoint) settings.value.ai_endpoint = allSettings.ai_endpoint;
        if (allSettings.ai_model) settings.value.ai_model = allSettings.ai_model;
        if (allSettings.ai_api_key) settings.value.ai_api_key = allSettings.ai_api_key;

    } catch (e) {
        console.error('Failed to load settings:', e);
    }
});

const saveSetting = async (key: string, value: string) => {
    try {
        if (key === 'theme') {
            // Use setTheme to apply theme immediately
            await setTheme(value as Theme);
        } else {
            await invoke('set_setting', { key, value });
        }
        if (key === 'language') {
            locale.value = value;
        }
        console.log(`Saved ${key}: ${value}`);
    } catch (e) {
        console.error(`Failed to save ${key}:`, e);
    }
};

const browseGitPath = async () => {
    const selected = await open({
        directory: false,
        multiple: false,
        title: $t('settings.env.select_git_executable_title')
    });
    if (selected && typeof selected === 'string') {
        settings.value.git_path = selected;
        await saveSetting('git_path', selected);
    }
};
</script>
