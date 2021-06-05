use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let n3: u8 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    if n1 > n3 {
        println!("n1 is bigger");
    } else {
        println!("n2 is bigger");
    }
}

// fn main() {
//     let x = 33;

//     if x == 3{
//         println!("x is 3");
//     }
// }
