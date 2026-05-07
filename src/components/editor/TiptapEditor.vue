<template>
  <div v-if="editorStore.currentNodeId" class="tiptap-editor-wrapper">
    <EditorContent :editor="editor" />
  </div>
  <div v-else class="empty-state">
    <div class="icon">📝</div>
    <div class="text">选择节点开始编辑</div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, watch } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import Highlight from '@tiptap/extension-highlight'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import { useEditorStore } from '../../stores/editor'

const props = defineProps<{ nodeId: number }>()
const editorStore = useEditorStore()

let saveTimer: ReturnType<typeof setTimeout> | null = null

const editor = useEditor({
  extensions: [
    StarterKit.configure({
      heading: { levels: [1, 2, 3] },
    }),
    Placeholder.configure({
      placeholder: '输入 / 打开斜杠菜单，或直接开始写作...',
    }),
    Highlight,
    TaskList,
    TaskItem.configure({ nested: true }),
  ],
  content: '',
  editorProps: {
    attributes: {
      class: 'tiptap-editor',
    },
  },
  onUpdate: ({ editor }) => {
    const html = editor.getHTML()
    editorStore.updateContent(html)

    // Slash menu detection
    const { $from } = editor.state.selection
    const textBefore = $from.parent.textContent.slice(0, $from.parentOffset)
    if (textBefore.endsWith('/')) {
      showSlashMenu(editor)
    }
  },
})

// Expose editor to parent for toolbar
editorStore.editorInstance = editor.value!

// Load content when nodeId changes
watch(() => props.nodeId, async (newId) => {
  if (newId) {
    await editorStore.loadContent(newId)
    if (editor.value) {
      editor.value.commands.setContent(editorStore.currentContent || '')
    }
  }
}, { immediate: true })

onMounted(async () => {
  if (props.nodeId) {
    await editorStore.loadContent(props.nodeId)
    if (editor.value) {
      editor.value.commands.setContent(editorStore.currentContent || '')
    }
  }
})

onBeforeUnmount(() => {
  editorStore.saveNow()
  editor.value?.destroy()
})

// Slash menu handler
function showSlashMenu(editor: any) {
  // Simple slash menu: create a floating menu with options
  const { node } = editor.view.dom.getBoundingClientRect()
  const slashItems = [
    { label: '标题 1', icon: '𝗛₁', action: () => editor.chain().focus().toggleHeading({ level: 1 }).run() },
    { label: '标题 2', icon: '𝗛₂', action: () => editor.chain().focus().toggleHeading({ level: 2 }).run() },
    { label: '标题 3', icon: '𝗛₃', action: () => editor.chain().focus().toggleHeading({ level: 3 }).run() },
    { label: '无序列表', icon: '•', action: () => editor.chain().focus().toggleBulletList().run() },
    { label: '有序列表', icon: '1.', action: () => editor.chain().focus().toggleOrderedList().run() },
    { label: '任务列表', icon: '☑', action: () => editor.chain().focus().toggleTaskList().run() },
    { label: '代码块', icon: '⚙', action: () => editor.chain().focus().toggleCodeBlock().run() },
    { label: '引用', icon: '❝', action: () => editor.chain().focus().toggleBlockquote().run() },
    { label: '分割线', icon: '───', action: () => editor.chain().focus().setHorizontalRule().run() },
  ]

  // Get cursor position
  const { from } = editor.state.selection
  const coords = editor.view.coordsAtPos(from)

  // Create menu element
  const menu = document.createElement('div')
  menu.className = 'slash-menu'
  menu.style.left = coords.left + 'px'
  menu.style.top = (coords.bottom + 8) + 'px'

  let selectedIndex = 0

  slashItems.forEach((item, i) => {
    const el = document.createElement('div')
    el.className = 'slash-menu-item' + (i === 0 ? ' selected' : '')
    el.innerHTML = `<span class="icon">${item.icon}</span><span class="label">${item.label}</span>`
    el.addEventListener('click', () => {
      // Delete the slash character
      editor.chain().focus().deleteRange({
        from: from - 1,
        to: from,
      }).run()
      item.action()
      menu.remove()
    })
    el.addEventListener('mouseenter', () => {
      menu.querySelectorAll('.slash-menu-item').forEach(e => e.classList.remove('selected'))
      el.classList.add('selected')
      selectedIndex = i
    })
    menu.appendChild(el)
  })

  document.body.appendChild(menu)

  // Keyboard navigation
  const handleKey = (e: KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault()
      selectedIndex = Math.min(selectedIndex + 1, slashItems.length - 1)
      menu.querySelectorAll('.slash-menu-item').forEach((el, i) => {
        el.classList.toggle('selected', i === selectedIndex)
      })
    } else if (e.key === 'ArrowUp') {
      e.preventDefault()
      selectedIndex = Math.max(selectedIndex - 1, 0)
      menu.querySelectorAll('.slash-menu-item').forEach((el, i) => {
        el.classList.toggle('selected', i === selectedIndex)
      })
    } else if (e.key === 'Enter') {
      e.preventDefault()
      // Delete slash and execute
      editor.chain().focus().deleteRange({ from: from - 1, to: from }).run()
      slashItems[selectedIndex].action()
      cleanup()
    } else if (e.key === 'Escape') {
      cleanup()
    }
  }

  const handleClickOutside = (e: MouseEvent) => {
    if (!menu.contains(e.target as Node)) {
      cleanup()
    }
  }

  function cleanup() {
    menu.remove()
    document.removeEventListener('keydown', handleKey)
    document.removeEventListener('click', handleClickOutside)
  }

  document.addEventListener('keydown', handleKey)
  setTimeout(() => document.addEventListener('click', handleClickOutside), 0)
}
</script>
