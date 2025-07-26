<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { create, BaseDirectory } from '@tauri-apps/plugin-fs';

const apiTypes = [
  { label: 'MyMemory（免费）', value: 'mymemory' },
  { label: '百度翻译', value: 'baidu' },
  { label: '有道翻译', value: 'youdao' },
  { label: 'Google翻译', value: 'google' },
  { label: 'DeepL', value: 'deepl' }
];

const selectedApi = ref('mymemory');
const apiKey = ref('');
const status = ref('');

// 保存到本地
async function saveSetting() {
  try {
    const file = await create('foo/bar.txt', { baseDir: BaseDirectory.AppData });
    await file.write(new TextEncoder().encode('Hello world'));
    await file.close();

    await invoke('save_api_config', {
      config: {
        api_type: selectedApi.value,
        api_key: apiKey.value
      }
    });
    status.value = '保存成功！';
    setTimeout(() => status.value = '', 1500);
  } catch (error) {
    status.value = `保存失败: ${error}`;
    setTimeout(() => status.value = '', 3000);
  }
}

// 读取本地配置
onMounted(async () => {
  try {
    const config = await invoke('load_api_config');
    selectedApi.value = config.api_type;
    apiKey.value = config.api_key;
  } catch (error) {
    console.error('加载配置失败:', error);
  }
});
</script>

<template>
  <div class="api-setting">
    <h3>翻译API设置</h3>
    <div class="form-row">
      <label>API类型：</label>
      <select v-model="selectedApi">
        <option v-for="api in apiTypes" :key="api.value" :value="api.value">{{ api.label }}</option>
      </select>
    </div>
    <div class="form-row">
      <label>API Key：</label>
      <input v-model="apiKey" placeholder="请输入API Key（如有）" />
    </div>
    <button class="save-btn" @click="saveSetting">保存</button>
    <span v-if="status" class="status">{{ status }}</span>
  </div>
</template>

<style scoped>
.api-setting {
  max-width: 400px;
  margin: 40px auto;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 16px rgba(60,80,180,0.06);
  padding: 32px 32px 24px 32px;
}
.api-setting h3 {
  margin-bottom: 24px;
  color: #3b5bfd;
  font-size: 1.2rem;
}
.form-row {
  display: flex;
  align-items: center;
  margin-bottom: 18px;
}
.form-row label {
  width: 80px;
  color: #555;
}
.form-row input, .form-row select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #e5e5e5;
  border-radius: 6px;
  font-size: 15px;
}
.save-btn {
  background: linear-gradient(135deg, #3b5bfd 0%, #667eea 100%);
  color: #fff;
  border: none;
  border-radius: 8px;
  padding: 10px 32px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  margin-top: 10px;
  transition: background 0.2s;
}
.save-btn:hover {
  background: linear-gradient(135deg, #667eea 0%, #3b5bfd 100%);
}
.status {
  margin-left: 16px;
  color: #28a745;
  font-size: 14px;
}
</style> 