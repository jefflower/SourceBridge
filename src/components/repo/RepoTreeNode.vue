<template>
  <li class="pl-2">
    <div
      class="flex items-center py-1 px-2 rounded cursor-pointer hover:bg-muted/50 transition-colors"
      :class="{ 'bg-muted': isSelected }"
      @click.stop="selectNode"
      @contextmenu.prevent.stop="handleContextMenu"
    >
      <!-- Icon -->
      <span class="mr-2 opacity-70">
        <component :is="icon" class="w-4 h-4" />
      </span>

      <!-- Name -->
      <span class="text-sm truncate select-none flex-1 font-medium" :class="{'font-bold': isGroup}">{{ node.name }}</span>

      <!-- Expand Toggle for Group -->
      <span v-if="isGroup && hasChildren" @click.stop="toggleExpand" class="ml-auto p-1 hover:bg-background rounded">
         <ChevronDown v-if="isExpanded" class="w-3 h-3" />
         <ChevronRight v-else class="w-3 h-3" />
      </span>
    </div>

    <!-- Children -->
    <ul v-if="isGroup && isExpanded" class="border-l border-border ml-3">
        <!-- Recursive Groups -->
        <repo-tree-node
            v-for="childGroup in node.children"
            :key="childGroup.id"
            :node="childGroup"
            @select="$emit('select', $event)"
            @context-menu="$emit('context-menu', $event, $event[1])"
        />
        <!-- Repos in this group -->
        <repo-tree-node
            v-for="repo in node.repos"
            :key="repo.id"
            :node="{ ...repo, type: 'repo' }"
            @select="$emit('select', $event)"
            @context-menu="$emit('context-menu', $event, $event[1])"
        />
    </ul>
  </li>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { Folder, Package, ChevronRight, ChevronDown } from 'lucide-vue-next';

const props = defineProps<{
  node: any;
}>();

const emit = defineEmits(['select', 'context-menu']);

const isExpanded = ref(true); // Default open
const isGroup = computed(() => props.node.children !== undefined || props.node.repos !== undefined); // Simple heuristic
const hasChildren = computed(() => isGroup.value && (props.node.children?.length > 0 || props.node.repos?.length > 0));

const icon = computed(() => isGroup.value ? Folder : Package);

const isSelected = false; // To be managed by parent or store state ideally

const toggleExpand = () => {
    isExpanded.value = !isExpanded.value;
};

const selectNode = () => {
    emit('select', props.node);
};

const handleContextMenu = (event: MouseEvent) => {
    emit('context-menu', event, props.node);
};
</script>
