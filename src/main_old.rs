extern crate itertools;

mod sudoku {
    /*mod legacy {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Square {
            Fix(char),
            Var(char),
            Blank,
        }

        impl Square {
            fn fixed(x: char) -> Square {
                Square::Fix(x)
            }
            fn variable(x: char) -> Square {
                Square::Var(x)
            }
            fn blank() -> Square {
                Square::Blank
            }

            fn is_blank(&self) -> bool {
                match *self {
                    Square::Blank => true,
                    _ => false,
                }
            }

            fn as_char(&self) -> char {
                match *self {
                    Square::Fix(x) => x,
                    Square::Var(x) => x,
                    Square::Blank => ' ',
                }
            }
        }

        mod grid {
            pub fn get_value<'a, T>(
                row: &usize,
                column: &usize,
                values: &'a [(usize, usize, T)],
                d: &'a T,
            ) -> &'a T {
                let found = values
                    .iter()
                    .filter_map(|&(x, y, ref z)| {
                        if x == *row && y == *column {
                            Some(z.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if found.is_empty() {
                    d
                } else {
                    &found[0]
                }
            }

            fn create_flat_grid<T, F>(rows: usize, columns: usize, f: F) -> Vec<T>
            where
                T: Clone,
                F: Fn(usize, usize) -> T,
            {
                (0..)
                    .take(rows)
                    .flat_map(|x| {
                        (0..)
                            .take(columns)
                            .map(|y| f(x, y).clone())
                            .collect::<Vec<T>>()
                    })
                    .collect::<Vec<T>>()
            }

            pub fn create_grid<T, F>(rows: usize, columns: usize, f: F) -> Vec<Vec<T>>
            where
                T: Clone,
                F: Fn(usize, usize) -> T,
            {
                (0..)
                    .take(rows)
                    .map(|x| {
                        (0..)
                            .take(columns)
                            .map(|y| f(x, y).clone())
                            .collect::<Vec<T>>()
                    })
                    .collect::<Vec<Vec<T>>>()
            }

            fn flatten_grid<T>(grid: Vec<Vec<T>>) -> Vec<T>
            where
                T: Clone,
            {
                grid.concat()
            }

            pub fn get_grid_value<T>(row: usize, column: usize, grid: &Vec<Vec<T>>) -> Option<T>
            where
                T: Clone,
            {
                let grid_values = grid.iter()
                    .enumerate()
                    .flat_map(|(row_index, row_values)| {
                        row_values
                            .iter()
                            .enumerate()
                            .map(|(column_index, value)| (row_index, column_index, value))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let found = grid_values
                    .iter()
                    .filter_map(|&(x, y, v)| {
                        if x == row && y == column {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if found.is_empty() {
                    None
                } else {
                    Some(found[0].clone())
                }
            }

            #[cfg(test)]
            mod tests {
                #[test]
                fn test_get_value() {
                    let values = [(0, 0, "Hello"), (2, 3, "World")];
                    use sudoku::grid::get_value;
                    assert_eq!(get_value(&0, &0, &values, &"Default"), &"Hello");
                    assert_eq!(get_value(&2, &3, &values, &"Default"), &"World");
                    assert_eq!(get_value(&1, &1, &values, &"Default"), &"Default");
                }
                #[test]
                fn test_create_flat_grid() {
                    let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
                    use sudoku::grid::create_flat_grid;
                    use sudoku::grid::get_value;
                    assert_eq!(
                        create_flat_grid(3, 4, |x, y| get_value(&x, &y, &values, &0).clone()),
                        [1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2]
                    );
                }
                #[test]
                fn test_create_grid() {
                    let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
                    use sudoku::grid::create_grid;
                    use sudoku::grid::get_value;
                    assert_eq!(
                        create_grid(3, 4, |x, y| get_value(&x, &y, &values, &0).clone()),
                        [[1, 0, 0, 0], [0, 0, 3, 0], [0, 0, 0, 2]]
                    );
                }
                #[test]
                fn test_flatten_grid() {
                    let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
                    use sudoku::grid::create_grid;
                    use sudoku::grid::get_value;
                    use sudoku::grid::flatten_grid;
                    assert_eq!(
                        flatten_grid(create_grid(3, 4, |x, y| get_value(&x, &y, &values, &0)
                            .clone())),
                        [1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2]
                    );
                }
                #[test]
                fn test_get_grid_value() {
                    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
                    use sudoku::grid::get_grid_value;
                    assert_eq!(get_grid_value(0, 0, &grid), Some(1));
                    assert_eq!(get_grid_value(100, 100, &grid), None);
                }
            }
        }

        mod rule {
            fn mask<T>(grid_values: Vec<Vec<T>>, indices: &[(usize, usize)]) -> Vec<T>
            where
                T: Clone,
            {
                use sudoku::grid::get_grid_value;
                indices
                    .iter()
                    .filter_map(|&(x, y)| get_grid_value(x, y, &grid_values))
                    .collect::<Vec<T>>()
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
            use sudoku::Square;
            pub fn check_rule(g: Vec<Vec<Square>>, rule: &[(usize, usize)]) -> bool {
                let values = mask(g, rule);
                return !contains_duplicates(&values);
            }

            pub enum RuleType {
                Row(usize),
                Column(usize),
                Square(usize, usize),
            }

            pub fn generate_rule(rule_type: RuleType) -> Vec<(usize, usize)> {
                use itertools::Itertools;
                match rule_type {
                    RuleType::Row(x) => (0..9).map(|y| (x, y)).collect::<Vec<(usize, usize)>>(),
                    RuleType::Column(y) => (0..9).map(|x| (x, y)).collect::<Vec<(usize, usize)>>(),
                    RuleType::Square(x, y) => (3 * x..(3 * (x + 1)))
                        .cartesian_product((3 * y..(3 * (y + 1))))
                        .collect::<Vec<(usize, usize)>>(),
                }
            }
            #[cfg(test)]
            mod tests {
                #[test]
                fn test_mask() {
                    use sudoku::rule::mask;
                    let values = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
                    assert_eq!(mask(values, &[(0, 0), (1, 2), (5, 5)]), [1, 7]);
                }
                #[test]
                fn test_contains_duplicates() {
                    use sudoku::rule::contains_duplicates;
                    {
                        //empty gives false
                        let values: Vec<i32> = Vec::new();
                        assert_eq!(contains_duplicates(&values), false);
                    }
                    {
                        let values = vec![1, 2, 3, 4, 5, 6];
                        assert_eq!(contains_duplicates(&values), false);
                    }
                    {
                        let values = vec![1, 1, 2, 3, 4, 5];
                        assert_eq!(contains_duplicates(&values), true);
                    }
                    {
                        let values = vec![1, 2, 3, 4, 5, 5];
                        assert_eq!(contains_duplicates(&values), true);
                    }
                    {
                        let values = vec![1, 2, 3, 1, 4, 5];
                        assert_eq!(contains_duplicates(&values), true);
                    }
                }
                #[test]
                fn test_generate_rule() {
                    use sudoku::rule::generate_rule;
                    use sudoku::rule::RuleType;
                    {
                        assert_eq!(
                            generate_rule(RuleType::Row(0)),
                            [
                                (0, 0),
                                (0, 1),
                                (0, 2),
                                (0, 3),
                                (0, 4),
                                (0, 5),
                                (0, 6),
                                (0, 7),
                                (0, 8)
                            ]
                        );
                        assert_eq!(
                            generate_rule(RuleType::Row(1)),
                            [
                                (1, 0),
                                (1, 1),
                                (1, 2),
                                (1, 3),
                                (1, 4),
                                (1, 5),
                                (1, 6),
                                (1, 7),
                                (1, 8)
                            ]
                        );
                        assert_eq!(
                            generate_rule(RuleType::Row(2)),
                            [
                                (2, 0),
                                (2, 1),
                                (2, 2),
                                (2, 3),
                                (2, 4),
                                (2, 5),
                                (2, 6),
                                (2, 7),
                                (2, 8)
                            ]
                        );
                        assert_eq!(
                            generate_rule(RuleType::Row(3)),
                            [
                                (3, 0),
                                (3, 1),
                                (3, 2),
                                (3, 3),
                                (3, 4),
                                (3, 5),
                                (3, 6),
                                (3, 7),
                                (3, 8)
                            ]
                        );
                        assert_eq!(
                            generate_rule(RuleType::Row(4)),
                            [
                                (4, 0),
                                (4, 1),
                                (4, 2),
                                (4, 3),
                                (4, 4),
                                (4, 5),
                                (4, 6),
                                (4, 7),
                                (4, 8)
                            ]
                        );
                    }
                    {
                        assert_eq!(
                            generate_rule(RuleType::Column(0)),
                            [
                                (0, 0),
                                (1, 0),
                                (2, 0),
                                (3, 0),
                                (4, 0),
                                (5, 0),
                                (6, 0),
                                (7, 0),
                                (8, 0)
                            ]
                        );
                        assert_eq!(
                            generate_rule(RuleType::Column(5)),
                            [
                                (0, 5),
                                (1, 5),
                                (2, 5),
                                (3, 5),
                                (4, 5),
                                (5, 5),
                                (6, 5),
                                (7, 5),
                                (8, 5)
                            ]
                        );
                    }
                    {
                        assert_eq!(
                            generate_rule(RuleType::Square(0, 0)),
                            [
                                (0, 0),
                                (0, 1),
                                (0, 2),
                                (1, 0),
                                (1, 1),
                                (1, 2),
                                (2, 0),
                                (2, 1),
                                (2, 2)
                            ]
                        );
                    }
                }
            }
        }

    }*/
    /*
    struct Problem {
        known_grid_values: Vec<(usize, usize, char)>,
        unknown_grid_points: Vec<(usize, usize)>,
    }

    struct PartialSolution {
        values: Vec<(usize, usize, Option<char>)>,
    }

    mod detail {
        pub fn unused(x: &[(usize, usize, char)]) -> Vec<(usize, usize)> {
            use itertools::Itertools;
            let y = x.iter()
                .map(|&(i, j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>();
            (0..9)
                .cartesian_product(0..9)
                .filter(|z| !y.contains(z))
                .collect::<Vec<(usize, usize)>>()
        }
    }

    impl Problem {
        fn new(values: &[(usize, usize, char)]) -> Problem {
            Problem {
                known_grid_values: values.to_vec(),
                unknown_grid_points: detail::unused(values),
            }
        }
    }

    fn root(problem: &Problem) -> PartialSolution {
        PartialSolution {
            values: problem
                .unknown_grid_points
                .iter()
                .map(|&(i, j)| (i, j, None))
                .collect::<Vec<_>>(),
        }
    }

    mod grid {
        use std::collections::HashMap;
        fn create_grid_map(
            use sudoku::Problem;
            use sudoku::PartialSolution;
            problem: &Problem,
            trial: &PartialSolution,
        ) -> HashMap<(usize, usize), char> {
            let all = problem
                .known_grid_values
                .iter()
                .chain(trial.filter_map(|(i, j, v)| v.map(|x| (i, j, x))));
            println!("{:?}", all.collect::<Vec<_>>());
            HashMap::new()
        }
        #[cfg(test)]
        mod tests {
            #[test]
            fn test_create_grid_map() {
                let problem = Problem::new(&[(0, 0, '1')]);
                let trial = root(problem);
                grid::create_grid_map(problem, trial);
            }
        }
    }

    /*fn show(g: &Vec<Vec<Square>>) -> String {
        let show_element = |index: usize, element: &Square| {
            if index % 3 == 0 && index != 0 {
                format!("| {} ", element.as_char())
            } else {
                format!("{} ", element.as_char())
            }
        };

        let show_row = |row: &Vec<Square>| {
            row.iter()
                .enumerate()
                .map(|(i, x)| show_element(i, &x))
                .fold(String::new(), |acc, x| format!("{}{}", acc, x))
        };

        g.iter()
            .enumerate()
            .map(|(index, row)| {
                if index % 3 == 0 && index != 0 {
                    format!("------+-------+------\n{}", show_row(row))
                } else {
                    format!("{}", show_row(row))
                }
            })
            .fold(String::new(), |acc, x| format!("{}\n{}", acc, x))
    }*/

