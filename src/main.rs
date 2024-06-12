use simplex::{
    optimize,
    tableau::Tableau,
    OptimizeResult::{MultipleOptimal, Optimal, Unbounded},
};

fn main() {
    let tableau = match Tableau::new(vec![
        vec![1.0, -5.0, -6.0, 0.0, 0.0, 0.0, -7.0],
        vec![0.0, 10.0, 10.0, 1.0, 0.0, 0.0, 40.0],
        vec![0.0, 10.0, 20.0, 0.0, 1.0, 0.0, 60.0],
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 3.0],
    ]) {
        Ok(tableau) => tableau,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let (result, tableaus) = optimize(tableau);

    for (index, tableau) in tableaus.iter().enumerate() {
        println!("Tableau {}:\n{}", (index + 1), tableau);
        println!()
    }

    print!("Status: ");
    match result {
        Optimal => println!("Optimal"),
        MultipleOptimal => println!("Multiple optimal solutions. Check out both last tableaus."),
        Unbounded => println!("Unbounded"),
    }
}
