//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const MSGLEN: i32 = 800;
const NRANDM: i32 = 2000;
const UBEL: i32 = 9;
const UBPL: i32 = 4;

//$Procedure F_INELPL ( Ellipse/plane intersection routine tests )
pub fn F_INELPL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut ALPHA: f64 = 0.0;
    let mut BETA: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CONST: f64 = 0.0;
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut PPT = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SFACTR: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut XPTS = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut N: i32 = 0;
    let mut SEED: i32 = 0;
    let mut VALXPT: bool = false;

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
    // In this test family, we encapsulate some of the geometric
    // tests within functions defined in this file. The function
    //
    //    T_ISPXEL
    //
    // test whether a set of points constitute the intersection of a
    // specified plane and ellipse.  It does this by testing the points
    // to see whether they are
    //
    //    - On the ellipse:  a 2-d point P = (x,y) is "on" the
    //      ellipse with semi-axis lengths a, b if
    //      the "level surface parameter"
    //
    //         lambda = x**2/a**2 + y**2/b**2
    //
    //      is sufficiently close to 1.  In three dimensions, the
    //      point must lie in the plane of the ellipse and must
    //      satisfy the above equation when mapped to a basis
    //      spanned by the ellipse's semi-axes.
    //
    //
    //    - On the plane:  a point P = (x,y,z) is "on" the
    //      plane with normal vector N and constant C if
    //      the dot product
    //
    //         < N, P >
    //
    //      is sufficiently close to C.
    //
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_INELPL", ctx)?;

    //
    //     Perform a number of simple INELPL tests for which we can compute
    //     the intersection points in advance.
    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(&mut TITLE, b"Ellipse and plane are parallel and disjoint.");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(3, CENTER.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, SMINOR.as_slice_mut());

    //
    // Create the ellipse.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 1.0;

    //
    // Create the plane.
    //
    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the intersection.
    //
    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intersection points.
    //
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(
        &mut TITLE,
        b"Ellipse and plane are orthogonal and disjoint.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(3, CENTER.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, SMINOR.as_slice_mut());

    //
    // Create the ellipse.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, NORMAL.as_slice_mut());
    CONST = 3.0;

    //
    // Create the plane.
    //
    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the intersection.
    //
    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intersection points.
    //
    testutil::CHCKSI(b"N", N, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(&mut TITLE, b"Plane contains ellipse.");
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(3, CENTER.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, SMINOR.as_slice_mut());

    //
    // Create the ellipse.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    //
    // Create the plane.
    //
    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the intersection.
    //
    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intersection points.
    //
    testutil::CHCKSI(b"N", N, b"=", -1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(
        &mut TITLE,
        b"Plane and ellipse intersect in a single point.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(3, CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, SMINOR.as_slice_mut());

    //
    // Create the ellipse.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, NORMAL.as_slice_mut());
    CONST = 1.0;

    //
    // Create the plane.
    //
    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the intersection.
    //
    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intersection points.
    //
    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    //
    // Check the intersection points.
    //
    spicelib::VPACK(1.0, 0.0, 0.0, XPT.as_slice_mut());

    testutil::CHCKAD(
        b"XPT1",
        XPTS.subarray([1, 1]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"XPT2",
        XPTS.subarray([1, 2]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    fstr::assign(
        &mut TITLE,
        b"Plane and ellipse intersect along the line x = 0.5.",
    );
    testutil::TCASE(&TITLE, ctx)?;

    spicelib::CLEARD(3, CENTER.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, SMINOR.as_slice_mut());

    //
    // Create the ellipse.
    //
    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    spicelib::VPACK(1.0, 0.0, 0.0, NORMAL.as_slice_mut());
    CONST = 0.5;

    //
    // Create the plane.
    //
    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the intersection.
    //
    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intersection points.
    //
    testutil::CHCKSI(b"N", N, b"=", 2, 0, OK, ctx)?;

    //
    // Check the intersection points.
    //
    if (XPTS[[2, 1]] < 0.0) {
        S = -1.0;
    } else {
        S = 1.0;
    }

    spicelib::VPACK(0.5, ((S * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());

    testutil::CHCKAD(
        b"XPT1",
        XPTS.subarray([1, 1]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::VPACK(0.5, -((S * f64::sqrt(3.0)) / 2.0), 0.0, XPT.as_slice_mut());

    testutil::CHCKAD(
        b"XPT2",
        XPTS.subarray([1, 2]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // Now for some more difficult cases.  We'll generate the ellipses
    // and planes using random numbers.  There are four components to
    // generate:
    //
    //    - random plane normal vectors
    //    - random ellipse axes
    //    - random plane constants
    //    - random scale factors for the ellipse and plane; these are
    //      used to create a wide range of scales
    //

    SEED = -1;

    for I in 1..=NRANDM {
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
        // Generate a random point in the rectangle spanned by
        // the linear combinations
        //
        //    CENTER + alpha*SMAJOR + beta*SMINOR,
        //
        //        -1 <  alpha, beta  <  1
        //           -               -
        //
        ALPHA = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        BETA = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        spicelib::VLCOM3(
            1.0,
            CENTER.as_slice(),
            ALPHA,
            SMAJOR.as_slice(),
            BETA,
            SMINOR.as_slice(),
            PPT.as_slice_mut(),
        );

        //
        // We gotta have a plane normal vector.
        //
        NORMAL[1] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        NORMAL[2] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        NORMAL[3] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        spicelib::VHATIP(NORMAL.as_slice_mut());

        fstr::assign(&mut TITLE, b"INELPL Random case #.  CENTER = (#, #, #); V1 = (#, #, #); V2 = (#, #, #); PPT = (#, #, #); NORMAL = (#, #, #)");

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
        spicelib::REPMD(&TITLE.clone(), b"#", PPT[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", PPT[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", PPT[3], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Form the plane.
        //
        spicelib::NVP2PL(NORMAL.as_slice(), PPT.as_slice(), PLANE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
        spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (N > 0) {
            //
            // Make sure the intersection points we found are
            // contained in both the plane and the the
            // ellipse.
            //
            VALXPT = T_ISPXEL(
                ELLIPS.as_slice(),
                PLANE.as_slice(),
                N,
                XPTS.as_slice(),
                OK,
                ctx,
            )?;

            testutil::CHCKSL(b"Is intersection valid?", VALXPT, true, OK, ctx)?;
        }
    }

    //
    //     INELPL non-error exception cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INELPL: ellipse has one zero-length axis. and intersects plane in one point.",
        ctx,
    )?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, CENTER.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 4.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", N, b"=", 1, 0, OK, ctx)?;
    //
    // Check intersection points directly (T_ISPXEL isn't
    // designed to work with degenerate ellipses).
    //
    spicelib::VPACK(1.0, 2.0, 0.0, XPT.as_slice_mut());

    testutil::CHCKAD(
        b"XPTS 1",
        XPTS.subarray([1, 1]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"XPTS 2",
        XPTS.subarray([1, 2]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INELPL: ellipse has one zero-length axis. and lies in plane.",
        ctx,
    )?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    // The ellipse and plane are chosen so that round-off
    // errors should not prevent INELPL from deciding that
    // the ellipse lies in the plane.
    //
    spicelib::VPACK(1.0, 2.0, 0.0, CENTER.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", N, b"=", -1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INELPL: ellipse has one zero-length axis. and is disjoint from, but parallel to, the plane.", ctx)?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    spicelib::VPACK(1.0, 2.0, -5.0, CENTER.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", N, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INELPL: ellipse has one zero-length axis. and is disjoint from, and not parallel to, the plane.", ctx)?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    spicelib::VPACK(1.0, 2.0, -5.0, CENTER.as_slice_mut());
    spicelib::VPACK(4.0, 0.0, 1.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", N, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INELPL: ellipse is a single point and intersects plane in one point.",
        ctx,
    )?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    spicelib::VPACK(1.0, 2.0, 0.0, CENTER.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", N, b"=", 1, 0, OK, ctx)?;
    //
    // Check intersection points directly (T_ISPXEL isn't
    // designed to work with degenerate ellipses).
    //
    spicelib::VPACK(1.0, 2.0, 0.0, XPT.as_slice_mut());

    testutil::CHCKAD(
        b"XPTS 1",
        XPTS.subarray([1, 1]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"XPTS 2",
        XPTS.subarray([1, 2]),
        b"~",
        XPT.as_slice(),
        3,
        TIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"INELPL: ellipse is a single point and does not intersect plane.",
        ctx,
    )?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, CENTER.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMAJOR.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, SMINOR.as_slice_mut());

    spicelib::CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLIPS.as_slice_mut(),
        ctx,
    )?;

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", 0, b"=", 0, 0, OK, ctx)?;

    //
    //     INELPL error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INELPL: invalid plane", ctx)?;

    spicelib::CLEARD(4, PLANE.as_slice_mut());

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDPLANE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INELPL: invalid ellipse having non-orthogonal axes.", ctx)?;

    //
    // Create the plane.
    //
    spicelib::VPACK(0.0, 0.0, 1.0, NORMAL.as_slice_mut());
    CONST = 0.0;

    spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the ellipse. Note that we can't use
    // CGV2EL for this purpose, since it will create
    // proper, orthogonal semi-axis vectors.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, ELLIPS.subarray_mut(1));
    spicelib::VPACK(0.0, 1.0, 1.0, ELLIPS.subarray_mut(4));
    spicelib::VPACK(0.0, 1.0, 0.0, ELLIPS.subarray_mut(7));

    let [arg3, arg4] = XPTS.get_disjoint_slices_mut([[1, 1], [1, 2]]).unwrap();
    spicelib::INELPL(ELLIPS.as_slice(), PLANE.as_slice(), &mut N, arg3, arg4, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDELLIPSE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
