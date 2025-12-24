import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import Repositories from './Repositories.vue';
import { ask } from '@tauri-apps/plugin-dialog';

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(() => Promise.resolve([])), // Mock list_repo_tree to return empty array
}));

// Mock tauri ask dialog
vi.mock('@tauri-apps/plugin-dialog', () => ({
  ask: vi.fn(() => Promise.resolve(true)),
}));

// Mock child components
vi.mock('@/components/repo/RepoTree.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/repo/RepoDetail.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/repo/AddRepoDialog.vue', () => ({
  default: {
    template: '<div></div>',
    methods: {
      open: vi.fn(),
    },
  },
}));
vi.mock('@/components/repo/ScanImportDialog.vue', () => ({
  default: {
    template: '<div></div>',
    methods: {
      open: vi.fn(),
    },
  },
}));
vi.mock('@/components/common/ContextMenu.vue', () => ({
  default: {
    template: '<div><slot /></div>',
    props: ['items'],
    methods: {
      open: vi.fn(),
    },
  },
}));

// Mock lucide-vue-next icons
vi.mock('lucide-vue-next', async (importOriginal) => {
  const actual = await importOriginal();
  return {
    ...(actual as any),
    FolderPlus: { template: '<span></span>' },
    Plus: { template: '<span></span>' },
    Search: { template: '<span></span>' },
    PackageOpen: { template: '<span></span>' },
    Trash2: { template: '<span></span>' },
    FolderSearch: { template: '<span></span>' },
    X: { template: '<span></span>' },
    PackagePlus: { template: '<span></span>' },
  };
});


// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      nav: {
        repos: 'Repositories',
      },
      repo: {
        no_repos: 'Select a repository to view details or add a new one.',
        scan: {
            title: 'Scan Import'
        },
        group: {
            new: 'New Group'
        },
        add: 'Add Repository',
        context: {
          new_subgroup: 'New Sub-Group',
          add_repo: 'Add Repo Here',
          delete_group: 'Delete Group',
          delete_repo: 'Delete Repository',
        },
        delete_confirm: 'Are you sure you want to delete "{name}"?',
      },
      common: {
        filter: 'Filter...',
        no_matches: 'No matching results',
        delete_confirmation_title: 'Confirm Deletion',
        delete_failed: 'Deletion failed',
        move_failed: 'Move failed',
        error: 'Error',
      },
    },
  },
});

describe('Repositories.vue', () => {
  it('should display the translated title and action buttons', async () => {
    const wrapper = mount(Repositories, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('h2').text()).toBe('Repositories');
    expect(wrapper.find('button[title="Scan Import"]').exists()).toBe(true);
    expect(wrapper.find('button[title="New Group"]').exists()).toBe(true);
    expect(wrapper.find('button[title="Add Repository"]').exists()).toBe(true);
  });

  it('should display translated context menu items for a group node', async () => {
    const wrapper = mount(Repositories, {
      global: {
        plugins: [i18n],
      },
    });

    const mockGroupNode = { id: 'group-1', name: 'My Group', children: {}, repos: [] };
    
    // Simulate context menu open
    (wrapper.vm as any).onContextMenu(new MouseEvent('contextmenu'), mockGroupNode);
    await wrapper.vm.$nextTick();

    const contextMenu = wrapper.findComponent({ name: 'ContextMenu' });
    expect(contextMenu.props('items')[0].label).toBe('New Sub-Group');
    expect(contextMenu.props('items')[1].label).toBe('Add Repo Here');
    expect(contextMenu.props('items')[2].label).toBe('Delete Group');
  });

  it('should display translated context menu items for a repo node', async () => {
    const wrapper = mount(Repositories, {
      global: {
        plugins: [i18n],
      },
    });

    const mockRepoNode = { id: 'repo-1', name: 'My Repo' };
    
    // Simulate context menu open
    (wrapper.vm as any).onContextMenu(new MouseEvent('contextmenu'), mockRepoNode);
    await wrapper.vm.$nextTick();

    const contextMenu = wrapper.findComponent({ name: 'ContextMenu' });
    expect(contextMenu.props('items')[0].label).toBe('Delete Repository');
  });

  it('should call tauri ask with translated message and title when deleting a group', async () => {
    const wrapper = mount(Repositories, {
      global: {
        plugins: [i18n],
      },
    });

    const mockGroupNode = { id: 'group-1', name: 'My Group', children: {}, repos: [] };
    
    (wrapper.vm as any).onContextMenu(new MouseEvent('contextmenu'), mockGroupNode);
    await wrapper.vm.$nextTick();

    const contextMenu = wrapper.findComponent({ name: 'ContextMenu' });
    await contextMenu.vm.$emit('select', 'delete');

    expect(ask).toHaveBeenCalledWith('Are you sure you want to delete "My Group"?', {
      title: 'Confirm Deletion',
      kind: 'warning',
    });
  });

  it('should call tauri ask with translated message and title when deleting a repo', async () => {
    const wrapper = mount(Repositories, {
      global: {
        plugins: [i18n],
      },
    });

    const mockRepoNode = { id: 'repo-1', name: 'My Repo' };
    
    // Simulate context menu open
    (wrapper.vm as any).onContextMenu(new MouseEvent('contextmenu'), mockRepoNode);
    await wrapper.vm.$nextTick();

    const contextMenu = wrapper.findComponent({ name: 'ContextMenu' });
    await contextMenu.vm.$emit('select', 'delete');

    expect(ask).toHaveBeenCalledWith('Are you sure you want to delete "My Repo"?', {
      title: 'Confirm Deletion',
      kind: 'warning',
    });
  });
});
