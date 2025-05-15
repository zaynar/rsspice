//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXDEG: i32 = 35;

struct SaveVars {
    MINV: ActualArray2D<f64>,
    PRVDEG: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MINV = ActualArray2D::<f64>::new(1..=MAXDEG, 1..=MAXDEG);
        let mut PRVDEG: i32 = 0;

        PRVDEG = -1;

        Self { MINV, PRVDEG }
    }
}

//$Procedure  T_PD2DT ( Polynomial derivatives to difference table )
pub fn T_PD2DT(
    DEGP: i32,
    PDERIV: &mut [f64],
    WORK: &mut [f64],
    DIFTAB: &mut [f64],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PDERIV = DummyArrayMut::new(PDERIV, 0..=DEGP);
    let mut WORK = DummyArrayMut2D::new(WORK, 0..=(DEGP - 1), 0..=(DEGP - 1));
    let mut DIFTAB = DummyArrayMut::new(DIFTAB, 0..=DEGP);
    let mut FACT: f64 = 0.0;
    let mut DEGPSQ: i32 = 0;
    let mut SIGN: i32 = 0;
    let mut USEMNV: bool = false;

    //

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // The first element of the difference table is just the
    // constant term from the polynomial.
    //
    DIFTAB[0] = PDERIV[0];

    //
    // Use the work space array to build the mystery matrix
    //
    //     -1
    //    M
    //
    //
    // The (j,k) entry of the mystery matrix is
    //
    //
    //        k-j
    //    (-1)    * ( <k choose j> )
    //
    //
    // for k >= j.  The lower triangular portion of the matrix is
    // identically zero.
    //
    if ((DEGP == save.PRVDEG) && (DEGP <= MAXDEG)) {
        //
        // We can use the previously computed matrix MINV.
        //
        USEMNV = true;
    } else {
        DEGPSQ = (DEGP * DEGP);

        spicelib::CLEARD(DEGPSQ, WORK.as_slice_mut());

        for J in 0..=(DEGP - 1) {
            SIGN = 1;

            for K in J..=(DEGP - 1) {
                WORK[[J, K]] = (SIGN * T_CHOOSE(K, J)) as f64;

                SIGN = -SIGN;
            }
        }

        if (DEGP <= MAXDEG) {
            spicelib::MOVED(WORK.as_slice(), DEGPSQ, save.MINV.as_slice_mut());
        }

        USEMNV = false;
    }

    //
    // Divide the jth element of PDERIV by j!.
    //
    FACT = 1 as f64;

    for J in 1..=DEGP {
        FACT = ((J as f64) * FACT);

        PDERIV[J] = (PDERIV[J] / FACT);
    }

    //
    // Now left-multiply our vector of polynomial derivatives by
    // the mystery matrix.
    //
    if USEMNV {
        spicelib::MXVG(
            save.MINV.as_slice(),
            PDERIV.subarray(1),
            DEGP,
            DEGP,
            DIFTAB.subarray_mut(1),
        );
    } else {
        spicelib::MXVG(
            WORK.as_slice(),
            PDERIV.subarray(1),
            DEGP,
            DEGP,
            DIFTAB.subarray_mut(1),
        );
    }

    save.PRVDEG = DEGP;
}
