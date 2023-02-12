mod message {
    use crate::utils::clear;
    pub fn print_hello() {
        clear();
        println!("┌─────────────────────────────────────────────────┐");
        println!("│      Добро пожаловать в игру 2048 на Rust!      │");
        println!("│  Необходимо сдвигать поле в разные направления  │");
        println!("│ чтобы складывать одинаковые блоки. 2 + 2 = 4... │");
        println!("│     Сдвигать поле можно клавишами - W A S D     │");
        println!("│       Цель получить клеточку 2048, удачи!       │");
        println!("└─────────────────────────────────────────────────┘")
    }
}

mod color {
    use crossterm::style::Color;

    pub const CELL_2: Color = Color::Rgb {
        r: 238,
        g: 228,
        b: 218,
    };
    pub const CELL_4: Color = Color::Rgb {
        r: 237,
        g: 224,
        b: 200,
    };
    pub const CELL_8: Color = Color::Rgb {
        r: 242,
        g: 177,
        b: 121,
    };
    pub const CELL_16: Color = Color::Rgb {
        r: 245,
        g: 149,
        b: 99,
    };
    pub const CELL_32: Color = Color::Rgb {
        r: 246,
        g: 124,
        b: 95,
    };
    pub const CELL_64: Color = Color::Rgb {
        r: 246,
        g: 94,
        b: 59,
    };
    pub const CELL_128: Color = Color::Rgb {
        r: 237,
        g: 207,
        b: 114,
    };
    pub const CELL_256: Color = Color::Rgb {
        r: 237,
        g: 204,
        b: 97,
    };
    pub const CELL_512: Color = Color::Rgb {
        r: 237,
        g: 200,
        b: 90,
    };
    pub const CELL_1024: Color = Color::Rgb {
        r: 237,
        g: 197,
        b: 63,
    };
    pub const CELL_2048: Color = Color::Rgb {
        r: 237,
        g: 194,
        b: 46,
    };

    pub const DARK_GREY: Color = Color::DarkGrey;
}

mod game {
    use std::cmp::min;

    use rand::{random, seq::SliceRandom, thread_rng};

    use crate::{
        draw,
        utils::{clear, flip},
    };

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Cell {
        Value(u16),
        Empty,
    }

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub struct State {
        pub width: u32,
        pub height: u32,
        pub field: Vec<Vec<Cell>>,
        pub is_run: bool,
        pub empty_count: u32,
    }

    impl Default for State {
        fn default() -> Self {
            State::new(4, 4)
        }
    }

    impl State {
        pub fn new(width: u32, height: u32) -> State {
            Self {
                field: (0..width)
                    .map(|_| (0..height).map(|_| Cell::Empty).collect())
                    .collect(),
                is_run: true,
                empty_count: width * height,
                width,
                height,
            }
        }

        pub fn randomize(&mut self) {
            let mut empty_cells: Vec<(usize, usize)> =
                Vec::with_capacity((self.width * self.height) as usize);

            for (i, row) in self.field.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if matches!(c, Cell::Empty) {
                        empty_cells.push((i, j))
                    }
                }
            }

            empty_cells.shuffle(&mut thread_rng());

            // Рассчитываем максимальное количество случайных чисел
            // по формуле в целых числах log2(h * w) - 2
            let rand_bound = (f64::log2((self.width * self.height) as f64).floor() as u32) - 2;
            let rand_count = min(random::<u32>() % rand_bound + 1, empty_cells.len() as u32);

            // Шанс 75% на 2, 25% на 4
            for i in 0..rand_count {
                let (r, c) = empty_cells[i as usize];
                self.field[r][c] = if random::<u32>() % 100 + 1 >= 75 {
                    Cell::Value(4)
                } else {
                    Cell::Value(2)
                };
            }

