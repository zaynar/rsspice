//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const MSGLEN: i32 = 240;
const NRANDM: i32 = 1000;

//$Procedure F_NPLNPT ( NPLNPT tests )
pub fn F_NPLNPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut LINEDR = StackArray::<f64, 3>::new(1..=3);
    let mut LINEPT = StackArray::<f64, 3>::new(1..=3);
    let mut LTRANS = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut POINT = StackArray::<f64, 3>::new(1..=3);
    let mut PTRANS = StackArray::<f64, 3>::new(1..=3);
    let mut SFACTR: f64 = 0.0;
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut XDIST: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut XTRANS = StackArray::<f64, 3>::new(1..=3);
    let mut SEED: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_NPLNPT", ctx)?;

    //
    //     Run some simple tests where the correct results can be
    //     determined by inspection.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"NPLNPT simple case:  line is x = y; z = 0.  POINT is  (-1, 0, 0 ).",
    );
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(1.0, 1.0, 0.0, LINEDR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, LINEPT.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, POINT.as_slice_mut());

    A = (f64::sqrt(2.0) / 2 as f64);

    spicelib::VPACK(-0.5, -0.5, 0.0, XPT.as_slice_mut());

    spicelib::NPLNPT(
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        POINT.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test the near point and distance.
    //
    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~/",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", DIST, b"~", A, TIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"NPLNPT non-error exception case:  POINT is on the line.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(1.0, 1.0, 0.0, LINEDR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, LINEPT.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, POINT.as_slice_mut());

    spicelib::VPACK(-1.0, 0.0, 0.0, XPT.as_slice_mut());

    spicelib::NPLNPT(
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        POINT.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test the near point and distance.
    //
    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~/",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"DIST", DIST, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // Now for some more difficult cases.  We'll generate the points
    // and lines using random numbers.  There are ten components to
    // generate:
    //
    //    - random line direction vectors
    //    - random line points
    //    - random points off the line
    //    - random scale factors for the line and point; these are
    //      used to create a wide range of scales
    //

    SEED = -1;

    for I in 1..=NRANDM {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"NPLNPT random case #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Get a scale factor.
        //
        SFACTR = f64::powf(10.0, testutil::T_RANDD(-306.0, 306.0, &mut SEED, ctx)?);

        //
        // We gotta have a line direction vector.
        //
        LINEDR[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEDR[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEDR[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        spicelib::VHATIP(LINEDR.as_slice_mut());

        //
        // We need a point on the line.
        //
        LINEPT[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEPT[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEPT[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // We also need a point off the line.  Scale the point up to
        // increase the likelihood of a non-intercept case.
        //
        POINT[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        POINT[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        POINT[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // Find the expected near point.  First define a frame whose
        // x-axis is parallel to LINEDR.  Then transform POINT
        // into this frame. Also transform LINEPT into the same frame.
        //
        spicelib::VEQU(LINEDR.as_slice(), X.as_slice_mut());

        spicelib::FRAME(X.as_slice_mut(), Y.as_slice_mut(), Z.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=3 {
            TRANS[[1, J]] = X[J];
            TRANS[[2, J]] = Y[J];
            TRANS[[3, J]] = Z[J];
        }

        spicelib::MXV(TRANS.as_slice(), POINT.as_slice(), PTRANS.as_slice_mut());
        spicelib::MXV(TRANS.as_slice(), LINEPT.as_slice(), LTRANS.as_slice_mut());

        //
        // The y-z projection of the resulting point is the closest point
        // on the line to the origin. The expected near point has the
        // same x-component as PTRANS.
        //
        spicelib::VPACK(PTRANS[1], LTRANS[2], LTRANS[3], XTRANS.as_slice_mut());

        //
        // Map XTRANS back to the base frame.
        //
        spicelib::MTXV(TRANS.as_slice(), XTRANS.as_slice(), XPT.as_slice_mut());

        //
        // The distance between XPT and POINT is the expected
        // distance from the near point.
        //
        XDIST = spicelib::VDIST(XPT.as_slice(), POINT.as_slice());

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NPLNPT(
            LINEPT.as_slice(),
            LINEDR.as_slice(),
            POINT.as_slice(),
            PNEAR.as_slice_mut(),
            &mut DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Test the near point and distance.
        //
        testutil::CHCKAD(
            b"PNEAR",
            PNEAR.as_slice(),
            b"~~/",
            XPT.as_slice(),
            3,
            TIGHT,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(b"DIST", DIST, b"~/", XDIST, TIGHT, OK, ctx)?;
    }

    //
    //     NPLNPT error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NPLNPT: zero direction vector,", ctx)?;

    spicelib::FILLD(10.0, 3, LINEPT.as_slice_mut());
    spicelib::CLEARD(3, LINEDR.as_slice_mut());

    spicelib::VPACK(1.0, 1.0, 1.0, POINT.as_slice_mut());

    spicelib::NPLNPT(
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        POINT.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
