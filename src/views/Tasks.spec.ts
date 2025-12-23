import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import Tasks from './Tasks.vue';
import { invoke } from '@tauri-apps/api/core'; // Added this import
import { ask } from '@tauri-apps/plugin-dialog'; // Added this import for clarity

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn((command) => {
    if (command === 'list_tasks') {
      return Promise.resolve([]); // Mock list_tasks to return an empty array
    }
    return Promise.resolve();
  }),
}));

// Mock child components
vi.mock('@/components/task/TaskCard.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/task/TaskBuilder.vue', () => ({
  default: {
    template: '<div></div>',
  },
}));
vi.mock('@/components/log/LogConsole.vue', () => ({
  default: {
    template: '<div></div>',
    methods: {
      open: vi.fn(),
    },
  },
}));

// Mock tauri ask dialog
vi.mock('@tauri-apps/plugin-dialog', () => ({
  ask: vi.fn(() => Promise.resolve(true)),
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      nav: {
        tasks: 'Tasks',
      },
      task: {
        new: 'New Task',
        empty: 'No tasks found. Create one to get started.',
        confirm_delete: 'Are you sure you want to delete task "{name}"?',
        delete_title: 'Confirm Delete',
      },
      common: {
        error: 'Error',
      },
    },
  },
});

describe('Tasks.vue', () => {
  it('should display the translated main title', async () => {
    const wrapper = mount(Tasks, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('h1').text()).toBe('Tasks');
  });

  it('should display "No tasks found." message when there are no tasks', async () => {
    const wrapper = mount(Tasks, {
      global: {
        plugins: [i18n],
      },
    });

    // loadTasks is called on mounted, and invoke('list_tasks') is mocked to return []
    await wrapper.vm.$nextTick(); 

    expect(wrapper.text()).toContain('No tasks found. Create one to get started.');
  });

  it('should display "New Task" button', async () => {
    const wrapper = mount(Tasks, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.find('button').text()).toBe('+ New Task'); // Corrected assertion
  });

  it('should call invoke("delete_task") when a task is deleted and confirmed', async () => {
    // Need to reset the mock for invoke and ask for this specific test
    vi.clearAllMocks(); // Clear previous mocks
    vi.mocked(invoke).mockImplementation((command) => { // Using vi.mocked here
      if (command === 'list_tasks') {
        return Promise.resolve([mockTask]);
      }
      if (command === 'delete_task') {
        return Promise.resolve();
      }
      return Promise.resolve();
    });
    vi.mocked(ask).mockResolvedValue(true); // User confirms deletion

    const mockTask = { id: 'task-1', name: 'Test Task' };
    
    const wrapper = mount(Tasks, {
      global: {
        plugins: [i18n],
      },
    });

    // Simulate task deletion
    await wrapper.vm.deleteTask(mockTask);

    expect(ask).toHaveBeenCalledWith('Are you sure you want to delete task "Test Task"?', {
      title: 'Confirm Delete',
      kind: 'warning',
    });
    expect(invoke).toHaveBeenCalledWith('delete_task', { id: 'task-1' });
  });
});
