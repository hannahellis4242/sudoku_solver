extern crate itertools;

mod sudoku {
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

    #[derive(Debug)]
    pub struct PartialSolution {
        trial_values: Vec<char>,
    }

    mod utils {
        use sudoku::Problem;
        use sudoku::PartialSolution;
        fn merge<T>(x: &[Option<T>], y: &[T]) -> Vec<Option<T>>
        where
            T: Clone,
        {
            let mut y_iter = y.iter();
            x.iter()
                .map(|a| match a {
                    &Some(ref v) => Some(v.clone()),
                    &None => y_iter.next().map(|b| b.clone()),
                })
                .collect::<Vec<_>>()
        }

        pub fn create_trial_values(p: &Problem, ps: &PartialSolution) -> Vec<Option<char>> {
            merge(&p.values, &ps.trial_values)
        }

        pub fn find_next<'t, T>(x: &T, xs: &'t [T]) -> Option<&'t T>
        where
            T: PartialEq,
        {
            xs.split_first().and_then(|(y, ys)| if y == x {
                ys.first()
            } else {
                find_next(x, ys)
            })
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
                &Rule::Row(i) => {
                    (0..g.width)
                        .map(|j| (i, j))
                        .collect::<Vec<(usize, usize)>>()
                }
                &Rule::Column(j) => {
                    (0..g.height)
                        .map(|i| (i, j))
                        .collect::<Vec<(usize, usize)>>()
                }
                &Rule::Square(i, j) => {
                    (g.square * j..g.square * (j + 1))
                        .cartesian_product(g.square * i..g.square * (i + 1))
                        .collect::<Vec<(usize, usize)>>()
                }
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
            let indices = flatten_indices(&generate_rule(&r, &gi), &gi);
            let values = indices.iter().filter_map(|i| g[*i]).collect::<Vec<char>>();
            let out = !contains_duplicates(&values);
            //println!("check_rule : {:?} -> {:?}", values, out);
            out
        }

        pub fn check_rules(g: &[Option<char>], gi: &GridInfo) -> bool {
            let row_rules = (0..9).map(|x| Rule::Row(x));
            let column_rules = (0..9).map(|x| Rule::Column(x));
            use itertools::Itertools;
            let square_rules = (0..3).cartesian_product(0..3).map(
                |(x, y)| Rule::Square(x, y),
            );

            let mut rules = row_rules.chain(column_rules).chain(square_rules);
            //              ^ why is this mutable...
            let out = rules.all(|x| check_rule(g, &x, gi));
            //println!("check_rules -> {:?}", out);
            out
        }
    }

    pub fn root() -> PartialSolution {
        PartialSolution { trial_values: Vec::new() }
    }

    fn reject(p: &Problem, c: &PartialSolution) -> bool {
        let trial_values = utils::create_trial_values(p, c);

        let out = !rule::check_rules(&trial_values, &p.grid);
        //println!("reject : {:?} -> {:?}", trial_values, out);
        out
    }

    fn accept(p: &Problem, c: &PartialSolution) -> bool {
        !utils::create_trial_values(p, c).iter().any(|x| x.is_none())
    }

    fn first(p: &Problem, c: &PartialSolution) -> Option<PartialSolution> {
        use std::iter;
        p.grid.values.as_slice().first().map(|f| {
            PartialSolution {
                trial_values: c.trial_values
                    .iter()
                    .chain(iter::once(f))
                    .map(|x| *x)
                    .collect::<Vec<char>>(),
            }
        })
    }

    fn next(p: &Problem, c: &PartialSolution) -> Option<PartialSolution> {
        use std::iter;
        c.trial_values.split_last().and_then(|(x, xs)| {
            utils::find_next(x, p.grid.values.as_slice()).map(|n| {
                PartialSolution {
                    trial_values: xs.iter()
                        .chain(iter::once(n))
                        .map(|n| *n)
                        .collect::<Vec<char>>(),
                }
            })
        })
    }

  /*  fn backtrack_recurse<F>(p: &Problem, c: Option<PartialSolution>, output: F)
    where
        F: Fn(&Problem, &PartialSolution) -> bool,
    {
        match c {
            Some(solution) => {
                backtrack(p, &solution, &output);
                backtrack_recurse(p, next(p, &solution), output)
            }
            None => (),
        }
        
    }*/

    pub fn backtrack<F>(p: &Problem, c: &PartialSolution, output: &F)
    where
        F: Fn(&Problem, &PartialSolution) -> bool,
    {
        //println!("backtrack : {:?}", c.trial_values);
        if reject(p, c) {
            //println!("reject")
        } else {
            if accept(p, c) {
                //println!("accept");
                let should_stop = output(p, c);
                if should_stop {
                    ()
                }
            }
            let mut s= first(p,c);
            while s.is_some(){
            let su = s.unwrap();
                backtrack(p,&su,output);
                s = next(p,&su);
            }
        }
    }

    pub fn output_one(p: &Problem, c: &PartialSolution) -> bool {
        println!("{:?}", c.trial_solution);
        true
    }
}

fn main() {
    let g = sudoku::GridInfo {
        height: 9,
        width: 9,
        square: 3,
        values: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    };
    let values = vec![
        None,
        None,
        Some('2'),
        None,
        None,
        Some('6'),
        None,
        None,
        Some('7'),
        Some('5'),
        Some('7'),
        Some('9'),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some('4'),
        None,
        Some('2'),
        Some('7'),
        None,
        Some('3'),
        None,
        None,
        Some('4'),
        None,
        None,
        Some('3'),
        Some('8'),
        Some('2'),
        None,
        None,
        Some('1'),
        Some('8'),
        Some('5'),
        None,
        Some('9'),
        None,
        Some('4'),
        None,
        Some('3'),
        Some('2'),
        Some('2'),
        None,
        None,
        Some('6'),
        Some('5'),
        Some('7'),
        None,
        None,
        Some('4'),
        None,
        None,
        Some('8'),
        None,
        Some('6'),
        Some('5'),
        None,
        Some('4'),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some('7'),
        Some('8'),
        Some('9'),
        Some('7'),
        None,
        None,
        Some('1'),
        None,
        None,
        Some('2'),
        None,
        None,
    ];
    let p = sudoku::Problem {
        grid: g,
        values: values,
    };

    sudoku::backtrack(&p, &sudoku::root(), &sudoku::output_one);
}
