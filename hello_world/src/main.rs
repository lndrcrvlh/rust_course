fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32= 120.0;

    let average = ((((a as f32 + b as f32 + c)/ 3 as f32) as f64)*10.0).round()/10.0;


    assert_eq!(average, 45.1);
    println!("Test passed!");
}
