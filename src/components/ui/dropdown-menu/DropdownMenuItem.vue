<template>
  <div
    class="block px-4 py-2 text-sm text-popover-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer"
    @click="handleClick"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';

const { close } = inject('dropdown') as any;

const handleClick = (e: Event) => {
    close();
}
</script>
