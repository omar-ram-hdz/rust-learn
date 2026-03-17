use libexample::shapes;
fn main() {
    let cuad = shapes::Cuadrado::new(10);
    if let Ok(area) = cuad.area() {
      println!("{}", area);
    }
}
