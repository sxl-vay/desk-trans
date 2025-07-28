<script setup lang="ts">
import { ref } from 'vue';
import ShortcutInput from './ShortcutInput.vue';

const shortcut = ref('CmdOrCtrl+Shift+T');
</script>

<template>
  <header class="header">
    <h1>⌨️ 快捷键设置</h1>
    <p class="subtitle">设置全局快捷键，便于快速翻译</p>
  </header>
  <div class="main">
    <div class="shortcut-section">
      <ShortcutInput v-model="shortcut" placeholder="点击开始监听或输入快捷键" />
    </div>
  </div>
</template>

<style scoped>
.header {
  text-align: center;
  margin-bottom: 40px;
  margin-top: 32px;
}
.header h1 {
  color: #3b5bfd;
  font-size: 2rem;
  margin-bottom: 10px;
}
.subtitle {
  color: #666;
  font-size: 1.1rem;
  margin: 0;
}
.main {
  max-width: 800px;
  margin: 0 auto;
  flex: 1;
  width: 100%;
}
.shortcut-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.1);
  backdrop-filter: blur(10px);
}
</style> 