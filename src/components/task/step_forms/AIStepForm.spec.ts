import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import AIStepForm from './AIStepForm.vue';
import { createI18n } from 'vue-i18n';

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      task: {
        steps: {
          ai: {
            system_prompt: 'System Prompt',
            system_prompt_placeholder: 'Optional system instructions...',
            user_prompt: 'User Prompt',
            user_prompt_placeholder: 'Enter your prompt here...'
          }
        }
      }
    }
  }
});

describe('AIStepForm.vue', () => {
  it('renders input fields correctly', () => {
    const wrapper = mount(AIStepForm, {
      props: {
        modelValue: {
          system_prompt: '',
          user_prompt: ''
        }
      },
      global: {
        plugins: [i18n]
      }
    });

    expect(wrapper.text()).toContain('System Prompt');
    expect(wrapper.text()).toContain('User Prompt');
    expect(wrapper.findAll('textarea').length).toBe(2);
  });

  it('updates modelValue on input', async () => {
    const modelValue = {
      system_prompt: '',
      user_prompt: ''
    };

    const wrapper = mount(AIStepForm, {
      props: {
        modelValue
      },
      global: {
        plugins: [i18n]
      }
    });

    const textareas = wrapper.findAll('textarea');
    await textareas[0].setValue('You are a bot');
    await textareas[1].setValue('Hello');

    // Since v-model mutates the object prop in Vue 3 if it's reactive,
    // but here we are passing a plain object.
    // Wait, v-model on component emits update:modelValue.
    // However, v-model on native input inside the component with `modelValue.prop`
    // will mutate the prop directly if it's an object, which works in Vue but is technically prop mutation.
    // Let's verify if the component implementation handles this.
    // The component uses `v-model="modelValue.system_prompt"`. This mutates the prop object.

    // In Vue 3, mutating a nested property of a prop object is possible but often discouraged.
    // However, for form bindings it's a common pattern to avoid verbose emits.
    // Let's check if the wrapper.props().modelValue is updated.

    expect(wrapper.props().modelValue.system_prompt).toBe('You are a bot');
    expect(wrapper.props().modelValue.user_prompt).toBe('Hello');
  });
});
