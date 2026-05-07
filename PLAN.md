# 📝 OpNote — 运维知识笔记软件 方案 v2

> 基于 v1 反馈调整：Markdown 优先 / Windows 11 优先 / 节点深度 10 级 / 技术栈越轻越好

---

## 一、技术选型（轻量优先）

### 最终选型：Tauri 2 + Vue 3 + SQLite

| 层 | 技术 | 体积 | 选它理由 |
|---|------|------|----------|
| **桌面壳** | **Tauri 2** | 安装包 ~3MB | 比 Wails(15MB) 更轻，比 Electron(150MB) 轻 50 倍 |
| **后端** | **Rust (Tauri 内置)** | 编译进二进制 | Tauri 自带后端能力，不需要额外 Go/Node 服务 |
| **前端** | **Vue 3 + TypeScript** | ~200KB gzip | 生态好，Tiptap 适配完善 |
| **编辑器** | **Tiptap (ProseMirror)** | ~100KB | 块编辑 + / 菜单，Markdown 快捷输入 |
| **数据库** | **SQLite** (via tauri-plugin-sql) | 0 | 零依赖，单文件，FTS5 |
| **节点树** | **vue3-tree** (自研) | ~20KB | CherryTree 风格，轻量实现 |
| **全文搜索** | **SQLite FTS5** | 0 | 内建，后续加 jieba 中文分词 |

### 为什么选 Tauri 2 而不是 Wails？

| 对比 | Tauri 2 | Wails | Electron |
|------|---------|-------|----------|
| 安装包 | **~3MB** | ~15MB | ~150MB |
| 运行内存 | **~30MB** | ~80MB | ~300MB |
| 后端语言 | Rust (内置) | Go (需额外编译) | Node.js |
| Windows 适配 | ✅ WebView2 (Win11 内置) | ✅ | ✅ Chromium |
| Win11 零依赖 | ✅ 系统自带 WebView2 | 需自带 WebView2 | 自带 Chromium |
| 热重载 | ✅ | ✅ | ✅ |
| 生态 | ✅ 插件丰富 | 一般 | ✅ 最丰富 |

**关键点：Win11 自带 WebView2 runtime，Tauri 打包出来的 exe 直接能跑，零额外依赖。**

### 极简架构

```
┌──────────────────────────────────────┐
│         Tauri 2 桌面窗口 (3MB)        │
│  ┌─────────────┐ ┌────────────────┐  │
│  │  节点树      │ │  编辑器        │  │
│  │  (Vue 3)    │ │  (Tiptap)      │  │
│  │             │ │                │  │
│  │ 📁运维知识   │ │ # Nginx 配置   │  │
│  │  📂 Nginx   │ │               │  │
│  │   📄 基础   │ │ ```nginx       │  │
│  │   📄 SSL    │ │ server { ... } │  │
│  │  📂 Docker  │ │ ```            │  │
│  │   📄 入门   │ │               │  │
│  └─────────────┘ └────────────────┘  │
│         │ Tauri Commands (IPC)        │
│  ┌──────▼──────────────────────────┐  │
│  │     Rust 后端 (Tauri 内置)      │  │
│  │  节点CRUD │ 内容读写 │ FTS5搜索  │  │
│  │         │ SQLite                │  │
│  │  ┌──────▼──────────────────┐    │  │
│  │  │  opnote.db (单文件)     │    │  │
│  │  └─────────────────────────┘    │  │
│  └─────────────────────────────────┘  │
└──────────────────────────────────────┘
```

**比 v1 简化了什么：**
- ❌ 去掉 Go 后端 → Rust 由 Tauri 内置，少一层
- ❌ 去掉 Gin/REST API → Tauri IPC 直接调 Rust 函数，更快更轻
- ❌ 去掉 Wails → Tauri 体积只有 Wails 的 1/5

---

## 二、编辑器设计 — Markdown 优先

### 默认行为

- 打开笔记 → **Markdown 编辑模式**（左侧源码 + 右侧实时预览）
- 顶部有切换按钮：`Markdown` | `富文本`
- Markdown 模式下，快捷键自动格式化（Wolai 风格）

### Markdown 快捷键

