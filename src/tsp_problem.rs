pub struct TspProblem {
    pub cities: Vec<(f64, f64)>,
    dist_matrix: Vec<f64>,
}

impl TspProblem {
    pub fn new(cities: Vec<(f64, f64)>) -> Self {
        let n = cities.len();
        let mut dist_matrix = Vec::with_capacity(n * n);

        for i in 0..n {
            for j in 0..n {
                let (x1, y1) = cities[i];
                let (x2, y2) = cities[j];
                let dx = x1 - x2;
                let dy = y1 - y2;
                dist_matrix.push((dx * dx + dy * dy).sqrt());
            }
        }

        TspProblem {
            cities,
            dist_matrix,
        }
    }

    #[inline(always)]
    pub fn dist(&self, i: usize, j: usize) -> f64 {
        self.dist_matrix[(self.cities.len() * i) + j]
    }
}
