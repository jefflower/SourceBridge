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
let originalModel: monaco.editor.ITextModel | null = null;
let modifiedModel: monaco.editor.ITextModel | null = null;

const createEditor = () => {
    if (!editorContainer.value) return;
    
    // Dispose existing editor if any
    disposeEditor();
    
    console.log('[MonacoDiffEditor] Creating editor with renderSideBySide:', props.renderSideBySide);
    
    diffEditor = monaco.editor.createDiffEditor(editorContainer.value, {
        originalEditable: false,
        readOnly: true,
        renderSideBySide: props.renderSideBySide ?? true,
        automaticLayout: true,
        enableSplitViewResizing: true,
    });
    
    updateModels();
};

const disposeEditor = () => {
    if (originalModel) {
        originalModel.dispose();
        originalModel = null;
    }
    if (modifiedModel) {
        modifiedModel.dispose();
        modifiedModel = null;
    }
    if (diffEditor) {
        diffEditor.dispose();
        diffEditor = null;
    }
};

onMounted(() => {
    createEditor();
});

onUnmounted(() => {
    disposeEditor();
});

watch(() => [props.original, props.modified], () => {
    updateModels();
});

// When sideBySide changes, recreate the editor
watch(() => props.renderSideBySide, (newVal) => {
    console.log('[MonacoDiffEditor] renderSideBySide changed to:', newVal);
    createEditor();
});

const updateModels = () => {
    if (!diffEditor) return;

    // Dispose old models
    if (originalModel) {
        originalModel.dispose();
    }
    if (modifiedModel) {
        modifiedModel.dispose();
    }

    // Create new models
    originalModel = monaco.editor.createModel(
        props.original, 
        undefined
    );
    modifiedModel = monaco.editor.createModel(
        props.modified, 
        undefined
    );

    diffEditor.setModel({
        original: originalModel,
        modified: modifiedModel
    });
};
</script>

