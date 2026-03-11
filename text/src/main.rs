fn main() {
    /*
      let neww = format!("{} {}", "Hola", "mundo");
     */

    // Mini proyecto 1
    let mut saludo = String::from("Hola");
    saludo.push_str(" mundo");
    saludo.push('!');
    println!("{saludo}");

    // Mini proyecto 2
    let texto = String::from("Rust es genial");
    for ch in texto.chars(){
      println!("{}", ch);
    }
}
