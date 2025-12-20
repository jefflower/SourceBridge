<template>
    <div class="space-y-4">
        <div>
            <label class="block text-sm font-medium mb-1">Script</label>
            <textarea
                v-model="model.script"
                class="w-full min-h-[100px] rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
                placeholder="# Write your script here..."
            ></textarea>
        </div>
        <div class="flex items-center gap-2">
            <input
                type="checkbox"
                v-model="model.continue_on_error"
                id="continue_on_error"
                class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
            />
            <label for="continue_on_error" class="text-sm font-medium">Continue on Error</label>
        </div>
    </div>
</template>

<script setup lang="ts">
const model = defineModel<{
    script: string;
    continue_on_error: boolean;
}>({ required: true });
</script>
