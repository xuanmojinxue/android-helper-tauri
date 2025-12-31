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

const rootInfo = ref('')
const rootModules = ref<{id: string, name: string, version: string, disabled: boolean}[]>([])
const lspModules = ref<{pkg: string, name: string}[]>([])
const selectedModules = ref<string[]>([])
const selectedLspModules = ref<string[]>([])
const moduleZip = ref('')

async function detectRoot() {
  if (!deviceStore.currentDevice) return
  logStore.log('检测Root方案...')
  
  const info: string[] = []
  
  try {
    const ksu = await adb.shell("su -c 'cat /data/adb/ksu/version 2>/dev/null'", deviceStore.currentDevice)
    if (ksu.trim()) info.push(`✅ KernelSU: v${ksu.trim()}`)
  } catch {}
  
  try {
    const ap = await adb.shell("su -c 'cat /data/adb/ap/version 2>/dev/null'", deviceStore.currentDevice)
    if (ap.trim()) info.push(`✅ APatch: v${ap.trim()}`)
  } catch {}
  
  try {
    const magisk = await adb.shell("su -c 'magisk -v 2>/dev/null'", deviceStore.currentDevice)
    if (magisk.trim() && !magisk.includes('not found')) info.push(`✅ Magisk: ${magisk.trim()}`)
  } catch {}
  
  try {
    const sukisu = await adb.shell('pm path com.sukisu.ultra', deviceStore.currentDevice)
    if (sukisu.trim()) info.push('✅ SukiSU Ultra: 已安装')
  } catch {}
  
  try {
    const zygisk = await adb.shell("su -c 'cat /data/adb/modules/zygisksu/module.prop 2>/dev/null | grep ^version'", deviceStore.currentDevice)
    if (zygisk.trim()) info.push(`✅ Zygisk: ${zygisk.trim().replace('version=','')}`)
  } catch {}
  
  try {
    const lsp = await adb.shell("su -c 'cat /data/adb/modules/zygisk_lsposed/module.prop 2>/dev/null | grep ^version'", deviceStore.currentDevice)
    if (lsp.trim()) info.push(`✅ LSPosed: ${lsp.trim().replace('version=','')}`)
  } catch {}
  
  rootInfo.value = info.length > 0 ? info.join('\n') : '❌ 未检测到Root'
  logStore.log(rootInfo.value)
}

async function openRootManager() {
  if (!deviceStore.currentDevice) return
  const managers = ['com.sukisu.ultra', 'me.weishu.kernelsu', 'me.bmax.apatch', 'com.topjohnwu.magisk']
  
  for (const pkg of managers) {
    try {
      const path = await adb.shell(`pm path ${pkg}`, deviceStore.currentDevice)
      if (path.trim()) {
        await adb.shell(`monkey -p ${pkg} -c android.intent.category.LAUNCHER 1`, deviceStore.currentDevice)
        logStore.success(`已打开 ${pkg}`)
        return
      }
    } catch {}
  }
  logStore.error('未找到Root管理器')
}

async function loadModules() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取Root模块...')
  rootModules.value = []
  
  try {
    const result = await adb.shell("su -c 'ls -1 /data/adb/modules/ 2>/dev/null'", deviceStore.currentDevice)
    
    for (const id of result.split('\n').filter(m => m.trim() && !m.startsWith('.'))) {
      const prop = await adb.shell(`su -c 'cat /data/adb/modules/${id.trim()}/module.prop 2>/dev/null'`, deviceStore.currentDevice)
      let name = id.trim(), version = ''
      
      for (const line of prop.split('\n')) {
        if (line.startsWith('name=')) name = line.slice(5).trim()
        else if (line.startsWith('version=')) version = line.slice(8).trim()
      }
      
      const disabled = (await adb.shell(`su -c 'test -f /data/adb/modules/${id.trim()}/disable && echo 1'`, deviceStore.currentDevice)).includes('1')
      rootModules.value.push({ id: id.trim(), name, version, disabled })
    }
    
    logStore.success(`找到 ${rootModules.value.length} 个模块`)
  } catch (e) {
    logStore.error('获取模块失败')
  }
}

async function loadLspModules() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取LSPosed模块...')
  lspModules.value = []
  
  const keywords = ['xposed','lsp','hook','lucky','tool','hide','hma','fake','vanced']
  
  try {
    const result = await adb.shell('pm list packages -3', deviceStore.currentDevice)
    
    for (const line of result.split('\n')) {
      const pkg = line.replace('package:', '').trim()
      if (pkg && keywords.some(k => pkg.toLowerCase().includes(k))) {
        lspModules.value.push({ pkg, name: pkg.split('.').pop() || pkg })
      }
    }
    
    logStore.success(`找到 ${lspModules.value.length} 个LSPosed模块`)
  } catch (e) {
    logStore.error('获取失败')
  }
}

function toggleModule(id: string) {
  const idx = selectedModules.value.indexOf(id)
  if (idx >= 0) selectedModules.value.splice(idx, 1)
  else selectedModules.value.push(id)
}

function toggleLspModule(pkg: string) {
  const idx = selectedLspModules.value.indexOf(pkg)
  if (idx >= 0) selectedLspModules.value.splice(idx, 1)
  else selectedLspModules.value.push(pkg)
}

async function deleteModules() {
  if (selectedModules.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const id of selectedModules.value) {
    await adb.shell(`su -c 'rm -rf /data/adb/modules/${id}'`, deviceStore.currentDevice)
    logStore.success(`${id} 已删除`)
  }
  selectedModules.value = []
  loadModules()
}

