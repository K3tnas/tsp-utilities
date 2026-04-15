use crate::tour::Tour;

impl<'a> Tour<'a> {
    pub fn local_search<F>(&mut self, neigh_fun: F)
    where
        F: Fn(&Tour<'a>) -> Option<Tour<'a>>,
    {
        while let Some(improved) = neigh_fun(self) {
            *self = improved;
        }
    }

    pub fn local_search_with_counting<F>(&mut self, neigh_fun: F) -> usize
    where
        F: Fn(&Tour<'a>) -> Option<Tour<'a>>,
    {
        let mut n = 0;

        while let Some(improved) = neigh_fun(self) {
            *self = improved;
            n += 1;
        }

        n
    }
}

#[allow(dead_code)]
pub fn invert<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let n = tour.permutation.len();
    let perm = &tour.permutation;
    let dist = &tour.context.dist_matrix;

    let mut best_ij: Option<(usize, usize)> = None;
    let mut best_delta = 0.0;

    let a = perm[n - 1];
    let b = perm[0];
    for j in 1..(n - 1) {
        let c = perm[j];
        let d = perm[(j + 1) % n];

        let delta = (dist[a][c] + dist[b][d]) - (dist[a][b] + dist[c][d]);

        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((0, j));
        }
    }

    for i in 1..(n - 1) {
        let a = perm[i - 1];
        let b = perm[i];

        for j in (i + 1)..n {
            let c = perm[j];
            let d = perm[(j + 1) % n];

            let delta = (dist[a][c] + dist[b][d]) - (dist[a][b] + dist[c][d]);

            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    best_ij.map(|(i, j)| {
        let mut permutation = perm.clone();
        permutation[i..=j].reverse();

        let length = tour.length + best_delta;

        Tour {
            permutation,
            length,
            context: tour.context,
        }
    })
}

#[allow(dead_code)]
pub fn transpose<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let mut permutation = tour.permutation.clone();
    let n = permutation.len();
    let dist = &tour.context.dist_matrix;

    let mut best_ij = None;
    let mut best_delta = 0.0;

    for i in 0..n {
        let a = permutation[(n + i - 1) % n];
        let b = permutation[i];
        let c = permutation[(n + i + 1) % n];

        for j in (i + 1)..n {
            let d = permutation[(n + j - 1) % n];
            let e = permutation[j];
            let f = permutation[(j + 1) % n];

            let before: f64;
            let after: f64;

            if i == 0 && j == n - 1 {
                before = dist[d][e] + dist[b][c];
                after = dist[d][b] + dist[e][c];
            } else if j == i + 1 {
                before = dist[a][b] + dist[e][f];
                after = dist[a][e] + dist[b][f];
            } else {
                before = dist[a][b] + dist[b][c] + dist[d][e] + dist[e][f];

                after = dist[a][e] + dist[e][c] + dist[d][b] + dist[b][f];
            }

            let delta = after - before;

            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    best_ij.map(|(i, j)| {
        let length = tour.length + best_delta;
        permutation.swap(i, j);

        Tour {
            permutation,
            length,
            context: tour.context,
        }
    })
}

// #[allow(dead_code)]
// pub fn transpose<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
//     let n = tour.permutation.len();
//     let perm = &tour.permutation;
//     let problem = tour.problem;
//
//     let mut best_ij = None;
//     let mut best_delta = 0.0;
//
//     for i in 0..n {
//         for j in (i + 1)..n {
//             let a = perm[(n + i - 1) % n];
//             let b = perm[i];
//             let c = perm[(n + i + 1) % n];
//             let d = perm[(n + j - 1) % n];
//             let e = perm[j];
//             let f = perm[(j + 1) % n];
//
//             let before: f64;
//             let after: f64;
//
//             if i == 0 && j == n - 1 {
//                 before = problem.dist(d, e) + problem.dist(b, c);
//                 after = problem.dist(d, b) + problem.dist(e, c);
//             } else if j == i + 1 {
//                 before = problem.dist(a, b) + problem.dist(e, f);
//                 after = problem.dist(a, e) + problem.dist(b, f);
//             } else {
//                 before = problem.dist(a, b)
//                     + problem.dist(b, c)
//                     + problem.dist(d, e)
//                     + problem.dist(e, f);
//
//                 after = problem.dist(a, e)
//                     + problem.dist(e, c)
//                     + problem.dist(d, b)
//                     + problem.dist(b, f);
//             }
//
//             let delta = after - before;
//
//             if delta < best_delta {
//                 best_delta = delta;
//                 best_ij = Some((i, j));
//             }
//         }
//     }
//
//     best_ij.map(|(i, j)| {
//         let mut permutation = tour.permutation.to_vec();
//         permutation.swap(i, j);
//         let length = tour.length + best_delta;
//         Tour {
//             permutation,
//             length,
//             problem,
//         }
//     })
// }
//
// pub fn invert_with_selection<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
//     let n = tour.permutation.len();
//     let perm = &tour.permutation;
//     let mut rng = rand::rng();
//
//     let mut best_ij = None;
//     let mut best_delta = 0.0;
//     let neighbours: Vec<(usize, usize)> = (0..n)
//         .map(|_| {
//             let i = rng.random_range(0..n - 1);
//             let j = rng.random_range(i + 1..n);
//             (i, j)
//         })
//         .collect();
//
//     for (i, j) in neighbours {
//         if i == 0 && j == n - 1 {
//             continue;
//         }
//
//         let a = perm[(n + i - 1) % n];
//         let b = perm[i];
//         let c = perm[j];
//         let d = perm[(j + 1) % n];
//
//         let before = tour.problem.dist(a, b) + tour.problem.dist(c, d);
//         let after = tour.problem.dist(a, c) + tour.problem.dist(b, d);
//         let delta = after - before;
//
//         if delta < best_delta {
//             best_delta = delta;
//             best_ij = Some((i, j));
//         }
//     }
//
//     best_ij.map(|(i, j)| {
//         let mut permutation = perm.to_vec();
//         permutation[i..=j].reverse();
//         Tour {
//             permutation,
//             length: tour.length + best_delta,
//             problem: tour.problem,
//         }
//     })
// }
