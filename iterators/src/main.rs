fn main() {
  let nums = vec![1,2,3,4,5,6];
  println!("Vector original: {:?}", nums);
  let pares: Vec<i32> = nums.into_iter().filter(|e| e % 2 == 0).collect();
  println!("Vector con solo pares: {:?}", pares);
  let producto: Vec<i32> = pares.iter().map(|e| e * 10).collect();
  println!("Vector con pares por 10: {:?}", producto);
  let suma: i32 = producto.iter().sum();
  println!("Suma: {}", suma);
}
