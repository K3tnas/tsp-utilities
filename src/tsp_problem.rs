pub struct TspProblem {
    pub cities: Box<[(f64, f64)]>,
    pub dist_matrix: Box<[Box<[f64]>]>,
}

impl TspProblem {
    pub fn new(cities: &[(f64, f64)]) -> Self {
        let n = cities.len();
        let dist_matrix: Box<[Box<[f64]>]> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let (x1, y1) = cities[i];
                        let (x2, y2) = cities[j];
                        let dx = x1 - x2;
                        let dy = y1 - y2;
                        (dx * dx + dy * dy).sqrt().round()
                    })
                    .collect()
            })
            .collect();

        let cities: Box<[(f64, f64)]> = cities.into();

        TspProblem {
            cities,
            dist_matrix,
        }
    }
}
