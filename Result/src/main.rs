/*
enum Result <T, E>{
  Ok(T),
  Err(e),
}
 */

fn main() {
  // Mini proyecto 1
  println!("{:?}", leer_entero("10"));
  println!("{:?}", leer_entero("hola"));
  if let Ok(res) = leer_entero("20"){
    println!("{}", res)
  };

  // Mini proyecto 2
  let result = division_segura(10.0, 2.0);
  imprimir_resultado(result);
  let result_wrong = division_segura(10.0, 0.0);
  imprimir_resultado(result_wrong);
}

fn imprimir_resultado(result: Result<f64, String>){
  match result {
    Ok(r) => println!("El resultado es: {}", r),
    Err(e) => println!("Error: {}", e)
  }
}

fn division_segura(a: f64, b: f64) -> Result<f64, String>{
  if b == 0.0 {
    Err("No se puede dividir por 0".to_string())
  }else {
    Ok(a/b)
  }
}

fn leer_entero(texto: &str) -> Result<i32, String>{
  let entero = texto.parse::<i32>();
  match entero {
    Ok(n) => Ok(n),
    Err(_) => Err("No es un número".to_string()),
  }
}
/*
fn leer_entero(texto: &str) -> Result<i32, String>{
    texto
        .parse::<i32>()
        .map_err(|_| "No es un número".to_string())
}

fn leer_entero(texto: &str) -> Result<i32, String>{
    let n = texto.parse::<i32>()
        .map_err(|_| "No es un número".to_string())?;
    Ok(n)
}
*/