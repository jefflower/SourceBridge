<template>
  <div @click="toggle" class="cursor-pointer">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';

const { toggle } = inject('dropdown') as any;
</script>