            self.empty_count -= rand_count;
        }

        pub fn move_to(&mut self, direction: Direction) -> bool {
            let mut is_moved = false;

            match direction {
                Direction::Up => {
                    for j in 0..self.width as usize {
                        let mut i0: usize = 0;
                        let mut i1: usize = 1;

                        loop {
                            match self.field[i0][j] {
                                Cell::Empty => match self.field[i1][j] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        self.field[i0][j] = Cell::Value(i1_v);
                                        self.field[i1][j] = Cell::Empty;

                                        is_moved = true;
                                    }
                                },
                                Cell::Value(i0_v) => match self.field[i1][j] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        if i0_v == i1_v {
                                            self.field[i0][j] = Cell::Value(i0_v * 2);
                                            self.field[i1][j] = Cell::Empty;

                                            self.empty_count += 1;
                                        } else {
                                            let tmp = self.field[i0 + 1][j];
                                            self.field[i0 + 1][j] = self.field[i1][j];
                                            self.field[i1][j] = tmp;

                                            i0 += 1;
                                        }
                                        is_moved = true;
                                    }
                                },
                            }
                            i1 = i1.wrapping_add(1);

                            if i1 >= self.height as usize {
                                break;
                            }
                        }
                    }
                }
                Direction::Down => {
                    for j in 0..self.width as usize {
                        let mut i0: usize = (self.height - 1) as usize;
                        let mut i1: usize = (self.height - 2) as usize;

                        loop {
                            match self.field[i0][j] {
                                Cell::Empty => match self.field[i1][j] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        self.field[i0][j] = Cell::Value(i1_v);
                                        self.field[i1][j] = Cell::Empty;

                                        is_moved = true;
                                    }
                                },
                                Cell::Value(i0_v) => match self.field[i1][j] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        if i0_v == i1_v {
                                            self.field[i0][j] = Cell::Value(i0_v * 2);
                                            self.field[i1][j] = Cell::Empty;

                                            self.empty_count += 1;
                                        } else {
                                            let tmp = self.field[i0 - 1][j];
                                            self.field[i0 - 1][j] = self.field[i1][j];
                                            self.field[i1][j] = tmp;

                                            i0 -= 1;
                                        }
                                        is_moved = true;
                                    }
                                },
                            }
                            i1 = i1.wrapping_sub(1);

                            if i1 >= self.height as usize {
                                break;
                            }
                        }
                    }
                }
                Direction::Left => {
                    for i in 0..self.height as usize {
                        let mut j0: usize = 0 as usize;
                        let mut j1: usize = 1 as usize;

                        loop {
                            match self.field[i][j0] {
                                Cell::Empty => match self.field[i][j1] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        self.field[i][j0] = Cell::Value(i1_v);
                                        self.field[i][j1] = Cell::Empty;

                                        is_moved = true;
                                    }
                                },
                                Cell::Value(j0_v) => match self.field[i][j1] {
                                    Cell::Empty => (),
                                    Cell::Value(j1_v) => {
                                        if j0_v == j1_v {
                                            self.field[i][j0] = Cell::Value(j0_v * 2);
                                            self.field[i][j1] = Cell::Empty;

                                            self.empty_count += 1;
                                        } else {
                                            let tmp = self.field[i][j0 + 1];
                                            self.field[i][j0 + 1] = self.field[i][j1];
                                            self.field[i][j1] = tmp;

                                            j0 += 1;
                                        }

                                        is_moved = true;
                                    }
                                },
                            }
                            j1 = j1.wrapping_add(1);

                            if j1 >= self.width as usize {
                                break;
                            }
                        }
                    }
                }
                Direction::Right => {
                    for i in 0..self.height as usize {
                        let mut j0: usize = (self.width - 1) as usize;
                        let mut j1: usize = (self.width - 2) as usize;

                        loop {
                            match self.field[i][j0] {
                                Cell::Empty => match self.field[i][j1] {
                                    Cell::Empty => (),
                                    Cell::Value(i1_v) => {
                                        self.field[i][j0] = Cell::Value(i1_v);
                                        self.field[i][j1] = Cell::Empty;

                                        is_moved = true;
                                    }
                                },
                                Cell::Value(j0_v) => match self.field[i][j1] {
                                    Cell::Empty => (),
                                    Cell::Value(j1_v) => {
                                        if j0_v == j1_v {
                                            self.field[i][j0] = Cell::Value(j0_v * 2);
                                            self.field[i][j1] = Cell::Empty;

                                            self.empty_count += 1;
                                        } else {
                                            let tmp = self.field[i][j0 - 1];
                                            self.field[i][j0 - 1] = self.field[i][j1];
                                            self.field[i][j1] = tmp;

                                            j0 -= 1;
                                        }

                                        is_moved = true;
                                    }
                                },
                            }
                            j1 = j1.wrapping_sub(1);

                            if j1 >= self.width as usize {
                                break;
                            }
                        }
                    }
                }
            }

            is_moved
        }

        pub fn check_lose(&self) -> bool {
            if self.empty_count != 0 {
                return false;
            }

            for i in 0..(self.height) as usize {
                for j in 0..(self.width) as usize {
                    if matches!(self.field[i][j], Cell::Empty) {
                        return false;
                    }

                    if i > 0 && self.field[i][j] == self.field[i - 1][j] {
                        // up
                        return false;
                    }

                    if i < self.height as usize - 1 && self.field[i][j] == self.field[i + 1][j] {
                        // down
                        return false;
                    }

                    if j > 0 && self.field[i][j] == self.field[i][j - 1] {
                        // left
                        return false;
                    }

                    if j < self.width as usize - 1 && self.field[i][j] == self.field[i][j + 1] {
                        // right
                        return false;
                    }
                }
            }

            true
        }

        pub fn check_win(&self) -> bool {
            for i in 0..self.height as usize {
                for j in 0..self.width as usize {
                    if let Cell::Value(2048) = self.field[i][j] {
                        return true;
                    }
                }
            }
            false
        }
    }

    pub fn run() {
        use crate::message::print_hello;
        use crate::utils::get_input;

        print_hello();

        let mut state = State::default(); // TODO: сделать выбор размера поля
        state.randomize();
        draw::draw(&state);

        while state.is_run {
            let input_c = get_input(&"aswd");

            let is_moved = match input_c {
                'w' => state.move_to(Direction::Up),
                's' => state.move_to(Direction::Down),
                'a' => state.move_to(Direction::Left),
                'd' => state.move_to(Direction::Right),
                _ => {
                    println!("Некорректная команда!");
                    continue;
                }
            };

            if is_moved {
                if state.check_win() {
                    flip(&state);
                    println!("Вы победили, поздравляем!");
                    println!("Нажмите Enter клавишу чтобы выйти...");
                    get_input("\x0d");
                    clear();
                    break;
                }

                state.randomize();

                flip(&state);

                if state.check_lose() {
                    println!("Увы, вы проиграли, попробуйте снова!");
                    println!("Нажмите Enter любую клавишу чтобы выйти...");
                    get_input("\x0d");
                    clear();
                    break;
                }
            } else {
                flip(&state);
            }
        }
    }
}

