import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useSettingsStore = defineStore('settings', () => {
  const outputDir = ref('')
  const initialized = ref(false)
  
  // 子目录
  const subDirs = {
    apk: 'APK提取',
    backup: '分区备份',
    screenshot: '截图',
    record: '录屏',
    rom: 'ROM提取',
    module: '模块提取',
    log: '日志'
  }
  
  async function init() {
    if (initialized.value) return
    
    // 获取程序所在目录下的 data 文件夹
    try {
      outputDir.value = await invoke<string>('get_data_dir')
    } catch (e) {
      console.error('获取数据目录失败:', e)
      outputDir.value = './data'
    }
    
    // 确保目录存在
    await ensureOutputDir()
    initialized.value = true
  }
  
  async function ensureOutputDir() {
    try {
      // 使用 Rust 后端创建目录
      await invoke('ensure_dir', { path: outputDir.value })
      
      // 创建子目录
      for (const subDir of Object.values(subDirs)) {
        const path = `${outputDir.value}/${subDir}`
        await invoke('ensure_dir', { path })
      }
    } catch (e) {
      console.error('创建输出目录失败:', e)
    }
  }
  
  async function getSubDir(type: keyof typeof subDirs): Promise<string> {
    await init()
    return `${outputDir.value}/${subDirs[type]}`
  }
  
  // 获取带时间戳的文件名
  function getTimestampedName(prefix: string, ext: string): string {
    const now = new Date()
    const timestamp = now.toISOString().replace(/[:.]/g, '-').slice(0, 19)
    return `${prefix}_${timestamp}.${ext}`
  }
  
  return {
    outputDir,
    initialized,
    subDirs,
    init,
    getSubDir,
    getTimestampedName,
    ensureOutputDir
  }
})
