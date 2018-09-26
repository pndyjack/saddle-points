pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
  let input_len: Vec<usize> = (0..input.len()).collect();
  let res = input_len
    .iter()
    .map(|&i| get_point(i, input))
    .filter(|&point| match point {
      Some(_) => true,
      None => false,
    }).map(|point| point.unwrap())
    .collect();
  res
}

fn get_point(i: usize, input: &[Vec<u64>]) -> Option<(usize, usize)> {
  if input[i].len() == 0 {
    return None;
  }
  let row_max = *(input[i].iter().max().unwrap());
  let input_row_len: Vec<usize> = (0..input[i].len()).collect();
  let &j = input_row_len
    .iter()
    .find(|&&j| input[i][j] == row_max)
    .unwrap();
  let point = (i, j);
  let input_len: Vec<usize> = (0..input.len()).collect();
  let found = input_len
    .iter()
    .all(|&k| input[point.0][point.1] <= input[k][point.1]);
  if found {
    Some(point)
  } else {
    None
  }
}
