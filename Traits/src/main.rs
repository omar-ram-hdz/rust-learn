trait Describir {
  fn describir(&self) -> String;
}

struct Libro{
  titulo:String,
}

struct Pelicula{
  nombre: String,
}

impl Libro{
  fn new(titulo: String) -> Libro{
    Libro { titulo }
  }
}

impl Pelicula{
  fn new(nombre:String) -> Pelicula{
    Pelicula { nombre }
  }
}

impl Describir for Libro{
  fn describir(&self) -> String {
      format!("El titulo del libro es: {}", self.titulo)
  }
}

impl Describir for Pelicula{
  fn describir(&self) -> String {
      format!("El nombre de la película es: {}", self.nombre)
  }
}

use std::collections::HashMap;

fn main() {
  let libro = Libro::new("Hombres con estilo - Bere Casillas".to_string());
  let pelicula = Pelicula::new("El hombre que conocía el infinito".to_string());
  let mut descripciones: HashMap<i32, String> = HashMap::new();
  descripciones.insert(1, libro.describir());
  descripciones.insert(2, pelicula.describir());
  println!("Descripción libro: {:?}", descripciones.get(&1));
  println!("Descripción película: {:?}", descripciones.get(&2));
}
