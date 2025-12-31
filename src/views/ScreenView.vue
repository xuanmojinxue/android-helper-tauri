<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'

const deviceStore = useDeviceStore()
const logStore = useLogStore()
const settingsStore = useSettingsStore()

const resolution = ref(1080)
const bitrate = ref(8)
const options = ref({
  borderless: false,
  alwaysOnTop: false,
  fullscreen: false,
  turnScreenOff: false
})

async function startScrcpy() {
  const args: string[] = []
  args.push('-m', resolution.value.toString())
  args.push('-b', `${bitrate.value}M`)
  
  if (options.value.borderless) args.push('--window-borderless')
  if (options.value.alwaysOnTop) args.push('--always-on-top')
  if (options.value.fullscreen) args.push('--fullscreen')
  if (options.value.turnScreenOff) args.push('--turn-screen-off')
  
  if (deviceStore.currentDevice) {
    args.push('-s', deviceStore.currentDevice)
  }
  
  logStore.command(`scrcpy ${args.join(' ')}`)
  try {
    await invoke('start_scrcpy', { args })
    logStore.success('Scrcpy已启动')
  } catch (e: any) {
    logStore.error(`启动失败: ${e}`)
  }
}

async function screenshot() {
  if (!deviceStore.currentDevice) return
  logStore.log('截图中...')
  try {
    const outputDir = await settingsStore.getSubDir('screenshot')
    const path = await invoke('take_screenshot', { 
      device: deviceStore.currentDevice,
      outputDir 
    })
    logStore.success(`截图已保存: ${path}`)
  } catch (e: any) {
    logStore.error(`截图失败: ${e}`)
  }
}

async function startRecord() {
  logStore.log('开始录屏...')
  try {
    const outputDir = await settingsStore.getSubDir('record')
    await invoke('start_record', { 
      device: deviceStore.currentDevice,
      outputDir
    })
    logStore.success(`录屏已开始，保存到: ${outputDir}`)
  } catch (e: any) {
    logStore.error(`录屏失败: ${e}`)
  }
}
</script>

<template>
  <div class="screen-view">
    <div class="card">
      <div class="card-title">🖥️ Scrcpy投屏</div>
      <div class="grid grid-2" style="margin-bottom:16px">
        <div class="setting-item">
          <label>分辨率</label>
          <input v-model="resolution" type="number" class="input" />
        </div>
        <div class="setting-item">
          <label>码率 (Mbps)</label>
          <input v-model="bitrate" type="number" class="input" />
        </div>
      </div>
      
      <div class="options">
        <label class="checkbox-item">
          <input type="checkbox" v-model="options.borderless" />
          <span>无边框</span>
        </label>
        <label class="checkbox-item">
          <input type="checkbox" v-model="options.alwaysOnTop" />
          <span>置顶</span>
        </label>
        <label class="checkbox-item">
          <input type="checkbox" v-model="options.fullscreen" />
          <span>全屏</span>
        </label>
        <label class="checkbox-item">
          <input type="checkbox" v-model="options.turnScreenOff" />
          <span>关闭屏幕</span>
        </label>
      </div>
      
      <button class="btn btn-ghost" style="width:100%;margin-top:16px" @click="startScrcpy">
        🚀 启动投屏
      </button>
    </div>
    
    <div class="card">
      <div class="card-title">📸 截图录屏</div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="screenshot">📷 截图</button>
        <button class="btn btn-ghost" @click="startRecord">🔴 录屏</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.setting-item {
  label {
    display: block;
    margin-bottom: 6px;
    color: var(--text-secondary);
    font-size: 13px;
  }
  
  .input {
    width: 100%;
  }
}

.options {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  
  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    accent-color: var(--accent);
  }
}
</style>
