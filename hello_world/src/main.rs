fn main() {
    let celsius_temp = 23.0;
    let farenheit_temp = celsius_to_farenheit(celsius_temp);

    assert_eq!(farenheit_temp,73.4);
    println!("Test passed!")
}


fn celsius_to_farenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}