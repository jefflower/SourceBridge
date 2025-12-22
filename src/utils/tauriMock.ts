/**
 * Tauri API Mock for Browser Development
 * This file provides mock implementations of Tauri APIs when running in a browser
 * without the Tauri runtime (e.g., via npm run dev without tauri)
 */

// Extend Window interface for Tauri globals
declare global {
  interface Window {
    __TAURI_INTERNALS__?: {
      invoke: (cmd: string, args?: any) => Promise<any>;
      metadata?: any;
    };
  }
}

// Check if we're running in Tauri or browser
export const isTauri = (): boolean => {
  return typeof window !== 'undefined' && 
         (window.__TAURI_INTERNALS__ !== undefined || 
          (window as any).__TAURI_IPC__ !== undefined);
};

// Mock data storage (in-memory for browser development)
const mockStorage: Record<string, any> = {
  settings: {
    theme: 'system',
    language: 'zh',
    git_path: 'git'
  },
  repoGroups: [
    { id: 'group-1', name: '工作项目', parent_id: null, sort_order: 0 },
    { id: 'group-2', name: '个人项目', parent_id: null, sort_order: 1 },
    { id: 'group-3', name: '前端项目', parent_id: 'group-1', sort_order: 0 },
  ],
  repositories: [
    { id: 'repo-1', name: 'SourceBridge', local_path: '/path/to/source-bridge', group_id: 'group-3' },
    { id: 'repo-2', name: 'MyApp', local_path: '/path/to/my-app', group_id: 'group-2' },
  ],
  routeGroups: [
    { id: 'route-group-1', name: '同步路线组', parent_id: null, sort_order: 0 },
  ],
  routes: [],
  tasks: []
};

// Helper to build tree from flat data
function buildRepoTree(): any[] {
  const groups = mockStorage.repoGroups;
  const repos = mockStorage.repositories;
  
  const buildTree = (parentId: string | null): any[] => {
    const result: any[] = [];
    
    for (const group of groups) {
      if (group.parent_id === parentId) {
        const children = buildTree(group.id);
        const groupRepos = repos
          .filter((r: any) => r.group_id === group.id)
          .map((r: any) => ({
            id: r.id,
            name: r.name,
            path: r.local_path,
            group_id: r.group_id
          }));
        
        result.push({
          id: group.id,
          name: group.name,
          children,
          repos: groupRepos
        });
      }
    }
    
    // Add uncategorized repos at root level
    if (parentId === null) {
      const rootRepos = repos.filter((r: any) => !r.group_id);
      if (rootRepos.length > 0) {
        result.push({
          id: 'root_virtual',
          name: 'Uncategorized',
          children: [],
          repos: rootRepos.map((r: any) => ({
            id: r.id,
            name: r.name,
            path: r.local_path,
            group_id: null
          }))
        });
      }
    }
    
    return result;
  };
  
  return buildTree(null);
}

function buildRouteTree() {
  const groups = mockStorage.routeGroups;
  const routes = mockStorage.routes;
  
  const buildTree = (parentId: string | null): any[] => {
    const result: any[] = [];
    
    for (const group of groups) {
      if (group.parent_id === parentId) {
        const children = buildTree(group.id);
        const groupRoutes = routes
          .filter((r: any) => r.group_id === group.id)
          .map((r: any) => ({
            id: r.id,
            name: r.name,
            group_id: r.group_id,
            source_id: r.source_id,
            target_id: r.target_id
          }));
        
        result.push({
          id: group.id,
          name: group.name,
          children,
          routes: groupRoutes
        });
      }
    }
    
    return result;
  };
  
  return buildTree(null);
}

