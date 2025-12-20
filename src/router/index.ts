import { createRouter, createWebHistory } from 'vue-router';
import MainLayout from '@/components/layout/MainLayout.vue';
import Dashboard from '@/views/Dashboard.vue';
import Repositories from '@/views/Repositories.vue';
import Routes from '@/views/Routes.vue';
import Tasks from '@/views/Tasks.vue';
import Settings from '@/views/Settings.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: MainLayout,
      children: [
        { path: '', component: Dashboard },
        { path: 'repos', component: Repositories },
        { path: 'routes', component: Routes },
        { path: 'tasks', component: Tasks },
        { path: 'settings', component: Settings },
      ],
    },
  ],
});

export default router;
