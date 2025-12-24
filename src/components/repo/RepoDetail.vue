<template>
  <div class="h-full flex flex-col bg-background">
    <!-- Header -->
    <div class="border-b p-6 flex flex-col gap-2">
      <div class="flex items-center gap-2 text-muted-foreground text-sm">
        <Package class="w-4 h-4" />
        <span>{{ repo.id }}</span>
        <!-- In real app, maybe show path breadcrumbs -->
      </div>
      <div class="flex justify-between items-start">
        <div class="flex items-center gap-3">
            <h1 class="text-2xl font-bold tracking-tight">{{ repo.name }}</h1>
            <button @click="togglePin" class="p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground" :title="localRepo.pinned ? 'Unpin Repository' : 'Pin Repository'">
                <PinOff v-if="localRepo.pinned" class="w-4 h-4" />
                <Pin v-else class="w-4 h-4" />
            </button>
        </div>
         <div class="flex gap-2">
             <!-- AI Actions Dropdown -->
             <DropdownMenu v-if="aiCommands.length > 0">
                <DropdownMenuTrigger asChild>
                    <Button variant="outline" class="gap-2">
                        <Sparkles class="w-4 h-4" />
                        AI Actions
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuItem
                        v-for="cmd in aiCommands"
                        :key="cmd.id"
                        @click="executeAiCommand(cmd)"
                    >
                        {{ cmd.name }}
                    </DropdownMenuItem>
                </DropdownMenuContent>
             </DropdownMenu>
             <div v-else class="text-xs text-muted-foreground flex items-center">
                 No AI commands configured.
                 <router-link to="/ai-commands" class="ml-1 underline">Configure</router-link>
             </div>
        </div>
      </div>
      <div class="flex items-center gap-4 text-sm mt-2 justify-between">
        <code class="bg-muted px-2 py-1 rounded">{{ repo.path }}</code>

        <div class="flex gap-2">
            <button @click="openInFolder" class="p-1 hover:bg-muted rounded text-muted-foreground hover:text-foreground" :title="$t('repo.actions.open_folder', 'Open Folder')">
                <FolderOpen class="w-4 h-4" />
            </button>
            <button @click="openInTerminal" class="p-1 hover:bg-muted rounded text-muted-foreground hover:text-foreground" :title="$t('repo.actions.open_terminal', 'Open Terminal')">
                <Terminal class="w-4 h-4" />
            </button>
            <!-- IDE Selector -->
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <button class="p-1 hover:bg-muted rounded text-muted-foreground hover:text-foreground flex items-center" :title="$t('repo.actions.open_ide', 'Open in IDE')">
                        <Code class="w-4 h-4" />
                    </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuItem @click="openInIde(preferredEditor)">
                        Default ({{ preferredEditor }})
                    </DropdownMenuItem>
                    <DropdownMenuItem v-for="ide in installedIdes" :key="ide.command" @click="openInIde(ide.command)">
                        {{ ide.name }}
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
      </div>
    </div>

    <!-- Tabs Header -->
    <div class="flex border-b px-6">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        @click="currentTab = tab.value"
        class="px-4 py-3 text-sm font-medium border-b-2 transition-colors"
        :class="currentTab === tab.value ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Tab Content -->
    <div class="flex-1 overflow-auto p-6">
      <div v-if="currentTab === 'overview'">
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
          <h3 class="text-lg font-semibold mb-4">Git Status</h3>

          <div class="grid gap-4 max-w-md">
              <div class="flex items-center justify-between">
                  <span class="text-sm font-medium text-muted-foreground">{{ $t('repo.branch.label', 'Branch') }}</span>
                  <div class="flex items-center gap-2">
                      <select
                        v-model="currentBranch"
                        @change="onBranchChange"
                        :disabled="isLoadingBranches"
                        class="h-9 w-48 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                      >
                          <option v-for="b in branches" :key="b.name" :value="b.name">
                              {{ b.name }} {{ b.is_remote ? '(Remote)' : '' }}
                          </option>
                      </select>
                      <Loader2 v-if="isLoadingBranches" class="w-4 h-4 animate-spin text-muted-foreground" />
                  </div>
              </div>
              <!-- Placeholder for commit info, could be expanded later -->
          </div>
        </div>
      </div>

      <div v-if="currentTab === 'history'">
          <div class="rounded-lg border bg-card text-card-foreground shadow-sm overflow-hidden">
              <div v-if="isLoadingLog" class="p-8 flex justify-center">
                  <Loader2 class="w-6 h-6 animate-spin text-muted-foreground" />
              </div>
              <div v-else-if="gitLog.length === 0" class="p-8 text-center text-muted-foreground">
                  {{ $t('repo.history.no_log', 'No commit history found') }}
              </div>
              <table v-else class="w-full text-sm text-left">
                  <thead class="bg-muted/50 text-muted-foreground">
                      <tr>
                          <th class="px-4 py-2 font-medium">Hash</th>
                          <th class="px-4 py-2 font-medium">Message</th>
                          <th class="px-4 py-2 font-medium">Author</th>
                          <th class="px-4 py-2 font-medium">Date</th>
                      </tr>
                  </thead>
                  <tbody>
                      <tr v-for="commit in gitLog" :key="commit.id" class="border-t hover:bg-muted/50 transition-colors">
                          <td class="px-4 py-2 font-mono text-xs">{{ commit.id }}</td>
                          <td class="px-4 py-2 max-w-xs truncate" :title="commit.message">{{ commit.message }}</td>
                          <td class="px-4 py-2">{{ commit.author }}</td>
                          <td class="px-4 py-2 text-muted-foreground text-xs">{{ commit.time }}</td>
                      </tr>
                  </tbody>
              </table>
          </div>
      </div>

      <div v-if="currentTab === 'settings'">
        <div class="grid gap-4 max-w-xl">
            <div class="grid gap-2">
                <label class="text-sm font-medium">{{ $t('repo.form.name.label') }}</label>
                <input v-model="localRepo.name" class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
            </div>
            <div class="grid gap-2">
                <label class="text-sm font-medium">{{ $t('repo.form.path.label') }}</label>
                <div class="flex gap-2">
                    <input v-model="localRepo.path" class="flex-1 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" />
                    <button type="button" @click="browsePath" class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-3">
                        {{ $t('common.browse') }}
                    </button>
                </div>
            </div>
             <div class="flex justify-end gap-2 mt-4">
                <button @click="deleteRepo" class="bg-destructive text-destructive-foreground hover:bg-destructive/90 px-4 py-2 rounded text-sm font-medium">
                    {{ $t('repo.context.delete') }}
                </button>
                <button @click="save" class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded text-sm font-medium">
                    {{ $t('actions.save') }}
                </button>
            </div>
        </div>
      </div>
    </div>
  </div>

  <!-- AI Result Modal -->
  <AIResultModal
    v-if="showAiModal"
    :isOpen="showAiModal"
    :title="currentAiCommandName"
    :content="aiResultContent"
    @close="showAiModal = false"
  />

  <CommandLogViewer ref="commandLogViewer" />
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { Package, Loader2, FolderOpen, Terminal, Code, Sparkles, Pin, PinOff } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import CommandLogViewer from '@/components/common/CommandLogViewer.vue';
import AIResultModal from '@/components/common/AIResultModal.vue';
import { Button } from '@/components/ui/button';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';

