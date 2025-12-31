import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      redirect: '/device'
    },
    // 设备管理
    {
      path: '/device',
      name: 'device',
      component: () => import('@/views/DeviceView.vue'),
      meta: { title: '设备连接', icon: '📱', category: '设备管理' }
    },
    {
      path: '/device-info',
      name: 'device-info',
      component: () => import('@/views/DeviceInfoView.vue'),
      meta: { title: '设备详情', icon: '📋', category: '设备管理' }
    },
    // 系统操作
    {
      path: '/flash',
      name: 'flash',
      component: () => import('@/views/FlashView.vue'),
      meta: { title: '刷机工具', icon: '💾', category: '系统操作' }
    },
    {
      path: '/patch-boot',
      name: 'patch-boot',
      component: () => import('@/views/PatchBootView.vue'),
      meta: { title: 'Boot修补', icon: '🔧', category: '系统操作' }
    },
    {
      path: '/backup',
      name: 'backup',
      component: () => import('@/views/BackupView.vue'),
      meta: { title: '备份还原', icon: '💿', category: '系统操作' }
    },

    {
      path: '/extract',
      name: 'extract',
      component: () => import('@/views/ExtractView.vue'),
      meta: { title: 'ROM提取', icon: '📦', category: '系统操作' }
    },
    // 应用管理
    {
      path: '/apps',
      name: 'apps',
      component: () => import('@/views/AppsView.vue'),
      meta: { title: '应用管理', icon: '📦', category: '应用管理' }
    },
    {
      path: '/root',
      name: 'root',
      component: () => import('@/views/RootView.vue'),
      meta: { title: 'Root管理', icon: '🔓', category: '应用管理' }
    },
    {
      path: '/apk-info',
      name: 'apk-info',
      component: () => import('@/views/ApkInfoView.vue'),
      meta: { title: '应用分析', icon: '🔍', category: '应用管理' }
    },
    // 系统工具
    {
      path: '/prop',
      name: 'prop',
      component: () => import('@/views/PropView.vue'),
      meta: { title: 'Prop编辑', icon: '📝', category: '系统工具' }
    },
    {
      path: '/activity',
      name: 'activity',
      component: () => import('@/views/ActivityView.vue'),
      meta: { title: '活动启动', icon: '🎯', category: '系统工具' }
    },
    {
      path: '/permission',
      name: 'permission',
      component: () => import('@/views/PermissionView.vue'),
      meta: { title: '权限管理', icon: '🔒', category: '系统工具' }
    },
    {
      path: '/battery',
      name: 'battery',
      component: () => import('@/views/BatteryView.vue'),
      meta: { title: '电池管理', icon: '🔋', category: '系统工具' }
    },
    // 网络工具
    {
      path: '/proxy',
      name: 'proxy',
      component: () => import('@/views/ProxyView.vue'),
      meta: { title: '代理设置', icon: '🌐', category: '网络工具' }
    },
    {
      path: '/network',
      name: 'network',
      component: () => import('@/views/NetworkView.vue'),
      meta: { title: '网络诊断', icon: '📶', category: '网络工具' }
    },
    // 调试工具
    {
      path: '/logcat',
      name: 'logcat',
      component: () => import('@/views/LogcatView.vue'),
      meta: { title: 'Logcat', icon: '📡', category: '调试工具' }
    },
    {
      path: '/perf',
      name: 'perf',
      component: () => import('@/views/PerfView.vue'),
      meta: { title: '性能监控', icon: '📊', category: '调试工具' }
    },
    {
      path: '/security',
      name: 'security',
      component: () => import('@/views/SecurityView.vue'),
      meta: { title: '安全检测', icon: '🛡️', category: '调试工具' }
    },
    {
      path: '/shell',
      name: 'shell',
      component: () => import('@/views/ShellView.vue'),
      meta: { title: 'Shell终端', icon: '💻', category: '调试工具' }
    },
    {
      path: '/favorites',
      name: 'favorites',
      component: () => import('@/views/FavoritesView.vue'),
      meta: { title: '命令收藏', icon: '⭐', category: '调试工具' }
    },
    // 文件管理
    {
      path: '/file',
      name: 'file',
      component: () => import('@/views/FileView.vue'),
      meta: { title: '文件传输', icon: '📂', category: '文件管理' }
    },
    {
      path: '/screen',
      name: 'screen',
      component: () => import('@/views/ScreenView.vue'),
      meta: { title: '投屏控制', icon: '🖥️', category: '文件管理' }
    },
    // 设置
    {
      path: '/settings',
      name: 'settings',
      component: () => import('@/views/SettingsView.vue'),
      meta: { title: '设置', icon: '⚙️', category: '设置' }
    }
  ]
})

export default router
