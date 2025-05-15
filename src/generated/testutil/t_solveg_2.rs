//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      T_SOLVEG_2 ( Solve a system of algebraic equations )
pub fn T_SOLVEG_2(
    B: &mut [f64],
    DIM: i32,
    NEQ: i32,
    A: &mut [f64],
    X: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) {
    let mut B = DummyArrayMut2D::new(B, 1..=DIM, 1..=NEQ);
    let mut A = DummyArrayMut2D::new(A, 1..=DIM, 1..=DIM);
    let mut X = DummyArrayMut2D::new(X, 1..=DIM, 1..=NEQ);
    let mut MAG: f64 = 0.0;
    let mut MAXMAG: f64 = 0.0;
    let mut PART: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut MOVE: i32 = 0;
    let mut START: i32 = 0;

    //
    // SPICELIB Functions
    //
    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Standard Error Handling.
    //
    if spicelib::RETURN(ctx) {
        return;
    }

    //
    // Nothing found yet.
    //
    *FOUND = false;

    START = 1;

    while (START <= DIM) {
        //
        // First normalize each row of the matrix.
        //
        for I in START..=DIM {
            MAXMAG = 0 as f64;
            MAG = 0 as f64;

            for J in START..=DIM {
                if (f64::abs(A[[I, J]]) > MAXMAG) {
                    MAXMAG = f64::abs(A[[I, J]]);
                }
            }

            if (MAXMAG == 0 as f64) {
                //
                // The found flag is already set.
                //
                return;
            }

            for J in START..=DIM {
                TEMP = (A[[I, J]] / MAXMAG);
                MAG = (MAG + (TEMP * TEMP));
            }

            MAG = (MAXMAG * f64::sqrt(MAG));

            for J in START..=DIM {
                A[[I, J]] = (A[[I, J]] / MAG);
            }

            for K in 1..=NEQ {
                B[[I, K]] = (B[[I, K]] / MAG);
            }
        }

        //
        // Find the row with the maximum first entry.
        //
        MOVE = START;
        MAXMAG = f64::abs(A[[START, START]]);

        for I in (START + 1)..=DIM {
            if (f64::abs(A[[I, START]]) > MAXMAG) {
                MAXMAG = f64::abs(A[[I, START]]);
                MOVE = I;
            }
        }

        //
        // If the first row is not the one with the largest
        // component of interest swap with the one that does.
        //
        if (MOVE != START) {
            for I in START..=DIM {
                spicelib::SWAPD_ARRAY(
                    A.subscript([MOVE, I]),
                    A.subscript([START, I]),
                    A.as_slice_mut(),
                );
            }

            for K in 1..=NEQ {
                spicelib::SWAPD_ARRAY(
                    B.subscript([MOVE, K]),
                    B.subscript([START, K]),
                    B.as_slice_mut(),
                );
            }
        }

        //
        // Now normalize the START row so that it begins with a 1.
        //
        MAG = A[[START, START]];

        for K in 1..=NEQ {
            B[[START, K]] = (B[[START, K]] / MAG);
        }

        for I in (START + 1)..=DIM {
            A[[START, I]] = (A[[START, I]] / MAG);
        }

        for I in (START + 1)..=DIM {
            for K in 1..=NEQ {
                B[[I, K]] = (B[[I, K]] - (A[[I, START]] * B[[START, K]]));
            }

            for J in (START + 1)..=DIM {
                A[[I, J]] = (A[[I, J]] - (A[[I, START]] * A[[START, J]]));
            }
        }

        START = (START + 1);
    }

    //
    // Back substitute to get the solution.
    //
    for K in 1..=NEQ {
        for I in intrinsics::range(DIM, 1, -1) {
            PART = 0 as f64;

            for J in intrinsics::range(DIM, (I + 1), -1) {
                PART = (PART + (A[[I, J]] * X[[J, K]]));
            }

            X[[I, K]] = (B[[I, K]] - PART);
        }
    }

    //
    // We've got a solution if we made it this far.
    //
    *FOUND = true;
}
