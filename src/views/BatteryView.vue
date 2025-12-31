<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const batteryInfo = ref('')
const fakeLevel = ref('100')

async function refreshBattery() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取电池信息...')
  try {
    const result = await adb.shell('dumpsys battery', deviceStore.currentDevice)
    batteryInfo.value = result
    logStore.success('电池信息已刷新')
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function getBatteryStats() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取电池统计...')
  try {
    const result = await adb.shell('dumpsys batterystats | head -100', deviceStore.currentDevice)
    batteryInfo.value = result
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function setFakeBattery() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell(`dumpsys battery set level ${fakeLevel.value}`, deviceStore.currentDevice)
    logStore.success(`电量已设置为 ${fakeLevel.value}%`)
    refreshBattery()
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function setCharging(status: string) {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell(`dumpsys battery set status ${status}`, deviceStore.currentDevice)
    logStore.success('充电状态已设置')
    refreshBattery()
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function resetBattery() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell('dumpsys battery reset', deviceStore.currentDevice)
    logStore.success('电池状态已重置')
    refreshBattery()
  } catch (e: any) {
    logStore.error(`重置失败: ${e}`)
  }
}

async function unplugBattery() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell('dumpsys battery unplug', deviceStore.currentDevice)
    logStore.success('已模拟拔出充电器')
    refreshBattery()
  } catch (e: any) {
    logStore.error(`操作失败: ${e}`)
  }
}
</script>

<template>
  <div class="battery-view">
    <div class="card">
      <div class="card-title">🔋 电池信息</div>
      <pre class="battery-info">{{ batteryInfo || '点击刷新获取电池信息...' }}</pre>
      <div class="grid grid-3" style="margin-top:12px">
        <button class="btn btn-ghost" @click="refreshBattery">🔄 刷新</button>
        <button class="btn btn-ghost" @click="getBatteryStats">电池统计</button>
        <button class="btn btn-ghost" @click="resetBattery">重置状态</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🎭 模拟电池</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <span style="color:var(--text-secondary)">模拟电量:</span>
        <select v-model="fakeLevel" class="input" style="width:100px">
          <option v-for="i in [0,5,10,15,20,25,30,40,50,60,70,80,90,100]" :key="i" :value="String(i)">{{ i }}%</option>
        </select>
        <button class="btn btn-ghost" @click="setFakeBattery">设置</button>
      </div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="setCharging('2')">模拟充电中</button>
        <button class="btn btn-ghost" @click="setCharging('3')">模拟未充电</button>
        <button class="btn btn-ghost" @click="unplugBattery">拔出充电器</button>
        <button class="btn btn-ghost" @click="resetBattery">恢复真实</button>
      </div>
      <p class="hint">💡 用于测试应用在不同电量下的行为</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.battery-info {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  max-height: 250px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}

.hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 12px;
}
</style>
