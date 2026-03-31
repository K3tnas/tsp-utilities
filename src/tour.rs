use plotters::drawing::IntoDrawingArea;
use plotters::style::Color;
use std::error::Error;
use std::path::PathBuf;

use plotters::chart::ChartBuilder;
use plotters::prelude::BitMapBackend;
use plotters::prelude::Circle;
use plotters::series::LineSeries;
use plotters::style::BLUE;
use plotters::style::RED;
use plotters::style::WHITE;
use rand::seq::SliceRandom;

use crate::TspProblem;
use crate::compute_length;

pub struct Tour<'a> {
    pub permutation: Vec<usize>,
    pub length: f64,
    pub problem: &'a TspProblem,
}

impl<'a> Tour<'a> {
    pub fn new_rand_tour<R: rand::Rng>(problem: &'a TspProblem, rng: &mut R) -> Self {
        let mut permutation: Vec<usize> = (0..problem.cities.len()).collect();
        permutation.shuffle(rng);
        let length = compute_length(&permutation, problem);

        Self {
            problem,
            permutation,
            length,
        }
    }

    pub fn plot_tour(&self, dirpath: &str, filename: &str) -> Result<(), Box<dyn Error>> {
        let cities: Vec<(f64, f64)> = self.problem.cities.iter().map(|(x, y)| (*y, *x)).collect();

        if cities.is_empty() {
            return Err("No cities to plot".into());
        }

        std::fs::create_dir_all(dirpath)?;

        let (mut x_min, mut y_min) = cities[0];
        let (mut x_max, mut y_max) = cities[0];

        for &(x, y) in &cities {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }

        let margin_x = (x_max - x_min) * 0.05;
        let margin_y = (y_max - y_min) * 0.05;

        let x_range = (x_min - margin_x)..(x_max + margin_x);
        let y_range = (y_min - margin_y)..(y_max + margin_y);

        let width = (x_max - x_min).max(1.0);
        let height = (y_max - y_min).max(1.0);

        let aspect_ratio = width / height;

        let base_size = 3200u32;

        let (img_width, img_height) = if aspect_ratio >= 1.0 {
            ((base_size as f64 * aspect_ratio) as u32, base_size)
        } else {
            (base_size, (base_size as f64 / aspect_ratio) as u32)
        };

        let filepath = PathBuf::from(dirpath).join(format!("{filename}.png"));

        let root = BitMapBackend::new(&filepath, (img_width, img_height)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(filename, ("sans-serif", 40))
            .build_cartesian_2d(x_range.clone(), y_range.clone())?;

        chart.draw_series(
            cities
                .iter()
                .map(|&(x, y)| Circle::new((x, y), 5, RED.filled())),
        )?;

        let mut path: Vec<(f64, f64)> = self.permutation.iter().map(|&idx| cities[idx]).collect();

        if let Some(&first) = path.first() {
            path.push(first);
        }

        chart.draw_series(LineSeries::new(path, &BLUE))?;

        root.present()?;

        Ok(())
    }
}
