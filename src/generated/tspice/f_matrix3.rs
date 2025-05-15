//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const VTIGHT: f64 = 0.000000000000001;
const SEED0: i32 = 19920810;
const N: i32 = 3;
const NE: i32 = (N * N);

//$Procedure F_MATRIX3 (Family of tests on 3x3-matrix operations)
pub fn F_MATRIX3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut INDX: i32 = 0;
    let mut SEED: i32 = 0;
    let mut VINDX = StackArray::<f64, 3>::new(1..=N);
    let mut VRAN = StackArray::<f64, 3>::new(1..=N);
    let mut VOUT = StackArray::<f64, 3>::new(1..=N);
    let mut VEXPT = StackArray::<f64, 3>::new(1..=N);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=N);
    let mut MZERO = StackArray2D::<f64, 9>::new(1..=N, 1..=N);
    let mut MRAN = StackArray2D::<f64, 9>::new(1..=N, 1..=N);
    let mut MINDX = StackArray2D::<f64, 9>::new(1..=N, 1..=N);
    let mut MOUT = StackArray2D::<f64, 9>::new(1..=N, 1..=N);
    let mut EVAL: f64 = 0.0;
    let mut EXPT: f64 = 0.0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Initial seed number for pseudo-random number generation
    //

    //
    // Dimension of 3-by-3 square matrices
    //

    //
    // Number of elements in an N-by-N matrix.
    //

    //
    // Local Variables
    //

    //
    // indices
    //

    //
    // Seed number for pseudo-random number generation
    //

    //
    // 3-component vectors
    //

    //
    // 3-by-3 matrices
    //

    //
    // Evaluated number from a library routine.
    //

    //
    // Expected value of EVAL
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_MATRIX3", ctx)?;

    //
    // Form arrays to be used for more than one test case.
    //

    //
    // A zero matrix
    //
    for J in 1..=N {
        for I in 1..=N {
            MZERO[[I, J]] = 0.0;
        }
    }

    //
    // An index vector, whose components are their indices
    //
    for I in 1..=N {
        VINDX[I] = (I as f64);
    }

    //
    // An index matrix, whose elements are their column-wise
    // storage indices
    //
    INDX = 0;
    for J in 1..=N {
        for I in 1..=N {
            INDX = (INDX + 1);
            MINDX[[I, J]] = (INDX as f64);
        }
    }

    //
    // Initialize the seed number of random number generation.
    //
    SEED = SEED0;

    //
    // Form a random vector, whose components are
    // uniformly distributed on the interval [-1.0D0, +1.0D0].
    //

    for I in 1..=N {
        VRAN[I] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    }

    //
    // Form a random matrix, whose elements are
    // uniformly distributed on the interval [-1.0D0, +1.0D0].
    //
    for J in 1..=N {
        for I in 1..=N {
            MRAN[[I, J]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    //
    // Cases to test MEQU
    //

    testutil::TCASE(b"MEQU: copying a 3x3 zero matrix and checking strict equality between the zero matrix and the copied matrix", ctx)?;

    spicelib::MEQU(MZERO.as_slice(), MOUT.as_slice_mut());

    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"=",
        MZERO.as_slice(),
        NE,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"MEQU: copying a 3x3 random matrix and checking strict equality between the random matrix and the copied matrix", ctx)?;

    spicelib::MEQU(MRAN.as_slice(), MOUT.as_slice_mut());

    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"=",
        MRAN.as_slice(),
        NE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test XPOSE
    //
    testutil::TCASE(b"XPOSE: generating the transpose of an index matrix, and checking equality between original and transposed elements", ctx)?;

    spicelib::XPOSE(MINDX.as_slice(), MOUT.as_slice_mut());

    INDX = 0;
    for J in 1..=N {
        for I in 1..=N {
            INDX = (INDX + 1);
            EXPT = (INDX as f64);
            testutil::CHCKSD(b"MOUT(J,I)", MOUT[[J, I]], b"=", EXPT, 0.0, OK, ctx)?;
        }
    }

    //

    testutil::TCASE(b"XPOSE: generating the transpose of a random matrix, and checking equality between original and transposed elements", ctx)?;

    spicelib::XPOSE(MRAN.as_slice(), MOUT.as_slice_mut());

    for J in 1..=N {
        for I in 1..=N {
            testutil::CHCKSD(b"MOUT(J,I)", MOUT[[J, I]], b"=", MRAN[[I, J]], 0.0, OK, ctx)?;
        }
    }

    //
    // Cases to test TRACE
    //

    testutil::TCASE(b"TRACE: an index matrix, whose trace is 15.0D0", ctx)?;

    EVAL = spicelib::TRACE(MINDX.as_slice());

    EXPT = 15.0;

    testutil::CHCKSD(b"EVAL", EVAL, b"=", EXPT, 0.0, OK, ctx)?;

    testutil::TCASE(b"TRACE: random matrix", ctx)?;

    EVAL = spicelib::TRACE(MRAN.as_slice());

    EXPT = 0.0;
    for I in 1..=N {
        EXPT = (EXPT + MRAN[[I, I]]);
    }

    testutil::CHCKSD(b"EVAL", EVAL, b"=", EXPT, 0.0, OK, ctx)?;

    //
    // Cases to test DET
    //

    testutil::TCASE(b"DET: random matrix", ctx)?;

    EVAL = spicelib::DET(MRAN.as_slice());

    //
    // The determinant of a 3x3x matrix can be computed by adding three
    // up diagonal followed by subtracting three down-diagonal products.
    //
    EXPT = (((((((MRAN[[1, 1]] * MRAN[[2, 2]]) * MRAN[[3, 3]])
        + ((MRAN[[1, 2]] * MRAN[[2, 3]]) * MRAN[[3, 1]]))
        + ((MRAN[[1, 3]] * MRAN[[2, 1]]) * MRAN[[3, 2]]))
        - ((MRAN[[1, 1]] * MRAN[[2, 3]]) * MRAN[[3, 2]]))
        - ((MRAN[[1, 2]] * MRAN[[2, 1]]) * MRAN[[3, 3]]))
        - ((MRAN[[1, 3]] * MRAN[[2, 2]]) * MRAN[[3, 1]]));

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, VTIGHT, OK, ctx)?;

    //
    // Cases to test MXV
    //
    testutil::TCASE(b"MXV: random matrix and random vector", ctx)?;

    spicelib::MXV(MRAN.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());

    for I in 1..=N {
        for J in 1..=N {
            VTEMP[J] = MRAN[[I, J]];
        }
        VEXPT[I] = spicelib::VDOT(VTEMP.as_slice(), VRAN.as_slice());
    }

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VEXPT.as_slice(),
        N,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MTXV
    //
    testutil::TCASE(b"MTXV: random matrix and random vector", ctx)?;

    spicelib::MTXV(MRAN.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());

    for I in 1..=N {
        for J in 1..=N {
            VTEMP[J] = MRAN[[J, I]];
        }
        VEXPT[I] = spicelib::VDOT(VTEMP.as_slice(), VRAN.as_slice());
    }

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"=",
        VEXPT.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test VTMV
    //
    testutil::TCASE(b"VTMV: random vector, random matrix, index vector ", ctx)?;

    EVAL = spicelib::VTMV(VRAN.as_slice(), MRAN.as_slice(), VINDX.as_slice());

    //
    // First compute the product of MRAN and VINDX.
    //
    spicelib::MXV(MRAN.as_slice(), VINDX.as_slice(), VTEMP.as_slice_mut());

    //
    // Then, compute the expected value using the routine VDOT.
    //
    EXPT = spicelib::VDOT(VRAN.as_slice(), VTEMP.as_slice());

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test MXM
    //
    testutil::TCASE(b"MXM: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = MRAN * MINDX.
    //
    spicelib::MXM(MRAN.as_slice(), MINDX.as_slice(), MOUT.as_slice_mut());

    //
    // Check MRAN * ( MINDX * VRAN ) = MOUT * VRAN, where VRAN is
    // an arbitrary vector.
    //
    spicelib::MXV(MINDX.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());
    spicelib::MXV(MRAN.as_slice(), VOUT.as_slice(), VTEMP.as_slice_mut());
    spicelib::MXV(MOUT.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MTXM
    //
    testutil::TCASE(b"MTXM: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = Transpose(MRAN) * MINDX.
    //
    spicelib::MTXM(MRAN.as_slice(), MINDX.as_slice(), MOUT.as_slice_mut());

    //
    // Check Transpose(MRAN) * ( MINDX * VRAN ) = MOUT * VRAN,
    // where VRAN is an arbitrary vector.
    //
    spicelib::MXV(MINDX.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());
    spicelib::MTXV(MRAN.as_slice(), VOUT.as_slice(), VTEMP.as_slice_mut());
    spicelib::MXV(MOUT.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MXMT
    //
    testutil::TCASE(b"MXMT: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = MRAN * Transpose(MINDX).
    //
    spicelib::MXMT(MRAN.as_slice(), MINDX.as_slice(), MOUT.as_slice_mut());

    //
    // Check MRAN * ( Transpose(MINDX) * VRAN ) = MOUT * VRAN,
    // where VRAN is an arbitrary vector.
    //
    spicelib::MTXV(MINDX.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());
    spicelib::MXV(MRAN.as_slice(), VOUT.as_slice(), VTEMP.as_slice_mut());
    spicelib::MXV(MOUT.as_slice(), VRAN.as_slice(), VOUT.as_slice_mut());

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        N,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
