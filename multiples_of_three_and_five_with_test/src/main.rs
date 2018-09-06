pub fn get_sum_of_multiples() -> i16 {
  let sum_of_multiples = sum_multiples_of_three_and_five();
  return sum_of_multiples
}

fn sum_multiples_of_three_and_five() -> i16 {
  let mut sum_of_multiples = 0;
  for number in 0..10{
    if number % 3 == 0  || number % 5 == 0 {
      sum_of_multiples = sum_of_multiples + number;
    }
  }
  return sum_of_multiples
}

#[cfg(test)]
mod tests {
    use super::get_sum_of_multiples;

    #[test]
    fn testing_sum_of_multiples() {
      let result = get_sum_of_multiples();
      assert_eq!(23, result);
    }

}