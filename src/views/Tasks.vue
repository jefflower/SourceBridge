<template>
  <div class="flex h-full flex-col relative">
    <!-- List Header -->
    <div class="flex items-center justify-between mb-6" v-if="!isBuilderOpen">
        <h1 class="text-2xl font-bold">{{ $t('nav.tasks') }}</h1>
        <button @click="openBuilder()" class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
            + {{ $t('task.new') }}
        </button>
    </div>

    <!-- Task List -->
    <div v-if="!isBuilderOpen" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-auto pb-20">
        <TaskCard
            v-for="task in tasks"
            :key="task.id"
            :task="task"
            @edit="openBuilder(task)"
            @toggle="toggleTask"
            @run="runTask"
            @delete="deleteTask"
            @logs="logConsole?.open(task)"
        />
        <div v-if="tasks.length === 0" class="col-span-full flex flex-col items-center justify-center text-muted-foreground min-h-[200px]">
            <p>{{ $t('task.empty') }}</p>
        </div>
    </div>

    <!-- Task Builder Overlay -->
    <div v-if="isBuilderOpen" class="absolute inset-0 bg-background z-10 border rounded-lg overflow-hidden">
        <TaskBuilder
            :task="selectedTask"
            @close="closeBuilder"
            @save="saveTask"
        />
    </div>

    <LogConsole ref="logConsole" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ask } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';
import TaskCard from '@/components/task/TaskCard.vue';
import TaskBuilder from '@/components/task/TaskBuilder.vue';
import LogConsole from '@/components/log/LogConsole.vue';

const { t } = useI18n();

const tasks = ref<any[]>([]);
const isBuilderOpen = ref(false);
const selectedTask = ref<any>(null);
const logConsole = ref<any>(null);

const loadTasks = async () => {
    try {
        tasks.value = await invoke('list_tasks');
    } catch (e) {
        console.error('Failed to load tasks', e);
    }
};

onMounted(() => {
    loadTasks();
});

const openBuilder = async (task: any = null) => {
    if (task && task.id) {
        // Editing existing task - fetch full data with steps
        try {
            const fullTask = await invoke('get_task_with_steps', { id: task.id });
            selectedTask.value = fullTask;
        } catch (e) {
            console.error('Failed to load task details:', e);
            selectedTask.value = task;
        }
    } else {
        // Creating new task
        selectedTask.value = null;
    }
    isBuilderOpen.value = true;
};

const closeBuilder = () => {
    isBuilderOpen.value = false;
    selectedTask.value = null;
};

const saveTask = async (task: any) => {
    console.log('[saveTask] Saving task:', JSON.stringify(task, null, 2));
    try {
        if (task.id) {
            console.log('[saveTask] Updating existing task with id:', task.id);
            // Update existing task
            await invoke('update_task', { task });
        } else {
            console.log('[saveTask] Creating new task');
            // Create new task
            await invoke('create_task', { task });
        }
        closeBuilder();
        await loadTasks();
    } catch (e) {
        console.error('[saveTask] Error:', e);
        alert(t('common.error') + ': ' + e);
    }
};

const toggleTask = async (task: any) => {
    console.log("Toggle not fully implemented in backend yet", task);
};

const runTask = async (task: any) => {
    try {
        await invoke('run_task_now', { id: task.id });
        setTimeout(loadTasks, 1000);
    } catch (e) {
        console.error(e);
    }
};

const deleteTask = async (task: any) => {
    const confirmed = await ask(t('task.confirm_delete', { name: task.name }), {
        title: t('task.delete_title'),
        kind: 'warning'
    });
    if (confirmed) {
        try {
            await invoke('delete_task', { id: task.id });
            await loadTasks();
        } catch (e) {
            console.error(e);
        }
    }
};
</script>
