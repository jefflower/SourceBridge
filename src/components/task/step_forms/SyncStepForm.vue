<template>
    <div class="space-y-4">
        <div>
            <label class="block text-sm font-medium mb-1">{{ $t('task.steps.sync.route_id') }}</label>
            <RouteSelector
                v-model="model.route_id"
                :routes="routes"
                :placeholder="$t('task.steps.sync.select_route', '选择同步路线...')"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import RouteSelector from '@/components/route/RouteSelector.vue';

defineProps<{
    routes: any[];
}>();

const model = defineModel<{
    route_id: string | null;
}>({ required: true });
</script>
