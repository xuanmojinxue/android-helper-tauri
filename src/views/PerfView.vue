<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const perfData = ref({
  cpu: '-',
  memory: '-',
  temperature: '-',
  battery: '-',
  uptime: '-'
})

const isRunning = ref(false)
let intervalId: number | null = null

async function updatePerf() {
  if (!deviceStore.currentDevice) return
  
  try {
    // CPU
    const cpu = await adb.shell("top -n 1 | head -5", deviceStore.currentDevice)
    const cpuMatch = cpu.match(/(\d+)%cpu/)
    perfData.value.cpu = cpuMatch ? `${cpuMatch[1]}%` : cpu.split('\n')[0] || '-'
    
    // 内存
    const mem = await adb.shell("cat /proc/meminfo | head -3", deviceStore.currentDevice)
    const memLines = mem.split('\n')
    const total = memLines[0]?.match(/(\d+)/)?.[1]
    const free = memLines[1]?.match(/(\d+)/)?.[1]
    if (total && free) {
      const usedMB = Math.round((parseInt(total) - parseInt(free)) / 1024)
      const totalMB = Math.round(parseInt(total) / 1024)
      perfData.value.memory = `${usedMB}MB / ${totalMB}MB`
    }
    
    // 温度
    const temp = await adb.shell("cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null", deviceStore.currentDevice)
    if (temp.trim()) {
      const tempC = parseInt(temp.trim()) / 1000
      perfData.value.temperature = `${tempC.toFixed(1)}°C`
    }
    
    // 电池
    const battery = await adb.shell("dumpsys battery | grep level", deviceStore.currentDevice)
    const batteryMatch = battery.match(/level:\s*(\d+)/)
    perfData.value.battery = batteryMatch ? `${batteryMatch[1]}%` : '-'
    
    // 运行时间
    const uptime = await adb.shell("uptime", deviceStore.currentDevice)
    const uptimeMatch = uptime.match(/up\s+([^,]+)/)
    perfData.value.uptime = uptimeMatch ? uptimeMatch[1].trim() : '-'
    
  } catch (e: any) {
    console.error('更新性能数据失败:', e)
  }
}

function startMonitor() {
  if (isRunning.value) return
  isRunning.value = true
  logStore.log('性能监控已启动')
  updatePerf()
  intervalId = window.setInterval(updatePerf, 2000)
}

function stopMonitor() {
  if (!isRunning.value) return
  isRunning.value = false
  if (intervalId) {
    clearInterval(intervalId)
    intervalId = null
  }
  logStore.log('性能监控已停止')
}

async function runCommand(cmd: string) {
  if (!deviceStore.currentDevice) return
  logStore.command(cmd)
  try {
    const result = await adb.shell(cmd, deviceStore.currentDevice)
    logStore.log(result)
  } catch (e: any) {
    logStore.error(`执行失败: ${e}`)
  }
}

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>

<template>
  <div class="perf-view">
    <div class="card">
      <div class="card-title">⏱️ 实时监控</div>
      
      <div class="perf-grid">
        <div class="perf-item">
          <span class="perf-icon">💻</span>
          <span class="perf-label">CPU</span>
          <span class="perf-value">{{ perfData.cpu }}</span>
        </div>
        <div class="perf-item">
          <span class="perf-icon">🧠</span>
          <span class="perf-label">内存</span>
          <span class="perf-value">{{ perfData.memory }}</span>
        </div>
        <div class="perf-item">
          <span class="perf-icon">🌡️</span>
          <span class="perf-label">温度</span>
          <span class="perf-value">{{ perfData.temperature }}</span>
        </div>
        <div class="perf-item">
          <span class="perf-icon">🔋</span>
          <span class="perf-label">电池</span>
          <span class="perf-value">{{ perfData.battery }}</span>
        </div>
        <div class="perf-item">
          <span class="perf-icon">⏰</span>
          <span class="perf-label">运行时间</span>
          <span class="perf-value">{{ perfData.uptime }}</span>
        </div>
      </div>
      
      <div class="flex flex-gap" style="margin-top:16px">
        <button class="btn btn-ghost" @click="startMonitor" :disabled="isRunning">▶ 开始监控</button>
        <button class="btn btn-ghost" @click="stopMonitor" :disabled="!isRunning">⏹ 停止</button>
        <span class="status" :class="{ running: isRunning }">
          {{ isRunning ? '🟢 监控中' : '⚪ 已停止' }}
        </span>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🛡️ 安全检测</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="runCommand('getenforce')">SELinux状态</button>
        <button class="btn btn-ghost" @click="runCommand('su -c id')">Root检测</button>
        <button class="btn btn-ghost" @click="runCommand('getprop ro.boot.verifiedbootstate')">Verified Boot</button>
        <button class="btn btn-ghost" @click="runCommand('getprop ro.debuggable')">调试模式</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📊 系统信息</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="runCommand('cat /proc/cpuinfo | grep processor')">CPU核心</button>
        <button class="btn btn-ghost" @click="runCommand('cat /proc/meminfo')">内存详情</button>
        <button class="btn btn-ghost" @click="runCommand('df -h')">存储空间</button>
        <button class="btn btn-ghost" @click="runCommand('dumpsys battery')">电池详情</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.perf-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 12px;
}

.perf-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 10px;
  
  .perf-icon {
    font-size: 24px;
    margin-bottom: 8px;
  }
  
  .perf-label {
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }
  
  .perf-value {
    font-size: 14px;
    font-weight: 600;
  }
}

.status {
  display: flex;
  align-items: center;
  font-size: 13px;
  color: var(--text-secondary);
  
  &.running {
    color: var(--success);
  }
}
</style>