const { t } = useI18n();

const props = defineProps<{
  repo: any;
}>();

const emit = defineEmits(['update', 'delete']);

const tabs = [
  { label: t('repo.tabs.overview'), value: 'overview' },
  { label: t('repo.tabs.history'), value: 'history' },
  { label: t('repo.tabs.settings'), value: 'settings' },
];

const currentTab = ref('overview');
const localRepo = ref({ ...props.repo });
const branches = ref<any[]>([]);
const currentBranch = ref('');
const isLoadingBranches = ref(false);
const gitLog = ref<any[]>([]);
const isLoadingLog = ref(false);
const preferredEditor = ref('code');
const installedIdes = ref<any[]>([]);
const commandLogViewer = ref<any>(null);

// AI Commands
const aiCommands = ref<any[]>([]);
const showAiModal = ref(false);
const currentAiCommandName = ref('');
const aiResultContent = ref('');

const loadAiCommands = async () => {
    try {
        aiCommands.value = await invoke('get_ai_commands');
    } catch (e) {
        console.error('Failed to load AI commands', e);
    }
};

const executeAiCommand = async (cmd: any) => {
    if (!props.repo.path) return;

    currentAiCommandName.value = cmd.name;

    // Handle Context Gathering
    let contextContent = "";
    if (cmd.requires_diff) {
        // Fetch git diff
        // We might need a command for this
        try {
            // Reusing existing command if available, or run shell
            // For now, let's just get `git diff` via shell
             const output = await invoke('run_shell_command', {
                command: 'git',
                args: ['diff'],
                cwd: props.repo.path
            });
            contextContent += `\n\nGit Diff:\n${output}`;
        } catch (e) {
            console.error("Failed to get diff", e);
        }
    }

    if (cmd.requires_selection) {
        // Not implemented yet - this would require file selection UI
        contextContent += "\n\n[File Selection Context Placeholder]";
    }

    if (cmd.command_type === 'gemini') {
        // Call AI Service
        try {
            // We treat the template as User Prompt if system prompt is not separated,
            // or we can split it. For now, let's treat it as user prompt, possibly with context appended.
            const userPrompt = `${cmd.template}\n${contextContent}`;
            const result = await invoke('generate_ai_response', {
                userPrompt: userPrompt,
                systemPrompt: null
            });
            aiResultContent.value = result as string;
            showAiModal.value = true;
        } catch (e) {
            console.error('AI execution failed', e);
            alert('AI execution failed: ' + e);
        }
    } else if (cmd.command_type === 'shell') {
        // Execute Shell Script
        // NOTE: Executing arbitrary shell scripts from template might be risky if we don't handle args safely.
        // Assuming template IS the command line for now.
        // Or if it's a script content, we might need to write it to temp file.
        // Let's assume the template is "command arg1 arg2 ..."

        // However, standard use case is something like "git commit -m '...'" which is hard to template without variables.
        // If the template is just a static command:
        const parts = cmd.template.split(' ');
        const command = parts[0];
        const args = parts.slice(1);

        commandLogViewer.value?.open();
        try {
            await invoke('run_shell_command', {
                command: command,
                args: args,
                cwd: props.repo.path
            });
        } catch (e) {
             console.error('Shell execution failed', e);
        }
    }
};

