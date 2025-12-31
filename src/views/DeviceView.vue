<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const wifiIp = ref('')
const wifiPort = ref(5555)

async function getWifiIp() {
  if (!deviceStore.currentDevice) return
  try {
    const result = await adb.shell("ip addr show wlan0 | grep 'inet '", deviceStore.currentDevice)
    const match = result.match(/inet (\d+\.\d+\.\d+\.\d+)/)
    if (match) {
      wifiIp.value = match[1]
      logStore.success(`获取到IP: ${match[1]}`)
    } else {
      logStore.error('未获取到IP地址')
    }
  } catch (e) {
    logStore.error('获取IP失败')
  }
}

async function startWireless() {
  if (!deviceStore.currentDevice) return
  try {
    await adb.shell('setprop service.adb.tcp.port 5555', deviceStore.currentDevice)
    await adb.shell('stop adbd', deviceStore.currentDevice)
    await adb.shell('start adbd', deviceStore.currentDevice)
    logStore.success('无线ADB已开启，端口: 5555')
  } catch (e) {
    logStore.error('开启失败')
  }
}

async function connectWireless() {
  if (!wifiIp.value) {
    logStore.error('请先输入IP地址')
    return
  }
  try {
    const result = await adb.connect(`${wifiIp.value}:${wifiPort.value}`)
    logStore.log(result)
    deviceStore.refreshDevices()
  } catch (e) {
    logStore.error('连接失败')
  }
}

async function disconnectWireless() {
  if (!wifiIp.value) return
  try {
    await adb.disconnect(`${wifiIp.value}:${wifiPort.value}`)
    logStore.success(`已断开 ${wifiIp.value}`)
    deviceStore.refreshDevices()
  } catch (e) {
    logStore.error('断开失败')
  }
}

async function disconnectAll() {
  try {
    await adb.disconnect()
    logStore.success('已断开所有无线连接')
    deviceStore.refreshDevices()
  } catch (e) {
    logStore.error('断开失败')
  }
}

async function reboot(mode?: string) {
  if (!deviceStore.currentDevice) return
  try {
    await adb.reboot(mode, deviceStore.currentDevice)
    logStore.success(`重启到 ${mode || '系统'}`)
  } catch (e) {
    logStore.error('重启失败')
  }
}
</script>

<template>
  <div class="device-view">
    <div class="card">
      <div class="card-title">📱 设备信息</div>
      <div v-if="deviceStore.deviceInfo" class="device-details">
        <div class="detail-row">
          <span class="label">品牌</span>
          <span class="value">{{ deviceStore.deviceInfo.brand }}</span>
        </div>
        <div class="detail-row">
          <span class="label">型号</span>
          <span class="value">{{ deviceStore.deviceInfo.model }}</span>
        </div>
        <div class="detail-row">
          <span class="label">Android</span>
          <span class="value">{{ deviceStore.deviceInfo.android }}</span>
        </div>
        <div class="detail-row">
          <span class="label">序列号</span>
          <span class="value">{{ deviceStore.deviceInfo.serial }}</span>
        </div>
      </div>
      <div v-else class="no-device">
        请连接设备
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📶 无线ADB</div>
      <div class="form-row">
        <label>设备IP:</label>
        <input v-model="wifiIp" class="input" placeholder="192.168.x.x" style="flex:1;max-width:200px" />
        <button class="btn btn-ghost" @click="getWifiIp">获取IP</button>
      </div>
      <div class="form-row">
        <label>端口:</label>
        <input v-model="wifiPort" class="input" type="number" style="width:100px" />
      </div>
      <div class="grid grid-5" style="margin-top:12px">
        <button class="btn btn-ghost" @click="startWireless">开启无线</button>
        <button class="btn btn-ghost" @click="connectWireless">连接</button>
        <button class="btn btn-ghost" @click="disconnectWireless">断开</button>
        <button class="btn btn-ghost" @click="disconnectAll">断开全部</button>
      </div>
      <div class="hint-box">
        <div class="hint-title">💡 使用说明</div>
        <ol>
          <li>先用USB连接设备</li>
          <li>点击"开启无线"</li>
          <li>点击"获取IP"获取设备IP地址</li>
          <li>拔掉USB线</li>
          <li>点击"连接"通过WiFi连接设备</li>
        </ol>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔄 重启选项</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="reboot()">重启系统</button>
        <button class="btn btn-ghost" @click="reboot('recovery')">Recovery</button>
        <button class="btn btn-ghost" @click="reboot('bootloader')">Bootloader</button>
        <button class="btn btn-danger" @click="reboot('edl')">EDL模式</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.device-details {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border-radius: 8px;
  
  .label {
    color: var(--text-secondary);
  }
  
  .value {
    font-weight: 500;
  }
}

.no-device {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
  
  label {
    width: 70px;
    color: var(--text-secondary);
  }
}

.hint-box {
  margin-top: 16px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  
  .hint-title {
    font-size: 13px;
    font-weight: 500;
    margin-bottom: 8px;
  }
  
  ol {
    margin: 0;
    padding-left: 20px;
    color: var(--text-secondary);
    font-size: 12px;
    line-height: 1.8;
  }
}

.grid-5 {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}
</style>
