<template>
  <div class="space-y-4">
    <div class="bg-muted/30 p-4 rounded-lg border">
      <h3 class="text-sm font-semibold mb-3 flex items-center gap-2">
        <Rocket class="w-4 h-4 text-primary" />
        {{ $t('workspace.title') }}
      </h3>

      <div class="grid gap-4">
        <div class="grid gap-2">
          <label class="text-xs font-medium">{{ $t('workspace.source_path') }}</label>
          <div class="flex gap-2">
            <input
              v-model="config.source_path"
              class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm font-mono"
              :placeholder="$t('workspace.placeholder.source')"
            />
            <button @click="pickFolder('source')" class="px-3 border rounded-md hover:bg-muted">
              <FolderOpen class="w-4 h-4" />
            </button>
          </div>
        </div>

        <div class="grid gap-2">
          <label class="text-xs font-medium">{{ $t('workspace.target_path') }}</label>
          <div class="flex gap-2">
            <input
              v-model="config.target_path"
              class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm font-mono"
              :placeholder="$t('workspace.placeholder.target')"
            />
            <button @click="pickFolder('target')" class="px-3 border rounded-md hover:bg-muted">
              <FolderOpen class="w-4 h-4" />
            </button>
          </div>
        </div>

        <div class="grid gap-2">
          <label class="text-xs font-medium">{{ $t('workspace.open_command') }}</label>
          <div class="flex gap-2">
            <input
              v-model="config.open_command"
              class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm font-mono"
              :placeholder="$t('workspace.placeholder.command')"
            />
             <!-- Quick presets -->
             <div class="flex items-center gap-1">
                <button
                  v-for="preset in ['code .', 'cursor .', 'idea .']"
                  :key="preset"
                  @click="config.open_command = preset"
                  class="text-[10px] px-2 py-1 bg-secondary text-secondary-foreground rounded hover:opacity-80 whitespace-nowrap"
                >
                  {{ preset }}
                </button>
             </div>
          </div>
          <p class="text-[10px] text-muted-foreground">{{ $t('workspace.command_hint') }}</p>
        </div>

        <div class="flex justify-end gap-2 pt-2">
            <button
                @click="launch"
                :disabled="!isValid"
                class="bg-green-600 text-white hover:bg-green-700 px-4 py-2 rounded text-sm font-medium flex items-center gap-2 disabled:opacity-50"
            >
                <Play class="w-3 h-3" /> {{ $t('workspace.launch') }}
            </button>
            <button
                @click="save"
                class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium"
            >
                {{ $t('actions.save') }}
            </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { Rocket, FolderOpen, Play } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';


const props = defineProps<{
  groupId: string;
}>();



interface WorkspaceConfig {
  source_path?: string;
  target_path?: string;
  open_command?: string;
}

const config = ref<WorkspaceConfig>({
  source_path: '',
  target_path: '',
  open_command: 'code .'
});

const loadConfig = async () => {
  try {
    const data = await invoke<WorkspaceConfig | null>('get_workspace_config', { groupId: props.groupId });
    if (data) {
      config.value = {
        source_path: data.source_path || '',
        target_path: data.target_path || '',
        open_command: data.open_command || 'code .'
      };
    } else {
      // Defaults
       config.value = {
        source_path: '',
        target_path: '',
        open_command: 'code .'
      };
    }
  } catch (e) {
    console.error('Failed to load workspace config', e);
  }
};

watch(() => props.groupId, loadConfig, { immediate: true });

const isValid = computed(() => {
    return (config.value.source_path || config.value.target_path) && config.value.open_command;
});

const pickFolder = async (field: 'source' | 'target') => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      if (field === 'source') config.value.source_path = selected as string;
      else config.value.target_path = selected as string;
    }
  } catch (e) {
    console.error('Failed to pick folder', e);
  }
};

const save = async () => {
  try {
    await invoke('save_workspace_config', {
      groupId: props.groupId,
      sourcePath: config.value.source_path,
      targetPath: config.value.target_path,
      openCommand: config.value.open_command
    });
    // Maybe show toast
  } catch (e) {
    console.error('Failed to save workspace config', e);
  }
};

const launch = async () => {
  try {
     // Save first to ensure latest config is used? Or pass params directly?
     // Backend 'launch_workspace' reads from DB. So we must save first.
     await save();
     await invoke('launch_workspace', { groupId: props.groupId });
  } catch (e) {
    console.error('Failed to launch workspace', e);
    alert('Launch failed: ' + e);
  }
};
</script>
