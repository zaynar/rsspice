//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const UBEL: i32 = 9;
const MSGLEN: i32 = 400;
const NRANDM: i32 = 5000;

//$Procedure F_NPELPT ( NPELPT tests )
pub fn F_NPELPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut POINT = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut SFACTR: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut XDIST: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut SEED: i32 = 0;
    let mut VALPT: bool = false;

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
    // In this test family, we encapsulate some of the geometric
    // tests within functions defined in this file. The function
    //
    //    T_ISNPEL
    //
    // tests whether a 3-dpoint is on a specified ellipse embedded in
    // 3-dimensional space:
    //
    //    - A point P = (p1,p2,p3) is "on" the ellipse with center C
    //      and semi-axes SMAJOR and SMINOR if P lies in the plane
    //      of the ellipse and if the "level surface parameter"
    //
    //         lambda =  x**2/A**2 + y**2/B**2
    //
    //      is sufficiently close to 1, where
    //
    //         A = ||SMAJOR||
    //         B = ||SMINOR||
    //
    //         x = <P-C, SMAJOR/A> * SMAJOR / A
    //         y = <P-C, SMINOR/B> * SMINOR / B
    //
    //
    // The function
    //
    //    T_ISNPEL
    //
    // test whether the near point found by NPELPT has the property
    // that the outward normal at the near point can be extended to
    // intersect the input line at right angles.
    //
    // T_ISNPEL also checks the distance of the near point from the line:
    // the distance is checked against the value obtained from NPLNPT.
    //
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_NPELPT", ctx)?;

    //
    //     Run some simple tests where the correct results can be
    //     determined by inspection.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Ellipse is unit circle in x-y plane.  Point is (1,0,1)",
    );
    spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(1.0, 0.0, 1.0, POINT.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, V1.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, V2.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, CENTER.as_slice_mut());
    spicelib::CGV2EL(
        CENTER.as_slice(),
        V1.as_slice(),
        V2.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::NPELPT(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test results.
    //
    spicelib::VEQU(V1.as_slice(), XPT.as_slice_mut());
    XDIST = 1.0;

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
    testutil::CHCKSD(b"DIST", DIST, b"~", XDIST, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Ellipse is unit circle in z=5 plane.  Point is (4,0,1)",
    );
    spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(4.0, 0.0, 1.0, POINT.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, V1.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, V2.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 5.0, CENTER.as_slice_mut());
    spicelib::CGV2EL(
        CENTER.as_slice(),
        V1.as_slice(),
        V2.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::NPELPT(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test results.
    //
    spicelib::VPACK(1.0, 0.0, 5.0, XPT.as_slice_mut());
    XDIST = 5.0;

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
    testutil::CHCKSD(b"DIST", DIST, b"~", XDIST, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut TITLE,
        b"Ellipse is unit circle in z=5 plane.  Point is (0,0,5)",
    );
    spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::VPACK(0.0, 0.0, 5.0, POINT.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, V1.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, V2.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 5.0, CENTER.as_slice_mut());
    spicelib::CGV2EL(
        CENTER.as_slice(),
        V1.as_slice(),
        V2.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::NPELPT(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test results.
    //
    // For this case, we don't know which point will be selected
    // as the near point.  Just check that it's a valid solution.
    //
    //
    VALPT = T_ISNPEL(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice(),
        DIST,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"Is near point on ellipse?", VALPT, true, OK, ctx)?;

    //
    // Now for some more difficult cases.  We'll generate the ellipses
    // and planes using random numbers.  There are four components to
    // generate:
    //
    //    - random points
    //    - random ellipse axes
    //    - random ellipse centers
    //    - random scale factors for the ellipse and point; these are
    //      used to create a wide range of scales
    //

    SEED = -1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NRANDM;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // --- Case: ------------------------------------------------------
            //

            //
            // Get a scale factor.
            //
            SFACTR = f64::powf(10.0, testutil::T_RANDD(-290.0, 290.0, &mut SEED, ctx)?);

            //
            // Make up ellipse vectors.
            //
            for J in 1..=3 {
                CENTER[J] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
                V1[J] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
                V2[J] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
            }

            //
            // Pack the ellipse.
            //
            spicelib::CGV2EL(
                CENTER.as_slice(),
                V1.as_slice(),
                V2.as_slice(),
                ELLIPS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Find the semi-axes of this ellipse.
            //
            spicelib::SAELGV(
                V1.as_slice(),
                V2.as_slice(),
                SMAJOR.as_slice_mut(),
                SMINOR.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We also need a point.
            //
            POINT[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
            POINT[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
            POINT[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

            fstr::assign(&mut TITLE, b"NPELPT Random case #.  CENTER = (#, #, #); V1 = (#, #, #); V2 = (#, #, #); POINT = (#, #, #)");

            spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", CENTER[1], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", CENTER[2], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", CENTER[3], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V1[1], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V1[2], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V1[3], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V2[1], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V2[2], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", V2[3], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", POINT[1], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", POINT[2], 14, &mut TITLE, ctx);
            spicelib::REPMD(&TITLE.clone(), b"#", POINT[3], 14, &mut TITLE, ctx);

            testutil::TCASE(&TITLE, ctx)?;
            //
            // Cross our fingers and toes and let 'er rip.
            //
            spicelib::NPELPT(
                POINT.as_slice(),
                ELLIPS.as_slice(),
                PNEAR.as_slice_mut(),
                &mut DIST,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure point we found lies on the ellipse.
            //
            VALPT = T_ISNPEL(
                POINT.as_slice(),
                ELLIPS.as_slice(),
                PNEAR.as_slice(),
                DIST,
                OK,
                ctx,
            )?;

            testutil::CHCKSL(b"Is near point on ellipse?", VALPT, true, OK, ctx)?;

            I += m3__;
        }
    }

    //
    // NPELPT error cases:
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NPELPT: ellipse has one zero-length axis.", ctx)?;

    spicelib::FILLD(10.0, 3, POINT.as_slice_mut());

    spicelib::VPACK(0.0, 0.0, 0.0, V1.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, V2.as_slice_mut());
    spicelib::VPACK(2.0, 2.0, 2.0, CENTER.as_slice_mut());
    spicelib::CGV2EL(
        CENTER.as_slice(),
        V1.as_slice(),
        V2.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::NPELPT(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Swap the semi-axes and repeat.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        V2.as_slice(),
        V1.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;
    spicelib::NPELPT(
        POINT.as_slice(),
        ELLIPS.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
