#![feature(int_abs_diff)]
#![allow(unused)]

pub mod sonar {
    #[derive(Debug)]
    enum DepthDelta {
        NA,
        Increased,
        Decreased,
    }

    fn calculate_deltas(delta_arr: &[usize]) -> Vec<DepthDelta> {
        let mut out: Vec<DepthDelta> = vec![];
        for item in delta_arr.iter().enumerate() {
            let (i, x) = item;
            let mut delta: DepthDelta = DepthDelta::NA; // ugly but fair enough
            if i == 0 {
                out.push(delta);
                continue;
            } else if &delta_arr[i - 1] > x {
                delta = DepthDelta::Decreased
            } else {
                delta = DepthDelta::Increased
            }
            out.push(delta);
        }
        out
    }
}
pub mod submarine_body {
    pub struct Position {
        horizontal: usize,
        depth: usize,
    }

    impl Position {
        pub fn new() -> Position {
            Position {
                horizontal: 0,
                depth: 0,
            }
        }

        pub fn forward(&mut self, x: usize) {
            self.horizontal += x;
        }

        pub fn down(&mut self, y: usize) {
            self.depth += y;
        }

        pub fn up(&mut self, y: usize) {
            self.depth -= y;
        }

        pub fn print_pos(&self) {
            println!(
                "Submarine position: distance {}, depth {}",
                self.horizontal, self.depth
            );
        }
    }
    pub fn test() {
        let mut submarine = Position::new();
        submarine.down(10);
        submarine.up(2);
        submarine.forward(7);
        submarine.print_pos();
    }
}

pub mod diagnostics {

    fn calculate_gamma(reaport: Vec<u8>) -> u8 {
        let mut occurences_of_one: [usize; 5] = [0, 0, 0, 0, 0];
        for i in reaport.iter() {
            let jj: [usize; 5] = [0, 1, 2, 3, 4];
            for j in jj {
                if (1 << j & i) != 0 {
                    occurences_of_one[j] += 1;
                }
            }
        }
        let mut out: u8 = 0;
        for i in 0..occurences_of_one.len() {
            if occurences_of_one[i] >= reaport.len() / 2 {
                out += 1 << i;
            }
        }
        out
    }

    pub fn test() {
        let input: Vec<u8> = vec![0b10001, 0b01110, 0b00010, 0b00000];
        let out = calculate_gamma(input);
        println!("{}", out);
    }
}

pub mod bingo {

    const BOARD_SIZE: usize = 5;
    struct Board {
        number_table: Vec<Vec<usize>>,
        marked_ones: Vec<Vec<bool>>, // this table is for highlighting choosen fields only
        won_possibilities: Vec<usize>, //this should contain number of shoots in all available rws and cols, len should be 2*size of number table (or 10 because BINGO have 5 letters in itself)
    }

    trait BingoBoard {
        // Trait is not needed Here but I'd like to test of Trait implementation
        fn display(&self);
        fn mark_number(&mut self, input: usize) -> bool;
    }

