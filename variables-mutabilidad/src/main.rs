fn main() {
    // Mini proyecto 1
    const FREEZING_POINT: u16 = 32;
    const C: u16 = 25;
    const F: u16 = C * 9/5 + 32;
    println!("Freezing point: {}", FREEZING_POINT);
    println!("{}° son {}°F",C,F);
    // Mini proyecto 2
    let texto = "42";
    println!("Texto original: {}",texto);
    let mut numero: u16 = texto.parse().unwrap();
    numero += 8;
    println!("Número final: {}", numero);
}
