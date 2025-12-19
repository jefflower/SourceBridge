<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
    <div class="bg-background border rounded-lg shadow-lg w-full max-w-4xl h-5/6 flex flex-col p-6">
        <div class="flex justify-between items-center mb-4">
            <h2 class="text-lg font-semibold">{{ $t('log.title') }} - {{ taskName }}</h2>
            <button @click="close" class="px-3 py-1 text-sm border rounded hover:bg-muted">Close</button>
        </div>

        <div class="flex-1 bg-black text-white p-4 rounded-md overflow-auto font-mono text-sm whitespace-pre-wrap">
            <div v-if="logs.length === 0" class="text-gray-500">No logs available.</div>
            <div v-for="(log, index) in logs" :key="index" class="border-b border-gray-800 pb-2 mb-2">
                <div class="flex gap-4 text-xs text-gray-400 mb-1">
                    <span>{{ $t('log.start') }}: {{ log.start_time }}</span>
                    <span>{{ $t('log.end') }}: {{ log.end_time }}</span>
                    <span :class="getStatusColor(log.status)">{{ log.status }}</span>
                </div>
                <div>{{ log.output_log }}</div>
            </div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineExpose } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const isOpen = ref(false);
const taskName = ref('');
const logs = ref<any[]>([]);

const open = async (task: any) => {
    isOpen.value = true;
    taskName.value = task.name;
    logs.value = [];
    try {
        // We need a command to fetch logs for a task.
        // It wasn't implemented in Task 004 commands explicitly, but Task 004 description mentioned `get_task_logs(id)`.
        // Let's check `src-tauri/src/commands/task.rs` content again.
        // It has create, run, list, delete. No get_logs.
        // I will implement it in next step or now.
        // For now, I'll assume it exists or I'll add it in the next backend fix step.
        // Since I can't change backend in this frontend step easily without context switching in plan,
        // I will mock or try to call `get_task_logs`.
        // Wait, I am in "Frontend Implementation" step. I should have added `get_task_logs` in step 1 commands if it was missing.
        // I missed it. I will add it in a subsequent fix or assume I can add it now.
        // Actually, I can modify `task.rs` in this turn if I want, but I am in "Frontend Implementation" plan step.
        // I will implement the UI to call it, and then add the command in a "Fix" step if needed or if I can squeeze it in backend step (which is done).
        // I will add a `Fix Backend` step after this.

        logs.value = await invoke('get_task_logs', { taskId: task.id });
    } catch (e) {
        console.error("Failed to fetch logs", e);
        logs.value = [{ output_log: "Failed to fetch logs: " + e }];
    }
};

const close = () => {
    isOpen.value = false;
};

const getStatusColor = (status: string) => {
    switch (status) {
        case 'success': return 'text-green-400';
        case 'failed': return 'text-red-400';
        default: return 'text-yellow-400';
    }
};

defineExpose({ open, close });
</script>
