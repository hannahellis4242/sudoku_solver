pub struct GridInfo {
    pub width: usize,
    pub height: usize,
    pub square: usize,
    pub values: Vec<char>,
}

pub struct Problem {
    pub grid: GridInfo,
    pub values: Vec<Option<char>>,
}
mod helper;
mod rule;
mod utils;
pub fn solve(problem: &Problem) -> Vec<Vec<char>> {
    use sudoku::helper;
    helper::create_children(&problem, &problem.values, &[])
}
