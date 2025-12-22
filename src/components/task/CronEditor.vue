<script setup lang="ts">
import { ref, computed, watch } from 'vue';

const props = defineProps<{
    modelValue: string;
}>();

const emit = defineEmits(['update:modelValue']);

// Cron fields
const mode = ref<'simple' | 'advanced'>('simple');
const frequency = ref<'every_minute' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'custom'>('daily');

// Simple mode settings
const minute = ref(0);
const hour = ref(0);
const dayOfMonth = ref(1);
const dayOfWeek = ref<number[]>([1]); // Monday

// Parse incoming value
const parseCron = (cron: string) => {
    if (!cron) {
        frequency.value = 'daily';
        minute.value = 0;
        hour.value = 0;
        return;
    }
    
    const parts = cron.split(' ');
    if (parts.length !== 5) {
        mode.value = 'advanced';
        return;
    }
    
    const [min, hr, dom, mon, dow] = parts;
    
    // Every minute
    if (min === '*' && hr === '*' && dom === '*' && mon === '*' && dow === '*') {
        frequency.value = 'every_minute';
        return;
    }
    
    // Hourly (at specific minute)
    if (min !== '*' && hr === '*' && dom === '*' && mon === '*' && dow === '*') {
        frequency.value = 'hourly';
        minute.value = parseInt(min) || 0;
        return;
    }
    
    // Daily (at specific time)
    if (min !== '*' && hr !== '*' && dom === '*' && mon === '*' && dow === '*') {
        frequency.value = 'daily';
        minute.value = parseInt(min) || 0;
        hour.value = parseInt(hr) || 0;
        return;
    }
    
    // Weekly
    if (min !== '*' && hr !== '*' && dom === '*' && mon === '*' && dow !== '*') {
        frequency.value = 'weekly';
        minute.value = parseInt(min) || 0;
        hour.value = parseInt(hr) || 0;
        dayOfWeek.value = dow.split(',').map(d => parseInt(d)).filter(d => !isNaN(d));
        return;
    }
    
    // Monthly
    if (min !== '*' && hr !== '*' && dom !== '*' && mon === '*' && dow === '*') {
        frequency.value = 'monthly';
        minute.value = parseInt(min) || 0;
        hour.value = parseInt(hr) || 0;
        dayOfMonth.value = parseInt(dom) || 1;
        return;
    }
    
    // Custom - switch to advanced mode
    mode.value = 'advanced';
};

// Generate cron expression from settings
const generatedCron = computed(() => {
    switch (frequency.value) {
        case 'every_minute':
            return '* * * * *';
        case 'hourly':
            return `${minute.value} * * * *`;
        case 'daily':
            return `${minute.value} ${hour.value} * * *`;
        case 'weekly':
            return `${minute.value} ${hour.value} * * ${dayOfWeek.value.join(',')}`;
        case 'monthly':
            return `${minute.value} ${hour.value} ${dayOfMonth.value} * *`;
        default:
            return props.modelValue || '0 0 * * *';
    }
});

// Watch for external changes
watch(() => props.modelValue, (newVal) => {
    if (newVal !== generatedCron.value) {
        parseCron(newVal);
    }
}, { immediate: true });

// Emit on change
watch([frequency, minute, hour, dayOfMonth, dayOfWeek], () => {
    if (mode.value === 'simple') {
        emit('update:modelValue', generatedCron.value);
    }
}, { deep: true });

const advancedValue = ref(props.modelValue || '');
watch(() => props.modelValue, (v) => { advancedValue.value = v || ''; });

const updateAdvanced = () => {
    emit('update:modelValue', advancedValue.value);
};

const toggleDayOfWeek = (day: number) => {
    const idx = dayOfWeek.value.indexOf(day);
    if (idx >= 0) {
        if (dayOfWeek.value.length > 1) {
            dayOfWeek.value.splice(idx, 1);
        }
    } else {
        dayOfWeek.value.push(day);
    }
};

const weekDays = [
    { value: 0, label: 'cron.days.sun' },
    { value: 1, label: 'cron.days.mon' },
    { value: 2, label: 'cron.days.tue' },
    { value: 3, label: 'cron.days.wed' },
    { value: 4, label: 'cron.days.thu' },
    { value: 5, label: 'cron.days.fri' },
    { value: 6, label: 'cron.days.sat' },
];