const loadBranches = async () => {
    if (!props.repo.path) return;
    isLoadingBranches.value = true;
    try {
        const res = await invoke('get_git_branches', { path: props.repo.path });
        branches.value = res as any[];
        const active = branches.value.find((b: any) => b.is_current);
        if (active) currentBranch.value = active.name;
    } catch (e) {
        console.error('Failed to load branches:', e);
    } finally {
        isLoadingBranches.value = false;
    }
};

const onBranchChange = async () => {
    if (!currentBranch.value) return;
    try {
        await invoke('switch_git_branch', { path: props.repo.path, branch: currentBranch.value });
        alert(t('repo.branch.switch_success'));
    } catch (e) {
        console.error('Failed to switch branch:', e);
        alert(`${t('repo.branch.switch_failed')}: ${e}`);
    } finally {
        await loadBranches();
    }
};

const loadHistory = async () => {
    if (!props.repo.path) return;
    isLoadingLog.value = true;
    try {
        const res = await invoke('get_git_log', { path: props.repo.path, count: 50 });
        gitLog.value = res as any[];
    } catch (e) {
        console.error('Failed to load git log:', e);
    } finally {
        isLoadingLog.value = false;
    }
};

const openInFolder = async () => {
    if (!props.repo.path) return;
    try {
        await invoke('open_in_folder', { path: props.repo.path });
    } catch(e) {
        console.error(e);
        alert(e);
    }
};

const openInTerminal = async () => {
    if (!props.repo.path) return;
     try {
        await invoke('open_in_terminal', { path: props.repo.path });
    } catch(e) {
        console.error(e);
        alert(e);
    }
}

const openInIde = async (command: string) => {
    if (!props.repo.path) return;
    try {
        await invoke('open_in_ide', { path: props.repo.path, ide_command: command });
    } catch(e) {
        console.error(e);
        alert(e);
    }
}

watch(() => props.repo, (newVal) => {
    localRepo.value = { ...newVal };
    if (newVal.path) {
        loadBranches();
        if (currentTab.value === 'history') {
            loadHistory();
        }
    }
}, { immediate: true });

watch(currentTab, (newVal) => {
    if (newVal === 'history' && props.repo.path) {
        loadHistory();
    }
});

const save = () => {
    emit('update', localRepo.value);
};

const deleteRepo = () => {
    emit('delete', props.repo.id);
}

const togglePin = async () => {
    try {
        const newPinnedState = await invoke('toggle_pin_repo', { id: props.repo.id });
        localRepo.value.pinned = newPinnedState;
        // Optionally emit update if parent needs to know immediately, although parent passes prop.
        // If parent re-fetches, it might overwrite unless we emit.
        emit('update', localRepo.value);
    } catch (e) {
        console.error('Failed to toggle pin', e);
        alert('Failed to toggle pin: ' + e);
    }
};

const browsePath = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
        title: t('repo.dialog.select_path_title'),
    });
    if (selected && typeof selected === 'string') {
        localRepo.value.path = selected;
    }
};

onMounted(async () => {
    try {
        const val = await invoke('get_setting', { key: 'preferred_editor' });
        if (val) preferredEditor.value = val as string;

        loadAiCommands();

        // Load installed IDEs
        installedIdes.value = await invoke('get_available_ides');
    } catch (e) {
        console.error('Failed to load settings', e);
    }
});
</script>
