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
            <p>No tasks found. Create one to get started.</p>
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
import TaskCard from '@/components/task/TaskCard.vue';
import TaskBuilder from '@/components/task/TaskBuilder.vue';
import LogConsole from '@/components/log/LogConsole.vue';

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

const openBuilder = (task: any = null) => {
    selectedTask.value = task;
    isBuilderOpen.value = true;
};

const closeBuilder = () => {
    isBuilderOpen.value = false;
    selectedTask.value = null;
};

const saveTask = async (task: any) => {
    try {
        await invoke('create_task', { task });
        closeBuilder();
        await loadTasks();
    } catch (e) {
        console.error(e);
        alert('Error: ' + e);
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
    if (confirm('Are you sure?')) {
        try {
            await invoke('delete_task', { id: task.id });
            await loadTasks();
        } catch (e) {
            console.error(e);
        }
    }
};
</script>
