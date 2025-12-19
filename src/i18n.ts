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
    },
    route: {
        group: {
            new: 'New Route Group',
            name: 'Group Name',
            parent: 'Parent Group',
        },
        add: 'Add Route',
        context: {
            new_subgroup: 'New Sub-Group',
            add_route: 'Add Route Here',
            rename: 'Rename',
            delete: 'Delete'
        },
        form: {
            name: {
                label: 'Name',
                placeholder: 'Route Name'
            },
            source: {
                label: 'Source Repo'
            },
            target: {
                label: 'Target Repo'
            },
            group: {
                label: 'Group'
            }
        },
        mapping: {
            source: 'Source Path (Glob)',
            target: 'Target Path',
            mode: 'Mode',
            actions: 'Actions',
            add: 'Add Rule',
            test: 'Test Match',
            test_placeholder: 'Test Path (e.g. src/main.ts)',
            modes: {
                copy: 'Copy',
                ignore: 'Ignore'
            }
        },
        test: {
            match: 'Matches Rule #{index}: {target}',
            no_match: 'No Match'
        },
        no_routes: 'Select a route to configure mappings.'
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
    },
    route: {
        group: {
            new: '新建同步分组',
            name: '分组名称',
            parent: '父级分组',
        },
        add: '添加同步路线',
        context: {
            new_subgroup: '新建子分组',
            add_route: '在此添加路线',
            rename: '重命名',
            delete: '删除'
        },
        form: {
            name: {
                label: '名称',
                placeholder: '路线名称'
            },
            source: {
                label: '源仓库'
            },
            target: {
                label: '目标仓库'
            },
            group: {
                label: '分组'
            }
        },
        mapping: {
            source: '源路径 (Glob)',
            target: '目标路径',
            mode: '模式',
            actions: '操作',
            add: '添加规则',
            test: '测试匹配',
            test_placeholder: '测试路径 (如 src/main.ts)',
            modes: {
                copy: '复制',
                ignore: '忽略'
            }
        },
        test: {
            match: '匹配规则 #{index}: {target}',
            no_match: '无匹配'
        },
        no_routes: '请选择一条路线配置映射规则。'
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
