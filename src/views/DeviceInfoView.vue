<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const deviceInfo = ref<Record<string, string>>({})
const basebandInfo = ref<Record<string, string>>({})
const partitions = ref('')
const loading = ref(false)

async function refreshInfo() {
  if (!deviceStore.currentDevice) return
  loading.value = true
  logStore.log('获取设备详情...')
  
  try {
    const props = [
      ['型号', 'ro.product.model'],
      ['品牌', 'ro.product.brand'],
      ['Android', 'ro.build.version.release'],
      ['SDK', 'ro.build.version.sdk'],
      ['指纹', 'ro.build.fingerprint'],
      ['安全补丁', 'ro.build.version.security_patch'],
      ['内核', 'ro.build.kernel.version'],
    ]
    
    for (const [label, prop] of props) {
      try {
        const val = await adb.shell(`getprop ${prop}`, deviceStore.currentDevice)
        deviceInfo.value[label] = val.trim() || '-'
      } catch {
        deviceInfo.value[label] = '-'
      }
    }
    
    // 分辨率
    try {
      const size = await adb.shell('wm size', deviceStore.currentDevice)
      const match = size.match(/(\d+x\d+)/)
      deviceInfo.value['分辨率'] = match ? match[1] : '-'
    } catch {
      deviceInfo.value['分辨率'] = '-'
    }
    
    // 存储
    try {
      const df = await adb.shell('df -h /data | tail -1', deviceStore.currentDevice)
      const parts = df.trim().split(/\s+/)
      if (parts.length >= 4) {
        deviceInfo.value['存储'] = `${parts[2]}/${parts[1]}`
      }
    } catch {
      deviceInfo.value['存储'] = '-'
    }
    
    // 基带信息
    const basebandProps = [
      ['IMEI', 'persist.radio.imei'],
      ['基带版本', 'gsm.version.baseband'],
      ['运营商', 'gsm.operator.alpha'],
    ]
    
    for (const [label, prop] of basebandProps) {
      try {
        const val = await adb.shell(`getprop ${prop}`, deviceStore.currentDevice)
        basebandInfo.value[label] = val.trim() || '-'
      } catch {
        basebandInfo.value[label] = '-'
      }
    }
    
    // 分区信息
    try {
      const parts = await adb.shell("ls /dev/block/by-name/ 2>/dev/null | head -20", deviceStore.currentDevice)
      partitions.value = parts.trim() || '无法获取分区信息'
    } catch {
      partitions.value = '无法获取分区信息'
    }
    
    logStore.success('设备信息已刷新')
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  } finally {
    loading.value = false
  }
}

async function backupBasebandInfo() {
  if (!deviceStore.currentDevice) return
  const info = [
    `设备: ${deviceInfo.value['型号']} (${deviceInfo.value['品牌']})`,
    `Android: ${deviceInfo.value['Android']}`,
    `IMEI: ${basebandInfo.value['IMEI']}`,
    `基带: ${basebandInfo.value['基带版本']}`,
    `指纹: ${deviceInfo.value['指纹']}`,
    `时间: ${new Date().toLocaleString()}`
  ].join('\n')
  
  await navigator.clipboard.writeText(info)
  logStore.success('基带信息已复制到剪贴板')
}

onMounted(() => {
  if (deviceStore.currentDevice) {
    refreshInfo()
  }
})
</script>

<template>
  <div class="device-info-view">
    <div class="card">
      <div class="card-title">📱 设备信息</div>
      <div class="info-grid">
        <div v-for="(val, key) in deviceInfo" :key="key" class="info-item">
          <span class="label">{{ key }}</span>
          <span class="value">{{ val }}</span>
        </div>
      </div>
      <button class="btn btn-ghost" @click="refreshInfo" :disabled="loading" style="margin-top:12px">
        🔄 刷新信息
      </button>
    </div>
    
    <div class="card">
      <div class="card-title">📡 基带/IMEI信息</div>
      <div class="info-grid">
        <div v-for="(val, key) in basebandInfo" :key="key" class="info-item">
          <span class="label">{{ key }}</span>
          <span class="value">{{ val }}</span>
        </div>
      </div>
      <button class="btn btn-ghost" @click="backupBasebandInfo" style="margin-top:12px">
        📋 备份基带信息
      </button>
    </div>
    
    <div class="card">
      <div class="card-title">📂 分区信息</div>
      <pre class="partition-list">{{ partitions }}</pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border-radius: 8px;
  
  .label {
    color: var(--text-secondary);
    font-size: 13px;
  }
  
  .value {
    font-weight: 500;
    font-size: 13px;
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.partition-list {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  max-height: 150px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
