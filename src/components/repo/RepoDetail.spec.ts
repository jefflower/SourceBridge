import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import RepoDetail from './RepoDetail.vue';
import { invoke } from '@tauri-apps/api/core';

// Mock tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock tauri event API
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

// Mock tauri dialog API
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

// Mock lucide-vue-next icons
vi.mock('lucide-vue-next', () => ({
    Package: { template: '<span></span>' },
    Loader2: { template: '<span></span>' },
    FolderOpen: { template: '<span></span>' },
    Terminal: { template: '<span></span>' },
    Code: { template: '<span></span>' },
    Sparkles: { template: '<span></span>' },
    ArrowDownToLine: { template: '<span></span>' },
    X: { template: '<span></span>' },
}));

// Mock i18n
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {
    en: {
      repo: {
        branch: {
          label: 'Branch',
          switch_success: 'Branch switched successfully',
          switch_failed: 'Failed to switch branch',
        },
        history: {
          no_log: 'No commit history found',
        },
        tabs: {
            overview: 'Overview',
            history: 'History',
            settings: 'Settings',
        },
        form: {
          name: { label: 'Name' },
          path: { label: 'Path' },
        },
        actions: {
          open_folder: 'Open Folder',
          open_terminal: 'Open Terminal',
          open_ide: 'Open in IDE',
        },
        context: {
          delete: 'Delete',
        },
      },
      actions: {
        save: 'Save',
      },
      common: {
        error: 'Error',
        browse: 'Browse',
      },
    },
  },
});

describe('RepoDetail.vue', () => {
  const mockRepo = {
    id: '1',
    name: 'test-repo',
    path: '/path/to/test-repo',
    groupPath: 'group/path',
  };

  beforeEach(() => {
    vi.mocked(invoke).mockImplementation((command) => {
      if (command === 'get_git_branches') {
        return Promise.resolve([{ name: 'main', is_current: true }, { name: 'dev', is_current: false }]);
      }
      if (command === 'get_git_log') {
        return Promise.resolve([]);
      }
      return Promise.resolve();
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should display the translated tab labels', async () => {
    const wrapper = mount(RepoDetail, {
      props: {
        repo: mockRepo,
      },
      global: {
        plugins: [i18n],
      },
    });

    // The component calls loadBranches and loadHistory onMounted, which are async.
    // We need to wait for them to resolve and for the DOM to update.
    await vi.waitFor(() => expect(invoke).toHaveBeenCalledWith('get_git_branches', { path: mockRepo.path }));
    await wrapper.vm.$nextTick(); // Wait for component to re-render after data load
    await new Promise(resolve => setTimeout(resolve, 100)); // Small delay to ensure DOM updates are flushed
    
    expect(wrapper.find('.flex.border-b.px-6 button:nth-child(1)').html()).toContain('Overview');
    expect(wrapper.find('.flex.border-b.px-6 button:nth-child(2)').html()).toContain('History');
    expect(wrapper.find('.flex.border-b.px-6 button:nth-child(3)').html()).toContain('Settings');
  });

  it('should call open_in_ide when Code button is clicked', async () => {
    const wrapper = mount(RepoDetail, {
      props: {
        repo: mockRepo,
      },
      global: {
        plugins: [i18n],
      },
    });

    // Set setting
    vi.mocked(invoke).mockResolvedValue('code');

    await wrapper.vm.$nextTick();
    await new Promise(resolve => setTimeout(resolve, 50));

    // Find button with Code icon. Since we mocked the icon to be a span, we can find it by looking for the button that contains it?
    // Actually the button has a title.
    const ideBtn = wrapper.findAll('button').find(b => b.attributes('title') === 'Open in IDE');
    expect(ideBtn).toBeDefined();
    await ideBtn?.trigger('click');
    expect(invoke).toHaveBeenCalledWith('open_in_ide', { path: mockRepo.path, ide_command: 'code' });
  });

  it('should call run_shell_command when AI Commit button is clicked', async () => {
    const wrapper = mount(RepoDetail, {
      props: {
        repo: mockRepo,
      },
      global: {
        plugins: [i18n],
      },
    });

    // Mock prompt
    vi.spyOn(window, 'prompt').mockReturnValue('Fix stuff');

    await wrapper.vm.$nextTick();
    const commitBtn = wrapper.findAll('button').find(b => b.text().includes('AI Commit'));
    expect(commitBtn).toBeDefined();

    await commitBtn?.trigger('click');
    expect(invoke).toHaveBeenCalledWith('run_shell_command', expect.objectContaining({
        command: 'gemini',
        args: ['/commit', 'Fix stuff', '--yes'],
        cwd: mockRepo.path
    }));
  });
});
