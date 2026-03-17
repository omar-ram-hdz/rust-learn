mod geometry{
  pub struct Cuadrado(i32);

  impl Cuadrado{
    pub fn new(lado: i32) -> Cuadrado {
      Cuadrado(lado)
    }

    pub fn area_cuadrado(&self) -> Result<i32, String>{
      Self::validar_lado(&self.0).map_err(|e| format!("Error: {}", e))?;
      Ok(&self.0 * &self.0)
    }
    
    // fn validar_lado(lado: i32) -> Result<(), String>
    fn validar_lado(lado: &i32) -> Result<bool, String>{
      if *lado <= 0 {
        Err("El lado debe ser mayor a 0".to_string())
      }else {
        Ok(true)
      }
    }
  }
}

fn main() {
  let cuadrado = geometry::Cuadrado::new(0);
  println!("{:?}", cuadrado.area_cuadrado());
  let cuaddrado = geometry::Cuadrado::new(2);
  println!("{:?}", cuaddrado.area_cuadrado());
}
