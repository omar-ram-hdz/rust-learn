enum Semaforo{
  Verde,
  Amarillo,
  Rojo,
}

enum Operacion{
  Suma(i32, i32),
  Resta(i32, i32),
  Multiplicacion(i32, i32),
}

fn main() {
  // Mini proyecto 1
  accion_semaforo(Semaforo::Amarillo);
  accion_semaforo(Semaforo::Rojo);
  accion_semaforo(Semaforo::Verde);

  // Mini proyecto 2
  let mut a = 32;
  let mut b = 5;
  imprimir_resultado_operacion(a, b, Operacion::Suma(a, b));
  a = 10;
  b = 3;
  imprimir_resultado_operacion(a, b, Operacion::Resta(a, b));
  a=11;
  b=3;
  imprimir_resultado_operacion(a, b, Operacion::Multiplicacion(a, b));
}

fn imprimir_resultado_operacion(a: i32, b: i32, operacion: Operacion) {
  let signo = match operacion {
      Operacion::Suma(_a, _b) => "+",
      Operacion::Resta(_a, _b) => "-",
      Operacion::Multiplicacion(_a, _b) => "*"
  };
  let res = realizar_operacion(operacion);
  println!("El resultado de {} {} {} es: {}", a, signo, b, res);
}

fn realizar_operacion(operacion: Operacion) -> i32{
  match operacion {
      Operacion::Suma(a, b) => a + b,
      Operacion::Resta(a, b) => a - b,
      Operacion::Multiplicacion(a, b) => a * b,
  }
}

fn accion_semaforo(estado: Semaforo){
  match estado {
      Semaforo::Rojo => println!("Detente"),
      Semaforo::Amarillo => println!("Precaución"),
      Semaforo::Verde => println!("Avanza"),
  }
}