mod draw {
    use crossterm::style::Stylize;

    use crate::color;
    use crate::game::{Cell, State};
    use std::iter::repeat;

    impl Cell {
        pub(super) fn get_icon(&self) -> String {
            match self {
                Cell::Empty => String::from("#").with(color::DARK_GREY).to_string(),
                Cell::Value(x) => match x {
                    2 => String::from("2").with(color::CELL_2).to_string(),
                    4 => String::from("4").with(color::CELL_4).to_string(),
                    8 => String::from("8").with(color::CELL_8).to_string(),
                    16 => String::from("A").with(color::CELL_16).to_string(),
                    32 => String::from("B").with(color::CELL_32).to_string(),
                    64 => String::from("C").with(color::CELL_64).to_string(),
                    128 => String::from("D").with(color::CELL_128).to_string(),
                    256 => String::from("E").with(color::CELL_256).to_string(),
                    512 => String::from("F").with(color::CELL_512).to_string(),
                    1024 => String::from("G").with(color::CELL_1024).to_string(),
                    2048 => String::from("*").with(color::CELL_2048).to_string(),
                    _ => panic!("Invalid cell value!"),
                },
            }
        }
    }

    pub fn draw(state: &State) {
        // header
        println!(
            "┌─{}┐",
            repeat("──").take(state.width as usize).collect::<String>()
        );

        for row in state.field.iter() {
            print!("│ ");
            let row_s = row
                .iter()
                .map(|c| c.get_icon())
                .collect::<Vec<String>>()
                .join(" ");
            println!("{} │", row_s);
        }

        // footer
        println!(
            "└─{}┘",
            repeat("──").take(state.width as usize).collect::<String>()
        );
    }
}

mod utils {
    use crate::draw::draw;
    use crate::game::State;
    use crossterm::terminal;
    use std::io;
    use std::io::Read;

    pub fn clear() {
        print!("{esc}c", esc = 27 as char)
    }

    pub fn flip(state: &State) {
        clear();
        draw(&state);
    }

    fn ctrlc_handler() {
        if crossterm::terminal::is_raw_mode_enabled().expect("Could not check is raw mode enabled")
        {
            terminal::disable_raw_mode().expect("Could not disable raw mode");
        }
        enable_cursor();
        clear();
        std::process::exit(-1);
    }

    pub fn set_ctrlc_handler() {
        ctrlc::set_handler(ctrlc_handler).expect("Error setting Ctrl-C handler");
    }

    pub fn get_input(one_of: &str) -> char {
        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
        let mut buf = [0; 1];
        let ch: char;
        loop {
            io::stdin().read(&mut buf).expect("Failed to read line");
            match buf[0] {
                b'\x03' => {
                    ctrlc_handler();
                }
                x => {
                    if one_of.len() > 0 && !one_of.contains(x as char) {
                        continue;
                    }

                    ch = x as char;
                    break;
                }
            }
        }
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        ch
    }

    pub fn disable_cursor() {
        print!("{esc}[?25l", esc = 27 as char)
    }

    pub fn enable_cursor() {
        print!("{esc}[?25h", esc = 27 as char)
    }
}

fn main() {
    utils::disable_cursor();
    utils::set_ctrlc_handler();
    game::run();
}
