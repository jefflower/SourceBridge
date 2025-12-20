<template>
  <div class="flex h-full border rounded-lg overflow-hidden bg-background">
    <!-- Sidebar -->
    <div class="w-64 border-r flex flex-col bg-muted/10">
      <div class="p-4 border-b flex items-center justify-between">
        <h2 class="font-semibold text-sm">{{ $t('nav.repos') }}</h2>
        <div class="flex gap-1">
            <button @click="openScanDialog" class="p-1 hover:bg-muted rounded" :title="$t('repo.scan.title', 'Scan Import')">
                <FolderSearch class="w-4 h-4" />
            </button>
            <button @click="openDialog('group')" class="p-1 hover:bg-muted rounded" :title="$t('repo.group.new')">
                <FolderPlus class="w-4 h-4" />
            </button>
            <button @click="openDialog('repo')" class="p-1 hover:bg-muted rounded" :title="$t('repo.add')">
                <Plus class="w-4 h-4" />
            </button>
        </div>
      </div>

      <!-- Filter -->
      <div class="p-2 border-b">
         <div class="relative">
             <Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
             <input v-model="searchQuery" :placeholder="$t('common.filter')" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 pl-8" />
         </div>
      </div>

      <!-- Tree -->
      <div class="flex-1 overflow-auto">
        <RepoTree
            :treeData="treeData"
            @select="selectNode"
            @context-menu="onContextMenu"
            @move="handleMove"
        />
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 overflow-hidden">
        <div v-if="!selectedRepo" class="h-full flex flex-col items-center justify-center text-muted-foreground">
            <PackageOpen class="w-16 h-16 mb-4 opacity-20" />
            <p>{{ $t('repo.no_repos') }}</p>
        </div>
        <RepoDetail
            v-else
            :repo="selectedRepo"
            @update="updateRepo"
            @delete="deleteRepo"
        />
    </div>

    <AddRepoDialog ref="dialogRef" @create="handleCreate" />
    <ScanImportDialog ref="scanDialogRef" @import-complete="handleImportComplete" />
    
    <!-- Context Menu -->
    <ContextMenu ref="contextMenuRef" :items="contextMenuItems" @select="handleContextMenuAction" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ask } from '@tauri-apps/plugin-dialog';
import { FolderPlus, Plus, Search, PackageOpen, Trash2, FolderPlus as NewSubgroup, PackagePlus, FolderSearch } from 'lucide-vue-next';
import RepoTree from '@/components/repo/RepoTree.vue';
import RepoDetail from '@/components/repo/RepoDetail.vue';
import AddRepoDialog from '@/components/repo/AddRepoDialog.vue';
import ScanImportDialog from '@/components/repo/ScanImportDialog.vue';
import ContextMenu from '@/components/common/ContextMenu.vue';
import type { MenuItem } from '@/components/common/ContextMenu.vue';

const treeData = ref<any[]>([]);
const searchQuery = ref('');
const selectedRepo = ref<any>(null);
const dialogRef = ref<any>(null);
const scanDialogRef = ref<any>(null);
const contextMenuRef = ref<any>(null);
const contextMenuNode = ref<any>(null);

// Context menu items based on node type
const contextMenuItems = computed<MenuItem[]>(() => {
    if (!contextMenuNode.value) return [];
    
    const isGroup = contextMenuNode.value.children !== undefined || contextMenuNode.value.repos !== undefined;
    
    if (isGroup) {
        return [
            { key: 'new_subgroup', label: '新建子分组', icon: NewSubgroup },
            { key: 'add_repo', label: '在此添加仓库', icon: PackagePlus },
            { key: 'delete', label: '删除分组', icon: Trash2, danger: true },
        ];
    } else {
        return [
            { key: 'delete', label: '删除仓库', icon: Trash2, danger: true },
        ];
    }
});

const loadTree = async () => {
    try {
        const data = await invoke('list_repo_tree');
        console.log('[loadTree] Tree data from backend:', JSON.stringify(data, null, 2));
        treeData.value = data as any[];
    } catch (e) {
        console.error('Failed to load repo tree:', e);
    }
};

