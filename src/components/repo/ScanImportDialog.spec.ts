import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import ScanImportDialog from './ScanImportDialog.vue';


// Mock tauri dialog API
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock tauri event listener
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => vi.fn()), // listen returns an unlisten function
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
    template: '<div class="dialog-description"><slot /></div>', // Render description directly with a class
  },
  DialogFooter: {
    template: '<div><slot /></div>',
  },
}));

// Mock UI components
vi.mock('@/components/ui/button', () => ({
    Button: { template: '<button><slot /></button>' },
}));
vi.mock('@/components/ui/input', () => ({
    Input: { template: '<input />' },
}));
vi.mock('@/components/ui/label', () => ({
    Label: { template: '<label><slot /></label>' },
}));
vi.mock('@/components/ui/checkbox', () => ({
    Checkbox: { template: '<input type="checkbox" />' },
}));

// Mock lucide-vue-next icons
vi.mock('lucide-vue-next', () => ({
    Loader2: { template: '<span></span>' },
}));


// Mock i18n with a missing key
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      repo: {
        scan: {
          title: 'Scan & Import Repositories',
          desc: 'Select a folder to recursively scan for git repositories.',
        },
      },
      common: {
        browse: 'Browse',
        scan: 'Scan',
        scanning: 'Scanning...', 
        cancel: 'Cancel',
        import: 'Import Selected',
        importing: 'Importing...', 
        no_matches: 'No matches found',
      },
      scan: {
        root_path: 'Root Path',
        click_browse: 'Click Browse to select folder',
        grouping: {
            label: 'Grouping Strategy',
            path: 'By Path',
            git: 'By Git URL',
        },
        found: 'Found: {count}',
        selected: 'Selected: {count}',
        no_results: 'No git repositories found.',
        progress: 'Importing: {current} / {total}',
      }
    },
  },
});

describe('ScanImportDialog.vue', () => {
  it('should display the translated title and description', async () => {
    const wrapper = mount(ScanImportDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open();
    await wrapper.vm.$nextTick();

    expect(wrapper.find('h2').text()).toBe('Scan & Import Repositories');
    expect(wrapper.find('div.dialog-description').text()).toBe('Select a folder to recursively scan for git repositories.');
  });

  it('should display translated labels, placeholders and buttons for folder selection', async () => {
    const wrapper = mount(ScanImportDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open();
    await wrapper.vm.$nextTick();

    const labels = wrapper.findAll('label');
    expect(labels[0].text()).toBe('Root Path');
    expect(wrapper.find('input[placeholder="Click Browse to select folder"]').exists()).toBe(true);
    expect(wrapper.findAll('button')[0].text()).toBe('Browse');
    expect(wrapper.findAll('button')[1].text()).toBe('Scan');
  });

  it('should display translated grouping strategy options', async () => {
    const wrapper = mount(ScanImportDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open();
    await wrapper.vm.$nextTick();

    const labels = wrapper.findAll('label');
    expect(labels[1].text()).toBe('Grouping Strategy'); // Corrected index
    expect(labels[2].text()).toBe('By Path');
    expect(labels[3].text()).toBe('By Git URL');
  });

  it('should display translated scan results and import buttons', async () => {
    const wrapper = mount(ScanImportDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open();
    await wrapper.vm.$nextTick();

    // Mock scannedRepos length for 'Found:' text
    (wrapper.vm as any).scannedRepos = [{ name: 'test', relative_path: '/test', remote_url: null, checked: true }];
    await wrapper.vm.$nextTick();

    expect(wrapper.find('label[for="selectAll"]').text()).toContain('Found: 1');
    expect(wrapper.find('.text-muted-foreground').text()).toContain('Selected: 1'); // Selected count
    expect(wrapper.findAll('button')[2].text()).toBe('Cancel'); // Corrected index
    expect(wrapper.findAll('button')[3].text()).toBe('Import Selected'); // Corrected index
  });

  it('should display "No git repositories found." when no results', async () => {
    const wrapper = mount(ScanImportDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open();
    await wrapper.vm.$nextTick();

    (wrapper.vm as any).scannedRepos = [];
    (wrapper.vm as any).hasScanned = true; // Simulate scan finished with no results
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('No git repositories found.');
  });
});

