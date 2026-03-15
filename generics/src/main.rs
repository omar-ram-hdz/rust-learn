struct Caja<T>{val:T}

impl<T> Caja<T>{
  fn new(val: T) -> Caja<T>{
    Caja{val}
  }
  fn get(&self) -> &T{
    &self.val
  }
}

fn main() {
  // Mini proyecto 1
  let mut vector: Vec<i32> = vec![];
  vector.push(32);
  vector.push(10);
  primero(&vector);
  let mut vector: Vec<f64> = vec![];
  vector.push(12.0);
  vector.push(3.0);
  primero(&vector);
  let mut vector: Vec<String> = vec![];
  primero(&vector);

  // Mini proyecto 2
  let caja: Caja<i32> = Caja::new(32);
  caja.get();
  let caja = Caja::new("Hola mundo".to_string());
  caja.get();
  let caja: Caja<Vec<i32>> = Caja::new(vec![10,12,14]);
  caja.get(); // Use debug mode to print
}

fn primero<T>(vec: &Vec<T>) -> Option<&T>{
  vec.get(0)
}
/*
fn primero<T>(vec: &Vec<T>) -> Option<&T> {
    vec.get(0)
}
*/