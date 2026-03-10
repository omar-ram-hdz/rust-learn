use std::io::stdin;

fn main() {
  // Mini proyecto 1
  for n in 1..=5{
    if n % 2 == 0{
      println!("{} es par", n);
    }else{
      println!("{} es impar", n);
    }
  }
  // Mini proyecto 2
  loop {
    println!("\n\n******** Menú de opciones ********");
    println!("1. Saludar");
    println!("2. Mostrar numero favorito");
    println!("3. Salir");
    let mut opcion = String::new();
    stdin()
        .read_line(&mut opcion)
        .expect("Failed to read line");
    if opcion == "1"{
      println!("Hola humano");
    } else if opcion == "2"{
      println!("21");
    } else if opcion == "3"{
      break;
    }else {
      println!("Opción no reconocida, vuelve a intentarlo");
    }
  }
}
