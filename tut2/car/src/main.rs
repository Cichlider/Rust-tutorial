struct Car {
    manufacturer: String,
    model: String,
    speed: f64,
}

impl Car {
    fn new(manufacturer: String, model: String) -> Self {
        Car {
            manufacturer,
            model,
            speed: 0.0,
        }
    }
    
    fn accelerate(&mut self, amount: f64) {
        if amount > 0.0 {
            self.speed += amount;
            println!("加速 {:.1} km/h", amount);
        }
    }
    
    fn decelerate(&mut self, amount: f64) {
        if amount > 0.0 {
            self.speed = (self.speed - amount).max(0.0);
            println!("减速 {:.1} km/h", amount);
        }
    }
    
    fn get_current_speed(&self) -> f64 {
        self.speed
    }
    
    fn display_info(&self) {
        println!("{} {} - 当前速度: {:.1} km/h", 
                 self.manufacturer, self.model, self.speed);
    }
}

fn main() {
    let mut car = Car::new("Toyota".to_string(), "Camry".to_string());
    
    println!("=== 汽车测试 ===");
    car.display_info();
    
    car.accelerate(30.0);
    car.display_info();
    
    car.accelerate(20.0);
    car.display_info();
    
    car.decelerate(15.0);
    car.display_info();
    
    // 测试不会低于0
    car.decelerate(100.0);
    car.display_info();
}