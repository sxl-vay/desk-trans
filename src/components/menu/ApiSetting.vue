<script setup lang="ts">
import { create, readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { ref, onMounted } from 'vue';

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

const configFile = 'config.json';

// 保存配置
async function saveSetting() {
  const config = {
    api_type: selectedApi.value,
    api_key: apiKey.value
  };
  const jsonStr = JSON.stringify(config, null, 2);

  // 创建文件并写入内容
  const file = await create(configFile, { baseDir: BaseDirectory.AppData });
  await file.write(new TextEncoder().encode(jsonStr));
  await file.close();

  status.value = '保存成功！';

  // 保存后立即读取并更新页面
  await loadSetting();
}

// 读取配置
async function loadSetting() {
  try {
    const content = await readTextFile(configFile, { baseDir: BaseDirectory.AppData });
    const config: { api_type: string; api_key: string } = JSON.parse(content);
    selectedApi.value = config.api_type;
    apiKey.value = config.api_key;
  } catch (e) {
    // 文件不存在时忽略
  }
}

// 页面加载时自动读取
onMounted(() => {
  loadSetting();
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