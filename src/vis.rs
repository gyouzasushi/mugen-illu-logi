use gif::Gif;
use itertools::Itertools;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Text};
use svg::Document;

const D: i32 = 24;
const THEME: &str = "#7BC96F";

pub fn vis_grid_inner(h: usize, w: usize, d: i32, board: &Vec<Vec<Option<bool>>>) -> String {
    let mut doc = Document::new()
        .set("id", "vis")
        .set("viewBox", (0, 0, d * w as i32, d * h as i32))
        .set("width", d * w as i32)
        .set("height", d * h as i32);
    // grids
    for y in 0..h {
        for x in 0..w {
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "fill",
                        if board[y][x] == Some(true) {
                            "black"
                        } else {
                            "white"
                        },
                    )
                    .set("x", x as i32 * d)
                    .set("y", y as i32 * d)
                    .set("width", d)
                    .set("height", d)
                    .set("stroke-width", 0),
            );
        }
    }
    doc.to_string()
}

pub fn vis_gif_inner(h: usize, w: usize, d: u16, boards: &[Vec<Vec<bool>>]) -> Vec<u8> {
    let mut gif = Gif::new(h as u16 * d, w as u16 * d);
    for (i, board) in boards.iter().enumerate() {
        let delay = if i == boards.len() - 1 { 200 } else { 20 };
        gif.add(board, delay).unwrap();
    }
    gif.buffer
}

pub fn vis_board_inner(
    h: usize,
    w: usize,
    board: &Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
    fill: &str,
    offset_y: i32,
    offset_x: i32,
) -> String {
    let hints_hidden = get_hints_hidden(board, hints);

    let mut doc = Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                -5,
                -5,
                D * w as i32 + offset_x + 10,
                D * h as i32 + offset_y + 10,
            ),
        )
        .set("width", D * w as i32 + offset_x + 10)
        .set("height", D * h as i32 + offset_y + 10);

    // grids
    for y in 0..h {
        for x in 0..w {
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "fill",
                        if board[y][x] == Some(true) {
                            fill
                        } else {
                            "white"
                        },
                    )
                    .set("x", x as i32 * D + offset_x)
                    .set("y", y as i32 * D + offset_y)
                    .set("width", D)
                    .set("height", D)
                    .set("stroke", "black")
                    .set("stroke-width", 1),
            );
        }
    }

    // crosses
    for y in 0..h {
        for x in 0..w {
            if board[y][x] == Some(false) {
                doc = doc.add(
                    Path::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 0.5)
                        .set(
                            "d",
                            Data::new()
                                .move_to((x as i32 * D + offset_x, y as i32 * D + offset_y))
                                .line_by((D, D))
                                .move_by((0, -D))
                                .line_by((-D, D)),
                        ),
                );
            }
        }
    }

    // frames
    for y in (0..h).step_by(5) {
        for x in (0..w).step_by(5) {
            doc = doc.add(
                Rectangle::new()
                    .set("fill-opacity", 0)
                    .set("x", x as i32 * D + offset_x)
                    .set("y", y as i32 * D + offset_y)
                    .set("width", D * 5)
                    .set("height", D * 5)
                    .set("stroke", "black")
                    .set("stroke-width", 4),
            );
        }
    }

    // hints
    for (y, hints) in hints.0.iter().enumerate() {
        for (x, num) in hints.iter().enumerate() {
            let dx = if *num >= 10 { D / 4 } else { 0 };
            doc = doc.add(
                Text::new()
                    .set(
                        "x",
                        offset_x - D * (hints.len() - 1 - x) as i32 - D / 2 + dx,
                    )
                    .set("y", offset_y + D * y as i32 + D / 2)
                    .set(
                        "fill",
                        if hints_hidden.0[y][x] {
                            "lightgray"
                        } else {
                            "black"
                        },
                    )
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "middle")
                    .add(svg::node::Text::new(format!("{}", num))),
            );
        }
    }

    for (x, hints) in hints.1.iter().enumerate() {
        for (y, num) in hints.iter().enumerate() {
            doc = doc.add(
                Text::new()
                    .set("x", offset_x + D * x as i32 + D / 2)
                    .set("y", offset_y - D * (hints.len() - 1 - y) as i32 - D / 2)
                    .set(
                        "fill",
                        if hints_hidden.1[x][y] {
                            "lightgray"
                        } else {
                            "black"
                        },
                    )
                    .set("text-anchor", "middle")
                    .add(svg::node::Text::new(format!("{}", num))),
            );
        }
    }

    doc.to_string()
}

