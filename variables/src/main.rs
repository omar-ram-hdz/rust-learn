fn main() {
    let mut x: u8 = 5;
    println!("Valor original: {}", x);

    x = 10;
    println!("Valor cambiado:{}", x);

    let x: u8 = x + 1;
    println!("Valor mas 1:{}", x);

    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("Spaces len: {}", spaces)
}
