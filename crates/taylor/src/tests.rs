use std::str::FromStr;

use symm::{Axis, Plane};

use super::*;

fn load_vec<T>(filename: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut ret = Vec::new();
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    for line in lines {
        ret.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }
    ret
}

#[test]
fn test_forces() {
    let got = Taylor::new(5, 3, None, None).forces;
    #[rustfmt::skip]
	let want = vec![
	    vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2],
	    vec![0, 0, 3], vec![0, 0, 4], vec![0, 1, 0],
	    vec![0, 1, 1], vec![0, 1, 2], vec![0, 1, 3],
	    vec![0, 2, 0], vec![0, 2, 1], vec![0, 2, 2],
	    vec![0, 3, 0], vec![0, 3, 1], vec![0, 4, 0],
	    vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2],
	    vec![1, 0, 3], vec![1, 1, 0], vec![1, 1, 1],
	    vec![1, 1, 2], vec![1, 2, 0], vec![1, 2, 1],
	    vec![1, 3, 0], vec![2, 0, 0], vec![2, 0, 1],
	    vec![2, 0, 2], vec![2, 1, 0], vec![2, 1, 1],
	    vec![2, 2, 0], vec![3, 0, 0], vec![3, 0, 1],
	    vec![3, 1, 0], vec![4, 0, 0]];
    assert_eq!(got, want);
}

/// number of ways to choose k items from n, computed one step at a time so it
/// stays exact and never overflows for the sizes used here
fn n_choose_k(n: usize, k: usize) -> usize {
    let mut ans = 1;
    for i in 0..k {
        ans = ans * (n - i) / (i + 1);
    }
    ans
}

#[test]
fn test_forces_large() {
    for n in [10, 27, 28, 30] {
        let got = Taylor::new(5, n, None, None).forces;

        // 1. every term of total degree <= 4 in n coordinates is present
        assert_eq!(got.len(), n_choose_k(n + 4, 4), "wrong term count for n = {n}");

        // 2. no coordinate is missing from the expansion
        for i in 0..n {
            assert!(
                got.iter().any(|row| row[i] > 0),
                "coordinate {i} never appears for n = {n}"
            );
        }

        // 3. every term is a valid quartic-or-lower term
        for row in &got {
            let degree: usize = row.iter().map(|&x| x as usize).sum();
            assert!(degree <= 4, "term {row:?} has degree {degree} for n = {n}");
        }
    }
}

#[test]
fn test_forces_with_checks() {
    let got = Taylor::new(
        5,
        9,
        Some(Checks([vec![5, 6, 7], vec![8], vec![9]])),
        Some(Checks([vec![5, 6, 7], vec![8], vec![9]])),
    );
    let want = load_vec::<u8>("testfiles/force.txt");
    assert_eq!(got.forces, want);
}

#[test]
fn test_forces_with_zero_checks() {
   let got = Taylor::new(
        5,
        3,
        Some(Checks([vec![3], vec![], vec![]])),
        Some(Checks([vec![3], vec![], vec![]])),
    );
    #[rustfmt::skip]
    let want = vec![
        vec![0, 0, 0], vec![0, 0, 2], vec![0, 0, 4],
        vec![0, 1, 0], vec![0, 1, 2], vec![0, 2, 0],
        vec![0, 2, 2], vec![0, 3, 0], vec![0, 4, 0],
        vec![1, 0, 0], vec![1, 0, 2], vec![1, 1, 0],
        vec![1, 1, 2], vec![1, 2, 0], vec![1, 3, 0],
        vec![2, 0, 0], vec![2, 0, 2], vec![2, 1, 0],
        vec![2, 2, 0], vec![3, 0, 0], vec![3, 1, 0],
        vec![4, 0, 0],
    ];
    assert_eq!(got.forces, want);
}

#[test]
fn test_disps() {
    let got = Taylor::new(5, 3, None, None).disps();
    let mut want = Disps(load_vec::<i8>("testfiles/dispu.h2o.txt"));
    // the order doesn't matter, so let rust sort both
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn test_disps_with_checks() {
    let got = Taylor::new(
        5,
        9,
        Some(Checks([vec![5, 6, 7], vec![8], vec![9]])),
        Some(Checks([vec![5, 6, 7], vec![8], vec![9]])),
    )
    .disps();
    let mut want = Disps(load_vec::<i8>("testfiles/dispu.c3h2.mod.txt"));
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn test_disps_with_zero_checks() {
    let got = Taylor::new(
        5,
        3,
        Some(Checks([vec![3], vec![], vec![]])),
        Some(Checks([vec![3], vec![], vec![]])),
    )
    .disps();
    let mut want = Disps(load_vec::<i8>("testfiles/dispu.h2o.mod.txt"));
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn make_checks_c2v() {
    use Irrep::*;
    let irreps = vec![
        (0, A1),
        (2, A1),
        (3, A1),
        (8, A1),
        (1, B2),
        (4, B2),
        (6, B2),
        (7, B1),
        (5, A2),
    ];
    let pg = PointGroup::C2v {
        axis: Axis::Y,
        planes: [Plane(Axis::Y, Axis::Z), Plane(Axis::X, Axis::Y)],
    };
    let got = Taylor::make_checks(irreps, &pg);
    let w = Checks([vec![2, 5, 7], vec![8], vec![6]]);
    let want = (Some(w.clone()), Some(w));
    assert_eq!(got, want);
}

#[test]
fn make_checks_cs() {
    use Irrep::*;
    let irreps = vec![
        (0, Ap),
        (2, Ap),
        (3, App),
        (8, Ap),
        (1, Ap),
        (4, Ap),
        (6, Ap),
        (7, App),
        (5, App),
    ];
    let pg = PointGroup::Cs {
        plane: Plane(Axis::Y, Axis::Z),
    };
    let got = Taylor::make_checks(irreps, &pg);
    let w = Checks([vec![4, 8, 6], vec![], vec![]]);
    let want = (Some(w.clone()), Some(w));
    assert_eq!(got, want);
}
