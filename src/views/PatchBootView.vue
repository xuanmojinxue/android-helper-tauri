<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'
import { open } from '@tauri-apps/plugin-dialog'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const bootFile = ref('')
const patchResult = ref('')

async function selectBootFile() {
  const file = await open({
    filters: [{ name: 'Image', extensions: ['img'] }],
    title: '选择Boot镜像'
  })
  if (file && typeof file === 'string') {
    bootFile.value = file
  }
}

async function extractBootFromDevice() {
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  
  const saveDir = await open({ directory: true, title: '选择保存目录' })
  if (!saveDir || typeof saveDir !== 'string') return
  
  logStore.log('从设备提取Boot镜像...')
  patchResult.value = '正在提取...\n'
  
  try {
    // 获取当前槽位
    const slot = await adb.shell('getprop ro.boot.slot_suffix', deviceStore.currentDevice)
    const slotSuffix = slot.trim() || ''
    
    // 尝试提取boot或init_boot
    for (const part of ['init_boot', 'boot']) {
      const partName = `${part}${slotSuffix}`
      patchResult.value += `尝试提取 ${partName}...\n`
      
      try {
        await adb.shell(`su -c 'dd if=/dev/block/by-name/${partName} of=/sdcard/${part}.img'`, deviceStore.currentDevice)
        await adb.pull(`/sdcard/${part}.img`, `${saveDir}/${part}.img`, deviceStore.currentDevice)
        await adb.shell(`rm /sdcard/${part}.img`, deviceStore.currentDevice)
        
        bootFile.value = `${saveDir}/${part}.img`
        patchResult.value += `✅ ${part}.img 已保存到 ${saveDir}\n`
        logStore.success(`${part}.img 提取成功`)
        return
      } catch (e) {
        patchResult.value += `${partName} 提取失败，尝试下一个...\n`
      }
    }
    
    patchResult.value += '❌ 无法提取Boot镜像\n'
    logStore.error('提取失败')
  } catch (e: any) {
    patchResult.value += `❌ 错误: ${e}\n`
    logStore.error(`提取失败: ${e}`)
  }
}

async function extractBootFromRom() {
  const romFile = await open({
    filters: [
      { name: 'ROM', extensions: ['zip', 'bin'] }
    ],
    title: '选择ROM包或payload.bin'
  })
  if (!romFile || typeof romFile !== 'string') return
  
  patchResult.value = '请使用ROM提取功能提取boot.img\n'
  logStore.log('请前往"ROM提取"页面提取boot镜像')
}

async function patchBoot(method: string) {
  if (!bootFile.value) {
    logStore.error('请先选择Boot镜像')
    return
  }
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  
  const filename = bootFile.value.split(/[/\\]/).pop()
  logStore.log(`使用 ${method} 修补 ${filename}...`)
  patchResult.value = `开始修补...\n方式: ${method}\n`
  
  try {
    // 推送boot镜像到设备
    await adb.push(bootFile.value, '/sdcard/boot.img', deviceStore.currentDevice)
    patchResult.value += '✅ Boot镜像已推送到设备\n'
    
    // 根据不同方式修补
    let patchCmd = ''
    let managerPkg = ''
    
    switch (method) {
      case 'magisk':
        managerPkg = 'com.topjohnwu.magisk'
        patchCmd = `am start -n ${managerPkg}/.ui.MainActivity`
        break
      case 'apatch':
        managerPkg = 'me.bmax.apatch'
        patchCmd = `am start -n ${managerPkg}/.ui.MainActivity`
        break
      case 'ksu':
        managerPkg = 'me.weishu.kernelsu'
        patchCmd = `am start -n ${managerPkg}/.ui.MainActivity`
        break
      case 'sukisu':
        managerPkg = 'com.sukisu.ultra'
        patchCmd = `am start -n ${managerPkg}/.ui.MainActivity`
        break
    }
    
    // 检查管理器是否安装
    const checkPkg = await adb.shell(`pm path ${managerPkg}`, deviceStore.currentDevice)
    if (!checkPkg.trim()) {
      patchResult.value += `❌ 未安装 ${managerPkg}\n`
      patchResult.value += `请先安装对应的Root管理器\n`
      logStore.error(`未安装 ${managerPkg}`)
      return
    }
    
    // 打开管理器
    await adb.shell(patchCmd, deviceStore.currentDevice)
    patchResult.value += `✅ 已打开 ${method} 管理器\n`
    patchResult.value += `\n📋 请在管理器中:\n`
    patchResult.value += `1. 选择"安装"或"修补"\n`
    patchResult.value += `2. 选择 /sdcard/boot.img\n`
    patchResult.value += `3. 等待修补完成\n`
    patchResult.value += `4. 修补后的文件通常在 /sdcard/Download/\n`
    
    logStore.success('请在设备上完成修补操作')
  } catch (e: any) {
    patchResult.value += `❌ 错误: ${e}\n`
    logStore.error(`修补失败: ${e}`)
  }
}

async function pullPatchedBoot() {
  if (!deviceStore.currentDevice) return
  
  const saveDir = await open({ directory: true, title: '选择保存目录' })
  if (!saveDir || typeof saveDir !== 'string') return
  
  logStore.log('拉取修补后的Boot...')
  
  try {
    const files = await adb.shell('ls /sdcard/Download/*patched*.img 2>/dev/null', deviceStore.currentDevice)
    if (files.trim()) {
      const firstFile = files.trim().split('\n')[0]
      const filename = firstFile.split('/').pop()
      await adb.pull(firstFile, `${saveDir}/${filename}`, deviceStore.currentDevice)
      logStore.success(`已保存: ${saveDir}/${filename}`)
      patchResult.value += `\n✅ 修补后的文件已保存: ${filename}\n`
    } else {
      logStore.error('未找到修补后的文件')
      patchResult.value += '\n❌ 未找到修补后的文件\n'
    }
  } catch (e: any) {
    logStore.error(`拉取失败: ${e}`)
  }
}
</script>

<template>
  <div class="patch-boot-view">
    <div class="card">
      <div class="card-title">📥 选择Boot镜像</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="bootFile" class="input" placeholder="选择boot.img或init_boot.img..." style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectBootFile">浏览</button>
      </div>
      <div class="grid grid-2">
        <button class="btn btn-primary" @click="extractBootFromDevice">从设备提取Boot</button>
        <button class="btn btn-ghost" @click="extractBootFromRom">从ROM提取</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">🔧 修补方式</div>
      <div class="grid grid-4">
        <button class="btn btn-ghost" @click="patchBoot('magisk')">Magisk</button>
        <button class="btn btn-ghost" @click="patchBoot('apatch')">APatch</button>
        <button class="btn btn-ghost" @click="patchBoot('ksu')">KernelSU</button>
        <button class="btn btn-ghost" @click="patchBoot('sukisu')">SukiSU</button>
      </div>
      <p class="hint">需要设备上已安装对应的Root管理器</p>
    </div>
    
    <div class="card">
      <div class="card-title">📤 修补结果</div>
      <pre class="result-box">{{ patchResult || '选择Boot镜像后开始修补...' }}</pre>
      <button class="btn btn-primary" @click="pullPatchedBoot" style="margin-top:12px">
        📥 拉取修补后的Boot
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 12px;
}

.result-box {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  min-height: 150px;
  max-height: 250px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
