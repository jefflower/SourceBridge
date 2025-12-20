<template>
  <Dialog :open="isOpen" @update:open="setOpen">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>
            {{ mode === 'group' ? $t('route.group.new') : $t('route.add') }}
        </DialogTitle>
      </DialogHeader>

      <div class="grid gap-4 py-4">
        <!-- Name -->
        <div class="grid grid-cols-4 items-center gap-4">
          <Label class="text-right">{{ $t('route.form.name.label') }}</Label>
          <Input 
            v-model="form.name" 
            class="col-span-3" 
            :placeholder="$t('route.form.name.placeholder')" 
          />
        </div>

        <!-- Source/Target (Route Only) -->
        <template v-if="mode === 'route'">
          <div class="grid grid-cols-4 items-center gap-4">
            <Label class="text-right">{{ $t('route.form.source.label') }}</Label>
            <div class="col-span-3">
               <RepoSelector v-model="form.source_id" :repos="repos" :placeholder="$t('route.form.source.label')" />
            </div>
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <Label class="text-right">{{ $t('route.form.target.label') }}</Label>
            <div class="col-span-3">
               <RepoSelector v-model="form.target_id" :repos="repos" :placeholder="$t('route.form.target.label')" />
            </div>
          </div>
        </template>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="close">
          {{ $t('common.cancel') }}
        </Button>
        <Button @click="submit" :disabled="!form.name">
          {{ $t('actions.save') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import RepoSelector from '../repo/RepoSelector.vue';
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

defineProps<{
    repos: any[];
}>();

const isOpen = ref(false);
const mode = ref<'route' | 'group'>('route');
const parentId = ref<string | null>(null);

const form = ref({
    name: '',
    source_id: null,
    target_id: null,
});

const emit = defineEmits(['create']);

const open = (type: 'route' | 'group', parent: string | null = null) => {
    mode.value = type;
    parentId.value = parent;
    form.value = { name: '', source_id: null, target_id: null };
    isOpen.value = true;
};

const close = () => {
    isOpen.value = false;
};

const setOpen = (val: boolean) => {
    isOpen.value = val;
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
