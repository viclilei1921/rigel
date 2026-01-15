# Rigel

## 参考文档地址

[tauri](https://v2.tauri.app/zh-cn/start/create-project/)

## 基本命令

```shell
# 创建项目
cargo create-tauri-app

# 桌面端
npm run tauri dev

# android端
npm run tauri android dev
```

## Eslint

安装eslint

```shell
npm i --save-dev eslint vue-tsc @antfu/eslint-config jiti
```

[配置文件](./eslint.config.ts)

## StyleLint

安装stylelint

```shell
npm i --save-dev less postcss-html
npm i --save-dev stylelint stylelint-config-recommended-vue stylelint-config-standard stylelint-config-standard-less stylelint-config-standard-vue stylelint-order @stylistic/stylelint-plugin
```

[配置文件](./.stylelintrc.mjs)

# 项目架构设计文档

## 1. 技术栈

- **前端框架**: Vue 3 (Composition API)
- **UI 组件库**: Vuetify 3
- **路由管理**: Vue Router 4
- **后端/跨端框架**: Tauri 2.0 (Rust)
- **多媒体处理**: FFmpeg (通过 Tauri 的 Command 调用或侧载)

## 2. 布局设计

- **Main Layout**:
  - **Sidebar (Left)**: 一级路由导航（如：FFmpeg, 设置, 关于）。
  - **Main Content (Right)**:
    - **Titlebar (Top)**: 二级路由导航（针对 FFmpeg 模块：转换、精彩剪辑、合成、剪辑）。
    - **View Area (Center)**: 具体功能展示区域。

## 3. 路由结构

- `/ffmpeg`: 一级路由
  - `/ffmpeg/convert`: 视频转换
  - `/ffmpeg/highlight`: 精彩剪辑合成
  - `/ffmpeg/merge`: 视频合成
  - `/ffmpeg/edit`: 视频剪辑
- `/settings`: 设置
- `/about`: 关于

## 4. FFmpeg 集成方案

- **Tauri Command**: 在 Rust 端定义命令，前端通过 `invoke` 调用。
- **FFmpeg 路径**: 自动检测系统 FFmpeg 或在应用目录下提供 sidecar。
- **解码播放**:
  - 对于浏览器原生支持的格式（MP4, WebM），直接使用 `<video>` 标签。
  - 对于不支持的格式（如 MKV, AVI），使用 FFmpeg 实时转码为流或临时 MP4 文件进行播放。

## 5. 功能模块细化

- **视频转换**: 选择文件 -> 选择目标格式 -> 开始转换 -> 进度显示。
- **视频剪辑**: 选择文件 -> 设置开始/结束时间 -> 预览 -> 导出。
- **视频合成**: 选择多个文件 -> 排序 -> 合成 -> 导出。
- **精彩剪辑**: 自动/手动识别片段 -> 批量剪辑 -> 顺序合成。
