use std::collections::HashMap;

fn main() {
  // Mini proyecto 1
  let mut mapa_edades: HashMap<&str, i32> = HashMap::new();
  mapa_edades.insert("Ana", 21);
  mapa_edades.insert("Luis", 19);
  mapa_edades.insert("Marta", 25);
  for (nombre, edad) in &mapa_edades{
    println!("{} tiene {} años", nombre, edad);
  }

  // Mini proyecto 2
  let mut lenguajes: HashMap<&str, i32> = HashMap::new();
  // let valor = lenguajes.get("key");
  /*for palabra in "rust rust python rust go python".split_whitespace(){
    lenguajes.entry(palabra)
    .and_modify(|count| *count +=1)
    .or_insert(1);
  }*/
  for palabra in "rust rust python rust go python".split_whitespace(){
    let contador = lenguajes.entry(palabra).or_insert(0);
    *contador +=1;
  }
  println!("{:?}", lenguajes);
  /*
  if let contador = lenguajes.get_mut(lenguaje){
    *contador += 1;
  }
   */
}
