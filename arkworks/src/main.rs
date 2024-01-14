use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{Boolean, EqGadget, AllocVar},
    uint8::UInt8
};
use ark_relations::r1cs::{SynthesisError, ConstraintSystem};
use cmp::CmpGadget;

mod cmp;
mod alloc;

pub struct Puzzle<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);
pub struct Solution<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);

fn check_rows<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, Sg_N, ConstraintF>,
) -> Result<(), SynthesisError> {
    for row in &solution.0 {
        for (j, cell) in row.iter().enumerate() {
            for prior_cell in &row[0..j] {
                cell.is_neq(&prior_cell)?
                    .enforce_equal(&Boolean::TRUE)?;
            }
        }
    }
    Ok(())
}

fn check_cols<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, Sg_N, ConstraintF>,
) -> Result<(), SynthesisError> {
    for col_index in 0..N {
        for (i, cell) in solution.0.iter().enumerate() {
            for prior_cell in &solution.0[0..i] {
                cell[col_index]
                    .is_neq(&prior_cell[col_index])?
                    .enforce_equal(&Boolean::TRUE)?;
            }
        }
    }
    Ok(())
}

fn check_subgrids<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, Sg_N, ConstraintF>,
) -> Result<(), SynthesisError> {
    let sg_i = 0;
    let mut subgrids = Vec::with_capacity(N); 
    for h_section_i in (0..N).step_by(Sg_N) {
        for v_section_i in (0..N).step_by(Sg_N) {
            for sg_row_i in 0..Sg_N {
                for sg_col_i in 0..Sg_N {
                    if sg_i==0 {
                        subgrids.push(Vec::with_capacity(N))
                    }
                    let row_i = h_section_i + sg_row_i;
                    let col_i = v_section_i + sg_col_i;
                    let row_vector = &subgrids[row_i];
                    let cell = &row_vector[col_i];
                    for prior_cell in subgrids[sg_i] {
                        cell.is_neq(&prior_cell)?
                            .enforce_equal(&Boolean::TRUE)?;
                    }
                    subgrids[sg_i].push(cell);
                }
            }
            sg_i+=1;
        }
    }
    Ok(())
}

fn check_consistency<const N: usize,const Sg_N: usize,ConstraintF: PrimeField>(
    puzzle: &Puzzle<N, Sg_N, ConstraintF>,
    solution: &Solution<N, Sg_N, ConstraintF>,
) -> Result<(), SynthesisError> {
    for (p_row, s_row) in puzzle.0.iter().zip(&solution.0) {
        for (p, s) in p_row.iter().zip(s_row) {
            // Ensure that the solution `s` is in the range [1, N]
            s.is_leq(&UInt8::constant(N as u8))?
                .and(&s.is_geq(&UInt8::constant(1))?)?
                .enforce_equal(&Boolean::TRUE)?;

            // Ensure that either the puzzle slot is 0, or that
            // the slot matches equivalent slot in the solution
            (p.is_eq(s)?.or(&p.is_eq(&UInt8::constant(0))?)?)
                .enforce_equal(&Boolean::TRUE)?;
        }
    }
    Ok(())
}

fn check_helper<const N: usize,const Sg_N: usize, ConstraintF: PrimeField>(
    puzzle: &[[u8; N]; N],
    solution: &[[u8; N]; N],
) {
    let cs = ConstraintSystem::<ConstraintF>::new_ref();
    let puzzle_var = Puzzle::new_input(cs.clone(), || Ok(puzzle)).unwrap();
    let solution_var = Solution::new_witness(cs.clone(), || Ok(solution)).unwrap();
    check_consistency(&puzzle_var, &solution_var).unwrap();
    check_rows(&solution_var).unwrap();
    check_cols(&solution_var).unwrap();
    check_subgrids(&solution_var).unwrap();
    assert!(cs.is_satisfied().unwrap());
}

fn main() {
    use ark_bls12_381::Fq as F;
    // Check that it accepts a valid solution.
    let solution = [
        [1,4 , 2,3],
        [2,3 , 1,4],

        [3,1 , 4,2],
        [4,2 , 3,1]
    ];
    let puzzle = [
        [0,0 , 0,0],
        [2,3 , 0,0],

        [0,0 , 4,0],
        [0,2 , 0,0]
    ];
    check_helper::<4,2, F>(&puzzle, &solution);
}
