<template>
  <div class="h-full border-r bg-muted/10 overflow-auto">
    <ul class="p-2 space-y-1">
        <li v-for="file in changes" :key="file.path"
            class="flex items-center px-2 py-1.5 rounded cursor-pointer hover:bg-muted text-sm"
            :class="{'bg-muted': selectedFile?.path === file.path}"
            @click="$emit('select', file)"
        >
            <div class="w-2 h-2 rounded-full mr-2" :class="getStatusColor(file.change_type)"></div>
            <span class="truncate">{{ file.path }}</span>
        </li>
    </ul>
    <div v-if="changes.length === 0" class="p-4 text-center text-muted-foreground text-sm">
        {{ $t('diff.no_changes') }}
    </div>
  </div>
</template>

<script setup lang="ts">

defineProps<{
  changes: any[];
  selectedFile: any;
}>();

defineEmits(['select']);

const getStatusColor = (type: string) => {
    switch (type) {
        case 'Added': return 'bg-green-500';
        case 'Modified': return 'bg-yellow-500';
        case 'Deleted': return 'bg-red-500';
        default: return 'bg-gray-400';
    }
};
</script>
