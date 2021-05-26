fn main() {
    
    let mut value = 0b1111_0101u8;

    println!("Value is {}", value);
    println!("the value is {:08b}", value);

    value = !value;

    println!("Value is {}", value);
    println!("the value is {:08b}", value);

    value = value & 0b1111_0111;

    println!("Value is {}", value);
    println!("the value is {:08b}", value);

    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("Value is {}", value);
    println!("the value is {:08b}", value);

    println!("bit 6 is {}", value & 0b0100_0000);

    value = value ^ 0b0101_0101;
    println!("Value is {}", value);
    println!("the value is {:08b}", value);
    
    value = value << 4;
    println!("Value after left shift 4 times {}", value);
    println!("the value is {:08b}", value);

    value = value >> 2;
    println!("Value after right shift 2 times {}", value);
    println!("the value is {:08b}", value);

    value = value >> 2;
    println!("Value after right shift 2 more times {}", value);
    println!("the value is {:08b}", value);
    value = value >> 2;
    println!("Value after right shift 2 more times {}", value);
    println!("the value is {:08b}", value);
}
