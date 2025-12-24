<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-background/90 backdrop-blur-sm">
    <div class="bg-background border rounded-lg shadow-lg w-11/12 h-5/6 flex flex-col overflow-hidden">
        <!-- Header -->
        <div class="border-b p-4 flex flex-col gap-2">
           <div class="flex items-center justify-between">
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

           <!-- Dependency Warnings -->
           <div v-if="depWarnings.length > 0" class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-2 text-sm flex items-start gap-2 max-h-24 overflow-auto">
               <AlertTriangle class="w-4 h-4 mt-0.5 flex-shrink-0" />
               <div class="flex-1">
                   <div class="font-bold mb-1">{{ $t('scan.dependency_warning_title', {count: depWarnings.length}) }}</div>
                   <div v-for="(warn, i) in depWarnings" :key="i" class="text-xs font-mono">
                      {{ warn.package_name }}: {{ warn.source_version }} (Source) vs {{ warn.target_version }} (Target) - {{ warn.severity }}
                   </div>
               </div>
           </div>
        </div>

        <!-- Content -->
        <div class="flex-1 flex overflow-hidden">
            <!-- Sidebar -->
            <div class="w-64 flex-shrink-0">
                <div v-if="loading" class="p-4 text-center text-muted-foreground">{{ $t('route.diff.loading') }}</div>
                <FileChangeTree v-else :changes="changes" :selectedFile="selectedFile" @select="selectFile" />
            </div>

            <!-- Editor -->
            <div class="flex-1 border-l relative flex flex-col">
                <div v-if="selectedFile" class="border-b p-2 flex justify-end bg-muted/10">
                    <button
                        @click="explainDiff"
                        class="text-xs flex items-center gap-1 bg-primary/10 text-primary hover:bg-primary/20 px-2 py-1 rounded"
                        :title="$t('ai.explain_diff.title')"
                    >
                        <Sparkles class="w-3 h-3" />
                        {{ $t('ai.explain_diff.title') }}
                    </button>
                </div>

                <div class="flex-1 relative">
                    <div v-if="loadingContent" class="absolute inset-0 flex items-center justify-center bg-background/50 z-10">
                        {{ $t('route.diff.loading_content') }}
                    </div>
                    <MonacoDiffEditor
                        v-if="selectedFile"
                        :key="sideBySide ? 'split' : 'inline'"
                        :original="content.original"
                        :modified="content.modified"
                        :renderSideBySide="sideBySide"
                    />
                    <div v-else class="h-full flex items-center justify-center text-muted-foreground">
                        {{ $t('route.diff.select_file_to_view_diff') }}
                    </div>
                </div>
            </div>
        </div>
    </div>
    <AIResultModal ref="aiModalRef" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { AlertTriangle, Sparkles } from 'lucide-vue-next';
import FileChangeTree from './FileChangeTree.vue';
import MonacoDiffEditor from './MonacoDiffEditor.vue';
import AIResultModal from '@/components/ai/AIResultModal.vue';

const isOpen = ref(false);
const loading = ref(false);
const loadingContent = ref(false);
const route = ref<any>(null);
const changes = ref<any[]>([]);
const selectedFile = ref<any>(null);
const content = ref({ original: '', modified: '' });
const sideBySide = ref(true);
const depWarnings = ref<any[]>([]);
const aiModalRef = ref<any>(null);

const open = async (r: any) => {
    route.value = r;
    isOpen.value = true;
    loading.value = true;
    changes.value = [];
    selectedFile.value = null;
    content.value = { original: '', modified: '' };
    depWarnings.value = [];

    try {
        // Fetch diff
        const summary: any = await invoke('preview_route_diff', { routeId: r.id });
        changes.value = summary.changes;

        // Check dependencies (if paths available in route/repos)
        // We need source/target paths.
        // `preview_route_diff` backend logic knows paths, but frontend only has route ID.
        // We can fetch details or just ask backend to scan.
        // Or we can get repo paths from repo list if available.
        // Let's assume we can call scan_dependencies if we know the paths.
        // Or better, `preview_route_diff` could return warnings?
        // Modifying `preview_route_diff` is cleaner but changing existing contract.

        // Let's look up paths.
        // We can fetch route details.
        const routeDetails: any = await invoke('get_route_details', { id: r.id });
        if (routeDetails && routeDetails.main_repo_id && routeDetails.slave_repo_id) {
             // We need actual paths.
             // This requires fetching repos. Too many calls?
             // Maybe we can expose a command `scan_route_dependencies`?
             // Or just pass the paths if we have them.

             // Let's iterate repos prop if we had it, but this modal is generic.
             // Let's create `scan_route_dependencies` in backend for convenience?
             // Or just use `scan_dependencies` if we fetch repo paths.

             // Quick hack: fetch all repos and find paths.
             const tree: any[] = await invoke('list_repo_tree');
             let sourcePath = '';
             let targetPath = '';

             const findRepo = (nodes: any[], id: string): string | null => {
                for (const node of nodes) {
                   if (node.repos) {
                      const found = node.repos.find((rp: any) => rp.id === id);
                      if (found) return found.path;
                   }
                   if (node.children) {
                      const res = findRepo(node.children, id);
                      if (res) return res;
                   }
                }
                return null;
             };

             sourcePath = findRepo(tree, routeDetails.main_repo_id) || '';
             targetPath = findRepo(tree, routeDetails.slave_repo_id) || '';

             if (sourcePath && targetPath) {
                 depWarnings.value = await invoke('scan_dependencies', { sourcePath, targetPath });
             }
        }
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

const explainDiff = () => {
    if (!content.value.original && !content.value.modified) return;

    // Construct diff string approximation or send content
    const diffText = `Old:\n${content.value.original}\n\nNew:\n${content.value.modified}`;
    aiModalRef.value?.open('explain', diffText);
};

defineExpose({ open, close });
</script>
