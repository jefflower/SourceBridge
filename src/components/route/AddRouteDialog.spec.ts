import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import AddRouteDialog from './AddRouteDialog.vue';
import RepoSelector from '../repo/RepoSelector.vue'; // Mock this component

// Mock RepoSelector since its behavior is not under test here
vi.mock('../repo/RepoSelector.vue', () => ({
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
        add: 'Add Route',
        group: {
          new: 'New Route Group',
        },
        form: {
          name: { label: 'Name', placeholder: 'Route Name' },
          source: { label: 'Source Repo' },
          target: { label: 'Target Repo' },
        },
      },
      common: {
        cancel: 'Cancel',
      },
      actions: {
        save: 'Save',
      },
    },
  },
});

describe('AddRouteDialog.vue', () => {
  it('should display the translated title for "Add Route" mode', async () => {
    const wrapper = mount(AddRouteDialog, {
      props: {
        repos: [],
      },
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('route');
    await wrapper.vm.$nextTick();

    expect(wrapper.find('h2').text()).toBe('Add Route');
  });

  it('should display the translated title for "New Group" mode', async () => {
    const wrapper = mount(AddRouteDialog, {
      props: {
        repos: [],
      },
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('group');
    await wrapper.vm.$nextTick();

    const dialogTitle = wrapper.find('h2'); // find h2 directly since DialogTitle is mocked to render h2
    expect(dialogTitle.exists()).toBe(true);
    expect(dialogTitle.text()).toBe('New Route Group');
  });

  it('should display translated labels and placeholders for form fields in "route" mode', async () => {
    const wrapper = mount(AddRouteDialog, {
      props: {
        repos: [],
      },
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('route');
    await wrapper.vm.$nextTick();

    const labels = wrapper.findAll('label');
    expect(labels.length).toBe(3); // Name, Source Repo, Target Repo
    expect(labels[0].text()).toBe('Name');
    expect(labels[1].text()).toBe('Source Repo');
    expect(labels[2].text()).toBe('Target Repo');

    expect(wrapper.find('input[placeholder="Route Name"]').exists()).toBe(true);
  });

  it('should display translated buttons', async () => {
    const wrapper = mount(AddRouteDialog, {
      props: {
        repos: [],
      },
      global: {
        plugins: [i18n],
      },
    });

    wrapper.vm.open('route');
    await wrapper.vm.$nextTick();

    const buttons = wrapper.findAll('button');
    expect(buttons[0].text()).toBe('Cancel');
    expect(buttons[1].text()).toBe('Save');
  });
});
