// Export a simple function to trigger toast
import { ref } from 'vue';

// Global state for toasts
const toasts = ref<any[]>([]);

export const toast = (props: any) => {
    // console.log("Toast:", props);
    toasts.value.push(props);
    setTimeout(() => {
        const index = toasts.value.indexOf(props);
        if (index > -1) toasts.value.splice(index, 1);
    }, 3000);
};

export const useToast = () => {
    return { toast, toasts };
};
