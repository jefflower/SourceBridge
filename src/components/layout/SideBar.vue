<template>
  <aside class="w-16 flex flex-col items-center py-4 bg-muted/40 border-r h-full pt-12">
    <nav class="flex flex-col gap-4 w-full">
      <router-link
        v-for="item in menuItems"
        :key="item.path"
        :to="item.path"
        class="w-10 h-10 flex items-center justify-center rounded-md hover:bg-muted transition-colors mx-auto"
        :class="{ 'bg-primary text-primary-foreground hover:bg-primary/90': isActive(item.path) }"
        :title="$t(item.label)"
      >
        <component :is="item.icon" class="w-5 h-5" />
      </router-link>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { Home, Package, Waypoints, Zap, Settings } from 'lucide-vue-next';

const route = useRoute();

const menuItems = [
  { path: '/', icon: Home, label: 'nav.dashboard' },
  { path: '/repos', icon: Package, label: 'nav.repos' },
  { path: '/routes', icon: Waypoints, label: 'nav.routes' },
  { path: '/tasks', icon: Zap, label: 'nav.tasks' },
  { path: '/settings', icon: Settings, label: 'nav.settings' },
];

const isActive = (path: string) => {
    if (path === '/' && route.path === '/') return true;
    if (path !== '/' && route.path.startsWith(path)) return true;
    return false;
}
</script>
