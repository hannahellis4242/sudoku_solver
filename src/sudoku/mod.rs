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

mod utils {
    /*pub fn simplify(xs: &[Option<char>]) -> String {
        xs.iter()
            .map(|x| match x {
                Some(v) => *v,
                None => '-',
            }).fold(String::new(), |mut acc, x| {
                acc.push(x);
                acc
            })
    }*/
    pub fn splice<T>(xs: &[Option<T>], y: T) -> Vec<Option<T>>
    where
        T: Clone,
    {
        use std::iter;
        let mut y_iter = iter::once(y);
        xs.iter()
            .map(|x| match x {
                Some(value) => Some((*value).clone()),
                None => y_iter.next(),
            }).collect::<Vec<Option<T>>>()
    }
    pub fn append<T>(xs: &[T], y: &T) -> Vec<T>
    where
        T: Clone,
    {
        use std::iter;
        let y_iter = iter::once(y);
        xs.into_iter().chain(y_iter).cloned().collect::<Vec<T>>()
    }
}

mod rule {
    enum Rule {
        Row(usize),
        Column(usize),
        Square(usize, usize),
    }

    use sudoku::GridInfo;
    fn generate_rule(r: &Rule, g: &GridInfo) -> Vec<(usize, usize)> {
        use itertools::Itertools;
        match r {
            &Rule::Row(i) => (0..g.width)
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>(),
            &Rule::Column(j) => (0..g.height)
                .map(|i| (i, j))
                .collect::<Vec<(usize, usize)>>(),
            &Rule::Square(i, j) => (g.square * j..g.square * (j + 1))
                .cartesian_product(g.square * i..g.square * (i + 1))
                .collect::<Vec<(usize, usize)>>(),
        }
    }

    fn flat_index(x: &usize, y: &usize, g: &GridInfo) -> Option<usize> {
        if *x < g.height && *y < g.width {
            Some(y + g.width * x)
        } else {
            None
        }
    }

    fn flatten_indices(xs: &[(usize, usize)], g: &GridInfo) -> Vec<usize> {
        xs.iter()
            .filter_map(|&(x, y)| flat_index(&x, &y, &g))
            .collect::<Vec<usize>>()
    }

    fn contains_duplicates<T>(x: &[T]) -> bool
    where
        T: PartialEq,
    {
        match x.split_first() {
            Some((y, ys)) => {
                if ys.contains(y) {
                    true
                } else {
                    contains_duplicates(ys)
                }
            }
            None => false,
        }
    }
    fn check_rule(g: &[Option<char>], r: &Rule, gi: &GridInfo) -> bool {
        //returns true if pass false if fail
        //println!("----------check_rule----------");
        let indices = flatten_indices(&generate_rule(&r, &gi), &gi);
        let values = indices.iter().filter_map(|i| g[*i]).collect::<Vec<char>>();
        let out = !contains_duplicates(&values);
        //println!("check_rule : {:?} -> {:?}", values, out);
        //println!("==========check_rule==========");
        out
    }

    pub fn check_rules(g: &[Option<char>], gi: &GridInfo) -> u32 {
        let row_rules = (0..9).map(|x| Rule::Row(x));
        let column_rules = (0..9).map(|x| Rule::Column(x));
        use itertools::Itertools;
        let square_rules = (0..3)
            .cartesian_product(0..3)
            .map(|(x, y)| Rule::Square(x, y));

        let rules = row_rules.chain(column_rules).chain(square_rules);
        let out = rules
            .map(|x| check_rule(g, &x, gi))
            .map(|x| if x { 0 } else { 1 })
            .sum();
        out
    }
}

mod helper {
    use sudoku;
    fn create_child(
        value: char,
        problem: &sudoku::Problem,
        parent_solution: &[Option<char>],
        solutions: &[Vec<char>],
    ) -> Vec<Vec<char>> {
        use sudoku::rule;
        use sudoku::utils;
        /*println!("----------create_child----------");
        println!("value:{:?}", &value);
        println!(
            "parent_solution:{:?}",
            utils::simplify(&parent_solution.solution)
        );*/
        let trial_values = utils::splice(&parent_solution, value);
        //println!("trial_values   :{:?}", utils::simplify(&trial_values));
        let broken_rule_count = rule::check_rules(&trial_values, &problem.grid);
        //println!("broken_rule_count:{:?}", &broken_rule_count);
        if broken_rule_count == 0 {
            //see if we are done
            let done = trial_values
                .iter()
                .filter(|x| x.is_none())
                .collect::<Vec<_>>()
                .is_empty();
            //println!("done:{:?}", &done);
            if done {
                let solution = trial_values
                    .iter()
                    .filter_map(|x| *x)
                    .collect::<Vec<char>>();
                //println!("solution : {:?}", solution);
                //println!("Done");
                //println!("==========create_child==========");

                utils::append(&solutions, &solution)
            } else {
                let children = create_children(&problem, &trial_values, &solutions);
                //println!("partial_solution : {:?}", partial_solution);
                //println!("number of children : {:?}", children.len());
                //println!("Partial");
                //println!("==========create_child==========");
                children.to_vec()
            }
        } else {
            solutions.to_vec()
        }
    }
    pub fn create_children(
        problem: &sudoku::Problem,
        parent_solution: &[Option<char>],
        solutions: &[Vec<char>],
    ) -> Vec<Vec<char>> {
        problem
            .grid
            .values
            .iter()
            .flat_map(move |&value| create_child(value, &problem, &parent_solution, &solutions))
            .collect::<Vec<Vec<char>>>()
    }
}
pub fn solve(problem: &Problem) -> Vec<Vec<char>> {
    use sudoku::helper;
    helper::create_children(
        &problem,
        &problem.values,
        &[],
    )
}
