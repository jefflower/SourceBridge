import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import Routes from './Routes.vue';
import { invoke } from '@tauri-apps/api/core';
import { ask } from '@tauri-apps/plugin-dialog';

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(() => Promise.resolve([])),
}));

// Mock tauri ask dialog
vi.mock('@tauri-apps/plugin-dialog', () => ({
  ask: vi.fn(() => Promise.resolve(true)),
}));

// Mock child components
vi.mock('@/components/route/RouteTree.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/route/RouteDetail.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/route/AddRouteDialog.vue', () => ({
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
    ...actual,
    FolderPlus: { template: '<span></span>' },
    Plus: { template: '<span></span>' },
    Waypoints: { template: '<span></span>' },
  };
});


// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      nav: {
        routes: 'Routes',
      },
      route: {
        no_routes: 'Select a route to configure mappings.',
        group: {
            new: 'New Route Group'
        },
        add: 'Add Route',
      },
      common: {
        filter: 'Filter...',
      },
    },
  },
});

describe('Routes.vue', () => {
  it('should display the translated title and action buttons', async () => {
    const wrapper = mount(Routes, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('h2').text()).toBe('Routes');
    expect(wrapper.find('button[title="New Route Group"]').exists()).toBe(true);
    expect(wrapper.find('button[title="Add Route"]').exists()).toBe(true);
  });
});
