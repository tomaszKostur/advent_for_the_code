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
            if occurences_of_one[i] >= reaport.len()/2 {
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

fn main() {
    diagnostics::test();
}
