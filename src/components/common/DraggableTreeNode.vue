<template>
  <li class="pl-2">
    <div
      class="flex items-center py-1 px-2 rounded cursor-pointer hover:bg-muted/50 transition-colors select-none"
      :class="{ 'bg-muted': isSelected, 'bg-primary/20 border-2 border-primary border-dashed': isDragOver }"
      @click.stop="selectNode"
      @contextmenu.prevent.stop="handleContextMenu"
      draggable="true"
      @dragstart="onDragStart"
      @dragend="onDragEnd"
      @dragenter.prevent="onDragEnter"
      @dragover.prevent="onDragOver"
      @dragleave.prevent="onDragLeave"
      @drop.prevent="onDrop"
    >
      <!-- Drag Handle -->
      <span class="mr-1 cursor-grab opacity-50 hover:opacity-100 pointer-events-none">
        <GripVertical class="w-3 h-3" />
      </span>
      
      <!-- Icon -->
      <span class="mr-2 opacity-70 pointer-events-none">
        <component :is="icon" class="w-4 h-4" />
      </span>

      <!-- Name -->
      <span class="text-sm truncate flex-1 font-medium pointer-events-none" :class="{'font-bold': isGroup}">{{ node.name }}</span>

      <!-- Expand Toggle for Group - Needs pointer-events-auto -->
      <span v-if="isGroup && hasChildren" @click.stop="toggleExpand" class="ml-auto p-1 hover:bg-background rounded cursor-pointer pointer-events-auto">
         <ChevronDown v-if="isExpanded" class="w-3 h-3 pointer-events-none" />
         <ChevronRight v-else class="w-3 h-3 pointer-events-none" />
      </span>
    </div>

    <!-- Children -->
    <ul v-if="isGroup && isExpanded" class="border-l border-border ml-3">
        <!-- Recursive Groups -->
        <draggable-tree-node
            v-for="childGroup in node.children"
            :key="childGroup.id"
            :node="childGroup"
            :tree-type="treeType"
            @select="$emit('select', $event)"
            @context-menu="onChildContextMenu"
            @move="$emit('move', $event)"
        />
        <!-- Items in this group (repos or routes) -->
        <draggable-tree-node
            v-for="item in nodeItems"
            :key="item.id"
            :node="{ ...item, type: itemType }"
            :tree-type="treeType"
            @select="$emit('select', $event)"
            @context-menu="onChildContextMenu"
            @move="$emit('move', $event)"
        />
    </ul>
  </li>
</template>


<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { Folder, Package, Waypoints, ChevronRight, ChevronDown, GripVertical } from 'lucide-vue-next';

defineOptions({
  name: 'DraggableTreeNode'
});

const props = defineProps<{
  node: any;
  treeType: 'repo' | 'route';
}>();

const emit = defineEmits(['select', 'context-menu', 'move']);

const isExpanded = ref(true);

// Watch for external expansion control (e.g. from search filtering)
watch(() => props.node.isExpanded, (newVal) => {
    if (newVal !== undefined) {
        isExpanded.value = newVal;
    }
}, { immediate: true });
const isDragOver = ref(false);
const dragCounter = ref(0);

const isGroup = computed(() => props.node.children !== undefined || props.node.repos !== undefined || props.node.routes !== undefined);
const nodeItems = computed(() => props.treeType === 'repo' ? props.node.repos : props.node.routes);
const itemType = computed(() => props.treeType === 'repo' ? 'repo' : 'route');
const hasChildren = computed(() => isGroup.value && ((props.node.children?.length > 0) || (nodeItems.value?.length > 0)));

const icon = computed(() => {
  if (isGroup.value) return Folder;
  return props.treeType === 'repo' ? Package : Waypoints;
});

const isSelected = false;

const toggleExpand = () => {
    isExpanded.value = !isExpanded.value;
};

const selectNode = () => {
    emit('select', props.node);
};

const handleContextMenu = (event: MouseEvent) => {
    emit('context-menu', event, props.node);
};

// Drag and Drop with robust event handling
const onDragStart = (e: DragEvent) => {
  console.log('[DragStart]', props.node.name, 'isGroup:', isGroup.value);
  const dragData = JSON.stringify({
    id: props.node.id,
    type: isGroup.value ? 'group' : props.treeType,
    name: props.node.name
  });
  if (e.dataTransfer) {
    e.dataTransfer.setData('text/plain', dragData);
    e.dataTransfer.effectAllowed = 'all'; // Maximize compatibility
  }
};

const onDragEnd = () => {
  console.log('[DragEnd]', props.node.name);
  isDragOver.value = false;
  dragCounter.value = 0;
};

const onDragEnter = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  
  if (!isGroup.value) return;
  
  // Important: Set dropEffect in dragenter too for some browsers
  if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'move';
  }
  
  dragCounter.value++;
  if (dragCounter.value === 1) {
    isDragOver.value = true;
    console.log('[DragEnter] Group:', props.node.name, 'counter:', dragCounter.value);
  }
};

const onDragOver = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  
  if (!isGroup.value) return;
  
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move';
  }
  
  // Ensure isDragOver is true if counter is > 0
  if (dragCounter.value > 0 && !isDragOver.value) {
    isDragOver.value = true;
  }
};

const onDragLeave = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  
  if (!isGroup.value) return;
  
  dragCounter.value--;
  if (dragCounter.value <= 0) {
    dragCounter.value = 0;
    isDragOver.value = false;
    console.log('[DragLeave] Group:', props.node.name, 'counter:', dragCounter.value);
  }
};

const onChildContextMenu = (event: MouseEvent, node: any) => {
    emit('context-menu', event, node);
};

const onDrop = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  isDragOver.value = false;
  
  console.log('[Drop] Target:', props.node.name, 'isGroup:', isGroup.value);
  
  if (!isGroup.value) {
    console.log('[Drop] Not a group, ignoring');
    return;
  }
  
  try {
    const rawData = e.dataTransfer?.getData('text/plain');
    console.log('[Drop] Raw data:', rawData);
    
    if (!rawData) {
      console.log('[Drop] No data found');
      return;
    }
    
    const data = JSON.parse(rawData);
    console.log('[Drop] Parsed data:', data);
    
    if (data.id === props.node.id) {
      console.log('[Drop] Cannot drop on self');
      return;
    }
    
    const movePayload = {
      draggedId: data.id,
      draggedType: data.type,
      targetGroupId: props.node.id
    };
    console.log('[Drop] Emitting move:', movePayload);
    emit('move', movePayload);
  } catch (err) {
    console.error('[Drop] Error:', err);
  }
};
</script>

