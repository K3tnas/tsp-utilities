use std::{fs, io};
use tsplib::NodeCoord;
pub mod tsp_problem;
pub use tsp_problem::TspProblem;

pub mod tour;
pub use tour::Tour;

pub mod local_search;
pub use local_search::*;

pub mod hemisphere;
pub use hemisphere::Hemisphere;

pub trait TspUtilities {
    fn get_region_name(&self) -> Option<String>;
    fn get_2d_points(&self) -> Option<Vec<(f64, f64)>>;
}

impl TspUtilities for tsplib::Instance {
    fn get_region_name(&self) -> Option<String> {
        self.comment[0]
            .split_once(" in ")
            .map(|(_, region)| String::from(region).replace(" ", "_"))
    }

    fn get_2d_points(&self) -> Option<Vec<(f64, f64)>> {
        let Some(NodeCoord::Two(cities_with_idx)) = &self.node_coord else {
            return None;
        };

        Some(
            cities_with_idx
                .iter()
                .map(|(_, altitude, latitude)| ((*altitude as f64), (*latitude as f64)))
                .collect(),
        )
    }
}

pub fn get_instances(path: &str) -> io::Result<Vec<tsplib::Instance>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && let Some("tsp") = path.extension().and_then(|ext| ext.to_str())
        {
            files.push(tsplib::read(&path)?);
        }
    }

    Ok(files)
}

pub fn compute_length(permutation: &[usize], problem: &TspProblem) -> f64 {
    let n = permutation.len();
    let mut distance = 0.0;

    for i in 0..n - 1 {
        distance += problem.dist_matrix[permutation[i]][permutation[i + 1]];
    }

    distance += problem.dist_matrix[permutation[n - 1]][permutation[0]];
    distance
}
