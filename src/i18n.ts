import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import zh from './locales/zh-CN.json';

const i18n = createI18n({
  legacy: false, // Vue 3 Composition API
  locale: 'en', // default locale
  fallbackLocale: 'en',
  messages: {
    en,
    zh, // Simplified Chinese
    'zh-CN': zh
  },
});

export default i18n;
