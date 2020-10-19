extern crate termion;
use termion::color;
// Rules
// 1. Any live cell with two or three live neighbours survives.
// 2. Any dead cell with three live neighbours becomes a live cell.
// 3. All other live cells die in the next generation. Similarly, 
//    all other dead cells stay dead.
fn main() {
  let live_cells = vec![
    vec![0, 1],
    vec![0, 2],
    vec![0, 3],
    vec![1, 2],
    vec![4, 1],
    vec![4, 2],
    vec![4, 3],
  ];
  let rounds = 10;
  let size = 10;

  println!("{}Round 0", color::Fg(color::Yellow));
  print_matrix((&live_cells).to_vec(), size);
  let mut results = vec![live_cells];

  for i in 1..=rounds {
    println!("\nRound {}", i);

    let last_item = &results[results.len() - 1];

    let round = calculate_round((&last_item).to_vec(), size);
    print_matrix((&round).to_vec(), size);
    results.push(round)
  }
}

fn calculate_live_neigbours(ir: i32, ii: i32, live_cells: Vec<Vec<i32>>) -> i32 {
  let calc = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  let neigbours: i32 = calc
    .iter()
    // calculates neigbours
    .map(|x| [ir + x[0], ii + x[1]])
    .filter(|y| {
      let el = vec![y[0], y[1]];
      // return only those that are in the grid && are alive
      (y[0] > 0 || y[1] > 0) && live_cells.contains(&el)
    })
    .count() as i32;

  neigbours
}

fn print_matrix(live_cells: Vec<Vec<i32>>, size: i32) {
  let mut matrix = vec![];
  for ir1 in 0..size {
    let mut row = String::from("");
    for ii1 in 0..size {
      let el1 = vec![ir1, ii1];
      if live_cells.contains(&el1) {
        row.push_str("o ")
      } else {
        row.push_str("x ")
      }
    }
    matrix.push(row)
  }

  for row in matrix.iter() {
    println!("{:?}", row)
  }
}

fn calculate_round(live_cells: Vec<Vec<i32>>, size: i32) -> Vec<Vec<i32>> {
  let mut matrix = vec![];
  for ir in 0..size {
    for ii in 0..size {
      let el = vec![ir, ii];

      let number_of_live_neibours = calculate_live_neigbours(ir, ii, (&live_cells).to_vec());

      match (live_cells.contains(&el), number_of_live_neibours) {
        (false, 3) => matrix.push(el),
        (true, 2) => matrix.push(el),
        (true, 3) => matrix.push(el),
        _ => (),
      }
    }
  }

  matrix
}