onMounted(() => {
    loadTree();
});

const selectNode = (node: any) => {
    if (node.group_id !== undefined || node.type === 'repo') {
        selectedRepo.value = node;
    } else {
        selectedRepo.value = null;
    }
};

const onContextMenu = (event: MouseEvent, node: any) => {
    contextMenuNode.value = node;
    contextMenuRef.value?.open(event);
};

const handleContextMenuAction = async (action: string) => {
    const node = contextMenuNode.value;
    if (!node) return;
    
    const isGroup = node.children !== undefined || node.repos !== undefined;
    
    switch (action) {
        case 'new_subgroup':
            dialogRef.value?.open('group', node.id);
            break;
        case 'add_repo':
            dialogRef.value?.open('repo', node.id);
            break;
        case 'delete':
            const confirmed = await ask(`确定要删除"${node.name}"吗？`, {
                title: '删除确认',
                kind: 'warning'
            });
            
            if (confirmed) {
                try {
                    if (isGroup) {
                        await invoke('delete_repo_group', { id: node.id });
                    } else {
                        await invoke('delete_repository', { id: node.id });
                    }
                    if (selectedRepo.value?.id === node.id) {
                        selectedRepo.value = null;
                    }
                    await loadTree();
                } catch (e) {
                    console.error(e);
                    alert('删除失败: ' + e);
                }
            }
            break;
    }
};

const openDialog = (type: 'repo' | 'group') => {
    dialogRef.value?.open(type);
};

const openScanDialog = () => {
    scanDialogRef.value?.open();
};

const handleImportComplete = async (msg: string) => {
    console.log(msg);
    await loadTree();
};

const handleCreate = async (payload: any) => {
    console.log('[handleCreate] Payload:', payload);
    try {
        if (payload.type === 'group') {
            console.log('[handleCreate] Creating group with parent_id:', payload.parentId);
            await invoke('create_repo_group', {
                name: payload.data.name,
                parent_id: payload.parentId
            });
        } else {
            console.log('[handleCreate] Creating repo with group_id:', payload.parentId);
             await invoke('add_repository', {
                name: payload.data.name,
                path: payload.data.path,
                url: null,
                group_id: payload.parentId
            });
        }
        console.log('[handleCreate] Success, reloading tree...');
        await loadTree();
    } catch (e) {
        console.error('[handleCreate] Error:', e);
        alert('Error: ' + e);
    }
};

const updateRepo = async (repo: any) => {
     try {
        await invoke('update_repository', {
            id: repo.id,
            name: repo.name,
            path: repo.path,
            url: null,
            group_id: repo.group_id
        });
        await loadTree();
    } catch (e) {
        console.error(e);
    }
};

const deleteRepo = async (id: string) => {
    const confirmed = await ask('确定要删除该仓库吗？', {
        title: '删除确认',
        kind: 'warning'
    });
    
    if (!confirmed) return;

     try {
        await invoke('delete_repository', { id });
        selectedRepo.value = null;
        await loadTree();
    } catch (e) {
        console.error(e);
    }
}

const handleMove = async (data: { draggedId: string; draggedType: string; targetGroupId: string }) => {
    console.log('[handleMove] Received:', data);
    try {
        if (data.draggedType === 'group') {
            console.log('[handleMove] Moving group', data.draggedId, 'to parent', data.targetGroupId);
            await invoke('update_repo_group_parent', { 
                id: data.draggedId, 
                parent_id: data.targetGroupId 
            });
        } else {
            console.log('[handleMove] Moving repo', data.draggedId, 'to group', data.targetGroupId);
            await invoke('update_repository_group', { 
                id: data.draggedId, 
                group_id: data.targetGroupId 
            });
        }
        console.log('[handleMove] Success, reloading tree...');
        await loadTree();
    } catch (e) {
        console.error('[handleMove] Error:', e);
        alert('移动失败: ' + e);
    }
};
</script>
