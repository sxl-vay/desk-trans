# 🔤 实时翻译工具

一个基于Tauri + Vue的跨平台桌面翻译应用，支持快捷键监控剪贴板并自动翻译。

## ✨ 功能特性

- **全局快捷键监控**：自定义快捷键，一键获取剪贴板内容并翻译
- **手动翻译**：支持手动输入文本进行翻译
- **多语言支持**：支持多种语言之间的翻译
- **一键复制**：翻译结果可直接复制到剪贴板
- **现代化界面**：美观的渐变背景和卡片式设计
- **跨平台支持**：支持Windows、macOS、Linux

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm (推荐) 或 npm

### 安装依赖

```bash
# 安装前端依赖
pnpm install

# 或使用npm
npm install
```

### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev
```

### 构建应用

```bash
# 构建生产版本
pnpm tauri build
```

## 📖 使用说明

### 1. 快捷键设置

1. 在应用界面中设置您喜欢的快捷键（默认：`CmdOrCtrl+Shift+T`）
2. 点击"注册快捷键"按钮
3. 快捷键将在全局范围内生效

### 2. 自动翻译

1. 复制任意文本到剪贴板
2. 按下设置的快捷键
3. 应用会自动获取剪贴板内容并翻译
4. 翻译结果会显示在界面上

### 3. 手动翻译

1. 在文本框中输入要翻译的内容
2. 点击"翻译"按钮
3. 查看翻译结果

### 4. 复制翻译结果

点击"📋 复制译文"按钮，翻译结果会自动复制到剪贴板。

## 🔧 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri
- **翻译API**: MyMemory Translation API (免费)
- **UI**: 自定义CSS + 渐变设计

## 🎨 界面预览

应用采用现代化的设计风格：
- 渐变背景色
- 卡片式布局
- 响应式设计
- 优雅的动画效果

## 🔑 快捷键说明

- `CmdOrCtrl+Shift+T`：默认翻译快捷键
- 支持自定义快捷键组合
- 支持跨平台快捷键映射

## 🌐 翻译API

当前使用MyMemory Translation API，这是一个免费的翻译服务。您可以根据需要替换为其他翻译API：

- Google Translate API
- DeepL API
- 百度翻译API
- 有道翻译API

## 📝 开发说明

### 项目结构

```
desk-trans/
├── src/                 # Vue前端代码
│   ├── App.vue         # 主应用组件
│   └── main.ts         # 应用入口
├── src-tauri/          # Rust后端代码
│   ├── src/
│   │   ├── main.rs     # 应用入口
│   │   └── lib.rs      # 核心功能实现
│   └── tauri.conf.json # Tauri配置
└── package.json        # 项目配置
```

### 核心功能

1. **剪贴板监控**：使用Tauri的clipboard API
2. **全局快捷键**：使用Tauri的global-shortcut API
3. **HTTP请求**：使用reqwest进行API调用
4. **事件通信**：前后端通过Tauri事件系统通信

## 🤝 贡献

欢迎提交Issue和Pull Request！

## 📄 许可证

MIT License

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式JavaScript框架
- [MyMemory Translation API](https://mymemory.translated.net/) - 免费翻译服务
