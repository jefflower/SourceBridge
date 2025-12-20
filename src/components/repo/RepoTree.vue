<template>
  <div class="repo-tree overflow-y-auto h-full p-2">
    <ul>
      <repo-tree-node
        v-for="node in treeData"
        :key="node.id"
        :node="node"
        @select="onSelect"
        @context-menu="onContextMenu"
      />
    </ul>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import RepoTreeNode from './RepoTreeNode.vue';

// Define recursive structure for TypeScript if needed, or use any
// In a real app, define interfaces: RepoNode, GroupNode

const props = defineProps<{
  treeData: any[];
}>();

const emit = defineEmits(['select', 'context-menu']);

const onSelect = (node: any) => {
  emit('select', node);
};

const onContextMenu = (event: MouseEvent, node: any) => {
  emit('context-menu', event, node);
};
</script>
