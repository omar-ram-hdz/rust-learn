const PI: f64 = 3.1416;

#[derive(Debug)]
struct Circulo {
  radio: f64,
}

impl Circulo{
  fn area(&self) -> f64{
    PI * self.radio * self.radio
  }
}

#[derive(Debug)]
struct Contador{
  valor:i32,
}

impl Contador{
  fn new(valor: i32) -> Contador{
    Contador { valor: valor }
  }

  fn incrementar(&mut self) {
    self.valor += 1;
  }

  fn mostrar(&self){
    println!("El valor es: {}", self.valor);
  }
}

fn main() {
    // Mini proyecto 1
    let circle = Circulo{
      radio: 4.0,
    };
    let area = circle.area();
    println!("El área del círculo: {:?} es: {}", circle, area);

    // Mini proyecto 2
    let mut counter = Contador::new(3);
    counter.mostrar();
    counter.incrementar();
    counter.mostrar();
    counter.incrementar();
    counter.incrementar();
    counter.incrementar();
    counter.incrementar();
    counter.mostrar();
}
