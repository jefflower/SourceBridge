import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";
import { setupBrowserMocks } from "./utils/tauriMock";

// Setup mocks for browser development (no-op if running in Tauri)
setupBrowserMocks();

createApp(App).use(router).use(i18n).mount("#app");
