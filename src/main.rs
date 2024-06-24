fn main() {
    let name = String::from("Artem");
    println!("{}", name);

    let age: i32 = 19;

    if age >= 18 {
        println!("Ты готов заходи")
    }

    let num = 24;

    match num {
        10 => println!("Num is 10"),
        23 => {
            println!("num is 23!");
            println!("Num is matched!");
        },
        10..=50 => {
            println!("diapason 50")
        }
        __ => {
            println!("defaul value")
        }
    }
}
