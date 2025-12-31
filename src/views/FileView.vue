<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'
import { open } from '@tauri-apps/plugin-dialog'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

// 推送
const pushLocal = ref('')
const pushRemote = ref('/sdcard/')

// 拉取
const pullRemote = ref('/sdcard/')
const pullLocal = ref('')

async function selectPushFile() {
  const file = await open({ title: '选择要推送的文件' })
  if (file && typeof file === 'string') {
    pushLocal.value = file
  }
}

async function selectPullDir() {
  const dir = await open({ directory: true, title: '选择保存目录' })
  if (dir && typeof dir === 'string') {
    pullLocal.value = dir
  }
}

async function pushFile() {
  if (!pushLocal.value || !pushRemote.value || !deviceStore.currentDevice) {
    logStore.error('请填写完整信息')
    return
  }
  
  logStore.log(`推送: ${pushLocal.value} -> ${pushRemote.value}`)
  try {
    const result = await adb.push(pushLocal.value, pushRemote.value, deviceStore.currentDevice)
    logStore.success(`推送成功: ${result}`)
  } catch (e: any) {
    logStore.error(`推送失败: ${e}`)
  }
}

async function pullFile() {
  if (!pullRemote.value || !pullLocal.value || !deviceStore.currentDevice) {
    logStore.error('请填写完整信息')
    return
  }
  
  logStore.log(`拉取: ${pullRemote.value} -> ${pullLocal.value}`)
  try {
    const result = await adb.pull(pullRemote.value, pullLocal.value, deviceStore.currentDevice)
    logStore.success(`拉取成功: ${result}`)
  } catch (e: any) {
    logStore.error(`拉取失败: ${e}`)
  }
}

async function listDir(path: string) {
  if (!deviceStore.currentDevice) return
  logStore.command(`ls -la ${path}`)
  try {
    const result = await adb.shell(`ls -la ${path}`, deviceStore.currentDevice)
    logStore.log(result)
  } catch (e: any) {
    logStore.error(`列出失败: ${e}`)
  }
}

async function deleteFile() {
  if (!pullRemote.value || !deviceStore.currentDevice) return
  logStore.command(`rm ${pullRemote.value}`)
  try {
    await adb.shell(`rm -rf ${pullRemote.value}`, deviceStore.currentDevice)
    logStore.success('删除成功')
  } catch (e: any) {
    logStore.error(`删除失败: ${e}`)
  }
}
</script>

<template>
  <div class="file-view">
    <div class="card">
      <div class="card-title">📤 推送到设备</div>
      <div class="form-row">
        <label>本地文件:</label>
        <input v-model="pushLocal" class="input" placeholder="选择文件..." readonly />
        <button class="btn btn-ghost" @click="selectPushFile">浏览</button>
      </div>
      <div class="form-row">
        <label>设备路径:</label>
        <input v-model="pushRemote" class="input" placeholder="/sdcard/" />
        <button class="btn btn-ghost" @click="pushFile">📤 推送</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📥 从设备拉取</div>
      <div class="form-row">
        <label>设备路径:</label>
        <input v-model="pullRemote" class="input" placeholder="/sdcard/xxx" />
        <button class="btn btn-ghost" @click="listDir(pullRemote)">列出</button>
      </div>
      <div class="form-row">
        <label>保存到:</label>
        <input v-model="pullLocal" class="input" placeholder="选择目录..." readonly />
        <button class="btn btn-ghost" @click="selectPullDir">浏览</button>
        <button class="btn btn-ghost" @click="pullFile">📥 拉取</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📂 快捷操作</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="listDir('/sdcard/')">列出SD卡</button>
        <button class="btn btn-ghost" @click="listDir('/sdcard/Download/')">下载目录</button>
        <button class="btn btn-ghost" @click="listDir('/sdcard/DCIM/')">相册目录</button>
        <button class="btn btn-ghost" @click="deleteFile">删除文件</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.form-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
  
  label {
    width: 80px;
    flex-shrink: 0;
    color: var(--text-secondary);
    font-size: 13px;
  }
  
  .input {
    flex: 1;
  }
}
</style>
