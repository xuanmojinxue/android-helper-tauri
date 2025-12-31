<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const proxyIp = ref('')
const proxyPort = ref(8888)
const currentProxy = ref('')

async function getProxy() {
  if (!deviceStore.currentDevice) return
  try {
    const result = await adb.shell('settings get global http_proxy', deviceStore.currentDevice)
    currentProxy.value = result.trim() || '未设置'
    logStore.log(`当前代理: ${currentProxy.value}`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function setProxy() {
  if (!proxyIp.value || !deviceStore.currentDevice) {
    logStore.error('请输入代理IP')
    return
  }
  try {
    await adb.shell(`settings put global http_proxy ${proxyIp.value}:${proxyPort.value}`, deviceStore.currentDevice)
    logStore.success(`代理已设置: ${proxyIp.value}:${proxyPort.value}`)
    getProxy()
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function clearProxy() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell('settings put global http_proxy :0', deviceStore.currentDevice)
    logStore.success('代理已清除')
    getProxy()
  } catch (e: any) {
    logStore.error(`清除失败: ${e}`)
  }
}


</script>

<template>
  <div class="proxy-view">
    <div class="card">
      <div class="card-title">🌐 WiFi代理设置</div>
      
      <div class="current-proxy" v-if="currentProxy">
        当前代理: <span class="proxy-value">{{ currentProxy }}</span>
      </div>
      
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="proxyIp" class="input" placeholder="代理IP (如 192.168.1.100)" style="flex:1" />
        <input v-model.number="proxyPort" class="input" type="number" placeholder="端口" style="width:100px" />
      </div>
      
      <div class="grid grid-3">
        <button class="btn btn-ghost" @click="setProxy">设置代理</button>
        <button class="btn btn-ghost" @click="clearProxy">清除代理</button>
        <button class="btn btn-ghost" @click="getProxy">查看当前</button>
      </div>
    </div>
    
    <div class="card hint-card">
      <div class="card-title">💡 使用说明</div>
      <ol>
        <li>在电脑上启动抓包工具 (Charles/Fiddler/mitmproxy)</li>
        <li>确保手机和电脑在同一WiFi网络</li>
        <li>输入电脑的局域网IP和抓包工具端口</li>
        <li>点击"设置代理"</li>
        <li>在手机上安装抓包工具的CA证书</li>
      </ol>
      <p class="common-ports">
        常用端口: Charles(8888), Fiddler(8866), mitmproxy(8080)
      </p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.current-proxy {
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 12px;
  
  .proxy-value {
    font-weight: 600;
    color: var(--accent);
    font-family: monospace;
  }
}

.hint-card {
  ol {
    margin: 0;
    padding-left: 20px;
    color: var(--text-secondary);
    font-size: 13px;
    line-height: 1.8;
  }
  
  .common-ports {
    margin-top: 12px;
    font-size: 12px;
    color: var(--text-secondary);
    padding: 8px 12px;
    background: var(--bg-secondary);
    border-radius: 6px;
  }
}
</style>
