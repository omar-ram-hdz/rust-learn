fn main() {
    // Mini proyecto 1
    let texto = "Hola mundo cruel";
    println!("Texto original: {}", texto);
    let texto = primera_palabra(texto);
    println!("Primera palabra: {}", texto);

    // Mini proyecto 2
    let arr = [5,10,15,20,25,30];
    println!("Original: {:?}", arr);
    let arr = &arr[1..=3];
    println!("Valores centrales: {:?}", arr);
}

fn primera_palabra(texto: &str) -> &str {
  for (i,b) in texto.bytes().enumerate(){
    if b == b' '{
      return &texto[..i]
    }      
  }
  &texto
}