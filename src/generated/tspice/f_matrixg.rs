//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const SEED0: i32 = 20140416;
const NR: i32 = 7;
const NC: i32 = 6;
const N: i32 = 7;
const NE: i32 = (NR * NC);

//$Procedure F_MATRIXG (Family of tests on general matrix operations)
pub fn F_MATRIXG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MRAN = StackArray2D::<f64, 42>::new(1..=NR, 1..=NC);
    let mut MNX1 = StackArray2D::<f64, 7>::new(1..=N, 1..=1);
    let mut M1XN = StackArray2D::<f64, 7>::new(1..=1, 1..=N);
    let mut MOUT = StackArray2D::<f64, 42>::new(1..=NR, 1..=NC);
    let mut XPOSEM = StackArray2D::<f64, 42>::new(1..=NC, 1..=NR);
    let mut MATSQ = StackArray2D::<f64, 49>::new(1..=N, 1..=N);
    let mut MNRN = StackArray2D::<f64, 49>::new(1..=NR, 1..=N);
    let mut MNNC = StackArray2D::<f64, 42>::new(1..=N, 1..=NC);
    let mut MNNR = StackArray2D::<f64, 49>::new(1..=N, 1..=NR);
    let mut MNCN = StackArray2D::<f64, 42>::new(1..=NC, 1..=N);
    let mut VCOL = StackArray::<f64, 6>::new(1..=NC);
    let mut VROW = StackArray::<f64, 7>::new(1..=NR);
    let mut VOUT = StackArray::<f64, 7>::new(1..=N);
    let mut VTEMP = StackArray::<f64, 7>::new(1..=N);
    let mut VEXPT = StackArray::<f64, 7>::new(1..=N);
    let mut SEED: i32 = 0;
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
    // Number of rows in an arbitrary matrix
    //

    //
    // Number of columns in an arbitrary matrix
    //

    //
    // Number of rows (or columns) of a square matrix.
    // Should be set to MAX( NR, NC ).
    //

    //
    // Number of elements in an NR-by-NC matrix
    //

    //
    // Local Variables
    //

    //
    // An NR-by-NC pseudo-random matrix
    //

    //
    // A single-column matrix
    //

    //
    // A single-row matrix
    //

    //
    // Matrices having the same dimension as that of MRAN
    //

    //
    // Transpose of MRAN
    //

    //
    // An N-by-N matrix
    //

    //
    // Some other work space matrices
    //

    //
    // A column vector
    //

    //
    // A row vector
    //

    //
    // Some other work space vectors
    //

    //
    // seed number for pseudo-random number generation
    //

    //
    // An index
    //

    //
    // A row index
    //

    //
    // A column index
    //

    //
    // Some double precision numbers
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_MATRIXG", ctx)?;

    //
    // Form arrays to be used for more than one test case.
    //

    //
    // Arbitrary matrices, whose components are pseudo-random
    // numbers uniformly distributed on the interval [-1.0D0,+1.0D0].
    //
    SEED = SEED0;

    for IC in 1..=NC {
        for IR in 1..=NR {
            MRAN[[IR, IC]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    for I in 1..=N {
        for IR in 1..=NR {
            MNRN[[IR, I]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    for IC in 1..=NC {
        for I in 1..=N {
            MNNC[[I, IC]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    for I in 1..=NR {
        for IR in 1..=N {
            MNNR[[IR, I]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    for IC in 1..=N {
        for I in 1..=NC {
            MNCN[[I, IC]] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        }
    }

    //
    // A single-row matrix, whose elements are their column index
    //
    for I in 1..=N {
        M1XN[[1, I]] = (I as f64);
    }

    //
    // A square index matrix, whose components are storage indices
    //
    EVAL = 0.0;

    for IC in 1..=N {
        for IR in 1..=N {
            EVAL = (EVAL + 1.0);
            MATSQ[[IR, IC]] = EVAL;
        }
    }

    //
    // Arbitrary vectors, whose components are pseudo-random
    // numbers uniformly distributed on the interval [-1.0D0,+1.0D0]
    //
    for IC in 1..=NC {
        VCOL[IC] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    }

    for IR in 1..=NR {
        VROW[IR] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
    }

    //
    // Cases to test MEGUG
    //

    testutil::TCASE(b"MEQUG: arbitrary matrix", ctx)?;

    spicelib::MEQUG(MRAN.as_slice(), NR, NC, MOUT.as_slice_mut());

    //
    // Numerical tolerance for this test should be zero.
    //
    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"~",
        MRAN.as_slice(),
        NE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test XPOSEG
    //

    testutil::TCASE(b"XPOSEG: 1-by-N matrix", ctx)?;

    spicelib::XPOSEG(M1XN.as_slice(), 1, N, MNX1.as_slice_mut());

    //
    // Numerical tolerance for this test should be zero.
    //
    testutil::CHCKAD(
        b"MNX1",
        MNX1.as_slice(),
        b"~",
        M1XN.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"XPOSEG: N-by-1 matrix", ctx)?;

    spicelib::XPOSEG(MNX1.as_slice(), N, 1, M1XN.as_slice_mut());

    //
    // Numerical tolerance for this test should be zero.
    //
    testutil::CHCKAD(
        b"M1XN",
        M1XN.as_slice(),
        b"~",
        MNX1.as_slice(),
        N,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"XPOSEG: arbitrary matrix", ctx)?;

    spicelib::XPOSEG(MRAN.as_slice(), NR, NC, XPOSEM.as_slice_mut());

    //
    // Check the equality of M = ( M + Transpose( Transpose( M ) ) ) / 2
    // for an arbitrary matrix M ( that is MRAN herein).
    //
    for IC in 1..=NC {
        for IR in 1..=NR {
            MOUT[[IR, IC]] = ((MRAN[[IR, IC]] + XPOSEM[[IC, IR]]) / 2.0);
        }
    }

    //
    // Numerical tolerance for this test should be zero.
    //
    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"~",
        MRAN.as_slice(),
        NE,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"XPOSEG: applying twice.", ctx)?;

    spicelib::XPOSEG(MRAN.as_slice(), NR, NC, XPOSEM.as_slice_mut());

    spicelib::XPOSEG(XPOSEM.as_slice(), NC, NR, MOUT.as_slice_mut());

    //
    // Numerical tolerance for this test should be zero.
    //
    testutil::CHCKAD(
        b"MOUT",
        MOUT.as_slice(),
        b"~",
        MRAN.as_slice(),
        NE,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Cases to test TRACEG
    //
    testutil::TCASE(b"TRACEG: square integer matrix", ctx)?;

    EVAL = spicelib::TRACEG(MATSQ.as_slice(), N);

    EXPT = 0.0;
    for I in 1..=N {
        EXPT = (EXPT + MATSQ[[I, I]]);
    }

    //
    // Being MATSQ an integer matrix, numerical tolerance for this test
    // should be zero.
    //
    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, 0.0, OK, ctx)?;

    //
    // Cases to test MXVG
    //
    testutil::TCASE(
        b"MXVG: multiplying an arbitrary matrix and an arbitrary column vector.",
        ctx,
    )?;

    spicelib::MXVG(
        MRAN.as_slice(),
        VCOL.as_slice(),
        NR,
        NC,
        VOUT.as_slice_mut(),
    );

    for IR in 1..=NR {
        for IC in 1..=NC {
            VTEMP[IC] = MRAN[[IR, IC]];
        }
        VEXPT[IR] = spicelib::VDOTG(VTEMP.as_slice(), VCOL.as_slice(), NC);
    }

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VEXPT.as_slice(),
        NR,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MTXVG
    //
    testutil::TCASE(
        b"MTXVG: multiplying the transpose of an arbitrary matrix and an arbitrary vector",
        ctx,
    )?;

    spicelib::MTXVG(
        MRAN.as_slice(),
        VROW.as_slice(),
        NC,
        NR,
        VOUT.as_slice_mut(),
    );

    for IC in 1..=NC {
        for IR in 1..=NR {
            VTEMP[IR] = MRAN[[IR, IC]];
        }
        VEXPT[IC] = spicelib::VDOTG(VTEMP.as_slice(), VROW.as_slice(), NR);
    }

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VEXPT.as_slice(),
        NC,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test VTMVG
    //
    testutil::TCASE(b"VTMVG: ", ctx)?;

    EVAL = spicelib::VTMVG(VROW.as_slice(), MRAN.as_slice(), VCOL.as_slice(), NR, NC);

    EXPT = 0.0;
    for IC in 1..=NC {
        for IR in 1..=NR {
            EXPT = (EXPT + ((VROW[IR] * MRAN[[IR, IC]]) * VCOL[IC]));
        }
    }

    testutil::CHCKSD(b"EVAL", EVAL, b"~", EXPT, TIGHT, OK, ctx)?;

    //
    // Cases to test MXMG
    //
    testutil::TCASE(b"MXMG: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = MNRN * MNNC.
    //
    spicelib::MXMG(
        MNRN.as_slice(),
        MNNC.as_slice(),
        NR,
        N,
        NC,
        MOUT.as_slice_mut(),
    );

    //
    // Check MNRN * ( MNNC * VCOL ) = MOUT * VCOL, where VCOL is
    // an arbitrary vector.
    //
    spicelib::MXVG(MNNC.as_slice(), VCOL.as_slice(), N, NC, VOUT.as_slice_mut());
    spicelib::MXVG(
        MNRN.as_slice(),
        VOUT.as_slice(),
        NR,
        N,
        VTEMP.as_slice_mut(),
    );
    spicelib::MXVG(
        MOUT.as_slice(),
        VCOL.as_slice(),
        NR,
        NC,
        VOUT.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        NR,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MTXMG
    //
    testutil::TCASE(b"MTXMG: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = Transpose(MNNR) * MNNC.
    //
    spicelib::MTXMG(
        MNNR.as_slice(),
        MNNC.as_slice(),
        NR,
        N,
        NC,
        MOUT.as_slice_mut(),
    );

    //
    // Check Transpose(MNNR) * ( MNNC * VCOL ) = MOUT * VCOL,
    // where VCOL is an arbitrary vector.
    //
    spicelib::MXVG(MNNC.as_slice(), VCOL.as_slice(), N, NC, VOUT.as_slice_mut());
    spicelib::MTXVG(
        MNNR.as_slice(),
        VOUT.as_slice(),
        NR,
        N,
        VTEMP.as_slice_mut(),
    );
    spicelib::MXVG(
        MOUT.as_slice(),
        VCOL.as_slice(),
        NR,
        NC,
        VOUT.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        NR,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Cases to test MXMTG
    //
    testutil::TCASE(b"MXMTG: Freivald\'s Algorithm", ctx)?;

    //
    // Compute the product MOUT = MNRN * Transpose(MNCN).
    //
    spicelib::MXMTG(
        MNRN.as_slice(),
        MNCN.as_slice(),
        NR,
        N,
        NC,
        MOUT.as_slice_mut(),
    );

    //
    // Check MNRN * ( Transpose(MNCN) * VCOL ) = MOUT * VCOL,
    // where VCOL is an arbitrary vector.
    //
    spicelib::MTXVG(MNCN.as_slice(), VCOL.as_slice(), N, NC, VOUT.as_slice_mut());
    spicelib::MXVG(
        MNRN.as_slice(),
        VOUT.as_slice(),
        NR,
        N,
        VTEMP.as_slice_mut(),
    );
    spicelib::MXVG(
        MOUT.as_slice(),
        VCOL.as_slice(),
        NR,
        NC,
        VOUT.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"VOUT",
        VOUT.as_slice(),
        b"~",
        VTEMP.as_slice(),
        NR,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::T_SUCCESS(OK, ctx);

    //
    // Tests for zero/negative NR/NC inputs.
    //
    // THERE CAN BE NO SUCH TESTS BECAUSE OF THE WAY INPUT AND
    // OUTPUT MATRICES ARE DECLARED IN THESE ROUTINES.
    //

    Ok(())
}
