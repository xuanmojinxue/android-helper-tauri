<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb, fastboot } from '@/utils/adb'
import { open, confirm } from '@tauri-apps/plugin-dialog'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const imgPath = ref('')
const partition = ref('boot')
const showPartitionMenu = ref(false)

const partitions = ['boot', 'init_boot', 'recovery', 'vbmeta', 'system', 'vendor', 'vendor_boot', 'dtbo']

async function selectImage() {
  const file = await open({
    filters: [{ name: 'Image', extensions: ['img'] }]
  })
  if (file && typeof file === 'string') {
    imgPath.value = file
  }
}

async function checkFastboot() {
  logStore.command('fastboot devices')
  try {
    const result = await fastboot.devices()
    logStore.log(result || '无Fastboot设备')
  } catch (e: any) {
    logStore.error(`检测失败: ${e}`)
  }
}

async function flashImage() {
  if (!imgPath.value) {
    logStore.error('请先选择镜像文件')
    return
  }
  logStore.command(`fastboot flash ${partition.value} ${imgPath.value}`)
  try {
    const result = await fastboot.flash(partition.value, imgPath.value)
    logStore.success(`刷入成功: ${result}`)
  } catch (e: any) {
    logStore.error(`刷入失败: ${e}`)
  }
}

async function flashVbmeta() {
  if (!imgPath.value) {
    logStore.error('请先选择vbmeta.img')
    return
  }
  logStore.command('fastboot flash vbmeta --disable-verity --disable-verification')
  try {
    // 需要在 Rust 后端添加特殊处理
    await adb.shell('echo "请手动执行: fastboot flash vbmeta --disable-verity --disable-verification vbmeta.img"')
    logStore.log('请在命令行手动执行带参数的vbmeta刷入')
  } catch (e: any) {
    logStore.error(`操作失败: ${e}`)
  }
}

async function fbReboot(mode?: string) {
  logStore.command(`fastboot reboot ${mode || ''}`)
  try {
    await fastboot.reboot(mode)
    logStore.success('重启命令已发送')
  } catch (e: any) {
    logStore.error(`重启失败: ${e}`)
  }
}

async function unlockBl() {
  const confirmed = await confirm('解锁Bootloader会清除所有数据！确定继续？', {
    title: '警告',
    kind: 'warning'
  })
  if (!confirmed) return
  logStore.command('fastboot flashing unlock')
  try {
    await fastboot.unlock()
    logStore.success('解锁命令已发送，请在设备上确认')
  } catch (e: any) {
    logStore.error(`解锁失败: ${e}`)
  }
}

async function erasePartition(part: string) {
  const confirmed = await confirm(`确定要擦除 ${part} 分区吗？`, {
    title: '确认',
    kind: 'warning'
  })
  if (!confirmed) return
  logStore.command(`fastboot erase ${part}`)
  try {
    await fastboot.erase(part)
    logStore.success(`${part} 已擦除`)
  } catch (e: any) {
    logStore.error(`擦除失败: ${e}`)
  }
}

async function getVar(variable: string) {
  logStore.command(`fastboot getvar ${variable}`)
  try {
    const result = await fastboot.getVar(variable)
    logStore.log(result)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}
</script>

<template>
  <div class="flash-view">
    <div class="card">
      <div class="card-title">⚡ Fastboot操作</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="checkFastboot">检测设备</button>
        <button class="btn btn-ghost" @click="fbReboot()">重启系统</button>
        <button class="btn btn-ghost" @click="fbReboot('recovery')">重启Recovery</button>
        <button class="btn btn-ghost" @click="fbReboot('fastboot')">重启Fastbootd</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔓 Bootloader</div>
      <div class="grid grid-2">
        <button class="btn btn-danger" @click="unlockBl">🔓 解锁 Bootloader</button>
        <button class="btn btn-ghost" @click="getVar('unlocked')">检查解锁状态</button>
      </div>
      <p class="hint">⚠️ 解锁会清除所有数据</p>
    </div>
    
    <div class="card">
      <div class="card-title">💾 刷入镜像</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <div class="partition-select">
          <div class="partition-current" @click="showPartitionMenu = !showPartitionMenu">
            {{ partition }}
            <span class="arrow">▼</span>
          </div>
          <div class="partition-menu" v-show="showPartitionMenu">
            <div 
              v-for="p in partitions" 
              :key="p" 
              class="partition-option"
              :class="{ active: partition === p }"
              @click="partition = p; showPartitionMenu = false"
            >
              {{ p }}
            </div>
          </div>
        </div>
        <input v-model="imgPath" class="input" placeholder="选择镜像文件..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectImage">浏览</button>
      </div>
      <div class="grid grid-2">
        <button class="btn btn-ghost" @click="flashImage">🔥 刷入</button>
        <button class="btn btn-ghost" @click="flashVbmeta">刷vbmeta(禁验证)</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔧 ADB模式切换</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="adb.reboot('bootloader', deviceStore.currentDevice!)">
          进入Fastboot
        </button>
        <button class="btn btn-ghost" @click="adb.reboot('recovery', deviceStore.currentDevice!)">
          进入Recovery
        </button>
        <button class="btn btn-ghost" @click="adb.reboot('sideload', deviceStore.currentDevice!)">
          进入Sideload
        </button>
        <button class="btn btn-danger" @click="adb.reboot('edl', deviceStore.currentDevice!)">
          进入9008
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🗑️ 擦除分区</div>
      <div class="grid grid-4">
        <button class="btn btn-danger" @click="erasePartition('cache')">擦除cache</button>
        <button class="btn btn-danger" @click="erasePartition('userdata')">擦除userdata</button>
        <button class="btn btn-danger" @click="erasePartition('metadata')">擦除metadata</button>
        <button class="btn btn-ghost" @click="getVar('all')">查看所有变量</button>
      </div>
    </div>
    
    <div class="warning-box">
      ⚠️ 刷机有风险，操作需谨慎！请确保镜像文件与设备匹配。
    </div>
  </div>
</template>

<style lang="scss" scoped>
.warning-box {
  padding: 16px;
  background: rgba(255, 152, 0, 0.15);
  border: 1px solid var(--warning);
  border-radius: 8px;
  color: var(--warning);
  text-align: center;
}

.hint {
  margin-top: 10px;
  font-size: 12px;
  color: var(--text-secondary);
}

.partition-select {
  position: relative;
  width: 150px;
}

.partition-current {
  padding: 8px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  
  .arrow {
    font-size: 10px;
    color: var(--text-secondary);
  }
}

.partition-menu {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 6px;
  margin-top: 4px;
  z-index: 100;
  max-height: 200px;
  overflow-y: auto;
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}

.partition-option {
  padding: 8px 12px;
  cursor: pointer;
  
  &:hover {
    background: var(--bg-hover);
  }
  
  &.active {
    background: var(--accent);
    color: white;
  }
}
</style>
