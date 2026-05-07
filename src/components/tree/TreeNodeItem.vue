<template>
  <div
    class="tree-item"
    :class="{ active: treeStore.selectedNodeId === node.id }"
    :style="{ '--depth': depth }"
    @click="onClick"
    @contextmenu.prevent="onContextMenu"
    @dblclick="onDblClick"
    draggable="true"
    @dragstart="onDragStart"
    @dragover.prevent="onDragOver"
    @drop="onDrop"
  >
    <!-- Expand arrow -->
    <span
      class="arrow"
      :class="{
        expanded: treeStore.expandedIds.has(node.id),
        hidden: node.children.length === 0
      }"
      @click.stop="treeStore.toggleExpand(node.id)"
    >▶</span>

    <!-- Icon -->
    <span class="node-icon">{{ node.icon || '📄' }}</span>

    <!-- Name -->
    <span v-if="!isEditing" class="node-name">{{ node.name }}</span>
    <input
      v-else
      v-model="editName"
      class="node-name"
      style="background: var(--bg-tertiary); border: 1px solid var(--accent); border-radius: 3px; padding: 1px 4px; color: var(--text-primary); font-size: 13px; outline: none; width: 100%;"
      @blur="finishEdit"
      @keydown.enter="finishEdit"
      @keydown.escape="cancelEdit"
      ref="editInput"
    />
  </div>

  <!-- Children (recursive) -->
  <template v-if="treeStore.expandedIds.has(node.id)">
    <TreeNodeItem
      v-for="child in node.children"
      :key="child.id"
      :node="child"
      :depth="depth + 1"
    />
  </template>

  <!-- Context Menu -->
  <Teleport to="body">
    <div
      v-if="showContextMenu"
      class="context-menu"
      :style="{ left: contextMenuPos.x + 'px', top: contextMenuPos.y + 'px' }"
    >
      <div class="context-menu-item" @click="addChild('rich_text')">
        <span>📄</span> 新建富文本笔记
      </div>
      <div class="context-menu-item" @click="addChild('code')">
        <span>💻</span> 新建代码片段
      </div>
      <div class="context-menu-item" @click="addChild('folder')">
        <span>📂</span> 新建文件夹
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" @click="startEdit">
        <span>✏️</span> 重命名
      </div>
      <div class="context-menu-item danger" @click="deleteThis">
        <span>🗑️</span> 删除
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { useTreeStore } from '../../stores/tree'
import { useEditorStore } from '../../stores/editor'
import type { TreeNode } from '../../lib/api'

const props = defineProps<{
  node: TreeNode
  depth: number
}>()

const treeStore = useTreeStore()
const editorStore = useEditorStore()

const isEditing = ref(false)
const editName = ref('')
const editInput = ref<HTMLInputElement | null>(null)
const showContextMenu = ref(false)
const contextMenuPos = ref({ x: 0, y: 0 })

let dragNodeId: number | null = null

function onClick() {
  treeStore.selectNode(props.node.id)
  if (props.node.node_type !== 'folder') {
    editorStore.loadContent(props.node.id)
  }
}

function onContextMenu(e: MouseEvent) {
  treeStore.selectNode(props.node.id)
  showContextMenu.value = true
  contextMenuPos.value = { x: e.clientX, y: e.clientY }

  // Close on click elsewhere
  const close = () => {
    showContextMenu.value = false
    document.removeEventListener('click', close)
  }
  setTimeout(() => document.addEventListener('click', close), 0)
}

function onDblClick() {
  startEdit()
}

function startEdit() {
  showContextMenu.value = false
  isEditing.value = true
  editName.value = props.node.name
  nextTick(() => {
    editInput.value?.focus()
    editInput.value?.select()
  })
}

async function finishEdit() {
  if (editName.value.trim() && editName.value !== props.node.name) {
    await treeStore.renameNode(props.node.id, editName.value.trim())
  }
  isEditing.value = false
}

function cancelEdit() {
  isEditing.value = false
}

async function addChild(type: string) {
  showContextMenu.value = false
  const names: Record<string, string> = {
    rich_text: '新笔记',
    code: '代码片段',
    folder: '新文件夹',
  }
  await treeStore.addNode(props.node.id, names[type], type)
  treeStore.expandedIds.add(props.node.id)
}

async function deleteThis() {
  showContextMenu.value = false
  if (confirm(`确定删除「${props.node.name}」及其所有子节点？`)) {
    await treeStore.removeNode(props.node.id)
  }
}

// Drag & Drop
function onDragStart(e: DragEvent) {
  dragNodeId = props.node.id
  e.dataTransfer!.effectAllowed = 'move'
}

function onDragOver(e: DragEvent) {
  e.dataTransfer!.dropEffect = 'move'
}

async function onDrop(e: DragEvent) {
  if (dragNodeId !== null && dragNodeId !== props.node.id) {
    await treeStore.moveNodeTo(dragNodeId, props.node.id, props.node.sort_order + 1)
  }
  dragNodeId = null
}
</script>