    /*fn reject(problem: Problem, trial: PartialSolution) -> bool {
        use sudoku::rule::check_rule;
        use sudoku::rule::generate_rule;
        use sudoku::rule::RuleType;
        let row_rules = (0..9).map(|x| RuleType::Row(x));
        let column_rules = (0..9).map(|x| RuleType::Column(x));
        use itertools::Itertools;
        let square_rules = (0..3)
            .cartesian_product((0..3))
            .map(|(x, y)| RuleType::Square(x, y));

        let rules = row_rules.chain(column_rules).chain(square_rules);

        !rules
            .map(|x| generate_rule(x))
            .all(|x| check_rule(g.clone(), &x))
    }*/
    /*
    fn accept(g: &Vec<Vec<Square>>) -> bool {
        !g.iter().any(|r| r.iter().any(|c| c.is_blank()))
    }

    /*fn first(g: &Vec<Vec<Square>>) -> Vec<Vec<Square>> {
        //give back the grid, except that the first Blank is replaced by a
        //variable '1'
    }*/
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_root() {
            use sudoku::root;
            use sudoku::show;
            let values = [
                (0, 0, '9'),
                (0, 3, '7'),
                (1, 2, '7'),
                (1, 3, '1'),
                (1, 4, '9'),
                (1, 6, '4'),
                (1, 7, '6'),
                (1, 8, '2'),
                (2, 0, '6'),
                (2, 1, '1'),
                (2, 3, '2'),
                (2, 6, '9'),
                (2, 7, '7'),
                (2, 8, '3'),
                (3, 0, '2'),
                (3, 6, '3'),
                (3, 7, '8'),
                (3, 8, '7'),
                (4, 2, '8'),
                (4, 3, '3'),
                (4, 5, '2'),
                (4, 8, '6'),
                (5, 0, '4'),
                (5, 1, '7'),
                (5, 2, '3'),
                (5, 3, '8'),
                (5, 4, '5'),
                (5, 5, '6'),
                (5, 6, '1'),
                (5, 7, '2'),
                (6, 1, '6'),
                (6, 2, '4'),
                (6, 5, '7'),
                (6, 6, '2'),
                (6, 7, '9'),
                (6, 8, '1'),
                (7, 4, '8'),
                (7, 5, '1'),
                (8, 1, '3'),
                (8, 2, '1'),
                (8, 3, '9'),
                (8, 7, '5'),
                (8, 8, '8'),
            ];
            let g = root(&values);
            println!("{}", show(&g));
            use sudoku::Square::Fix;
            use sudoku::Square::Blank;
            assert_eq!(
                g,
                /*vec![
                    vec![Fix('9'), Blank, Blank, Fix('7')],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ]*/
                [
                    [
                        Fix('9'),
                        Blank,
                        Blank,
                        Fix('7'),
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Blank
                    ],
                    [
                        Blank,
                        Blank,
                        Fix('7'),
                        Fix('1'),
                        Fix('9'),
                        Blank,
                        Fix('4'),
                        Fix('6'),
                        Fix('2')
                    ],
                    [
                        Fix('6'),
                        Fix('1'),
                        Blank,
                        Fix('2'),
                        Blank,
                        Blank,
                        Fix('9'),
                        Fix('7'),
                        Fix('3')
                    ],
                    [
                        Fix('2'),
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Fix('3'),
                        Fix('8'),
                        Fix('7')
                    ],
                    [
                        Blank,
                        Blank,
                        Fix('8'),
                        Fix('3'),
                        Blank,
                        Fix('2'),
                        Blank,
                        Blank,
                        Fix('6')
                    ],
                    [
                        Fix('4'),
                        Fix('7'),
                        Fix('3'),
                        Fix('8'),
                        Fix('5'),
                        Fix('6'),
                        Fix('1'),
                        Fix('2'),
                        Blank
                    ],
                    [
                        Blank,
                        Fix('6'),
                        Fix('4'),
                        Blank,
                        Blank,
                        Fix('7'),
                        Fix('2'),
                        Fix('9'),
                        Fix('1')
                    ],
                    [
                        Blank,
                        Blank,
                        Blank,
                        Blank,
                        Fix('8'),
                        Fix('1'),
                        Blank,
                        Blank,
                        Blank
                    ],
                    [
                        Blank,
                        Fix('3'),
                        Fix('1'),
                        Fix('9'),
                        Blank,
                        Blank,
                        Blank,
                        Fix('5'),
                        Fix('8')
                    ]
                ]
            );
        }
        #[test]
        fn test_reject() {
            /*use sudoku::reject;
            use sudoku::grid::create_grid;
            use sudoku::grid::get_value;
            use sudoku::Square;

            let values = ((0, 0, 1), (0, 1, 2), (0, 2, 3), (0, 3, 4), (0, 4, 4))
                .iter()
                .map(|(x, y, z)| (x, y, Square::Fixed(z)))
                .collect::<Vec<_>>();

            let g = create_grid(9, 9, |x, y| {
                get_value(&x, &y, &values, &Square::Blank).clone()
            });*/
        }
    }*/
    */

}

fn main() {}
