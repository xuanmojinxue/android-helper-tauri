<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const proxyIp = ref('')
const proxyPort = ref(8888)
const pingTarget = ref('8.8.8.8')
const netResult = ref('')

async function setProxy() {
  if (!proxyIp.value || !deviceStore.currentDevice) return
  try {
    await adb.shell(`settings put global http_proxy ${proxyIp.value}:${proxyPort.value}`, deviceStore.currentDevice)
    logStore.success(`代理已设置: ${proxyIp.value}:${proxyPort.value}`)
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function clearProxy() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell('settings put global http_proxy :0', deviceStore.currentDevice)
    logStore.success('代理已清除')
  } catch (e: any) {
    logStore.error(`清除失败: ${e}`)
  }
}

async function doPing() {
  if (!deviceStore.currentDevice) return
  netResult.value = `Ping ${pingTarget.value}...\n`
  try {
    const result = await adb.shell(`ping -c 4 ${pingTarget.value}`, deviceStore.currentDevice)
    netResult.value += result
    logStore.log(result)
  } catch (e: any) {
    netResult.value += `Ping失败: ${e}`
    logStore.error(`Ping失败: ${e}`)
  }
}

async function checkWifi() {
  if (!deviceStore.currentDevice) return
  netResult.value = '📶 WiFi状态:\n'
  try {
    const result = await adb.shell("dumpsys wifi | grep -E 'Wi-Fi is|mNetworkInfo'", deviceStore.currentDevice)
    netResult.value += result || '无法获取'
  } catch (e: any) {
    netResult.value += `获取失败: ${e}`
  }
}

async function checkIp() {
  if (!deviceStore.currentDevice) return
  netResult.value = '🌐 IP地址:\n'
  try {
    const result = await adb.shell("ip addr show wlan0 | grep 'inet '", deviceStore.currentDevice)
    netResult.value += result || '未获取到IP'
  } catch (e: any) {
    netResult.value += `获取失败: ${e}`
  }
}

async function checkDns() {
  if (!deviceStore.currentDevice) return
  netResult.value = '🔤 DNS服务器:\n'
  try {
    const dns1 = await adb.shell('getprop net.dns1', deviceStore.currentDevice)
    const dns2 = await adb.shell('getprop net.dns2', deviceStore.currentDevice)
    netResult.value += `DNS1: ${dns1.trim() || '-'}\nDNS2: ${dns2.trim() || '-'}`
  } catch (e: any) {
    netResult.value += `获取失败: ${e}`
  }
}

async function checkGateway() {
  if (!deviceStore.currentDevice) return
  netResult.value = '🚪 网关:\n'
  try {
    const result = await adb.shell('ip route | grep default', deviceStore.currentDevice)
    netResult.value += result || '未获取到网关'
  } catch (e: any) {
    netResult.value += `获取失败: ${e}`
  }
}
</script>

<template>
  <div class="network-view">
    <div class="card">
      <div class="card-title">🌐 WiFi代理</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="proxyIp" class="input" placeholder="代理IP (如 192.168.1.100)" style="flex:1" />
        <input v-model="proxyPort" class="input" type="number" style="width:100px" />
      </div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="setProxy">设置代理</button>
        <button class="btn btn-ghost" @click="clearProxy">清除代理</button>
      </div>
      <p class="hint">💡 用于Charles/Fiddler抓包</p>
    </div>
    
    <div class="card">
      <div class="card-title">📶 网络诊断</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="pingTarget" class="input" placeholder="目标地址" style="flex:1" />
        <button class="btn btn-ghost" @click="doPing">Ping</button>
      </div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="checkWifi">WiFi状态</button>
        <button class="btn btn-ghost" @click="checkIp">IP地址</button>
        <button class="btn btn-ghost" @click="checkDns">DNS</button>
        <button class="btn btn-ghost" @click="checkGateway">网关</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📋 输出</div>
      <pre class="net-result">{{ netResult || '点击上方按钮查看网络信息' }}</pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.hint {
  margin-top: 10px;
  font-size: 12px;
  color: var(--text-secondary);
}

.net-result {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  min-height: 150px;
  max-height: 300px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
