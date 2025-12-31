<script setup lang="ts">
import { reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const categories = [
  {
    name: '📱 设备管理',
    key: 'device',
    items: [
      { path: '/device', label: '设备连接' },
      { path: '/device-info', label: '设备详情' },
    ]
  },
  {
    name: '🔄 系统操作',
    key: 'system',
    items: [
      { path: '/flash', label: '刷机工具' },
      { path: '/patch-boot', label: 'Boot修补' },
      { path: '/backup', label: '分区与更新' },
      { path: '/extract', label: 'ROM提取' },
    ]
  },
  {
    name: '📦 应用管理',
    key: 'apps',
    items: [
      { path: '/apps', label: '应用安装' },
      { path: '/root', label: 'Root管理' },
      { path: '/apk-info', label: '应用分析' },
    ]
  },
  {
    name: '🔧 系统工具',
    key: 'tools',
    items: [
      { path: '/prop', label: 'Prop编辑' },
      { path: '/activity', label: '活动启动' },
      { path: '/permission', label: '权限管理' },
      { path: '/battery', label: '电池管理' },
    ]
  },
  {
    name: '🌐 网络工具',
    key: 'network',
    items: [
      { path: '/proxy', label: '代理设置' },
      { path: '/network', label: '网络诊断' },
    ]
  },
  {
    name: '📊 调试工具',
    key: 'debug',
    items: [
      { path: '/logcat', label: 'Logcat' },
      { path: '/perf', label: '性能监控' },
      { path: '/security', label: '安全检测' },
      { path: '/shell', label: 'Shell终端' },
      { path: '/favorites', label: '命令收藏' },
    ]
  },
  {
    name: '📂 文件管理',
    key: 'file',
    items: [
      { path: '/file', label: '文件传输' },
      { path: '/screen', label: '投屏控制' },
    ]
  },
  {
    name: '⚙️ 设置',
    key: 'settings',
    items: [
      { path: '/settings', label: '应用设置' },
    ]
  }
]

// 折叠状态，默认只展开设备管理
const expanded = reactive<Record<string, boolean>>({
  device: true,
  system: false,
  apps: false,
  tools: false,
  network: false,
  debug: false,
  file: false
})

function toggleCategory(key: string) {
  expanded[key] = !expanded[key]
}

function navigate(path: string) {
  router.push(path)
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-title">
      <span class="logo">📱</span>
      <span>玩机助手</span>
    </div>
    <nav class="sidebar-nav">
      <div v-for="cat in categories" :key="cat.key" class="category">
        <div class="category-title" @click="toggleCategory(cat.key)">
          <span class="arrow" :class="{ expanded: expanded[cat.key] }">▶</span>
          <span>{{ cat.name }}</span>
        </div>
        <div class="category-items" v-show="expanded[cat.key]">
          <div
            v-for="item in cat.items"
            :key="item.path"
            class="nav-item"
            :class="{ active: route.path === item.path }"
            @click="navigate(item.path)"
          >
            <span class="nav-label">{{ item.label }}</span>
          </div>
        </div>
      </div>
    </nav>
  </aside>
</template>

<style lang="scss" scoped>
.sidebar {
  width: 180px;
  background: var(--bg-primary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.sidebar-title {
  padding: 14px;
  font-size: 15px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
  border-bottom: 1px solid var(--border);
  
  .logo {
    font-size: 18px;
  }
}

.sidebar-nav {
  flex: 1;
  padding: 6px;
  overflow-y: auto;
}

.category {
  margin-bottom: 4px;
}

.category-title {
  font-size: 12px;
  color: var(--text-primary);
  padding: 8px 10px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  border-radius: 6px;
  
  &:hover {
    background: var(--bg-hover);
  }
  
  .arrow {
    font-size: 10px;
    transition: transform 0.2s;
    color: var(--text-secondary);
    
    &.expanded {
      transform: rotate(90deg);
    }
  }
}

.category-items {
  margin-left: 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  margin-bottom: 1px;
  
  &:hover {
    background: var(--bg-hover);
  }
  
  &.active {
    background: var(--accent);
    color: white;
  }
  
  .nav-label {
    font-size: 12px;
  }
}
</style>
