//! Bundled submission-script templates used by PBQFF.

pub const PBS_MOPAC: &str = r#"#!/bin/sh
#PBS -N {{.basename}}
#PBS -S /bin/bash
#PBS -j oe
#PBS -o {{.filename}}.out
#PBS -W umask=022
#PBS -l walltime=1000:00:00
#PBS -l ncpus=1
#PBS -l mem=1gb
#PBS -q workq
#PBS -V

module load openpbs

export WORKDIR=$PBS_O_WORKDIR
cd $WORKDIR

export MOPAC_CMD=${MOPAC_CMD:-mopac}
"#;

pub const PBS_MOLPRO: &str = r#"#!/bin/sh
#PBS -N {{.basename}}
#PBS -S /bin/bash
#PBS -j oe
#PBS -o {{.basename}}.out
#PBS -W umask=022
#PBS -l walltime=1000:00:00
#PBS -l ncpus=1
#PBS -l mem=8gb
#PBS -q workq
#PBS -V

module load openpbs molpro

export WORKDIR=$PBS_O_WORKDIR
export TMPDIR=/tmp/$USER/$PBS_JOBID
cd $WORKDIR
mkdir -p $TMPDIR
trap 'rm -rf $TMPDIR' EXIT

export MOLPRO_CMD=${MOLPRO_CMD:-"molpro -t $NCPUS --no-xml-output"}
"#;

pub const PBS_CFOUR: &str = r#"#!/bin/sh
#PBS -N {{.basename}}
#PBS -S /bin/bash
#PBS -j oe
#PBS -o {{.filename}}.out
#PBS -W umask=022
#PBS -l walltime=1000:00:00
#PBS -l ncpus=1
#PBS -l mem=8gb
#PBS -q workq
#PBS -V

module load openpbs

export WORKDIR=$PBS_O_WORKDIR
cd $WORKDIR

export CFOUR_CMD=${CFOUR_CMD:-xcfour}
"#;

pub const PBS_DFTBPLUS: &str = r#"#!/bin/sh
#PBS -N {{.basename}}
#PBS -S /bin/bash
#PBS -j oe
#PBS -o {{.filename}}.out
#PBS -W umask=022
#PBS -l walltime=1000:00:00
#PBS -l ncpus=1
#PBS -l mem=8gb
#PBS -q workq
#PBS -V

module load openpbs

export WORKDIR=$PBS_O_WORKDIR
cd $WORKDIR

export DFTB_CMD=${DFTB_CMD:-dftb+}
"#;

pub const PBS_ORCA: &str = r#"#!/bin/bash
#PBS -N {{.basename}}
#PBS -S /bin/bash
#PBS -j oe
#PBS -o {{.filename}}.out
#PBS -l mem=16gb
#PBS -l nodes=1:ppn=8
#PBS -l walltime=2150:00:00
#PBS -q workq
#PBS -V

module load openpbs

export OMP_NUM_THREADS=8
export ORCA_DIR=${ORCA_DIR:-$(dirname "$(command -v orca)")}
export XTBEXE=$ORCA_DIR/xtb/xtb-dist/bin/xtb
export PATH=$ORCA_DIR:$ORCA_DIR/xtb/xtb-dist/bin:$PATH
export WORKDIR=$PBS_O_WORKDIR
export TMPDIR=/tmp/$USER.$PBS_JOBID

mkdir -p "$TMPDIR"
trap 'rm -rf "$TMPDIR"' EXIT
cd "$WORKDIR"

run_orca() {
    input=$1
    name=$(basename "${input%.inp}")
    scratch="$TMPDIR/$name"
    mkdir -p "$scratch"
    cp "$input" "$scratch/$name.inp" || return
    (
        cd "$scratch" || exit 1
        "$ORCA_DIR/orca" "$name.inp" '--bind-to core:overload-allowed --use-hwthread-cpus'
    )
    status=$?
    rm -rf "$scratch"
    return $status
}
ORCA_CMD=run_orca
"#;

pub const SLURM_MOPAC: &str = r#"#!/bin/bash
#SBATCH --job-name={{.basename}}
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH -o {{.filename}}.out
#SBATCH --no-requeue
#SBATCH --mem=1gb
 
export MOPAC_CMD=${MOPAC_CMD:-mopac}
echo $SLURM_JOB_ID
date
hostname
"#;
 
pub const SLURM_MOLPRO: &str = r#"#!/bin/bash
#SBATCH --job-name={{.basename}}
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH -o {{.filename}}.out
#SBATCH --no-requeue
#SBATCH --mem=8gb
 
export MOLPRO_CMD=${MOLPRO_CMD:-"molpro -t 1 --no-xml-output"}
"#;
 
pub const SLURM_CFOUR: &str = r#"#!/bin/bash
#SBATCH --job-name={{.basename}}
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH -o {{.filename}}.out
#SBATCH --no-requeue
#SBATCH --mem=8gb
 
export CFOUR_CMD=${CFOUR_CMD:-xcfour}
"#;
 
pub const SLURM_DFTBPLUS: &str = r#"#!/bin/bash
#SBATCH --job-name={{.basename}}
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH -o {{.filename}}.out
#SBATCH --no-requeue
#SBATCH --mem=8gb
 
export DFTB_CMD=${DFTB_CMD:-dftb+}
"#;
 
pub const SLURM_ORCA: &str = r#"#!/bin/bash
#SBATCH --job-name={{.basename}}
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH -o {{.filename}}.out
#SBATCH --no-requeue
#SBATCH --mem=8gb
 
export ORCA_CMD=${ORCA_CMD:-orca}
"#;

pub const LOCAL_MOPAC: &str = "export MOPAC_CMD=${MOPAC_CMD:-/opt/mopac/mopac}
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:-/opt/mopac/}\n";
pub const LOCAL_MOLPRO: &str = "";
pub const LOCAL_CFOUR: &str = "CFOUR_CMD=${CFOUR_CMD:-/opt/cfour/cfour}\n";
pub const LOCAL_DFTBPLUS: &str = "DFTB_CMD=${DFTB_CMD:-/opt/dftb+/dftb+}\n";
pub const LOCAL_ORCA: &str = "ORCA_CMD=${ORCA_CMD:-orca}\n";
