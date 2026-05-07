# 📝 OpNote — 运维知识笔记软件

基于 **Tauri 2 + Vue 3 + Tiptap + SQLite** 的轻量级桌面笔记应用，专为运维知识管理设计。

## ✨ 特性

- 🌲 CherryTree 风格节点树，支持 10 级深度
- ✍️ Tiptap 块编辑器，Markdown 快捷输入
- 🔍 SQLite FTS5 全文搜索
- 💾 单文件 SQLite 存储，零依赖
- 🪶 安装包仅 ~3MB，运行内存 ~30MB

## 🛠️ 技术栈

| 层 | 技术 | 说明 |
|---|------|------|
| 桌面壳 | Tauri 2 | 轻量，Win11 内置 WebView2 |
| 后端 | Rust (Tauri 内置) | 编译进二进制 |
| 前端 | Vue 3 + TypeScript | 响应式 UI |
| 编辑器 | Tiptap (ProseMirror) | 块编辑 + / 菜单 |
| 数据库 | SQLite | 单文件，FTS5 |

## 📥 下载安装

前往 [Releases](https://github.com/zhangzqc/opnote/releases) 下载对应平台的安装包：

- **Windows**: `.msi` 或 `.exe` 安装包（需 Win10+，Win11 自带 WebView2）
- **macOS**: `.dmg` 安装包
- **Linux**: `.deb` 或 `.AppImage`

## 🔨 从源码构建

### 前置要求

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable)
- [Tauri CLI](https://tauri.app/start/prerequisites/) 系统依赖

### 开发模式

```bash
git clone https://github.com/zhangzqc/opnote.git
cd opnote
npm install
npm run tauri dev
```

### 构建安装包

```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`：

| 平台 | 产物路径 |
|------|---------|
| Windows | `msi/OpNote_0.1.0_x64_en-US.msi` |
| macOS | `dmg/OpNote_0.1.0_x64.dmg` |
| Linux | `deb/opnote_0.1.0_amd64.deb` |

### 发布到 GitHub Releases

```bash
# 安装 gh CLI: https://cli.github.com/
gh auth login

# 创建 Release 并上传安装包
gh release create v0.1.0 \
  src-tauri/target/release/bundle/msi/*.msi \
  src-tauri/target/release/bundle/nsis/*.exe \
  --title "OpNote v0.1.0" \
  --notes "首个版本发布 🎉"
```

## 📄 许可证

MIT
