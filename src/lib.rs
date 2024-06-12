use core::f64;
use std::fmt::{Display, Formatter};

use tableau::Tableau;

pub mod tableau;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub enum OptimizeResult {
    Optimal,
    MultipleOptimal,
    Unbounded,
}

#[derive(Debug)]
pub enum FindPivotElementResult {
    Found(Point),
    Optimal,
    Unbounded,
}

#[derive(Debug)]
pub enum TableauVectorVariable {
    Basic(f64),
    NonBasic(f64),
}

impl Display for TableauVectorVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableauVectorVariable::Basic(value) => write!(f, "BV({})", value),
            TableauVectorVariable::NonBasic(value) => write!(f, "NBV({})", value),
        }
    }
}

pub fn find_pivot_column(row: &[f64]) -> Option<usize> {
    let row_min_value = row.iter().fold(f64::INFINITY, |current_value, next_value| {
        if *next_value < current_value {
            *next_value
        } else {
            current_value
        }
    });

    if row_min_value >= 0.0 {
        return None;
    }

    let pivot_column = row
        .iter()
        .position(|&value| value == row_min_value)
        .unwrap(); // Value is guaranteed to be found

    Some(pivot_column)
}

pub fn find_pivot_row(pivot_column: &[f64], rhs_column: &[f64]) -> Option<usize> {
    let pivot_column_max_value =
        pivot_column
            .iter()
            .fold(f64::NEG_INFINITY, |current_value, next_value| {
                if *next_value > current_value {
                    *next_value
                } else {
                    current_value
                }
            });

    if pivot_column_max_value <= 0.0 {
        return None;
    }

    let quotients = pivot_column
        .iter()
        .zip(rhs_column)
        .map(|(pivot_column_value, rhs_value)| {
            if *pivot_column_value > 0.0 {
                rhs_value / pivot_column_value
            } else {
                f64::INFINITY
            }
        })
        .collect::<Vec<f64>>();

    let quotients_min_value = quotients
        .iter()
        .fold(f64::INFINITY, |current_value, next_value| {
            if *next_value < current_value {
                *next_value
            } else {
                current_value
            }
        });

    let pivot_row = quotients
        .iter()
        .position(|&value| value == quotients_min_value)
        .unwrap(); // Value is guaranteed to be found

    Some(pivot_row)
}

pub fn find_pivot_element(tableau: &Tableau) -> FindPivotElementResult {
    let target_row_without_x0_rhs = &tableau.rows[0]
        .iter()
        .skip(1) // Skip x0 column
        .take(tableau.rows[0].len() - 2) // Cut off RHS column
        .copied()
        .collect::<Vec<f64>>();

    let pivot_column_index = match find_pivot_column(target_row_without_x0_rhs) {
        Some(pivot_column) => pivot_column,
        None => return FindPivotElementResult::Optimal,
    };

    let pivot_column_index = pivot_column_index + 1; // Make up for the x0 column skip

    let pivot_column = tableau
        .rows
        .iter()
        .skip(1) // Skip target row
        .map(|row| row[pivot_column_index])
        .collect::<Vec<f64>>();

    let rhs_column = tableau
        .rows
        .iter()
        .skip(1) // Skip target row
        .map(|row| *row.last().unwrap())
        .collect::<Vec<f64>>();

    let pivot_row_index = match find_pivot_row(&pivot_column, &rhs_column) {
        Some(pivot_row) => pivot_row,
        None => return FindPivotElementResult::Unbounded,
    };

    let pivot_row_index = pivot_row_index + 1; // Make up for the target row skip

    let pivot_point = Point::new(pivot_column_index, pivot_row_index);
    FindPivotElementResult::Found(pivot_point)
}

pub fn get_vector(tableau: &Tableau) -> Vec<TableauVectorVariable> {
    let mut vector = vec![];

    for x in 0..tableau.rows[0].len() - 1 {
        let column_values = tableau.rows.iter().map(|row| row[x]).collect::<Vec<f64>>();
        let accumulated_value = column_values.iter().fold(0.0, |acc, &value| acc + value);
        let is_basic = accumulated_value == 1.0;

        if !is_basic {
            vector.push(TableauVectorVariable::NonBasic(0.0));
        } else {
            let row_index = column_values
                .iter()
                .position(|&value| value == 1.0)
                .unwrap(); // Value is guaranteed to be found

            let rhs_value = tableau.rows[row_index][tableau.rows[0].len() - 1];

            vector.push(TableauVectorVariable::Basic(rhs_value));
        }
    }

    vector
}

pub fn pivot(tableau: &mut Tableau, pivot_element: &Point) {
    let pivot_element_value = tableau.rows[pivot_element.y][pivot_element.x];
    tableau.rows[pivot_element.y] = tableau.rows[pivot_element.y]
        .iter()
        .map(|value| *value / pivot_element_value)
        .collect();

    for y in 0..tableau.rows.len() {
        if y == pivot_element.y {
            continue;
        }

        let factor = tableau.rows[y][pivot_element.x];

        for x in 0..tableau.rows[y].len() {
            tableau.rows[y][x] -= factor * tableau.rows[pivot_element.y][x];
        }
    }
}

pub fn optimize(tableau: Tableau) -> (OptimizeResult, Vec<Tableau>) {
    let mut tableaus = vec![tableau];

    loop {
        let last_tableau = tableaus.last().unwrap();

        let pivot_element = match find_pivot_element(last_tableau) {
            FindPivotElementResult::Found(pivot_element) => pivot_element,
            FindPivotElementResult::Unbounded => return (OptimizeResult::Unbounded, tableaus),
            FindPivotElementResult::Optimal => {
                let target_row = &last_tableau.rows[0];
                let vector = get_vector(last_tableau);

                let nbv_indexes_with_target_row_values = vector
                    .iter()
                    .enumerate()
                    .filter_map(|(index, variable)| match variable {
                        TableauVectorVariable::NonBasic(_) => Some((index, target_row[index])),
                        _ => None,
                    })
                    .collect::<Vec<(usize, f64)>>();

                let is_degenerate = nbv_indexes_with_target_row_values
                    .iter()
                    .any(|(_, value)| *value == 0.0);

                if !is_degenerate {
                    return (OptimizeResult::Optimal, tableaus);
                }

                let pivot_column_index = nbv_indexes_with_target_row_values
                    .iter()
                    .find(|(_, value)| *value == 0.0)
                    .unwrap() // Value is guaranteed to be found
                    .0;

                let pivot_column = last_tableau
                    .rows
                    .iter()
                    .skip(1) // Skip target row
                    .map(|row| row[pivot_column_index])
                    .collect::<Vec<f64>>();

                let rhs_column = last_tableau
                    .rows
                    .iter()
                    .skip(1) // Skip target row
                    .map(|row| *row.last().unwrap())
                    .collect::<Vec<f64>>();

                let pivot_row = find_pivot_row(&pivot_column, &rhs_column).unwrap(); // Value is guaranteed to be found
                let pivot_row = pivot_row + 1; // Make up for the target row skip

                let pivot_element = Point::new(pivot_column_index, pivot_row);

                let mut next_tableau = last_tableau.clone();
                pivot(&mut next_tableau, &pivot_element);

                tableaus.push(next_tableau);
                return (OptimizeResult::MultipleOptimal, tableaus);
            }
        };

        let mut next_tableau = last_tableau.clone();
        pivot(&mut next_tableau, &pivot_element);

        tableaus.push(next_tableau);
    }
}
