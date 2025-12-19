<template>
  <div class="flex h-full border rounded-lg overflow-hidden bg-background">
    <!-- Sidebar -->
    <div class="w-64 border-r flex flex-col bg-muted/10">
      <div class="p-4 border-b flex items-center justify-between">
        <h2 class="font-semibold text-sm">{{ $t('nav.routes') }}</h2>
        <div class="flex gap-1">
            <button @click="openDialog('group')" class="p-1 hover:bg-muted rounded" :title="$t('route.group.new')">
                <FolderPlus class="w-4 h-4" />
            </button>
            <button @click="openDialog('route')" class="p-1 hover:bg-muted rounded" :title="$t('route.add')">
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
        <RouteTree
            :treeData="treeData"
            @select="selectNode"
            @context-menu="onContextMenu"
        />
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 overflow-hidden">
        <div v-if="!selectedRoute" class="h-full flex flex-col items-center justify-center text-muted-foreground">
            <Waypoints class="w-16 h-16 mb-4 opacity-20" />
            <p>{{ $t('route.no_routes') }}</p>
        </div>
        <RouteDetail
            v-else
            :route="selectedRoute"
            :repos="repos"
            @update="updateRoute"
            @delete="deleteRoute"
        />
    </div>

    <AddRouteDialog ref="dialogRef" :repos="repos" @create="handleCreate" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { FolderPlus, Plus, Search, Waypoints } from 'lucide-vue-next';
import RouteTree from '@/components/route/RouteTree.vue';
import RouteDetail from '@/components/route/RouteDetail.vue';
import AddRouteDialog from '@/components/route/AddRouteDialog.vue';

const treeData = ref<any[]>([]);
const searchQuery = ref('');
const selectedRoute = ref<any>(null);
const dialogRef = ref<any>(null);
const repos = ref<any[]>([]); // Need to fetch flat list of repos for dropdowns

const loadTree = async () => {
    try {
        const data = await invoke('list_route_tree');
        treeData.value = data as any[];
    } catch (e) {
        console.error('Failed to load route tree:', e);
    }
};

const loadRepos = async () => {
    // We need a flat list of repos. `list_repo_tree` returns nested.
    // For now, let's just fetch the tree and flatten it, or assuming backend has a list_all_repos (we didn't implement it yet explicitly flat, but tree contains all).
    // Let's implement a simple flatten helper on frontend for now.
    try {
        const tree: any[] = await invoke('list_repo_tree');
        const flat: any[] = [];
        const traverse = (nodes: any[]) => {
            for (const node of nodes) {
                if (node.repos) {
                    flat.push(...node.repos);
                    traverse(node.children);
                }
            }
        };
        traverse(tree);
        repos.value = flat;
    } catch (e) {
        console.error(e);
    }
}

onMounted(() => {
    loadTree();
    loadRepos();
});

const selectNode = (node: any) => {
    if (node.source_id !== undefined || node.target_id !== undefined) {
        // It's a route
        selectedRoute.value = node;
    } else {
        selectedRoute.value = null;
    }
};

const onContextMenu = (event: MouseEvent, node: any) => {
    console.log('Context menu', node);
};

const openDialog = (type: 'route' | 'group') => {
    dialogRef.value?.open(type);
};

const handleCreate = async (payload: any) => {
    try {
        if (payload.type === 'group') {
            await invoke('create_route_group', {
                name: payload.data.name,
                parent_id: payload.parentId
            });
        } else {
             await invoke('create_route', {
                name: payload.data.name,
                description: null,
                source_id: payload.data.source_id,
                target_id: payload.data.target_id,
                group_id: payload.parentId
            });
        }
        await loadTree();
    } catch (e) {
        console.error(e);
        alert('Error: ' + e);
    }
};

const updateRoute = async (payload: any) => {
     try {
        if (payload.type === 'info') {
            await invoke('update_route', {
                id: payload.data.id,
                name: payload.data.name,
                description: payload.data.description,
                source_id: payload.data.source_id,
                target_id: payload.data.target_id,
                group_id: payload.data.group_id
            });
        }
        // mappings are handled inside component via `update_route_mappings`
        await loadTree();
    } catch (e) {
        console.error(e);
    }
};

const deleteRoute = async (id: string) => {
     try {
        await invoke('delete_route', { id });
        selectedRoute.value = null;
        await loadTree();
    } catch (e) {
        console.error(e);
    }
}
</script>
