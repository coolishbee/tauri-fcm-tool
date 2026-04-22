# Tauri FCM Tool

*其他语言版本：[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [Español](README.es.md) | [Français](README.fr.md)*

由 LLM 辅助开发驱动的可扩展开发者工具工作台。

## 概述

Tauri FCM Tool 是一款桌面应用程序，可帮助您通过直观的侧边栏界面构建和管理自己的开发实用工具集合。使用 LLM 辅助即时添加新工具，打造您的个性化生产力工作台。

## 主要特性

- 侧边栏导航，方便访问工具
- LLM 辅助的工具创建和定制
- 基于 Tauri + Rust 后端的原生性能
- 基于类别的工具组织

## 内置工具

### 编码
- **Base64** - 文本与 Base64 编码/解码

### LLM 工具
- **Motion Descriptor** - 将鼠标手势转换为自然语言描述

### 游戏开发
- **Sprite Sheet Describer** - 为精灵图网格添加注释，导出为 JSON
- **Tween Visualizer** - 可视化和比较 30 多种缓动函数

## 快速开始

### 前置要求
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### 开发

```bash
bun install
bun run tauri dev
```

### 生产构建

```bash
bun run tauri build
```

## 技术栈

- 前端：SvelteKit 2.x + Svelte 5, Skeleton UI
- 后端：Rust + Tauri 2.0
- 类型安全：tauri-specta
- 样式：Tailwind CSS 4

## 项目理念

Tauri FCM Tool 不仅仅是一个固定的工具集，而是一个构建您自己工具的框架。利用 LLM 快速搭建实用工具，创建适合您工作流程的工作台。

## 添加新工具

1. 在 `src/lib/config/tools.ts` 中定义工具
2. 创建路由：`src/routes/tools/{category}/{tool-id}/+page.svelte`
3. （可选）在 `src-tauri/src/tools/` 中添加后端命令

详细开发指南请参见 [CLAUDE.md](../Claude.md)。

## 项目结构

```
tauri-fcm-tool/
├── src/                    # 前端 (SvelteKit)
│   ├── routes/tools/      # 工具实现
│   ├── lib/components/    # UI 组件
│   └── lib/config/        # 工具元数据
├── src-tauri/             # 后端 (Rust)
│   ├── src/tools/         # 后端命令
│   └── src/modules/       # 实用工具
└── .github/workflows/     # CI/CD
```

## 发布

推送版本标签以触发 GitHub Actions 自动构建：

```bash
git tag v1.0.0
git push origin v1.0.0
```

构建产物：
- Windows：`.exe` 安装程序
- macOS：Universal `.dmg` + `.app`（Intel + Apple Silicon）
- Linux：`.AppImage` + `.deb`

## 贡献

欢迎贡献！我们欢迎新工具和改进建议。

1. Fork 本仓库
2. 创建您的功能分支
3. 按照项目结构添加您的工具
4. 提交 Pull Request

## 许可证

MIT

## 致谢

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