| 输入 | 自动变为 |
|------|----------|
| `#` + 空格 | # 一级标题 |
| `##` + 空格 | ## 二级标题 |
| `###` + 空格 | ### 三级标题 |
| `####` + 空格 | #### 四级标题 |
| `-` 或 `*` + 空格 | • 无序列表 |
| `1.` + 空格 | 1. 有序列表 |
| `- [ ]` + 空格 | ☐ 待办列表 |
| `` ``` `` + 回车 | 代码块（弹出语言选择） |
| `` ```bash `` + 回车 | bash 代码块 |
| `>` + 空格 | > 引用块 |
| `---` + 回车 | --- 分割线 |
| `**text**` | **粗体**（实时渲染） |
| `*text*` | *斜体*（实时渲染） |
| `~~text~~` | ~~删除线~~（实时渲染） |
| `` `code` `` | `行内代码`（实时渲染） |
| `[text](url)` | [超链接]（实时渲染） |

### / 斜杠菜单（第一版）

```
输入 / 弹出：
┌──────────────────────┐
│ 🔍 搜索块...          │
├──────────────────────┤
│ 📝 正文        段落    │
│ # 一级标题      H1    │
│ ## 二级标题     H2    │
│ ### 三级标题    H3    │
│ 💻 代码块      Code   │
│ 📋 引用       Quote  │
│ • 无序列表    Bullet  │
│ 1. 有序列表   Number  │
│ ☐ 待办列表    Todo   │
│ ── 分割线    Divider  │
│ 📊 表格       Table  │
│ 🔗 链接       Link   │
│ 🖼️ 图片      Image  │
└──────────────────────┘
```

- 上下键选择，回车插入
- 输入 `/h1` `/code` `/table` 快速过滤

### 富文本模式

- 切换到富文本模式后，Markdown 源码被 Tiptap 解析渲染
- 所见即所得编辑
- 可随时切回 Markdown 模式，内容双向转换

---

## 三、节点树设计 — CherryTree 风格 + 10 级深度

### 视觉规范

```
📕 运维知识库                  ← Level 0 (根) 14px 加粗 📕
├── 🌐 Nginx                  ← Level 1       14px 加粗 🌐
│   ├── 📄 基础配置            ← Level 2       13px 正常 📄
│   ├── 📄 SSL 证书配置        ← Level 2       13px 正常
│   └── 📄 反向代理            ← Level 2       13px 正常
├── 🐳 Docker                 ← Level 1       14px 加粗 🐳
│   ├── 📄 常用命令            ← Level 2       13px 正常
│   ├── 📂 部署方案            ← Level 2       13px 正常 📂
│   │   ├── 📄 生产环境        ← Level 3       12px 正常
│   │   └── 📄 测试环境        ← Level 3       12px 正常
│   └── 📄 Compose 模板        ← Level 2       13px 正常
├── 🖥️ Linux                  ← Level 1       14px 加粗
│   ├── 📄 磁盘管理            ← Level 2       13px 正常
│   ├── 📂 网络排查            ← Level 2       13px 正常
│   │   ├── 📄 TCP 排查        ← Level 3       12px 正常
│   │   ├── 📄 DNS 排查        ← Level 3       12px 正常
│   │   └── 📂 深入分析        ← Level 3       12px 正常
│   │       └── 📄 抓包方法    ← Level 4       12px 正常
│   └── 📄 进程管理            ← Level 2       13px 正常
└── 📋 命令速查                ← Level 1       14px 加粗
```

### 层级样式

| Level | 字号 | 字重 | 缩进 | 图标 |
|-------|------|------|------|------|
| 0 (根) | 15px | Bold | 0px | 📕 (固定) |
| 1 | 14px | Bold | 20px | 分类图标 |
| 2 | 13px | Normal | 40px | 📄/📂/💻 |
| 3-4 | 12px | Normal | 60px/80px | 📄/📂/💻 |
| 5-9 | 12px | Normal | 100px+ | 📄/📂/💻 |

### 节点类型

| 类型 | 图标 | 说明 |
|------|------|------|
| 富文本节点 | 📄 | 默认类型，Markdown/富文本编辑 |
| 代码节点 | 💻 | 纯代码，语法高亮，一键复制 |
| 文件夹节点 | 📂 | 仅做分类，不承载内容 |

### 右键菜单

```
右键节点弹出：
┌────────────────────┐
│ ➕ 新建子节点       │
│ 📂 新建文件夹       │
│ 💻 新建代码节点     │
├────────────────────┤
│ ✏️ 重命名           │
│ 📋 复制             │
│ 🗑️ 删除             │
├────────────────────┤
│ ⬆️ 上移             │
│ ⬇️ 下移             │
│ 🔄 升级(减少缩进)  │
│ 🔄 降级(增加缩进)  │
├────────────────────┤
│ 📥 导出为 Markdown  │
│ 📥 导出为 HTML      │
└────────────────────┘
```

### 拖拽

- 拖拽节点调整排序
- 拖拽到另一个节点上 → 成为该节点的子节点
- 拖拽到节点之间 → 插入为同级
- 最深 10 级，超限提示

