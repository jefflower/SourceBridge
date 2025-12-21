<template>
    <div class="space-y-4">
        <div>
            <label class="block text-sm font-medium mb-1">{{ $t('task.steps.git.repo_id') }}</label>
            <input
                type="number"
                v-model="model.repo_id"
                class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                :placeholder="$t('task.steps.git.repo_id')"
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
            </select>
        </div>
    </div>
</template>

<script setup lang="ts">
const model = defineModel<{
    repo_id: number | null;
    operation: string;
}>({ required: true });
</script>
