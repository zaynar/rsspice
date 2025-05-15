//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZMSXF ( Multiply sequence of state transformations )
pub fn ZZMSXF(MATRIX: &[f64], N: i32, OUTPUT: &mut [f64]) {
    let MATRIX = DummyArray3D::new(MATRIX, 1..=6, 1..=6, 1..);
    let mut OUTPUT = DummyArrayMut2D::new(OUTPUT, 1..=6, 1..=6);
    let mut L: i32 = 0;
    let mut M: i32 = 0;
    let mut GET: i32 = 0;
    let mut PUT: i32 = 0;
    let mut INCR: i32 = 0;
    let mut TEMP = StackArray3D::<f64, 72>::new(1..=6, 1..=6, 1..=2);

    //
    // If we have more than 2 matrices to deal with we will need to
    // set up the PUT location
    //
    PUT = 1;

    //
    // We perform tests in the order they seem most likely to
    // occur.
    //
    if (N == 2) {
        //
        // If there are exactly two inputs, then the output takes
        // only a single matrix multiply.
        //
        for J in 1..=3 {
            for K in 1..=3 {
                OUTPUT[[J, K]] = (((MATRIX[[J, 1, 2]] * MATRIX[[1, K, 1]])
                    + (MATRIX[[J, 2, 2]] * MATRIX[[2, K, 1]]))
                    + (MATRIX[[J, 3, 2]] * MATRIX[[3, K, 1]]));
            }
        }

        for J in 4..=6 {
            for K in 1..=3 {
                OUTPUT[[J, K]] = ((((((MATRIX[[J, 1, 2]] * MATRIX[[1, K, 1]])
                    + (MATRIX[[J, 2, 2]] * MATRIX[[2, K, 1]]))
                    + (MATRIX[[J, 3, 2]] * MATRIX[[3, K, 1]]))
                    + (MATRIX[[J, 4, 2]] * MATRIX[[4, K, 1]]))
                    + (MATRIX[[J, 5, 2]] * MATRIX[[5, K, 1]]))
                    + (MATRIX[[J, 6, 2]] * MATRIX[[6, K, 1]]));
            }
        }

        for J in 1..=3 {
            for K in 4..=6 {
                OUTPUT[[J, K]] = 0.0;
            }
        }

        for J in 4..=6 {
            L = (J - 3);
            for K in 4..=6 {
                M = (K - 3);
                OUTPUT[[J, K]] = OUTPUT[[L, M]];
            }
        }
    } else if (N > 2) {
        //
        // We need to compute the product
        //
        //    MATRIX( , ,N) * MATRIX( , ,N-1) * ... * MATRIX( , , 1 )
        //
        // Compute the first product.  MATRIX( , ,2) * MATRIX( , ,1)
        //
        //
        // First compute the upper left hand 3x3 portion of the product...
        //
        for J in 1..=3 {
            for K in 1..=3 {
                TEMP[[J, K, PUT]] = (((MATRIX[[J, 1, 2]] * MATRIX[[1, K, 1]])
                    + (MATRIX[[J, 2, 2]] * MATRIX[[2, K, 1]]))
                    + (MATRIX[[J, 3, 2]] * MATRIX[[3, K, 1]]));
            }
        }

        //
        // Next compute the lower left hand 3x3 portion of the product.
        //
        for J in 4..=6 {
            for K in 1..=3 {
                TEMP[[J, K, PUT]] = ((((((MATRIX[[J, 1, 2]] * MATRIX[[1, K, 1]])
                    + (MATRIX[[J, 2, 2]] * MATRIX[[2, K, 1]]))
                    + (MATRIX[[J, 3, 2]] * MATRIX[[3, K, 1]]))
                    + (MATRIX[[J, 4, 2]] * MATRIX[[4, K, 1]]))
                    + (MATRIX[[J, 5, 2]] * MATRIX[[5, K, 1]]))
                    + (MATRIX[[J, 6, 2]] * MATRIX[[6, K, 1]]));
            }
        }
        //
        // We don't bother to compute the upper right hand 3x3 portion
        // of the matrix since it is always zero.
        //
        // Finally we could copy the lower right hand 3x3 portion of the
        // product from the upper left hand portion.  But as you can
        // see below we never actually have to reference TEMP(I,K,GET)
        // for K = 4 to 6.  So we can just skip that part of the
        // computation.
        //
        //
        // Now continue building the product.  Note we will toggle
        // back and forth from TEMP(,,1) to TEMP(,,2) for storing
        // (PUTting) the results of our computations.  This way we
        // don't have to spend time moving any of the our computation
        // results to get ready for the next product.  See the end
        // of the loop below (keeping mind the next three values) to
        // see the little trick that's used to toggle back and forth.
        //
        INCR = -1;
        PUT = 2;
        GET = 1;

        for I in 3..=(N - 1) {
            //
            // First the upper left hand portion of the product.
            //
            for J in 1..=3 {
                for K in 1..=3 {
                    TEMP[[J, K, PUT]] = (((MATRIX[[J, 1, I]] * TEMP[[1, K, GET]])
                        + (MATRIX[[J, 2, I]] * TEMP[[2, K, GET]]))
                        + (MATRIX[[J, 3, I]] * TEMP[[3, K, GET]]));
                }
            }
            //
            // Next the lower left hand portion of the product.
            //
            for J in 4..=6 {
                for K in 1..=3 {
                    TEMP[[J, K, PUT]] = ((((((MATRIX[[J, 1, I]] * TEMP[[1, K, GET]])
                        + (MATRIX[[J, 2, I]] * TEMP[[2, K, GET]]))
                        + (MATRIX[[J, 3, I]] * TEMP[[3, K, GET]]))
                        + (MATRIX[[J, 4, I]] * TEMP[[4, K, GET]]))
                        + (MATRIX[[J, 5, I]] * TEMP[[5, K, GET]]))
                        + (MATRIX[[J, 6, I]] * TEMP[[6, K, GET]]));
                }
            }
            //
            // And as before, we don't need to compute the upper right
            // or lower right hand 3x3 portions of the matrix. So
            // we just skip them.  Toggle GET and PUT so we will
            // be ready for the next pass.
            //
            GET = PUT;
            PUT = (PUT + INCR);
            INCR = -INCR;
        }

        //
        // Finally compute the last product.  First the upper
        // left hand portion of the product.
        //
        for J in 1..=3 {
            for K in 1..=3 {
                OUTPUT[[J, K]] = (((MATRIX[[J, 1, N]] * TEMP[[1, K, GET]])
                    + (MATRIX[[J, 2, N]] * TEMP[[2, K, GET]]))
                    + (MATRIX[[J, 3, N]] * TEMP[[3, K, GET]]));
            }
        }
        //
        // The lower left hand portion of the product.
        //
        for J in 4..=6 {
            for K in 1..=3 {
                OUTPUT[[J, K]] = ((((((MATRIX[[J, 1, N]] * TEMP[[1, K, GET]])
                    + (MATRIX[[J, 2, N]] * TEMP[[2, K, GET]]))
                    + (MATRIX[[J, 3, N]] * TEMP[[3, K, GET]]))
                    + (MATRIX[[J, 4, N]] * TEMP[[4, K, GET]]))
                    + (MATRIX[[J, 5, N]] * TEMP[[5, K, GET]]))
                    + (MATRIX[[J, 6, N]] * TEMP[[6, K, GET]]));
            }
        }
        //
        // The upper right hand portion of the product is zero.
        //
        for J in 1..=3 {
            for K in 4..=6 {
                OUTPUT[[J, K]] = 0.0;
            }
        }

        //
        // The lower right hand portion of the product is a copy of
        // the upper left hand portion of the product.
        //
        for J in 4..=6 {
            L = (J - 3);
            for K in 4..=6 {
                M = (K - 3);

                OUTPUT[[J, K]] = OUTPUT[[L, M]];
            }
        }
    } else if (N == 1) {
        //
        // If there is only one matrix in the list the output is
        // simply the input.
        //
        for I in 1..=6 {
            for J in 1..=6 {
                OUTPUT[[J, I]] = MATRIX[[J, I, 1]];
            }
        }
    } else if (N <= 0) {
        for J in 1..=6 {
            OUTPUT[[J, J]] = 1.0;

            for K in (J + 1)..=6 {
                OUTPUT[[J, K]] = 0.0;
                OUTPUT[[K, J]] = 0.0;
            }
        }
    }
}
