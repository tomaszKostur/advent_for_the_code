
pub mod sonar{
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
            println!("Submarine position: distance {}, depth {}", self.horizontal, self.depth);
        }
    }
}


fn main() {
    let mut submarine = submarine_body::Position::new();
    submarine.down(10);
    submarine.up(2);
    submarine.forward(7);
    submarine.print_pos();
}
