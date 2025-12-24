import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import AddRepoDialog from './AddRepoDialog.vue';


// Mock tauri dialog API
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      repo: {
        add: 'Add Repository',
        group: {
          new: 'New Group',
        },
        form: {
          name: { label: 'Name', placeholder: 'Repository Name' },
          path: { label: 'Path', placeholder: 'Local Path' },
        },
      },
      common: {
        browse: 'Browse',
        cancel: 'Cancel',
      },
      actions: {
        save: 'Save',
      },
    },
  },
});

describe('AddRepoDialog.vue', () => {
  it('should display the translated title for "Add Repository" mode', async () => {
    const wrapper = mount(AddRepoDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('repo'); // Open the dialog in 'repo' mode
    await wrapper.vm.$nextTick(); // Wait for DOM update

    expect(wrapper.find('h2').text()).toBe('Add Repository');
  });

  it('should display the translated title for "New Group" mode', async () => {
    const wrapper = mount(AddRepoDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('group'); // Open the dialog in 'group' mode
    await wrapper.vm.$nextTick(); // Wait for DOM update

    expect(wrapper.find('h2').text()).toBe('New Group');
  });

  it('should display translated labels and placeholders for form fields in "repo" mode', async () => {
    const wrapper = mount(AddRepoDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('repo');
    await wrapper.vm.$nextTick();

    const labels = wrapper.findAll('label');
    expect(labels.length).toBe(2);
    expect(labels[0].text()).toBe('Name');
    expect(labels[1].text()).toBe('Path');

    expect(wrapper.find('input[placeholder="Repository Name"]').exists()).toBe(true);
    expect(wrapper.find('input[placeholder="Local Path"]').exists()).toBe(true);
  });

  it('should display translated buttons', async () => {
    const wrapper = mount(AddRepoDialog, {
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('repo');
    await wrapper.vm.$nextTick();

    expect(wrapper.find('button').text()).toBe('Browse');
    expect(wrapper.findAll('button')[1].text()).toBe('Cancel');
    expect(wrapper.findAll('button')[2].text()).toBe('Save');
  });
});
