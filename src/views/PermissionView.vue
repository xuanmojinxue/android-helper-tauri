<script setup lang="ts">
import { ref } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const pkgName = ref('')
const permissions = ref<{name: string, granted: boolean}[]>([])
const selectedPerms = ref<string[]>([])

async function listPermissions() {
  if (!pkgName.value.trim() || !deviceStore.currentDevice) {
    logStore.error('请输入包名')
    return
  }
  
  logStore.log(`获取 ${pkgName.value} 的权限...`)
  permissions.value = []
  
  try {
    const result = await adb.shell(`dumpsys package ${pkgName.value} | grep -E 'permission|granted'`, deviceStore.currentDevice)
    
    const lines = result.split('\n')
    for (const line of lines) {
      const match = line.match(/android\.permission\.(\w+).*granted=(\w+)/)
      if (match) {
        permissions.value.push({
          name: match[1],
          granted: match[2] === 'true'
        })
      }
    }
    
    // 去重
    const seen = new Set()
    permissions.value = permissions.value.filter(p => {
      if (seen.has(p.name)) return false
      seen.add(p.name)
      return true
    })
    
    logStore.success(`找到 ${permissions.value.length} 个权限`)
  } catch (e: any) {
    logStore.error(`获取失败: ${e}`)
  }
}

function togglePerm(name: string) {
  const idx = selectedPerms.value.indexOf(name)
  if (idx >= 0) {
    selectedPerms.value.splice(idx, 1)
  } else {
    selectedPerms.value.push(name)
  }
}

async function grantPermissions() {
  if (selectedPerms.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const perm of selectedPerms.value) {
    try {
      await adb.shell(`pm grant ${pkgName.value} android.permission.${perm}`, deviceStore.currentDevice)
      logStore.success(`已授予: ${perm}`)
    } catch (e: any) {
      logStore.error(`授予失败: ${perm}`)
    }
  }
  
  selectedPerms.value = []
  listPermissions()
}

async function revokePermissions() {
  if (selectedPerms.value.length === 0 || !deviceStore.currentDevice) return
  
  for (const perm of selectedPerms.value) {
    try {
      await adb.shell(`pm revoke ${pkgName.value} android.permission.${perm}`, deviceStore.currentDevice)
      logStore.success(`已撤销: ${perm}`)
    } catch (e: any) {
      logStore.error(`撤销失败: ${perm}`)
    }
  }
  
  selectedPerms.value = []
  listPermissions()
}

async function grantAllRuntime() {
  if (!pkgName.value.trim() || !deviceStore.currentDevice) return
  
  logStore.log('授予所有运行时权限...')
  try {
    await adb.shell(`pm grant ${pkgName.value} android.permission.READ_EXTERNAL_STORAGE`, deviceStore.currentDevice)
    await adb.shell(`pm grant ${pkgName.value} android.permission.WRITE_EXTERNAL_STORAGE`, deviceStore.currentDevice)
    await adb.shell(`pm grant ${pkgName.value} android.permission.CAMERA`, deviceStore.currentDevice)
    await adb.shell(`pm grant ${pkgName.value} android.permission.RECORD_AUDIO`, deviceStore.currentDevice)
    await adb.shell(`pm grant ${pkgName.value} android.permission.ACCESS_FINE_LOCATION`, deviceStore.currentDevice)
    await adb.shell(`pm grant ${pkgName.value} android.permission.ACCESS_COARSE_LOCATION`, deviceStore.currentDevice)
    logStore.success('常用权限已授予')
    listPermissions()
  } catch (e: any) {
    logStore.error(`授予失败: ${e}`)
  }
}
</script>

<template>
  <div class="permission-view">
    <div class="card">
      <div class="card-title">🔒 应用权限</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="pkgName" class="input" placeholder="包名 (com.example.app)" style="flex:1" />
        <button class="btn btn-ghost" @click="listPermissions">查看权限</button>
      </div>
      <button class="btn btn-ghost" @click="grantAllRuntime">授予常用权限</button>
    </div>
    
    <div class="card">
      <div class="card-title">📋 权限列表</div>
      <div class="perm-list">
        <div 
          v-for="perm in permissions" 
          :key="perm.name"
          class="perm-item"
          :class="{ selected: selectedPerms.includes(perm.name), granted: perm.granted }"
          @click="togglePerm(perm.name)"
        >
          <span class="status">{{ perm.granted ? '✅' : '❌' }}</span>
          <span class="name">{{ perm.name }}</span>
        </div>
        <div v-if="permissions.length === 0" class="empty">
          输入包名后点击"查看权限"
        </div>
      </div>
      
      <div class="flex flex-gap" style="margin-top:12px">
        <button class="btn btn-ghost" @click="grantPermissions" :disabled="selectedPerms.length === 0">
          ✅ 授予选中 ({{ selectedPerms.length }})
        </button>
        <button class="btn btn-ghost" @click="revokePermissions" :disabled="selectedPerms.length === 0">
          ❌ 撤销选中
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.perm-list {
  max-height: 350px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.perm-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  transition: background 0.2s;
  
  &:hover {
    background: var(--bg-hover);
  }
  
  &.selected {
    background: rgba(79, 140, 255, 0.15);
  }
  
  &:last-child {
    border-bottom: none;
  }
  
  .name {
    font-size: 12px;
    font-family: monospace;
  }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
}
</style>
