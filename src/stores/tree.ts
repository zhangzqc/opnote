import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getNodeTree, createNode, deleteNode, moveNode, updateNode, type TreeNode } from '../lib/api'

export const useTreeStore = defineStore('tree', () => {
  const tree = ref<TreeNode[]>([])
  const selectedNodeId = ref<number | null>(null)
  const expandedIds = ref<Set<number>>(new Set())

  const selectedNode = computed(() => {
    if (!selectedNodeId.value) return null
    return findNode(tree.value, selectedNodeId.value)
  })

  function findNode(nodes: TreeNode[], id: number): TreeNode | null {
    for (const node of nodes) {
      if (node.id === id) return node
      const found = findNode(node.children, id)
      if (found) return found
    }
    return null
  }

  // Flatten tree to get breadcrumb path
  function getBreadcrumb(id: number): TreeNode[] {
    const path: TreeNode[] = []
    function walk(nodes: TreeNode[], target: number): boolean {
      for (const node of nodes) {
        if (node.id === target) {
          path.push(node)
          return true
        }
        if (node.children.length > 0 && walk(node.children, target)) {
          path.unshift(node)
          return true
        }
      }
      return false
    }
    walk(tree.value, id)
    return path
  }

  async function loadTree() {
    try {
      tree.value = await getNodeTree()
      // Auto-expand all nodes marked as expanded
      const ids = new Set<number>()
      function collectExpanded(nodes: TreeNode[]) {
        for (const n of nodes) {
          if (n.is_expanded) ids.add(n.id)
          collectExpanded(n.children)
        }
      }
      collectExpanded(tree.value)
      expandedIds.value = ids
    } catch (e) {
      console.error('Failed to load tree:', e)
    }
  }

  function toggleExpand(id: number) {
    if (expandedIds.value.has(id)) {
      expandedIds.value.delete(id)
    } else {
      expandedIds.value.add(id)
    }
  }

  function selectNode(id: number) {
    selectedNodeId.value = id
  }

  async function addNode(parentId: number | null, name: string, nodeType: string) {
    await createNode(parentId, name, nodeType)
    await loadTree()
  }

  async function removeNode(id: number) {
    await deleteNode(id)
    if (selectedNodeId.value === id) {
      selectedNodeId.value = null
    }
    await loadTree()
  }

  async function moveNodeTo(id: number, parentId: number | null, sortOrder: number) {
    await moveNode(id, parentId, sortOrder)
    await loadTree()
  }

  async function renameNode(id: number, name: string) {
    await updateNode(id, name)
    await loadTree()
  }

  return {
    tree,
    selectedNodeId,
    selectedNode,
    expandedIds,
    getBreadcrumb,
    loadTree,
    toggleExpand,
    selectNode,
    addNode,
    removeNode,
    moveNodeTo,
    renameNode,
  }
})
