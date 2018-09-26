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
