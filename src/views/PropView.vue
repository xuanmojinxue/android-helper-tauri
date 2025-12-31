<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const propKey = ref('')
const propValue = ref('')
const propList = ref('')

async function getProp() {
  if (!propKey.value || !deviceStore.currentDevice) return
  try {
    const result = await adb.shell(`getprop ${propKey.value}`, deviceStore.currentDevice)
    propValue.value = result.trim()
    logStore.log(`${propKey.value} = ${propValue.value}`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function setProp() {
  if (!propKey.value || !propValue.value || !deviceStore.currentDevice) return
  try {
    await adb.shell(`su -c 'setprop ${propKey.value} ${propValue.value}'`, deviceStore.currentDevice)
    logStore.success('设置成功')
  } catch (e: any) {
    logStore.error(`设置失败: ${e}`)
  }
}

async function resetProp() {
  if (!propKey.value || !deviceStore.currentDevice) return
  try {
    await adb.shell(`su -c 'resetprop --delete ${propKey.value}'`, deviceStore.currentDevice)
    logStore.success('属性已重置')
  } catch (e: any) {
    logStore.error(`重置失败: ${e}`)
  }
}

async function listAllProps() {
  if (!deviceStore.currentDevice) return
  logStore.log('获取所有属性...')
  try {
    const result = await adb.shell('getprop', deviceStore.currentDevice)
    propList.value = result
    logStore.success(`获取到 ${result.split('\n').length} 个属性`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

async function quickGetProp(prop: string) {
  if (!deviceStore.currentDevice) return
  propKey.value = prop
  await getProp()
}

const commonProps = [
  { label: '设备型号', prop: 'ro.product.model' },
  { label: '品牌', prop: 'ro.product.brand' },
  { label: '设备名', prop: 'ro.product.device' },
  { label: '指纹', prop: 'ro.build.fingerprint' },
  { label: 'SDK版本', prop: 'ro.build.version.sdk' },
  { label: 'Android版本', prop: 'ro.build.version.release' },
  { label: '安全补丁', prop: 'ro.build.version.security_patch' },
  { label: '序列号', prop: 'ro.serialno' },
]
</script>

<template>
  <div class="prop-view">
    <div class="card">
      <div class="card-title">🔑 系统属性</div>
      <div class="flex flex-gap" style="margin-bottom:10px">
        <input v-model="propKey" class="input" placeholder="属性名 (如 ro.build.fingerprint)" style="flex:1" />
        <button class="btn btn-ghost" @click="getProp">获取</button>
      </div>
      <div class="flex flex-gap">
        <input v-model="propValue" class="input" placeholder="属性值" style="flex:1" />
        <button class="btn btn-ghost" @click="setProp">设置</button>
        <button class="btn btn-ghost" @click="resetProp">重置</button>
      </div>
      <p class="hint">💡 设置属性需要Root权限</p>
    </div>
    
    <div class="card">
      <div class="card-title">📋 常用属性</div>
      <div class="grid grid-4">
        <button 
          v-for="item in commonProps" 
          :key="item.prop"
          class="btn btn-ghost"
          @click="quickGetProp(item.prop)"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📜 所有属性</div>
      <button class="btn btn-ghost" @click="listAllProps" style="margin-bottom:12px">
        🔄 获取所有属性
      </button>
      <pre class="prop-list">{{ propList || '点击上方按钮获取所有属性...' }}</pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 10px;
}

.prop-list {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 11px;
  max-height: 300px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
