# Aether 编辑器 - 对标 VS Code 的路线图

## 当前状态
- 纯 Rust + Win32 + Direct2D 渲染
- 单文件编辑、基础语法高亮（C语言）、PieceTable 缓冲区
- 欢迎页、侧边栏文件树、撤销/重做、剪贴板
- 性能已优化（缓存 Brush/TextFormat、DrawText、词法分析缓存）

## 第一阶段：核心编辑能力（当前 → 2周）

### 1.1 多文件标签页系统
- 文件: `crates/aether-win32/src/window.rs`
- 新增 `Tab` 结构体管理多个打开的文件
- 标签栏渲染：可滚动、关闭按钮、未保存指示点
- 快捷键：`Ctrl+Tab` 切换、`Ctrl+W` 关闭

### 1.2 多语言语法高亮
- 文件: `crates/aether-core/src/lexer/mod.rs`（新建）
- 实现 Rust、Python、JavaScript/TypeScript、JSON、Markdown、Toml 的词法分析器
- 统一 `Lexer` trait 接口，根据文件扩展名自动选择
- 增量词法分析：只重新 lex 修改的行

### 1.3 查找与替换
- 文件: `crates/aether-win32/src/window.rs`
- `Ctrl+F` 查找框、`Ctrl+H` 替换框
- 支持正则表达式（用 `regex` crate）
- 查找结果高亮、F3/Shift+F3 跳转
- 全部替换、逐个替换

### 1.4 命令面板
- 文件: `crates/aether-win32/src/window.rs`
- `Ctrl+Shift+P` 打开命令面板
- 模糊搜索所有可用命令
- 命令注册系统

## 第二阶段：高级编辑体验（2-4周）

### 2.1 代码折叠
- 基于缩进和语法结构的代码折叠
- 折叠指示器在 gutter 区域
- `Ctrl+Shift+[`/`]` 折叠/展开

### 2.2 多光标编辑
- `Alt+Click` 添加光标
- `Ctrl+D` 选中下一个相同单词
- 多光标同时输入、删除

### 2.3 自动补全（基础版）
- 基于当前文件文本的单词补全
- `Ctrl+Space` 触发
- 简单下拉列表选择

### 2.4  minimap（代码缩略图）
- 右侧显示整文件缩略图
- 当前视口区域高亮
- 点击跳转

### 2.5 括号匹配与自动闭合
- 输入 `(` 自动补全 `)`
- 括号匹配高亮
- 引号自动闭合

## 第三阶段：IDE 级功能（4-8周）

### 3.1 LSP 客户端
- 文件: `crates/aether-core/src/lsp_client.rs`（重建）
- 使用 `tower-lsp` 或自研 JSON-RPC 客户端
- 支持 rust-analyzer、typescript-language-server、python-lsp-server
- 诊断错误（红色波浪线）、代码补全、跳转到定义

### 3.2 集成终端
- 文件: `crates/aether-terminal/src/lib.rs`（重建）
- 使用 Windows ConPTY API
- 底部面板嵌入终端
- `Ctrl+`` 切换

### 3.3 Git 集成
- 文件状态显示在侧边栏和 gutter
- 行内 diff 显示
- 基础 Git 操作（提交、分支切换）

### 3.4 设置系统
- JSON 格式的用户设置文件
- 字体、主题、键绑定可配置
- 设置编辑器 UI

## 第四阶段：性能与打磨（持续）

### 4.1 渲染性能极致优化
- 脏区域渲染：只重绘变化区域
- 文本布局缓存：缓存每行的 TextLayout
- GPU 纹理字体 atlas
- 虚拟滚动：超大文件只渲染可见行

### 4.2 启动速度优化
- 延迟加载非核心模块
- 文件索引后台线程
- 启动时间 < 500ms 目标

### 4.3 内存优化
- 大文件分块加载（>10MB）
- 内存映射持久化
- 行缓存 LRU 淘汰

## 技术栈保持
- **纯 Rust + Direct2D**：无 WebView、无 Electron
- **Win32 API**：原生 Windows 体验
- **PieceTable**：文本缓冲区（已验证）
- **DirectWrite**：文本渲染
- **Direct2D**：2D 图形渲染

## 立即开始：第一阶段 1.1 多文件标签页
