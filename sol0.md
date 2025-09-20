# Solana 智能合约开发教程

> 从 Rust 基础到 Solana 区块链开发

## 🎯 学习目标

掌握 Solana 智能合约（Program）开发，能够：
- 理解 Solana 的账户模型
- 使用 Anchor 框架开发智能合约
- 部署和测试合约
- 与合约进行交互

---

## 前置知识检查

在开始之前，确保你已经：
- ✅ 掌握 Rust 基础语法（变量、函数、结构体、所有权）
- ✅ 安装了 Solana CLI 和 Anchor 框架
- ✅ 配置了开发环境

如果还没有，请先完成环境配置。

---

## 第1章：区块链基础概念

### 1.1 什么是智能合约？

智能合约是运行在区块链上的程序，具有以下特点：
- **不可篡改**：部署后代码无法修改
- **去中心化**：运行在区块链网络上
- **自动执行**：满足条件时自动执行
- **透明公开**：所有人都能查看代码

### 1.2 Solana vs 以太坊

| 特性 | Solana | 以太坊 |
|------|--------|--------|
| **编程语言** | Rust, C, C++ | Solidity |
| **交易速度** | ~65,000 TPS | ~15 TPS |
| **交易费用** | $0.00025 | $1-50+ |
| **共识机制** | Proof of History + PoS | Proof of Stake |
| **账户模型** | Account Model | Account Model |

### 1.3 Solana 的核心概念

**Account（账户）**：
- Solana 上的一切都是账户
- 存储数据和 SOL 代币
- 有唯一的公钥地址

**Program（程序）**：
- 智能合约在 Solana 上叫 Program
- 存储可执行代码
- 无状态（状态存储在其他账户中）

**Transaction（交易）**：
- 包含一个或多个指令
- 消耗 SOL 作为手续费

---

## 第2章：Anchor 框架入门

### 2.1 什么是 Anchor？

Anchor 是 Solana 的开发框架，类似于：
- 以太坊的 Hardhat/Truffle
- Web 开发的 Express.js

**Anchor 的优势**：
- 简化合约开发
- 自动生成客户端代码
- 内置安全检查
- 优秀的测试工具

### 2.2 创建第一个 Anchor 项目

```bash
# 创建新项目
anchor init my_first_program
cd my_first_program

# 项目结构
# ├── Anchor.toml          # 项目配置
# ├── Cargo.toml           # Rust 配置
# ├── programs/            # 智能合约代码
# │   └── my_first_program/
# │       ├── Cargo.toml
# │       └── src/
# │           └── lib.rs   # 主合约文件
# ├── tests/               # 测试文件
# └── app/                 # 前端代码（可选）
```

### 2.3 基本合约结构

```rust
// programs/my_first_program/src/lib.rs
use anchor_lang::prelude::*;

// 声明程序 ID（部署后会自动生成）
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// 程序模块
#[program]
pub mod my_first_program {
    use super::*;

    // 指令函数
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

// 指令的账户结构
#[derive(Accounts)]
pub struct Initialize {}
```

---

## 第3章：练习1 - Hello World 合约

### 目标
创建一个最简单的 Solana 程序，打印 "Hello, Solana!"

### 步骤

