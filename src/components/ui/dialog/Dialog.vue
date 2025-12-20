<template>
  <div v-if="open" class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-4">
    <div class="fixed inset-0" @click="$emit('update:open', false)"></div>
    <div class="z-50 w-full max-w-lg gap-4 border bg-background p-6 shadow-lg duration-200 sm:rounded-lg">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  open: boolean;
}>();
defineEmits(['update:open']);
</script>
