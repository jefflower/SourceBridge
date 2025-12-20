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
          <h3 class="text-lg font-semibold mb-2">Git Status</h3>
          <p class="text-muted-foreground">Branch: <span class="text-foreground font-medium">main</span></p>
          <p class="text-muted-foreground">Last Commit: <span class="font-mono text-xs">a1b2c3d</span> - Initial commit</p>
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
import { ref, watch } from 'vue';
import { Package } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-dialog';

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

watch(() => props.repo, (newVal) => {
    localRepo.value = { ...newVal };
});

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
