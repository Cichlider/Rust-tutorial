# Rust 开发环境完整配置指南

本指南将带你从零开始配置完整的 Rust 开发环境，包括 Rust 工具链、VSCode 编辑器、调试环境以及 Solana 开发工具。

## 📋 目录

1. [安装 Rust 工具链](#1-安装-rust-工具链)
2. [配置 VSCode 编辑器](#2-配置-vscode-编辑器)
3. [创建第一个项目](#3-创建第一个项目)
4. [配置调试环境](#4-配置调试环境)
5. [安装 Solana 开发工具](#5-安装-solana-开发工具可选)
6. [验证环境配置](#6-验证环境配置)
7. [常见问题解决](#7-常见问题解决)

---

## 1. 安装 Rust 工具链

### Windows 系统

1. **访问官方安装页面**
   - 打开浏览器，访问 [https://rustup.rs/](https://rustup.rs/)
   - 下载 `rustup-init.exe` 文件

2. **运行安装程序**
   ```cmd
   # 双击运行下载的 rustup-init.exe
   # 在弹出的命令行窗口中，选择默认安装
   1
   ```

3. **重启终端**
   - 关闭当前命令行窗口
   - 重新打开 PowerShell 或 CMD

### macOS/Linux 系统

1. **一键安装命令**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **选择安装选项**
   ```bash
   # 出现提示时，选择默认安装
   1
   ```

3. **重新加载环境变量**
   ```bash
   source ~/.cargo/env
   # 或者重启终端
   ```

### 验证安装

```bash
# 检查 Rust 编译器版本
rustc --version

# 检查 Cargo 包管理器版本
cargo --version

# 预期输出示例：
# rustc 1.75.0 (82e1608df 2023-12-21)
# cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

---

## 2. 配置 VSCode 编辑器

### 安装 VSCode

如果尚未安装 VSCode：
- 访问 [https://code.visualstudio.com/](https://code.visualstudio.com/)
- 下载并安装适合你操作系统的版本

### 安装必需扩展

打开 VSCode，按 `Ctrl+Shift+X` (Windows/Linux) 或 `Cmd+Shift+X` (macOS) 打开扩展市场：

#### 核心扩展（必装）

1. **rust-analyzer**
   - 作者：rust-lang
   - 功能：Rust 语言服务器，提供智能补全、错误检查等

2. **CodeLLDB**
   - 作者：Vadim Chugunov
   - 功能：调试器支持

3. **Even Better TOML**
   - 作者：tamasfe
   - 功能：Cargo.toml 文件语法高亮和补全

#### 推荐扩展（可选）

4. **Error Lens**
   - 功能：在代码行内直接显示错误信息

5. **Bracket Pair Colorizer 2**
   - 功能：括号配对颜色高亮

6. **GitLens**
   - 功能：增强 Git 集成功能

### 配置 VSCode 设置

#### 方法一：图形界面配置（推荐初学者）

1. **打开设置**
   - 按 `Ctrl+,` (Windows/Linux) 或 `Cmd+,` (macOS)
   - 或点击左下角齿轮图标 ⚙️ → Settings

2. **配置 rust-analyzer**
   
   在搜索框中输入以下关键词并启用相应选项：

   | 搜索关键词 | 设置项 | 操作 |
   |------------|--------|------|
   | `rust-analyzer checkOnSave` | Check On Save › Command | 选择 `cargo check` |
   | `rust-analyzer cargo autoReload` | Cargo: Auto Reload | ✅ 勾选启用 |
   | `rust-analyzer inlayHints` | Inlay Hints: Enable | ✅ 勾选启用 |
   | `rust-analyzer completion autoself` | Completion › Autoself: Enable | ✅ 勾选启用 |
   | `rust-analyzer completion autoimport` | Completion › Autoimport: Enable | ✅ 勾选启用 |

#### 方法二：JSON 配置文件（不推荐，因为主播使用这个办法总是报错）

1. **打开设置 JSON**
   - 按 `Ctrl+Shift+P` (Windows/Linux) 或 `Cmd+Shift+P` (macOS)
   - 输入 `Preferences: Open Settings (JSON)`
   - 按回车

2. **添加配置**
   ```json
   {
       "rust-analyzer.checkOnSave.command": "cargo check",
       "rust-analyzer.cargo.autoReload": true,
       "rust-analyzer.inlayHints.enable": true,
       "rust-analyzer.completion.autoself.enable": true,
       "rust-analyzer.completion.autoimport.enable": true,
       "rust-analyzer.hover.actions.enable": true,
       "rust-analyzer.lens.enable": true
   }
   ```

### 各设置项说明

| 设置项 | 功能说明 |
|--------|----------|
| `checkOnSave.command` | 保存文件时自动运行 `cargo check` 检查错误 |
| `cargo.autoReload` | 修改 `Cargo.toml` 后自动重新加载项目 |
| `inlayHints.enable` | 显示类型提示信息（如变量类型） |
| `completion.autoself.enable` | 自动补全 `self` 关键字 |
| `completion.autoimport.enable` | 自动导入需要的模块 |

---

## 3. 创建第一个项目

### 创建新项目

```bash
# 创建名为 hello_rust 的新项目
cargo new hello_rust

# 进入项目目录
cd hello_rust

# 用 VSCode 打开项目
code .
```

### 项目结构

```
hello_rust/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件（自动生成）
└── src/
    └── main.rs         # 主源代码文件
```

### 编辑代码

打开 `src/main.rs`，你会看到：

```rust
fn main() {
    println!("Hello, world!");
}
```

### 运行项目

在 VSCode 的终端中（按 `Ctrl+\`` 打开终端）：

```bash
# 编译并运行
cargo run

# 预期输出：
# Hello, world!
```

### 其他常用命令

```bash
# 只编译不运行
cargo build

# 检查代码（更快）
cargo check

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查和建议
cargo clippy
```

---

## 4. 配置调试环境（主播跳过了这一步，没有任何关系）
### 创建调试配置

在项目根目录创建 `.vscode` 文件夹和调试配置：

```bash
# 在项目根目录
mkdir .vscode
```

### 创建 launch.json

在 `.vscode/` 目录下创建 `launch.json` 文件：

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'hello_rust'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=hello_rust",
                    "--package=hello_rust"
                ],
                "filter": {
                    "name": "hello_rust",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=hello_rust",
                    "--package=hello_rust"
                ],
                "filter": {
                    "name": "hello_rust",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### 使用调试器

1. **设置断点**
   - 在代码行号左侧点击，出现红点表示断点

2. **开始调试**
   - 按 `F5` 或点击左侧调试图标
   - 选择 "Debug executable 'hello_rust'"

3. **调试控制**
   - `F5`: 继续执行
   - `F10`: 单步跳过
   - `F11`: 单步进入
   - `Shift+F11`: 单步跳出

---

## 5. 安装 Solana 开发工具（可选）

如果你要开发 Solana 区块链应用，需要安装 Solana CLI 和 Anchor 框架。

### 安装 Solana CLI（主播是mac用户，请忽略法1，2，直接使用方法 3 Homebrew安装）

#### 方法一：官方安装脚本

**Windows (PowerShell):**
```powershell
iex (irm https://release.solana.com/stable/install)
```

**macOS/Linux:**
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

#### 方法二：国内镜像（推荐）

如果官方链接访问困难：

```bash
# 使用清华大学镜像
export SOLANA_DOWNLOAD_ROOT=https://mirrors.tuna.tsinghua.edu.cn/solana-releases
sh -c "$(curl -sSfL https://mirrors.tuna.tsinghua.edu.cn/solana/install)"
```

#### 方法三：Homebrew (macOS)

```bash
# 安装 Homebrew（如果未安装）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装 Solana
brew install solana
```

### 配置 Solana

```bash
# 重新加载环境变量
source ~/.bashrc  # 或 source ~/.zshrc

# 验证安装
solana --version

# 设置开发网络
solana config set --url devnet

# 生成密钥对
solana-keygen new --outfile ~/.config/solana/id.json

# 检查配置
solana config get

# 查看地址
solana address

# 申请测试代币
solana airdrop 1
```

### 安装 Anchor 框架

```bash
# 方法1：使用 cargo 安装
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# 方法2：使用 npm 安装
npm install -g @coral-xyz/anchor-cli

# 验证安装
anchor --version
```

### 创建 Anchor 项目

```bash
# 创建新的 Anchor 项目
anchor init my-solana-project
cd my-solana-project

# 用 VSCode 打开
code .

# 构建项目
anchor build

# 运行测试
anchor test
```

---

## 6. 验证环境配置

### 创建测试项目

```bash
# 创建测试项目
cargo new rust_test
cd rust_test
code .
```

### 测试代码示例

将以下代码粘贴到 `src/main.rs`：

```rust
fn main() {
    // 测试基本语法
    let name = "Rust";
    let version = 1.75;
    
    println!("Hello, {}! Version: {}", name, version);
    
    // 测试向量和迭代器
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    
    // 测试结构体
    let person = Person::new("Alice".to_string(), 30);
    person.greet();
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hi, I'm {} and I'm {} years old!", self.name, self.age);
    }
}
```

### 运行测试

```bash
# 运行程序
cargo run

# 预期输出：
# Hello, Rust! Version: 1.75
# Numbers: [1, 2, 3, 4, 5]
# Sum: 15
# Hi, I'm Alice and I'm 30 years old!
```

### 检查 VSCode 功能

确认以下功能正常工作：

- ✅ **语法高亮**: 代码应该有颜色高亮
- ✅ **类型提示**: 鼠标悬停在变量上显示类型信息
- ✅ **自动补全**: 输入时出现补全建议
- ✅ **错误检查**: 错误代码下有红色波浪线
- ✅ **格式化**: 右键选择 "Format Document" 能自动格式化
- ✅ **跳转定义**: Ctrl+Click 能跳转到定义

---

## 7. 常见问题解决

### 问题 1: rust-analyzer 不工作

**症状**: 没有智能补全，没有错误提示

**解决方案**:
```bash
# 方法1: 重启 rust-analyzer 服务
# 在 VSCode 中按 Ctrl+Shift+P，输入 "rust-analyzer: Restart server"

# 方法2: 重新安装 rust-analyzer
# 在扩展市场中卸载后重新安装

# 方法3: 更新 Rust 工具链
rustup update
```

### 问题 2: 编译错误 "linker not found"

**症状**: 编译时报告链接器错误

**解决方案**:

**Windows:**
```bash
# 安装 Visual Studio Build Tools
# 下载地址: https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

**macOS:**
```bash
# 安装 Xcode 命令行工具
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install build-essential
```

### 问题 3: cargo 命令找不到

**症状**: 终端中运行 `cargo` 提示命令不存在

**解决方案**:
```bash
# 手动添加到 PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# 或者重新安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 问题 4: 网络连接问题

**症状**: 下载失败或连接超时

**解决方案**:
```bash
# 使用国内镜像源
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

# 配置 cargo 使用国内源
mkdir ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

### 问题 5: 调试器无法启动

**症状**: 按 F5 调试时报错

**解决方案**:
1. 确认已安装 CodeLLDB 扩展
2. 检查 launch.json 配置是否正确
3. 在 Windows 上可能需要安装 C++ 构建工具

---

## 🎉 配置完成

恭喜！你现在拥有了一个完整的 Rust 开发环境：

- ✅ Rust 工具链（rustc, cargo）
- ✅ VSCode 编辑器配置
- ✅ 智能代码补全和错误检查
- ✅ 调试环境
- ✅ Solana 开发工具（可选）

### 下一步

1. 开始学习 Rust 基础语法
2. 完成 Rust 练习题
3. 阅读《The Rust Programming Language》
4. 参与开源项目

### 有用的资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings 练习](https://github.com/rust-lang/rustlings)
- [Solana 开发文档](https://docs.solana.com/)

现在你可以开始你的 Rust 学习之旅了！🚀