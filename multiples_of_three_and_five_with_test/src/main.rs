pub fn multiples() -> i16 {
  let mut sum_of_multiples = 0;
  for number in 0..10{
    if number % 3 == 0  || number % 5 == 0{
      sum_of_multiples = sum_of_multiples + i;
    }
  }
  return sum
}


#[cfg(test)]
mod tests {
    use super::multiples;

    #[test]
    fn testing_sum_of_multiples() {
      let result = multiples();
      assert_eq!(23, result);
    }

}