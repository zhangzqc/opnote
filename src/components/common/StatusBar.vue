<template>
  <div class="status-bar">
    <span v-if="editorStore.currentNodeId">
      节点 #{{ editorStore.currentNodeId }}
    </span>
    <span v-if="editorStore.isDirty" style="color: var(--warning);">● 未保存</span>
    <span v-if="editorStore.isSaving" style="color: var(--accent);">⏳ 保存中...</span>
    <span v-if="!editorStore.isDirty && !editorStore.isSaving && editorStore.currentNodeId" style="color: var(--success);">✓ 已保存</span>
    <span style="margin-left: auto;">
      {{ editorStore.currentFormat === 'markdown' ? 'Markdown' : editorStore.currentFormat }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { useEditorStore } from '../../stores/editor'

const editorStore = useEditorStore()
</script>
