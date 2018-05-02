extern crate itertools;

mod sudoku {
    struct GridInfo {
        width: usize,
        height: usize,
        square: usize,
        values: Vec<char>,
    }

    struct Problem {
        grid: GridInfo,
        values: Vec<Option<char>>,
    }

    struct PartialSolution {
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
            xs.split_first().and_then(|(y, ys)| {
                if y == x {
                    ys.first()
                } else {
                    find_next(x, ys)
                }
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
            let indices = flatten_indices(&generate_rule(&r, &gi), &gi);
            let values = indices.iter().filter_map(|i| g[*i]).collect::<Vec<char>>();
            !contains_duplicates(&values)
        }

        pub fn check_rules(g: &[Option<char>], gi: &GridInfo) -> bool {
            let row_rules = (0..9).map(|x| Rule::Row(x));
            let column_rules = (0..9).map(|x| Rule::Column(x));
            use itertools::Itertools;
            let square_rules = (0..3)
                .cartesian_product((0..3))
                .map(|(x, y)| Rule::Square(x, y));

            let rules = row_rules.chain(column_rules).chain(square_rules);

            !rules.all(|x| check_rule(g, &x, gi))
        }
    }

    fn root(p: &Problem) -> PartialSolution {
        PartialSolution {
            trial_values: Vec::new(),
        }
    }

    fn reject(p: &Problem, c: &PartialSolution) -> bool {
        let trial_values = utils::create_trial_values(p, c);
        !rule::check_rules(&trial_values, &p.grid)
    }

    fn accept(p: &Problem, c: &PartialSolution) -> bool {
        !utils::create_trial_values(p, c).iter().any(|x| x.is_none())
    }

    fn first(p: &Problem, c: &PartialSolution) -> Option<PartialSolution> {
        use std::iter;
        p.grid.values.as_slice().first().map(|f| PartialSolution {
            trial_values: c.trial_values
                .iter()
                .chain(iter::once(f))
                .map(|x| *x)
                .collect::<Vec<char>>(),
        })
    }

    fn next(p: &Problem, c: &PartialSolution) -> Option<PartialSolution> {
        use std::iter;
        PartialSolution {
            trial_values: c.trial_values
                .split_last()
                .and_then(|(x, xs)| {
                    utils::find_next(x, p.grid.values)
                    .map(|n|xs.iter().chain(iter::once(n)).collect::<Vec<char>>())
                    })
    }
}

fn main() {}
