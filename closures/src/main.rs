mod pipeline{
  pub struct PipeLine{
    steps: Vec<Box<dyn Fn(i32) -> i32>>
  }
  
  impl PipeLine{
    pub fn new() -> Self {
      Self{steps: vec![]}
    }

    pub fn add_rep<F>(&mut self, func: F) where F: Fn(i32) -> i32 + 'static{
      self.steps.push(Box::new(func));
    }

    pub fn execute(&self, mut value: i32) -> i32 {
      for f in &self.steps {
        value=f(value);
      }
      value
    }
  }
}

fn main() {

  // Mini proyecto 1
  let mut pl = pipeline::PipeLine::new();
  pl.add_rep(|e| e +2);
  pl.add_rep(|e| e*e);
  pl.add_rep(|e| e-10);
  println!("{}", pl.execute(5));
}

/*
// Fn -> Lectura
let closure = |a: i32| a* 2;

// FnMut -> Cambio
let mut count = 0;
let mut f = || {
    count += 1;
};

// FnOnce -> Posesión
let v = vec![1,2,3]; 
let f = move || drop(v);




fn crear() -> impl Fn(i32) -> i32 {
    |x| x + 1 Siempre y cuando se devuelva la misma funcion 
}

fn crear(flag: bool) -> Box<dyn Fn(i32)->i32> {
  if flag {
    Box::new(|x| x + 1)
  } else {
    Box::new(|x| x + 2)
  }
}

struct Pipeline<F> where F: Fn(i32)->i32 {
  step: F
}
let p = Pipeline {
  step: |x| x * 2
};


fn crear(factor: i32) -> impl Fn(i32)->i32 {
  move |x| x * factor
}
*/