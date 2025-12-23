import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import Settings from './Settings.vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock tauri dialog API
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

// Mock useTheme composable
vi.mock('@/composables/useTheme', () => ({
  setTheme: vi.fn(),
}));

// Mock i18n - Defined once at the top level
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      settings: {
        title: 'Settings',
        appearance: {
          title: 'Appearance',
          language: 'Language',
          theme: 'Theme',
          language_options: {
            en: 'English',
            zh: 'Chinese',
          },
        },
        env: {
          title: 'Environment',
          git_path: 'Git Executable Path',
          select_git_executable_title: 'Select Git Executable',
        },
        system: {
          title: 'System',
          check: 'Startup Check',
        },
      },
      common: {
        browse: 'Browse',
      },
    },
  },
});

describe('Settings.vue', () => {
  // Mock initial settings loading for all tests
  beforeEach(() => {
    vi.mocked(invoke).mockImplementation((command) => {
      if (command === 'get_all_settings') {
        return Promise.resolve({ language: 'en', theme: 'system', git_path: 'git' });
      }
      return Promise.resolve();
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should display the translated settings title', async () => {
    const wrapper = mount(Settings, {
      global: {
        plugins: [i18n],
      },
    });

    await wrapper.vm.$nextTick(); // Wait for settings to load and component to re-render
    expect(wrapper.find('h1').text()).toBe('Settings');
  });

  it('should display translated language options', async () => {
    const wrapper = mount(Settings, {
      global: {
        plugins: [i18n],
      },
    });

        await wrapper.vm.$nextTick(); // Wait for settings to load and component to re-render

        console.log(wrapper.html());

        expect(wrapper.find('select').find('option[value="en"]').text()).toBe('English');
    expect(wrapper.find('select').find('option[value="zh"]').text()).toBe('Chinese');
  });

  it('should display translated browse button', async () => {
    const wrapper = mount(Settings, {
      global: {
        plugins: [i18n],
      },
    });

    await wrapper.vm.$nextTick(); // Wait for settings to load and component to re-render

    expect(wrapper.find('button').text()).toBe('Browse');
  });

  it('should display translated "Startup Check" button', async () => {
    const wrapper = mount(Settings, {
      global: {
        plugins: [i18n],
      },
    });

    await wrapper.vm.$nextTick(); // Wait for settings to load and component to re-render

    expect(wrapper.findAll('button')[1].text()).toBe('Startup Check');
  });

  it('should call tauri open with translated title when browsing git path', async () => {
    const wrapper = mount(Settings, {
      global: {
        plugins: [i18n],
      },
    });

    await wrapper.vm.$nextTick(); // Wait for settings to load and component to re-render

    await wrapper.find('button').trigger('click'); // Click the browse button

    expect(open).toHaveBeenCalledWith({
      directory: false,
      multiple: false,
      title: 'Select Git Executable',
    });
  });
});
