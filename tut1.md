## 编译和运行(非常重要)

对于每个练习：
```bash
# 创建新项目
cargo new exercise_name
cd exercise_name

# 例如 cargo new hello_world
# 然后 cd hello_world

# 运行程序

cargo run

# 检查代码
cargo check
cargo clippy
```

## Mac 终端常用指令小抄
| 指令 | 功能 |
|-------|------|
| `pwd` | 显示当前目录路径 |
| `ls` | 列出当前目录文件和文件夹 |
| `ls -l` | 列出详细信息 |
| `ls -a` | 显示隐藏文件 |
| `cd <目录>` | 切换到指定目录 |
| `cd ..` | 返回上级目录(我觉得这很实用) |
| `cd ../..` | 返回上两级目录 |
| `cd ~` | 返回用户主目录 |
| `cd -` | 返回上次所在目录 |



# Rust 基础知识讲解

> 从零开始学习 Rust，为后续的结构体练习打好基础

## 第1章：变量和数据类型

### 1.1 Hello World

先来一个最简单的程序：

```rust
fn main() {
    println!("Hello, world!");
}
```

**解释**：
- `fn main()` - 程序的入口函数
- `println!` - 打印宏（注意感叹号！）
- 用 `cargo run` 运行程序

### 1.2 变量声明

```rust
fn main() {
    // 不可变变量（默认）
    let x = 5;
    println!("x 的值是: {}", x);
    
    // x = 6; // 错误！不能修改不可变变量
    
    // 可变变量
    let mut y = 5;
    println!("y 的值是: {}", y);
    y = 6; // 正确！可以修改
    println!("y 的新值是: {}", y);
}
```

**重点**：
- `let` - 声明变量
- `mut` - 使变量可变
- Rust 默认变量是不可变的（安全性）

### 1.3 数据类型

#### 整数类型
```rust
fn main() {
    let a: i32 = 42;        // 32位有符号整数
    let b: u32 = 42;        // 32位无符号整数
    let c = 42;             // 编译器自动推断为 i32
    
    println!("a={}, b={}, c={}", a, b, c);
}
```

#### 浮点数类型
```rust
fn main() {
    let x: f64 = 3.14;      // 64位浮点数（默认）
    let y: f32 = 2.0;       // 32位浮点数
    
    println!("x={}, y={}", x, y);
}
```

#### 布尔类型
```rust
fn main() {
    let is_true: bool = true;
    let is_false = false;   // 自动推断为 bool
    
    println!("真: {}, 假: {}", is_true, is_false);
}
```

#### 字符和字符串
```rust
fn main() {
    // 字符（单个 Unicode 字符）
    let c: char = 'A';
    let emoji: char = '😀';
    
    // 字符串切片（&str）
    let s1: &str = "Hello";
    
    // 字符串（String）
    let s2: String = String::from("World");
    let s3: String = "Hello".to_string();
    
    println!("字符: {}, emoji: {}", c, emoji);
    println!("字符串: {} {}", s1, s2);
}
```

**String vs &str 的区别**：
- `&str` - 字符串切片，不可变，存储在栈上
- `String` - 可变字符串，存储在堆上

---

## 第2章：函数

### 2.1 基本函数

```rust
// 无参数，无返回值
fn say_hello() {
    println!("Hello!");
}

// 有参数，无返回值
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数，有返回值
fn add(a: i32, b: i32) -> i32 {
    a + b  // 注意：没有分号，这是返回值
}

// 或者使用 return 关键字
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    say_hello();
    greet("张三");
    
    let sum = add(5, 3);
    let diff = subtract(10, 4);
    
    println!("和: {}, 差: {}", sum, diff);
}
```

**重点**：
- `->` 表示返回类型
- 最后一行没有分号的表达式是返回值
- 参数必须声明类型

### 2.2 表达式 vs 语句

```rust
fn main() {
    // 语句（statement）- 不返回值
    let x = 5;
    
    // 表达式（expression）- 返回值
    let y = {
        let a = 3;
        a + 1  // 没有分号，这是表达式，返回 4
    };
    
    println!("x={}, y={}", x, y);
}
```

---

## 第3章：控制流

### 3.1 条件语句

```rust
fn main() {
    let number = 6;
    
    // if-else
    if number < 5 {
        println!("数字小于5");
    } else if number == 5 {
        println!("数字等于5");
    } else {
        println!("数字大于5");
    }
    
    // if 作为表达式
    let result = if number > 5 { "大" } else { "小" };
    println!("结果: {}", result);
}
```

### 3.2 循环

#### loop 循环
```rust
fn main() {
    let mut count = 0;
    
    loop {
        count += 1;
        println!("计数: {}", count);
        
        if count == 3 {
            break;  // 跳出循环
        }
    }
}
```

#### while 循环
```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("发射！");
}
```

#### for 循环
```rust
fn main() {
    // 遍历范围
    for i in 1..4 {  // 1, 2, 3（不包含4）
        println!("数字: {}", i);
    }
    
    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("值: {}", element);
    }
    
    // 遍历数组的索引和值
    for (index, value) in arr.iter().enumerate() {
        println!("索引: {}, 值: {}", index, value);
    }
}
```

