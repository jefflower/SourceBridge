<template>
  <div class="h-full flex flex-col bg-background relative">
    <!-- Header -->
    <div class="border-b p-4 flex items-center justify-between">
      <input v-model="localTask.name" class="font-bold text-lg bg-transparent border-none focus:outline-none" :placeholder="$t('task.form.name.placeholder')" />
      <div class="flex gap-2">
        <button @click="$emit('close')" class="px-3 py-1 text-sm border rounded hover:bg-muted">{{ $t('task.actions.close') }}</button>
        <button @click="save" class="px-3 py-1 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90">{{ $t('actions.save') }}</button>
      </div>
    </div>

    <div class="flex-1 overflow-auto p-6 grid gap-6 grid-cols-1 lg:grid-cols-3">
        <!-- Left: Settings -->
        <div class="space-y-6">
            <div class="border rounded-lg p-4 bg-card">
                <h3 class="font-semibold mb-4">{{ $t('settings.title') }}</h3>

                <div class="flex items-center justify-between mb-4">
                    <label class="text-sm">{{ $t('task.form.schedule.enable') }}</label>
                    <input type="checkbox" v-model="localTask.enabled" />
                </div>

                <div v-if="localTask.enabled" class="space-y-2">
                    <label class="text-sm font-medium">{{ $t('task.form.schedule.cron') }}</label>
                    <input v-model="localTask.cron" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm" :placeholder="$t('task.form.schedule.placeholder')" />
                </div>
            </div>
        </div>

        <!-- Right: Pipeline -->
        <div class="lg:col-span-2 border rounded-lg p-4 bg-muted/10 flex flex-col">
            <h3 class="font-semibold mb-4">{{ $t('task.steps_title') }}</h3>

            <div class="flex-1 space-y-4 overflow-y-auto min-h-[300px]">
                <div v-for="(step, index) in steps" :key="index" class="border rounded bg-card p-4 relative group">
                    <div class="flex items-center gap-2 mb-2">
                        <span class="bg-muted text-xs font-bold px-2 py-1 rounded">#{{ index + 1 }}</span>
                        <span class="font-medium text-sm">{{ getStepTitle(step.action_type) }}</span>
                        <button @click="removeStep(index)" class="ml-auto opacity-0 group-hover:opacity-100 text-destructive">
                            <Trash2 class="w-4 h-4" />
                        </button>
                    </div>

                    <!-- Dynamic Step Form -->
                    <component :is="getStepComponent(step.action_type)" v-model="step.params" />
                </div>
            </div>

            <!-- Add Step -->
            <div class="mt-4 border-2 border-dashed border-muted-foreground/20 rounded-lg p-4 flex justify-center gap-4">
                <button @click="addStep('script')" class="flex items-center gap-2 px-3 py-2 bg-background border rounded hover:bg-muted text-sm">
                    <Terminal class="w-4 h-4" /> {{ $t('task.step_types.script') }}
                </button>
                <button @click="addStep('git')" class="flex items-center gap-2 px-3 py-2 bg-background border rounded hover:bg-muted text-sm">
                    <GitBranch class="w-4 h-4" /> {{ $t('task.step_types.git') }}
                </button>
                <button @click="addStep('sync')" class="flex items-center gap-2 px-3 py-2 bg-background border rounded hover:bg-muted text-sm">
                    <Waypoints class="w-4 h-4" /> {{ $t('task.step_types.sync') }}
                </button>
            </div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { Trash2, Terminal, GitBranch, Waypoints } from 'lucide-vue-next';
import ScriptStepForm from './step_forms/ScriptStepForm.vue';
import GitStepForm from './step_forms/GitStepForm.vue';
import SyncStepForm from './step_forms/SyncStepForm.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const props = defineProps<{
  task: any;
}>();

const emit = defineEmits(['save', 'close']);

const localTask = ref<any>({
    id: '',
    name: '',
    cron: '',
    enabled: false,
    ...props.task
});

const steps = ref<any[]>(props.task?.steps ? [...props.task.steps] : []);

// Parse params from JSON string if needed, mostly backend sends struct, frontend needs check
// The DTO says params is String (JSON). We need to parse it for the form, and stringify on save.
// Let's assume for Builder state we keep it as object, and serialize later.
// But incoming prop might be the backend model where params is string.

watch(() => props.task, (newVal) => {
    if (newVal) {
        localTask.value = { ...newVal, cron: newVal.cron_expression };
        steps.value = (newVal.steps || []).map((s: any) => ({
            ...s,
            params: typeof s.params === 'string' ? JSON.parse(s.params) : s.params
        }));
    } else {
        localTask.value = { name: '', enabled: false, cron: '' };
        steps.value = [];
    }
}, { immediate: true });

const getStepTitle = (type: string) => {
    return t(`task.step_types.${type}`);
};

const getStepComponent = (type: string) => {
    switch (type) {
        case 'script': return ScriptStepForm;
        case 'git': return GitStepForm;
        case 'sync': return SyncStepForm;
        default: return null;
    }
};

const addStep = (type: string) => {
    let defaultParams = {};
    if (type === 'script') defaultParams = { script: '', continue_on_error: false };
    if (type === 'git') defaultParams = { repo_id: null, operation: 'pull' };
    if (type === 'sync') defaultParams = { route_id: null };

    steps.value.push({
        action_type: type,
        params: defaultParams
    });
};

const removeStep = (index: number) => {
    steps.value.splice(index, 1);
};

const save = () => {
    // Serialize params
    const stepsToSend = steps.value.map(s => ({
        ...s,
        params: JSON.stringify(s.params)
    }));

    emit('save', {
        ...localTask.value,
        steps: stepsToSend
    });
};
</script>
