<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
    <div class="bg-background border rounded-lg shadow-lg w-full max-w-2xl max-h-[80vh] flex flex-col p-6">
      <h2 class="text-xl font-bold mb-4">{{ title }}</h2>

      <div v-if="loading" class="flex-1 flex items-center justify-center min-h-[200px]">
         <div class="flex flex-col items-center gap-2">
            <Loader2 class="w-8 h-8 animate-spin text-primary" />
            <span class="text-sm text-muted-foreground">{{ $t('ai.generating') }}</span>
         </div>
      </div>

      <div v-else class="flex-1 overflow-auto bg-muted/30 p-4 rounded-md font-mono text-sm whitespace-pre-wrap">
         {{ content }}
      </div>

      <div class="flex justify-end gap-2 mt-4">
        <button @click="copy" class="bg-secondary text-secondary-foreground hover:bg-secondary/90 px-4 py-2 rounded text-sm font-medium">
            {{ $t('actions.copy') }}
        </button>
        <button @click="close" class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium">
            {{ $t('window.close') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Loader2 } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';

const isOpen = ref(false);
const loading = ref(false);
const title = ref('');
const content = ref('');

const open = async (type: 'release_notes' | 'explain' | 'weekly_report', data: string) => {
    isOpen.value = true;
    loading.value = true;
    content.value = '';

    try {
        if (type === 'release_notes') {
            title.value = "AI Release Notes";
            // data should be diff summary or log
            content.value = await invoke('generate_release_notes', { diffSummary: data });
        } else if (type === 'weekly_report') {
            title.value = "Weekly Report";
            content.value = await invoke('generate_weekly_report');
        } else {
            title.value = "AI Explanation";
            // data is diff content
            content.value = await invoke('explain_diff', { diffContent: data });
        }
    } catch (e) {
        content.value = "Error: " + e;
    } finally {
        loading.value = false;
    }
};

const close = () => {
    isOpen.value = false;
};

const copy = async () => {
    try {
        await navigator.clipboard.writeText(content.value);
    } catch (e) {
        console.error(e);
    }
};

defineExpose({ open, close });
</script>