async function extractModules() {
  if (selectedModules.value.length === 0 || !deviceStore.currentDevice) return
  
  const saveDir = await settingsStore.getSubDir('module')
  
  for (const id of selectedModules.value) {
    logStore.log(`提取模块: ${id}`)
    try {
      await adb.shell(`su -c 'cd /data/adb/modules && zip -r /sdcard/${id}.zip ${id}'`, deviceStore.currentDevice)
      await adb.pull(`/sdcard/${id}.zip`, `${saveDir}/${id}.zip`, deviceStore.currentDevice)
      await adb.shell(`rm /sdcard/${id}.zip`, deviceStore.currentDevice)
      logStore.success(`${id} 已提取到 ${saveDir}`)
    } catch (e: any) {
      logStore.error(`${id} 提取失败: ${e}`)
    }
  }
  selectedModules.value = []
}

async function extractLspModules() {
  if (selectedLspModules.value.length === 0 || !deviceStore.currentDevice) return
  
  const saveDir = await settingsStore.getSubDir('apk')
  
  for (const pkg of selectedLspModules.value) {
    logStore.log(`提取: ${pkg}`)
    try {
      await adb.extractApk(pkg, saveDir, deviceStore.currentDevice)
      logStore.success(`${pkg} 已提取到 ${saveDir}`)
    } catch (e: any) {
      logStore.error(`${pkg} 提取失败: ${e}`)
    }
  }
  selectedLspModules.value = []
}

async function uninstallLspModules() {
  if (selectedLspModules.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const pkg of selectedLspModules.value) {
    try {
      await adb.uninstall(pkg, deviceStore.currentDevice)
      logStore.success(`${pkg} 已卸载`)
    } catch (e: any) {
      logStore.error(`${pkg} 卸载失败: ${e}`)
    }
  }
  selectedLspModules.value = []
  loadLspModules()
}

async function selectModuleZip() {
  const file = await open({ filters: [{ name: 'Module', extensions: ['zip'] }] })
  if (file && typeof file === 'string') {
    moduleZip.value = file
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
    loadModules()
  } catch (e: any) {
    logStore.error(`安装失败: ${e}`)
  }
}

async function updateModules() {
  logStore.log('检查模块更新...')
  logStore.log('请在Root管理器中检查更新')
  openRootManager()
}
</script>

<template>
  <div class="root-view">
    <div class="card">
      <div class="card-title">⚡ 快捷操作</div>
      <div class="grid grid-3">
        <button class="btn btn-ghost" @click="detectRoot">🔍 检测Root</button>
        <button class="btn btn-ghost" @click="openRootManager">打开管理器</button>
        <button class="btn btn-ghost" @click="adb.shell('getenforce', deviceStore.currentDevice!)">SELinux状态</button>
      </div>
      <div v-if="rootInfo" class="root-info">{{ rootInfo }}</div>
    </div>
    
    <div class="card">
      <div class="card-title">📦 安装模块</div>
      <div class="flex flex-gap">
        <input v-model="moduleZip" class="input" placeholder="选择模块zip..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectModuleZip">浏览</button>
        <button class="btn btn-ghost" @click="installModule">安装</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔧 Root模块</div>
      <div class="module-list">
        <div 
          v-for="mod in rootModules" 
          :key="mod.id"
          class="module-item"
          :class="{ selected: selectedModules.includes(mod.id), disabled: mod.disabled }"
          @click="toggleModule(mod.id)"
        >
          <span class="status">{{ mod.disabled ? '🔴' : '🟢' }}</span>
          <span class="name">{{ mod.name }}</span>
          <span class="version">{{ mod.version }}</span>
        </div>
        <div v-if="rootModules.length === 0" class="empty">
          点击读取模块加载列表
        </div>
      </div>
      <div class="grid grid-4" style="margin-top:12px">
        <button class="btn btn-ghost" @click="loadModules">📖 读取模块</button>
        <button class="btn btn-ghost" @click="updateModules">🔄 更新模块</button>
        <button class="btn btn-ghost" @click="extractModules" :disabled="selectedModules.length === 0">
          📤 提取模块
        </button>
        <button class="btn btn-danger" @click="deleteModules" :disabled="selectedModules.length === 0">
          🗑️ 删除模块
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🎯 LSPosed模块</div>
      <div class="module-list">
        <div 
          v-for="mod in lspModules" 
          :key="mod.pkg"
          class="module-item"
          :class="{ selected: selectedLspModules.includes(mod.pkg) }"
          @click="toggleLspModule(mod.pkg)"
        >
          <span class="status">📦</span>
          <span class="name">{{ mod.name }}</span>
          <span class="version">{{ mod.pkg }}</span>
        </div>
        <div v-if="lspModules.length === 0" class="empty">
          点击读取模块加载列表
        </div>
      </div>
      <div class="grid grid-4" style="margin-top:12px">
        <button class="btn btn-ghost" @click="loadLspModules">📖 读取模块</button>
        <button class="btn btn-ghost" @click="updateModules">🔄 更新模块</button>
        <button class="btn btn-ghost" @click="extractLspModules" :disabled="selectedLspModules.length === 0">
          📤 提取模块
        </button>
        <button class="btn btn-danger" @click="uninstallLspModules" :disabled="selectedLspModules.length === 0">
          🗑️ 卸载模块
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.root-info {
  margin-top: 16px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  white-space: pre-line;
  font-size: 13px;
}

.module-list {
  max-height: 200px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 8px;
}

.module-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  transition: background 0.2s;
  
  &:hover { background: var(--bg-hover); }
  &.selected { background: rgba(79, 140, 255, 0.15); }
  &.disabled { opacity: 0.6; }
  &:last-child { border-bottom: none; }
  
  .name { flex: 1; font-size: 13px; }
  .version { color: var(--text-secondary); font-size: 12px; }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
}

.grid-3 {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}
</style>
