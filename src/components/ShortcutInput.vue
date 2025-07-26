<script setup lang="ts">
import { ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  modelValue?: string;
  placeholder?: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
  (e: 'register', shortcut: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "点击开始监听或输入快捷键"
});

const emit = defineEmits<Emits>();

const isListeningShortcut = ref(false);
const shortcutInputRef = ref<HTMLInputElement | null>(null);
const shortcut = ref(props.modelValue);

// 开始监听快捷键输入
function startListeningShortcut() {
  isListeningShortcut.value = true;
  shortcut.value = "";
  emit('update:modelValue', "");
  
  // 聚焦到输入框
  nextTick(() => {
    if (shortcutInputRef.value) {
      shortcutInputRef.value.focus();
    }
  });
}

// 停止监听快捷键输入
function stopListeningShortcut() {
  isListeningShortcut.value = false;
}

// 处理键盘事件
function handleKeyDown(event: KeyboardEvent) {
  if (!isListeningShortcut.value) return;
  
  event.preventDefault();
  event.stopPropagation();
  
  const keys: string[] = [];
  
  // 检查修饰键
  if (event.metaKey) {
    keys.push("meta");
  }

  if (event.ctrlKey) {
    keys.push("ctrlKey");
  }
  
  if (event.altKey) {
    keys.push("Alt");
  }
  if (event.shiftKey) {
    keys.push("Shift");
  }
  
  // 添加主键
  if (event.key !== "Meta" && event.key !== "Control" && event.key !== "Alt" && event.key !== "Shift") {
    keys.push(event.key.toUpperCase());
  }
  
  if (keys.length > 0) {
    const shortcutValue = keys.join("+");
    shortcut.value = shortcutValue;
    emit('update:modelValue', shortcutValue);
    stopListeningShortcut();
  }
}

// 处理键盘释放事件
function handleKeyUp(event: KeyboardEvent) {
  if (!isListeningShortcut.value) return;
  
  // 如果所有修饰键都释放了，停止监听
  if (!event.metaKey && !event.ctrlKey && !event.altKey && !event.shiftKey) {
    if (shortcut.value === "") {
      stopListeningShortcut();
    }
  }
}

// 注册快捷键
async function registerShortcut() {
  if (!shortcut.value) return;
  
  try {
    await invoke("register_shortcut", { shortcut: shortcut.value });
    emit('register', shortcut.value);
  } catch (error) {
    console.error('注册快捷键失败:', error);
  }
}

// 监听props变化
import { watch } from 'vue';
watch(() => props.modelValue, (newValue) => {
  shortcut.value = newValue;
});
</script>

<template>
  <div class="shortcut-input-container">
    <div class="shortcut-input">
      <input 
        ref="shortcutInputRef"
        v-model="shortcut" 
        :placeholder="isListeningShortcut ? '请按下快捷键组合...' : placeholder"
        class="shortcut-field"
        :class="{ 'listening': isListeningShortcut }"
        @focus="startListeningShortcut"
        @blur="stopListeningShortcut"
        @keydown="handleKeyDown"
        @keyup="handleKeyUp"
        readonly
      />
      <button @click="startListeningShortcut" class="listen-btn">
        {{ isListeningShortcut ? '停止监听' : '开始监听' }}
      </button>
      <button @click="registerShortcut" class="register-btn" :disabled="!shortcut">
        注册快捷键
      </button>
    </div>
    <p class="hint">💡 点击输入框或"开始监听"按钮，然后按下快捷键组合</p>
  </div>
</template>

<style scoped>
.shortcut-input-container {
  width: 100%;
}

.shortcut-input {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.shortcut-field {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.3s;
  background-color: white;
}

.shortcut-field:focus {
  outline: none;
  border-color: #667eea;
}

.shortcut-field.listening {
  border-color: #ff6b6b;
  background-color: #fff5f5;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(255, 107, 107, 0.7);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(255, 107, 107, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(255, 107, 107, 0);
  }
}

.listen-btn {
  padding: 12px 16px;
  background: linear-gradient(135deg, #ff6b6b 0%, #ee5a52 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s;
  font-size: 14px;
  white-space: nowrap;
}

.listen-btn:hover {
  transform: translateY(-2px);
}

.register-btn {
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s;
  white-space: nowrap;
}

.register-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}

.register-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.hint {
  color: #666;
  font-size: 14px;
  margin: 0;
}

@media (max-width: 768px) {
  .shortcut-input {
    flex-direction: column;
  }
  
  .listen-btn,
  .register-btn {
    align-self: stretch;
  }
}
</style> 