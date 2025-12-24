import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import { createRouter, createWebHistory } from 'vue-router';
import SideBar from './SideBar.vue';

// Mock the router
const routes = [
  { path: '/', component: { template: '<div>Dashboard</div>' } },
  { path: '/repos', component: { template: '<div>Repos</div>' } },
  { path: '/routes', component: { template: '<div>Routes</div>' } },
  { path: '/tasks', component: { template: '<div>Tasks</div>' } },
  { path: '/ai-commands', component: { template: '<div>AI Commands</div>' } },
  { path: '/settings', component: { template: '<div>Settings</div>' } },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Mock i18n with complete translations
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      nav: {
        dashboard: 'Dashboard',
        repos: 'Repositories',
        routes: 'Routes',
        tasks: 'Tasks',
        ai_commands: 'AI Commands',
        settings: 'Settings',
      },
    },
  },
});

describe('SideBar.vue', () => {
  it('renders navigation links with correct titles from i18n', async () => {
    const wrapper = mount(SideBar, {
      global: {
        plugins: [router, i18n],
      },
    });

    await router.isReady();

    const links = wrapper.findAll('a');
    const menuItems = [
      { path: '/', label: 'Dashboard' },
      { path: '/repos', label: 'Repositories' },
      { path: '/routes', label: 'Routes' },
      { path: '/tasks', label: 'Tasks' },
      { path: '/ai-commands', label: 'AI Commands' },
      { path: '/settings', label: 'Settings' },
    ];

    expect(links.length).toBe(menuItems.length);

    links.forEach((link, index) => {
      expect(link.attributes('title')).toBe(menuItems[index].label);
    });
  });
});
