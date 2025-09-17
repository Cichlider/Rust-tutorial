struct HelloWorld;

impl HelloWorld {
    fn new() -> Self {
        HelloWorld
    }
    
    fn say_hello(&self) {
        println!("Hello world!");
    }
}

fn main() {
    let hello = HelloWorld::new();
    hello.say_hello();
}
