<template>
  <Dialog :open="isOpen" @update:open="setOpen">
    <DialogContent class="sm:max-w-[600px] h-[80vh] flex flex-col">
      <DialogHeader>
        <DialogTitle>{{ $t('repo.scan.title') }}</DialogTitle>
        <DialogDescription>
          {{ $t('repo.scan.desc') }}
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 flex flex-col gap-4 py-4 min-h-0">
        <!-- Folder Selection -->
        <div class="flex gap-2 items-end">
          <div class="flex-1 gap-1.5 grid">
            <Label for="scanPath">{{ $t('scan.root_path') }}</Label>
            <Input id="scanPath" v-model="rootPath" readonly :placeholder="$t('scan.click_browse')" />
          </div>
          <Button @click="selectFolder" variant="outline">
            {{ $t('common.browse') }}
          </Button>
          <Button @click="startScan" :disabled="!rootPath || isScanning || isImporting">
            {{ isScanning ? $t('common.scanning') : $t('common.scan') }}
          </Button>
        </div>

        <!-- Scan Results -->
        <div class="flex-1 border rounded-md overflow-hidden flex flex-col min-h-0 relative">
            <div v-if="isScanning" class="absolute inset-0 bg-background/50 flex items-center justify-center z-10">
                <Loader2 class="w-8 h-8 animate-spin text-primary" />
            </div>

            <div class="p-2 border-b bg-muted/20 flex items-center justify-between text-xs">
                <div class="flex items-center gap-2">
                    <Checkbox :checked="allChecked" @update:checked="toggleAll" id="selectAll" />
                    <label for="selectAll" class="cursor-pointer font-medium">{{ $t('scan.found', { count: scannedRepos.length }) }}</label>
                </div>
                <span class="text-muted-foreground">{{ $t('scan.selected', { count: selectedCount }) }}</span>
            </div>

            <div class="flex-1 overflow-y-auto p-2 space-y-1">
                <div v-for="(repo, index) in scannedRepos" :key="index" class="flex items-center gap-2 p-2 hover:bg-muted rounded text-sm">
                     <Checkbox :checked="repo.checked" @update:checked="(v: boolean) => repo.checked = v" />
                     <div class="flex-1 min-w-0">
                         <div class="font-medium truncate" :title="repo.name">{{ repo.name }}</div>
                         <div class="text-xs text-muted-foreground truncate" :title="repo.relative_path">{{ repo.relative_path }}</div>
                     </div>
                </div>
                <div v-if="scannedRepos.length === 0 && !isScanning && hasScanned" class="text-center py-8 text-muted-foreground">
                    {{ $t('scan.no_results') }}
                </div>
            </div>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="isOpen = false">
          {{ $t('common.cancel', 'Cancel') }}
        </Button>
        <Button @click="confirmImport" :disabled="selectedCount === 0 || isImporting">
            <Loader2 v-if="isImporting" class="w-4 h-4 mr-2 animate-spin" />
            {{ isImporting ? $t('common.importing', 'Importing...') : $t('common.import', 'Import Selected') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Loader2 } from 'lucide-vue-next';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';

const emit = defineEmits(['import-complete']);

const isOpen = ref(false);
const rootPath = ref('');
const isScanning = ref(false);
const isImporting = ref(false);
const hasScanned = ref(false);

interface ScannedRepo {
    path: string;
    name: string;
    relative_path: string;
    checked: boolean;
}

const scannedRepos = ref<ScannedRepo[]>([]);

const selectedCount = computed(() => scannedRepos.value.filter(r => r.checked).length);
const allChecked = computed(() => scannedRepos.value.length > 0 && scannedRepos.value.every(r => r.checked));

const setOpen = (val: boolean) => {
    isOpen.value = val;
    if (!val) {
        // Reset state slightly? Or keep it?
        // Keep it for now.
    }
};

const openDialog = () => {
    isOpen.value = true;
};

const selectFolder = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
    });

    if (selected) {
        rootPath.value = selected as string;
        // Optionally auto-scan?
        // startScan();
    }
};

const startScan = async () => {
    if (!rootPath.value) return;

    isScanning.value = true;
    scannedRepos.value = [];
    hasScanned.value = false;

    try {
        const result = await invoke<Omit<ScannedRepo, 'checked'>[]>('scan_local_repos', { root_path: rootPath.value });
        scannedRepos.value = result.map(r => ({ ...r, checked: true }));
        hasScanned.value = true;
    } catch (e) {
        console.error(e);
        // Show error toast/alert
        alert('Scan failed: ' + e);
    } finally {
        isScanning.value = false;
    }
};

const toggleAll = (checked: boolean) => {
    scannedRepos.value.forEach(r => r.checked = checked);
};

const confirmImport = async () => {
    const toImport = scannedRepos.value.filter(r => r.checked);
    if (toImport.length === 0) return;

    isImporting.value = true;

    try {
        // Strip the 'checked' property before sending
        const payload = toImport.map(({ checked, ...rest }) => rest);

        const message = await invoke<string>('import_scanned_repos', {
            root_path: rootPath.value,
            repos: payload
        });

        // alert(message);
        emit('import-complete', message);
        isOpen.value = false;

        // Clear state
        scannedRepos.value = [];
        rootPath.value = '';
        hasScanned.value = false;

    } catch (e) {
        console.error(e);
        alert('Import failed: ' + e);
    } finally {
        isImporting.value = false;
    }
};

defineExpose({ open: openDialog });
</script>