---

## 四、数据库设计

```sql
-- 节点表 (支持 10 级深度)
CREATE TABLE nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id   INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    node_type   TEXT NOT NULL DEFAULT 'rich_text',  -- rich_text / code / folder
    icon        TEXT,           -- 自定义图标 emoji
    syntax      TEXT,           -- 代码节点语言: bash/python/nginx/sql...
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_expanded BOOLEAN DEFAULT 1,
    created_at  TEXT DEFAULT (datetime('now')),
    updated_at  TEXT DEFAULT (datetime('now')),
    CHECK (node_type IN ('rich_text', 'code', 'folder'))
);

-- 深度校验：应用层在插入/移动时检查，不超过 10 级

-- 内容表
CREATE TABLE contents (
    node_id     INTEGER PRIMARY KEY REFERENCES nodes(id) ON DELETE CASCADE,
    content     TEXT NOT NULL DEFAULT '',
    format      TEXT NOT NULL DEFAULT 'markdown',  -- markdown / html
    word_count  INTEGER DEFAULT 0,
    char_count  INTEGER DEFAULT 0,
    updated_at  TEXT DEFAULT (datetime('now'))
);

-- 全文搜索 (FTS5)
CREATE VIRTUAL TABLE contents_fts USING fts5(
    name,
    content,
    content=contents,
    content_rowid=node_id,
    tokenize='unicode61'
);

-- 标签
CREATE TABLE tags (
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    name  TEXT NOT NULL UNIQUE
);
CREATE TABLE node_tags (
    node_id INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    tag_id  INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (node_id, tag_id)
);

-- 自动更新 updated_at
CREATE TRIGGER nodes_updated_at
    AFTER UPDATE ON nodes
    FOR EACH ROW
BEGIN
    UPDATE nodes SET updated_at = datetime('now') WHERE id = NEW.id;
END;
```

### 数据文件位置 (Windows)

```
%APPDATA%\opnote\
├── opnote.db          ← 主数据库
├── config.toml        ← 配置
├── attachments\       ← 图片/附件
└── backups\           ← 自动备份
```

---

## 五、项目结构

```
opnote/
├── src-tauri/                  # Rust 后端 (Tauri)
│   ├── Cargo.toml
│   ├── tauri.conf.json         # Tauri 配置
│   ├── src/
│   │   ├── main.rs             # 入口
│   │   ├── lib.rs              # Tauri 命令注册
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── init.rs         # SQLite 初始化 + 迁移
│   │   │   └── models.rs       # 数据模型
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── node.rs         # 节点 CRUD 命令
│   │   │   ├── content.rs      # 内容读写命令
│   │   │   ├── search.rs       # 搜索命令
│   │   │   └── export.rs       # 导出命令
│   │   └── utils.rs
│   └── icons/                  # 应用图标
│
├── src/                        # Vue 3 前端
│   ├── App.vue
│   ├── main.ts
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppLayout.vue   # 左右分栏布局
│   │   │   └── TitleBar.vue    # 自定义标题栏
│   │   ├── tree/
│   │   │   ├── NodeTree.vue    # 节点树容器
│   │   │   ├── TreeNode.vue    # 单个节点（递归）
│   │   │   ├── TreeContextMenu.vue
│   │   │   └── TreeStore.ts    # 树状态管理
│   │   ├── editor/
│   │   │   ├── EditorPane.vue      # 编辑器面板(切换模式)
│   │   │   ├── MarkdownEditor.vue  # Markdown 编辑+预览
│   │   │   ├── RichTextEditor.vue  # Tiptap 富文本
│   │   │   ├── SlashMenu.vue       # / 斜杠菜单
│   │   │   ├── EditorToolbar.vue   # 工具栏
│   │   │   └── extensions/
│   │   │       ├── slash-command.ts
│   │   │       ├── markdown-shortcuts.ts
│   │   │       └── code-block-copy.ts
│   │   └── common/
│   │       ├── SearchBar.vue
│   │       └── StatusBar.vue
│   ├── stores/
│   │   ├── nodeStore.ts        # Pinia: 节点状态
│   │   └── editorStore.ts      # Pinia: 编辑器状态
│   ├── styles/
│   │   ├── global.css
│   │   ├── tree.css
│   │   └── editor.css
│   └── lib/
│       └── tauri-ipc.ts        # Tauri IPC 封装
│
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

---

## 六、Tauri IPC 命令（替代 REST API）

```rust
// 节点命令
#[tauri::command]
async fn get_node_tree() -> Result<Vec<TreeNode>, String>