pub fn vis_gaming_boards_inner(
    h: usize,
    w: usize,
    board: &Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
    offset_y: i32,
    offset_x: i32,
) -> String {
    [
        "#ff0000", "#ff7f00", "#ffff00", "#7fff00", "#00ff00", "#00ff7f", "#00ffff", "#007fff",
        "#0000ff", "#7f00ff", "#ff00ff", "#ff007f",
    ]
    .iter()
    .map(|&fill| vis_board_inner(h, w, board, hints, fill, offset_y, offset_x))
    .join("$")
}

pub fn vis_cursor_inner(
    h: usize,
    w: usize,
    y: usize,
    x: usize,
    offset_y: i32,
    offset_x: i32,
) -> String {
    let mut doc = Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                -5,
                -5,
                D * w as i32 + offset_x + 10,
                D * h as i32 + offset_y + 10,
            ),
        )
        .set("width", D * w as i32 + offset_x + 10)
        .set("height", D * h as i32 + offset_y + 10);

    // cursor
    doc = doc.add(
        Rectangle::new()
            .set("x", x as i32 * D + offset_x)
            .set("y", y as i32 * D + offset_y)
            .set("width", D)
            .set("height", D)
            .set("fill-opacity", 0)
            .set("stroke", THEME)
            .set("stroke-width", 4),
    );

    // highlights
    doc = doc.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", D * y as i32 + offset_y)
            .set("width", D * w as i32 + offset_x)
            .set("height", D)
            .set("fill", THEME)
            .set("fill-opacity", 0.2)
            .set("stroke", THEME)
            .set("stroke-width", 0),
    );
    doc = doc.add(
        Rectangle::new()
            .set("x", D * x as i32 + offset_x)
            .set("y", 0)
            .set("width", D)
            .set("height", D * h as i32 + offset_x)
            .set("fill", THEME)
            .set("fill-opacity", 0.2)
            .set("stroke", THEME)
            .set("stroke-width", 0),
    );

    doc.to_string()
}

pub fn set_inner(
    y: usize,
    x: usize,
    val: Option<bool>,
    board: &mut Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
) {
    let (h, w) = (board.len(), board[0].len());

    board[y][x] = val;

    // y軸方向
    let true_count = (0..w).filter(|&x| board[y][x] == Some(true)).count();
    let none_count = (0..w).filter(|&x| board[y][x].is_none()).count();
    if true_count > hints.0[y].iter().sum::<i32>() as usize || none_count == 0 {
    } else {
        // (1) None <- true として完全一致か？
        let line = (0..w).map(|x| board[y][x] != Some(false)).collect_vec();
        let filled = compress(&line);
        if filled == hints.0[y] {
            for x in 0..w {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(true), board, hints);
                }
            }
        }
        // (2) None <- false として完全一致か？
        let line = (0..w).map(|x| board[y][x] == Some(true)).collect_vec();
        let filled = compress(&line);
        if filled == hints.0[y] {
            for x in 0..w {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(false), board, hints);
                }
            }
        }
        // (3) 前から見てどこまで一致するか？
        if (0..w).any(|x| board[y][x].is_none()) {
            let mut x = 0;
            let mut i = 0;
            while x < w && i < hints.0[y].len() {
                let mut nx = x;
                while nx < w && board[y][x] == board[y][nx] {
                    nx += 1;
                }
                let len = (nx - x) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.0[y][i] {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if x > 0 && board[y][x - 1] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                x = nx;
            }
        }
        // (4) 後から見てどこまで一致するか？
        if (0..w).any(|x| board[y][x].is_none()) {
            let mut x = w - 1;
            let mut i = hints.0[y].len() - 1;
            while x < w && i < hints.0[y].len() {
                let mut nx = x;
                while nx < w && board[y][x] == board[y][nx] {
                    nx -= 1;
                }
                let len = (x - nx) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.0[y][i] {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if x + 1 < w && board[y][x + 1] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                x = nx;
            }
        }
    }

    // x軸方向
    let true_count = (0..h).filter(|&y| board[y][x] == Some(true)).count();
    let none_count = (0..h).filter(|&y| board[y][x].is_none()).count();
    if true_count > hints.1[x].iter().sum::<i32>() as usize || none_count == 0 {
    } else {
        // (1) None <- true として完全一致か？
        let line = (0..h).map(|y| board[y][x] != Some(false)).collect_vec();
        let filled = compress(&line);
        if filled == hints.1[x] {
            for y in 0..h {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(true), board, hints);
                }
            }
        }
        // (2) None <- false として完全一致か？
        let line = (0..h).map(|y| board[y][x] == Some(true)).collect_vec();
        let filled = compress(&line);
        if filled == hints.1[x] {
            for y in 0..h {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(false), board, hints);
                }
            }
        }
        // (3) 前から見てどこまで一致するか？
        if (0..h).any(|y| board[y][x].is_none()) {
            let mut y = 0;
            let mut i = 0;
            while y < h && i < hints.1[x].len() {
                let mut ny = y;
                while ny < h && board[y][x] == board[ny][x] {
                    ny += 1;
                }
                let len = (ny - y) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.1[x][i] {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if y > 0 && board[y - 1][x] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                y = ny;
            }
        }
        // (4) 後から見てどこまで一致するか？
        if (0..h).any(|y| board[y][x].is_none()) {
            let mut y = h - 1;
            let mut i = hints.1[x].len() - 1;
            while y < h && i < hints.1[x].len() {
                let mut ny = y;
                while ny < h && board[y][x] == board[ny][x] {
                    ny -= 1;
                }
                let len = (y - ny) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.1[x][i] {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if y + 1 < h && board[y + 1][x] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                y = ny;
            }
        }
    }
}

