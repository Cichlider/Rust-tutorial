struct Greeter {
    name: String,
}

impl Greeter {
    fn new(name: String) -> Self {
        Greeter { name }
    }
    
    fn say_hello(&self) {
        println!("Hello {}!", self.name);
    }
}

fn main() {
    let greeter = Greeter::new("John".to_string());
    greeter.say_hello();
}