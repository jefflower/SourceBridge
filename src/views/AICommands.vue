<template>
  <div class="h-full flex flex-col space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold tracking-tight">AI Commands</h2>
        <p class="text-muted-foreground">Manage your custom AI commands and prompts.</p>
      </div>
      <button @click="isDialogOpen = true" class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 flex items-center gap-2">
        <span>Create Command</span>
      </button>
    </div>

    <!-- Empty State -->
    <div v-if="commands.length === 0" class="flex-1 border rounded-lg bg-card text-card-foreground shadow-sm p-8 flex flex-col items-center justify-center text-center">
      <div class="rounded-full bg-muted p-4 mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-8 h-8 text-muted-foreground"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><line x1="12" x2="12" y1="19" y2="22"/></svg>
      </div>
      <h3 class="text-lg font-semibold">No commands yet</h3>
      <p class="text-sm text-muted-foreground max-w-sm mt-2 mb-4">
        Create your first AI command to automate your workflow.
      </p>
      <button @click="isDialogOpen = true" class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
        Create Command
      </button>
    </div>

    <!-- Command List -->
     <div v-else class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div v-for="cmd in commands" :key="cmd.id" class="rounded-lg border bg-card text-card-foreground shadow-sm p-6 flex flex-col">
           <div class="flex justify-between items-start mb-2">
               <h3 class="font-semibold flex items-center gap-2">
                 <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-primary"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><line x1="12" x2="12" y1="19" y2="22"/></svg>
                 {{ cmd.name }}
               </h3>
               <button @click="deleteCommand(cmd.id)" class="text-muted-foreground hover:text-destructive transition-colors">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
               </button>
           </div>
           <div class="flex gap-2 mb-2">
             <span v-if="cmd.context !== 'none'" class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80">
                {{ cmd.context }}
             </span>
           </div>
           <p class="text-sm text-muted-foreground line-clamp-3 bg-muted/50 p-2 rounded flex-1">{{ cmd.prompt }}</p>
        </div>
     </div>

    <!-- Console Section -->
    <div class="h-1/3 border-t bg-black text-green-400 font-mono text-sm p-4 rounded-t-lg flex flex-col mt-4">
        <div class="flex items-center justify-between mb-2 border-b border-green-900 pb-2">
            <span class="font-bold">✨ Gemini Console (Preview)</span>
            <button @click="consoleLogs = []" class="text-xs hover:text-green-200">Clear</button>
        </div>
        <div class="flex-1 overflow-auto space-y-1 mb-2">
            <div v-for="(log, i) in consoleLogs" :key="i">
                <span class="opacity-50">[{{ log.time }}]</span> {{ log.text }}
            </div>
        </div>
        <div class="flex items-center gap-2">
            <span class="text-green-600">➜</span>
            <input 
                v-model="consoleInput"
                @keyup.enter="handleConsoleCommand"
                type="text" 
                class="flex-1 bg-transparent border-none outline-none text-green-400 placeholder-green-900"
                placeholder="Type 'gemini /pull' or 'gemini your prompt'..."
            />
        </div>
    </div>

    <!-- Create Dialog -->
    <Dialog :open="isDialogOpen" @update:open="isDialogOpen = $event">
        <DialogHeader>
          <DialogTitle>Create AI Command</DialogTitle>
          <DialogDescription>
            Define a new AI command or choose from a template.
          </DialogDescription>
        </DialogHeader>
        
        <div class="grid gap-4 py-4">
          <!-- Template Selector -->
          <div class="grid gap-2">
             <Label>Template (Optional)</Label>
             <select 
                @change="applyTemplate($event)"
                class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
             >
                <option value="">Custom Command</option>
                <option value="smart_pull">Smart Pull (Check Incoming Changes)</option>
                <option value="smart_commit">Smart Commit (Generate Message)</option>
                <option value="explain_code">Explain Code (Code Review)</option>
             </select>
          </div>

          <div class="grid gap-2">
            <Label for="name">Name</Label>
            <Input id="name" v-model="newCommand.name" placeholder="e.g., Explain Code" />
          </div>
          <div class="grid gap-2">
            <Label for="prompt">Prompt</Label>
            <textarea 
              id="prompt" 
              v-model="newCommand.prompt" 
              rows="4"
              class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="Enter your system prompt here..."
            />
          </div>
          <div class="grid gap-2">
            <Label for="context">Context</Label>
            <select
              id="context" 
              v-model="newCommand.context"
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
                <option value="none">None</option>
                <option value="diff_staged">Staged Changes (Git Diff)</option>
                <option value="diff_working">Working Changes (Git Diff)</option>
                <option value="incoming_commits">Incoming Commits (Git Pull)</option>
            </select>
            <p class="text-xs text-muted-foreground">The data that will be fed to the AI along with your prompt.</p>
          </div>
        </div>

        <DialogFooter>
          <button @click="isDialogOpen = false" class="px-4 py-2 hover:bg-muted rounded-md text-sm font-medium text-foreground">Cancel</button>
          <button @click="saveCommand" class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 text-sm font-medium">Save Command</button>
        </DialogFooter>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Dialog, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog';
