<script setup lang="ts">

const props = defineProps<{
  modelValue: string;
}>();
const emit = defineEmits(['update:modelValue']);

const menus = [
  { key: 'shortcut', label: '快捷键设置', icon: '⌨️' },
  { key: 'api', label: '翻译API设置', icon: '🌐' }
];

function selectMenu(key: string) {
  emit('update:modelValue', key);
}
</script>

<template>
  <nav class="side-menu">
    <ul>
      <li v-for="menu in menus" :key="menu.key"
          :class="{ active: props.modelValue === menu.key }"
          @click="selectMenu(menu.key)">
        <span class="icon">{{ menu.icon }}</span>
        <span>{{ menu.label }}</span>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.side-menu {
  width: 180px;
  background: #fff;
  border-right: 1px solid #e5e5e5;
  height: 100vh;
  padding-top: 40px;
  box-sizing: border-box;
}
.side-menu ul {
  list-style: none;
  padding: 0;
  margin: 0;
}
.side-menu li {
  display: flex;
  align-items: center;
  padding: 16px 24px;
  cursor: pointer;
  font-size: 16px;
  color: #333;
  border-left: 4px solid transparent;
  transition: background 0.2s, border-color 0.2s;
}
.side-menu li.active {
  background: #f0f4ff;
  color: #3b5bfd;
  border-left: 4px solid #3b5bfd;
}
.side-menu li:hover {
  background: #f7faff;
}
.icon {
  margin-right: 10px;
  font-size: 18px;
}
</style> 