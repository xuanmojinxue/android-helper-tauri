<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const securityResult = ref('')

async function fullCheck() {
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  
  logStore.log('开始安全检测...')
  securityResult.value = '🔍 安全检测中...\n\n'
  
  try {
    // Verified Boot
    const vb = await adb.shell('getprop ro.boot.verifiedbootstate', deviceStore.currentDevice)
    securityResult.value += `🔐 Verified Boot: ${vb.trim() || 'unknown'}\n`
    
    // SELinux
    const se = await adb.shell('getenforce', deviceStore.currentDevice)
    securityResult.value += `🛡️ SELinux: ${se.trim() || 'unknown'}\n`
    
    // Root检测
    const root = await adb.shell("su -c 'id' 2>/dev/null", deviceStore.currentDevice)
    const hasRoot = root.includes('uid=0')
    securityResult.value += `🔓 Root: ${hasRoot ? '是' : '否'}\n`
    
    // 调试模式
    const debug = await adb.shell('getprop ro.debuggable', deviceStore.currentDevice)
    securityResult.value += `🐛 调试模式: ${debug.trim() === '1' ? '开启' : '关闭'}\n`
    
    // USB调试
    const usb = await adb.shell('settings get global adb_enabled', deviceStore.currentDevice)
    securityResult.value += `🔌 USB调试: ${usb.trim() === '1' ? '开启' : '关闭'}\n`
    
    // 开发者选项
    const dev = await adb.shell('settings get global development_settings_enabled', deviceStore.currentDevice)
    securityResult.value += `⚙️ 开发者选项: ${dev.trim() === '1' ? '开启' : '关闭'}\n`
    
    // 安装来源
    const unknown = await adb.shell('settings get secure install_non_market_apps', deviceStore.currentDevice)
    securityResult.value += `📦 未知来源: ${unknown.trim() === '1' ? '允许' : '禁止'}\n`
    
    // 加密状态
    const encrypt = await adb.shell('getprop ro.crypto.state', deviceStore.currentDevice)
    securityResult.value += `🔒 加密状态: ${encrypt.trim() || 'unknown'}\n`
    
    // Bootloader状态
    const bl = await adb.shell('getprop ro.boot.flash.locked', deviceStore.currentDevice)
    securityResult.value += `🔓 Bootloader: ${bl.trim() === '1' ? '已锁定' : '已解锁'}\n`
    
    securityResult.value += '\n✅ 检测完成'
    logStore.success('安全检测完成')
  } catch (e: any) {
    securityResult.value += `\n❌ 检测出错: ${e}\n`
    logStore.error(`检测失败: ${e}`)
  }
}

async function quickCheck(cmd: string, label: string) {
  if (!deviceStore.currentDevice) return
  logStore.command(cmd)
  try {
    const result = await adb.shell(cmd, deviceStore.currentDevice)
    logStore.log(`${label}: ${result.trim()}`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function setSELinux(mode: string) {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell(`su -c 'setenforce ${mode}'`, deviceStore.currentDevice)
    logStore.success(`SELinux已设置为 ${mode === '0' ? 'Permissive' : 'Enforcing'}`)
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}
</script>

<template>
  <div class="security-view">
    <div class="card">
      <div class="card-title">🛡️ 安全检测</div>
      <pre class="security-result">{{ securityResult || '点击"全面检测"开始...' }}</pre>
      <button class="btn btn-ghost" @click="fullCheck" style="margin-top:12px">
        🔍 全面检测
      </button>
    </div>
    
    <div class="card">
      <div class="card-title">🔧 快捷检测</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="quickCheck('getenforce', 'SELinux')">SELinux状态</button>
        <button class="btn btn-ghost" @click="quickCheck('su -c id', 'Root')">Root检测</button>
        <button class="btn btn-ghost" @click="quickCheck('getprop ro.boot.verifiedbootstate', 'VB')">Verified Boot</button>
        <button class="btn btn-ghost" @click="quickCheck('getprop ro.debuggable', '调试')">调试模式</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">⚙️ SELinux控制</div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="setSELinux('0')">设为 Permissive</button>
        <button class="btn btn-ghost" @click="setSELinux('1')">设为 Enforcing</button>
      </div>
      <p class="hint">⚠️ 需要Root权限，重启后恢复</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.security-result {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 13px;
  min-height: 200px;
  max-height: 300px;
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
