import { defineStore } from 'pinia'
import { ref } from 'vue'
import { adb } from '@/utils/adb'

export interface Device {
  serial: string
  status: string
  model?: string
  brand?: string
  android?: string
}

export const useDeviceStore = defineStore('device', () => {
  const devices = ref<Device[]>([])
  const currentDevice = ref<string | null>(null)
  const deviceInfo = ref<Device | null>(null)
  const isConnected = ref(false)
  const isLoading = ref(false)
  const refreshInterval = ref(5000) // 可配置的刷新间隔
  
  let monitorTimer: number | null = null
  
  async function refreshDevices() {
    if (isLoading.value) return
    isLoading.value = true
    
    try {
      const list = await adb.getDevices()
      devices.value = list
      isConnected.value = list.length > 0
      
      // 如果当前设备断开，自动切换到第一个可用设备
      if (currentDevice.value && !list.find(d => d.serial === currentDevice.value)) {
        currentDevice.value = list.length > 0 ? list[0].serial : null
        if (currentDevice.value) {
          await refreshDeviceInfo()
        } else {
          deviceInfo.value = null
        }
      } else if (list.length > 0 && !currentDevice.value) {
        currentDevice.value = list[0].serial
        await refreshDeviceInfo()
      }
    } catch (e) {
      console.error('刷新设备失败:', e)
    } finally {
      isLoading.value = false
    }
  }
  
  async function refreshDeviceInfo() {
    if (!currentDevice.value) {
      deviceInfo.value = null
      return
    }
    
    try {
      const [model, brand, android] = await Promise.all([
        adb.shell('getprop ro.product.model', currentDevice.value).catch(() => ''),
        adb.shell('getprop ro.product.brand', currentDevice.value).catch(() => ''),
        adb.shell('getprop ro.build.version.release', currentDevice.value).catch(() => '')
      ])
      
      deviceInfo.value = {
        serial: currentDevice.value,
        status: 'device',
        model: model.trim() || '未知',
        brand: brand.trim() || '未知',
        android: android.trim() || '未知'
      }
    } catch (e) {
      console.error('获取设备信息失败:', e)
      deviceInfo.value = null
    }
  }
  
  function selectDevice(serial: string) {
    if (currentDevice.value !== serial) {
      currentDevice.value = serial
      refreshDeviceInfo()
    }
  }
  
  function setRefreshInterval(ms: number) {
    refreshInterval.value = ms
    if (monitorTimer) {
      stopMonitor()
      startMonitor()
    }
  }
  
  function startMonitor() {
    if (monitorTimer) return
    refreshDevices()
    monitorTimer = window.setInterval(refreshDevices, refreshInterval.value)
  }
  
  function stopMonitor() {
    if (monitorTimer) {
      clearInterval(monitorTimer)
      monitorTimer = null
    }
  }
  
  return {
    devices,
    currentDevice,
    deviceInfo,
    isConnected,
    isLoading,
    refreshInterval,
    refreshDevices,
    refreshDeviceInfo,
    selectDevice,
    setRefreshInterval,
    startMonitor,
    stopMonitor
  }
})
