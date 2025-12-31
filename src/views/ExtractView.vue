<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useLogStore } from '@/stores/log'
import { useSettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const logStore = useLogStore()
const settingsStore = useSettingsStore()

const romFile = ref('')
const extractDir = ref('')
const partitions = ref<{name: string, size: string, selected: boolean}[]>([])
const extractResult = ref('')
const isExtracting = ref(false)

onMounted(async () => {
  // 默认使用统一的ROM提取目录
  extractDir.value = await settingsStore.getSubDir('rom')
})

async function selectRomFile() {
  const file = await open({
    filters: [
      { name: 'ROM', extensions: ['zip', 'bin'] }
    ],
    title: '选择ROM包或payload.bin'
  })
  if (file && typeof file === 'string') {
    romFile.value = file
  }
}

async function selectExtractDir() {
  const dir = await open({ directory: true, title: '选择输出目录' })
  if (dir && typeof dir === 'string') {
    extractDir.value = dir
  }
}

async function parseRom() {
  if (!romFile.value) {
    logStore.error('请先选择ROM文件')
    return
  }
  
  logStore.log('解析ROM包...')
  extractResult.value = '正在解析...\n'
  
  try {
    const result = await invoke<string>('parse_payload', { path: romFile.value })
    const lines = result.split('\n').filter(l => l.trim())
    
    partitions.value = lines.map(line => {
      const parts = line.split(':')
      return {
        name: parts[0]?.trim() || '',
        size: parts[1]?.trim() || '',
        selected: false
      }
    }).filter(p => p.name)
    
    extractResult.value = `找到 ${partitions.value.length} 个分区\n`
    logStore.success(`解析完成，找到 ${partitions.value.length} 个分区`)
  } catch (e: any) {
    extractResult.value = `解析失败: ${e}\n`
    extractResult.value += '\n请确保:\n'
    extractResult.value += '1. 文件是有效的payload.bin或包含payload.bin的zip\n'
    extractResult.value += '2. 已安装payload-dumper-go工具\n'
    logStore.error(`解析失败: ${e}`)
  }
}

function selectAll() {
  partitions.value.forEach(p => p.selected = true)
}

function selectNone() {
  partitions.value.forEach(p => p.selected = false)
}

function selectBootParts() {
  partitions.value.forEach(p => {
    p.selected = ['boot', 'init_boot', 'vendor_boot', 'recovery', 'vbmeta', 'dtbo'].includes(p.name)
  })
}

async function extractPartitions() {
  const selected = partitions.value.filter(p => p.selected)
  if (selected.length === 0) {
    logStore.error('请先选择要提取的分区')
    return
  }
  if (!extractDir.value) {
    logStore.error('请先选择输出目录')
    return
  }
  
  isExtracting.value = true
  extractResult.value = `开始提取 ${selected.length} 个分区...\n`
  logStore.log(`提取分区: ${selected.map(p => p.name).join(', ')}`)
  
  try {
    const partNames = selected.map(p => p.name)
    const result = await invoke<string>('extract_payload', {
      path: romFile.value,
      outputDir: extractDir.value,
      partitions: partNames
    })
    
    extractResult.value += result + '\n'
    extractResult.value += `\n✅ 提取完成，文件保存在: ${extractDir.value}\n`
    logStore.success('提取完成')
  } catch (e: any) {
    extractResult.value += `\n❌ 提取失败: ${e}\n`
    logStore.error(`提取失败: ${e}`)
  } finally {
    isExtracting.value = false
  }
}

async function extractAll() {
  selectAll()
  await extractPartitions()
}
</script>

<template>
  <div class="extract-view">
    <div class="card">
      <div class="card-title">📦 ROM包提取</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <input v-model="romFile" class="input" placeholder="支持 payload.bin / OTA.zip / 线刷包.zip" style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectRomFile">浏览</button>
        <button class="btn btn-ghost" @click="parseRom">解析分区</button>
      </div>
      <div class="flex flex-gap">
        <input v-model="extractDir" class="input" placeholder="输出目录" style="flex:1" readonly />
        <button class="btn btn-ghost" @click="selectExtractDir">浏览</button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📂 可用分区</div>
      <div class="flex flex-gap" style="margin-bottom:12px">
        <button class="btn btn-ghost" @click="selectAll">全选</button>
        <button class="btn btn-ghost" @click="selectNone">全不选</button>
        <button class="btn btn-ghost" @click="selectBootParts">只选boot相关</button>
      </div>
      
      <div class="partition-grid" v-if="partitions.length > 0">
        <label 
          v-for="part in partitions" 
          :key="part.name"
          class="partition-item"
          :class="{ selected: part.selected }"
        >
          <input type="checkbox" v-model="part.selected" />
          <span class="part-name">{{ part.name }}</span>
          <span class="part-size">{{ part.size }}</span>
        </label>
      </div>
      <div v-else class="empty">
        选择ROM后点击"解析分区"
      </div>
      
      <div class="flex flex-gap" style="margin-top:12px">
        <button class="btn btn-ghost" @click="extractPartitions" :disabled="isExtracting">
          🔧 提取选中分区
        </button>
        <button class="btn btn-ghost" @click="extractAll" :disabled="isExtracting">
          提取全部分区
        </button>
      </div>
    </div>
    
    <div class="card">
      <div class="card-title">📋 提取结果</div>
      <pre class="result-box">{{ extractResult || '选择ROM并解析后开始提取...' }}</pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.partition-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
  padding: 8px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.partition-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: var(--bg-primary);
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  
  &.selected {
    background: rgba(79, 140, 255, 0.2);
  }
  
  input[type="checkbox"] {
    accent-color: var(--accent);
  }
  
  .part-name {
    flex: 1;
  }
  
  .part-size {
    color: var(--text-secondary);
    font-size: 11px;
  }
}

.empty {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.result-box {
  background: var(--bg-secondary);
  padding: 12px;
  border-radius: 8px;
  font-family: Consolas, monospace;
  font-size: 12px;
  min-height: 120px;
  max-height: 200px;
  overflow-y: auto;
  white-space: pre-wrap;
  margin: 0;
}
</style>
