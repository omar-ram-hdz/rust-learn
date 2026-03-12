enum Sensor {
  Temperatura(f32),
  Presion(f32),
  Apagado
}

struct Punto{
  x: i32,
  y: i32,
}

fn main() {
  // Mini proyecto 1
  leer_sensor(Sensor::Apagado);
  leer_sensor(Sensor::Temperatura(25.0));
  leer_sensor(Sensor::Presion(50.0));
  leer_sensor(Sensor::Apagado);


  // Mini proyecto 2
  analizar_punto(Punto { x: 0, y: 0 });
  analizar_punto(Punto { x: 10, y: 0 });
}

/*
match numero {
    n @ 1..=5 => println!("Número pequeño: {}", n),
    _ => println!("Número grande"),
}
match punto {
    (0, y) => println!("sobre eje Y"),
    (x, 0) => println!("sobre eje X"),
    _ => println!("otro punto"),
}
match letra {
    'a'..='z' => println!("minúscula"),
    'A'..='Z' => println!("mayúscula"),
    _ => println!("otro"),
}
match score {
    90..=100 => "Excelente",
    80..=89 => "Muy bien",
    _ => "Otro",
}
match numero {
    1 | 2 => println!("Uno o dos"),
    3 => println!("Tres"),
    _ => println!("Otro"),
}
*/

fn leer_sensor(sensor: Sensor){
  match sensor {
    Sensor::Temperatura(t) => println!("Temperatura: {}°C", t),
    Sensor::Presion(p) => println!("Presión: {} kPa", p),
    Sensor::Apagado => println!("Sensor apagado"),
  }
}

fn analizar_punto(punto: Punto){
  match (punto.x, punto.y) {
    (x,y) if x== 0 && y == 0 => println!("Origen"),
    (x, y) if x == 0 => println!("Eje X"),
    (x, y) if y == 0 => println!("Eje Y"),
    (x, y) if x > 0 && y > 0 => println!("Cuadrante 1"),
    (x, y) if x < 0 && y > 0 => println!("Cuadrante 2"),
    (x, y) if x < 0 && y < 0 => println!("Cuadrante 3"),
    (x, y) if x > 0 && y < 0 => println!("Cuadrante 4"),
    (x, y) => println!("{}, {}", x, y),
  }
  /*
  match punto {
    Punto { x:0, y:0 } => println!("Origen"),
    Punto { x:_, y:0 } => println!("Eje X"),
    Punto { x:0, y:_ } => println!("Eje Y"),
    Punto { x, y } if x > 0 && y > 0 => println!("Cuadrante 1"),
    Punto { x, y } if x < 0 && y > 0 => println!("Cuadrante 2"),
    Punto { x, y } if x < 0 && y < 0 => println!("Cuadrante 3"),
    Punto { x, y } if x > 0 && y < 0 => println!("Cuadrante 4"),
  }
   */
}