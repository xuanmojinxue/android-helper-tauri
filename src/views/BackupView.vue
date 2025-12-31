<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { useSettingsStore } from '@/stores/settings'
import { adb, fastboot } from '@/utils/adb'
import { open, confirm } from '@tauri-apps/plugin-dialog'

const deviceStore = useDeviceStore()
const logStore = useLogStore()
const settingsStore = useSettingsStore()

const backupDir = ref('')
const otaFile = ref('')
const currentSlot = ref('')

const partitions = ['boot', 'init_boot', 'recovery', 'vbmeta', 'dtbo', 'vendor_boot']

onMounted(async () => {
  // 默认使用统一的备份目录
  backupDir.value = await settingsStore.getSubDir('backup')
})

async function selectBackupDir() {
  const dir = await open({ directory: true, title: '选择备份目录' })
  if (dir && typeof dir === 'string') {
    backupDir.value = dir
  }
}

async function selectOtaFile() {
  const file = await open({ filters: [{ name: 'OTA', extensions: ['zip'] }] })
  if (file && typeof file === 'string') {
    otaFile.value = file
  }
}

async function backupPartition(part: string) {
  if (!deviceStore.currentDevice || !backupDir.value) {
    logStore.error('请先选择备份目录')
    return
  }
  
  logStore.log(`备份 ${part} 分区...`)
  try {
    await adb.shell(`su -c 'dd if=/dev/block/by-name/${part} of=/sdcard/${part}.img'`, deviceStore.currentDevice)
    await adb.pull(`/sdcard/${part}.img`, `${backupDir.value}/${part}.img`, deviceStore.currentDevice)
    await adb.shell(`rm /sdcard/${part}.img`, deviceStore.currentDevice)
    logStore.success(`${part} 已备份到 ${backupDir.value}`)
  } catch (e: any) {
    logStore.error(`备份失败: ${e}`)
  }
}

async function backupAll() {
  if (!backupDir.value) {
    logStore.error('请先选择备份目录')
    return
  }
  for (const part of partitions) {
    await backupPartition(part)
  }
}

async function factoryReset() {
  const confirmed = await confirm('恢复出厂设置会清除所有数据！确定继续？', {
    title: '警告',
    kind: 'warning'
  })
  if (!confirmed || !deviceStore.currentDevice) return
  
  logStore.log('执行恢复出厂...')
  try {
    await adb.shell('am broadcast -a android.intent.action.MASTER_CLEAR', deviceStore.currentDevice)
    logStore.success('恢复出厂命令已发送')
  } catch (e: any) {
    logStore.error(`操作失败: ${e}`)
  }
}

async function sideloadOta() {
  if (!otaFile.value) {
    logStore.error('请先选择OTA包')
    return
  }
  
  logStore.log(`Sideload: ${otaFile.value}`)
  try {
    const result = await adb.sideload(otaFile.value, deviceStore.currentDevice || undefined)
    logStore.success(result)
  } catch (e: any) {
    logStore.error(`Sideload失败: ${e}`)
  }
}

async function enterSideload() {
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  try {
    await adb.reboot('sideload', deviceStore.currentDevice)
    logStore.success('正在进入Sideload模式...')
  } catch (e: any) {
    logStore.error(`进入失败: ${e}`)
  }
}

async function checkSlot() {
  if (!deviceStore.currentDevice) {
    try {
      const result = await fastboot.getVar('current-slot')
      currentSlot.value = result.trim() || '未知'
      logStore.log(`当前槽位: ${currentSlot.value}`)
    } catch {
      logStore.error('获取槽位失败')
    }
    return
  }
  
  try {
    const slot = await adb.shell('getprop ro.boot.slot_suffix', deviceStore.currentDevice)
    currentSlot.value = slot.trim() || '非A/B分区设备'
    logStore.log(`当前槽位: ${currentSlot.value}`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function switchSlot() {
  logStore.log('切换槽位...')
  try {
    await fastboot.setActive('other')
    logStore.success('槽位切换命令已发送')
    checkSlot()
  } catch (e: any) {
    logStore.error(`切换失败: ${e}`)
  }
}

async function setSlotA() {
  try {
    await fastboot.setActive('a')
    logStore.success('已设置为槽位A')
    checkSlot()
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function setSlotB() {
  try {
    await fastboot.setActive('b')
    logStore.success('已设置为槽位B')
    checkSlot()
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function eraseCache() {
  logStore.log('清除缓存分区...')
  try {
    await fastboot.erase('cache')
    logStore.success('缓存已清除')
  } catch (e: any) {
    logStore.error(`清除失败: ${e}`)
  }
}
</script>

<template>
  <div class="backup-view">
    <div class="card">
      <div class="card-title">💾 备份分区</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="backupDir" class="input" placeholder="选择备份目录..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectBackupDir">浏览</button>
        <button class="btn btn-ghost" @click="backupAll">备份全部</button>
      </div>
      <div class="grid grid-3">
        <button 
          v-for="part in partitions" 
          :key="part"
          class="btn btn-ghost"
          @click="backupPartition(part)"
        >
          备份 {{ part }}
        </button>
      </div>
      <p class="hint">⚠️ 需要Root权限</p>
    </div>
    
    <div class="card">
      <div class="card-title">🔄 OTA更新</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="otaFile" class="input" placeholder="选择OTA包..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectOtaFile">浏览</button>
      </div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="enterSideload">进入Sideload模式</button>
        <button class="btn btn-ghost" @click="sideloadOta">Sideload刷入</button>
      </div>
      <p class="hint">💡 设备需要在Recovery的Sideload模式下</p>
    </div>
    
    <div class="card">
      <div class="card-title">🔀 A/B分区</div>
      <div class="slot-info" v-if="currentSlot">
        当前槽位: <span class="slot-value">{{ currentSlot }}</span>
      </div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="checkSlot">查看槽位</button>
        <button class="btn btn-ghost" @click="switchSlot">切换槽位</button>
        <button class="btn btn-ghost" @click="setSlotA">设为槽位A</button>
        <button class="btn btn-ghost" @click="setSlotB">设为槽位B</button>
      </div>
      <p class="hint">💡 槽位操作需要在Fastboot模式下</p>
    </div>
    
    <div class="card">
      <div class="card-title">🔄 恢复/重置</div>
      <div class="grid grid-2">
        <button class="btn btn-danger" @click="factoryReset">恢复出厂设置</button>
        <button class="btn btn-ghost" @click="eraseCache">清除缓存 (Fastboot)</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.grid-3 {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.hint {
  margin-top: 10px;
  font-size: 12px;
  color: var(--text-secondary);
}

.slot-info {
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 12px;
  
  .slot-value {
    font-weight: 600;
    color: var(--accent);
  }
}
</style>
