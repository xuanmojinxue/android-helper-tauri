<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useLogStore } from '@/stores/log'

const logStore = useLogStore()
const logContainer = ref<HTMLElement | null>(null)
const isExpanded = ref(true)

watch(() => logStore.logs.length, async () => {
  await nextTick()
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight
  }
})

function getLogClass(type: string) {
  return {
    'log-success': type === 'success',
    'log-error': type === 'error',
    'log-command': type === 'command'
  }
}
</script>

<template>
  <div class="log-panel" :class="{ collapsed: !isExpanded }">
    <div class="log-header">
      <span>📋 命令输出</span>
      <div class="log-actions">
        <button class="btn btn-ghost" @click="logStore.clear">清空</button>
        <button class="btn btn-ghost" @click="isExpanded = !isExpanded">
          {{ isExpanded ? '收起' : '展开' }}
        </button>
      </div>
    </div>
    <div v-show="isExpanded" ref="logContainer" class="log-content">
      <div 
        v-for="(log, index) in logStore.logs" 
        :key="index"
        class="log-entry"
        :class="getLogClass(log.type)"
      >
        <span class="log-time">{{ log.time }}</span>
        <span class="log-message">{{ log.message }}</span>
      </div>
      <div v-if="logStore.logs.length === 0" class="log-empty">
        暂无日志
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.log-panel {
  background: #0d1117;
  border-top: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  height: 180px;
  transition: height 0.3s;
  
  &.collapsed {
    height: 40px;
  }
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: var(--bg-card);
  font-size: 14px;
}

.log-actions {
  display: flex;
  gap: 8px;
  
  .btn {
    padding: 4px 12px;
    font-size: 12px;
  }
}

.log-content {
  flex: 1;
  overflow-y: auto;
  padding: 10px 16px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.6;
}

.log-entry {
  display: flex;
  gap: 12px;
  
  .log-time {
    color: #6e7681;
    flex-shrink: 0;
  }
  
  .log-message {
    color: #c9d1d9;
    word-break: break-all;
  }
  
  &.log-success .log-message {
    color: var(--success);
  }
  
  &.log-error .log-message {
    color: var(--danger);
  }
  
  &.log-command .log-message {
    color: var(--accent);
  }
}

.log-empty {
  color: #6e7681;
  text-align: center;
  padding: 20px;
}
</style>
