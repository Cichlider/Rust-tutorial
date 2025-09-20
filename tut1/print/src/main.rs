use std::io;

fn main() {
    println!("Gauss a number!");

    let mut guess = String::new();

    // let mut foo =1;
    // let bar = foo;

    // foo = 2;

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是: {}", guess);

}