import Input from '@/components/ui/input/Input.vue';
import Label from '@/components/ui/label/Label.vue';

interface AICommand {
    id: string;
    name: string;
    prompt: string;
    context: string;
}

const isDialogOpen = ref(false);
const commands = ref<AICommand[]>([]);
const newCommand = ref({
    name: '',
    prompt: '',
    context: 'none'
});

// Console Logic
const consoleInput = ref('');
const consoleLogs = ref<{time: string, text: string}[]>([]);

const addLog = (text: string) => {
    const time = new Date().toLocaleTimeString();
    consoleLogs.value.push({ time, text });
};

const handleConsoleCommand = () => {
    const cmd = consoleInput.value.trim();
    if (!cmd) return;
    
    addLog(`$ ${cmd}`);
    consoleInput.value = '';

    if (cmd.startsWith('gemini')) {
        const args = cmd.replace('gemini', '').trim();
        
        if (args === '/pull') {
            addLog('⚡️ Executing Smart Pull Check...');
            addLog('   → Fetching origin...');
            addLog('   → Analyzing incoming commits...');
            setTimeout(() => addLog('✅ AI Analysis: No breaking changes detected. Safe to pull.'), 1000);
        } else if (args === '/commit') {
             addLog('⚡️ Generating Smart Commit Message...');
             addLog('   → Analyzing staged changes...');
             setTimeout(() => addLog('📝 Proposed: "feat: add interactive console to AI commands"'), 1000);
        } else {
             addLog(`🤖 Processing prompt: "${args}"`);
             setTimeout(() => addLog(`💬 AI Response for "${args}" (Simulation)`), 800);
        }
    } else {
        addLog(`❌ Unknown command: ${cmd.split(' ')[0]}. Try starting with 'gemini'.`);
    }
};

const applyTemplate = (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const value = target.value;
    
    if (value === 'smart_pull') {
        newCommand.value = {
            name: 'Smart Pull',
            prompt: 'Please summarize the incoming changes and highlight any potential breaking changes or risks.',
            context: 'incoming_commits'
        };
    } else if (value === 'smart_commit') {
        newCommand.value = {
            name: 'Smart Commit',
            prompt: 'Generate a conventional commit message based on the following staged changes.',
            context: 'diff_staged'
        };
    } else if (value === 'explain_code') {
        newCommand.value = {
            name: 'Explain Code',
            prompt: 'Explain the following code snippet in detail, focusing on logic and potential edge cases.',
            context: 'none'
        };
    } else {
        newCommand.value = { name: '', prompt: '', context: 'none' };
    }
};

const saveCommand = () => {
    if (!newCommand.value.name || !newCommand.value.prompt) return;
    
    commands.value.push({
        id: Date.now().toString(),
        name: newCommand.value.name,
        prompt: newCommand.value.prompt,
        context: newCommand.value.context
    });
    
    newCommand.value = { name: '', prompt: '', context: 'none' };
    isDialogOpen.value = false;
};

const deleteCommand = (id: string) => {
    commands.value = commands.value.filter(c => c.id !== id);
};
</script>
