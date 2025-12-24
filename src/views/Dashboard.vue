<template>
  <div class="p-6">
    <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-bold">{{ $t('nav.dashboard') }}</h1>
        <button @click="generateReport" class="text-sm bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded font-medium flex items-center gap-2">
            <FileText class="w-4 h-4" />
            {{ $t('dashboard.weekly_report') }}
        </button>
    </div>

    <!-- Pinned Repositories -->
    <div v-if="pinnedRepos.length > 0" class="mb-8">
        <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
            <Pin class="w-4 h-4" />
            Pinned Repositories
        </h2>
        <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Card v-for="repo in pinnedRepos" :key="repo.id" class="cursor-pointer hover:bg-accent/50 transition-colors" @click="goToRepo(repo)">
                <CardContent class="p-4 flex flex-col gap-2">
                    <div class="flex items-center justify-between">
                        <span class="font-medium truncate" :title="repo.name">{{ repo.name }}</span>
                        <Package class="w-4 h-4 text-muted-foreground" />
                    </div>
                    <code class="text-xs text-muted-foreground bg-muted p-1 rounded truncate">{{ repo.path }}</code>
                </CardContent>
            </Card>
        </div>
    </div>

    <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
       <!-- Status Widget -->
       <StatusWidget class="row-span-2" />

       <!-- Route Groups / Workspaces -->
       <div v-for="group in groups" :key="group.id" class="border rounded-lg p-4 bg-card shadow-sm">
          <div class="flex items-center justify-between mb-2">
             <div class="flex items-center gap-2">
                 <Folder class="w-5 h-5 text-primary" />
                 <h3 class="font-semibold">{{ group.name }}</h3>
             </div>
             <button
                @click="launchWorkspace(group.id)"
                class="text-xs bg-secondary text-secondary-foreground px-2 py-1 rounded hover:opacity-80 flex items-center gap-1"
                :title="$t('workspace.launch')"
            >
                <Rocket class="w-3 h-3" />
                {{ $t('workspace.launch') }}
             </button>
          </div>

          <div class="text-sm text-muted-foreground mb-4">
             {{ group.routes.length }} {{ $t('nav.routes') }}
          </div>

          <div class="space-y-1">
             <div v-for="route in group.routes.slice(0, 3)" :key="route.id" class="text-xs flex items-center gap-2 p-1 rounded hover:bg-muted/50">
                <Waypoints class="w-3 h-3 text-muted-foreground" />
                <span class="truncate">{{ route.name }}</span>
             </div>
             <div v-if="group.routes.length > 3" class="text-xs text-muted-foreground pl-1">
                + {{ group.routes.length - 3 }} more
             </div>
          </div>
       </div>
    </div>

    <div v-if="groups.length === 0 && pinnedRepos.length === 0" class="text-center py-12 text-muted-foreground">
        <p>{{ $t('dashboard.welcome', 'Welcome to SourceBridge.') }}</p>
    </div>
    <AIResultModal
        :isOpen="showReportModal"
        title="Weekly Report"
        :content="reportContent"
        @close="showReportModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { Folder, Waypoints, Rocket, FileText, Pin, Package } from 'lucide-vue-next';
import StatusWidget from '@/components/dashboard/StatusWidget.vue';
import AIResultModal from '@/components/common/AIResultModal.vue';
import { Card, CardContent } from '@/components/ui/card';

const router = useRouter();
const groups = ref<any[]>([]);
const pinnedRepos = ref<any[]>([]);
const showReportModal = ref(false);
const reportContent = ref('');

const loadGroups = async () => {
    try {
        const tree: any[] = await invoke('list_route_tree');
        groups.value = tree.filter(n => n.id !== 'route_root_virtual');
    } catch (e) {
        console.error(e);
    }
};

const loadPinnedRepos = async () => {
    try {
        const tree: any[] = await invoke('list_repo_tree');
        const pinned: any[] = [];

        const traverse = (nodes: any[]) => {
            for (const node of nodes) {
                if (node.repos) {
                    for (const repo of node.repos) {
                        if (repo.pinned) {
                            pinned.push(repo);
                        }
                    }
                }
                if (node.children) {
                    traverse(node.children);
                }
            }
        };

        traverse(tree);
        pinnedRepos.value = pinned;
    } catch (e) {
        console.error('Failed to load pinned repos', e);
    }
}

const launchWorkspace = async (groupId: string) => {
    try {
        await invoke('launch_workspace', { groupId });
    } catch (e) {
        console.error(e);
    }
};

const generateReport = async () => {
    try {
        const res = await invoke('generate_weekly_report');
        reportContent.value = res as string;
        showReportModal.value = true;
    } catch(e) {
        console.error(e);
        alert(e);
    }
};

const goToRepo = (repo: any) => {
    router.push({ path: '/repos', query: { select: repo.id } });
};

onMounted(() => {
    loadGroups();
    loadPinnedRepos();
});
</script>
