fn main(){
    let message = ['h','e','l','l','o',];

    for (index,item) in message.iter().enumerate() {
        print!("{}", item);
        println!("{}",index);
    }

    for number in 0..5{
        println!("number is {}", number);
    }
}