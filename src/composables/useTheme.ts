import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export type Theme = 'light' | 'dark' | 'system';

const theme = ref<Theme>('system');
const isDark = ref(false);

// Get system preference
const getSystemTheme = (): 'light' | 'dark' => {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
};

// Apply theme to document
const applyTheme = (t: Theme) => {
  const effectiveTheme = t === 'system' ? getSystemTheme() : t;
  isDark.value = effectiveTheme === 'dark';
  
  if (effectiveTheme === 'dark') {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
};

// Initialize theme
export const initTheme = async () => {
  try {
    const savedTheme = await invoke<string | null>('get_setting', { key: 'theme' });
    if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
      theme.value = savedTheme as Theme;
    }
  } catch (e) {
    console.error('Failed to load theme setting:', e);
  }
  
  applyTheme(theme.value);
  
  // Listen for system theme changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (theme.value === 'system') {
      applyTheme('system');
    }
  });
};

// Set and save theme
export const setTheme = async (newTheme: Theme) => {
  theme.value = newTheme;
  applyTheme(newTheme);
  
  try {
    await invoke('set_setting', { key: 'theme', value: newTheme });
  } catch (e) {
    console.error('Failed to save theme setting:', e);
  }
};

export const useTheme = () => {
  return {
    theme,
    isDark,
    setTheme,
    initTheme,
  };
};
