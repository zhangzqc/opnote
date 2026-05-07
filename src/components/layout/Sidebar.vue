<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <span class="logo">📕 OpNote</span>
      <button @click="showAddMenu = !showAddMenu" title="新建">➕</button>
      <button @click="collapseAll" title="全部折叠">📂</button>
    </div>

    <!-- Add node menu -->
    <div v-if="showAddMenu" class="context-menu" style="position: relative; margin: 4px 12px;">
      <div class="context-menu-item" @click="addNode('rich_text'); showAddMenu = false">
        <span>📄</span> <span>富文本笔记</span>
      </div>
      <div class="context-menu-item" @click="addNode('code'); showAddMenu = false">
        <span>💻</span> <span>代码片段</span>
      </div>
      <div class="context-menu-item" @click="addNode('folder'); showAddMenu = false">
        <span>📂</span> <span>文件夹</span>
      </div>
    </div>

    <div class="search-box">
      <input
        v-model="searchQuery"
        placeholder="搜索笔记... (Ctrl+K)"
        @input="onSearch"
        @keydown.escape="searchQuery = ''; searchResults = []"
      />
    </div>

    <!-- Search Results -->
    <div v-if="searchResults.length > 0" class="search-results" style="position: relative; width: auto; right: auto; top: auto; border: none; box-shadow: none; border-radius: 0;">
      <div
        v-for="result in searchResults"
        :key="result.node_id"
        class="search-result-item"
        @click="goToResult(result.node_id)"
      >
        <div class="name">{{ result.name }}</div>
        <div class="snippet" v-html="result.snippet"></div>
      </div>
    </div>

    <!-- Tree -->
    <div class="tree-container" v-if="searchResults.length === 0">
      <TreeNodeItem
        v-for="node in treeStore.tree"
        :key="node.id"
        :node="node"
        :depth="0"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTreeStore } from '../../stores/tree'
import { searchNodes, type SearchResult } from '../../lib/api'
import TreeNodeItem from '../tree/TreeNodeItem.vue'

const treeStore = useTreeStore()
const showAddMenu = ref(false)
const searchQuery = ref('')
const searchResults = ref<SearchResult[]>([])

let searchTimer: ReturnType<typeof setTimeout> | null = null

function addNode(type: string) {
  const parentId = treeStore.selectedNodeId
  const names: Record<string, string> = {
    rich_text: '新笔记',
    code: '代码片段',
    folder: '新文件夹',
  }
  treeStore.addNode(parentId, names[type] || '新节点', type)
}

function collapseAll() {
  treeStore.expandedIds = new Set()
}

async function onSearch() {
  if (searchTimer) clearTimeout(searchTimer)
  if (!searchQuery.value.trim()) {
    searchResults.value = []
    return
  }
  searchTimer = setTimeout(async () => {
    try {
      searchResults.value = await searchNodes(searchQuery.value)
    } catch (e) {
      console.error('Search failed:', e)
    }
  }, 300)
}

function goToResult(nodeId: number) {
  treeStore.selectNode(nodeId)
  treeStore.expandedIds.add(nodeId) // ensure visible
  searchQuery.value = ''
  searchResults.value = []
}
</script>
