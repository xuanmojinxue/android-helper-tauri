import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface LogEntry {
  time: string
  type: 'info' | 'success' | 'error' | 'command'
  message: string
}

export const useLogStore = defineStore('log', () => {
  const logs = ref<LogEntry[]>([])
  const maxLogs = 500
  
  function addLog(message: string, type: LogEntry['type'] = 'info') {
    const time = new Date().toLocaleTimeString()
    logs.value.push({ time, type, message })
    
    if (logs.value.length > maxLogs) {
      logs.value = logs.value.slice(-maxLogs)
    }
  }
  
  function log(message: string) {
    addLog(message, 'info')
  }
  
  function success(message: string) {
    addLog(message, 'success')
  }
  
  function error(message: string) {
    addLog(message, 'error')
  }
  
  function command(cmd: string) {
    addLog(`> ${cmd}`, 'command')
  }
  
  function clear() {
    logs.value = []
  }
  
  return { logs, log, success, error, command, clear }
})