// Mock command handlers
const mockCommands: Record<string, (args: any) => Promise<any>> = {
  // Settings
  get_setting: async ({ key }: { key: string }) => {
    return mockStorage.settings[key] || null;
  },
  set_setting: async ({ key, value }: { key: string; value: string }) => {
    mockStorage.settings[key] = value;
    console.log('[Mock] Setting saved:', key, '=', value);
  },
  get_all_settings: async () => {
    return mockStorage.settings;
  },
  
  // Repo Groups
  create_repo_group: async ({ name, parent_id }: { name: string; parent_id?: string }) => {
    const id = 'group-' + Date.now();
    mockStorage.repoGroups.push({ id, name, parent_id: parent_id || null, sort_order: 0 });
    console.log('[Mock] Created repo group:', name, 'parent:', parent_id);
  },
  delete_repo_group: async ({ id }: { id: string }) => {
    const index = mockStorage.repoGroups.findIndex((g: any) => g.id === id);
    if (index !== -1) {
      mockStorage.repoGroups.splice(index, 1);
      console.log('[Mock] Deleted repo group:', id);
    }
  },
  update_repo_group_parent: async ({ id, parent_id }: { id: string; parent_id?: string }) => {
    const group = mockStorage.repoGroups.find((g: any) => g.id === id);
    if (group) {
      group.parent_id = parent_id || null;
      console.log('[Mock] Updated repo group parent:', id, '->', parent_id);
    }
  },
  
  // Repositories
  add_repository: async ({ name, path, group_id }: { name: string; path: string; group_id?: string }) => {
    const id = 'repo-' + Date.now();
    mockStorage.repositories.push({ id, name, local_path: path, group_id: group_id || null });
    console.log('[Mock] Added repository:', name);
  },
  update_repository: async ({ id, name, path, group_id }: any) => {
    const repo = mockStorage.repositories.find((r: any) => r.id === id);
    if (repo) {
      repo.name = name;
      repo.local_path = path;
      repo.group_id = group_id;
      console.log('[Mock] Updated repository:', id);
    }
  },
  update_repository_group: async ({ id, group_id }: { id: string; group_id?: string }) => {
    const repo = mockStorage.repositories.find((r: any) => r.id === id);
    if (repo) {
      repo.group_id = group_id || null;
      console.log('[Mock] Moved repository:', id, 'to group:', group_id);
    }
  },
  delete_repository: async ({ id }: { id: string }) => {
    const index = mockStorage.repositories.findIndex((r: any) => r.id === id);
    if (index !== -1) {
      mockStorage.repositories.splice(index, 1);
      console.log('[Mock] Deleted repository:', id);
    }
  },
  list_repo_tree: async () => {
    return buildRepoTree();
  },
  
  // Route Groups
  create_route_group: async ({ name, parent_id }: { name: string; parent_id?: string }) => {
    const id = 'route-group-' + Date.now();
    mockStorage.routeGroups.push({ id, name, parent_id: parent_id || null, sort_order: 0 });
    console.log('[Mock] Created route group:', name);
  },
  delete_route_group: async ({ id }: { id: string }) => {
    const index = mockStorage.routeGroups.findIndex((g: any) => g.id === id);
    if (index !== -1) {
      mockStorage.routeGroups.splice(index, 1);
      console.log('[Mock] Deleted route group:', id);
    }
  },
  update_route_group_parent: async ({ id, parent_id }: { id: string; parent_id?: string }) => {
    const group = mockStorage.routeGroups.find((g: any) => g.id === id);
    if (group) {
      group.parent_id = parent_id || null;
      console.log('[Mock] Updated route group parent:', id, '->', parent_id);
    }
  },
  
  // Routes  
  create_route: async (data: any) => {
    const id = 'route-' + Date.now();
    mockStorage.routes.push({ id, ...data });
    console.log('[Mock] Created route:', data.name);
  },
  delete_route: async ({ id }: { id: string }) => {
    const index = mockStorage.routes.findIndex((r: any) => r.id === id);
    if (index !== -1) {
      mockStorage.routes.splice(index, 1);
      console.log('[Mock] Deleted route:', id);
    }
  },
  update_route_group_id: async ({ id, group_id }: { id: string; group_id?: string }) => {
    const route = mockStorage.routes.find((r: any) => r.id === id);
    if (route) {
      route.group_id = group_id || null;
      console.log('[Mock] Moved route:', id, 'to group:', group_id);
    }
  },
  list_route_tree: async () => {
    return buildRouteTree();
  },
  
  // Tasks
  list_tasks: async () => {
    return mockStorage.tasks;
  },
  create_task: async (data: any) => {
    const id = 'task-' + Date.now();
    mockStorage.tasks.push({ id, ...data });
    console.log('[Mock] Created task:', data.name);
  },
  delete_task: async ({ id }: { id: string }) => {
    const index = mockStorage.tasks.findIndex((t: any) => t.id === id);
    if (index !== -1) {
      mockStorage.tasks.splice(index, 1);
    }
  },
};

// Mock invoke function
export async function mockInvoke(cmd: string, args?: any): Promise<any> {
  console.log('[Mock Invoke]', cmd, args);
  
  const handler = mockCommands[cmd];
  if (handler) {
    return handler(args || {});
  }
  
  console.warn('[Mock] Unknown command:', cmd);
  return null;
}

// Create mock window APIs
export function setupBrowserMocks() {
  if (isTauri()) {
    console.log('[Tauri] Running in Tauri environment');
    return;
  }
  
  console.log('[Mock] Setting up browser mocks for development');
  
  // Mock window.__TAURI_INTERNALS__
  (window as any).__TAURI_INTERNALS__ = {
    invoke: mockInvoke,
    metadata: {
      currentWindow: { label: 'main' },
      currentWebview: { label: 'main' }
    }
  };
}
