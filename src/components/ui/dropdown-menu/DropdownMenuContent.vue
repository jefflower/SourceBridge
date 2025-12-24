<template>
  <div v-if="isOpen" class="absolute right-0 mt-2 w-56 origin-top-right rounded-md bg-popover shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-50 border">
    <div class="py-1">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';

const { isOpen } = inject('dropdown') as any;
</script>
