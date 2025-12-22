import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import RouteSelector from './RouteSelector.vue';

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      task: {
        steps: {
          sync: {
            select_route: 'Select a route...',
            no_routes: 'No routes in this directory',
          },
        },
      },
      common: {
        filter: 'Filter by name or group...',
        no_matches: 'No matches found',
        root: 'Root',
      },
    },
  },
});

const routes = [
  { id: '1', name: 'Route One', groupPath: 'Group A' },
  { id: '2', name: 'Route Two', groupPath: 'Group A' },
];

describe('RouteSelector.vue', () => {
  it('should display the translated placeholder text', async () => {
    const wrapper = mount(RouteSelector, {
      props: {
        modelValue: null,
        routes: [], // No routes to show placeholder
      },
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('.text-muted-foreground').text()).toContain('Select a route...');
  });

  it('should display "No matches found" when search yields no results', async () => {
    const wrapper = mount(RouteSelector, {
      props: {
        modelValue: null,
        routes: routes,
      },
      global: {
        plugins: [i18n],
      },
    });

    // Open dropdown and search
    await wrapper.find('.cursor-pointer').trigger('click');
    await wrapper.find('input[placeholder="Filter by name or group..."]').setValue('nonexistent');

    expect(wrapper.text()).toContain('No matches found');
  });

  it('should display "No routes in this directory" when a folder is empty', async () => {
    const wrapper = mount(RouteSelector, {
      props: {
        modelValue: null,
        routes: [], // No routes in general
      },
      global: {
        plugins: [i18n],
      },
    });

    // Open dropdown
    await wrapper.find('.cursor-pointer').trigger('click');

    expect(wrapper.text()).toContain('No routes in this directory');
  });

  it('should display "Root" as translated breadcrumb', async () => {
    const wrapper = mount(RouteSelector, {
      props: {
        modelValue: null,
        routes: routes,
      },
      global: {
        plugins: [i18n],
      },
    });

    // Open dropdown
    await wrapper.find('.cursor-pointer').trigger('click');

    expect(wrapper.find('.px-2.py-1.border-b button').text()).toBe('Root');
  });
});
