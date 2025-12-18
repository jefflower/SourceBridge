<template>
  <div data-tauri-drag-region class="h-8 bg-background border-b flex items-center justify-between select-none fixed top-0 left-0 right-0 z-50">
    <div class="flex items-center px-4 pointer-events-none">
      <span class="font-bold text-sm">SourceBridge</span>
    </div>
    <div class="flex h-full">
      <button @click="minimize" class="px-4 hover:bg-muted transition-colors h-full flex items-center justify-center" :title="$t('window.minimize')">
        <Minus class="w-4 h-4" />
      </button>
      <button @click="maximize" class="px-4 hover:bg-muted transition-colors h-full flex items-center justify-center" :title="$t('window.maximize')">
        <Square class="w-3 h-3" />
      </button>
      <button @click="close" class="px-4 hover:bg-red-500 hover:text-white transition-colors h-full flex items-center justify-center" :title="$t('window.close')">
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Minus, Square, X } from 'lucide-vue-next';

const appWindow = getCurrentWindow();

const minimize = () => appWindow.minimize();
const maximize = () => appWindow.toggleMaximize();
const close = () => appWindow.close();
</script>
