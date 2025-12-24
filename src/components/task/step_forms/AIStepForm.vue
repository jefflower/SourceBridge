<template>
  <div class="space-y-4">
    <div class="grid gap-2">
      <label class="text-sm font-medium">{{ $t('task.steps.ai.system_prompt') }}</label>
      <textarea
        v-model="modelValue.system_prompt"
        class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        :placeholder="$t('task.steps.ai.system_prompt_placeholder')"
      ></textarea>
    </div>
    <div class="grid gap-2">
      <label class="text-sm font-medium">{{ $t('task.steps.ai.user_prompt') }}</label>
      <textarea
        v-model="modelValue.user_prompt"
        class="flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        :placeholder="$t('task.steps.ai.user_prompt_placeholder')"
      ></textarea>
    </div>
  </div>
</template>

<script setup lang="ts">


const props = defineProps<{
  modelValue: {
    system_prompt: string;
    user_prompt: string;
  };
}>();

const emit = defineEmits(['update:modelValue']);
</script>
