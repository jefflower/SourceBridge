<template>
    <div class="space-y-4">
        <div>
            <label class="block text-sm font-medium mb-1">{{ $t('task.steps.git.repo_id') }}</label>
            <RepoSelector
                v-model="model.repo_id"
                :repos="repos"
                :placeholder="$t('task.steps.git.select_repo', '选择仓库...')"
            />
        </div>
        <div>
            <label class="block text-sm font-medium mb-1">{{ $t('task.steps.git.operation') }}</label>
            <select
                v-model="model.operation"
                class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            >
                <option value="pull">{{ $t('task.steps.git.ops.pull') }}</option>
                <option value="push">{{ $t('task.steps.git.ops.push') }}</option>
                <option value="fetch">{{ $t('task.steps.git.ops.fetch') }}</option>
                <option value="reset">{{ $t('task.steps.git.ops.reset') }}</option>
                <option value="reset_remote">{{ $t('task.steps.git.ops.reset_remote') }}</option>
            </select>
        </div>
        <!-- Force push option (only shown when push is selected) -->
        <div v-if="model.operation === 'push'" class="flex items-center gap-2">
            <input 
                type="checkbox" 
                id="force_push"
                v-model="model.force_push"
                class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
            />
            <label for="force_push" class="text-sm text-muted-foreground">
                {{ $t('task.steps.git.force_push') }}
            </label>
        </div>
    </div>
</template>

<script setup lang="ts">
import RepoSelector from '@/components/repo/RepoSelector.vue';

defineProps<{
    repos: any[];
}>();

const model = defineModel<{
    repo_id: string | null;
    operation: string;
    force_push?: boolean;
}>({ required: true });
</script>
