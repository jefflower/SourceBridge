<template>
  <div class="p-6">
    <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-bold">{{ $t('nav.dashboard') }}</h1>
        <button @click="generateReport" class="text-sm bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded font-medium flex items-center gap-2">
            <FileText class="w-4 h-4" />
            {{ $t('dashboard.weekly_report') }}
        </button>
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

    <div v-if="groups.length === 0" class="text-center py-12 text-muted-foreground">
        {{ $t('dashboard.welcome') }}
    </div>
    <AIResultModal ref="reportModal" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Folder, Waypoints, Rocket, FileText } from 'lucide-vue-next';
import StatusWidget from '@/components/dashboard/StatusWidget.vue';
import AIResultModal from '@/components/ai/AIResultModal.vue';

const groups = ref<any[]>([]);
const reportModal = ref<any>(null);

const loadGroups = async () => {
    try {
        const tree: any[] = await invoke('list_route_tree');
        // Filter out root virtual if empty or just not a real group
        // We only want top level groups for now? Or flatten?
        // Let's show top level groups.
        groups.value = tree.filter(n => n.id !== 'route_root_virtual');
    } catch (e) {
        console.error(e);
    }
};

const launchWorkspace = async (groupId: string) => {
    try {
        await invoke('launch_workspace', { groupId });
    } catch (e) {
        console.error(e);
        // If config missing, maybe redirect to settings or show toast?
        // For now alert
        // alert('Failed to launch: ' + e);
        // Better: silently fail or log, as per requirement "One-Click".
        // If no config, it just does nothing or errors.
    }
};

const generateReport = () => {
    reportModal.value?.open('weekly_report', '');
};

onMounted(() => {
    loadGroups();
});
</script>