#[tauri::command]
async fn create_node(parent_id: Option<i64>, name: String, node_type: String) -> Result<Node, String>

#[tauri::command]
async fn update_node(id: i64, name: Option<String>, icon: Option<String>) -> Result<(), String>

#[tauri::command]
async fn delete_node(id: i64) -> Result<(), String>

#[tauri::command]
async fn move_node(id: i64, parent_id: Option<i64>, sort_order: i32) -> Result<(), String>

// 内容命令
#[tauri::command]
async fn get_content(node_id: i64) -> Result<Content, String>

#[tauri::command]
async fn save_content(node_id: i64, content: String, format: String) -> Result<(), String>

// 搜索命令
#[tauri::command]
async fn search_nodes(query: String) -> Result<Vec<SearchResult>, String>

// 导出命令
#[tauri::command]
async fn export_node(node_id: i64, format: String, path: String) -> Result<(), String>
```

前端调用：
```typescript
import { invoke } from '@tauri-apps/api/core'

const tree = await invoke('get_node_tree')
await invoke('save_content', { nodeId: 1, content: '...', format: 'markdown' })
```

---

## 七、Windows 11 适配要点

| 要点 | 方案 |
|------|------|
| WebView2 | Win11 自带，无需额外安装 |
| 安装包 | Tauri + NSIS/MSI → ~3MB 安装包 |
| 窗口风格 | Win11 Mica/Acrylic 半透明效果 (Tauri 2 支持) |
| 系统托盘 | tauri-plugin-tray |
| 全局快捷键 | tauri-plugin-global-shortcut |
| 文件关联 | .opnote 文件双击打开 |
| 自启动 | 可选开机启动 |
| 数据目录 | %APPDATA%\opnote\ |
| 字体 | 默认微软雅黑 + Consolas(代码) |

---

## 八、开发路线

### Phase 1 — 能用（3 周）

| 周 | 交付 |
|---|------|
| W1 | Tauri 2 项目初始化 + Rust 后端 SQLite + 节点 CRUD IPC |
| W2 | Vue 前端骨架 + 节点树组件(展开/折叠/选中/右键/10级深度) |
| W3 | Markdown 编辑器 + / 斜杠菜单 + 自动保存 + 联调 |

**完成标准：**
- ✅ Win11 上双击 exe 能跑
- ✅ 10 级节点树：创建/删除/拖拽/展开折叠
- ✅ Markdown 编辑 + 实时预览
- ✅ / 斜杠菜单弹出基础块
- ✅ 内容自动保存到 SQLite
- ✅ 全文搜索

### Phase 2 — 好用（3 周）

- 代码节点 + 语法高亮 + 一键复制
- 快捷键体系 (Ctrl+N/S/F/P)
- 标签系统
- 导入 Markdown 文件夹
- 导出 HTML
- 暗色主题
- Win11 Mica 效果

### Phase 3 — 好看 + 分发（2 周）

- 安装包 (NSIS)
- 系统托盘
- 自动备份
- 版本历史 (内容 diff)
- 附件/图片管理

### Phase 4 — Web 端（后续）

- Rust 后端改用 Axum 跑 HTTP 服务
- 同一套前端，加鉴权层
- 多用户协作

---

## 九、开发环境搭建 (Windows 11)

```powershell
# 1. 安装 Rust
winget install Rustlang.Rustup

# 2. 安装 Node.js (LTS)
winget install OpenJS.NodeJS.LTS

# 3. 创建 Tauri 2 项目
npm create tauri-app@latest -- opnote --template vue-ts

# 4. 安装前端依赖
cd opnote
npm install
npm install @tiptap/vue-3 @tiptap/starter-kit @tiptap/extension-placeholder
npm install @tauri-apps/plugin-sql
npm install pinia

# 5. 开发模式
npm run tauri dev

# 6. 构建发布
npm run tauri build
```

---

## 十、与 v1 方案的变更对照

| 项目 | v1 | v2 | 变更原因 |
|------|----|----|----------|
| 桌面壳 | Wails (Go) | **Tauri 2 (Rust)** | 安装包 3MB vs 15MB，更轻 |
| 后端 | Go + Gin | **Rust (Tauri 内置)** | 少一层，IPC 直调 |
| API | REST API | **Tauri IPC** | 更快更轻，无需 HTTP |
| 编辑器默认 | 未定 | **Markdown 优先** | 用户要求 |
| 节点深度 | 建议 3 级 | **10 级** | 用户要求 |
| 平台优先 | Linux | **Windows 11** | 用户要求 |
| Vim 模式 | 待定 | **不需要** | 用户要求 |
