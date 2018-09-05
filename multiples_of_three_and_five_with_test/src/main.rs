pub fn multiples() -> i16{
  let mut sum = 0;
  for i in 0..10{
    if i % 3 == 0  || i % 5 == 0{
      sum = sum + i;
    }
  }
  return sum
}

#[cfg(test)]
mod tests {
    use super::multiples;

    #[test]
    fn testing_multiples_of_three() {
      let result = multiples();
      assert_eq!(23, result);
    }

}