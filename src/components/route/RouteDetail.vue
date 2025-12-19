<template>
  <div class="h-full flex flex-col bg-background">
    <!-- Header -->
    <div class="border-b p-6 flex flex-col gap-2">
      <div class="flex items-center gap-2 text-muted-foreground text-sm">
        <Waypoints class="w-4 h-4" />
        <span>{{ route.id }}</span>
      </div>
      <h1 class="text-2xl font-bold tracking-tight">{{ route.name }}</h1>
      <p v-if="route.description" class="text-muted-foreground">{{ route.description }}</p>
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

      <!-- Info Tab -->
      <div v-if="currentTab === 'info'" class="max-w-xl grid gap-4">
         <div class="grid gap-2">
            <label class="text-sm font-medium">{{ $t('route.form.name.label') }}</label>
            <input v-model="localRoute.name" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
         </div>
         <div class="grid gap-2">
            <label class="text-sm font-medium">{{ $t('route.form.source.label') }}</label>
            <select v-model="localRoute.source_id" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm">
                <option :value="null">Select Source</option>
                <option v-for="repo in repos" :key="repo.id" :value="repo.id">{{ repo.name }}</option>
            </select>
         </div>
         <div class="grid gap-2">
            <label class="text-sm font-medium">{{ $t('route.form.target.label') }}</label>
            <select v-model="localRoute.target_id" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm">
                <option :value="null">Select Target</option>
                <option v-for="repo in repos" :key="repo.id" :value="repo.id">{{ repo.name }}</option>
            </select>
         </div>
         <div class="flex justify-end gap-2 mt-4">
            <button @click="deleteRoute" class="bg-destructive text-destructive-foreground hover:bg-destructive/90 px-4 py-2 rounded text-sm font-medium">
                {{ $t('repo.context.delete') }}
            </button>
            <button @click="saveInfo" class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium">
                {{ $t('actions.save') }}
            </button>
        </div>
      </div>

      <!-- Mappings Tab -->
      <div v-if="currentTab === 'mappings'" class="flex flex-col h-full">
        <div class="flex justify-between mb-4">
            <h3 class="text-lg font-semibold">{{ $t('route.mapping.add') }}</h3>
            <button @click="addRule" class="bg-secondary text-secondary-foreground hover:bg-secondary/80 px-3 py-1 rounded text-sm">
                + {{ $t('route.mapping.add') }}
            </button>
        </div>

        <div class="border rounded-md">
            <table class="w-full text-sm text-left">
                <thead class="bg-muted text-muted-foreground">
                    <tr>
                        <th class="px-4 py-2 font-medium">{{ $t('route.mapping.source') }}</th>
                        <th class="px-4 py-2 w-8"></th>
                        <th class="px-4 py-2 font-medium">{{ $t('route.mapping.target') }}</th>
                        <th class="px-4 py-2 font-medium">{{ $t('route.mapping.mode') }}</th>
                        <th class="px-4 py-2 font-medium w-20">{{ $t('route.mapping.actions') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(rule, index) in mappings" :key="index" class="border-t">
                        <td class="px-4 py-2">
                            <input v-model="rule.source" class="w-full bg-transparent border-none focus:outline-none" placeholder="src/**/*.ts" />
                        </td>
                        <td class="px-4 py-2 text-center text-muted-foreground">➜</td>
                        <td class="px-4 py-2">
                            <input v-model="rule.target" class="w-full bg-transparent border-none focus:outline-none" placeholder="dest/" />
                        </td>
                        <td class="px-4 py-2">
                            <select v-model="rule.mode" class="bg-transparent border-none focus:outline-none">
                                <option value="copy">{{ $t('route.mapping.modes.copy') }}</option>
                                <option value="ignore">{{ $t('route.mapping.modes.ignore') }}</option>
                            </select>
                        </td>
                        <td class="px-4 py-2 text-center">
                            <button @click="removeRule(index)" class="text-destructive hover:text-destructive/80">
                                <Trash2 class="w-4 h-4" />
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="mt-4 flex justify-end gap-2">
             <button @click="diffModal?.open(route)" class="bg-secondary text-secondary-foreground hover:bg-secondary/90 px-4 py-2 rounded text-sm font-medium flex items-center gap-2">
                <Eye class="w-4 h-4" /> Preview Diff
            </button>
             <button @click="saveMappings" class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium">
                {{ $t('actions.save') }}
            </button>
        </div>

        <!-- Test Panel -->
        <div class="mt-8 border rounded-lg p-4 bg-muted/20">
            <h4 class="font-semibold mb-2">{{ $t('route.mapping.test') }}</h4>
            <div class="flex gap-2">
                <input v-model="testPath" class="flex-1 flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm" :placeholder="$t('route.mapping.test_placeholder')" />
                <button @click="runTest" class="bg-secondary text-secondary-foreground hover:bg-secondary/80 px-3 py-1 rounded text-sm">Check</button>
            </div>
            <div v-if="testResult" class="mt-2 text-sm">
                <span v-if="testResult.matched" class="text-green-600 font-medium">
                    {{ $t('route.test.match', { index: testResult.rule_index + 1, target: testResult.target_path }) }}
                </span>
                <span v-else class="text-muted-foreground">
                    {{ $t('route.test.no_match') }}
                </span>
            </div>
        </div>
      </div>

    </div>
    <DiffViewerModal ref="diffModal" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, defineProps, defineEmits } from 'vue';
import { Waypoints, Trash2, Eye } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import DiffViewerModal from '../diff/DiffViewerModal.vue';

const props = defineProps<{
  route: any;
  repos: any[]; // List of available repos for selection
}>();

const emit = defineEmits(['update', 'delete']);

const tabs = [
  { label: 'Info', value: 'info' },
  { label: 'Mappings', value: 'mappings' },
];

const currentTab = ref('info');
const localRoute = ref({ ...props.route });
const mappings = ref<any[]>([]);
const testPath = ref('');
const testResult = ref<any>(null);
const diffModal = ref<any>(null);

const loadMappings = async () => {
    // Backend returns route with mappings (JSON string) or we fetch separately?
    // Current backend `get_route_details` returns Model. Model has `mappings: Option<String>`.
    try {
        const details: any = await invoke('get_route_details', { id: props.route.id });
        if (details && details.mappings) {
            mappings.value = JSON.parse(details.mappings);
        } else {
            mappings.value = [];
        }
    } catch (e) {
        console.error("Failed to load details", e);
    }
};

watch(() => props.route, (newVal) => {
    localRoute.value = { ...newVal };
    loadMappings();
}, { immediate: true });

const saveInfo = () => {
    emit('update', { type: 'info', data: localRoute.value });
};

const saveMappings = async () => {
    try {
        const json = JSON.stringify(mappings.value);
        await invoke('update_route_mappings', { id: props.route.id, mappings: json });
        // Optionally notify parent
    } catch (e) {
        console.error(e);
    }
};

const deleteRoute = () => {
    if(confirm('Are you sure?')) {
        emit('delete', props.route.id);
    }
};

const addRule = () => {
    mappings.value.push({ source: '', target: '', mode: 'copy' });
};

const removeRule = (index: number) => {
    mappings.value.splice(index, 1);
};

const runTest = async () => {
    try {
        const json = JSON.stringify(mappings.value);
        const res = await invoke('test_route_mapping', { path: testPath.value, mappings: json });
        testResult.value = res;
    } catch (e) {
        console.error(e);
    }
};
</script>
