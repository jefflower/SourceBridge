<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-background/90 backdrop-blur-sm">
    <div class="bg-background border rounded-lg shadow-lg w-11/12 h-5/6 flex flex-col overflow-hidden">
        <!-- Header -->
        <div class="border-b p-4 flex items-center justify-between">
            <h2 class="font-bold text-lg">{{ $t('route.diff.title') }}: {{ route?.name }}</h2>
            <div class="flex gap-2">
                <button @click="sideBySide = !sideBySide" class="px-3 py-1 text-sm border rounded hover:bg-muted">
                    {{ sideBySide ? $t('route.diff.view.inline') : $t('route.diff.view.split') }}
                </button>
                <button @click="confirmSync" class="px-3 py-1 text-sm bg-green-600 text-white rounded hover:bg-green-700">
                    {{ $t('route.diff.exec_sync') }}
                </button>
                <button @click="close" class="px-3 py-1 text-sm border rounded hover:bg-muted">
                    {{ $t('window.close') }}
                </button>
            </div>
        </div>

        <!-- Content -->
        <div class="flex-1 flex overflow-hidden">
            <!-- Sidebar -->
            <div class="w-64 flex-shrink-0">
                <div v-if="loading" class="p-4 text-center text-muted-foreground">{{ $t('diff.loading') }}</div>
                <FileChangeTree v-else :changes="changes" :selectedFile="selectedFile" @select="selectFile" />
            </div>

            <!-- Editor -->
            <div class="flex-1 border-l relative">
                <div v-if="loadingContent" class="absolute inset-0 flex items-center justify-center bg-background/50 z-10">
                    Loading content...
                </div>
                <MonacoDiffEditor
                    v-if="selectedFile"
                    :key="sideBySide ? 'split' : 'inline'"
                    :original="content.original"
                    :modified="content.modified"
                    :renderSideBySide="sideBySide"
                />
                <div v-else class="h-full flex items-center justify-center text-muted-foreground">
                    Select a file to view diff
                </div>
            </div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import FileChangeTree from './FileChangeTree.vue';
import MonacoDiffEditor from './MonacoDiffEditor.vue';

const isOpen = ref(false);
const loading = ref(false);
const loadingContent = ref(false);
const route = ref<any>(null);
const changes = ref<any[]>([]);
const selectedFile = ref<any>(null);
const content = ref({ original: '', modified: '' });
const sideBySide = ref(true);

const open = async (r: any) => {
    route.value = r;
    isOpen.value = true;
    loading.value = true;
    changes.value = [];
    selectedFile.value = null;
    content.value = { original: '', modified: '' };

    try {
        const summary: any = await invoke('preview_route_diff', { routeId: r.id });
        changes.value = summary.changes;
    } catch (e) {
        console.error("Failed to preview diff:", e);
        alert("Diff failed: " + e);
    } finally {
        loading.value = false;
    }
};

const close = () => {
    isOpen.value = false;
};

const confirmSync = async () => {
    if (!route.value) return;
    try {
         await invoke('sync_route', { id: route.value.id });
         // Alert or Toast
         alert("Sync triggered successfully!");
         close();
    } catch (e) {
        console.error(e);
        alert("Sync failed: " + e);
    }
}

const selectFile = async (file: any) => {
    selectedFile.value = file;
    loadingContent.value = true;
    try {
        const [original, modified] = await invoke<[string, string]>('get_file_diff', {
            sourcePath: file.source_path,
            targetPath: file.target_path
        });
        content.value = { original, modified };
    } catch (e) {
        console.error("Failed to load file diff:", e);
    } finally {
        loadingContent.value = false;
    }
};

defineExpose({ open, close });
</script>