fn compress(vec: &[bool]) -> Vec<i32> {
    let mut ret = Vec::<i32>::new();
    let mut i = 0;
    while i < vec.len() {
        let mut ni = i;
        while ni < vec.len() && vec[i] == vec[ni] {
            ni += 1;
        }
        if vec[i] {
            ret.push((ni - i) as i32);
        }
        i = ni;
    }
    ret
}

fn get_hints_hidden(
    board: &Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let (h, w) = (board.len(), board[0].len());
    let mut hints_hidden = (
        hints
            .0
            .iter()
            .map(|hints| hints.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        hints
            .1
            .iter()
            .map(|hints| hints.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    for y in 0..h {
        // y軸方向
        let true_count = (0..w).filter(|&x| board[y][x] == Some(true)).count();
        let none_count = (0..w).filter(|&x| board[y][x].is_none()).count();
        if true_count > hints.0[y].iter().sum::<i32>() as usize {
            // 全部間違っていることにする
            for hidden in &mut hints_hidden.0[y] {
                *hidden = false;
            }
        } else if none_count == 0 {
            let line = (0..w).map(|x| board[y][x].unwrap()).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                // 全部正しいことにする
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            } else {
                // 全部間違っていることにする
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = false;
                }
            }
        } else {
            // (1) None <- true として完全一致か？
            let line = (0..w).map(|x| board[y][x] != Some(false)).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            }
            // (2) None <- false として完全一致か？
            let line = (0..w).map(|x| board[y][x] == Some(true)).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            }
            // (3) 前から見てどこまで一致するか？
            if (0..w).any(|x| board[y][x].is_none()) {
                let mut x = 0;
                let mut i = 0;
                while x < w && i < hints.0[y].len() {
                    let mut nx = x;
                    while nx < w && board[y][x] == board[y][nx] {
                        nx += 1;
                    }
                    let len = (nx - x) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.0[y][i] {
                                break;
                            } else {
                                hints_hidden.0[y][i] = true;
                                i += 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    x = nx;
                }
            }
            // (4) 後から見てどこまで一致するか？
            if (0..w).any(|x| board[y][x].is_none()) {
                let mut x = w - 1;
                let mut i = hints.0[y].len() - 1;
                while x < w && i < hints.0[y].len() {
                    let mut nx = x;
                    while nx < w && board[y][x] == board[y][nx] {
                        nx -= 1;
                    }
                    let len = (x - nx) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.0[y][i] {
                                break;
                            } else {
                                hints_hidden.0[y][i] = true;
                                i -= 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    x = nx;
                }
            }
        }
    }
    for x in 0..w {
        let true_count = (0..h).filter(|&y| board[y][x] == Some(true)).count();
        let none_count = (0..h).filter(|&y| board[y][x].is_none()).count();
        if true_count > hints.1[x].iter().sum::<i32>() as usize {
            for hidden in &mut hints_hidden.1[x] {
                *hidden = false;
            }
        } else if none_count == 0 {
            let line = (0..h).map(|y| board[y][x].unwrap()).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                // 全部正しいことにする
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            } else {
                // 全部間違っていることにする
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = false;
                }
            }
        } else {
            // (1) None <- true として完全一致か？
            let line = (0..h).map(|y| board[y][x] != Some(false)).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            }
            // (2) None <- false として完全一致か？
            let line = (0..h).map(|y| board[y][x] == Some(true)).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            }
            // (3) 前から見てどこまで一致するか？
            if (0..h).any(|y| board[y][x].is_none()) {
                let mut y = 0;
                let mut i = 0;
                while y < h && i < hints.1[x].len() {
                    let mut ny = y;
                    while ny < h && board[y][x] == board[ny][x] {
                        ny += 1;
                    }
                    let len = (ny - y) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.1[x][i] {
                                break;
                            } else {
                                hints_hidden.1[x][i] = true;
                                i += 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    y = ny;
                }
            }
            // (4) 後から見てどこまで一致するか？
            if (0..h).any(|y| board[y][x].is_none()) {
                let mut y = h - 1;
                let mut i = hints.1[x].len() - 1;
                while y < h && i < hints.1[x].len() {
                    let mut ny = y;
                    while ny < h && board[y][x] == board[ny][x] {
                        ny -= 1;
                    }
                    let len = (y - ny) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.1[x][i] {
                                break;
                            } else {
                                hints_hidden.1[x][i] = true;
                                i -= 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    y = ny;
                }
            }
        }
    }

    hints_hidden
}

