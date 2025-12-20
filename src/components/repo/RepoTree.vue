<template>
  <div 
    class="repo-tree overflow-y-auto h-full p-2 border-2 border-transparent"
    :class="{ 'border-primary/20 bg-primary/5': isDraggingAny }"
    @dragover.prevent="onDragOver"
    @drop.prevent="onDrop"
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
  >
    <ul class="min-h-[100px]">
      <DraggableTreeNode
        v-for="node in treeData"
        :key="node.id"
        :node="node"
        tree-type="repo"
        @select="onSelect"
        @context-menu="onContextMenu"
        @move="onMove"
      />
    </ul>
    
    <!-- Empty State / Drop at root level -->
    <div v-if="treeData.length === 0" class="h-full flex items-center justify-center text-muted-foreground text-sm py-8">
      拖拽仓库或分组到此处
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import DraggableTreeNode from '@/components/common/DraggableTreeNode.vue';

defineProps<{
  treeData: any[];
}>();

const emit = defineEmits(['select', 'context-menu', 'move']);

const isDraggingAny = ref(false);
const dragCounter = ref(0);

const onSelect = (node: any) => {
  emit('select', node);
};

const onContextMenu = (event: MouseEvent, node: any) => {
  emit('context-menu', event, node);
};

const onMove = (data: any) => {
  emit('move', data);
};

// Container-level drag handlers for debugging and root-level drop
const onDragEnter = (e: DragEvent) => {
  e.preventDefault();
  dragCounter.value++;
  if (dragCounter.value === 1) {
    isDraggingAny.value = true;
    console.log('[RepoTree] Global drag enter');
  }
};

const onDragOver = (e: DragEvent) => {
  e.preventDefault();
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move';
  }
};

const onDragLeave = (e: DragEvent) => {
  e.preventDefault();
  dragCounter.value--;
  if (dragCounter.value <= 0) {
    dragCounter.value = 0;
    isDraggingAny.value = false;
    console.log('[RepoTree] Global drag leave');
  }
};

const onDrop = (e: DragEvent) => {
  e.preventDefault();
  isDraggingAny.value = false;
  dragCounter.value = 0;
  
  try {
    const rawData = e.dataTransfer?.getData('text/plain');
    if (!rawData) return;
    
    const data = JSON.parse(rawData);
    console.log('[RepoTree] Global drop:', data);
    
    // If dropped on the container (and not captured by a sub-node), 
    // it means move to root level.
    emit('move', {
      draggedId: data.id,
      draggedType: data.type,
      targetGroupId: null // Root level
    });
  } catch (err) {
    console.error('[RepoTree] Drop error:', err);
  }
};
</script>

