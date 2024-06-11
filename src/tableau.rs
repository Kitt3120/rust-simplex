use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::Number;

#[derive(Debug, Error)]
pub enum TableauCreationError {
    #[error("Tableau must have at least two rows, current tableau has {0} rows")]
    NotEnoughRows(usize),
    #[error("Tableau must at least have the x0 and RHS columns")]
    NotEnoughColumns,
    #[error("All rows must have the same number of columns. First row has {0} columns, but row {1} has {2} columns")]
    UnevenColumns(usize, usize, usize),
}

#[derive(Debug, Clone)]
pub struct Tableau<T>
where
    T: Number,
{
    pub rows: Vec<Vec<T>>,
}

impl<T> Tableau<T>
where
    T: Number,
{
    pub fn new(rows: Vec<Vec<T>>) -> Result<Tableau<T>, TableauCreationError> {
        if rows.len() < 2 {
            return Err(TableauCreationError::NotEnoughRows(rows.len()));
        }

        let columns = rows[0].len();
        if columns < 2 {
            return Err(TableauCreationError::NotEnoughColumns);
        }

        for (index, row) in rows.iter().enumerate() {
            if row.len() != columns {
                return Err(TableauCreationError::UnevenColumns(
                    columns,
                    index + 1,
                    row.len(),
                ));
            }
        }

        Ok(Self { rows })
    }

    pub fn apply_all(&mut self, function: impl Fn(T) -> T) {
        for row in &mut self.rows {
            for cell in row {
                let cell_value = *cell;
                *cell = function(cell_value);
            }
        }
    }

    pub fn apply_row(&mut self, row_index: usize, function: impl Fn(T) -> T) {
        for cell in &mut self.rows[row_index] {
            let cell_value = *cell;
            *cell = function(cell_value);
        }
    }

    pub fn apply_column(&mut self, column_index: usize, function: impl Fn(T) -> T) {
        for row in &mut self.rows {
            let cell = &mut row[column_index];
            let cell_value = *cell;
            *cell = function(cell_value);
        }
    }

    fn get_column_width(&self, column_index: usize) -> usize {
        self.rows
            .iter()
            .map(|row| row[column_index].to_string().len())
            .max()
            .unwrap()
    }
}

impl<T> Display for Tableau<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let column_widths: Vec<usize> = (0..self.rows[0].len())
            .map(|column_index| self.get_column_width(column_index))
            .map(|column_width| if column_width < 2 { 2 } else { column_width })
            .collect();

        let total_width = column_widths.iter().sum::<usize>() + column_widths.len() * 3 + 1;

        for (index, column_width) in column_widths.iter().enumerate() {
            match index {
                0 => write!(f, "{:>column_width$} |", "x0")?,
                _ if index == column_widths.len() - 1 => {
                    write!(f, " | {:>column_width$}", "RHS")?;
                    break;
                }
                _ => write!(f, " {:>column_width$} ", format!("x{}", index))?,
            }

            if index < column_widths.len() - 2 {
                write!(f, " ")?;
            }
        }

        writeln!(f)?;

        for (row_index, row) in self.rows.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                let cell_string = cell.to_string();
                let column_width = column_widths[cell_index];

                match cell_index {
                    0 => write!(f, "{:>column_width$} |", cell_string)?,
                    _ if cell_index == row.len() - 1 => {
                        write!(f, "| {:>column_width$}", cell_string)?;
                        break;
                    }
                    _ => write!(f, " {:>column_width$} ", cell_string)?,
                }

                if cell_index < row.len() - 1 {
                    write!(f, " ")?;
                }
            }

            if row_index == 0 {
                writeln!(f)?;
                write!(f, "{:-<total_width$}", "-")?;
            }

            if row_index < self.rows.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