**1. 修改合约代码**

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>, name: String) -> Result<()> {
        msg!("Hello, {}! Welcome to Solana!", name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
```

**2. 构建和部署**

```bash
# 构建合约
anchor build

# 部署到本地网络
anchor deploy
```

**3. 编写测试**

```typescript
// tests/hello_solana.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";

describe("hello_solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;

  it("Says hello!", async () => {
    const tx = await program.methods
      .sayHello("Solana Developer")
      .rpc();
    
    console.log("Transaction signature", tx);
  });
});
```

**4. 运行测试**

```bash
anchor test
```

---

## 第4章：练习2 - 计数器合约

### 目标
创建一个可以增加/减少数值的计数器合约

### 合约代码

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized with value: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented to: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count -= 1;
        msg!("Counter decremented to: {}", counter.count);
        Ok(())
    }
}

// 计数器账户结构
#[account]
pub struct Counter {
    pub count: i64,
}

// 初始化指令的账户
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8  // discriminator + i64
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 更新指令的账户
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
```

### 测试代码

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Counter as Program<Counter>;

  const counter = anchor.web3.Keypair.generate();

  it("Initializes counter", async () => {
    await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counter])
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("Increments counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });

  it("Decrements counter", async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });
});
```

---

## 第5章：练习3 - 学生记录合约

### 目标
创建一个可以存储和管理学生信息的合约

### 合约代码

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod student_record {
    use super::*;

    pub fn create_student(
        ctx: Context<CreateStudent>,
        name: String,
        age: u8,
        student_id: String,
    ) -> Result<()> {
        let student = &mut ctx.accounts.student;
        student.name = name;
        student.age = age;
        student.student_id = student_id;
        student.grades = Vec::new();
        student.authority = ctx.accounts.user.key();
        
        msg!("Student created: {}", student.name);
        Ok(())
    }

    pub fn add_grade(
        ctx: Context<UpdateStudent>,
        grade: u8,
    ) -> Result<()> {
        let student = &mut ctx.accounts.student;
        
        require!(grade <= 100, StudentError::InvalidGrade);
        student.grades.push(grade);
        
        msg!("Grade {} added for student {}", grade, student.name);
        Ok(())
    }

    pub fn get_average(ctx: Context<ViewStudent>) -> Result<()> {
        let student = &ctx.accounts.student;
        
        if student.grades.is_empty() {
            msg!("No grades recorded for {}", student.name);
            return Ok(());
        }

        let sum: u32 = student.grades.iter().map(|&x| x as u32).sum();
        let average = sum / student.grades.len() as u32;
        
        msg!("Average grade for {}: {}", student.name, average);
        Ok(())
    }
}

#[account]
pub struct Student {
    pub name: String,
    pub age: u8,
    pub student_id: String,
    pub grades: Vec<u8>,
    pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(name: String, student_id: String)]
pub struct CreateStudent<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + // discriminator
                4 + name.len() + // String name
                1 + // u8 age
                4 + student_id.len() + // String student_id
                4 + 100 * 1 + // Vec<u8> grades (max 100 grades)
                32 // Pubkey authority
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        has_one = authority @ StudentError::Unauthorized
    )]
    pub student: Account<'info, Student>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewStudent<'info> {
    pub student: Account<'info, Student>,
}

#[error_code]
pub enum StudentError {
    #[msg("Grade must be between 0 and 100")]
    InvalidGrade,
    #[msg("You are not authorized to update this student record")]
    Unauthorized,
}
```

---

## 第6章：练习4 - 简单代币转账

### 目标
创建一个简单的代币系统，用户可以铸造和转账代币

### 合约代码

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod simple_token {
    use super::*;

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        total_supply: u64,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.authority = ctx.accounts.authority.key();
        mint.total_supply = total_supply;
        mint.current_supply = 0;
        
        msg!("Token mint initialized with total supply: {}", total_supply);
        Ok(())
    }

    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        let account = &mut ctx.accounts.token_account;
        account.owner = ctx.accounts.owner.key();
        account.balance = 0;
        
        msg!("Token account created for: {}", account.owner);
        Ok(())
    }

    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let account = &mut ctx.accounts.token_account;
        
        require!(
            mint.current_supply + amount <= mint.total_supply,
            TokenError::ExceedsSupply
        );
        
        mint.current_supply += amount;
        account.balance += amount;
        
        msg!("Minted {} tokens to {}", amount, account.owner);
        Ok(())
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64,
    ) -> Result<()> {
        let from_account = &mut ctx.accounts.from_account;
        let to_account = &mut ctx.accounts.to_account;
        
        require!(
            from_account.balance >= amount,
            TokenError::InsufficientBalance
        );
        
        from_account.balance -= amount;
        to_account.balance += amount;
        
        msg!("Transferred {} tokens", amount);
        Ok(())
    }
}

