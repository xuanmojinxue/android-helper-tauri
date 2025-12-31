<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useLogStore } from '@/stores/log'
import { invoke } from '@tauri-apps/api/core'

const settingsStore = useSettingsStore()
const logStore = useLogStore()

const outputDir = ref('')

onMounted(async () => {
  await settingsStore.init()
  outputDir.value = settingsStore.outputDir
})

async function openOutputDir() {
  try {
    await invoke('open_folder', { path: settingsStore.outputDir })
  } catch (e: any) {
    logStore.error(`打开失败: ${e}`)
  }
}

function getSubDirDesc(key: string): string {
  const descs: Record<string, string> = {
    apk: '提取的APK文件',
    backup: '分区备份文件',
    screenshot: '设备截图',
    record: '屏幕录制',
    rom: 'ROM分区提取',
    module: 'Root模块提取',
    log: '日志文件'
  }
  return descs[key] || ''
}
</script>

<template>
  <div class="settings-view">
    <div class="card">
      <div class="card-title">📁 数据目录</div>
      <p class="desc">所有提取的文件、截图、备份等都会保存在程序目录下的 data 文件夹</p>
      
      <div class="flex flex-gap" style="margin-top:12px">
        <input v-model="outputDir" class="input" style="flex:1" readonly />
        <button class="btn btn-ghost" @click="openOutputDir">打开目录</button>
      </div>
      
      <div class="subdir-list">
        <div class="subdir-title">子目录结构:</div>
        <div class="subdir-item" v-for="(name, key) in settingsStore.subDirs" :key="key">
          <span class="icon">📂</span>
          <span class="name">{{ name }}</span>
          <span class="desc-text">{{ getSubDirDesc(key as string) }}</span>
        </div>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">ℹ️ 关于</div>
      <div class="about-info">
        <div class="info-row">
          <span class="label">应用名称</span>
          <span class="value">Android玩机助手</span>
        </div>
        <div class="info-row">
          <span class="label">版本</span>
          <span class="value">1.0.0</span>
        </div>
        <div class="info-row">
          <span class="label">框架</span>
          <span class="value">Tauri + Vue 3</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.desc {
  color: var(--text-secondary);
  font-size: 13px;
  margin-bottom: 8px;
}

.subdir-list {
  margin-top: 16px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.subdir-title {
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 10px;
}

.subdir-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
  font-size: 13px;
  
  &:last-child {
    border-bottom: none;
  }
  
  .icon {
    font-size: 14px;
  }
  
  .name {
    min-width: 100px;
  }
  
  .desc-text {
    color: var(--text-secondary);
    font-size: 12px;
  }
}

.about-info {
  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 10px 0;
    border-bottom: 1px solid var(--border);
    
    &:last-child {
      border-bottom: none;
    }
    
    .label {
      color: var(--text-secondary);
    }
  }
}
</style>
