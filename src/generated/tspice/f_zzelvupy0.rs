//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const LNSIZE: i32 = 255;
const NCASE: i32 = 5;
const NCOMBO: i32 = 7;
const MAXN: i32 = 100;

//$Procedure      F_ZZELVUPY0 ( Test ZZELVUPY )
pub fn F_ZZELVUPY0(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; LNSIZE as usize];
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut ECTR = StackArray::<f64, 3>::new(1..=3);
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut ESMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut ESMIN = StackArray::<f64, 3>::new(1..=3);
    let mut FOV = ActualArray2D::<f64>::new(1..=3, 1..=MAXN);
    let mut HAFRT2: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
    let mut N: i32 = 0;
    let mut FOUND: bool = false;
    let mut XFOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZELVUPY0", ctx)?;

    //
    // Test all ellipse/FOV combinations.
    //
    for I in 1..=NCOMBO {
        //
        // Set the default FOV shape.
        //

        //
        // WRITE (*,*) '================================================'
        // WRITE (*,*) '================================================'
        // WRITE (*,*) '================================================'
        // WRITE (*,*) '================================================'

        if (I == 1) {
            //
            // The first case is a square FOV.
            //
            N = 4;
            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 4]));

            spicelib::VPACK(-1.0, 0.0, 0.0, AXIS.as_slice_mut());
            spicelib::VPACK(1.0, 0.0, 0.0, VERTEX.as_slice_mut());

            //
            // The default ellipse is oriented with the major axis
            // vertical and is parallel to the x-z plane.
            //
            spicelib::VPACK(0.0, 0.0, 1.0, SMAJOR.as_slice_mut());
            spicelib::VPACK(0.0, 0.5, 0.0, SMINOR.as_slice_mut());
            spicelib::VPACK(-1.0, 0.0, 0.0, CENTER.as_slice_mut());
        } else if (I == 2) {
            //
            // Rotate the FOV starting index counterclockwise by pi/2.
            //
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 1]));
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 4]));
        } else if (I == 3) {
            //
            // Rotate the FOV starting index clockwise by pi/2.
            //
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 1]));
            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 4]));
        } else if (I == 4) {
            //
            // Rotate the FOV starting index clockwise by pi.
            //
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 1]));
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 4]));
        } else if (I == 5) {
            //
            // Reverse ordering of FOV boundary vectors.
            //
            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 4]));
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 1]));
        } else if (I == 6) {
            //
            // For this case, we use an ellipse that is seen
            // edge-on from the apex of the pyramid.  This
            // ellipse lies in the x-y plane.
            //
            spicelib::VPACK(1.0, 0.0, 0.0, SMAJOR.as_slice_mut());
            spicelib::VPACK(0.0, 0.5, 0.0, SMINOR.as_slice_mut());
            spicelib::VPACK(-1.0, 0.0, 0.0, CENTER.as_slice_mut());
        } else if (I == 7) {
            //
            // Now we make the FOV into a four-pointed star.  The
            // star is formed by taking our original square FOV and
            // "pinching" it at the midpoints of each edge.
            //
            N = 8;

            spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
            spicelib::VPACK(-1.0, 0.0, -0.5, FOV.subarray_mut([1, 2]));
            spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 3]));
            spicelib::VPACK(-1.0, 0.5, 0.0, FOV.subarray_mut([1, 4]));
            spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 5]));
            spicelib::VPACK(-1.0, 0.0, 0.5, FOV.subarray_mut([1, 6]));
            spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 7]));
            spicelib::VPACK(-1.0, -0.5, 0.0, FOV.subarray_mut([1, 8]));

            //
            // Use the same axis and vertex as before.
            //
            spicelib::VPACK(-1.0, 0.0, 0.0, AXIS.as_slice_mut());
            spicelib::VPACK(1.0, 0.0, 0.0, VERTEX.as_slice_mut());

            //
            // The default ellipse is oriented with the major axis slanted
            // in the z=y direction and is parallel to the x-z plane.
            //
            HAFRT2 = (f64::sqrt(2.0) / 2.0);

            spicelib::VPACK(0.0, HAFRT2, HAFRT2, SMAJOR.as_slice_mut());
            spicelib::VPACK(0.0, -(HAFRT2 / 2 as f64), HAFRT2, SMINOR.as_slice_mut());
            spicelib::VPACK(-1.0, 0.0, 0.0, CENTER.as_slice_mut());
        }

        //
        // Test for geometric cases:
        //
        //    1) Ellipse inside FOV
        //    2) FOV inside ellipse
        //    3) FOV chops ellipse
        //    4) No intersection
        //

        for J in 1..=NCASE {
            if (J == 1) {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"Combo = #; geometric case = ellipse in FOV.");

                spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&TITLE, ctx)?;

                //
                // The ellipse is defined by the default components.
                //
                spicelib::VEQU(CENTER.as_slice(), ECTR.as_slice_mut());
                spicelib::VEQU(SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                spicelib::VEQU(SMINOR.as_slice(), ESMIN.as_slice_mut());

                //
                // We expect the intersection to be found.
                //
                XFOUND = true;
            } else if (J == 2) {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"Combo = #; geometric case = ellipse is not contained in FOV but contains center of FOV.");

                spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&TITLE, ctx)?;

                if (I <= 5) {
                    //
                    // The ellipse is scaled up by a factor of 10 and
                    // shifted in the -z direction so the center of the
                    // ellipse is not in the FOV.
                    //
                    spicelib::VPACK(0.0, 0.0, -9.5, OFFSET.as_slice_mut());

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());
                } else if (I == 6) {
                    spicelib::VPACK(0.0, -9.5, 0.0, OFFSET.as_slice_mut());

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());
                }

                //
                // We expect the intersection to be found.
                //
                XFOUND = true;
            } else if (J == 3) {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"Combo = #; geometric case = ellipse is chopped by FOV.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&TITLE, ctx)?;

                if (I <= 5) {
                    //
                    // The ellipse is scaled up by a factor of 10 and
                    // shifted in the -z direction so the center is not in
                    // the FOV and the center of the FOV is not in the
                    // ellipse.
                    //
                    spicelib::VPACK(0.0, 0.0, -10.999, OFFSET.as_slice_mut());

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());
                } else if (I == 6) {
                    // The ellipse is scaled up by a factor of 10 and
                    // shifted in the -y direction so the center is not in
                    // the FOV and the center of the FOV is not in the
                    // ellipse.
                    //
                    spicelib::VPACK(0.0, -5.5, 0.0, OFFSET.as_slice_mut());

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());
                }

                //
                // We expect the intersection to be found.
                //
                XFOUND = true;
            } else if (J == 4) {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(
                    &mut TITLE,
                    b"Combo = #; geometric case = bounding cones of ellipse and FOV are disjoint.",
                );

                spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&TITLE, ctx)?;

                //
                // The ellipse is scaled up by a factor of 10 and
                // shifted in the -z direction so the bounding cones
                // are disjoint.
                //
                spicelib::VPACK(0.0, 0.0, -30.0, OFFSET.as_slice_mut());

                spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());

                //
                // We expect the intersection NOT to be found.
                //
                XFOUND = false;
            } else if (J == 5) {
                //
                // --- Case: ------------------------------------------------------
                //
                fstr::assign(&mut TITLE, b"Combo = #; geometric case = bounding cones of ellipse and FOV are not disjoint, but there is no intersection.");

                spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::TCASE(&TITLE, ctx)?;

                if (I <= 5) {
                    //
                    // Make the ellipse very thin and position it
                    // so that it doesn't intersect the FOV, but so
                    // that the plane containing the apex and the top
                    // edge of the FOV does intersect the ellipse.
                    //
                    if (I <= 3) {
                        spicelib::VPACK(0.0, 2.00050001, 2.0, OFFSET.as_slice_mut());
                    } else {
                        spicelib::VPACK(0.0, -2.00050001, 2.0, OFFSET.as_slice_mut());
                    }

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VEQU(SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(0.001, SMINOR.as_slice(), ESMIN.as_slice_mut());
                } else {
                    //
                    // The ellipse is scaled up by a factor of 10 and
                    // shifted in the -z direction so the bounding cones
                    // are non disjoint, but the ellipse is outside the
                    // FOV.
                    //
                    spicelib::VPACK(0.0, 0.0, -15.0, OFFSET.as_slice_mut());

                    spicelib::VADD(CENTER.as_slice(), OFFSET.as_slice(), ECTR.as_slice_mut());
                    spicelib::VSCL(10.0, SMAJOR.as_slice(), ESMAJ.as_slice_mut());
                    spicelib::VSCL(10.0, SMINOR.as_slice(), ESMIN.as_slice_mut());
                }

                //
                // We expect the intersection NOT to be found.
                //
                XFOUND = false;
            }

            //
            // Pack the ellipse components.
            //
            spicelib::CGV2EL(
                ECTR.as_slice(),
                ESMAJ.as_slice(),
                ESMIN.as_slice(),
                ELLIPS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZELVUPY(
                ELLIPS.as_slice(),
                VERTEX.as_slice(),
                AXIS.as_slice(),
                N,
                FOV.as_slice(),
                &mut FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"FOUND", FOUND, XFOUND, OK, ctx)?;
        }
    }

    //
    // Test error cases.
    //
    N = 4;
    spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
    spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 2]));
    spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 3]));
    spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 4]));

    spicelib::VPACK(-1.0, 0.0, 0.0, AXIS.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, VERTEX.as_slice_mut());

    //
    // The default ellipse is oriented with the major axis
    // vertical and is parallel to the x-z plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.5, 0.0, SMINOR.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, CENTER.as_slice_mut());

    testutil::TCASE(b"Axis is the zero vector.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 0.0, AXIS.as_slice_mut());

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    testutil::TCASE(b"Second and third boundary vectors are the same.", ctx)?;
    //
    // Restore the orginal axis vector.
    //
    spicelib::VPACK(-1.0, 0.0, 0.0, AXIS.as_slice_mut());

    spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 3]));

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDFOV)", OK, ctx)?;

    testutil::TCASE(b"Third boundary vector is the zero vector.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 0.0, FOV.subarray_mut([1, 3]));

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    testutil::TCASE(b"Ellipse generating vectors are linearly dependent.  The semi-minor axis is therefore zero.", ctx)?;

    N = 4;
    spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
    spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 2]));
    spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 3]));
    spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 4]));

    spicelib::VPACK(0.0, 0.0, 1.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    testutil::TCASE(b"Ellipse semi-minor axis is zero.", ctx)?;

    N = 4;
    spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
    spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 2]));
    spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 3]));
    spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 4]));

    spicelib::VPACK(0.0, 0.0, 1.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    testutil::TCASE(b"Ellipse semi-axes are both zero.", ctx)?;

    N = 4;
    spicelib::VPACK(-1.0, -1.0, -1.0, FOV.subarray_mut([1, 1]));
    spicelib::VPACK(-1.0, 1.0, -1.0, FOV.subarray_mut([1, 2]));
    spicelib::VPACK(-1.0, 1.0, 1.0, FOV.subarray_mut([1, 3]));
    spicelib::VPACK(-1.0, -1.0, 1.0, FOV.subarray_mut([1, 4]));

    spicelib::VPACK(0.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZELVUPY(
        ELLIPS.as_slice(),
        VERTEX.as_slice(),
        AXIS.as_slice(),
        N,
        FOV.as_slice(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    // This commented-out block is used for timing tests.
    // We set up the square FOV and ellipse in the x-z plane.
    //
    // CALL VPACK ( -1.D0,  -1.D0, -1.D0,  FOV(1,1) )
    // CALL VPACK ( -1.D0,   1.D0, -1.D0,  FOV(1,2) )
    // CALL VPACK ( -1.D0,   1.D0,  1.D0,  FOV(1,3) )
    // CALL VPACK ( -1.D0,  -1.D0,  1.D0,  FOV(1,4) )
    //
    // CALL VPACK ( -1.D0,   0.D0,  0.D0,  AXIS     )
    // CALL VPACK (  1.D0,   0.D0,  0.D0,  VERTEX   )
    //
    // The default ellipse is oriented with the major axis
    // vertical and is parallel to the x-z plane.
    //
    // CALL VPACK (  0.D0,   0.D0,  1.D0,   SMAJOR )
    // CALL VPACK (  0.D0,   5.D-1, 0.D0,   SMINOR )
    // CALL VPACK ( -1.D0,   0.D0,  0.D0,   CENTER )
    //
    // This  block is used for the case where the bounding cones are
    // disjoint. This is the fastest case, but the one that should be
    // encountered most often.
    //
    // CALL VPACK ( 0.D0, 0.D0, -30.D0, OFFSET )
    //
    // CALL VADD  ( CENTER,  OFFSET, ECTR  )
    // CALL VSCL  ( 10.D0,   SMAJOR, ESMAJ )
    // CALL VSCL  ( 10.D0,   SMINOR, ESMIN )
    //
    // This commented-out block is for the fall-through
    // non-intersection case, which is the slowest.
    //
    // CALL VPACK ( 0.D0, 0.D0, -15.D0, OFFSET )
    //
    // CALL VADD  ( CENTER,  OFFSET, ECTR  )
    // CALL VSCL  ( 10.D0,   SMAJOR, ESMAJ )
    // CALL VSCL  ( 10.D0,   SMINOR, ESMIN )
    //
    //
    // CALL CGV2EL ( ECTR, ESMAJ, ESMIN, ELLIPS )
    //
    //
    // DO I = 1, 100000
    //    CALL ZZELVUPY ( ELLIPS, VERTEX, AXIS, N, FOV, FOUND )
    // END DO
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
