struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 构造函数
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    // 计算面积
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
    
    // 计算周长
    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// 实现默认值
impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            width: 0.0,
            height: 0.0,
        }
    }
}

fn main() {
    // 创建指定尺寸的矩形
    let rect1 = Rectangle::new(2.5, 7.5);
    println!("矩形1: 宽={}, 高={}", rect1.width, rect1.height);
    println!("矩形1面积: {}", rect1.calculate_area());
    println!("矩形1周长: {}", rect1.calculate_perimeter());
    
    // 创建默认矩形
    let rect2 = Rectangle::default();
    println!("矩形2: 宽={}, 高={}", rect2.width, rect2.height);
    println!("矩形2面积: {}", rect2.calculate_area());
    println!("矩形2周长: {}", rect2.calculate_perimeter());
}