use std::fmt;
fn main() {
    solve(
        15,
        15,
        vec![
            vec![2, 2],
            vec![1, 1, 1, 1],
            vec![15],
            vec![4, 1, 4],
            vec![3, 3, 3],
            vec![1, 1, 1],
            vec![1, 3, 1],
            vec![1, 1],
            vec![3, 1, 1, 3],
            vec![1, 1, 1, 1],
            vec![3, 3, 3],
            vec![1, 1, 1],
            vec![2, 2],
            vec![2, 2],
            vec![11],
        ],
        vec![
            vec![4, 7],
            vec![1, 4, 1, 1, 2],
            vec![5, 1, 1, 2],
            vec![2, 1],
            vec![1, 2, 1],
            vec![1, 1],
            vec![1, 1, 1, 1, 1],
            vec![6, 2, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![2, 1],
            vec![5, 1, 1, 2],
            vec![1, 4, 1, 1, 2],
            vec![4, 7],
        ],
    );
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Empty,
    White,
    Black,
}
struct Board(Vec<Vec<State>>);
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 {
            for state in line {
                write!(
                    f,
                    "{}",
                    match state {
                        State::Empty => "?",
                        State::White => ".",
                        State::Black => "o",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Board {
    fn new(h: usize, w: usize) -> Self {
        Board(vec![vec![State::Empty; w]; h])
    }
    fn transpose(&mut self) {
        let h = self.0.len();
        let w = self.0[0].len();
        self.0 = (0..w)
            .map(|x| (0..h).map(|y| self.0[y][x]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }
}
fn solve(h: usize, w: usize, hints_horizontal: Vec<Vec<i32>>, hints_vertical: Vec<Vec<i32>>) {
    let mut board = Board::new(h, w);
    loop {
        let mut upd = false;
        for y in 0..h {
            upd |= solve_line(&mut board.0[y], &hints_horizontal[y]);
        }
        board.transpose();
        for x in 0..w {
            upd |= solve_line(&mut board.0[x], &hints_vertical[x]);
        }
        board.transpose();
        if !upd {
            break;
        }
        println!("{:?}", board);
    }
}
fn solve_line(line: &mut Vec<State>, hint: &Vec<i32>) -> bool {
    let empty_count = line.iter().filter(|&&state| state == State::Empty).count();
    if empty_count == 0 {
        return false;
    }
    let mut upd = false;
    // 空きマスが少なければ全探索、多ければいい感じに埋める、がしたい〜〜
    let mut use_count: Vec<usize> = vec![0; line.len()];
    let mut tot: usize = 0;
    for bit in 0..(1 << empty_count) {
        let mut d = 0;
        let mut tmp_line = line.clone();
        for state in &mut tmp_line {
            if *state != State::Empty {
                continue;
            }
            *state = if bit >> d & 1 == 0 {
                State::Black
            } else {
                State::White
            };
            d += 1;
        }
        if compress(&tmp_line) != *hint {
            continue;
        }
        for (i, &state) in tmp_line.iter().enumerate() {
            if state == State::Black {
                use_count[i] += 1;
            }
        }
        tot += 1;
    }

    for (i, &count) in use_count.iter().enumerate() {
        if count == tot && line[i] == State::Empty {
            line[i] = State::Black;
            upd = true;
        } else if count == 0 && line[i] == State::Empty {
            line[i] = State::White;
            upd = true;
        }
    }
    if compress(line) == *hint {
        for state in line {
            if *state == State::Empty {
                *state = State::White;
                upd = true;
            }
        }
    }
    upd
}
fn compress(line: &Vec<State>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < line.len() {
        let mut j = i;
        while j < line.len() && line[i] == line[j] {
            j += 1;
        }
        if line[i] == State::Black {
            ret.push((j - i) as i32);
        }
        i = j;
    }
    ret
}
