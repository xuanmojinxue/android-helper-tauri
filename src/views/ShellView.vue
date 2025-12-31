<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { useDeviceStore } from '@/stores/device'
import { useLogStore } from '@/stores/log'
import { adb } from '@/utils/adb'

const deviceStore = useDeviceStore()
const logStore = useLogStore()

const command = ref('')
const useRoot = ref(false)
const history = ref<{cmd: string, output: string}[]>([])
const outputRef = ref<HTMLElement | null>(null)
const favorites = ref<string[]>([])
const newFavorite = ref('')

const FAVORITES_KEY = 'shell_favorites'

onMounted(() => {
  const saved = localStorage.getItem(FAVORITES_KEY)
  if (saved) {
    try {
      favorites.value = JSON.parse(saved)
    } catch {}
  }
})

function saveFavorites() {
  localStorage.setItem(FAVORITES_KEY, JSON.stringify(favorites.value))
}

async function executeCommand() {
  if (!command.value.trim() || !deviceStore.currentDevice) return
  
  let cmd = command.value.trim()
  if (useRoot.value) {
    cmd = `su -c '${cmd}'`
  }
  
  logStore.command(cmd)
  
  try {
    const output = await adb.shell(cmd, deviceStore.currentDevice)
    history.value.push({ cmd: command.value, output })
    logStore.log(output)
  } catch (e: any) {
    history.value.push({ cmd: command.value, output: `错误: ${e}` })
    logStore.error(e.toString())
  }
  
  command.value = ''
  await nextTick()
  if (outputRef.value) {
    outputRef.value.scrollTop = outputRef.value.scrollHeight
  }
}

function clearHistory() {
  history.value = []
}

function addToFavorites() {
  if (command.value.trim() && !favorites.value.includes(command.value.trim())) {
    favorites.value.push(command.value.trim())
    saveFavorites()
    logStore.success('已添加到收藏')
  }
}

function addNewFavorite() {
  if (newFavorite.value.trim() && !favorites.value.includes(newFavorite.value.trim())) {
    favorites.value.push(newFavorite.value.trim())
    saveFavorites()
    newFavorite.value = ''
    logStore.success('已添加到收藏')
  }
}

function removeFavorite(cmd: string) {
  const idx = favorites.value.indexOf(cmd)
  if (idx >= 0) {
    favorites.value.splice(idx, 1)
    saveFavorites()
  }
}

function executeFavorite(cmd: string) {
  command.value = cmd
  executeCommand()
}

const quickCommands = [
  { label: 'ID', cmd: 'id' },
  { label: '存储', cmd: 'df -h' },
  { label: '进程', cmd: 'ps -A | head -20' },
  { label: '内存', cmd: 'cat /proc/meminfo | head -5' },
  { label: 'CPU', cmd: 'cat /proc/cpuinfo | grep processor' },
  { label: '网络', cmd: 'ip addr' },
]
</script>

<template>
  <div class="shell-view">
    <div class="card">
      <div class="card-title">💻 Shell终端</div>
      
      <div class="terminal" ref="outputRef">
        <div v-for="(item, idx) in history" :key="idx" class="terminal-entry">
          <div class="terminal-cmd">$ {{ item.cmd }}</div>
          <div class="terminal-output">{{ item.output }}</div>
        </div>
        <div v-if="history.length === 0" class="terminal-empty">
          输入命令开始...
        </div>
      </div>
      
      <div class="input-area">
        <label class="root-toggle">
          <input type="checkbox" v-model="useRoot" />
          <span>Root</span>
        </label>
        <input 
          v-model="command" 
          class="input" 
          placeholder="输入Shell命令..." 
          @keyup.enter="executeCommand"
          style="flex:1"
        />
        <button class="btn btn-ghost" @click="addToFavorites" title="收藏">⭐</button>
        <button class="btn btn-ghost" @click="executeCommand">执行</button>
        <button class="btn btn-ghost" @click="clearHistory">清空</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">⚡ 快捷命令</div>
      <div class="grid grid-6">
        <button 
          v-for="qc in quickCommands" 
          :key="qc.cmd"
          class="btn btn-ghost"
          @click="command = qc.cmd; executeCommand()"
        >
          {{ qc.label }}
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">⭐ 收藏的命令</div>
      <div class="favorites-list">
        <div v-for="fav in favorites" :key="fav" class="favorite-item">
          <span class="fav-cmd" @click="executeFavorite(fav)">{{ fav }}</span>
          <button class="btn-icon" @click="removeFavorite(fav)">✕</button>
        </div>
        <div v-if="favorites.length === 0" class="empty">暂无收藏</div>
      </div>
      <div class="flex flex-gap" style="margin-top:12px">
        <input v-model="newFavorite" class="input" placeholder="添加新命令..." style="flex:1" @keyup.enter="addNewFavorite" />
        <button class="btn btn-ghost" @click="addNewFavorite">添加</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.terminal {
  background: #0d1117;
  border-radius: 8px;
  padding: 16px;
  height: 300px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 13px;
  margin-bottom: 12px;
}

.terminal-entry {
  margin-bottom: 12px;
}

.terminal-cmd {
  color: var(--accent);
  margin-bottom: 4px;
}

.terminal-output {
  color: #c9d1d9;
  white-space: pre-wrap;
  word-break: break-all;
}

.terminal-empty {
  color: #6e7681;
  text-align: center;
  padding: 40px;
}

.input-area {
  display: flex;
  gap: 10px;
  align-items: center;
}

.root-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-secondary);
  border-radius: 8px;
  cursor: pointer;
  
  input {
    accent-color: var(--danger);
  }
}

.grid-6 {
  grid-template-columns: repeat(6, 1fr);
}

.favorites-list {
  max-height: 150px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.favorite-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid var(--border);
  
  &:last-child {
    border-bottom: none;
  }
  
  .fav-cmd {
    font-family: Consolas, monospace;
    font-size: 12px;
    cursor: pointer;
    flex: 1;
    
    &:hover {
      color: var(--accent);
    }
  }
  
  .btn-icon {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px 8px;
    
    &:hover {
      color: var(--danger);
    }
  }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 20px;
}
</style>
