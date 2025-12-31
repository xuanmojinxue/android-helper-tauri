<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { useSettingsStore } from '@/stores/settings'
import { adb } from '@/utils/adb'
import { open } from '@tauri-apps/plugin-dialog'

const deviceStore = useDeviceStore()
const logStore = useLogStore()
const settingsStore = useSettingsStore()

const apkPath = ref('')
const moduleZip = ref('')
const apps = ref<string[]>([])
const selectedApps = ref<string[]>([])
const filterType = ref('all')
const searchText = ref('')

async function selectApk() {
  const file = await open({
    filters: [{ name: 'APK', extensions: ['apk'] }]
  })
  if (file && typeof file === 'string') {
    apkPath.value = file
  }
}

async function selectModuleZip() {
  const file = await open({
    filters: [{ name: 'Module', extensions: ['zip'] }]
  })
  if (file && typeof file === 'string') {
    moduleZip.value = file
  }
}

async function installApk() {
  if (!apkPath.value || !deviceStore.currentDevice) return
  logStore.command(`adb install ${apkPath.value}`)
  try {
    const result = await adb.install(apkPath.value, deviceStore.currentDevice)
    logStore.success(`安装成功: ${result}`)
  } catch (e: any) {
    logStore.error(`安装失败: ${e}`)
  }
}

async function installModule() {
  if (!moduleZip.value || !deviceStore.currentDevice) return
  const filename = moduleZip.value.split(/[/\\]/).pop()
  logStore.log(`安装模块: ${filename}`)
  try {
    await adb.push(moduleZip.value, `/sdcard/${filename}`, deviceStore.currentDevice)
    await adb.shell(`su -c 'magisk --install-module /sdcard/${filename}'`, deviceStore.currentDevice)
    await adb.shell(`rm /sdcard/${filename}`, deviceStore.currentDevice)
    logStore.success('模块安装成功，重启后生效')
  } catch (e: any) {
    logStore.error(`安装失败: ${e}`)
  }
}

async function loadApps() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取应用列表...')
  
  let cmd = 'pm list packages'
  if (filterType.value === 'user') cmd += ' -3'
  else if (filterType.value === 'system') cmd += ' -s'
  
  try {
    const result = await adb.shell(cmd, deviceStore.currentDevice)
    apps.value = result
      .split('\n')
      .map(line => line.replace('package:', '').trim())
      .filter(pkg => pkg && (!searchText.value || pkg.toLowerCase().includes(searchText.value.toLowerCase())))
      .sort()
    logStore.success(`找到 ${apps.value.length} 个应用`)
  } catch (e) {
    logStore.error('获取失败')
  }
}

function toggleSelect(pkg: string) {
  const idx = selectedApps.value.indexOf(pkg)
  if (idx >= 0) {
    selectedApps.value.splice(idx, 1)
  } else {
    selectedApps.value.push(pkg)
  }
}

function selectAll() {
  selectedApps.value = [...apps.value]
}

function selectNone() {
  selectedApps.value = []
}

async function extractApps() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  // 使用默认输出目录
  const outputDir = await settingsStore.getSubDir('apk')
  
  logStore.log(`提取 ${selectedApps.value.length} 个应用到 ${outputDir}`)
  
  let success = 0, failed = 0
  for (const pkg of selectedApps.value) {
    try {
      const path = await adb.extractApk(pkg, outputDir, deviceStore.currentDevice)
      logStore.success(`${pkg} 已提取: ${path}`)
      success++
    } catch (e: any) {
      logStore.error(`${pkg} 提取失败: ${e}`)
      failed++
    }
  }
  
  logStore.log(`提取完成: 成功 ${success}, 失败 ${failed}`)
  selectedApps.value = []
}

async function uninstallApps() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedApps.value) {
    logStore.command(`pm uninstall ${pkg}`)
    try {
      await adb.uninstall(pkg, deviceStore.currentDevice)
      logStore.success(`${pkg} 已卸载`)
    } catch (e) {
      logStore.error(`${pkg} 卸载失败`)
    }
  }
  selectedApps.value = []
  loadApps()
}

