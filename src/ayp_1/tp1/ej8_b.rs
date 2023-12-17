use std::io;

// Dados los catetos de un triángulo rectángulo calcular:
// a) Su hipotenusa
// b) Su perímetro
pub(crate) fn tp1_ej8_b() {
  
  println!("Cargar C1");
  let mut c1 = String::new();

  io::stdin().read_line(&mut c1)
    .expect("Error al leer C1");

  let c1: f32 = c1.trim().parse()
    .expect("C1 debe ser un numero");

  println!("Cargar C2");
  let mut c2 = String::new();

  io::stdin().read_line(&mut c2)
    .expect("Error al leer C2");

  let c2: f32 = c2.trim().parse()
    .expect("C2 debe ser un numero");

  let h = f32::sqrt(c1.powf(2.0) + c2.powf(2.0));

  let perimetro = c1 + c2 + h;

  println!("Triangulo c/ Catetos C1= {:.2} y C2={:.2}", c1, c2);
  println!("Hipotenusa={:.2}", h);
  println!("Perimetro={:.2}", perimetro);

}