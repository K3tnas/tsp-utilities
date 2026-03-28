use std::path::PathBuf;
use std::{fs, io};

use plotters::style::Color;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    series::LineSeries,
    style::{BLUE, RED, WHITE},
};
use rand::seq::SliceRandom;
use tsplib::NodeCoord;

pub struct Tour<'a> {
    pub cities: &'a [(f32, f32)],
    pub permutation: Vec<usize>,
    pub length: f32,
}

pub trait TspUtilities {
    fn get_region_name(&self) -> Option<String>;
    fn get_2d_points(&self) -> Option<Vec<(f32, f32)>>;
}

impl<'a> Tour<'a> {
    pub fn new_rand_tour<R: rand::Rng>(cities: &'a [(f32, f32)], rng: &mut R) -> Self {
        let mut permutation: Vec<usize> = (0..cities.len()).collect();
        permutation.shuffle(rng);
        let length = compute_length(&permutation, cities);

        Self {
            cities,
            permutation,
            length,
        }
    }

    pub fn plot_tour(
        &self,
        dirpath: &str,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.cities.is_empty() {
            return Err("No cities to plot".into());
        }

        std::fs::create_dir_all(dirpath)?;

        let (mut x_min, mut y_min) = self.cities[0];
        let (mut x_max, mut y_max) = self.cities[0];

        for &(x, y) in self.cities.iter() {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }

        let margin_x = (x_max - x_min) * 0.05;
        let margin_y = (y_max - y_min) * 0.05;

        let x_range = (x_min as f64 - margin_x as f64)..(x_max as f64 + margin_x as f64);
        let y_range = (y_min as f64 - margin_y as f64)..(y_max as f64 + margin_y as f64);

        let filepath = PathBuf::from(dirpath).join(&format!("{filename}.png"));

        let root = BitMapBackend::new(&filepath, (1200, 800)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(filename, ("sans-serif", 30))
            .build_cartesian_2d(x_range, y_range)?;


        chart.draw_series(
            self.cities
                .iter()
                .map(|(x, y)| Circle::new((*x as f64, *y as f64), 4, RED.filled())),
        )?;

        let mut path: Vec<(f64, f64)> = self
            .permutation
            .iter()
            .map(|&idx| {
                let (x, y) = self.cities[idx];
                (x as f64, y as f64)
            })
            .collect();

        if let Some(&first) = path.first() {
            path.push(first);
        }

        chart.draw_series(LineSeries::new(path, &BLUE))?;

        root.present()?;

        Ok(())
    }
}

impl TspUtilities for tsplib::Instance {
    fn get_region_name(&self) -> Option<String> {
        self.comment[0]
            .split_once(" in ")
            .map(|(_, region)| String::from(region).replace(" ", "_"))
    }

    fn get_2d_points(&self) -> Option<Vec<(f32, f32)>> {
        let Some(NodeCoord::Two(cities_with_idx)) = &self.node_coord else {
            return None;
        };

        Some(cities_with_idx.iter().map(|(_, x, y)| (*x, *y)).collect())
    }
}

pub fn get_instances(path: &str) -> io::Result<Vec<tsplib::Instance>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry: fs::DirEntry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some("tsp") = path.extension().and_then(|ext| ext.to_str()) {
                files.push(tsplib::read(&path)?);
            }
        }
    }

    Ok(files)
}

pub fn compute_length(permutation: &[usize], cities: &[(f32, f32)]) -> f32 {
    let n = permutation.len();
    if n == 0 {
        return 0.0;
    }

    let mut total = 0.0;

    for i in 0..n - 1 {
        let (x1, y1) = cities[permutation[i]];
        let (x2, y2) = cities[permutation[i + 1]];

        let dx = x1 - x2;
        let dy = y1 - y2;

        total += (dx * dx + dy * dy).sqrt();
    }

    let (x1, y1) = cities[permutation[n - 1]];
    let (x2, y2) = cities[permutation[0]];
    let dx = x1 - x2;
    let dy = y1 - y2;
    total += (dx * dx + dy * dy).sqrt();

    total
}
