import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    nav: {
      dashboard: 'Dashboard',
      repos: 'Repositories',
      routes: 'Routes',
      tasks: 'Tasks',
      settings: 'Settings',
    },
    window: {
      minimize: 'Minimize',
      maximize: 'Maximize',
      close: 'Close',
    },
    settings: {
        title: 'Settings',
        appearance: {
            title: 'Appearance',
            theme: 'Theme',
            language: 'Language',
        },
        env: {
            title: 'Environment',
            git_path: 'Git Executable Path',
            ssh_path: 'SSH Key Default Path',
        },
        system: {
            title: 'System',
            startup: 'Run on Startup',
            check: 'Startup Check',
        },
        actions: {
            save: 'Save',
            check_now: 'Check Now'
        }
    },
    repo: {
        group: {
            new: 'New Group',
            name: 'Group Name',
            parent: 'Parent Group',
        },
        add: 'Add Repository',
        context: {
            open_terminal: 'Open in Terminal',
            edit: 'Edit',
            delete: 'Delete',
            new_subgroup: 'New Sub-Group',
            add_repo: 'Add Repo Here',
            rename: 'Rename',
        },
        form: {
            name: {
                label: 'Name',
                placeholder: 'Repository Name'
            },
            path: {
                label: 'Path',
                placeholder: 'Local Path'
            },
            group: {
                label: 'Group'
            }
        },
        no_repos: 'Select a repository to view details or add a new one.'
    }
  },
  zh: {
    nav: {
      dashboard: '仪表盘',
      repos: '仓库管理',
      routes: '同步路线',
      tasks: '任务编排',
      settings: '全局设置',
    },
    window: {
      minimize: '最小化',
      maximize: '最大化',
      close: '关闭',
    },
    settings: {
        title: '设置',
        appearance: {
            title: '外观',
            theme: '主题',
            language: '语言',
        },
        env: {
            title: '环境',
            git_path: 'Git 可执行路径',
            ssh_path: 'SSH Key 默认路径',
        },
        system: {
            title: '系统',
            startup: '开机自启',
            check: '启动检查',
        },
        actions: {
            save: '保存',
            check_now: '立即检查'
        }
    },
    repo: {
        group: {
            new: '新建分组',
            name: '分组名称',
            parent: '父级分组',
        },
        add: '添加仓库',
        context: {
            open_terminal: '在终端打开',
            edit: '编辑',
            delete: '删除',
            new_subgroup: '新建子分组',
            add_repo: '在此添加仓库',
            rename: '重命名',
        },
        form: {
            name: {
                label: '名称',
                placeholder: '仓库名称'
            },
            path: {
                label: '路径',
                placeholder: '本地路径'
            },
            group: {
                label: '分组'
            }
        },
        no_repos: '请选择一个仓库查看详情，或添加新仓库。'
    }
  },
};

const i18n = createI18n({
  legacy: false, // Vue 3 Composition API
  locale: 'en', // default locale
  fallbackLocale: 'en',
  messages,
});

export default i18n;
