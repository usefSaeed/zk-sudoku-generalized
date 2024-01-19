use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{Boolean, EqGadget, AllocVar},
    uint8::UInt8
};
use ark_relations::r1cs::{SynthesisError, ConstraintSystem};
use cmp::CmpGadget;

mod cmp;
mod alloc;

pub struct Puzzle<const N: usize,const SG_N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);
pub struct Solution<const N: usize,const SG_N: usize, ConstraintF: PrimeField>([[UInt8<ConstraintF>; N]; N]);

fn check_rows<const N: usize,const SG_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, SG_N, ConstraintF>,
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

fn check_cols<const N: usize,const SG_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, SG_N, ConstraintF>,
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

fn check_subgrids<const N: usize,const SG_N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, SG_N, ConstraintF>,
) -> Result<(), SynthesisError> {
    let mut sg_i = 0;
    let mut subgrids = Vec::with_capacity(N); 
    for h_section_i in (0..N).step_by(SG_N) {
        for v_section_i in (0..N).step_by(SG_N) {
            for sg_row_i in 0..SG_N {
                for sg_col_i in 0..SG_N {
                    if sg_i==0 {
                        subgrids.push(Vec::with_capacity(N))
                    }
                    let row_i = h_section_i + sg_row_i;
                    let col_i = v_section_i + sg_col_i;
                    let row = &solution.0[row_i];
                    let cell = &row[col_i];
                    for prior_cell in &subgrids[sg_i] {
                        cell.is_neq(&prior_cell)?
                            .enforce_equal(&Boolean::TRUE)?;
                    }
                    subgrids[sg_i].push(cell.clone());
                }
            }
            sg_i+=1;
        }
    }
    Ok(())
}

