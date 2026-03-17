pub mod shapes {
  pub struct Cuadrado(i32);
  impl Cuadrado {
    pub fn new(lado: i32) -> Self {
      Cuadrado(lado)
    }
    pub fn area(&self)  -> Result<i32,String>{
      if self.0 < 0 {
        return Err("Ingresa un valor mayor".to_string());
      }
      Ok(&self.0 * &self.0)
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba(){}
}
