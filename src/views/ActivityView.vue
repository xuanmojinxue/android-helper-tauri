<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const actPkg = ref('')
const actName = ref('')
const intentAction = ref('')
const intentData = ref('')

async function startActivity() {
  if (!actPkg.value || !actName.value || !deviceStore.currentDevice) {
    logStore.error('请填写包名和Activity')
    return
  }
  
  const activity = actName.value.startsWith('.') ? `${actPkg.value}${actName.value}` : actName.value
  const cmd = `am start -n ${actPkg.value}/${activity}`
  
  logStore.command(cmd)
  try {
    await adb.shell(cmd, deviceStore.currentDevice)
    logStore.success('Activity已启动')
  } catch (e: any) {
    logStore.error(`启动失败: ${e}`)
  }
}

async function startIntent() {
  if (!intentAction.value || !deviceStore.currentDevice) {
    logStore.error('请填写Intent Action')
    return
  }
  
  let cmd = `am start -a ${intentAction.value}`
  if (intentData.value) {
    cmd += ` -d ${intentData.value}`
  }
  
  logStore.command(cmd)
  try {
    await adb.shell(cmd, deviceStore.currentDevice)
    logStore.success('Intent已发送')
  } catch (e: any) {
    logStore.error(`发送失败: ${e}`)
  }
}

async function quickStart(activity: string) {
  if (!deviceStore.currentDevice) return
  logStore.command(`am start -n ${activity}`)
  try {
    await adb.shell(`am start -n ${activity}`, deviceStore.currentDevice)
    logStore.success('已启动')
  } catch (e: any) {
    logStore.error(`启动失败: ${e}`)
  }
}

async function getCurrentActivity() {
  if (!deviceStore.currentDevice) return
  try {
    const result = await adb.shell("dumpsys activity activities | grep 'mResumedActivity'", deviceStore.currentDevice)
    logStore.log(`当前Activity: ${result.trim()}`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

const shortcuts = [
  { label: '设置', activity: 'com.android.settings/.Settings' },
  { label: '开发者选项', activity: 'com.android.settings/.DevelopmentSettings' },
  { label: '应用管理', activity: 'com.android.settings/.applications.ManageApplications' },
  { label: '关于手机', activity: 'com.android.settings/.DeviceInfoSettings' },
  { label: 'WiFi设置', activity: 'com.android.settings/.wifi.WifiSettings' },
  { label: '蓝牙设置', activity: 'com.android.settings/.bluetooth.BluetoothSettings' },
  { label: '显示设置', activity: 'com.android.settings/.DisplaySettings' },
  { label: '声音设置', activity: 'com.android.settings/.SoundSettings' },
]
</script>

<template>
  <div class="activity-view">
    <div class="card">
      <div class="card-title">🎯 启动Activity</div>
      <div class="flex flex-gap" style="margin-bottom:10px">
        <input v-model="actPkg" class="input" placeholder="包名 (com.example.app)" style="flex:1" />
      </div>
      <div class="flex flex-gap">
        <input v-model="actName" class="input" placeholder="Activity (.MainActivity)" style="flex:1" />
        <button class="btn btn-ghost" @click="startActivity">启动</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📨 发送Intent</div>
      <div class="flex flex-gap" style="margin-bottom:10px">
        <input v-model="intentAction" class="input" placeholder="Action (android.intent.action.VIEW)" style="flex:1" />
      </div>
      <div class="flex flex-gap">
        <input v-model="intentData" class="input" placeholder="Data URI (可选)" style="flex:1" />
        <button class="btn btn-ghost" @click="startIntent">发送</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📱 快捷启动</div>
      <div class="grid grid-4">
        <button 
          v-for="item in shortcuts" 
          :key="item.activity"
          class="btn btn-ghost"
          @click="quickStart(item.activity)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔍 调试</div>
      <button class="btn btn-ghost" @click="getCurrentActivity">获取当前Activity</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
</style>