async function disableApps() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedApps.value) {
    try {
      await adb.shell(`pm disable-user --user 0 ${pkg}`, deviceStore.currentDevice)
      logStore.success(`${pkg} 已禁用`)
    } catch (e) {
      logStore.error(`${pkg} 禁用失败`)
    }
  }
  selectedApps.value = []
  loadApps()
}

async function enableApps() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedApps.value) {
    try {
      await adb.shell(`pm enable ${pkg}`, deviceStore.currentDevice)
      logStore.success(`${pkg} 已启用`)
    } catch (e) {
      logStore.error(`${pkg} 启用失败`)
    }
  }
  selectedApps.value = []
  loadApps()
}

async function clearData() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedApps.value) {
    try {
      await adb.shell(`pm clear ${pkg}`, deviceStore.currentDevice)
      logStore.success(`${pkg} 数据已清除`)
    } catch (e) {
      logStore.error(`${pkg} 清除失败`)
    }
  }
  selectedApps.value = []
}

async function forceStop() {
  if (selectedApps.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedApps.value) {
    try {
      await adb.shell(`am force-stop ${pkg}`, deviceStore.currentDevice)
      logStore.success(`${pkg} 已停止`)
    } catch (e) {
      logStore.error(`${pkg} 停止失败`)
    }
  }
  selectedApps.value = []
}
</script>

<template>
  <div class="apps-view">
    <div class="card">
      <div class="card-title">📦 安装APK</div>
      <div class="flex flex-gap">
        <input v-model="apkPath" class="input" placeholder="选择APK文件..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectApk">浏览</button>
        <button class="btn btn-ghost" @click="installApk">安装</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔧 Root模块安装</div>
      <div class="flex flex-gap">
        <input v-model="moduleZip" class="input" placeholder="选择模块zip..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectModuleZip">浏览</button>
        <button class="btn btn-ghost" @click="installModule">安装</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📱 应用列表</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <select v-model="filterType" class="input" @change="loadApps" style="width:120px">
          <option value="all">全部应用</option>
          <option value="user">第三方</option>
          <option value="system">系统应用</option>
        </select>
        <input v-model="searchText" class="input" placeholder="搜索包名..." style="flex:1" @input="loadApps" />
        <button class="btn btn-ghost" @click="loadApps">🔄 刷新</button>
        <button class="btn btn-ghost" @click="selectAll">全选</button>
        <button class="btn btn-ghost" @click="selectNone">取消</button>
      </div>
      
      <div class="app-list">
        <div 
          v-for="pkg in apps" 
          :key="pkg"
          class="app-item"
          :class="{ selected: selectedApps.includes(pkg) }"
          @click="toggleSelect(pkg)"
        >
          <span class="checkbox">{{ selectedApps.includes(pkg) ? '☑' : '☐' }}</span>
          <span class="pkg-name">{{ pkg }}</span>
        </div>
        <div v-if="apps.length === 0" class="empty">
          点击刷新加载应用列表
        </div>
      </div>
      
      <div class="action-bar">
        <button class="btn btn-ghost" @click="extractApps" :disabled="selectedApps.length === 0">
          📤 提取 ({{ selectedApps.length }})
        </button>
        <button class="btn btn-ghost" @click="disableApps" :disabled="selectedApps.length === 0">
          🚫 禁用
        </button>
        <button class="btn btn-ghost" @click="enableApps" :disabled="selectedApps.length === 0">
          ✅ 启用
        </button>
        <button class="btn btn-ghost" @click="clearData" :disabled="selectedApps.length === 0">
          🗑️ 清数据
        </button>
        <button class="btn btn-ghost" @click="forceStop" :disabled="selectedApps.length === 0">
          ⏹️ 停止
        </button>
        <button class="btn btn-danger" @click="uninstallApps" :disabled="selectedApps.length === 0">
          🗑️ 卸载
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.app-list {
  max-height: 300px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 8px;
}

.app-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  transition: background 0.2s;
  
  &:hover {
    background: var(--bg-hover);
  }
  
  &.selected {
    background: rgba(79, 140, 255, 0.15);
  }
  
  &:last-child {
    border-bottom: none;
  }
  
  .checkbox {
    color: var(--accent);
  }
  
  .pkg-name {
    font-size: 13px;
    font-family: monospace;
  }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
}

.action-bar {
  display: flex;
  gap: 10px;
  margin-top: 12px;
  flex-wrap: wrap;
}
</style>
