<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import DeviceBar from './components/DeviceBar.vue'
import LogPanel from './components/LogPanel.vue'
import { useDeviceStore } from './stores/device'
import { useSettingsStore } from './stores/settings'

const deviceStore = useDeviceStore()
const settingsStore = useSettingsStore()

onMounted(async () => {
  await settingsStore.init()
  deviceStore.startMonitor()
})

onUnmounted(() => {
  deviceStore.stopMonitor()
})
</script>

<template>
  <div class="app-container">
    <DeviceBar />
    <div class="main-content">
      <Sidebar />
      <div class="content-area">
        <router-view />
      </div>
    </div>
    <LogPanel />
  </div>
</template>

<style lang="scss">
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background: var(--bg-secondary);
}
</style>
