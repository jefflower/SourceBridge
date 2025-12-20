<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
    <div class="bg-background border rounded-lg shadow-lg w-full max-w-md p-6 grid gap-4">
      <div class="flex flex-col space-y-1.5 text-center sm:text-left">
        <h2 class="text-lg font-semibold leading-none tracking-tight">
            {{ mode === 'group' ? $t('repo.group.new') : $t('repo.add') }}
        </h2>
      </div>

      <div class="grid gap-4 py-4">
        <!-- Name -->
        <div class="grid grid-cols-4 items-center gap-4">
          <label class="text-right text-sm font-medium">{{ $t('repo.form.name.label') }}</label>
          <input v-model="form.name" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" :placeholder="$t('repo.form.name.placeholder')" />
        </div>

        <!-- Path (Repo Only) -->
        <div v-if="mode === 'repo'" class="grid grid-cols-4 items-center gap-4">
          <label class="text-right text-sm font-medium">{{ $t('repo.form.path.label') }}</label>
          <input v-model="form.path" class="col-span-3 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" :placeholder="$t('repo.form.path.placeholder')" />
        </div>
      </div>

      <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
         <button @click="close" class="mt-2 sm:mt-0 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
            Cancel
         </button>
         <button @click="submit" class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
            Create
         </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineExpose, defineEmits } from 'vue';

const isOpen = ref(false);
const mode = ref<'repo' | 'group'>('repo');
const parentId = ref<string | null>(null);

const form = ref({
    name: '',
    path: '',
});

const emit = defineEmits(['create']);

const open = (type: 'repo' | 'group', parent: string | null = null) => {
    mode.value = type;
    parentId.value = parent;
    form.value = { name: '', path: '' };
    isOpen.value = true;
};

const close = () => {
    isOpen.value = false;
};

const submit = () => {
    emit('create', {
        type: mode.value,
        parentId: parentId.value,
        data: { ...form.value }
    });
    close();
};

defineExpose({ open, close });
</script>
