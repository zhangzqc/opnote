<template>
  <div class="app-layout">
    <Sidebar />
    <MainContent />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Sidebar from './Sidebar.vue'
import MainContent from './MainContent.vue'
import { useTreeStore } from '../../stores/tree'

const treeStore = useTreeStore()

onMounted(() => {
  treeStore.loadTree()
})
</script>
