<template>
  <div class="flex h-full border rounded-lg overflow-hidden bg-background">
    <!-- Sidebar -->
    <div class="w-64 border-r flex flex-col bg-muted/10">
      <div class="p-4 border-b flex items-center justify-between">
        <h2 class="font-semibold text-sm">{{ $t('nav.repos') }}</h2>
        <div class="flex gap-1">
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
             <input v-model="searchQuery" placeholder="Filter..." class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 pl-8" />
         </div>
      </div>

      <!-- Tree -->
      <div class="flex-1 overflow-auto">
        <RepoTree
            :treeData="treeData"
            @select="selectNode"
            @context-menu="onContextMenu"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { FolderPlus, Plus, Search, PackageOpen } from 'lucide-vue-next';
import RepoTree from '@/components/repo/RepoTree.vue';
import RepoDetail from '@/components/repo/RepoDetail.vue';
import AddRepoDialog from '@/components/repo/AddRepoDialog.vue';

const treeData = ref<any[]>([]);
const searchQuery = ref('');
const selectedRepo = ref<any>(null);
const dialogRef = ref<any>(null);

const loadTree = async () => {
    try {
        const data = await invoke('list_repo_tree');
        treeData.value = data as any[];
        // Flatten or process if needed, but tree component handles recursion
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
        // It's a group, maybe show group stats?
        selectedRepo.value = null;
    }
};

const onContextMenu = (event: MouseEvent, node: any) => {
    // Implement Context Menu (out of scope for quick verification, but logical step)
    console.log('Context menu', node);
};

const openDialog = (type: 'repo' | 'group') => {
    dialogRef.value?.open(type);
};

const handleCreate = async (payload: any) => {
    try {
        if (payload.type === 'group') {
            await invoke('create_repo_group', {
                name: payload.data.name,
                parentId: payload.parentId
            });
        } else {
             await invoke('add_repository', {
                name: payload.data.name,
                path: payload.data.path,
                url: null,
                groupId: payload.parentId
            });
        }
        await loadTree();
    } catch (e) {
        console.error(e);
        alert('Error: ' + e);
    }
};

const updateRepo = async (repo: any) => {
     try {
        await invoke('update_repository', {
            id: repo.id,
            name: repo.name,
            path: repo.path,
            url: null, // TODO
            groupId: repo.group_id
        });
        await loadTree();
        // re-select?
    } catch (e) {
        console.error(e);
    }
};

const deleteRepo = async (id: string) => {
     try {
        await invoke('delete_repository', { id });
        selectedRepo.value = null;
        await loadTree();
    } catch (e) {
        console.error(e);
    }
}
</script>
