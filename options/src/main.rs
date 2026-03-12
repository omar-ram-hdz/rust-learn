/*
enum Option<T> {
    Some(T),
    None,
}

Crear un Option
let n = Some(4);
let n2: Option<i32> = None;

match a {
    Some(v) => println!("Valor: {}", v),
    None => println!("No hay valor"),
}

if let Some(v) = a {
    println!("{}", v);
}


.unwrap() -> Confío en que hay valor
.expect("Feedback") -> Esperaba x
let v = b.unwrap_or(10) -> Valor default


let a = Some(5);
let b = a.map(|x| x * 2);
*/

fn main() {
    // Mini proyecto 1
    let vector = vec![10,20,30];
    let objetivo = 20;
    if let Some(id) = buscar_numero(&vector, objetivo) {
      println!("El índice de {} en el vector {:?} es: {}", objetivo, vector, id);
    }

    // Mini proyecto 2
    if let Some(resultado) = dividir(10.0, 2.0) {
      println!("El resultado es: {}", resultado);
    }
}

fn dividir(a: f64, b: f64) -> Option<f64>{
  // (b != 0.0).then(|| a/b)
  if b==0.0 {
    None
  }else {
    Some(a/b)
  }
}

// fn buscar_numero(vector: &[i32], objetivo: i32) -> Option<usize>
fn buscar_numero(vector: &Vec<i32>, objetivo: i32) -> Option<usize>{
  for (i, el) in vector.iter().enumerate(){
    if *el == objetivo {
      return Some(i);
    }
  }
  None
}
