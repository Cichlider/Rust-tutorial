## 编译和运行(非常重要)

对于每个练习：
```bash
# 创建新项目
cargo new exercise_name
cd exercise_name

# 例如 cargo hello_world
# 然后 cd hello_world

# 编辑 src/main.rs
# 运行程序

cargo run

# 检查代码
cargo check
cargo clippy
```

## 练习题

### 练习 1 - Hello World
**文件名**: `hello_world.rs`

编写一个名为 `HelloWorld` 的结构体，实现一个方法来打印 "Hello world!" 到屏幕上。

```rust
struct HelloWorld;

impl HelloWorld {
    // 在这里实现你的方法
}

fn main() {
    // 创建 HelloWorld 实例并调用方法
}
```

**测试**: 运行 `cargo run` 确保程序正确输出。

**提示**: 
- 使用 `impl` 块为结构体实现方法
- 可以创建一个无需参数的方法

### 练习 2 - Hello Someone
**文件名**: `hello_someone.rs`

创建一个 `Greeter` 结构体，能够向指定的人问好。

```rust
struct Greeter {
    // 添加必要的字段
}

impl Greeter {
    // 实现构造函数和问好方法
}

fn main() {
    // 创建 Greeter 实例并问好
    let greeter = Greeter::new("John".to_string());
    greeter.say_hello();
}
```

**预期输出**: `Hello John!`

### 练习 3 - 矩形面积和周长计算
**文件名**: `rectangle.rs`

逐步开发一个 `Rectangle` 结构体：

**步骤 1**: 创建基础结构体
```rust
struct Rectangle {
    width: f64,
    height: f64,
}
```

**步骤 2**: 添加构造函数
```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        // 实现构造函数
    }
}
```

**步骤 3**: 添加默认构造函数
```rust
impl Default for Rectangle {
    fn default() -> Self {
        // 实现默认值（width: 0.0, height: 0.0）
    }
}
```

**步骤 4**: 添加计算方法
```rust
impl Rectangle {
    fn calculate_area(&self) -> f64 {
        // 计算面积
    }
    
    fn calculate_perimeter(&self) -> f64 {
        // 计算周长
    }
}
```

**步骤 5**: 在 main 函数中测试
```rust
fn main() {
    let rect1 = Rectangle::new(2.5, 7.5);
    let rect2 = Rectangle::default();
    
    println!("矩形1: 宽={}, 高={}", rect1.width, rect1.height);
    println!("矩形1面积: {}", rect1.calculate_area());
    println!("矩形1周长: {}", rect1.calculate_perimeter());
    
    // 对 rect2 做同样的操作
}
```

### 练习 4 - 多种几何图形
**文件名**: `shapes.rs`

创建三个结构体来表示不同的几何图形：

**Rectangle 结构体**:
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        // 实现
    }
    
    fn calculate_area(&self) -> f64 {
        // 实现
    }
    
    fn calculate_perimeter(&self) -> f64 {
        // 实现
    }
}
```

**Circle 结构体**:
```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        // 实现
    }
    
    fn calculate_area(&self) -> f64 {
        // 使用 std::f64::consts::PI
    }
    
    fn calculate_perimeter(&self) -> f64 {
        // 实现
    }
}
```

**RightTriangle 结构体**:
```rust
struct RightTriangle {
    base: f64,
    height: f64,
}

impl RightTriangle {
    fn new(base: f64, height: f64) -> Self {
        // 实现
    }
    
    fn calculate_area(&self) -> f64 {
        // 实现
    }
    
    fn calculate_perimeter(&self) -> f64 {
        // 需要计算斜边长度：sqrt(base² + height²)
        // 使用 (self.base.powi(2) + self.height.powi(2)).sqrt()
    }
}
```

**主函数**:
```rust
fn main() {
    let rectangle = Rectangle::new(5.0, 3.0);
    let circle = Circle::new(2.0);
    let triangle = RightTriangle::new(3.0, 4.0);
    
    // 打印所有图形的面积和周长
}
```

### 练习 5 - 状态与行为 I
**文件名**: `car.rs`

创建一个 `Car` 结构体来模拟汽车的行为：

```rust
struct Car {
    manufacturer: String,
    model: String,
    speed: f64,
}

impl Car {
    fn new(manufacturer: String, model: String) -> Self {
        // 初始速度为 0.0
    }
    
    fn accelerate(&mut self, amount: f64) {
        // 增加速度
    }
    
    fn decelerate(&mut self, amount: f64) {
        // 减少速度，确保不会低于 0
    }
    
    fn get_current_speed(&self) -> f64 {
        // 返回当前速度
    }
}

fn main() {
    let mut car = Car::new("Toyota".to_string(), "Camry".to_string());
    
    println!("初始速度: {}", car.get_current_speed());
    
    car.accelerate(30.0);
    println!("加速后: {}", car.get_current_speed());
    
    car.accelerate(20.0);
    println!("再次加速后: {}", car.get_current_speed());
    
    car.decelerate(15.0);
    println!("减速后: {}", car.get_current_speed());
}
```

### 练习 6 - 状态与行为 II
**文件名**: `student.rs`

创建一个 `Student` 结构体来管理学生信息：

```rust
struct Student {
    name: String,
    id: u32,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: String, id: u32) -> Self {
        // 初始化空的成绩列表
    }
    
    fn add_grade(&mut self, grade: f64) {
        // 添加成绩到列表
        // 确保成绩为非负数
        if grade >= 0.0 {
            self.grades.push(grade);
        }
    }
    
    fn compute_average_grade(&self) -> f64 {
        // 计算平均分
        // 假设至少有一个成绩
        if self.grades.is_empty() {
            0.0
        } else {
            let sum: f64 = self.grades.iter().sum();
            sum / self.grades.len() as f64
        }
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_id(&self) -> u32 {
        self.id
    }
}

fn main() {
    let mut student = Student::new("张三".to_string(), 12345);
    
    student.add_grade(85.5);
    student.add_grade(92.0);
    student.add_grade(78.5);
    
    println!("学生姓名: {}", student.get_name());
    println!("学生ID: {}", student.get_id());
    println!("平均分: {:.2}", student.compute_average_grade());
}
```

## 扩展挑战

### 挑战 1 - 银行账户系统
创建一个 `BankAccount` 结构体，包含账户号、余额和账户持有人姓名。实现存款、取款和查询余额的方法。

### 挑战 2 - 图书管理系统
创建 `Book` 和 `Library` 结构体，实现图书的添加、借出、归还功能。

## 学习要点

1. **结构体定义**: 使用 `struct` 关键字组织相关数据
2. **方法实现**: 使用 `impl` 块为结构体添加方法
3. **所有权**: 理解 `&self`、`&mut self` 和 `self` 的区别
4. **构造函数**: 使用关联函数创建实例
5. **封装**: 通过公私有字段控制数据访问


完成这些练习后，你将掌握 Rust 中结构体和方法的基本用法，为后续学习特征（trait）和更高级的概念打下坚实基础。