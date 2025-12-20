<template>
  <Teleport to="body">
    <div
      v-if="visible"
      ref="menuRef"
      class="fixed z-[100] min-w-[160px] py-1 bg-popover border rounded-md shadow-lg"
      :style="{ left: position.x + 'px', top: position.y + 'px' }"
      @click.stop
    >
      <button
        v-for="item in items"
        :key="item.key"
        class="w-full px-3 py-1.5 text-sm text-left hover:bg-muted flex items-center gap-2 transition-colors"
        :class="{ 'text-destructive hover:bg-destructive/10': item.danger }"
        @click="handleClick(item)"
      >
        <component :is="item.icon" v-if="item.icon" class="w-4 h-4" />
        {{ item.label }}
      </button>
    </div>
    <div
      v-if="visible"
      class="fixed inset-0 z-[99]"
      @click="close"
      @contextmenu.prevent="close"
    />
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

export interface MenuItem {
  key: string;
  label: string;
  icon?: any;
  danger?: boolean;
}

const props = defineProps<{
  items: MenuItem[];
}>();

const emit = defineEmits(['select']);

const visible = ref(false);
const position = ref({ x: 0, y: 0 });
const menuRef = ref<HTMLElement | null>(null);

const open = (event: MouseEvent) => {
  position.value = {
    x: event.clientX,
    y: event.clientY
  };
  visible.value = true;

  // Adjust position if menu goes off screen
  setTimeout(() => {
    if (menuRef.value) {
      const rect = menuRef.value.getBoundingClientRect();
      if (rect.right > window.innerWidth) {
        position.value.x = window.innerWidth - rect.width - 10;
      }
      if (rect.bottom > window.innerHeight) {
        position.value.y = window.innerHeight - rect.height - 10;
      }
    }
  }, 0);
};

const close = () => {
  visible.value = false;
};

const handleClick = (item: MenuItem) => {
  emit('select', item.key);
  close();
};

// Close on escape key
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && visible.value) {
    close();
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
});

defineExpose({ open, close });
</script>
