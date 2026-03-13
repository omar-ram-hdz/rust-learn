trait Animal {
  fn sonido(&self) -> String;
}

struct Perro{}
struct Gato{}

impl Animal for Perro{
  fn sonido(&self) -> String {
      "Guau".into()
  }
}

impl Animal for Gato{
  fn sonido(&self) -> String {
      "Meow".into()
  }
}

trait Operacion {
  fn ejecutar(&self) -> i32;
}

struct Suma(i32, i32);
struct Multiplicacion(i32,i32);

impl Operacion for Suma {
  fn ejecutar(&self) -> i32 {
      &self.0 + &self.1
  }
}

impl Operacion for Multiplicacion { 
  fn ejecutar(&self) -> i32 {
      &self.0 * &self.1
  }
}

enum Operaciones{
  Suma(Suma),
  Multiplicacion(Multiplicacion),
}

/*
fn ejemplo_uso_dyn(parametro: &dyn Trait){}
*/

fn main() {
  // Mini proyecto 1
  let mut animales: Vec<Box<dyn Animal>> = vec![];
  animales.push(Box::new(Perro{}));
  animales.push(Box::new(Gato{}));
  for animal in animales{
    println!("Sonido: {}", animal.sonido());
  }

  // Mini proyecto 2
  let mut resultados: Vec<Box<dyn Operacion>> = vec![];
  resultados.push(Box::new(Suma(3,4)));
  resultados.push(Box::new(Multiplicacion(3,11)));
  for res in &resultados{
    // match res {
    //   Operaciones::Suma(Suma(a, b)) => println!("Resultado de {} + {} es: {}", res.0, res.1, res.ejecutar()),
    //   Operaciones::Multiplicacion(Suma(a, b)) => println!("Resultado de {} * {} es: {}", res.0, res.1, res.ejecutar()),
    // }
    
    println!("Resultado: {}", res.ejecutar());
  }
}
