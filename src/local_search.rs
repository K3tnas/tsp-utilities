use rand::RngExt;

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
    let mut best_delta: f64 = 0.0;

    let a0 = perm[n - 1];
    let b0 = perm[0];
    for j in 1..(n - 1) {
        let c = perm[j];
        let d = perm[j + 1];

        let delta = (dist[a0][c] + dist[b0][d]) - (dist[a0][b0] + dist[c][d]);
        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((0, j));
        }
    }

    for i in 1..(n - 2) {
        let a = perm[i - 1];
        let b = perm[i];

        for j in (i + 1)..(n - 1) {
            let c = perm[j];
            let d = perm[j + 1];
            let delta = (dist[a][c] + dist[b][d]) - (dist[a][b] + dist[c][d]);
            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    let cn = perm[n - 1];
    let dn = perm[0];
    for i in 1..(n - 1) {
        let a = perm[i - 1];
        let b = perm[i];
        let delta = (dist[a][cn] + dist[b][dn]) - (dist[a][b] + dist[cn][dn]);
        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((i, n - 1));
        }
    }

    best_ij.map(|(i, j)| {
        let mut permutation = perm.clone();
        permutation[i..=j].reverse();
        Tour {
            permutation,
            length: tour.length + best_delta,
            context: tour.context,
        }
    })
}

#[allow(dead_code)]
pub fn invert_first_n<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let n = tour.permutation.len();
    let perm = &tour.permutation;
    let dist = &tour.context.dist_matrix;

    let mut rng = rand::rng();
    let mut best_ij: Option<(usize, usize)> = None;
    let mut best_delta: f64 = 0.0;

    for _ in 0..n {
        let i = rng.random_range(0..(n - 1));
        let j = rng.random_range(i + 1..n);

        let a = perm[(i + n - 1) % n];
        let b = perm[i];
        let c = perm[j];
        let d = perm[(j + 1) % n];

        let delta = (dist[a][c] + dist[b][d]) - (dist[a][b] + dist[c][d]);

        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((i, j));
        }
    }

    best_ij.map(|(i, j)| {
        let mut permutation = perm.clone();
        permutation[i..=j].reverse();
        Tour {
            permutation,
            length: tour.length + best_delta,
            context: tour.context,
        }
    })
}

#[allow(dead_code)]
pub fn transpose<'a>(tour: &Tour<'a>) -> Option<Tour<'a>> {
    let n = tour.permutation.len();
    let permutation = &tour.permutation;
    let dist = &tour.context.dist_matrix;

    let mut best_ij = None;
    let mut best_delta = 0.0;

    //(0, n-1)
    {
        let i = 0;
        let j = n - 1;

        let b = permutation[i];
        let c = permutation[i + 1];

        let d = permutation[j];
        let e = permutation[j];

        let before = dist[d][e] + dist[b][c];
        let after = dist[d][b] + dist[e][c];

        let delta = after - before;

        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((i, j));
        }
    }

    //(j = i+1)
    for i in 0..(n - 1) {
        let j = i + 1;

        let a = permutation[(n + i - 1) % n];
        let b = permutation[i];
        let e = permutation[j];
        let f = permutation[(j + 1) % n];

        let before = dist[a][b] + dist[e][f];
        let after = dist[a][e] + dist[b][f];

        let delta = after - before;

        if delta < best_delta {
            best_delta = delta;
            best_ij = Some((i, j));
        }
    }

    for i in 0..(n - 1) {
        let a = permutation[(n + i - 1) % n];
        let b = permutation[i];
        let c = permutation[i + 1];

        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            }

            let d = permutation[j - 1];
            let e = permutation[j];
            let f = permutation[(j + 1) % n];

            let before = dist[a][b] + dist[b][c] + dist[d][e] + dist[e][f];

            let after = dist[a][e] + dist[e][c] + dist[d][b] + dist[b][f];

            let delta = after - before;

            if delta < best_delta {
                best_delta = delta;
                best_ij = Some((i, j));
            }
        }
    }

    best_ij.map(|(i, j)| {
        let length = tour.length + best_delta;
        let mut permutation = permutation.clone();
        permutation.swap(i, j);

        Tour {
            permutation,
            length,
            context: tour.context,
        }
    })
}
