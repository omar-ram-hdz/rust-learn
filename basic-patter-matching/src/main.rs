fn main() {
    // Mini proyecto 1
    let score = 87;
    let message = match score {
      90..=100 => "Excelente",
      80..=89 => "Muy bien",
      70..=79 => "Bien",
      60..=69 => "Suficiente",
      0..=59 => "Reprobado",
      _ => "Error"
    };
    println!("{}", message);

    // Mini proyecto 2
    let opcion = 2;
    match  opcion {
        1 => imprimir_opcion(1, "Ver perfil"),
        2 => imprimir_opcion(2, "Editar perfil"),
        3 => imprimir_opcion(3, "Configuración"),
        4 => imprimir_opcion(4, "Cerrar sesión"),
        _ => imprimir_opcion(5, "Opción no valida"),
    }
}

fn imprimir_opcion(index: u32, opcion: &str){
  println!("{}. {}", index, opcion);
}