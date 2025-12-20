<template>
  <div class="h-full flex flex-col bg-background">
    <!-- Header -->
    <div class="border-b p-6 flex flex-col gap-2">
      <div class="flex items-center gap-2 text-muted-foreground text-sm">
        <Package class="w-4 h-4" />
        <span>{{ repo.id }}</span>
        <!-- In real app, maybe show path breadcrumbs -->
      </div>
      <h1 class="text-2xl font-bold tracking-tight">{{ repo.name }}</h1>
      <div class="flex items-center gap-4 text-sm mt-2">
        <code class="bg-muted px-2 py-1 rounded">{{ repo.path }}</code>
        <!-- Actions -->
      </div>
    </div>

    <!-- Tabs Header -->
    <div class="flex border-b px-6">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        @click="currentTab = tab.value"
        class="px-4 py-3 text-sm font-medium border-b-2 transition-colors"
        :class="currentTab === tab.value ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Tab Content -->
    <div class="flex-1 overflow-auto p-6">
      <div v-if="currentTab === 'overview'">
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
          <h3 class="text-lg font-semibold mb-4">Git Status</h3>

          <div class="grid gap-4 max-w-md">
              <div class="flex items-center justify-between">
                  <span class="text-sm font-medium text-muted-foreground">{{ $t('repo.branch.label', 'Branch') }}</span>
                  <div class="flex items-center gap-2">
                      <select
                        v-model="currentBranch"
                        @change="onBranchChange"
                        :disabled="isLoadingBranches"
                        class="h-9 w-48 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                      >
                          <option v-for="b in branches" :key="b.name" :value="b.name">
                              {{ b.name }} {{ b.is_remote ? '(Remote)' : '' }}
                          </option>
                      </select>
                      <Loader2 v-if="isLoadingBranches" class="w-4 h-4 animate-spin text-muted-foreground" />
                  </div>
              </div>
              <!-- Placeholder for commit info, could be expanded later -->
          </div>
        </div>
      </div>

      <div v-if="currentTab === 'settings'">
        <div class="grid gap-4 max-w-xl">
            <div class="grid gap-2">
                <label class="text-sm font-medium">{{ $t('repo.form.name.label') }}</label>
                <input v-model="localRepo.name" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
            </div>
            <div class="grid gap-2">
                <label class="text-sm font-medium">{{ $t('repo.form.path.label') }}</label>
                <div class="flex gap-2">
                    <input v-model="localRepo.path" class="flex-1 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    <button type="button" @click="browsePath" class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-3">
                        Browse
                    </button>
                </div>
            </div>
             <div class="flex justify-end gap-2 mt-4">
                <button @click="deleteRepo" class="bg-destructive text-destructive-foreground hover:bg-destructive/90 px-4 py-2 rounded text-sm font-medium">
                    {{ $t('repo.context.delete') }}
                </button>
                <button @click="save" class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium">
                    {{ $t('actions.save') }}
                </button>
            </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { Package, Loader2 } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const props = defineProps<{
  repo: any;
}>();

const emit = defineEmits(['update', 'delete']);

const tabs = [
  { label: 'Overview', value: 'overview' },
  { label: 'Settings', value: 'settings' },
];

const currentTab = ref('overview');
const localRepo = ref({ ...props.repo });
const branches = ref<any[]>([]);
const currentBranch = ref('');
const isLoadingBranches = ref(false);

const loadBranches = async () => {
    if (!props.repo.path) return;
    isLoadingBranches.value = true;
    try {
        const res = await invoke('get_git_branches', { path: props.repo.path });
        branches.value = res as any[];
        const active = branches.value.find((b: any) => b.is_current);
        if (active) currentBranch.value = active.name;
    } catch (e) {
        console.error('Failed to load branches:', e);
    } finally {
        isLoadingBranches.value = false;
    }
};

const onBranchChange = async () => {
    if (!currentBranch.value) return;
    try {
        await invoke('switch_git_branch', { path: props.repo.path, branch: currentBranch.value });
        alert(t('repo.branch.switch_success'));
    } catch (e) {
        console.error('Failed to switch branch:', e);
        alert(`${t('repo.branch.switch_failed')}: ${e}`);
    } finally {
        await loadBranches();
    }
};

watch(() => props.repo, (newVal) => {
    localRepo.value = { ...newVal };
    if (newVal.path) {
        loadBranches();
    }
}, { immediate: true });

const save = () => {
    emit('update', localRepo.value);
};

const deleteRepo = () => {
    emit('delete', props.repo.id);
}

const browsePath = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Repository Path'
    });
    if (selected && typeof selected === 'string') {
        localRepo.value.path = selected;
    }
};
</script>