#[account]
pub struct TokenMint {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub current_supply: u64,
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8
    )]
    pub mint: Account<'info, TokenMint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        mut,
        has_one = authority @ TokenError::Unauthorized
    )]
    pub mint: Account<'info, TokenMint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        mut,
        has_one = owner @ TokenError::Unauthorized
    )]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum TokenError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Amount exceeds total supply")]
    ExceedsSupply,
    #[msg("Unauthorized operation")]
    Unauthorized,
}
```

---

## 第7章：关键概念详解

### 7.1 账户约束（Account Constraints）

```rust
#[derive(Accounts)]
pub struct Example<'info> {
    // 初始化新账户
    #[account(
        init,                    // 创建新账户
        payer = user,           // 谁支付账户创建费用
        space = 8 + 64          // 账户需要的空间
    )]
    pub data: Account<'info, MyData>,
    
    // 可变账户
    #[account(mut)]
    pub user: Signer<'info>,
    
    // 检查账户关系
    #[account(
        mut,
        has_one = authority @ MyError::Unauthorized
    )]
    pub protected_data: Account<'info, MyData>,
    
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### 7.2 错误处理

```rust
#[error_code]
pub enum MyError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Unauthorized access")]
    Unauthorized,
}

// 在函数中使用
pub fn my_function(ctx: Context<MyContext>) -> Result<()> {
    require!(condition, MyError::CustomError);
    Ok(())
}
```

### 7.3 事件（Events）

```rust
#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    // ... 转账逻辑
    
    emit!(TransferEvent {
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        amount,
    });
    
    Ok(())
}
```

---

## 第8章：开发工作流

### 8.1 本地开发环境

```bash
# 启动本地验证器
solana-test-validator

# 在另一个终端
# 构建项目
anchor build

# 部署到本地
anchor deploy

# 运行测试
anchor test --skip-local-validator
```

### 8.2 部署到 Devnet

```bash
# 切换到 devnet
solana config set --url devnet

# 申请测试 SOL
solana airdrop 2

# 部署
anchor deploy --provider.cluster devnet
```

### 8.3 与合约交互

```typescript
// 使用 TypeScript 客户端
import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
const program = anchor.workspace.MyProgram;

// 调用合约方法
const tx = await program.methods
  .myMethod(param1, param2)
  .accounts({
    account1: publicKey1,
    account2: publicKey2,
  })
  .rpc();
```

---

## 🎯 学习路径

### 阶段1：基础练习（1-2周）
1. ✅ 完成 Hello World 合约
2. ✅ 实现计数器合约
3. ✅ 构建学生记录系统
4. ✅ 创建简单代币系统

### 阶段2：进阶项目（2-3周）
- 投票系统
- NFT 铸造合约
- 去中心化交易所（DEX）基础版本
- 借贷协议简化版

### 阶段3：实战项目（3-4周）
- 完整的 DeFi 协议
- DAO 治理合约
- 游戏化应用
- 社交媒体平台

---

## 📚 学习资源

- [Anchor 官方文档](https://www.anchor-lang.com/)
- [Solana 官方文档](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor 示例项目](https://github.com/coral-xyz/anchor/tree/master/examples)

---

## 🚨 注意事项

1. **安全性**：智能合约一旦部署就无法修改，务必彻底测试
2. **账户租金**：Solana 账户需要支付租金来保持活跃
3. **计算单元**：复杂操作可能超出计算限制
4. **跨程序调用**：与其他合约交互需要特殊处理

---

现在开始你的第一个智能合约吧！建议按顺序完成这些练习，每个练习都会教授重要的概念。

有问题随时问我！ 🚀