---

## 第4章：集合类型

### 4.1 数组

```rust
fn main() {
    // 固定长度数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5];  // [3, 3, 3, 3, 3]
    
    // 访问元素
    println!("第一个元素: {}", arr[0]);
    println!("数组长度: {}", arr.len());
    
    // 遍历数组
    for item in &arr {
        println!("项: {}", item);
    }
}
```

### 4.2 向量（Vector）

```rust
fn main() {
    // 创建空向量
    let mut vec: Vec<i32> = Vec::new();
    
    // 或者使用宏
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 添加元素
    numbers.push(6);
    numbers.push(7);
    
    // 访问元素
    println!("第一个: {}", numbers[0]);
    
    // 安全访问（返回 Option）
    match numbers.get(100) {
        Some(value) => println!("值: {}", value),
        None => println!("索引越界"),
    }
    
    // 遍历
    for num in &numbers {
        println!("数字: {}", num);
    }
    
    println!("向量长度: {}", numbers.len());
}
```

---

## 第5章：所有权系统（Rust 的核心）

### 5.1 所有权规则

**三条规则**：
1. Rust 中的每一个值都有一个所有者
2. 值在任一时刻有且只有一个所有者
3. 当所有者离开作用域，这个值将被丢弃

```rust
fn main() {
    {
        let s = String::from("hello");  // s 拥有字符串
    }  // s 离开作用域，字符串被释放
    
    // println!("{}", s);  // 错误！s 已经不存在了
}
```

### 5.2 移动（Move）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权移动到 s2
    
    // println!("{}", s1);  // 错误！s1 不再有效
    println!("{}", s2);     // 正确！s2 拥有字符串
}
```

### 5.3 克隆（Clone）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("s1 = {}, s2 = {}", s1, s2);  // 都可以使用
}
```

### 5.4 借用（Borrowing）

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);  // 借用 s1
    
    println!("'{}' 的长度是 {}", s1, len);  // s1 仍然可用
}

fn calculate_length(s: &String) -> usize {
    s.len()  // s 是借用，不拥有字符串
}  // s 离开作用域，但不会释放字符串
```

### 5.5 可变借用

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);  // 可变借用
    
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**借用规则**：
- 同一时间只能有一个可变借用
- 或者可以有多个不可变借用
- 但不能同时有可变和不可变借用

---

## 第6章：结构体基础

### 6.1 定义结构体

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    // 创建结构体实例
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("张三"),
        age: 25,
        active: true,
    };
    
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
}
```

### 6.2 可变结构体

```rust
struct User {
    username: String,
    age: u32,
}

fn main() {
    let mut user = User {
        username: String::from("张三"),
        age: 25,
    };
    
    user.age = 26;  // 修改字段
    println!("新年龄: {}", user.age);
}
```

### 6.3 结构体方法

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 关联函数（类似构造函数）
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 方法（需要 &self）
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // 可变方法
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect = Rectangle::new(3.0, 4.0);
    
    println!("面积: {}", rect.area());
    
    rect.scale(2.0);
    println!("缩放后面积: {}", rect.area());
}
```

---

## 第7章：错误处理基础

### 7.1 Option 类型

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    
    match result {
        Some(value) => println!("结果: {}", value),
        None => println!("不能除以0"),
    }
    
    // 或者使用 if let
    if let Some(value) = divide(8.0, 4.0) {
        println!("结果: {}", value);
    }
}
```

### 7.2 Result 类型

```rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn main() {
    let number = parse_number("42");
    
    match number {
        Ok(n) => println!("数字: {}", n),
        Err(e) => println!("解析错误: {}", e),
    }
}
```

---

## 🎯 知识检查点

在继续学习结构体练习之前，确保你理解了：

- ✅ **变量声明**：`let` 和 `let mut` 的区别
- ✅ **数据类型**：整数、浮点数、布尔、字符串
- ✅ **函数**：参数、返回值、表达式 vs 语句
- ✅ **控制流**：if、loop、while、for
- ✅ **集合**：数组、向量的基本操作
- ✅ **所有权**：移动、借用、可变借用的概念
- ✅ **结构体**：定义、实例化、方法

## 📝 简单练习

在进入复杂练习前，先试试这些：

```rust
// 练习1：计算器
fn main() {
    let a = 10.0;
    let b = 3.0;
    
    println!("加法: {}", a + b);
    println!("减法: {}", a - b);
    println!("乘法: {}", a * b);
    println!("除法: {}", a / b);
}

// 练习2：判断奇偶
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let number = 42;
    if is_even(number) {
        println!("{} 是偶数", number);
    } else {
        println!("{} 是奇数", number);
    }
}

// 练习3：字符串操作
fn main() {
    let mut name = String::from("Rust");
    name.push_str(" 编程语言");
    println!("{}", name);
}
```

掌握了这些基础知识后，你就可以开始做tut2的结构体练习了！

**下一步**：开始第一个练习 - Hello World 结构体版本。