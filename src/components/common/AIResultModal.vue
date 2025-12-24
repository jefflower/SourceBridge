<template>
  <Dialog :open="isOpen" @update:open="$emit('close')">
    <DialogContent class="sm:max-w-[700px] h-[80vh] flex flex-col p-0">
      <DialogHeader class="p-6 pb-2">
        <DialogTitle>{{ title }}</DialogTitle>
        <DialogDescription>
          AI Generated Response
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 overflow-auto p-6 pt-0">
        <div class="bg-muted rounded-md p-4 whitespace-pre-wrap font-mono text-sm">
            {{ content }}
        </div>
      </div>

      <DialogFooter class="p-6 pt-2">
        <Button variant="outline" @click="$emit('close')">Close</Button>
        <Button @click="copyToClipboard">Copy</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { toast } from '@/components/ui/toast'; // Or your notification system

const props = defineProps<{
  isOpen: boolean;
  title: string;
  content: string;
}>();

const emit = defineEmits(['close']);

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(props.content);
    // Simple alert or toast if available
    alert('Copied to clipboard');
  } catch (e) {
    console.error('Failed to copy', e);
  }
};
</script>
