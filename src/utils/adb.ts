import { invoke } from '@tauri-apps/api/core'

export interface AdbDevice {
  serial: string
  status: string
}

// 通用错误处理
async function invokeWithRetry<T>(
  cmd: string, 
  args: Record<string, unknown>, 
  retries = 1
): Promise<T> {
  let lastError: unknown
  for (let i = 0; i <= retries; i++) {
    try {
      return await invoke<T>(cmd, args)
    } catch (e) {
      lastError = e
      if (i < retries) {
        await new Promise(r => setTimeout(r, 500))
      }
    }
  }
  throw lastError
}

export const adb = {
  async getDevices(): Promise<AdbDevice[]> {
    return await invoke('get_devices')
  },
  
  async shell(cmd: string, device?: string): Promise<string> {
    return await invoke('adb_shell', { cmd, device })
  },
  
  async install(apkPath: string, device?: string): Promise<string> {
    return await invoke('adb_install', { apkPath, device })
  },
  
  async uninstall(pkg: string, device?: string): Promise<string> {
    return await invoke('adb_uninstall', { package: pkg, device })
  },
  
  async push(local: string, remote: string, device?: string): Promise<string> {
    return await invoke('adb_push', { local, remote, device })
  },
  
  async pull(remote: string, local: string, device?: string): Promise<string> {
    return await invoke('adb_pull', { remote, local, device })
  },
  
  async reboot(mode?: string, device?: string): Promise<string> {
    return await invoke('adb_reboot', { mode, device })
  },
  
  async connect(address: string): Promise<string> {
    return await invokeWithRetry('adb_connect', { address }, 2)
  },
  
  async disconnect(address?: string): Promise<string> {
    return await invoke('adb_disconnect', { address })
  },
  
  async extractApk(pkg: string, outputDir: string, device?: string): Promise<string> {
    return await invoke('extract_apk', { package: pkg, outputDir, device })
  },
  
  async sideload(otaPath: string, device?: string): Promise<string> {
    return await invoke('adb_sideload', { otaPath, device })
  },
  
  async logcat(device?: string): Promise<string> {
    return await invoke('start_logcat', { device })
  },
  
  async clearLogcat(device?: string): Promise<string> {
    return await invoke('clear_logcat', { device })
  }
}

export const fastboot = {
  async devices(): Promise<string> {
    return await invoke('fastboot_devices')
  },
  
  async flash(partition: string, imagePath: string): Promise<string> {
    return await invoke('fastboot_flash', { partition, imagePath })
  },
  
  async reboot(mode?: string): Promise<string> {
    return await invoke('fastboot_reboot', { mode })
  },
  
  async unlock(): Promise<string> {
    return await invoke('fastboot_unlock')
  },
  
  async getVar(variable: string): Promise<string> {
    return await invoke('fastboot_get_var', { var: variable })
  },
  
  async setActive(slot: string): Promise<string> {
    return await invoke('fastboot_set_active', { slot })
  },
  
  async erase(partition: string): Promise<string> {
    return await invoke('fastboot_erase', { partition })
  }
}
