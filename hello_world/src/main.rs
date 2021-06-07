fn main(){
    let mut count = 0;
    let letters = ['a','b','c'];

    while count < letters.len() {
        println!("{}",letters[count]);
        count += 1;
    }
}