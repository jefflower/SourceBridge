import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import RepoSelector from './RepoSelector.vue';

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      repo: {
        select: 'Select a repository...',
        no_repos_in_dir: 'No repositories in this directory',
      },
      common: {
        no_matches: 'No matches found',
        filter: 'Filter by name or path...',
        root: 'Root',
      }
    },
  },
});

const repos = [
  { id: '1', name: 'Repo One', groupPath: 'Group A' },
  { id: '2', name: 'Repo Two', groupPath: 'Group A' },
  { id: '3', name: 'Repo Three', groupPath: 'Group B' },
];

describe('RepoSelector.vue', () => {
  it('should display the translated placeholder text', async () => {
    const wrapper = mount(RepoSelector, {
      props: {
        modelValue: null,
        repos: [], // No repos to show placeholder
      },
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('.text-muted-foreground').text()).toContain('Select a repository...');
  });

  it('should display "No matches found" when search yields no results', async () => {
    const wrapper = mount(RepoSelector, {
      props: {
        modelValue: null,
        repos: repos,
      },
      global: {
        plugins: [i18n],
      },
    });

    // Open dropdown and search
    await wrapper.find('.cursor-pointer').trigger('click');
    await wrapper.find('input[placeholder="Filter by name or path..."]').setValue('nonexistent');

    expect(wrapper.text()).toContain('No matches found');
  });

  it('should display "No repositories in this directory" when a folder is empty', async () => {
    const wrapper = mount(RepoSelector, {
      props: {
        modelValue: null,
        repos: [], // No repos in general
      },
      global: {
        plugins: [i18n],
      },
    });

    // Open dropdown
    await wrapper.find('.cursor-pointer').trigger('click');

    expect(wrapper.text()).toContain('No repositories in this directory');
  });

  it('should display "Root" as translated breadcrumb', async () => {
    const wrapper = mount(RepoSelector, {
      props: {
        modelValue: null,
        repos: repos,
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

