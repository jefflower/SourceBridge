<template>
  <div class="relative inline-block text-left">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { provide, ref } from 'vue';

const isOpen = ref(false);
const toggle = () => isOpen.value = !isOpen.value;
const close = () => isOpen.value = false;

provide('dropdown', {
  isOpen,
  toggle,
  close
});
</script>