const hours = Array.from({ length: 24 }, (_, i) => i);
const minutes = [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55];
const days = Array.from({ length: 31 }, (_, i) => i + 1);
</script>

<template>
    <div class="cron-editor space-y-3">
        <!-- Mode Toggle -->
        <div class="flex gap-2 text-xs">
            <button 
                @click="mode = 'simple'" 
                class="px-2 py-1 rounded border transition-colors"
                :class="mode === 'simple' ? 'bg-primary text-primary-foreground' : 'bg-background hover:bg-muted'"
            >
                {{ $t('cron.mode.simple') }}
            </button>
            <button 
                @click="mode = 'advanced'" 
                class="px-2 py-1 rounded border transition-colors"
                :class="mode === 'advanced' ? 'bg-primary text-primary-foreground' : 'bg-background hover:bg-muted'"
            >
                {{ $t('cron.mode.advanced') }}
            </button>
        </div>

        <!-- Simple Mode -->
        <template v-if="mode === 'simple'">
            <!-- Frequency -->
            <div>
                <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.frequency') }}</label>
                <select v-model="frequency" class="w-full h-9 rounded-md border border-input bg-background px-3 text-sm">
                    <option value="every_minute">{{ $t('cron.freq.every_minute') }}</option>
                    <option value="hourly">{{ $t('cron.freq.hourly') }}</option>
                    <option value="daily">{{ $t('cron.freq.daily') }}</option>
                    <option value="weekly">{{ $t('cron.freq.weekly') }}</option>
                    <option value="monthly">{{ $t('cron.freq.monthly') }}</option>
                </select>
            </div>

            <!-- Time (for hourly/daily/weekly/monthly) -->
            <div v-if="frequency !== 'every_minute'" class="flex gap-3">
                <!-- Hour (not for hourly) -->
                <div v-if="frequency !== 'hourly'" class="flex-1">
                    <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.hour') }}</label>
                    <select v-model="hour" class="w-full h-9 rounded-md border border-input bg-background px-3 text-sm">
                        <option v-for="h in hours" :key="h" :value="h">{{ String(h).padStart(2, '0') }}:00</option>
                    </select>
                </div>
                <!-- Minute -->
                <div class="flex-1">
                    <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.minute') }}</label>
                    <select v-model="minute" class="w-full h-9 rounded-md border border-input bg-background px-3 text-sm">
                        <option v-for="m in minutes" :key="m" :value="m">:{{ String(m).padStart(2, '0') }}</option>
                    </select>
                </div>
            </div>

            <!-- Day of Week (for weekly) -->
            <div v-if="frequency === 'weekly'">
                <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.day_of_week') }}</label>
                <div class="flex flex-wrap gap-1">
                    <button 
                        v-for="day in weekDays" 
                        :key="day.value"
                        @click="toggleDayOfWeek(day.value)"
                        class="px-2 py-1 text-xs rounded border transition-colors"
                        :class="dayOfWeek.includes(day.value) ? 'bg-primary text-primary-foreground' : 'bg-background hover:bg-muted'"
                    >
                        {{ $t(day.label) }}
                    </button>
                </div>
            </div>

            <!-- Day of Month (for monthly) -->
            <div v-if="frequency === 'monthly'">
                <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.day_of_month') }}</label>
                <select v-model="dayOfMonth" class="w-full h-9 rounded-md border border-input bg-background px-3 text-sm">
                    <option v-for="d in days" :key="d" :value="d">{{ d }}</option>
                </select>
            </div>

            <!-- Preview -->
            <div class="bg-muted/50 rounded p-2 text-xs">
                <span class="text-muted-foreground">Cron:</span>
                <code class="ml-2 font-mono bg-background px-1 py-0.5 rounded">{{ generatedCron }}</code>
            </div>
        </template>

        <!-- Advanced Mode -->
        <template v-else>
            <div>
                <label class="text-xs font-medium text-muted-foreground mb-1 block">{{ $t('cron.expression') }}</label>
                <input 
                    v-model="advancedValue"
                    @blur="updateAdvanced"
                    @keyup.enter="updateAdvanced"
                    class="w-full h-9 rounded-md border border-input bg-background px-3 text-sm font-mono"
                    placeholder="* * * * *"
                />
            </div>
            <div class="text-xs text-muted-foreground">
                {{ $t('cron.format_hint') }}
            </div>
        </template>
    </div>
</template>
