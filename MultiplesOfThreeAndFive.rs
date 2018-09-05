fn main() {
  let mut sum = 0;
  for i in 0..10{
    if i % 3 == 0  || i % 5 == 0{
      sum = sum + i;
    }
  }
  println!("El valor de numero: {}", sum)
}