fn check_consistency<const N: usize,const SG_N: usize,ConstraintF: PrimeField>(
    puzzle: &Puzzle<N, SG_N, ConstraintF>,
    solution: &Solution<N, SG_N, ConstraintF>,
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

fn check_helper<const N: usize,const SG_N: usize, ConstraintF: PrimeField>(
    puzzle: &[[u8; N]; N],
    solution: &[[u8; N]; N],
) {
    let cs = ConstraintSystem::<ConstraintF>::new_ref();
    let puzzle_var: Puzzle<N, SG_N, ConstraintF> = Puzzle::new_input(cs.clone(), || Ok(puzzle)).unwrap();
    let solution_var: Solution<N, SG_N, ConstraintF> = Solution::new_witness(cs.clone(), || Ok(solution)).unwrap();
    check_consistency(&puzzle_var, &solution_var).unwrap();
    check_rows(&solution_var).unwrap();
    check_cols(&solution_var).unwrap();
    check_subgrids(&solution_var).unwrap();
    assert!(cs.is_satisfied().unwrap());
}

fn main() {
    use ark_bls12_381::Fq as F;
    // 4x4
    // let solution = [
    //     [1,4 , 2,3],
    //     [2,3 , 1,4],

    //     [3,1 , 4,2],
    //     [4,2 , 3,1]
    // ];
    // let puzzle = [
    //     [0,0 , 0,0],
    //     [2,3 , 0,0],

    //     [0,0 , 4,0],
    //     [0,2 , 0,0]
    // ];
    // check_helper::<4,2, F>(&puzzle, &solution);
    // // 9x9
    // let solution = [
    //     [1,9,4 , 8,6,5 , 2,3,7],
    //     [7,3,5 , 4,1,2 , 9,6,8],
    //     [8,6,2 , 3,9,7 , 1,4,5],

    //     [9,2,1 , 7,4,8 , 3,5,6],
    //     [6,7,8 , 5,3,1 , 4,2,9],
    //     [4,5,3 , 9,2,6 , 8,7,1],

    //     [3,8,9 , 6,5,4 , 7,1,2],
    //     [2,4,6 , 1,7,9 , 5,8,3],
    //     [5,1,7 , 2,8,3 , 6,9,4]
    // ];
    // let puzzle = [
    //     [0,0,0 , 8,6,0 , 2,3,0],
    //     [7,0,5 , 0,0,0 , 9,0,8],
    //     [0,6,0 , 3,0,7 , 0,4,0],
        
    //     [0,2,0 , 7,0,8 , 0,5,0],
    //     [0,7,8 , 5,0,0 , 0,0,0],
    //     [4,0,0 , 9,0,6 , 0,7,0],

    //     [3,0,9 , 0,5,0 , 7,0,2],
    //     [0,4,0 , 1,0,9 , 0,8,0],
    //     [5,0,7 , 0,8,0 , 0,9,4]
    // ];
    // check_helper::<9,3, F>(&puzzle, &solution);
    // 16x16
    let solution = [
        [4,10,7,9 , 11,13,3,2 , 14,8,12,6 , 5,16,1,15], 
        [11,5,8,12 , 9,1,10,4 , 2,15,13,16 , 7,14,3,6],
        [2,15,16,1 , 6,8,12,14 , 11,7,5,3 , 9,4,13,10],
        [14,13,6,3 , 7,5,15,16 , 1,4,9,10 , 8,12,11,2],

        [12,4,5,15 , 10,6,11,8 , 7,1,16,14 , 3,2,9,13],
        [7,11,9,10 , 12,2,1,13 , 5,3,15,4 , 6,8,16,14],
        [1,6,14,13 , 3,15,16,9 , 12,2,11,8 , 4,7,10,5],
        [8,16,3,2 , 14,7,4,5 , 9,6,10,13 , 15,11,12,1],

        [10,7,4,11 , 2,14,6,15 , 16,12,8,1 , 13,3,5,9],
        [3,2,1,5 , 16,9,7,12 , 10,13,6,11 , 14,15,4,8],
        [15,14,12,6 , 8,11,13,3 , 4,5,7,9 , 10,1,2,16],
        [16,9,13,8 , 4,10,5,1 , 15,14,3,2 , 11,6,7,12],

        [9,3,15,16 , 5,4,2,11 , 8,10,14,12 , 1,13,6,7],
        [5,1,11,4 , 15,12,8,6 , 13,9,2,7 , 16,10,14,3],
        [6,8,2,7 , 13,16,14,10 , 3,11,1,5 , 12,9,15,4],
        [13,12,10,14 , 1,3,9,7 , 6,16,4,15 , 2,5,8,11]
    ];
    let puzzle = [
        [0,0,0,0 , 0,13,3,0 , 0,0,12,0 , 0,0,1,0], 
        [0,5,0,0 , 0,1,10,4 , 2,0,13,0 , 0,0,0,6],
        [2,0,16,0 , 6,0,12,0 , 11,0,5,0 , 9,0,0,10],
        [14,13,6,0 , 0,0,0,0 , 0,0,0,0 , 8,12,0,0],

        [12,4,0,15 , 0,0,0,0 , 0,1,16,0 , 3,0,0,13],
        [0,0,0,10 , 0,2,0,0 , 5,3,0,4 , 0,0,0,0],
        [0,6,0,13 , 0,15,0,0 , 0,2,0,8 , 0,0,10,0],
        [8,0,3,0 , 0,7,0,5 , 9,6,0,0 , 15,11,12,0],

        [0,0,4,0 , 2,14,6,0 , 16,12,0,0 , 0,0,0,9],
        [0,2,0,5 , 0,0,7,0 , 10,13,6,11 , 14,0,0,8],
        [15,0,0,0 , 0,11,13,0 , 4,5,0,9 , 10,1,0,16],
        [0,0,0,8 , 0,0,0,0 , 15,0,0,2 , 11,6,0,12],

        [9,0,15,0 , 5,0,2,0 , 8,0,0,12 , 0,0,0,0],
        [0,0,0,0 , 15,0,0,6 , 13,0,2,0 , 0,0,14,0],
        [0,8,2,7 , 13,16,14,10 , 0,0,1,5 , 0,0,15,0],
        [13,12,0,14 , 0,3,0,0 , 0,16,0,15 , 0,5,8,11]
    ];
    check_helper::<16,4, F>(&puzzle, &solution);
}
