<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const apkFile = ref('')
const pkgName = ref('')
const analysisResult = ref('')

async function selectApkFile() {
  const file = await open({
    filters: [{ name: 'APK', extensions: ['apk'] }],
    title: '选择APK文件'
  })
  if (file && typeof file === 'string') {
    apkFile.value = file
  }
}

async function analyzeLocalApk() {
  if (!apkFile.value) {
    logStore.error('请先选择APK文件')
    return
  }
  
  logStore.log(`分析APK: ${apkFile.value}`)
  analysisResult.value = '正在分析...\n'
  
  try {
    const result = await invoke<string>('analyze_apk', { path: apkFile.value })
    analysisResult.value = result
    logStore.success('分析完成')
  } catch (e: any) {
    // 如果没有aapt工具，显示基本信息
    analysisResult.value = `APK文件: ${apkFile.value.split(/[/\\]/).pop()}\n`
    analysisResult.value += `\n⚠️ 详细分析需要aapt工具\n`
    analysisResult.value += `请将aapt.exe放入tools目录\n`
    logStore.error(`分析失败: ${e}`)
  }
}

async function analyzeInstalledApp() {
  if (!pkgName.value.trim()) {
    logStore.error('请输入包名')
    return
  }
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  
  const pkg = pkgName.value.trim()
  logStore.log(`分析应用: ${pkg}`)
  analysisResult.value = `📦 包名: ${pkg}\n\n`
  
  try {
    // 基本信息
    const dumpsys = await adb.shell(`dumpsys package ${pkg} | head -50`, deviceStore.currentDevice)
    
    for (const line of dumpsys.split('\n')) {
      const trimmed = line.trim()
      if (trimmed.includes('versionCode=')) {
        analysisResult.value += `🔢 ${trimmed}\n`
      } else if (trimmed.includes('versionName=')) {
        analysisResult.value += `📝 ${trimmed}\n`
      } else if (trimmed.includes('firstInstallTime=')) {
        analysisResult.value += `📅 首次安装: ${trimmed.split('=')[1]}\n`
      } else if (trimmed.includes('lastUpdateTime=')) {
        analysisResult.value += `🔄 最后更新: ${trimmed.split('=')[1]}\n`
      } else if (trimmed.includes('dataDir=')) {
        analysisResult.value += `📁 数据目录: ${trimmed.split('=')[1]}\n`
      }
    }
    
    // APK路径和大小
    const pathResult = await adb.shell(`pm path ${pkg}`, deviceStore.currentDevice)
    if (pathResult.trim()) {
      const apkPath = pathResult.replace('package:', '').trim()
      analysisResult.value += `📦 APK路径: ${apkPath}\n`
      
      const sizeResult = await adb.shell(`ls -l ${apkPath} | awk '{print $5}'`, deviceStore.currentDevice)
      if (sizeResult.trim()) {
        const sizeMB = (parseInt(sizeResult.trim()) / 1024 / 1024).toFixed(2)
        analysisResult.value += `📏 APK大小: ${sizeMB} MB\n`
      }
    }
    
    // 数据大小
    const dataSize = await adb.shell(`du -sh /data/data/${pkg} 2>/dev/null | awk '{print $1}'`, deviceStore.currentDevice)
    if (dataSize.trim()) {
      analysisResult.value += `💾 数据大小: ${dataSize.trim()}\n`
    }
    
    // 权限
    analysisResult.value += `\n🔐 权限列表:\n`
    const perms = await adb.shell(`dumpsys package ${pkg} | grep 'android.permission' | head -15`, deviceStore.currentDevice)
    for (const line of perms.split('\n').slice(0, 15)) {
      const perm = line.trim()
      if (perm) {
        analysisResult.value += `  • ${perm.split('.').pop()}\n`
      }
    }
    
    // 组件统计
    analysisResult.value += `\n📊 组件统计:\n`
    const actCount = await adb.shell(`dumpsys package ${pkg} | grep -c 'Activity'`, deviceStore.currentDevice)
    const svcCount = await adb.shell(`dumpsys package ${pkg} | grep -c 'Service'`, deviceStore.currentDevice)
    const rcvCount = await adb.shell(`dumpsys package ${pkg} | grep -c 'Receiver'`, deviceStore.currentDevice)
    
    analysisResult.value += `  📱 Activity: ${actCount.trim() || '0'}\n`
    analysisResult.value += `  ⚙️ Service: ${svcCount.trim() || '0'}\n`
    analysisResult.value += `  📡 Receiver: ${rcvCount.trim() || '0'}\n`
    
    logStore.success('分析完成')
  } catch (e: any) {
    analysisResult.value += `\n❌ 分析失败: ${e}\n`
    logStore.error(`分析失败: ${e}`)
  }
}

async function openAppInfo() {
  if (!pkgName.value.trim() || !deviceStore.currentDevice) return
  try {
    await adb.shell(`am start -a android.settings.APPLICATION_DETAILS_SETTINGS -d package:${pkgName.value.trim()}`, deviceStore.currentDevice)
    logStore.success('已打开应用信息页面')
  } catch (e: any) {
    logStore.error(`打开失败: ${e}`)
  }
}

async function launchApp() {
  if (!pkgName.value.trim() || !deviceStore.currentDevice) return
  try {
    await adb.shell(`monkey -p ${pkgName.value.trim()} -c android.intent.category.LAUNCHER 1`, deviceStore.currentDevice)
    logStore.success('应用已启动')
  } catch (e: any) {
    logStore.error(`启动失败: ${e}`)
  }
}
</script>

<template>
  <div class="apk-info-view">
    <div class="card">
      <div class="card-title">📦 本地APK分析</div>
      <div class="flex flex-gap">
        <input v-model="apkFile" class="input" placeholder="选择APK文件..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectApkFile">浏览</button>
        <button class="btn btn-ghost" @click="analyzeLocalApk">分析</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📱 已安装应用分析</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="pkgName" class="input" placeholder="包名 (com.example.app)" style="flex:1" />
        <button class="btn btn-ghost" @click="analyzeInstalledApp">查询</button>
      </div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="openAppInfo">打开应用信息</button>
        <button class="btn btn-ghost" @click="launchApp">启动应用</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📊 分析结果</div>
      <pre class="result-box">{{ analysisResult || '选择APK或输入包名后开始分析...' }}</pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.result-box {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  min-height: 300px;
  max-height: 400px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
