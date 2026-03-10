fn main() {
    // Mini proyecto 1
    let string = String::from("Hola mundo");
    prestamo_lectura(&string);

    // Mini proyecto 2
    let mut string = String::from("Hola");
    println!("Original: \"{}\"", string);
    prestamo_mutable(&mut string);
    println!("Después del préstamo: \"{}\"", string);
}

fn prestamo_lectura(string: &String){
  println!("{}", string);
}

fn prestamo_mutable(string: &mut String){
  string.push_str(" mundo");
}