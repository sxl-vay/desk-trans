<script setup lang="ts">
import { ref } from "vue";
import SideMenu from "./components/SideMenu.vue";
import HomePage from "./components/HomePage.vue";
import ApiSetting from "./components/menu/ApiSetting.vue";
import ShortcutSetting from "./components/menu/ShortcutSetting.vue";

const activeMenu = ref('translate');
</script>

<template>
  <div class="app-layout">
    <SideMenu v-model="activeMenu" />
    <div class="main-content">
      <main class="main">
        <template v-if="activeMenu === 'translate'">·
          <HomePage />
        </template>
        <template v-else-if="activeMenu === 'shortcut'">
          <ShortcutSetting />
        </template>
        <template v-else-if="activeMenu === 'api'">
          <ApiSetting />
        </template>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
.main-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.main {
  max-width: 800px;
  margin: 0 auto;
  flex: 1;
  width: 100%;
}
@media (max-width: 768px) {
  .app-layout {
    flex-direction: column;
  }
  .main-content {
    padding: 0 8px;
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