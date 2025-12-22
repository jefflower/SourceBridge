import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import DiffViewerModal from './DiffViewerModal.vue';

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock child components
vi.mock('./FileChangeTree.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('./MonacoDiffEditor.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));

// Mock UI components to render their slots directly
vi.mock('@/components/ui/dialog', () => ({
  Dialog: {
    template: '<div><slot /></div>',
  },
  DialogContent: {
    template: '<div><slot /></div>',
  },
  DialogHeader: {
    template: '<div><slot /></div>',
  },
  DialogTitle: {
    template: '<h2><slot /></h2>', // Render h2 directly
  },
  DialogDescription: {
    template: '<div class="dialog-description"><slot /></div>',
  },
  DialogFooter: {
    template: '<div><slot /></div>',
  },
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      route: {
        diff: {
          title: 'Sync Preview',
          exec_sync: 'Confirm & Sync',
          view: {
            inline: 'Inline View',
            split: 'Side-by-Side',
          },
          loading: 'Scanning files...',
          loading_content: 'Loading content...',
          select_file_to_view_diff: 'Select a file to view diff',
        },
      },
      window: {
        close: 'Close',
      },
    },
  },
});

describe('DiffViewerModal.vue', () => {
  it('should display the translated title and buttons', async () => {
    const wrapper = mount(DiffViewerModal, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open({ id: 'route-1', name: 'Test Route' });
    await wrapper.vm.$nextTick();

    expect(wrapper.find('h2').text()).toContain('Sync Preview: Test Route');
    expect(wrapper.find('button').text()).toBe('Inline View'); // Corrected assertion
    expect(wrapper.findAll('button')[1].text()).toBe('Confirm & Sync');
    expect(wrapper.findAll('button')[2].text()).toBe('Close');
  });

  it('should display "Loading content..." when content is loading', async () => {
    const wrapper = mount(DiffViewerModal, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open({ id: 'route-1', name: 'Test Route' });
    wrapper.vm.loadingContent = true; // Manually set loading state
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('Loading content...');
  });

  it('should display "Select a file to view diff" when no file is selected', async () => {
    const wrapper = mount(DiffViewerModal, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open({ id: 'route-1', name: 'Test Route' });
    wrapper.vm.selectedFile = null; // Ensure no file is selected
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('Select a file to view diff');
  });
});
