<script setup lang="ts">
import { useDeviceStore } from '@/stores/device'

const deviceStore = useDeviceStore()

function onDeviceChange(e: Event) {
  const select = e.target as HTMLSelectElement
  deviceStore.selectDevice(select.value)
}
</script>

<template>
  <header class="device-bar">
    <div class="device-info">
      <span class="status-dot" :class="{ connected: deviceStore.isConnected, loading: deviceStore.isLoading }"></span>
      <span v-if="deviceStore.deviceInfo">
        {{ deviceStore.deviceInfo.brand }} {{ deviceStore.deviceInfo.model }}
        <span class="android-version">Android {{ deviceStore.deviceInfo.android }}</span>
      </span>
      <span v-else class="no-device">未连接设备</span>
    </div>
    
    <div class="device-select">
      <select 
        class="input" 
        :value="deviceStore.currentDevice"
        @change="onDeviceChange"
      >
        <option v-if="deviceStore.devices.length === 0" value="">无设备</option>
        <option 
          v-for="device in deviceStore.devices" 
          :key="device.serial"
          :value="device.serial"
        >
          {{ device.serial }} ({{ device.status }})
        </option>
      </select>
      <button 
        class="btn btn-ghost" 
        @click="deviceStore.refreshDevices"
        :disabled="deviceStore.isLoading"
      >
        {{ deviceStore.isLoading ? '⏳' : '🔄' }}
      </button>
    </div>
  </header>
</template>

<style lang="scss" scoped>
.device-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
}

.device-info {
  display: flex;
  align-items: center;
  gap: 10px;
  
  .status-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--danger);
    transition: all 0.3s;
    
    &.connected {
      background: var(--success);
      box-shadow: 0 0 8px var(--success);
    }
    
    &.loading {
      animation: pulse 1s infinite;
    }
  }
  
  .android-version {
    color: var(--text-secondary);
    font-size: 12px;
    margin-left: 8px;
  }
  
  .no-device {
    color: var(--text-secondary);
  }
}

.device-select {
  display: flex;
  gap: 8px;
  
  select {
    min-width: 200px;
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>
