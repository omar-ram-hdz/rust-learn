fn main() {
    // Mini proyecto 1
    let mut vector: Vec<i32> = Vec::new();
    for i in 1..=5{
      vector.push(i);
    }
    for n in &vector{
      println!("{}", n);
    }

    // Mini proyecto 2
    let mut vector = vec![10,20,30,40,50];
    for n in &mut vector{
      *n += 5;
    }
    println!("{:?}", vector);
}
