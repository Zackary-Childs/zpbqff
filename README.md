[![check](https://github.com/ntBre/pbqff/actions/workflows/check.yml/badge.svg)](https://github.com/ntBre/pbqff/actions/workflows/check.yml)
[![test](https://github.com/ntBre/pbqff/actions/workflows/test.yml/badge.svg)](https://github.com/ntBre/pbqff/actions/workflows/test.yml)

# pbqff
pbqff automates the construction of quartic force fields (QFFs) and the process
of generating [spectral data](https://github.com/ntBre/spectro) from them

# Installation

Install the [Rust toolchain](https://www.rust-lang.org/tools/install) with
rustup. No root access is needed, and the first build automatically downloads
the Rust version pinned in `rust-toolchain.toml`. Then, from the top of this
repository, run

```bash
cargo install --path crates/pbqff
```

This builds pbqff in release mode and puts the `pbqff` binary in
`~/.cargo/bin`, which rustup adds to your `PATH`.

On a cluster with several operating systems sharing one home directory, build
separately on each one (for example by setting `CARGO_TARGET_DIR` per OS),
because a binary built against a newer glibc will not run on an older system.

## Quantum chemistry programs

pbqff runs an external program for every single-point energy. It finds each
program through an environment variable, falling back to a default only when
the variable is unset:

| Program | Variable    | Default (local queue) |
|---------|-------------|-----------------------|
| MOPAC   | `MOPAC_CMD` | `/opt/mopac/mopac`    |
| Molpro  | `MOLPRO_CMD`| none, must be set     |
| CFOUR   | `CFOUR_CMD` | `/opt/cfour/cfour`    |
| DFTB+   | `DFTB_CMD`  | `/opt/dftb+/dftb+`    |
| ORCA    | `ORCA_CMD`  | `orca`                |

Export the variable for each program you use (for example in `~/.bashrc`).
Slurm passes your environment to jobs by default, and the bundled PBS
templates request the same with `#PBS -V`.

## Running the tests

```bash
PSQS_NO_RESUB=1 cargo test --workspace --all-features
```

The integration tests run real MOPAC calculations, so `MOPAC_CMD` must point to
a working MOPAC (the open-source releases at
https://github.com/openmopac/mopac/releases work). `PSQS_NO_RESUB=1` makes a
failed job submission stop the run instead of retrying.

# Coordinate Types
pbqff supports running QFFs in the following coordinate systems:
- Symmetry-internal coordinates (SICs) via
  [intder](https://github.com/ntBre/intder)
- Cartesian coordinates
- Normal coordinates

The normal coordinates are determined automatically by running a Cartesian
harmonic force field.

# Programs and Queues
pbqff supports MOPAC, Molpro, CFOUR, DFTB+, and ORCA for computing single-point
energies, and the PBS and Slurm queuing systems (plus a local queue) via [psqs](https://github.com/ntBre/psqs)

# Input
An example input file for a Mopac QFF on *c*-C<sub>3</sub>H<sub>2</sub> looks like:
```
geometry = """
C
C 1 CC
C 1 CC 2 CCC
H 2 CH 1 HCC 3 180.0
H 3 CH 1 HCC 2 180.0

CC =                  1.42101898
CCC =                55.60133141
CH =                  1.07692776
HCC =               147.81488230
"""
optimize = true
charge = 0
step_size = 0.005
coord_type = "sic"
program = "mopac"
queue = "slurm"
sleep_int = 2
job_limit = 2048
chunk_size = 1
template = """scfcrt=1.D-21 aux(precision=14 comp xp xs xw) PM6 THREADS=1 \
external=testfiles/params.dat"""
check_int = 100
```

# qffbuddy
An optional GUI for preparing input files and running pbqff is also included in
the qffbuddy directory

# Citations
For `pbqff` itself, please cite B. R. Westbrook and R. C. Fortenberry. "pbqff:
Push-Button Quartic Force Fields." *J. Chem. Theory Comput.*, 2023. DOI:
[10.1021/acs.jctc.3c00129](https://doi.org/10.1021/acs.jctc.3c00129)

```
@article{Westbrook23_pbqff,
  author = {Brent R. Westbrook and Ryan C. Fortenberry},
  title = {pbqff: Push-Button Quartic Force Fields},
  journal = {J. Chem. Theory Comput.},
  volume = 19,
  number = 9,
  pages = {2606-2615},
  year = 2023,
}
```

If you use symmetry-internal coordinates, you may also want to cite the original
`INTDER` code by Wesley Allen:

```
@misc{intder,
 author = {W. D. Allen and coworkers},
 note = {$INTDER\ 2005$ is a General Program Written by W. D. Allen and Coworkers, which Performs Vibrational Analysis and Higher-Order Non-Linear Transformations.},
 year = {2005}
}
```

And for the original VPT2 code in `SPECTRO`, you can cite

```
@incollection{spectro91,
 address = {Greenwich, Connecticut},
 author = {J. F. Gaw and A. Willets and W. H. Green and N. C. Handy},
 booktitle = {Advances in Molecular Vibrations and Collision Dynamics},
 editor = {Joel M. Bowman and Mark A. Ratner},
 pages = {170-185},
 publisher = {JAI Press, Inc.},
 title = {{SPECTRO: A} Program for the Derivation of Spectroscopic Constants From Provided Quartic Force Fields and Cubic Dipole Fields},
 year = {1991}
}
```

# Disclaimer

I still need to pick a license for this project, but for now here's an important
disclaimer from the MIT License:

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

I am happy to look at issues and want to fix them, but my responses may be
delayed.

# TODOs
- [ ] factor out commonality in first part of `CoordType::run`
- [ ] use or delete `Fitted` trait
  - `Findiff` actually has some default methods that make it useful, but
    `Fitted` doesn't provide any default methods and isn't used as a bound