    impl BingoBoard for Board {
        fn display(&self) {
            for row in self.number_table.iter() {
                for n in row.iter() {
                    if *n < 10 as usize {
                        print!("{}  ", n);
                    } else {
                        print!("{} ", n);
                    }
                }
                print!("\n");
            }
        }
        fn mark_number(&mut self, input: usize) -> bool {
            for (i, v) in self.number_table.iter().enumerate() {
                for (j, x) in v.iter().enumerate() {
                    if *x == input {
                        self.marked_ones[i][j] = true; // just to highlight what field was choosen
                        self.won_possibilities[i] += 1;
                        if self.won_possibilities[i] == BOARD_SIZE {
                            return true;
                        }
                        self.won_possibilities[j + BOARD_SIZE] += 1;
                        if self.won_possibilities[j + BOARD_SIZE] == BOARD_SIZE {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
    }
    pub fn test() {
        // I know, i should implement "::new()" for it
        let mut board = Board {
            number_table: vec![
                vec![1, 2, 1, 2, 1],
                vec![3, 4, 3, 4, 3],
                vec![5, 6, 5, 6, 5],
                vec![7, 8, 7, 8, 7],
                vec![9, 10, 9, 10, 9],
            ],
            marked_ones: vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
            ],
            won_possibilities: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        board.display();
        let result = board.mark_number(2);
        println!("{}", result);
        let result = board.mark_number(4);
        println!("{}", result);
        let result = board.mark_number(6);
        println!("{}", result);
        let result = board.mark_number(8);
        println!("{}", result);
        let result = board.mark_number(10);
        println!("{}", result);
    }
}


pub mod hydrothermal {
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Point {
        x: usize,
        y: usize
    }
    #[derive(Debug)]
    struct LineTerminals {
        a: Point,
        b: Point
    }
    use std::fs::File;
    use std::io::Read;

    fn import_line_terminals_from_file(filename: &str) -> Vec<LineTerminals> { // one of the most ineffecient parsers i guess.
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        // Parse to vector of LineTerminals
        let mut out: Vec<LineTerminals>  = Vec::new();
        for line in content.lines() {
            let numbers: Vec<&str> = line.split(" -> ").collect();
            let pa_xy: Vec<&str> = numbers[0 as usize].split(",").collect();
            let pa = Point{x:pa_xy[0 as usize].parse().unwrap(), y:pa_xy[1 as usize].parse().unwrap()};
            let pb_xy: Vec<&str> = numbers[1 as usize].split(",").collect();
            let pb = Point{x:pb_xy[0 as usize].parse().unwrap(), y:pb_xy[1 as usize].parse().unwrap()};
            let line = LineTerminals{a:pa, b:pb};
            out.push(line);
        }
        out
    }

    use std::collections::HashMap;
    fn create_danger_map(line_terminals: Vec<LineTerminals>) -> HashMap<Point, usize>{
        let mut danger_map: HashMap<Point, usize> = HashMap::new();
        for line_t in line_terminals {
            match calculate_danger_points(line_t) {
                Ok(danger_points) => {
                    for point in danger_points {
                        *danger_map.entry(point).or_insert(1) += 1;
                    }
                }
                Err(msg) => {
                    println!("WARNING: {}", msg);
                }
            }
        }
        danger_map
    }

    use std::cmp;
    fn calculate_danger_points(line_terminals: LineTerminals) -> Result<Vec<Point>, &'static str> {
        // This is not optimal solution, I tried with abs, but it would be better just ot make if statement
        // for which number is less, the iterate over it fo the value of second like : for i in line_terminals.a.x..line_terminals.b.x {}
        let mut danger_points: Vec<Point> = Vec::new();
        if line_terminals.a.x == line_terminals.b.x {
            let common_x = line_terminals.a.x;
            let dy = line_terminals.a.y.abs_diff(line_terminals.b.y);
            let begin = std::cmp::min(line_terminals.a.y, line_terminals.b.y);
            for i in 0..dy+1 {
                let point = Point{x: common_x, y: begin+i};
                danger_points.push(point);
            }
            Ok(danger_points)
        }
        else if line_terminals.a.y == line_terminals.b.y {
            let common_y = line_terminals.a.y;
            let dx = line_terminals.a.x.abs_diff(line_terminals.b.x);
            let begin = std::cmp::min(line_terminals.a.x, line_terminals.b.x);
            for i in 0..dx+1 {
                let point = Point{x: begin+i, y: common_y};
                danger_points.push(point);
            }
            Ok(danger_points)
        }
        else{
            Err("Line arent vertical or horizontal")
        }
    }

    pub fn test() {
        let line_terminals = import_line_terminals_from_file("line_terminals.txt");
        let danger_map = create_danger_map(line_terminals);
        println!("{:?}", danger_map);


    }
}

pub mod latarenfish {
    type Shoal = Vec<usize>;

    fn turn_day(shoal: &mut Shoal) {
        let mut new_generation : Vec<usize> = Vec::new();
        for mut fish in &mut shoal.iter_mut() {
            if *fish == 0 as usize {
                *fish = 6 as usize;
                new_generation.push(8);
            }
            else {
                *fish -= 1;
            }
        }
        shoal.append(&mut new_generation);
    }

    pub fn test() {
        let mut shoal: Shoal = vec![1, 2, 3];
        for i in 1..40 {
            turn_day(&mut shoal);
            println!("{:?}", &shoal);
        }
    }

}

pub mod crabs {
    type CrabSwarm = Vec<usize>;

    fn align_to_position(swarm: &CrabSwarm, destination_pos: usize) -> usize{
        let mut total_fuel: usize = 0;
        for pos in swarm.iter() {
            let fuel = destination_pos.abs_diff(*pos);
            //println!("Move from {}, to {}: {}, fuel.", pos, destination_pos, fuel);
            total_fuel += fuel
        }
        total_fuel
    }

    fn determine_most_economic_position(swarm: &CrabSwarm) -> (usize, usize) {
        // Why do simple arithmetic averange dowe not works here????
        // Brute force... meh :/
        let min = swarm.iter().min().unwrap();
        let max = swarm.iter().max().unwrap();
        let mut costs: Vec<usize> = Vec::new();
        for i in *min..*max {
            costs.push(align_to_position(swarm, i));
        }

        let mut min_i = 0;
        let mut min_v = costs[0];
        for (i, v) in costs.iter().enumerate() {
            if *v < min_v {
                min_v = *v;
                min_i = i;
            }
        }
        (min_i, min_v)
    }

    pub fn test() {
        let swarm: CrabSwarm = vec![16,1,2,0,4,2,7,1,2,14];
        for i in 0..16 {
            let total_fuel = align_to_position(&swarm, i);
            println!("For position: {}, total fuel: {}",i, total_fuel);
        }
        let (i, v) = determine_most_economic_position(&swarm);
        println!("Optimal opsition is: {}, and costs: {}", i, v);
    }
}

fn main() {
    crabs::test();
}
