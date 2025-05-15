//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure T_CHBIGR ( Test utility: integrate Chebyshev expansion  )
pub fn T_CHBIGR(
    NDIM: i32,
    DEGP: i32,
    CP: &[f64],
    X2S: &[f64],
    X: f64,
    WORK: &mut [f64],
    P: &mut [f64],
    ITGRLP: &mut [f64],
    ctx: &mut Context,
) {
    let CP = DummyArray2D::new(CP, 1..=(DEGP + 1), 1..=NDIM);
    let X2S = DummyArray::new(X2S, 1..=2);
    let mut WORK = DummyArrayMut2D::new(WORK, 1..=(DEGP + 2), 1..=2);
    let mut P = DummyArrayMut::new(P, 1..=NDIM);
    let mut ITGRLP = DummyArrayMut::new(ITGRLP, 1..=NDIM);
    let mut MIDPNT: f64 = 0.0;
    let mut RADIUS: f64 = 0.0;
    let mut X0: f64 = 0.0;
    let mut N: i32 = 0;
    let mut NTERMS: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return;
    }

    NTERMS = (DEGP + 1);

    //
    // Transform the independent variable X to the interval
    //
    //    [-1, 1]
    //
    // Call the result X0.
    //

    MIDPNT = X2S[1];
    RADIUS = X2S[2];

    X0 = ((X - MIDPNT) / RADIUS);

    //
    // Calculate the values of the Chebyshev polynomials
    //
    //    T  ... T
    //     0      DEGP
    //
    // at X0. Store these values in WORK(*,1). Also evaluate these
    // polynomials at 0 and store the values in WORK(*,2).
    //

    WORK[[1, 1]] = 1.0;
    WORK[[2, 1]] = X0;

    WORK[[1, 2]] = 1.0;
    WORK[[2, 2]] = 0.0;

    for I in 3..=(NTERMS + 1) {
        //
        // Compute
        //
        //    T   ( X0 )
        //     I-1
        //
        // and
        //
        //    T
        //     I-1 ( 0 )
        //
        //
        WORK[[I, 1]] = (-WORK[[(I - 2), 1]] + (((2 as f64) * X0) * WORK[[(I - 1), 1]]));

        WORK[[I, 2]] = -WORK[[(I - 2), 2]];
    }

    //
    // Compute the definite integral for each expansion. Evaluate each
    // expansion as well.
    //
    for DIX in 1..=NDIM {
        ITGRLP[DIX] = 0.0;
        P[DIX] = 0.0;
        //
        // The integrals of the remaining terms are obtained from the
        // formula
        //
        //                              1             1
        //    [I (T )](x) =  (1/2) * ( --- T   (x) - --- T   (x) )
        //         n                   n+1  n+1      n-1  n-1
        //
        //                             1              1
        //                 - (1/2) * ( --- T   (0) - --- T   (0) )
        //                             n+1  n+1      n-1  n-1
        //
        //
        for I in intrinsics::range(NTERMS, 3, -1) {
            //
            // Add on the next term of the expansion.
            //
            P[DIX] = (P[DIX] + (CP[[I, DIX]] * WORK[[I, 1]]));

            //
            // Add on the next term of the integral.
            //
            N = (I - 1);

            ITGRLP[DIX] = (ITGRLP[DIX]
                + (CP[[I, DIX]]
                    * (((0.5 / (N + 1) as f64) * (WORK[[(N + 2), 1]] - WORK[[(N + 2), 2]]))
                        - ((0.5 / (N - 1) as f64) * (WORK[[N, 1]] - WORK[[N, 2]])))));
        }

        if (NTERMS >= 2) {
            //
            // Add on the first two terms of the expansion.
            //
            P[DIX] = ((P[DIX] + CP[[1, DIX]]) + (CP[[2, DIX]] * X0));

            //
            // Add together the first two terms of the integral.
            //
            ITGRLP[DIX] = (ITGRLP[DIX] + (X0 * (CP[[1, DIX]] + ((CP[[2, DIX]] * X0) / 2 as f64))));
        } else {
            //
            // The integrand is constant.
            //
            P[DIX] = (P[DIX] + CP[[1, DIX]]);

            ITGRLP[DIX] = (ITGRLP[DIX] + (X0 * CP[[1, DIX]]));
        }
    }

    //
    // Scale the integral to account for the change of variables
    // (from the original domain to [1-,1]).
    //
    for I in 1..=NDIM {
        ITGRLP[I] = (RADIUS * ITGRLP[I]);
    }
}
