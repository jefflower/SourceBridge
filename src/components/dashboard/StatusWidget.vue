<template>
  <div class="border rounded-lg p-4 bg-card shadow-sm h-full flex flex-col">
    <div class="flex items-center justify-between mb-2">
       <h3 class="font-semibold text-sm flex items-center gap-2">
         <Activity class="w-4 h-4 text-primary" />
         {{ $t('dashboard.status.title') }}
       </h3>
       <button
         @click="refresh"
         class="p-1 rounded hover:bg-muted text-muted-foreground"
         :title="$t('actions.refresh')"
         :disabled="loading"
        >
         <RefreshCw class="w-3 h-3" :class="{'animate-spin': loading}" />
       </button>
    </div>

    <div v-if="loading" class="flex-1 flex items-center justify-center">
       <span class="text-xs text-muted-foreground">{{ $t('common.loading') }}</span>
    </div>

    <div v-else-if="error" class="text-xs text-destructive p-2">
       {{ error }}
    </div>

    <div v-else class="flex-1 space-y-3">
        <!-- Summary Stats -->
        <div class="grid grid-cols-3 gap-2 text-center py-2 border-b">
             <div>
                <div class="text-lg font-bold text-orange-500">{{ stats.behind }}</div>
                <div class="text-[10px] text-muted-foreground">{{ $t('dashboard.status.behind') }}</div>
             </div>
             <div>
                <div class="text-lg font-bold text-blue-500">{{ stats.uncommitted }}</div>
                <div class="text-[10px] text-muted-foreground">{{ $t('dashboard.status.modified') }}</div>
             </div>
             <div>
                <div class="text-lg font-bold text-red-500">{{ stats.diverged }}</div>
                <div class="text-[10px] text-muted-foreground">{{ $t('dashboard.status.diverged') }}</div>
             </div>
        </div>

        <!-- List of problematic repos -->
        <div class="overflow-auto max-h-[200px] text-xs space-y-1">
             <div v-if="problematicRepos.length === 0" class="text-center py-4 text-muted-foreground italic">
                 {{ $t('dashboard.status.all_clean') }}
             </div>
             <div
                v-for="repo in problematicRepos"
                :key="repo.id"
                class="flex items-center justify-between p-1.5 rounded hover:bg-muted/50"
            >
                <div class="flex items-center gap-2 truncate">
                    <AlertCircle v-if="repo.status === 'Error'" class="w-3 h-3 text-destructive" />
                    <GitMerge v-else-if="repo.status === 'Diverged'" class="w-3 h-3 text-red-500" />
                    <ArrowDown v-else-if="repo.status === 'Behind'" class="w-3 h-3 text-orange-500" />
                    <Edit3 v-else class="w-3 h-3 text-blue-500" />
                    <span class="truncate font-medium">{{ repo.name }}</span>
                </div>
                <span class="text-[10px] bg-muted px-1.5 py-0.5 rounded text-muted-foreground whitespace-nowrap">
                    {{ repo.short_summary }}
                </span>
             </div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Activity, RefreshCw, AlertCircle, GitMerge, ArrowDown, Edit3 } from 'lucide-vue-next';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

interface RepoStatus {
    id: string;
    name: string;
    path: string;
    status: string;
    short_summary: string;
}

const loading = ref(false);
const error = ref('');
const repoStatuses = ref<Record<string, RepoStatus>>({});

const refresh = async () => {
    loading.value = true;
    error.value = '';
    try {
        const result: { statuses: Record<string, RepoStatus> } = await invoke('get_repos_status');
        repoStatuses.value = result.statuses;
    } catch (e) {
        error.value = String(e);
    } finally {
        loading.value = false;
    }
};

const stats = computed(() => {
    let behind = 0;
    let uncommitted = 0;
    let diverged = 0;

    Object.values(repoStatuses.value).forEach(s => {
        if (s.status === 'Behind') behind++;
        if (s.status === 'Uncommitted') uncommitted++;
        if (s.status === 'Diverged') diverged++;
    });

    return { behind, uncommitted, diverged };
});

const problematicRepos = computed(() => {
    return Object.values(repoStatuses.value)
        .filter(s => s.status !== 'Clean')
        .sort((a, b) => {
            // Sort priority: Error > Diverged > Behind > Uncommitted > Ahead
            const score = (status: string) => {
                switch(status) {
                    case 'Error': return 5;
                    case 'Diverged': return 4;
                    case 'Behind': return 3;
                    case 'Uncommitted': return 2;
                    case 'Ahead': return 1;
                    default: return 0;
                }
            };
            return score(b.status) - score(a.status);
        });
});

onMounted(() => {
    refresh();
});
</script>
