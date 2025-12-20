<template>
  <div ref="editorContainer" class="w-full h-full"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import * as monaco from 'monaco-editor';

const props = defineProps<{
  original: string;
  modified: string;
  originalPath?: string;
  modifiedPath?: string;
  renderSideBySide?: boolean;
}>();

const editorContainer = ref<HTMLElement | null>(null);
let diffEditor: monaco.editor.IStandaloneDiffEditor | null = null;

onMounted(() => {
    if (editorContainer.value) {
        diffEditor = monaco.editor.createDiffEditor(editorContainer.value, {
            originalEditable: false,
            readOnly: true,
            renderSideBySide: props.renderSideBySide ?? true,
            automaticLayout: true
        });
        updateModels();
    }
});

onUnmounted(() => {
    diffEditor?.dispose();
});

watch(() => [props.original, props.modified], () => {
    updateModels();
});

watch(() => props.renderSideBySide, (newVal) => {
    diffEditor?.updateOptions({
        renderSideBySide: newVal
    });
});

const updateModels = () => {
    if (!diffEditor) return;

    const originalModel = monaco.editor.createModel(props.original, undefined, props.originalPath ? monaco.Uri.parse(props.originalPath) : undefined);
    const modifiedModel = monaco.editor.createModel(props.modified, undefined, props.modifiedPath ? monaco.Uri.parse(props.modifiedPath) : undefined);

    diffEditor.setModel({
        original: originalModel,
        modified: modifiedModel
    });
};
</script>