mod gif {
    use weezl::{encode::Encoder, BitOrder, LzwError};
    pub struct Gif {
        width: u16,
        height: u16,
        pub buffer: Vec<u8>,
    }
    impl Gif {
        pub fn new(width: u16, height: u16) -> Self {
            let width_upper = (width >> 8) as u8;
            let width_lower = (width % (1 << 8)) as u8;
            let height_upper = (height >> 8) as u8;
            let height_lower = (height % (1 << 8)) as u8;
            let header = [
                0x47,
                0x49,
                0x46,
                0x38,
                0x39,
                0x61,
                width_lower,
                width_upper,
                height_lower,
                height_upper,
                0x80,
                0x00,
                0x00,
                0xff,
                0xff,
                0xff,
                0x00,
                0x00,
                0x00,
            ];

            let application_extention = [
                0x21, 0xff, 0x0b, 0x4e, 0x45, 0x54, 0x53, 0x43, 0x41, 0x50, 0x45, 0x32, 0x2e, 0x30,
                0x03, 0x01, 0x00, 0x00, 0x00,
            ];

            let trailer = [0x3b];

            let mut buffer = Vec::<u8>::new();
            buffer.extend_from_slice(&header);
            buffer.extend_from_slice(&application_extention);
            buffer.extend_from_slice(&trailer);
            Self {
                width,
                height,
                buffer,
            }
        }

        pub fn add(&mut self, data: &[Vec<bool>], delay: u16) -> Result<(), LzwError> {
            let (h, w) = (data.len(), data[0].len());
            let size_h = self.height as usize / h;
            let size_w = self.height as usize / w;
            let data = (0..self.height as usize)
                .map(|y| (0..self.width as usize).map(move |x| data[y / size_h][x / size_w] as u8))
                .flatten()
                .collect::<Vec<_>>();
            self.add_inner(&data, delay)?;
            Ok(())
        }

        fn add_inner(&mut self, data: &[u8], delay: u16) -> Result<(), LzwError> {
            let trailer = self.buffer.pop().unwrap();

            let delay_upper = (delay >> 8) as u8;
            let delay_lower = (delay % (1 << 8)) as u8;
            let graphic_control_extention =
                [0x21, 0xf9, 0x04, 0x04, delay_lower, delay_upper, 0x00, 0x00];
            self.buffer.extend_from_slice(&graphic_control_extention);

            let width_upper = (self.width >> 8) as u8;
            let width_lower = (self.width % (1 << 8)) as u8;
            let height_upper = (self.height >> 8) as u8;
            let height_lower = (self.height % (1 << 8)) as u8;
            let mut image_block = Vec::<u8>::new();
            image_block.extend_from_slice(&[
                0x2c,
                0x00,
                0x00,
                0x00,
                0x00,
                width_lower,
                width_upper,
                height_lower,
                height_upper,
                0x00,
                0x02,
            ]);
            let buffer = Encoder::new(BitOrder::Lsb, 2).encode(data)?;
            let max_size = 0xff;
            for l in (0..buffer.len()).step_by(max_size) {
                let r = (l + max_size).min(buffer.len());
                let block_size = (r - l) as u8;
                image_block.push(block_size);
                image_block.extend_from_slice(&buffer[l..r]);
            }
            image_block.push(0x00);
            self.buffer.append(&mut image_block);
            self.buffer.push(trailer);
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
