// Rules
// 1. Any live cell with two or three live neighbours survives.
// 2. Any dead cell with three live neighbours becomes a live cell.
// 3. All other live cells die in the next generation. Similarly,
//    all other dead cells stay dead.

type LiveCells = Vec<Vec<i32>>;
type Size = i32;
type Round = i32;

fn calculate_live_neigbours(ir: i32, ii: i32, live_cells: LiveCells) -> i32 {
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

fn print_matrix(live_cells: LiveCells, size: Size, round: Round) {
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

  println!("Round {:?}", round);

  for row in matrix.iter() {
    println!("{:?}", row)
  }
}

fn calculate_round(live_cells: LiveCells, size: Size) -> LiveCells {
  let mut next_live_cells = vec![];
  for ir in 0..size {
    for ii in 0..size {
      let el = vec![ir, ii];

      let number_of_live_neibours = calculate_live_neigbours(ir, ii, (&live_cells).to_vec());

      match (live_cells.contains(&el), number_of_live_neibours) {
        (false, 3) => next_live_cells.push(el),
        (true, 2) => next_live_cells.push(el),
        (true, 3) => next_live_cells.push(el),
        _ => (),
      }
    }
  }

  next_live_cells
}

struct GameOfLife {
  live_cells: LiveCells,
  round: Round,
  size: Size,
}

impl Iterator for GameOfLife {
  type Item = (LiveCells, Round, Size);
  fn next(&mut self) -> Option<Self::Item> {
    let round = calculate_round((&self.live_cells).to_vec(), self.size);

    self.live_cells = round;
    self.round = self.round + 1;

    Some(((&self.live_cells).to_vec(), self.round, self.size))
  }
}

fn game_of_life(size: Size, live_cells: LiveCells) -> GameOfLife {
  GameOfLife {
    live_cells: live_cells,
    round: 0,
    size: size,
  }
}

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

  let size = 10;
  let rounds = 20;

  print_matrix((&live_cells).to_vec(), size, 0);

  for i in game_of_life(size, live_cells).take(rounds) {
    print_matrix(i.0, i.1, i.2);
  }
}
