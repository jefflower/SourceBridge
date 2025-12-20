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
            <label class="block text-sm font-medium mb-1">Operation</label>
            <select
                v-model="model.operation"
                class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            >
                <option value="pull">Pull</option>
                <option value="push">Push</option>
                <option value="fetch">Fetch</option>
                <option value="reset">Reset</option>
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
