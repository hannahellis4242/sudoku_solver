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
    pub struct Partial {
        solution: Vec<Option<char>>,
    }
    #[derive(Debug)]
    pub enum Node {
        Partial {
            value: char,
            solution: Partial,
            children: Vec<Node>,
        },
        Done {
            value: char,
            solution: Vec<char>,
        },
        Root {
            children: Vec<Node>,
        },
    }

    mod utils {
        pub fn simplify(xs: &[Option<char>]) -> String {
            xs.iter()
                .map(|x| match x {
                    Some(v) => *v,
                    None => '-',
                }).fold(String::new(),|mut acc,x|{acc.push(x);acc})
        }
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
            parent_solution: &sudoku::Partial,
        ) -> Option<sudoku::Node> {
            use sudoku::rule;
            use sudoku::utils;
            println!("----------create_child----------");
            println!("value:{:?}", &value);
            println!(
                "parent_solution:{:?}",
                utils::simplify(&parent_solution.solution)
            );
            let trial_values = utils::splice(&parent_solution.solution, value);
            println!("trial_values   :{:?}", utils::simplify(&trial_values));
            let broken_rule_count = rule::check_rules(&trial_values, &problem.grid);
            println!("broken_rule_count:{:?}", &broken_rule_count);
            if broken_rule_count == 0 {
                //see if we are done
                let done = trial_values
                    .iter()
                    .filter(|x| x.is_none())
                    .collect::<Vec<_>>()
                    .is_empty();
                println!("done:{:?}", &done);
                if done {
                    let solution = trial_values
                        .iter()
                        .filter_map(|x| *x)
                        .collect::<Vec<char>>();
                    println!("solution : {:?}", solution);
                    println!("Done");
                    println!("==========create_child==========");
                    Some(sudoku::Node::Done {
                        value: value,
                        solution: solution,
                    })
                } else {
                    let partial_solution = sudoku::Partial {
                        solution: trial_values,
                    };
                    let children = create_children(&problem, &partial_solution);

                    //println!("partial_solution : {:?}", partial_solution);
                    println!("number of children : {:?}", children.len());
                    println!("Partial");
                    println!("==========create_child==========");
                    if children.is_empty() {
                        None
                    } else {
                        Some(sudoku::Node::Partial {
                            value: value,
                            solution: partial_solution,
                            children: children,
                        })
                    }
                }
            } else {
                None
            }
        }
        pub fn create_children(
            problem: &sudoku::Problem,
            parent_solution: &sudoku::Partial,
        ) -> Vec<sudoku::Node> {
            problem
                .grid
                .values
                .iter()
                .filter_map(move |&value| create_child(value, &problem, &parent_solution))
                .collect::<Vec<sudoku::Node>>()
        }
    }

    struct IdGenerator {
        curr: u32,
    }

    impl Iterator for IdGenerator {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            self.curr = self.curr + 1;
            Some(self.curr)
        }
    }
    impl IdGenerator {
        fn new() -> IdGenerator {
            IdGenerator { curr: 0 }
        }
    }
    struct Dot {
        generator: IdGenerator,
        ids_and_labels: Vec<(u32, String)>,
        edges: Vec<(u32, u32)>,
    }

    impl Dot {
        fn new() -> Dot {
            Dot {
                generator: IdGenerator::new(),
                ids_and_labels: Vec::new(),
                edges: Vec::new(),
            }
        }
        fn add_node(&mut self, label: &str) -> u32 {
            let id = self.generator.next().unwrap();
            self.ids_and_labels.push((id, label.to_owned()));
            id
        }
        fn add_edge(&mut self, source: u32, target: u32) {
            self.edges.push((source, target));
        }
        fn show(&self, name: &str) -> String {
            fn show_id_and_label((id, label): &(u32, String)) -> String {
                let mut out = id.to_string();
                out += " [label=\"";
                out += label.as_str();
                out += "\"];\n";
                out
            }
            fn show_edge((s, t): &(u32, u32)) -> String {
                let mut out = s.to_string();
                out += " -> ";
                out += t.to_string().as_str();
                out += " ;\n";
                out
            }

            let mut out = "digraph ".to_owned();
            out += name;
            out += " {\n";
            out += "node[shape=record];\n";
            out += self
                .ids_and_labels
                .iter()
                .map(show_id_and_label)
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(x.as_str());
                    acc
                }).as_str();
            out += self
                .edges
                .iter()
                .map(show_edge)
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(x.as_str());
                    acc
                }).as_str();
            out += " }\n";
            out
        }
    }

    impl Node {
        pub fn new(problem: &Problem) -> Node {
            use sudoku;
            use sudoku::helper;
            Node::Root {
                children: helper::create_children(
                    &problem,
                    &sudoku::Partial {
                        solution: problem.values.clone(),
                    },
                ),
            }
        }
        fn dot_label(&self) -> String {
            use sudoku;
            match self {
                sudoku::Node::Root { children: _ } => "{root}".to_owned(),
                sudoku::Node::Done { value, solution } => {
                    let mut out = "{done|".to_owned();
                    out.push(*value);
                    out.push('|');
                    out.push_str("[ ");
                    out.push_str(
                        solution
                            .iter()
                            .fold(String::new(), |mut acc, &x| {
                                if acc.is_empty() {
                                    acc.push(x);
                                    acc
                                } else {
                                    acc.push_str(" , ");
                                    acc.push(x);
                                    acc
                                }
                            }).as_str(),
                    );
                    out.push_str(" ]");
                    out += "}";
                    out
                }
                sudoku::Node::Partial {
                    value,
                    solution: _,
                    children: _,
                } => {
                    let mut out = String::new();
                    out.push(*value);
                    out
                }
            }
        }

        fn visit(&self, d: &mut Dot) -> u32 {
            use sudoku;
            let id = d.add_node(self.dot_label().as_str());
            match self {
                sudoku::Node::Done {
                    value: _,
                    solution: _,
                } => (),
                sudoku::Node::Partial {
                    value: _,
                    solution: _,
                    children,
                } => {
                    children.iter().for_each(|child| {
                        let child_id = child.visit(d);
                        d.add_edge(id, child_id)
                    });
                }
                sudoku::Node::Root { children } => {
                    children.iter().for_each(|child| {
                        let child_id = child.visit(d);
                        d.add_edge(id, child_id)
                    });
                }
            };
            id
        }

        pub fn to_dot(self) -> String {
            let mut dot = Dot::new();
            self.visit(&mut dot);
            dot.show("sudoku")
        }
    }
}

fn main() {
    {
        let g = sudoku::GridInfo {
            height: 9,
            width: 9,
            square: 3,
            values: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'],
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
        let solution = sudoku::Node::new(&sudoku::Problem {
            grid: g,
            values: values,
        });
        println!("{}", solution.to_dot());
    }
}
