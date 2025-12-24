import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import RouteDetail from './RouteDetail.vue';
import { invoke } from '@tauri-apps/api/core';


// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock tauri dialog API
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
  ask: vi.fn(() => Promise.resolve(true)),
}));

// Mock lucide-vue-next icons
vi.mock('lucide-vue-next', () => ({
    Waypoints: { template: '<span></span>' },
    Trash2: { template: '<span></span>' },
    Eye: { template: '<span></span>' },
}));

// Mock child components
vi.mock('../repo/RepoSelector.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('../diff/DiffViewerModal.vue', () => ({
  default: {
    template: '<div></div>',
    methods: {
      open: vi.fn(),
    },
  },
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      route: {
        tabs: {
          info: 'Info',
          mappings: 'Mappings',
        },
        form: {
          name: { label: 'Name', placeholder: 'Route Name' },
          source: { label: 'Source Repo' },
          target: { label: 'Target Repo' },
        },
        mapping: {
          add: 'Add Rule',
          source: 'Source Path (Glob)',
          target: 'Target Path (Glob)',
          mode: 'Mode',
          actions: 'Actions',
          placeholder: {
            source: 'src/**/*.ts',
            target: 'src/**/*.ts (empty = same)',
          },
          modes: {
            copy: 'Copy',
            ignore: 'Ignore',
          },
        },
        preview: {
          title: 'Rule Preview',
          desc: 'Enter a Glob pattern to scan the repository and preview matched files.',
          placeholder: 'e.g. src/**/*.ts',
          scan: 'Scan',
          found: 'Found {count} files',
          truncated: 'Showing first 200 only',
          no_matches: 'No files matched',
          add_as_rule: 'Add as Rule',
        },
        diff: {
          preview: 'Preview Diff', // Added for the test
        },
      },
      repo: {
        context: {
          delete: 'Delete',
        },
      },
      actions: {
        save: 'Save',
      },
      common: {
        cancel: 'Cancel',
        error: 'Error',
        move_failed: 'Move failed',
        delete_confirmation_title: 'Confirm Deletion',
        delete_failed: 'Deletion failed',
      },
    },
  },
});

describe('RouteDetail.vue', () => {
  const mockRoute = {
    id: '1',
    name: 'test-route',
    description: 'Test Description',
    groupPath: 'group/path',
    source_id: 'repo-1',
    target_id: 'repo-2',
  };

  const mockRepos = [
    { id: 'repo-1', name: 'Repo One' },
    { id: 'repo-2', name: 'Repo Two' },
  ];

  beforeEach(() => {
    vi.mocked(invoke).mockImplementation((command) => {
      if (command === 'get_route_details') {
        return Promise.resolve({ mappings: '[]' });
      }
      return Promise.resolve();
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should display the translated "Preview Diff" button', async () => {
    const wrapper = mount(RouteDetail, {
      props: {
        route: mockRoute,
        repos: mockRepos,
      },
      global: {
        plugins: [i18n],
      },
    });

    await wrapper.vm.$nextTick();
    await wrapper.findAll('.flex.border-b button')[1].trigger('click'); // Switch to Mappings tab
    await wrapper.vm.$nextTick();

    const previewButton = wrapper.find('[data-testid="preview-diff-button"]');
    expect(previewButton.text()).toContain('Preview Diff');
  });
});
