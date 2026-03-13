fn main() {
  // Mini proyecto 1
  println!("{:?}", sumar_textos("5", "3"));
  println!("{:?}", sumar_textos("a", "5"));

  // Mini proyecto 2
  let vex = [10,11,12];
  println!("{:?}", promedio(&vex));
  let vex:[i32;0] = [];
  println!("{:?}", promedio(&vex));
}

fn sumar_textos(a:&str, b:&str) -> Result<i32, String>{
  let n1 = a.parse::<i32>()
  .map_err(|_| "El parámetro a no es un número".to_string())?;
  let n2 = b.parse::<i32>()
  .map_err(|_| "El parámetro b no es un número".to_string())?;
  Ok(n1+n2)
}

fn promedio(calificaciones:&[i32]) -> Option<f64>{
  let _ = calificaciones.get(0)?;
  let mut sum = 0;
  for n in calificaciones{
    sum+=n;
  }
  let sum = sum as f64;
  Some(sum/calificaciones.len() as f64)
}