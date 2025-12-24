<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-end justify-center bg-black/20 backdrop-blur-sm" @click="close">
    <div class="w-full max-w-4xl h-96 bg-background border-t border-x rounded-t-lg shadow-lg flex flex-col overflow-hidden" @click.stop>
      <div class="flex items-center justify-between p-4 border-b bg-muted/30">
        <h3 class="font-semibold text-sm flex items-center gap-2">
            <Terminal class="w-4 h-4" />
            Command Output
        </h3>
        <button @click="close" class="p-1 hover:bg-muted rounded">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div ref="logContainer" class="flex-1 overflow-y-auto p-4 font-mono text-xs whitespace-pre-wrap bg-black text-green-400">
        {{ logs }}
      </div>

      <div class="p-2 border-t bg-muted/30 text-xs text-muted-foreground flex justify-between">
          <span>{{ status }}</span>
          <span v-if="exitCode !== null">Exit Code: {{ exitCode }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { Terminal, X } from 'lucide-vue-next';
import { listen } from '@tauri-apps/api/event';

const isOpen = ref(false);
const logs = ref('');
const status = ref('Ready');
const exitCode = ref<number | null>(null);
const logContainer = ref<HTMLElement | null>(null);

let unlistenLog: any = null;
let unlistenEnd: any = null;

const open = () => {
    isOpen.value = true;
    logs.value = '';
    status.value = 'Running...';
    exitCode.value = null;
};

const close = () => {
    isOpen.value = false;
};

// Expose open method
defineExpose({ open });

onMounted(async () => {
    unlistenLog = await listen('cmd://log', (event: any) => {
        if (!isOpen.value) open(); // Auto open on log? Maybe optional.
        logs.value += event.payload.text;
        scrollToBottom();
    });

    unlistenEnd = await listen('cmd://end', (event: any) => {
        status.value = event.payload.success ? 'Success' : 'Failed';
        exitCode.value = event.payload.code;
    });
});

onUnmounted(() => {
    if (unlistenLog) unlistenLog();
    if (unlistenEnd) unlistenEnd();
});

const scrollToBottom = async () => {
    await nextTick();
    if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight;
    }
};
</script>
