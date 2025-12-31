<script setup lang="ts">
import { ref, computed } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const logcatText = ref('')
const filterText = ref('')
const levelFilter = ref('all')
const isRunning = ref(false)
let intervalId: number | null = null

const filteredLogs = computed(() => {
  if (!logcatText.value) return ''
  let lines = logcatText.value.split('\n')
  
  // 按级别过滤
  if (levelFilter.value !== 'all') {
    const levelMap: Record<string, string[]> = {
      'error': ['E/'],
      'warn': ['W/', 'E/'],
      'info': ['I/', 'W/', 'E/'],
      'debug': ['D/', 'I/', 'W/', 'E/'],
    }
    const levels = levelMap[levelFilter.value] || []
    if (levels.length > 0) {
      lines = lines.filter(line => levels.some(l => line.includes(l)))
    }
  }
  
  // 按关键字过滤
  if (filterText.value) {
    const keyword = filterText.value.toLowerCase()
    lines = lines.filter(line => line.toLowerCase().includes(keyword))
  }
  
  return lines.join('\n')
})

async function fetchLogcat() {
  if (!deviceStore.currentDevice) return
  try {
    const result = await adb.logcat(deviceStore.currentDevice)
    logcatText.value = result
  } catch (e: any) {
    logStore.error(`获取Logcat失败: ${e}`)
  }
}

function startLogcat() {
  if (isRunning.value) return
  isRunning.value = true
  logStore.log('Logcat 已启动')
  fetchLogcat()
  intervalId = window.setInterval(fetchLogcat, 2000)
}

function stopLogcat() {
  if (!isRunning.value) return
  isRunning.value = false
  if (intervalId) {
    clearInterval(intervalId)
    intervalId = null
  }
  logStore.log('Logcat 已停止')
}

async function clearLogcat() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.clearLogcat(deviceStore.currentDevice)
    logcatText.value = ''
    logStore.success('Logcat 已清空')
  } catch (e: any) {
    logStore.error(`清空失败: ${e}`)
  }
}

function copyLogs() {
  navigator.clipboard.writeText(filteredLogs.value)
  logStore.success('日志已复制到剪贴板')
}
</script>

<template>
  <div class="logcat-view">
    <div class="card">
      <div class="card-title">📡 实时日志</div>
      
      <div class="toolbar">
        <input v-model="filterText" class="input" placeholder="过滤关键字..." style="flex:1" />
        <select v-model="levelFilter" class="input" style="width:120px">
          <option value="all">全部级别</option>
          <option value="error">Error</option>
          <option value="warn">Warn+</option>
          <option value="info">Info+</option>
          <option value="debug">Debug+</option>
        </select>
        <button class="btn btn-ghost" @click="startLogcat" :disabled="isRunning">▶ 开始</button>
        <button class="btn btn-ghost" @click="stopLogcat" :disabled="!isRunning">⏹ 停止</button>
        <button class="btn btn-ghost" @click="clearLogcat">清空</button>
        <button class="btn btn-ghost" @click="copyLogs">复制</button>
      </div>
      
      <div class="logcat-container">
        <pre class="logcat-text">{{ filteredLogs || '点击"开始"获取日志...' }}</pre>
      </div>
      
      <div class="status-bar">
        <span :class="{ running: isRunning }">
          {{ isRunning ? '🟢 运行中' : '⚪ 已停止' }}
        </span>
        <span>{{ logcatText.split('\n').length }} 行</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.toolbar {
  display: flex;
  gap: 10px;
  margin-bottom: 12px;
}

.logcat-container {
  background: #0d1117;
  border-radius: 8px;
  height: 450px;
  overflow: hidden;
}

.logcat-text {
  height: 100%;
  overflow-y: auto;
  padding: 12px;
  margin: 0;
  font-family: Consolas, Monaco, monospace;
  font-size: 11px;
  line-height: 1.5;
  color: #c9d1d9;
  white-space: pre-wrap;
  word-break: break-all;
}

.status-bar {
  display: flex;
  justify-content: space-between;
  margin-top: 10px;
  font-size: 12px;
  color: var(--text-secondary);
  
  .running {
    color: var(--success);
  }
}
</style>
