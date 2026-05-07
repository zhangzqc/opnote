import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getContent, saveContent, type Content } from '../lib/api'
import type { Editor } from '@tiptap/vue-3'

export const useEditorStore = defineStore('editor', () => {
  const currentContent = ref<string>('')
  const currentFormat = ref<string>('markdown')
  const currentNodeId = ref<number | null>(null)
  const isDirty = ref(false)
  const isSaving = ref(false)
  const editorInstance = ref<Editor | null>(null)
  let saveTimer: ReturnType<typeof setTimeout> | null = null

  async function loadContent(nodeId: number) {
    try {
      const content = await getContent(nodeId)
      currentNodeId.value = nodeId
      currentContent.value = content.content
      currentFormat.value = content.format
      isDirty.value = false
    } catch (e) {
      console.error('Failed to load content:', e)
      currentNodeId.value = nodeId
      currentContent.value = ''
      currentFormat.value = 'markdown'
    }
  }

  function markDirty() {
    isDirty.value = true
    scheduleAutoSave()
  }

  function updateContent(content: string) {
    currentContent.value = content
    markDirty()
  }

  function scheduleAutoSave() {
    if (saveTimer) clearTimeout(saveTimer)
    saveTimer = setTimeout(() => {
      doSave()
    }, 1500)
  }

  async function doSave() {
    if (!currentNodeId.value || isSaving.value) return
    isSaving.value = true
    try {
      await saveContent(currentNodeId.value, currentContent.value, currentFormat.value)
      isDirty.value = false
    } catch (e) {
      console.error('Auto-save failed:', e)
    } finally {
      isSaving.value = false
    }
  }

  async function saveNow() {
    if (saveTimer) {
      clearTimeout(saveTimer)
      saveTimer = null
    }
    await doSave()
  }

  return {
    currentContent,
    currentFormat,
    currentNodeId,
    isDirty,
    isSaving,
    editorInstance,
    loadContent,
    updateContent,
    markDirty,
    saveNow,
  }
})
