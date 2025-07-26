<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import ShortcutInput from "./components/ShortcutInput.vue";

interface TranslationResult {
  original: string;
  translated: string;
  from: string;
  to: string;
}

const shortcut = ref("CmdOrCtrl+Shift+T");
const inputText = ref("");
const translationResult = ref<TranslationResult | null>(null);
const isTranslating = ref(false);
const statusMessage = ref("");

// 注册快捷键
async function registerShortcut(shortcutValue: string) {
  try {
    statusMessage.value = `快捷键 ${shortcutValue} 注册成功！`;
  } catch (error) {
    statusMessage.value = `快捷键注册失败: ${error}`;
  }
}

// 手动翻译
async function translateText() {
  if (!inputText.value.trim()) return;
  
  isTranslating.value = true;
  try {
    const result = await invoke("translate_text_command", {
      request: {
        text: inputText.value,
        from: null,
        to: "zh"
      }
    });
    translationResult.value = result as TranslationResult;
    statusMessage.value = "翻译完成！";
  } catch (error) {
    statusMessage.value = `翻译失败: ${error}`;
  } finally {
    isTranslating.value = false;
  }
}

// 复制翻译结果到剪贴板
async function copyToClipboard() {
  if (!translationResult.value) return;
  
  try {
    await invoke("set_clipboard_text", { text: translationResult.value.translated });
    statusMessage.value = "翻译结果已复制到剪贴板！";
  } catch (error) {
    statusMessage.value = `复制失败: ${error}`;
  }
}

// 监听翻译结果事件
let unlisten: (() => void) | null = null;

onMounted(async () => {
  // 注册默认快捷键
  await registerShortcut();
  
  // 监听翻译结果
  unlisten = await listen("translation-result", (event) => {
    translationResult.value = event.payload as TranslationResult;
    statusMessage.value = "自动翻译完成！";
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});
</script>

<template>
  <div class="app">
    <header class="header">
      <h1>🔤 实时翻译工具</h1>
      <p class="subtitle">监控剪贴板，一键翻译</p>
    </header>

    <main class="main">
      <!-- 快捷键设置 -->
      <div class="shortcut-section">
        <h3>⚡ 快捷键设置</h3>
        <ShortcutInput 
          v-model="shortcut"
          placeholder="点击开始监听或输入快捷键"
          @register="registerShortcut"
        />
      </div>

      <!-- 手动翻译 -->
      <div class="translate-section">
        <h3>✍️ 手动翻译</h3>
        <div class="translate-input">
          <textarea 
            v-model="inputText" 
            placeholder="输入要翻译的文本..."
            class="text-area"
            rows="4"
          ></textarea>
          <button 
            @click="translateText" 
            :disabled="isTranslating || !inputText.trim()"
            class="translate-btn"
          >
            {{ isTranslating ? '翻译中...' : '翻译' }}
          </button>
        </div>
      </div>

      <!-- 翻译结果 -->
      <div v-if="translationResult" class="result-section">
        <h3>📝 翻译结果</h3>
        <div class="result-card">
          <div class="original-text">
            <h4>原文:</h4>
            <p>{{ translationResult.original }}</p>
          </div>
          <div class="translated-text">
            <h4>译文:</h4>
            <p>{{ translationResult.translated }}</p>
          </div>
          <div class="result-actions">
            <button @click="copyToClipboard" class="copy-btn">
              📋 复制译文
            </button>
          </div>
        </div>
      </div>

      <!-- 状态消息 -->
      <div v-if="statusMessage" class="status-message">
        {{ statusMessage }}
      </div>
    </main>
  </div>
</template>

<style scoped>
.app {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
}

.header {
  text-align: center;
  margin-bottom: 40px;
}

.header h1 {
  color: white;
  font-size: 2.5rem;
  margin-bottom: 10px;
  text-shadow: 0 2px 4px rgba(0,0,0,0.3);
}

.subtitle {
  color: rgba(255,255,255,0.9);
  font-size: 1.1rem;
  margin: 0;
}

.main {
  max-width: 800px;
  margin: 0 auto;
}

.shortcut-section,
.translate-section,
.result-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
  backdrop-filter: blur(10px);
}

.shortcut-section h3,
.translate-section h3,
.result-section h3 {
  margin: 0 0 20px 0;
  color: #333;
  font-size: 1.3rem;
}



.translate-input {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.text-area {
  width: 100%;
  padding: 16px;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  font-size: 14px;
  resize: vertical;
  min-height: 100px;
  font-family: inherit;
  transition: border-color 0.3s;
}

.text-area:focus {
  outline: none;
  border-color: #667eea;
}

.translate-btn {
  align-self: flex-end;
  padding: 12px 32px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.translate-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}

.translate-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.result-card {
  background: #f8f9fa;
  border-radius: 12px;
  padding: 20px;
  border: 1px solid #e9ecef;
}

.original-text,
.translated-text {
  margin-bottom: 20px;
}

.original-text h4,
.translated-text h4 {
  margin: 0 0 8px 0;
  color: #495057;
  font-size: 14px;
  font-weight: 600;
}

.original-text p,
.translated-text p {
  margin: 0;
  padding: 12px;
  background: white;
  border-radius: 8px;
  border: 1px solid #dee2e6;
  line-height: 1.5;
  word-wrap: break-word;
}

.result-actions {
  display: flex;
  justify-content: flex-end;
}

.copy-btn {
  padding: 8px 16px;
  background: #28a745;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.copy-btn:hover {
  background: #218838;
}

.status-message {
  background: rgba(255,255,255,0.9);
  padding: 12px 20px;
  border-radius: 8px;
  text-align: center;
  color: #333;
  font-weight: 500;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

@media (max-width: 768px) {
  .app {
    padding: 16px;
  }
  
  .header h1 {
    font-size: 2rem;
  }
  
  .shortcut-input {
    flex-direction: column;
  }
  
  .register-btn {
    align-self: stretch;
  }
}
</style>
<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  line-height: 1.6;
}

#app {
  min-height: 100vh;
}
</style>