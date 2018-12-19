pub struct GridInfo {
    pub width: usize,
    pub height: usize,
    pub square: usize,
    pub values: Vec<char>,
}

impl Clone for GridInfo {
    fn clone(&self) -> GridInfo {
        GridInfo {
            width: self.width,
            height: self.height,
            square: self.square,
            values: self.values.clone(),
        }
    }
}
pub struct Problem {
    pub grid: GridInfo,
    pub values: Vec<Option<char>>,
}
impl Clone for Problem {
    fn clone(&self) -> Problem {
        Problem {
            grid: self.grid.clone(),
            values: self.values.clone(),
        }
    }
}

mod helper;
pub mod json;
mod rule;
mod utils;
pub fn solve(problem: Problem) -> Vec<Vec<char>> {
    use sudoku::helper;
    helper::create_children(&problem, &problem.values, &[])
}
