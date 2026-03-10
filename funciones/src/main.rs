fn main() {
    // Mini proyecto 1
    let grados = 30.0; // Celcius
    let grados = celsius_a_fahrenheit(grados);
    println!("Resultado: {}", grados);
    // Mini proyecto 2
    let a = 10;
    let b = 5;
    let maximo_numero = maximo(a, b);
    println!("El número más grande entre {} y {} es: {}", a, b, maximo_numero);
}

fn celsius_a_fahrenheit(celcius: f64) -> f64 {
  celcius * (9/5) as f64 + 32 as f64
}

fn maximo(a: i32, b: i32)  -> i32 {
  if a > b {a}
  else {b}
}