fn main(){
    let mut count = 0;

    let result = loop {
        if count == 10{
            break count * 10;
        }
        count += 1;
        println!("{}",count);

    };
    println!("looped, result is {}", result);
}