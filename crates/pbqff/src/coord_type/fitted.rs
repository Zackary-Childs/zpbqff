//! Compute force constants using [ordinary
//! least-squares](https://en.wikipedia.org/wiki/Ordinary_least_squares)
//! fitting.

use std::{io::Write, path::Path};

use psqs::geom::Geom;
use spectro::{Output, Spectro};
use symm::{Molecule, PointGroup};
use taylor::Taylor;

use super::FreqError;

pub(crate) type AtomicNumbers = Vec<usize>;
type AnpassRes = (Vec<anpass::fc::Fc>, anpass::Bias);
type AnpassError = Box<Result<(Spectro, Output), FreqError>>;

/// Required methods for a least-squares fitted [crate::coord_type::CoordType].
pub trait Fitted {
    /// The error returned by a failing [Self::generate_pts].
    type Error;

    /// Displace `mol` by `step_size` to generate all of the geometries
    /// comprising the QFF. Return the geometries, along with the [Taylor] and
    /// the atomic numbers needed to compute the frequencies.
    fn generate_pts<W: Write>(
        &mut self,
        dir: impl AsRef<Path>,
        w: &mut W,
        mol: &Molecule,
        pg: &PointGroup,
        step_size: f64,
    ) -> Result<(Vec<Geom>, Taylor, AtomicNumbers), Self::Error>;

    /// Perform the fitting of the potential energy surface. Return the final
    /// force constants ([anpass::fc::Fc]) and the corresponding
    /// [anpass::Bias].
    fn anpass<W: Write>(
        &self,
        dir: Option<impl AsRef<Path>>,
        energies: &mut [f64],
        taylor: &Taylor,
        step_size: f64,
        w: &mut W,
    ) -> Result<AnpassRes, AnpassError>;
}

/// Estimated peak memory in bytes for the least-squares fit in
/// [anpass::Anpass::fit]: the dense design matrix X (npoints x nterms),
/// its transpose, and XTX (nterms x nterms), all f64.
pub(crate) fn fit_memory_bytes(npoints: usize, nterms: usize) -> u128 {
    let (p, t) = (npoints as u128, nterms as u128);
    8 * (2 * p * t + t * t)
}

/// The memory this process may use: the smaller of the machine's total memory
/// and the memory limit of the job's cgroup (what Slurm `--mem` sets), when
/// either can be read.
fn memory_limit_bytes() -> Option<u128> {
    let total = std::fs::read_to_string("/proc/meminfo").ok().and_then(|s| {
        s.lines()
            .find(|l| l.starts_with("MemTotal:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|kb| kb.parse::<u128>().ok())
            .map(|kb| kb * 1024)
    });
    // cgroup v2: /proc/self/cgroup has a line "0::/path/of/this/cgroup"
    let cgroup = std::fs::read_to_string("/proc/self/cgroup")
        .ok()
        .and_then(|s| {
            s.lines()
                .find_map(|l| l.strip_prefix("0::").map(str::to_owned))
        })
        .and_then(|path| {
            std::fs::read_to_string(format!("/sys/fs/cgroup{path}/memory.max"))
                .ok()
        })
        .and_then(|max| max.trim().parse::<u128>().ok()); // "max" = no limit
    match (total, cgroup) {
        (Some(t), Some(c)) => Some(t.min(c)),
        (t, c) => t.or(c),
    }
}

/// Report the size of the upcoming fit, and stop before any single points are
/// run if it cannot fit in memory. Called right after the points are
/// generated, so a too-large QFF fails in seconds rather than after hours of
/// single-point energies.
pub(crate) fn check_fit_size<W: Write>(
    w: &mut W,
    npoints: usize,
    nterms: usize,
) {
    let need = fit_memory_bytes(npoints, nterms);
    let gb = |b: u128| b as f64 / 1e9;
    writeln!(
        w,
        "least-squares fit: {npoints} points x {nterms} terms, needs about {:.1} GB",
        gb(need)
    )
    .unwrap();
    if let Some(limit) = memory_limit_bytes()
        && need > limit
    {
        let msg = format!(
            "the least-squares fit needs about {:.1} GB but only {:.1} GB is \
             available, so this run would fail after computing every single \
             point. For normal coordinates set `findiff = true`; otherwise use a \
             smaller molecule, more symmetry, or a machine with more memory.",
            gb(need),
            gb(limit)
        );
        writeln!(w, "error: {msg}").unwrap();
        eprintln!("error: {msg}");
        std::process::exit(1);
    }
}
