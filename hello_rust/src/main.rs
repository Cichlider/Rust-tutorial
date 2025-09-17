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

// Wrote new keypair to /Users/cichlidfish/.config/solana/id.json
// ========================================================================
// pubkey: R2EKaQBQnsYja69BafkfAyjnxswzHfp4H85W4w9d1mV
// ========================================================================
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// galaxy document trend hold salon phone melt pluck panda wire bubble fire
// ========================================================================
// cichlidfish@Mac-2 hello_rust % 

// Config File: /Users/cichlidfish/.config/solana/cli/config.yml
// RPC URL: https://api.devnet.solana.com 
// WebSocket URL: wss://api.devnet.solana.com/ (computed)
// Keypair Path: /Users/cichlidfish/.config/solana/id.json 
// Commitment: confirmed 

// solana address
// R2EKaQBQnsYja69BafkfAyjnxswzHfp4H85W4w9d1mV

// solana balance
// 1 SOL