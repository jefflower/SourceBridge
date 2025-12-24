import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import TaskBuilder from './TaskBuilder.vue';
import { createI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';

// Mock invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(() => Promise.resolve([])),
}));

// Mock icons
vi.mock('lucide-vue-next', () => ({
  Trash2: { template: '<span></span>' },
  Terminal: { template: '<span></span>' },
  GitBranch: { template: '<span></span>' },
  Waypoints: { template: '<span></span>' },
  Sparkles: { template: '<span></span>' },
}));

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
        task: {
            form: { name: { placeholder: 'Name' }, schedule: { enable: 'Enable' } },
            actions: { close: 'Close' },
            step_types: { script: 'Script', git: 'Git', sync: 'Sync', AI_PROMPT: 'AI Prompt' },
            steps_title: 'Steps',
            steps: { ai: {
                system_prompt: 'System',
                system_prompt_placeholder: 'Sys Place',
                user_prompt: 'User',
                user_prompt_placeholder: 'User Place'
            } }
        },
        settings: { title: 'Settings' },
        actions: { save: 'Save' }
    }
  }
});

describe('TaskBuilder.vue', () => {
  it('renders correctly', () => {
    const wrapper = mount(TaskBuilder, {
      props: { task: null },
      global: { plugins: [i18n] }
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('adds an AI Prompt step when button clicked', async () => {
    const wrapper = mount(TaskBuilder, {
      props: { task: null },
      global: { plugins: [i18n] }
    });

    const aiBtn = wrapper.findAll('button').find(b => b.text().includes('AI Prompt'));
    expect(aiBtn).toBeDefined();
    await aiBtn?.trigger('click');

    const steps = wrapper.findAll('.group');
    expect(steps.length).toBe(1);
    expect(steps[0].text()).toContain('AI Prompt');

    // Check if form rendered
    expect(wrapper.find('textarea').exists()).toBe(true); // Should find AIStepForm textareas
  });
});
