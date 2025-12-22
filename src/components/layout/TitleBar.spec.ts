import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import TitleBar from './TitleBar.vue';

// Mock the tauri window API
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    startDragging: vi.fn(),
    minimize: vi.fn(),
    toggleMaximize: vi.fn(),
    close: vi.fn(),
  })),
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      app: {
        name: 'SourceBridge',
      },
      window: {
        minimize: 'Minimize',
        maximize: 'Maximize',
        close: 'Close',
      },
    },
  },
});

describe('TitleBar.vue', () => {
  it('should display the translated application title', () => {
    const wrapper = mount(TitleBar, {
      global: {
        plugins: [i18n],
      },
    });
    expect(wrapper.find('span.font-bold').text()).toBe('SourceBridge');
  });

  it('should display translated titles for window control buttons', () => {
    const wrapper = mount(TitleBar, {
      global: {
        plugins: [i18n],
      },
    });

    const minimizeButton = wrapper.find('button[title="Minimize"]');
    const maximizeButton = wrapper.find('button[title="Maximize"]');
    const closeButton = wrapper.find('button[title="Close"]');

    expect(minimizeButton.exists()).toBe(true);
    expect(maximizeButton.exists()).toBe(true);
    expect(closeButton.exists()).toBe(true);
  });
});
