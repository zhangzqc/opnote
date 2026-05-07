<template>
  <div class="main-content">
    <template v-if="treeStore.selectedNode">
      <!-- Header with breadcrumb -->
      <div class="content-header">
        <div class="breadcrumb">
          <template v-for="(node, idx) in breadcrumb" :key="node.id">
            <span v-if="idx > 0">›</span>
            <span :style="{ color: idx === breadcrumb.length - 1 ? 'var(--accent)' : 'var(--text-secondary)' }">
              {{ node.icon }} {{ node.name }}
            </span>
          </template>
        </div>
        <div class="actions">
          <button @click="exportMd" title="导出 Markdown">📝</button>
        </div>
      </div>

      <!-- Toolbar (only for non-folder) -->
      <div v-if="treeStore.selectedNode.node_type !== 'folder'" class="editor-toolbar">
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('bold') }" @click="editor?.chain().focus().toggleBold().run()"><b>B</b></button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('italic') }" @click="editor?.chain().focus().toggleItalic().run()"><i>I</i></button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('strike') }" @click="editor?.chain().focus().toggleStrike().run()"><s>S</s></button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('code') }" @click="editor?.chain().focus().toggleCode().run()">Code</button>
        <div class="toolbar-divider"></div>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('heading', { level: 1 }) }" @click="editor?.chain().focus().toggleHeading({ level: 1 }).run()">H1</button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('heading', { level: 2 }) }" @click="editor?.chain().focus().toggleHeading({ level: 2 }).run()">H2</button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('heading', { level: 3 }) }" @click="editor?.chain().focus().toggleHeading({ level: 3 }).run()">H3</button>
        <div class="toolbar-divider"></div>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('bulletList') }" @click="editor?.chain().focus().toggleBulletList().run()">• List</button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('orderedList') }" @click="editor?.chain().focus().toggleOrderedList().run()">1. List</button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('taskList') }" @click="editor?.chain().focus().toggleTaskList().run()">☑ Task</button>
        <div class="toolbar-divider"></div>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('blockquote') }" @click="editor?.chain().focus().toggleBlockquote().run()">❝ Quote</button>
        <button class="toolbar-btn" :class="{ 'is-active': editor?.isActive('codeBlock') }" @click="editor?.chain().focus().toggleCodeBlock().run()">⚙ Code</button>
        <button class="toolbar-btn" @click="editor?.chain().focus().setHorizontalRule().run()">───</button>
        <div class="toolbar-divider"></div>
        <button class="toolbar-btn" @click="editor?.chain().focus().undo().run()">↩ Undo</button>
        <button class="toolbar-btn" @click="editor?.chain().focus().redo().run()">↪ Redo</button>
      </div>

      <!-- Editor -->
      <div v-if="treeStore.selectedNode.node_type !== 'folder'" class="editor-area">
        <TiptapEditor
          :key="treeStore.selectedNodeId"
          :nodeId="treeStore.selectedNodeId!"
        />
      </div>

      <!-- Folder: show children overview -->
      <div v-else class="editor-area" style="display: flex; align-items: center; justify-content: center;">
        <div style="text-align: center; color: var(--text-muted);">
          <div style="font-size: 48px; margin-bottom: 12px;">📂</div>
          <div style="font-size: 15px;">{{ treeStore.selectedNode.name }}</div>
          <div style="font-size: 13px; margin-top: 4px;">{{ treeStore.selectedNode.children.length }} 个子节点</div>
        </div>
      </div>
    </template>

    <!-- Empty state -->
    <div v-else class="empty-state">
      <div class="icon">📕</div>
      <div class="text">选择左侧节点开始编辑</div>
      <div class="text" style="font-size: 12px;">或点击 ➕ 创建新笔记</div>
    </div>

    <!-- Status bar -->
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useTreeStore } from '../../stores/tree'
import { useEditorStore } from '../../stores/editor'
import TiptapEditor from '../editor/TiptapEditor.vue'
import StatusBar from '../common/StatusBar.vue'
import { exportNode } from '../../lib/api'

const treeStore = useTreeStore()
const editorStore = useEditorStore()
const editor = computed(() => editorStore.editorInstance)

const breadcrumb = computed(() => {
  if (!treeStore.selectedNodeId) return []
  return treeStore.getBreadcrumb(treeStore.selectedNodeId)
})

async function exportMd() {
  if (!treeStore.selectedNodeId) return
  try {
    await exportNode(treeStore.selectedNodeId, 'markdown', `~/opnote-export-${treeStore.selectedNodeId}.md`)
    alert('导出成功！')
  } catch (e) {
    console.error('Export failed:', e)
  }
}
</script>
