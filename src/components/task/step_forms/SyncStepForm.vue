<template>
    <div class="space-y-4">
        <div>
            <label class="block text-sm font-medium mb-1">Route ID</label>
            <input
                type="number"
                v-model="model.route_id"
                class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                placeholder="Route ID"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
const model = defineModel<{
    route_id: number | null;
}>({ required: true });
</script>
