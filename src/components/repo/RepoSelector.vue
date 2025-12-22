<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { Search, ChevronDown, Check, Package, ChevronRight, Folder } from 'lucide-vue-next';

interface Repo {
  id: string;
  name: string;
  groupPath: string;
  [key: string]: any;
}

const props = defineProps<{
  modelValue: string | null;
  repos: Repo[];
  placeholder?: string;
}>();

const emit = defineEmits(['update:modelValue']);

const isOpen = ref(false);
const searchQuery = ref('');
const dropdownRef = ref<HTMLElement | null>(null);

// State for tree navigation
const currentPath = ref<string[]>([]);

const selectedRepo = computed(() => {
    return props.repos.find(r => r.id === props.modelValue);
});

// Build a tree structure from flat repos
const repoTree = computed(() => {
    const root: any = { children: {}, repos: [] };
    
    props.repos.forEach(repo => {
        const segments = repo.groupPath ? repo.groupPath.split(' / ') : [];
        let current = root;
        
        segments.forEach(segment => {
            if (!current.children[segment]) {
                current.children[segment] = { children: {}, repos: [] };
            }
            current = current.children[segment];
        });
        
        current.repos.push(repo);
    });
    
    return root;
});

// Navigate to current folder in the tree
const currentFolder = computed(() => {
    let current = repoTree.value;
    for (const segment of currentPath.value) {
        if (current.children[segment]) {
            current = current.children[segment];
        } else {
            return { children: {}, repos: [] }; // Fallback
        }
    }
    return current;
});

// Breadcrumbs for navigation
const breadcrumbs = computed(() => {
    const crumbs = [{ name: 'Root', path: [] as string[] }];
    let acc: string[] = [];
    currentPath.value.forEach(p => {
        acc = [...acc, p];
        crumbs.push({ name: p, path: acc });
    });
    return crumbs;
});

// Flatten filter for search
const filteredResults = computed(() => {
    if (!searchQuery.value) return null;
    const q = searchQuery.value.toLowerCase();
    return props.repos.filter(r => 
        r.name.toLowerCase().includes(q) || 
        (r.groupPath && r.groupPath.toLowerCase().includes(q))
    );
});

const selectRepo = (id: string) => {
    emit('update:modelValue', id);
    isOpen.value = false;
    searchQuery.value = '';
    currentPath.value = [];
};

const enterFolder = (name: string) => {
    currentPath.value.push(name);
};

const goToPath = (path: string[]) => {
    currentPath.value = [...path];
};

const handleClickOutside = (event: MouseEvent) => {
    if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
        isOpen.value = false;
    }
};

onMounted(() => {
    document.addEventListener('mousedown', handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener('mousedown', handleClickOutside);
});
</script>

<template>
  <div class="relative w-full" ref="dropdownRef">
    <!-- Trigger -->
    <div 
        @click="isOpen = !isOpen"
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background cursor-pointer hover:bg-accent/50 transition-colors"
        :class="{ 'ring-2 ring-ring ring-offset-2': isOpen }"
    >
        <div class="flex items-center gap-2 truncate">
            <Package v-if="selectedRepo" class="w-4 h-4 text-muted-foreground flex-shrink-0" />
            <div v-if="selectedRepo" class="flex flex-col leading-tight truncate">
                <span class="truncate font-medium">{{ selectedRepo.name }}</span>
                <span v-if="selectedRepo.groupPath" class="text-[10px] text-muted-foreground truncate">{{ selectedRepo.groupPath }}</span>
            </div>
            <span v-else class="text-muted-foreground">{{ placeholder || $t('repo.select', 'Select a repository...') }}</span>
        </div>
        <ChevronDown class="h-4 w-4 opacity-50 flex-shrink-0" />
    </div>

    <!-- Dropdown -->
    <div 
        v-if="isOpen"
        class="absolute z-50 mt-1 w-full rounded-md border bg-popover text-popover-foreground shadow-md outline-none animate-in fade-in-0 zoom-in-95 flex flex-col min-w-[240px]"
    >
        <!-- Search Header -->
        <div class="flex items-center border-b px-3 h-10 shrink-0">
            <Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
            <input 
                v-model="searchQuery"
                class="flex h-full w-full bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
                :placeholder="$t('common.filter', '搜索名称或路径...')"
                autoFocus
            />
        </div>

        <!-- Navigation Breadcrumbs (Only when not searching) -->
        <div v-if="!searchQuery" class="px-2 py-1 border-b bg-muted/30 flex flex-wrap items-center text-[10px] gap-1">
            <template v-for="(crumb, idx) in breadcrumbs" :key="idx">
                <span v-if="idx > 0" class="text-muted-foreground/50">/</span>
                <button 
                    @click="goToPath(crumb.path)"
                    class="hover:text-primary transition-colors hover:underline px-1 py-0.5 rounded"
                    :class="idx === breadcrumbs.length - 1 ? 'font-bold text-foreground' : 'text-muted-foreground'"
                >
                    {{ crumb.name === 'Root' ? $t('common.root', 'Root') : crumb.name }}
                </button>
            </template>
        </div>
        
        <!-- Content Area -->
        <div class="max-h-[300px] overflow-y-auto p-1">
            <!-- Search Results Mode -->
            <template v-if="searchQuery">
                <div v-if="filteredResults && filteredResults.length === 0" class="py-6 text-center text-sm text-muted-foreground">
                    {{ $t('common.no_matches', 'No matches found') }}
                </div>
                <div 
                    v-for="repo in filteredResults" 
                    :key="repo.id"
                    @click="selectRepo(repo.id)"
                    class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground transition-all group"
                >
                    <Package class="mr-2 h-4 w-4 opacity-70 group-hover:text-primary" />
                    <div class="flex-1 truncate">
                        <div class="font-medium truncate">{{ repo.name }}</div>
                        <div class="text-[10px] text-muted-foreground truncate">{{ repo.groupPath }}</div>
                    </div>
                    <Check v-if="modelValue === repo.id" class="ml-2 h-4 w-4 text-primary" />
                </div>
            </template>

            <!-- Tree Navigation Mode -->
            <template v-else>
                <!-- Folders -->
                <div 
                    v-for="(_, name) in currentFolder.children" 
                    :key="name"
                    @click="enterFolder(String(name))"
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-2 text-sm hover:bg-accent transition-colors group"
                >
                    <Folder class="mr-2 h-4 w-4 text-primary/70 opacity-80" />
                    <span class="flex-1 truncate">{{ name }}</span>
                    <ChevronRight class="h-3 w-3 text-muted-foreground opacity-50" />
                </div>

                <!-- Repositories -->
                <div v-if="currentFolder.repos.length === 0 && Object.keys(currentFolder.children).length === 0" class="py-6 text-center text-sm text-muted-foreground italic">
                    {{ $t('repo.no_repos_in_dir', 'No repositories in this directory') }}
                </div>
                
                <div 
                    v-for="repo in currentFolder.repos" 
                    :key="repo.id"
                    @click="selectRepo(repo.id)"
                    class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-2 text-sm outline-none hover:bg-accent hover:text-accent-foreground transition-all group"
                    :class="{ 'bg-accent/50': modelValue === repo.id }"
                >
                    <Package class="mr-2 h-4 w-4 opacity-70 group-hover:text-primary" />
                    <span class="flex-1 truncate font-medium">{{ repo.name }}</span>
                    <Check v-if="modelValue === repo.id" class="ml-2 h-4 w-4 text-primary" />
                </div>
            </template>
        </div>
    </div>
  </div>
</template>
