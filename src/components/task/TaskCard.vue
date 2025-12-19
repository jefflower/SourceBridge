<template>
  <div class="border rounded-lg p-4 bg-card text-card-foreground shadow-sm hover:shadow-md transition-shadow cursor-pointer relative" @click="$emit('edit', task)">
    <div class="flex justify-between items-start mb-2">
      <h3 class="font-semibold truncate pr-8">{{ task.name }}</h3>
      <span class="text-xs px-2 py-0.5 rounded-full"
        :class="{
            'bg-green-100 text-green-700': task.last_run_status === 'success',
            'bg-red-100 text-red-700': task.last_run_status === 'failed',
            'bg-gray-100 text-gray-700': !task.last_run_status
        }">
        {{ task.last_run_status ? $t(`task.status.${task.last_run_status}`) : $t('task.status.pending') }}
      </span>
    </div>

    <div class="text-xs text-muted-foreground mb-4">
        <div v-if="task.cron_expression" class="flex items-center gap-1">
            <Clock class="w-3 h-3" /> {{ task.cron_expression }}
        </div>
        <div v-else>Manual Trigger</div>
    </div>

    <div class="flex justify-between items-center">
        <label class="relative inline-flex items-center cursor-pointer" @click.stop>
            <input type="checkbox" :checked="task.enabled" class="sr-only peer" @change="$emit('toggle', task)">
            <div class="w-9 h-5 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
        </label>

        <div class="flex gap-2">
            <button @click.stop="$emit('logs', task)" class="p-2 hover:bg-muted rounded-full text-muted-foreground" title="Logs">
                <List class="w-4 h-4" />
            </button>
            <button @click.stop="$emit('run', task)" class="p-2 hover:bg-muted rounded-full text-primary" :title="$t('task.actions.run')">
                <Play class="w-4 h-4" />
            </button>
        </div>
    </div>

    <button @click.stop="$emit('delete', task)" class="absolute top-2 right-2 p-1 text-muted-foreground hover:text-destructive transition-colors">
        <Trash2 class="w-3 h-3" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import { Clock, Play, Trash2, List } from 'lucide-vue-next';

defineProps<{
  task: any;
}>();
</script>
