/*use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Square {
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

    fn as_char(&self) -> char {
        match *self {
            Square::Fix(x) => x,
            Square::Var(x) => x,
            Square::Blank => ' ',
        }
    }
}

struct Vector<T> {
    value: Vec<T>,
}

impl<T> Vector<T> {
    fn new(x: &[T]) -> Vector<T>
    where
        T: Clone,
    {
        Vector::<T> { value: x.to_vec() }
    }
    fn mask(&self, m: &[bool]) -> Vector<T>
    where
        T: Clone,
    {
        let y = self.value
            .iter()
            .zip(m.iter())
            .filter(|&(_, &y)| y)
            .map(|(x, _)| x.clone())
            .collect::<Vec<_>>();
        Vector::<T> { value: y }
    }
}

#[derive(Debug)]
struct Grid<T> {
    value: Vec<Vec<T>>,
    rows: usize,
    columns: usize,
}

impl<T> Grid<T> {
    fn new(x: &[T], n: usize, m: usize) -> Option<Grid<T>>
    where
        T: Clone,
    {
        if x.len() == n * m {
            let v = x.chunks(m)
                .map(|a| a.iter().map(|b| b.clone()).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Some(Grid::<T> {
                value: v,
                rows: n,
                columns: m,
            })
        } else {
            None
        }
    }
    fn vectorise(&self) -> Vector<T>
    where
        T: Clone,
    {
        let x = self.value.concat();
        Vector::new(x.as_slice())
    }
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

fn check_rule(squares: &Vector<Square>, rule: &[bool]) -> bool {
    let y = squares
        .mask(rule)
        .value
        .iter()
        .filter(|&x| *x != Square::Blank)
        .map(|x| x.as_char())
        .collect::<Vec<_>>();
    !contains_duplicates(&y)
}
*/
mod grid {
    fn get_value<'a, T>(
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

    fn create_grid<T, F>(rows: usize, columns: usize, f: F) -> Vec<Vec<T>>
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

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_get_value() {
            let values = [(0, 0, "Hello"), (2, 3, "World")];
            use grid::get_value;
            assert_eq!(get_value(&0, &0, &values, &"Default"), &"Hello");
            assert_eq!(get_value(&2, &3, &values, &"Default"), &"World");
            assert_eq!(get_value(&1, &1, &values, &"Default"), &"Default");
        }
        #[test]
        fn test_create_flat_grid() {
            let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
            use grid::create_flat_grid;
            use grid::get_value;
            assert_eq!(
                create_flat_grid(3, 4, |x, y| get_value(&x, &y, &values, &0).clone()),
                [1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2]
            );
        }
        #[test]
        fn test_create_grid() {
            let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
            use grid::create_grid;
            use grid::get_value;
            assert_eq!(
                create_grid(3, 4, |x, y| get_value(&x, &y, &values, &0).clone()),
                [[1, 0, 0, 0], [0, 0, 3, 0], [0, 0, 0, 2]]
            );
        }
        #[test]
        fn test_flatten_grid() {
            let values = [(0, 0, 1), (2, 3, 2), (1, 2, 3)];
            use grid::create_grid;
            use grid::get_value;
            use grid::flatten_grid;
            assert_eq!(
                flatten_grid(create_grid(3, 4, |x, y| get_value(&x, &y, &values, &0)
                    .clone())),
                [1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2]
            );
        }
    }
}
/*
fn generate_rule(data: &[usize], s: usize) -> Vec<bool> {
    (0..)
        .take(s)
        .map(|x| data.contains(&x))
        .collect::<Vec<bool>>()
}

fn generate_matix_indices((&rows, &columns): (&usize, &usize)) -> Vec<(usize, usize)> {
    (0..)
        .take(rows)
        .flat_map(|x| {
            (0..)
                .take(columns)
                .map(|y| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_matix_indices() {
        //println!("{:?}", (0..).take(5).collect::<Vec<_>>());
        use generate_matix_indices;
        assert_eq!(generate_matix_indices((&0, &0)), Vec::new());
        assert_eq!(generate_matix_indices((&0, &10)), Vec::new());
        assert_eq!(generate_matix_indices((&1, &0)), Vec::new());
        assert_eq!(generate_matix_indices((&1, &2)), [(0, 0), (0, 1)]);
        assert_eq!(
            generate_matix_indices((&2, &2)),
            [(0, 0), (0, 1), (1, 0), (1, 1)]
        );
    }
}

fn square_value(row:usize,column,usize,map:HashMap<(usize,usize),char>)->Square{
    match map.get((row,column)){/
        Some(x)=>Square::fixed(x),
        None=>Square::Blank
    }
}

fn generate_grid<F>(f: F, rows: &usize, columns: &usize) -> Grid<Square>
where
    F: Fn(usize, usize) -> Square,
{
    let values = generate_matix_indices((rows, columns))
        .iter()
        .map(|&(x, y)| f(x, y))
        .collect::<Vec<Square>>();
    Grid::<Square>::new(values.as_slice(), *rows, *columns).unwrap()
}
*/
fn main() {
    /*let x = Vector::new(&[
        Square::fixed('3'),
        Square::variable('2'),
        Square::variable('1'),
        Square::Blank,
        Square::Blank,
        Square::Blank,
    ]);
    println!("{:?}", check_rule(&x, &[true, false, true, true, false]));
    /*{
        let x = Vector::new((0..9).collect::<Vec<_>>().as_slice());
        println!("{:?}", x.value);
        let y = x.mask(&[true, true, false, true, true]);
        println!("{:?}", y.value);
    }

    {
        Grid::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3).map(|x| println!("{:?}", x));
    }

    {
        let x = Vector::new(&[
            Square::fixed('1'),
            Square::variable('2'),
            Square::blank(),
            Square::fixed('4'),
            Square::Var('5'),
            Square::Blank,
            Square::Fix('7'),
            Square::Var('8'),
            Square::Blank,
        ]);
        println!("{:?}", x.value);
        let g = Grid::new(
            x.value
                .iter()
                .map(|x| x.as_char())
                .collect::<Vec<_>>()
                .as_slice(),
            3,
            3,
        );
        //g.map(|x| println!("{:?}", x));

        g.map(|x| x.vectorise()).map(|x| println!("{:?}", x.value));
    }*/*/
}
