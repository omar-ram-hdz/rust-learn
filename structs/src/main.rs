#[derive(Debug)]
struct Rectangulo{
  ancho: u32,
  alto: u32,
}

#[derive(Debug)]
struct Usuario{
  nombre: String,
  edad: u8,
  activo: bool,
}

fn main() {
    // Mini proyecto 1
    let rec = Rectangulo{
      ancho:30, alto:50,
    };
    let area = rec.alto * rec.ancho;
    println!("El área del rectángulo: {:?} es: {}", rec , area);

    // Mini proyecto 2
    let usuario1 = Usuario{
      nombre: "Josué".to_string(),
      edad: 17,
      activo: true,
    };
    println!("Usuario 1: {:?}", usuario1);
    let usuario2 = Usuario{
      edad: 21,
      ..usuario1
    };
    println!("Usuario 1: {:?}", usuario2);
}
