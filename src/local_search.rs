use crate::tour::Tour;

pub type NeighbourhoodFunction = dyn for<'a> Fn(&Tour<'a>) -> Option<Tour<'a>>;

impl<'a> Tour<'a> {
    pub fn local_search(&mut self, optimize: &NeighbourhoodFunction) {
        while let Some(optimized) = optimize(self) {
            *self = optimized;
        }
    }

    pub fn local_search_with_counting(&mut self, optimize: &NeighbourhoodFunction) -> usize {
        let mut n = 0;

        while let Some(optimized) = optimize(self) {
            *self = optimized;
            n += 1;
        }

        n
    }
}

#[allow(dead_code)]
pub fn invert<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let n = tour.permutation.len();
    let perm = &tour.permutation;

    let mut best_ij = None;
    let mut best_delta = 0.0;

    for i in 0..n {
        for j in (i + 1)..n {
            if i == 0 && j == n - 1 {
                continue;
            }

            let a = perm[(n + i - 1) % n];
            let b = perm[i];
            let c = perm[j];
            let d = perm[(j + 1) % n];

            let before = tour.problem.dist(a, b) + tour.problem.dist(c, d);
            let after = tour.problem.dist(a, c) + tour.problem.dist(b, d);
            let delta = after - before;

            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    best_ij.map(|(i, j)| {
        let mut permutation = perm.to_vec();
        permutation[i..=j].reverse();
        Tour {
            permutation,
            length: tour.length + best_delta,
            problem: tour.problem,
        }
    })
}

#[allow(dead_code)]
fn transpose<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let n = tour.permutation.len();
    let perm = &tour.permutation;
    let problem = tour.problem;

    let mut best_ij = None;
    let mut best_delta = 0.0;

    for i in 0..n {
        for j in (i + 1)..n {
            let a = perm[(n + i - 1) % n];
            let b = perm[i];
            let c = perm[(n + i + 1) % n];
            let d = perm[(n + j - 1) % n];
            let e = perm[j];
            let f = perm[(j + 1) % n];

            let before: f64;
            let after: f64;

            if i == 0 && j == n - 1 {
                before = problem.dist(d, e) + problem.dist(b, c);
                after = problem.dist(d, b) + problem.dist(e, c);
            } else if j == i + 1 {
                before = problem.dist(a, b) + problem.dist(e, f);
                after = problem.dist(a, e) + problem.dist(b, f);
            } else {
                before = problem.dist(a, b)
                    + problem.dist(b, c)
                    + problem.dist(d, e)
                    + problem.dist(e, f);

                after = problem.dist(a, e)
                    + problem.dist(e, c)
                    + problem.dist(d, b)
                    + problem.dist(b, f);
            }

            let delta = after - before;

            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    best_ij.map(|(i, j)| {
        let mut permutation = tour.permutation.to_vec();
        permutation.swap(i, j);
        let length = tour.length + best_delta;
        Tour {
            permutation,
            length,
            problem,
        }
    })
}
