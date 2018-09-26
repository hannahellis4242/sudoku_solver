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
