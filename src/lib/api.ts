import { invoke } from '@tauri-apps/api/core'

export interface TreeNode {
  id: number
  parent_id: number | null
  name: string
  node_type: 'rich_text' | 'code' | 'folder'
  icon: string | null
  syntax: string | null
  sort_order: number
  is_expanded: boolean
  children: TreeNode[]
}

export interface Node {
  id: number
  parent_id: number | null
  name: string
  node_type: string
  icon: string | null
  syntax: string | null
  sort_order: number
  is_expanded: boolean
  created_at: string
  updated_at: string
}

export interface Content {
  node_id: number
  content: string
  format: string
  word_count: number
  char_count: number
  updated_at: string
}

export interface SearchResult {
  node_id: number
  name: string
  snippet: string
  rank: number
}

// Node APIs
export const getNodeTree = (): Promise<TreeNode[]> => invoke('get_node_tree')
export const createNode = (parentId: number | null, name: string, nodeType: string): Promise<Node> =>
  invoke('create_node', { parentId, name, nodeType })
export const updateNode = (id: number, name?: string, icon?: string, syntax?: string): Promise<void> =>
  invoke('update_node', { id, name, icon, syntax })
export const deleteNode = (id: number): Promise<void> => invoke('delete_node', { id })
export const moveNode = (id: number, parentId: number | null, sortOrder: number): Promise<void> =>
  invoke('move_node', { id, parentId, sortOrder })

// Content APIs
export const getContent = (nodeId: number): Promise<Content> => invoke('get_content', { nodeId })
export const saveContent = (nodeId: number, content: string, format: string): Promise<void> =>
  invoke('save_content', { nodeId, content, format })

// Search APIs
export const searchNodes = (query: string): Promise<SearchResult[]> => invoke('search_nodes', { query })

// Export APIs
export const exportNode = (nodeId: number, format: string, path: string): Promise<void> =>
  invoke('export_node', { nodeId, format, path })
