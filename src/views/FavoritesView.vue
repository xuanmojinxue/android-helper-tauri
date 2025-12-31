<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const favorites = ref<{cmd: string, desc: string}[]>([])
const newCmd = ref('')
const newDesc = ref('')
const useRoot = ref(false)

const FAVORITES_KEY = 'shell_favorites_v2'

onMounted(() => {
  loadFavorites()
})

function loadFavorites() {
  const saved = localStorage.getItem(FAVORITES_KEY)
  if (saved) {
    try {
      favorites.value = JSON.parse(saved)
    } catch {}
  }
  
  // 如果为空，添加一些默认命令
  if (favorites.value.length === 0) {
    favorites.value = [
      { cmd: 'id', desc: '查看用户ID' },
      { cmd: 'df -h', desc: '查看存储空间' },
      { cmd: 'cat /proc/meminfo | head -5', desc: '查看内存信息' },
      { cmd: 'cat /proc/cpuinfo | grep processor', desc: '查看CPU核心' },
      { cmd: 'getprop ro.build.fingerprint', desc: '查看系统指纹' },
      { cmd: 'pm list packages -3', desc: '列出第三方应用' },
      { cmd: 'dumpsys battery', desc: '查看电池信息' },
      { cmd: 'ip addr', desc: '查看网络信息' },
    ]
    saveFavorites()
  }
}

function saveFavorites() {
  localStorage.setItem(FAVORITES_KEY, JSON.stringify(favorites.value))
}

function addFavorite() {
  if (!newCmd.value.trim()) return
  
  favorites.value.push({
    cmd: newCmd.value.trim(),
    desc: newDesc.value.trim() || newCmd.value.trim()
  })
  saveFavorites()
  
  newCmd.value = ''
  newDesc.value = ''
  logStore.success('命令已添加')
}

function removeFavorite(index: number) {
  favorites.value.splice(index, 1)
  saveFavorites()
}

async function executeFavorite(cmd: string) {
  if (!deviceStore.currentDevice) {
    logStore.error('请先连接设备')
    return
  }
  
  let execCmd = cmd
  if (useRoot.value) {
    execCmd = `su -c '${cmd}'`
  }
  
  logStore.command(execCmd)
  try {
    const result = await adb.shell(execCmd, deviceStore.currentDevice)
    logStore.log(result)
  } catch (e: any) {
    logStore.error(`执行失败: ${e}`)
  }
}

function editFavorite(index: number) {
  const fav = favorites.value[index]
  newCmd.value = fav.cmd
  newDesc.value = fav.desc
  removeFavorite(index)
}

function moveUp(index: number) {
  if (index <= 0) return
  const temp = favorites.value[index]
  favorites.value[index] = favorites.value[index - 1]
  favorites.value[index - 1] = temp
  saveFavorites()
}

function moveDown(index: number) {
  if (index >= favorites.value.length - 1) return
  const temp = favorites.value[index]
  favorites.value[index] = favorites.value[index + 1]
  favorites.value[index + 1] = temp
  saveFavorites()
}
</script>

<template>
  <div class="favorites-view">
    <div class="card">
      <div class="card-title">➕ 添加命令</div>
      <div class="flex flex-gap" style="margin-bottom:10px">
        <input v-model="newCmd" class="input" placeholder="Shell命令" style="flex:1" />
      </div>
      <div class="flex flex-gap">
        <input v-model="newDesc" class="input" placeholder="描述 (可选)" style="flex:1" />
        <button class="btn btn-ghost" @click="addFavorite">添加</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">⭐ 收藏的命令</div>
      
      <label class="root-toggle">
        <input type="checkbox" v-model="useRoot" />
        <span>使用Root执行</span>
      </label>
      
      <div class="favorites-list">
        <div v-for="(fav, index) in favorites" :key="index" class="favorite-item">
          <div class="fav-content" @click="executeFavorite(fav.cmd)">
            <span class="fav-desc">{{ fav.desc }}</span>
            <span class="fav-cmd">{{ fav.cmd }}</span>
          </div>
          <div class="fav-actions">
            <button class="btn-icon" @click="moveUp(index)" title="上移">↑</button>
            <button class="btn-icon" @click="moveDown(index)" title="下移">↓</button>
            <button class="btn-icon" @click="editFavorite(index)" title="编辑">✏️</button>
            <button class="btn-icon delete" @click="removeFavorite(index)" title="删除">✕</button>
          </div>
        </div>
        <div v-if="favorites.length === 0" class="empty">
          暂无收藏的命令
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.root-toggle {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  cursor: pointer;
  margin-bottom: 12px;
  font-size: 13px;
  
  input {
    accent-color: var(--danger);
  }
}

.favorites-list {
  max-height: 450px;
  overflow-y: auto;
}

.favorite-item {
  display: flex;
  align-items: center;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-bottom: 8px;
  
  .fav-content {
    flex: 1;
    cursor: pointer;
    
    &:hover {
      .fav-desc {
        color: var(--accent);
      }
    }
    
    .fav-desc {
      display: block;
      font-size: 13px;
      margin-bottom: 4px;
      transition: color 0.2s;
    }
    
    .fav-cmd {
      display: block;
      font-family: Consolas, monospace;
      font-size: 11px;
      color: var(--text-secondary);
    }
  }
  
  .fav-actions {
    display: flex;
    gap: 4px;
  }
  
  .btn-icon {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    
    &:hover {
      background: var(--bg-hover);
      color: var(--text-primary);
    }
    
    &.delete:hover {
      color: var(--danger);
    }
  }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
}
</style>
