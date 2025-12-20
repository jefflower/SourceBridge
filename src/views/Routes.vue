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
             <input v-model="searchQuery" :placeholder="$t('common.filter')" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 pl-8" />
         </div>
      </div>

      <!-- Tree -->
      <div class="flex-1 overflow-auto">
        <RouteTree
            :treeData="treeData"
            @select="selectNode"
            @context-menu="onContextMenu"
            @move="handleMove"
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
    
    <!-- Context Menu -->
    <ContextMenu ref="contextMenuRef" :items="contextMenuItems" @select="handleContextMenuAction" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { FolderPlus, Plus, Search, Waypoints, Trash2, FolderPlus as NewSubgroup, Route } from 'lucide-vue-next';
import RouteTree from '@/components/route/RouteTree.vue';
import RouteDetail from '@/components/route/RouteDetail.vue';
import AddRouteDialog from '@/components/route/AddRouteDialog.vue';
import ContextMenu from '@/components/common/ContextMenu.vue';
import type { MenuItem } from '@/components/common/ContextMenu.vue';

const treeData = ref<any[]>([]);
const searchQuery = ref('');
const selectedRoute = ref<any>(null);
const dialogRef = ref<any>(null);
const repos = ref<any[]>([]);
const contextMenuRef = ref<any>(null);
const contextMenuNode = ref<any>(null);

// Context menu items based on node type
const contextMenuItems = computed<MenuItem[]>(() => {
    if (!contextMenuNode.value) return [];
    
    const isGroup = contextMenuNode.value.children !== undefined || contextMenuNode.value.routes !== undefined;
    
    if (isGroup) {
        return [
            { key: 'new_subgroup', label: '新建子分组', icon: NewSubgroup },
            { key: 'add_route', label: '在此添加路线', icon: Route },
            { key: 'delete', label: '删除分组', icon: Trash2, danger: true },
        ];
    } else {
        return [
            { key: 'delete', label: '删除路线', icon: Trash2, danger: true },
        ];
    }
});

const loadTree = async () => {
    try {
        const data = await invoke('list_route_tree');
        treeData.value = data as any[];
    } catch (e) {
        console.error('Failed to load route tree:', e);
    }
};

const loadRepos = async () => {
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
        selectedRoute.value = node;
    } else {
        selectedRoute.value = null;
    }
};

const onContextMenu = (event: MouseEvent, node: any) => {
    contextMenuNode.value = node;
    contextMenuRef.value?.open(event);
};

const handleContextMenuAction = async (action: string) => {
    const node = contextMenuNode.value;
    if (!node) return;
    
    const isGroup = node.children !== undefined || node.routes !== undefined;
    
    switch (action) {
        case 'new_subgroup':
            dialogRef.value?.open('group', node.id);
            break;
        case 'add_route':
            dialogRef.value?.open('route', node.id);
            break;
        case 'delete':
            if (confirm(`确定要删除"${node.name}"吗？`)) {
                try {
                    if (isGroup) {
                        await invoke('delete_route_group', { id: node.id });
                    } else {
                        await invoke('delete_route', { id: node.id });
                    }
                    if (selectedRoute.value?.id === node.id) {
                        selectedRoute.value = null;
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

const handleMove = async (data: { draggedId: string; draggedType: string; targetGroupId: string }) => {
    try {
        if (data.draggedType === 'group') {
            await invoke('update_route_group_parent', { 
                id: data.draggedId, 
                parent_id: data.targetGroupId 
            });
        } else {
            await invoke('update_route_group_id', { 
                id: data.draggedId, 
                group_id: data.targetGroupId 
            });
        }
        await loadTree();
    } catch (e) {
        console.error('Move failed:', e);
        alert('移动失败: ' + e);
    }
};
</script>

