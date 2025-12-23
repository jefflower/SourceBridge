import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import Dashboard from './Dashboard.vue';

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      nav: {
        dashboard: 'Dashboard',
      },
      dashboard: {
        welcome: 'Welcome to SourceBridge.',
      },
    },
  },
});

describe('Dashboard.vue', () => {
  it('should display the translated dashboard title and welcome message', async () => {
    const wrapper = mount(Dashboard, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('h1').text()).toBe('Dashboard');
    expect(wrapper.find('p').text()).toBe('Welcome to SourceBridge.');
  });
});
