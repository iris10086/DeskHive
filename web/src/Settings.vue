<template>
  <div class="container">
    <!-- Top Tab Navigation (mobile) -->
    <div class="tab-bar">
      <div class="tab-bar-scroll">
        <button
          v-for="(section, key) in sections"
          :key="key"
          class="tab-item"
          :class="{ active: activeSection === key }"
          @click="activeSection = key"
        >
          <span class="tab-icon" v-html="section.icon"></span>
          <span class="tab-label">{{ section.name }}</span>
        </button>
      </div>
    </div>

    <div class="content">
      <div class="content-body">
        <div v-if="activeSection === 'appearance'" class="setting-section">
          <div class="section-title">窗口外观</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="setting-label-wrap">
                <div class="setting-label">主窗口透明度</div>
                <div class="setting-description">调整主窗口的透明程度，不影响设置窗口</div>
              </div>
              <div class="setting-control">
                <input
                  type="range"
                  v-model="opacityValue"
                  min="0.5"
                  max="1"
                  step="0.1"
                  @input="applyOpacityPreview"
                  class="range-input"
                >
                <span class="range-value">{{ Math.round(settings.opacity * 100) }}%</span>
              </div>
            </div>
            <div class="setting-item">
              <div class="setting-label-wrap">
                <div class="setting-label">主题模式</div>
                <div class="setting-description">切换日间或夜间主题</div>
              </div>
              <div class="setting-control">
                <div
                  class="theme-toggle-switch"
                  :class="{ 'theme-dark': settings.theme === 'dark' }"
                  @click="toggleTheme"
                >
                  <div class="theme-toggle-slider"></div>
                  <span class="theme-label light-label">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <circle cx="12" cy="12" r="5" fill="currentColor"/>
                      <path d="M12 1V3M12 21V23M4.22 4.22L5.64 5.64M18.36 18.36L19.78 19.78M1 12H3M21 12H23M4.22 19.78L5.64 18.36M18.36 5.64L19.78 4.22" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                    </svg>
                  </span>
                  <span class="theme-label dark-label">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" fill="currentColor"/>
                    </svg>
                  </span>
                </div>
              </div>
            </div>
            <div class="setting-item">
              <div class="setting-label-wrap">
                <div class="setting-label">高优先级颜色</div>
                <div class="setting-description">双击任务标记为高优先级时的圆点颜色</div>
              </div>
              <div class="setting-control">
                <div class="color-picker-wrapper">
                  <input
                    type="color"
                    v-model="settings.priority_color"
                    class="color-picker"
                    @input="applyPriorityColorPreview"
                  >
                  <span class="color-value">{{ settings.priority_color }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-if="activeSection === 'help'" class="setting-section">
          <div class="section-title">快速上手</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="help-content">
                <h3>📝 任务操作</h3>
                <p>• 新建任务：底部输入框输入内容后按回车键</p>
                <p>• 完成任务：鼠标悬停任务，点击左侧"✓"按钮</p>
                <p>• 撤销完成：在已完成分组中，悬停任务点击"↻"按钮</p>
                <p>• 编辑任务：右键任务选择"编辑任务"，或双击任务文字</p>
                <p>• 删除任务：右键任务选择"删除任务"</p>
                <p>• 标记优先级：双击任务快速标记/取消高优先级（橙色圆点）</p>
                <p>• 拖动排序：鼠标悬停任务，点住"☰"图标拖动调整顺序</p>
                <p>• 查看详情：右键任务查看创建时间、截止时间、优先级等信息</p>

                <h3>⏰ 截止时间</h3>
                <p>• 设置截止：右键任务选择"设置截止时间"</p>
                <p>• 快捷操作：对话框中使用 Tab 键切换输入框，Enter 键确认</p>
                <p>• 默认时间：首次设置默认为当前时间 1 小时后</p>
                <p>• 修改时间：右键任务选择"修改截止时间"，保留原有时间</p>
                <p>• 移除截止：右键任务选择"移除截止时间"</p>
                <p>• 倒计时显示：任务右侧显示剩余时间（绿色/黄色/红色）</p>
                <p>• 到期提醒：在"使用设置"中启用倒计时通知功能</p>

                <h3>📁 分组管理</h3>
                <p>• 快速创建：输入框输入"/分组名"后回车（如：/工作）</p>
                <p>• 菜单创建：点击底部"+"按钮，选择"新建分组"</p>
                <p>• 重命名分组：右键分组标题选择"重命名分组"</p>
                <p>• 删除分组：右键分组标题选择"删除分组"（任务会移到未分组）</p>
                <p>• 折叠/展开：点击分组标题左侧"▼"图标</p>
                <p>• 调整顺序：鼠标悬停分组标题，点击"▲▼"按钮上下移动</p>
                <p>• 未分组：新建任务默认添加到未分组，可拖动到其他分组</p>

                <h3>🔄 拖动功能</h3>
                <p>• 同组排序：拖动任务到目标位置释放，调整组内顺序</p>
                <p>• 跨组移动：拖动任务到其他分组的任务列表中</p>
                <p>• 快速移动：拖动任务到分组标题上，自动添加到该组末尾</p>
                <p>• 拖动提示：拖动时分组会高亮显示，表示可以放置</p>
                <p>• 禁止拖动：在"行为"设置中可禁止拖动窗口，方便调整位置</p>

                <h3>🎨 视图切换</h3>
                <p>• 列表视图：默认视图，按分组显示任务</p>
                <p>• 时间轴视图：点击标题栏"时间轴"图标切换</p>
                <p>• 时间轴排序：在"使用设置"中可选择按截止时间或创建时间排序</p>
                <p>• 时间轴操作：支持完成、删除、查看详情、标记优先级</p>

                <h3>✅ 已完成任务</h3>
                <p>• 查看已完成：点击底部"已完成"分组展开查看</p>
                <p>• 撤销完成：悬停已完成任务，点击"↻"按钮恢复为待办</p>
                <p>• 批量清理：点击"已完成"分组右侧垃圾桶图标清空所有</p>
                <p>• 自动清理：右键任务选择"移除完成7天前"，清理旧任务</p>
                <p>• 耗时显示：已完成任务显示从创建到完成的耗时天数</p>

                <h3>🔔 通知提醒</h3>
                <p>• 启用通知：在"使用设置"中开启"启用倒计时通知"</p>
                <p>• 提前时间：设置提前多少分钟提醒（默认 30 分钟）</p>
                <p>• 通知内容：显示剩余时间、任务内容、截止时间、优先级</p>
                <p>• 通知位置：Windows 系统右下角通知中心</p>
                <p>• 提示音：通知时播放系统提示音</p>
                <p>• 权限设置：如无通知，请检查 Windows 通知权限</p>

                <h3>⚙️ 外观设置</h3>
                <p>• 透明度：调整主窗口透明度（50%-100%），设置窗口保持不透明</p>
                <p>• 主题模式：切换日间/夜间主题，夜间模式更护眼</p>
                <p>• 优先级颜色：自定义高优先级任务的圆点颜色</p>

                <h3>🎯 行为设置</h3>
                <p>• 禁止拖动：开启后无法拖动窗口，防止误操作</p>
                <p>• 窗口层级：选择"置于顶层"或"置于桌面"</p>
                <p>• 开机自启：系统启动时自动运行应用</p>

                <h3>💡 实用技巧</h3>
                <p>• 托盘图标：左键点击快速显示/隐藏窗口</p>
                <p>• 托盘菜单：右键托盘图标访问快捷功能</p>
                <p>• 重置位置：托盘菜单选择"重置窗口位置"，自动关闭禁止拖动</p>
                <p>• 窗口位置：拖动窗口后自动记住位置，下次启动恢复</p>
                <p>• 关闭窗口：点击关闭按钮会最小化到托盘，不会退出程序</p>
                <p>• 完全退出：右键托盘图标选择"退出"</p>
                <p>• 数据保存：所有数据自动保存到文档目录，重装不丢失</p>
                <p>• 设置同步：所有设置修改即时生效，无需手动保存</p>
              </div>
            </div>
          </div>
        </div>

        <div v-if="activeSection === 'contact'" class="setting-section">
          <div class="section-title">联系方式</div>
          <div class="setting-group">
            <div class="setting-item contact-item">
              <div class="contact-content">
                <div class="contact-row logo-row">
                  <img src="/mypic/feijimiao.png" alt="作者Logo" class="contact-logo" />
                  <button class="blog-btn" @click="openBlog">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M14 3V5H17.59L7.76 14.83L9.17 16.24L19 6.41V10H21V3M19 19H5V5H12V3H5C3.89 3 3 3.9 3 5V19C3 20.1 3.89 21 5 21H19C20.1 21 21 20.1 21 19V12H19V19Z" fill="currentColor"/>
                    </svg>
                    给作者留言
                  </button>
                </div>

                <div class="contact-row text-row">
                  <p class="contact-text">💼 软件定制开发，联系作者</p>
                </div>

                <div class="contact-row qrcode-row">
                  <div class="qrcode-item">
                    <h3>📱 微信公众号</h3>
                    <img src="/mypic/gzh.png" alt="公众号二维码" class="contact-qrcode" />
                    <p class="qrcode-tip">扫码关注公众号</p>
                  </div>

                  <div class="qrcode-item">
                    <h3>💬 微信联系</h3>
                    <img src="/mypic/Snipaste_2025-11-23_01-09-52.png" alt="微信二维码" class="contact-qrcode" />
                    <p class="qrcode-tip">扫码添加微信</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-if="activeSection === 'about'" class="setting-section">
          <div class="section-title">应用信息</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="setting-label-wrap">
                <div class="setting-label">版本</div>
                <div class="setting-description">当前应用版本号</div>
              </div>
              <div class="setting-control">
                <span style="color: #6d6d70;">{{ appVersion }}</span>
              </div>
            </div>
          </div>

          <div class="section-title" style="margin-top: 24px;">版本更新</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="update-content">
                <h3>v1.0.0 (2025-12-11)</h3>
                <h4>✨ 新增功能</h4>
                <p>• 时间轴视图：全新的任务时间树展示方式，按时间线查看任务</p>
                <p>• 截止时间通知：支持定时系统通知，可自定义提前提醒时间</p>
                <p>• 通知设置：可配置是否启用截止时间通知和提前提醒分钟数</p>
                <p>• 窗口焦点优化：托盘图标和设置按钮点击时强制获取焦点</p>
                <p>• 已完成任务优化：右键菜单简化，移除不必要的编辑选项</p>

                <h4>🎨 界面优化</h4>
                <p>• 页面样式全面优化，更美观简洁</p>
                <p>• 修复高分辨率屏幕显示异常问题</p>
                <p>• 优化 Tooltip 提示框位置算法，智能避免遮挡</p>
                <p>• 改进"置于桌面"模式下的窗口显示逻辑</p>
                <p>• 优化设置窗口的焦点获取体验</p>

                <h4>🐛 问题修复</h4>
                <p>• 修复托盘点击需要两次才能显示的问题</p>
                <p>• 修复"置于桌面"模式下窗口无法获取焦点的问题</p>
                <p>• 修复设置窗口已打开时点击无反应的问题</p>
                <p>• 修复第一个任务的提示框被遮挡的问题</p>
                <p>• 优化窗口显示和隐藏的稳定性</p>

                <h4>🔧 功能调整</h4>
                <p>• 托盘菜单"显示/隐藏"改为"显示"，统一为获取焦点</p>
                <p>• 已完成任务右键菜单移除"编辑"、"设置截止时间"等选项</p>
                <p>• 使用 Windows API 强制获取窗口焦点，提升用户体验</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="bottom-bar">
        <button class="btn btn-primary" @click="closeWindow">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from './api'

const router = useRouter()

interface AppSettings {
  opacity: number
  disable_drag: boolean
  auto_start: boolean
  silent_start: boolean
  theme: string
  priority_color: string
  window_level: string
  timeline_deadline_priority: boolean
  enable_deadline_notification: boolean
  notification_minutes_before: number
  sync_enabled: boolean
  sync_server_url: string
}

type SectionKey = 'appearance' | 'help' | 'contact' | 'about'

interface Section {
  name: string
  icon: string
}

const activeSection = ref<SectionKey>('appearance')
const originalOpacity = ref(0.95)
const appVersion = ref('...')

const sections: Record<SectionKey, Section> = {
  appearance: {
    name: '外观',
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 2C11.5 2 11 2.19 10.59 2.59L2.59 10.59C1.8 11.37 1.8 12.63 2.59 13.41L10.59 21.41C11.37 22.2 12.63 22.2 13.41 21.41L21.41 13.41C22.2 12.63 22.2 11.37 21.41 10.59L13.41 2.59C13 2.19 12.5 2 12 2M12 4L20 12L12 20L4 12L12 4M12 6C9.79 6 8 7.79 8 10C8 12.21 9.79 14 12 14C14.21 14 16 12.21 16 10C16 7.79 14.21 6 12 6Z" fill="currentColor"/></svg>'
  },
  help: {
    name: '使用说明',
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M19 2H14.82C14.4 0.84 13.3 0 12 0C10.7 0 9.6 0.84 9.18 2H5C3.9 2 3 2.9 3 4V18C3 19.1 3.9 20 5 20H9.11C9.56 21.19 10.69 22 12 22C13.31 22 14.44 21.19 14.89 20H19C20.1 20 21 19.1 21 18V4C21 2.9 20.1 2 19 2M12 2C12.55 2 13 2.45 13 3C13 3.55 12.55 4 12 4C11.45 4 11 3.55 11 3C11 2.45 11.45 2 12 2M12 20C11.45 20 11 19.55 11 19C11 18.45 11.45 18 12 18C12.55 18 13 18.55 13 19C13 19.55 12.55 20 12 20M19 18H14.82C14.4 16.84 13.3 16 12 16C10.7 16 9.6 16.84 9.18 18H5V4H9.18C9.6 5.16 10.7 6 12 6C13.3 6 14.4 5.16 14.82 4H19V18M12 9C10.9 9 10 9.9 10 11C10 12.1 10.9 13 12 13C13.1 13 14 12.1 14 11C14 9.9 13.1 9 12 9Z" fill="currentColor"/></svg>'
  },
  contact: {
    name: '联系作者',
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M20 4H4C2.9 4 2.01 4.9 2.01 6L2 18C2 19.1 2.9 20 4 20H20C21.1 20 22 19.1 22 18V6C22 4.9 21.1 4 20 4M20 8L12 13L4 8V6L12 11L20 6V8Z" fill="currentColor"/></svg>'
  },
  about: {
    name: '关于',
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11 7H13V9H11V7M11 11H13V17H11V11M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2M12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/></svg>'
  }
}

const settings = reactive<AppSettings>({
  opacity: 1.0,
  disable_drag: false,
  auto_start: false,
  silent_start: false,
  theme: 'light',
  priority_color: '#FF9800',
  window_level: 'always_on_bottom',
  timeline_deadline_priority: true,
  enable_deadline_notification: false,
  notification_minutes_before: 30,
  sync_enabled: false,
  sync_server_url: ''
})

const opacityValue = computed({
  get: () => settings.opacity,
  set: (value: string | number) => {
    settings.opacity = typeof value === 'string' ? parseFloat(value) : value
  }
})

function toggleTheme() {
  settings.theme = settings.theme === 'light' ? 'dark' : 'light'
  document.body.className = settings.theme === 'dark' ? 'dark-theme' : ''
  invoke('emit_theme_changed', { theme: settings.theme })
}

async function applyOpacityPreview() {
  try {
    await invoke('apply_opacity', { opacity: parseFloat(settings.opacity.toString()) })
  } catch (error) {
    console.error('应用透明度预览失败:', error)
  }
}

async function applyPriorityColorPreview() {
  try {
    await invoke('emit_priority_color_changed', { color: settings.priority_color })
  } catch (error) {
    console.error('应用高优先级颜色预览失败:', error)
  }
}

async function saveSettingsImmediately() {
  try {
    const settingsToSave = {
      opacity: typeof settings.opacity === 'string' ? parseFloat(settings.opacity) : settings.opacity,
      disable_drag: Boolean(settings.disable_drag),
      auto_start: Boolean(settings.auto_start),
      silent_start: Boolean(settings.silent_start),
      theme: settings.theme,
      priority_color: settings.priority_color,
      window_level: settings.window_level,
      timeline_deadline_priority: Boolean(settings.timeline_deadline_priority),
      enable_deadline_notification: Boolean(settings.enable_deadline_notification),
      notification_minutes_before: typeof settings.notification_minutes_before === 'string'
        ? parseInt(settings.notification_minutes_before)
        : settings.notification_minutes_before,
      sync_enabled: Boolean(settings.sync_enabled),
      sync_server_url: settings.sync_server_url || ''
    }

    await invoke('save_app_settings', { settings: settingsToSave })

    if (settingsToSave.theme) {
      await invoke('emit_theme_changed', { theme: settingsToSave.theme })
    }
  } catch (error) {
    console.error('保存设置失败:', error)
  }
}

let saveTimeout: number | null = null
watch(settings, () => {
  if (saveTimeout) {
    clearTimeout(saveTimeout)
  }
  saveTimeout = window.setTimeout(() => {
    saveSettingsImmediately()
  }, 500)
}, { deep: true })

function closeWindow() {
  router.push('/')
}

async function loadSettings() {
  try {
    const loadedSettings = await invoke('load_app_settings') as AppSettings
    originalOpacity.value = loadedSettings.opacity
    Object.assign(settings, loadedSettings)
    document.body.className = settings.theme === 'dark' ? 'dark-theme' : ''
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

async function loadAppVersion() {
  try {
    const version = await invoke('get_app_version') as string
    appVersion.value = version
  } catch (error) {
    console.error('加载应用版本失败:', error)
    appVersion.value = '未知版本'
  }
}

function openBlog() {
  window.open('https://www.feijimiao.cn/contact', '_blank')
}

onMounted(async () => {
  await Promise.all([
    loadSettings(),
    loadAppVersion()
  ])
})
</script>

<style scoped>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

::-webkit-scrollbar {
  width: 0px;
  height: 0px;
  background: transparent;
}

* {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.container {
  width: 100%;
  height: 100vh;
  background: #fafafa;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
}

/* ===== Top Tab Bar ===== */
.tab-bar {
  flex-shrink: 0;
  background: #ffffff;
  border-bottom: 1px solid #e8eaed;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  z-index: 10;
  position: sticky;
  top: 0;
}

.tab-bar-scroll {
  display: flex;
  overflow-x: auto;
  overflow-y: hidden;
  -webkit-overflow-scrolling: touch;
  scrollbar-width: none;
  -ms-overflow-style: none;
  gap: 2px;
  padding: 0 8px;
}

.tab-bar-scroll::-webkit-scrollbar {
  display: none;
}

.tab-item {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 12px 14px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  color: #5f6368;
  white-space: nowrap;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  flex-shrink: 0;
  min-height: 44px;
  border-radius: 0;
  -webkit-tap-highlight-color: transparent;
}

.tab-item::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%) scaleX(0);
  width: 70%;
  height: 2.5px;
  background: #007aff;
  border-radius: 2px 2px 0 0;
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.tab-item.active {
  color: #007aff;
  background: rgba(0, 122, 255, 0.06);
}

.tab-item.active::after {
  transform: translateX(-50%) scaleX(1);
}

.tab-item:active {
  background: rgba(0, 122, 255, 0.08);
}

.tab-icon {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.tab-icon svg {
  width: 18px;
  height: 18px;
}

.tab-label {
  font-size: 13px;
  line-height: 1;
}

/* ===== Content ===== */
.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.content-body {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  background: #fafafa;
  -webkit-overflow-scrolling: touch;
}

.setting-section {
  margin-bottom: 20px;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 11px;
  font-weight: 700;
  color: #8e8e93;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  margin-bottom: 8px;
  padding: 0 2px;
}

.setting-group {
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e8eaed;
  overflow: hidden;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  transition: box-shadow 0.3s ease;
}

.setting-group:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  border-bottom: 1px solid #f0f1f3;
  min-height: 50px;
  transition: background-color 0.2s ease;
  gap: 12px;
}

.setting-item:hover {
  background: #fafbfc;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label-wrap {
  flex: 1;
  min-width: 0;
}

.setting-label {
  font-size: 14px;
  color: #202124;
  font-weight: 600;
  margin-bottom: 2px;
}

.setting-description {
  font-size: 12px;
  color: #5f6368;
  margin-top: 2px;
  line-height: 1.4;
}

.setting-control {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

/* ===== Toggle Switch ===== */
.toggle-switch {
  position: relative;
  width: 51px;
  height: 31px;
  background: #e0e0e0;
  border-radius: 16px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.toggle-switch.active {
  background: #34c759;
  box-shadow: 0 2px 6px rgba(52, 199, 89, 0.3);
}

.toggle-switch::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 25px;
  height: 25px;
  background: #ffffff;
  border-radius: 50%;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.toggle-switch.active::after {
  transform: translateX(20px);
}

/* ===== Range Input ===== */
.setting-control input[type="range"] {
  width: 100px;
  margin-right: 8px;
}

.setting-control select {
  padding: 8px 12px;
  border: 1px solid #e5e5e5;
  border-radius: 8px;
  background: #ffffff;
  color: #000;
  font-size: 17px;
  min-width: 120px;
}

/* ===== Theme Toggle ===== */
.theme-toggle-switch {
  position: relative;
  width: 60px;
  height: 30px;
  background: #e5e5e5;
  border-radius: 15px;
  cursor: pointer;
  transition: background-color 0.3s;
  overflow: hidden;
  flex-shrink: 0;
}

.theme-toggle-switch.theme-dark {
  background: #34c759;
}

.theme-toggle-slider {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 26px;
  height: 26px;
  background: #ffffff;
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.theme-toggle-switch.theme-dark .theme-toggle-slider {
  transform: translateX(30px);
}

.theme-label {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  pointer-events: none;
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-label svg {
  width: 14px;
  height: 14px;
}

.light-label {
  left: 7px;
  color: #fbbf24;
}

.dark-label {
  right: 7px;
  color: #60a5fa;
}

.theme-toggle-switch.theme-dark .light-label {
  color: rgba(251, 191, 36, 0.4);
}

.theme-toggle-switch.theme-dark .dark-label {
  color: #60a5fa;
}

/* ===== Range Value ===== */
.range-value {
  font-size: 14px;
  color: #007aff;
  font-weight: 500;
  min-width: 36px;
  text-align: right;
}

/* ===== Color Picker ===== */
.color-picker-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-picker {
  width: 44px;
  height: 30px;
  border: 2px solid #e5e5e5;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  padding: 0;
}

.color-picker:hover {
  border-color: #007aff;
}

.color-value {
  font-size: 13px;
  color: #5f6368;
  font-weight: 500;
  font-family: 'Courier New', monospace;
}

/* ===== Bottom Bar ===== */
.bottom-bar {
  padding: 12px 16px;
  padding-bottom: calc(12px + env(safe-area-inset-bottom, 0px));
  border-top: 1px solid #e8eaed;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  display: flex;
  justify-content: center;
  flex-shrink: 0;
}

/* ===== Buttons ===== */
.btn {
  padding: 10px 32px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 15px;
  font-weight: 600;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  letter-spacing: 0.2px;
  -webkit-tap-highlight-color: transparent;
}

.btn-primary {
  background: #007aff;
  color: white;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.25);
  width: 100%;
  max-width: 300px;
}

.btn-primary:hover {
  background: #0051d5;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.3);
}

.btn-primary:active {
  transform: scale(0.98);
}

.btn-secondary {
  background: #ffffff;
  color: #007aff;
  border: 2px solid #007aff;
}

.btn-secondary:hover {
  background: rgba(0, 122, 255, 0.08);
}

.btn-secondary:active {
  transform: scale(0.98);
}

/* ===== Help Content ===== */
.help-content {
  padding: 12px 14px;
  line-height: 1.5;
}

.help-content h3 {
  margin: 12px 0 6px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

.help-content p {
  margin: 3px 0;
  font-size: 13px;
  color: #333;
}

/* ===== Update Content ===== */
.update-content {
  padding: 12px 14px;
  line-height: 1.5;
}

.update-content h3 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #007aff;
}

.update-content h4 {
  margin: 10px 0 6px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

.update-content p {
  margin: 3px 0;
  font-size: 13px;
  color: #333;
  padding-left: 6px;
}

/* ===== Contact ===== */
.contact-item {
  display: block;
  padding: 0;
}

.contact-content {
  padding: 20px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.contact-row {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
}

.logo-row {
  gap: 16px;
  flex-wrap: wrap;
  justify-content: center;
}

.contact-logo {
  width: 72px;
  height: 72px;
  border-radius: 36px;
  object-fit: cover;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.blog-btn {
  padding: 9px 20px;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
  display: flex;
  align-items: center;
  gap: 6px;
  -webkit-tap-highlight-color: transparent;
}

.blog-btn svg {
  width: 18px;
  height: 18px;
}

.blog-btn:hover {
  background: #0056cc;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.4);
}

.blog-btn:active {
  transform: scale(0.97);
}

.qrcode-row {
  gap: 24px;
  flex-wrap: wrap;
}

.qrcode-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.qrcode-item h3 {
  margin: 0 0 10px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

.contact-qrcode {
  width: 130px;
  height: 130px;
  border-radius: 10px;
  object-fit: cover;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  border: 1px solid #e5e5e5;
}

.qrcode-tip {
  margin: 6px 0 0 0;
  font-size: 12px;
  color: #6d6d70;
}

.text-row {
  justify-content: center;
}

.contact-text {
  font-size: 14px;
  font-weight: 500;
  color: #007aff;
  margin: 0;
  padding: 8px 18px;
  background: rgba(0, 122, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(0, 122, 255, 0.2);
}

/* ===== Dark Theme ===== */
body.dark-theme .container {
  background: #0a0a0a;
}

body.dark-theme .tab-bar {
  background: #0f0f0f;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.3);
}

body.dark-theme .tab-item {
  color: #808080;
}

body.dark-theme .tab-item.active {
  color: #0a84ff;
  background: rgba(10, 132, 255, 0.12);
}

body.dark-theme .tab-item::after {
  background: #0a84ff;
}

body.dark-theme .tab-item:active {
  background: rgba(10, 132, 255, 0.08);
}

body.dark-theme .content-body {
  background: #0a0a0a;
}

body.dark-theme .section-title {
  color: #8e8e93;
}

body.dark-theme .setting-group {
  background: #141414;
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
}

body.dark-theme .setting-group:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
}

body.dark-theme .setting-item {
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

body.dark-theme .setting-item:hover {
  background: #1a1a1a;
}

body.dark-theme .setting-label {
  color: #e0e0e0;
}

body.dark-theme .setting-description {
  color: #808080;
}

body.dark-theme .toggle-switch {
  background: #202020;
}

body.dark-theme .toggle-switch.active {
  background: #30d158;
  box-shadow: 0 2px 6px rgba(48, 209, 88, 0.5);
}

body.dark-theme .setting-control select {
  padding: 8px 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  background: #1a1a1a;
  color: #e0e0e0;
  font-size: 17px;
  min-width: 120px;
}

body.dark-theme .theme-toggle-switch {
  background: #1a1a1a;
}

body.dark-theme .theme-toggle-switch.theme-dark {
  background: #34c759;
}

body.dark-theme .theme-toggle-slider {
  background: #e7e9ed;
}

body.dark-theme .light-label {
  color: #fbbf24;
}

body.dark-theme .dark-label {
  color: #60a5fa;
}

body.dark-theme .theme-toggle-switch.theme-dark .light-label {
  color: rgba(251, 191, 36, 0.4);
}

body.dark-theme .range-value {
  color: #0a84ff;
}

body.dark-theme .color-picker {
  border-color: rgba(255, 255, 255, 0.1);
}

body.dark-theme .color-picker:hover {
  border-color: #0a84ff;
}

body.dark-theme .color-value {
  color: #808080;
}

body.dark-theme .bottom-bar {
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 15, 15, 0.98);
}

body.dark-theme .btn-primary {
  background: #0a84ff;
  box-shadow: 0 2px 8px rgba(10, 132, 255, 0.4);
}

body.dark-theme .btn-primary:hover {
  background: #0077ed;
  box-shadow: 0 4px 12px rgba(10, 132, 255, 0.5);
}

body.dark-theme .btn-secondary {
  background: #141414;
  color: #0a84ff;
  border: 2px solid #0a84ff;
}

body.dark-theme .btn-secondary:hover {
  background: rgba(10, 132, 255, 0.15);
}

body.dark-theme .help-content h3 {
  color: #e7e9ed;
}

body.dark-theme .help-content p {
  color: #a0a6aa;
}

body.dark-theme .update-content h3 {
  color: #0a84ff;
}

body.dark-theme .update-content h4 {
  color: #e7e9ed;
}

body.dark-theme .update-content p {
  color: #a0a6aa;
}

body.dark-theme .contact-logo {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

body.dark-theme .blog-btn {
  background: #0a84ff;
  box-shadow: 0 2px 8px rgba(10, 132, 255, 0.4);
}

body.dark-theme .blog-btn:hover {
  background: #0056cc;
  box-shadow: 0 4px 12px rgba(10, 132, 255, 0.5);
}

body.dark-theme .contact-qrcode {
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  border: 1px solid #444b4f;
}

body.dark-theme .qrcode-tip {
  color: #a0a6aa;
}

body.dark-theme .contact-text {
  color: #0a84ff;
  background: rgba(10, 132, 255, 0.15);
  border: 1px solid rgba(10, 132, 255, 0.3);
}

body.dark-theme .qrcode-item h3 {
  color: #e7e9ed;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
