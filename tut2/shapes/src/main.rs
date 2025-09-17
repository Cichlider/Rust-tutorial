use std::f64::consts::PI;

// 矩形结构体
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
    
    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// 圆形结构体
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
    
    fn calculate_area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn calculate_perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// 直角三角形结构体
struct RightTriangle {
    base: f64,
    height: f64,
}

impl RightTriangle {
    fn new(base: f64, height: f64) -> Self {
        RightTriangle { base, height }
    }
    
    fn calculate_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    
    fn calculate_perimeter(&self) -> f64 {
        // 计算斜边长度：sqrt(base² + height²)
        let hypotenuse = (self.base.powi(2) + self.height.powi(2)).sqrt();
        self.base + self.height + hypotenuse
    }
}

fn main() {
    let rectangle = Rectangle::new(5.0, 3.0);
    let circle = Circle::new(2.0);
    let triangle = RightTriangle::new(3.0, 4.0);
    
    println!("=== 矩形 ===");
    println!("面积: {:.2}", rectangle.calculate_area());
    println!("周长: {:.2}", rectangle.calculate_perimeter());
    
    println!("=== 圆形 ===");
    println!("面积: {:.2}", circle.calculate_area());
    println!("周长: {:.2}", circle.calculate_perimeter());
    
    println!("=== 直角三角形 ===");
    println!("面积: {:.2}", triangle.calculate_area());
    println!("周长: {:.2}", triangle.calculate_perimeter());
}