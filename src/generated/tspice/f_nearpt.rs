//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const ATOL: f64 = 0.00000000000001;
const STOL: f64 = 0.00000000001;
const TIGHT: f64 = 0.0000000000001;
const MEDIUM: f64 = 0.000000001;
const MSGLEN: i32 = 400;
const NXRAND: i32 = 4000;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN }
    }
}

//$Procedure F_NEARPT ( NEARPT tests )
pub fn F_NEARPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut ALT: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut GV1 = StackArray::<f64, 3>::new(1..=3);
    let mut GV2 = StackArray::<f64, 3>::new(1..=3);
    let mut IPALT: f64 = 0.0;
    let mut IPNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut SFACTR: f64 = 0.0;
    let mut SRFX = StackArray::<f64, 3>::new(1..=3);
    let mut THETA: f64 = 0.0;
    let mut VIEWPT = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut PINDEX: i32 = 0;
    let mut SEED: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISNEAR: bool = false;
    let mut VALPT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
    //
    //
    // SPICELIB ellipse upper bound.
    //

    //
    // Altitude tolerance:  this is applied to the sum of
    // altitude of the view point and the maximum of the
    // ellipsoid's axis lengths.  The reason the tolerance is not
    // applied to the altitude directly is that the altitude may have
    // large relative errors, but still be accurate, when the view point
    // is close to the surface.
    //

    //
    // Angular separation tolerance:  this applies to the angular
    // separation between the near point-to-view point vector and
    // the properly signed normal vector at the near point.
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
    //    T_ISEDPT
    //
    // tests a point is on a specified ellipsoid:
    //
    //    - A point P = (x,y,z) is "on" the ellipsoid with semi-axis
    //      lengths a, b, c if the "level surface parameter"
    //
    //         lambda = x**2/a**2 + y**2/b**2 + z**2/c**2
    //
    //      is sufficiently close to 1.
    //
    //
    // The function
    //
    //    T_ISNPPT
    //
    // test whether the near point found by NEARPT has the property
    // that the outward normal at the near point can be extended to
    // touch the input point.
    //
    // T_ISNPPT also checks the distance of the near point from the
    // point: the distance is checked against the value obtained from
    // NPLNPT.

    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_NEARPT", ctx)?;

    //
    //     NEARPT simple cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEARPT: ellipsoid is sphere; point is at origin.", ctx)?;

    spicelib::CLEARD(3, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", ALT, b"~", -1.0, ATOL, OK, ctx)?;

    testutil::CHCKSD(
        b"VNORM(PNEAR)",
        spicelib::VNORM(PNEAR.as_slice()),
        b"~",
        1.0,
        ATOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: ellipsoid is sphere; point is exterior on +Z axis.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 5.0, VIEWPT.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, XPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~",
        XPT.as_slice(),
        3,
        ATOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"ALT", ALT, b"~", 4.0, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: ellipsoid is sphere; point is interior on ray generated by (1,1,1)",
        ctx,
    )?;

    S = ((1 as f64) / f64::sqrt(3.0));

    spicelib::VPACK((S / 2.0), (S / 2.0), (S / 2.0), VIEWPT.as_slice_mut());

    spicelib::VPACK(S, S, S, XPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~",
        XPT.as_slice(),
        3,
        ATOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"ALT", ALT, b"~", -0.5, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: ellipsoid is triaxial; point is on selected outward normal.",
        ctx,
    )?;
    //
    // Start with the unit sphere and a point on the ray (1,1,1);
    // multiply all participants by diag[3,2,1].
    //
    A = 3.0;
    B = 2.0;
    C = 1.0;

    S = ((1 as f64) / f64::sqrt(3.0));

    spicelib::VPACK((A * S), (B * S), (C * S), SRFX.as_slice_mut());

    //
    // Create the outward unit normal at SRFX.
    //
    spicelib::VPACK(
        (SRFX[1] / f64::powi(A, 2)),
        (SRFX[2] / f64::powi(B, 2)),
        (SRFX[3] / f64::powi(C, 2)),
        NORMAL.as_slice_mut(),
    );

    spicelib::VHATIP(NORMAL.as_slice_mut());

    //
    // Create a view point at altitude 1 along the normal.
    //
    spicelib::VADD(SRFX.as_slice(), NORMAL.as_slice(), VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", ALT, b"~", 1.0, ATOL, OK, ctx)?;

    //
    // The near point should be the surface point at which the
    // normal was generated.
    //
    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~",
        SRFX.as_slice(),
        3,
        ATOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: ellipsoid is triaxial; point is on selected inward normal.",
        ctx,
    )?;
    //
    // Start with the unit sphere and a point on the ray (1,1,1);
    // multiply all participants by diag[3,2,1].
    //
    A = 3.0;
    B = 2.0;
    C = 1.0;

    S = ((1 as f64) / f64::sqrt(3.0));

    spicelib::VPACK((A * S), (B * S), (C * S), SRFX.as_slice_mut());

    //
    // Create the outward unit normal at SRFX.
    //
    spicelib::VPACK(
        (SRFX[1] / f64::powi(A, 2)),
        (SRFX[2] / f64::powi(B, 2)),
        (SRFX[3] / f64::powi(C, 2)),
        NORMAL.as_slice_mut(),
    );

    spicelib::VHATIP(NORMAL.as_slice_mut());

    //
    // Create a view point at altitude 1 along the normal.
    //
    spicelib::VLCOM(
        1.0,
        SRFX.as_slice(),
        -0.1,
        NORMAL.as_slice(),
        VIEWPT.as_slice_mut(),
    );

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", ALT, b"~", -0.1, ATOL, OK, ctx)?;

    //
    // The near point should be the surface point at which the
    // normal was generated.
    //
    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~",
        SRFX.as_slice(),
        3,
        ATOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEARPT: ellipsoid is bi-axial; point is at origin.", ctx)?;

    spicelib::CLEARD(3, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        2.0,
        2.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", ALT, b"~", -1.0, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEARPT: ellipsoid is tri-axial; point is at origin.", ctx)?;

    spicelib::CLEARD(3, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        3.0,
        2.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", ALT, b"~", -1.0, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: interior prolate case. Viewpoint is on the long axis; solution is off-axis.",
        ctx,
    )?;

    A = 10.0;
    B = 5.0;
    C = 5.0;

    spicelib::VPACK(7.0, 0.0, 0.0, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the solution is on the ellipsoid.
    //
    VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

    testutil::CHCKSL(b"Is on ellipsoid", VALPT, true, OK, ctx)?;

    //
    // Make sure the solution satisfies the orthogonality
    // condition.
    //
    ISNEAR = T_ISNPPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice(),
        ALT,
        ATOL,
        STOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"Is critical point", ISNEAR, true, OK, ctx)?;

    //
    // Make sure altitude is greater than -2.95.
    //
    testutil::CHCKSD(b"ALT", ALT, b">", -2.95, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: interior prolate case. Repeat previous case with point closer to the origin.",
        ctx,
    )?;

    A = 10.0;
    B = 5.0;
    C = 5.0;

    spicelib::VPACK(1.0, 0.0, 0.0, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the solution is on the ellipsoid.
    //
    VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

    testutil::CHCKSL(b"Is on ellipsoid", VALPT, true, OK, ctx)?;

    //
    // Make sure the solution satisfies the orthogonality
    // condition.
    //
    ISNEAR = T_ISNPPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice(),
        ALT,
        ATOL,
        STOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"Is critical point", ISNEAR, true, OK, ctx)?;

    //
    // Make sure altitude is greater than -5.
    //
    testutil::CHCKSD(b"ALT", ALT, b">", -5.0, ATOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: interior prolate case. Repeat previous case with point near axis endpoint.",
        ctx,
    )?;

    A = 10.0;
    B = 5.0;
    C = 5.0;

    spicelib::VPACK(9.0, 0.0, 0.0, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the solution is on the ellipsoid.
    //
    VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

    testutil::CHCKSL(b"Is on ellipsoid", VALPT, true, OK, ctx)?;

    //
    // Make sure the solution satisfies the orthogonality
    // condition.
    //
    ISNEAR = T_ISNPPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice(),
        ALT,
        ATOL,
        STOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"Is critical point", ISNEAR, true, OK, ctx)?;

    //
    // Make sure near point is axis endpoint.
    //
    testutil::CHCKSD(b"ALT", ALT, b">", -1.0, ATOL, OK, ctx)?;

    spicelib::VPACK(A, 0.0, 0.0, XPT.as_slice_mut());
    testutil::CHCKAD(
        b"PNEAR",
        PNEAR.as_slice(),
        b"~~",
        XPT.as_slice(),
        3,
        ATOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // First set of random test cases:  random ellipsoids and random
    // *exterior* viewing points.
    //
    SEED = -1;

    for I in 1..=NXRAND {
        //
        // --- Case: ------------------------------------------------------
        //
        //
        //        Make up ellipsoid axis lengths.
        //
        A = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        B = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        C = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // Create viewing point.
        //
        VIEWPT[1] = f64::powf(10.0, testutil::T_RANDD(-3.0, 100.0, &mut SEED, ctx)?);
        VIEWPT[2] = f64::powf(10.0, testutil::T_RANDD(-3.0, 100.0, &mut SEED, ctx)?);
        VIEWPT[3] = f64::powf(10.0, testutil::T_RANDD(-3.0, 100.0, &mut SEED, ctx)?);

        //
        // Find the level surface parameter of the view point.
        //
        LEVEL = ((f64::powf((VIEWPT[1] / A), 2.0) + f64::powf((VIEWPT[2] / B), 2.0))
            + f64::powf((VIEWPT[3] / C), 2.0));

        //
        // If the viewing point is inside the ellipsoid, replace it
        // with the surface point found by scaling up the viewing
        // point.
        //
        if (LEVEL < 1.0) {
            //
            // FOUND will always be .TRUE. in this case.
            //
            spicelib::SURFPT(
                save.ORIGIN.as_slice(),
                VIEWPT.as_slice(),
                A,
                B,
                C,
                SRFX.as_slice_mut(),
                &mut FOUND,
                ctx,
            )?;
            //
            // Scale the point up a tad for safety.
            //
            spicelib::VSCL(
                (1.0 + 0.000000000000001),
                SRFX.as_slice(),
                VIEWPT.as_slice_mut(),
            );
        }

        fstr::assign(&mut TITLE, b"Initial NEARPT exterior random case #. For this set of cases, we don\'t use extreme scale differences for the axis lengths:  range is [0.1 : 10]. VIEWPT components are in range [0.001 : 1.e150]. A = #; B = #; C = #; VIEWPT = (#,#,#).");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", A, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", B, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", C, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NEARPT(
            VIEWPT.as_slice(),
            A,
            B,
            C,
            PNEAR.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            //
            // Make sure the near point we found belongs to
            // the surface of the ellipsoid.
            //
            VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

            testutil::CHCKSL(b"Is near point on surface?", VALPT, true, OK, ctx)?;

            //
            // The outward normal at the near point should point towards
            // the view point.
            //
            ISNEAR = T_ISNPPT(
                VIEWPT.as_slice(),
                A,
                B,
                C,
                PNEAR.as_slice(),
                ALT,
                ATOL,
                STOL,
                OK,
                ctx,
            )?;

            testutil::CHCKSL(b"Is signed outward normal parallel to  near point-view point vector?  Is distance correct?", ISNEAR, true, OK, ctx)?;
        }
    }

    //
    // Second set of random test cases:  random ellipsoids and random
    // *interior* viewing points.  At this point, we don't verify
    // that the solutions are absolute minima, but we do verify
    // that they are critical points.
    //
    SEED = -1;

    for I in 1..=NXRAND {
        //
        // --- Case: ------------------------------------------------------
        //
        //
        //        Make up ellipsoid axis lengths.
        //
        A = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        B = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        C = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // Create viewing point.
        //
        VIEWPT[1] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VIEWPT[2] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        VIEWPT[3] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        //
        // Find the level surface parameter of the view point.
        //
        LEVEL = ((f64::powf((VIEWPT[1] / A), 2.0) + f64::powf((VIEWPT[2] / B), 2.0))
            + f64::powf((VIEWPT[3] / C), 2.0));

        //
        // If the viewing point is outside the ellipsoid, replace it
        // with an interior point found by scaling down the surface
        // point corresponding to the viewing point.
        //
        if (LEVEL >= 1.0) {
            //
            // FOUND will always be .TRUE. in this case.
            //
            spicelib::SURFPT(
                save.ORIGIN.as_slice(),
                VIEWPT.as_slice(),
                A,
                B,
                C,
                SRFX.as_slice_mut(),
                &mut FOUND,
                ctx,
            )?;
            //
            // Scale the point down to make it usable.
            //
            SFACTR = testutil::T_RANDD(0.0, (1.0 - 0.000001), &mut SEED, ctx)?;

            spicelib::VSCL(SFACTR, SRFX.as_slice(), VIEWPT.as_slice_mut());
        }

        fstr::assign(&mut TITLE, b"Initial NEARPT interior random case #. For this set of cases, we don\'t use extreme scale differences for the axis lengths:  range is [0.1 : 10]. VIEWPT components are in range [0.001 : 1.e150]. A = #; B = #; C = #; VIEWPT = (#,#,#).");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", A, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", B, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", C, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NEARPT(
            VIEWPT.as_slice(),
            A,
            B,
            C,
            PNEAR.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the near point we found is contained in
        // both the plane and the surface of the ellipsoid.
        //
        if *OK {
            //
            // Make sure the near point we found belongs to
            // the surface of the ellipsoid.
            //
            VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

            testutil::CHCKSL(b"Is near point on surface?", VALPT, true, OK, ctx)?;

            //
            // The inward normal at the near point should point towards
            // the view point.
            //
            ISNEAR = T_ISNPPT(
                VIEWPT.as_slice(),
                A,
                B,
                C,
                PNEAR.as_slice(),
                ALT,
                ATOL,
                STOL,
                OK,
                ctx,
            )?;

            testutil::CHCKSL(b"Is signed normal parallel to  near point-view point vector?  Is distance correct?", ISNEAR, true, OK, ctx)?;
        }
    }

    //
    // Third set of random test cases:  random ellipsoids and random
    // viewing points lying on principal planes.
    //
    SEED = -1;

    for I in 1..=NXRAND {
        //
        // --- Case: ------------------------------------------------------
        //
        //
        //        Make up ellipsoid axis lengths.
        //
        A = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        B = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        C = f64::powf(10.0, testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // Do a few tests to check for problems in the special
        // cases where the axes are not distinct.
        //
        if (I < 200) {
            //
            // Turn the ellipsoid into a sphere.
            //
            B = A;
            C = A;
        } else if (I < 400) {
            //
            // The ellipsoid is symmetric about X.
            //
            B = C;
        } else if (I < 600) {
            //
            // The ellipsoid is symmetric about Y.
            //
            C = A;
        } else if (I < 800) {
            //
            // The ellipsoid is symmetric about Z.
            //
            B = A;
        }

        //
        // Create viewing point.  We pick a plane index, a scale factor
        // to give us the radius of the point, and an angular argument.
        //
        PINDEX = intrinsics::IDNINT(testutil::T_RANDD(1.0, 3.0, &mut SEED, ctx)?);
        SCALE = testutil::T_RANDD(0.0, 1.0, &mut SEED, ctx)?;
        THETA = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

        if (PINDEX == 1) {
            //
            // The point lies in the y-z plane.
            //
            VIEWPT[1] = 0.0;
            VIEWPT[2] = ((SCALE * B) * f64::cos(THETA));
            VIEWPT[3] = ((SCALE * C) * f64::sin(THETA));

            //
            // In addition to creating a viewing point, create generating
            // vectors for the intersection ellipse defined by the
            // ellipsoid and the y-z plane.  We'll use this ellipse later.
            //
            spicelib::VPACK(0.0, B, 0.0, GV1.as_slice_mut());
            spicelib::VPACK(0.0, 0.0, C, GV2.as_slice_mut());
        } else if (PINDEX == 2) {
            //
            // The point lies in the x-z plane.
            //
            VIEWPT[1] = ((SCALE * A) * f64::cos(THETA));
            VIEWPT[2] = 0.0;
            VIEWPT[3] = ((SCALE * C) * f64::sin(THETA));

            spicelib::VPACK(A, 0.0, 0.0, GV1.as_slice_mut());
            spicelib::VPACK(0.0, 0.0, C, GV2.as_slice_mut());
        } else {
            //
            // The point lies in the x-Y plane.
            //
            VIEWPT[1] = ((SCALE * A) * f64::cos(THETA));
            VIEWPT[2] = ((SCALE * B) * f64::sin(THETA));
            VIEWPT[3] = 0.0;

            spicelib::VPACK(A, 0.0, 0.0, GV1.as_slice_mut());
            spicelib::VPACK(0.0, B, 0.0, GV2.as_slice_mut());
        }

        //
        // Create the ellipse from the generating vectors.
        //
        spicelib::CGV2EL(
            save.ORIGIN.as_slice(),
            GV1.as_slice(),
            GV2.as_slice(),
            ELLIPS.as_slice_mut(),
            ctx,
        )?;

        //
        // If the viewing point is outside the ellipsoid, replace it
        // with an interior point found by scaling down the surface
        // point corresponding to the viewing point.
        //
        // Find the level surface parameter of the view point.
        //
        LEVEL = ((f64::powf((VIEWPT[1] / A), 2.0) + f64::powf((VIEWPT[2] / B), 2.0))
            + f64::powf((VIEWPT[3] / C), 2.0));

        if (LEVEL >= 1.0) {
            SFACTR = (1.0 - 0.000001);

            spicelib::VSCLIP(SFACTR, VIEWPT.as_slice_mut());
        }

        fstr::assign(&mut TITLE, b"NEARPT interior principal plane random case #.  For this set of cases, we don\'t use extreme scale differences for the axis lengths:  range is [0.1 : 10]. VIEWPT components are in range [0.001 : 1.e150]. A = #; B = #; C = #; VIEWPT = (#,#,#).");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", A, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", B, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", C, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", VIEWPT[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NEARPT(
            VIEWPT.as_slice(),
            A,
            B,
            C,
            PNEAR.as_slice_mut(),
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the near point we found is contained in
        // both the plane and the surface of the ellipsoid.
        //
        if *OK {
            //
            // Make sure the near point we found belongs to
            // the surface of the ellipsoid.
            //
            VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

            testutil::CHCKSL(b"Is near point on surface?", VALPT, true, OK, ctx)?;

            //
            // The inward normal at the near point should point towards
            // the view point.  For these cases we use a more lenient
            // tolerance for the angular separation.  We also use a
            // slightly looser altitude tolerance.
            //
            ISNEAR = T_ISNPPT(
                VIEWPT.as_slice(),
                A,
                B,
                C,
                PNEAR.as_slice(),
                ALT,
                TIGHT,
                MEDIUM,
                OK,
                ctx,
            )?;

            testutil::CHCKSL(b"Is signed normal parallel to  near point-view point vector?  Is distance correct?", ISNEAR, true, OK, ctx)?;

            if (PNEAR[PINDEX] != 0.0) {
                //
                // We have a solution that doesn't lie in the same plane
                // as the viewing point.  This may well be correct, but
                // we'll check it.  Find the in-plane solution and
                // compare altitudes.
                //
                spicelib::NPELPT(
                    VIEWPT.as_slice(),
                    ELLIPS.as_slice(),
                    IPNEAR.as_slice_mut(),
                    &mut IPALT,
                    ctx,
                )?;

                //
                // We expect the in-plane "altitude" to be greater than
                // or equal to that of the out-of-plane near point.
                //
                testutil::CHCKSD(b"IPALT", IPALT, b">=", ALT, 0.0, OK, ctx)?;

                // WRITE (*,*) 'IPALT - ALT = ', IPALT - ALT

                // WRITE (*,*) 'Out-of-plane solution'
                // WRITE (*,*) 'view point = ', VIEWPT
                // WRITE (*,*) 'near point = ', PNEAR
                // WRITE (*,*) 'altitude   = ', ALT
                // WRITE (*,*) '======================='
            }
        }
    }

    //
    //     NEARPT error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEARPT: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::FILLD(10.0, 3, VIEWPT.as_slice_mut());

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        0.0,
        1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        0.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        1.0,
        0.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NEARPT: ellipsoid has one negative-length axis.", ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        -1.0,
        1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        -1.0,
        1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        1.0,
        1.0,
        -1.0,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: ratio of longest to shortest axes is too large.",
        ctx,
    )?;

    A = 0.000001;
    B = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    C = 1.0;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        C,
        A,
        B,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::NEARPT(
        VIEWPT.as_slice(),
        B,
        C,
        A,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"NEARPT: product of scaled axis magnitude and scaled POSITN component is too large.",
        ctx,
    )?;

    A = 0.000001;
    B = 1000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    C = 1.0;

    spicelib::VPACK(A, f64::powf(B, 3.0), C, VIEWPT.as_slice_mut());
    spicelib::NEARPT(
        VIEWPT.as_slice(),
        A,
        B,
        C,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INPUTSTOOLARGE)", OK, ctx)?;

    spicelib::VPACK(C, A, f64::powf(B, 3.0), VIEWPT.as_slice_mut());
    spicelib::NEARPT(
        VIEWPT.as_slice(),
        C,
        A,
        B,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INPUTSTOOLARGE)", OK, ctx)?;

    spicelib::VPACK(f64::powf(B, 3.0), C, A, VIEWPT.as_slice_mut());
    spicelib::NEARPT(
        VIEWPT.as_slice(),
        B,
        C,
        A,
        PNEAR.as_slice_mut(),
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INPUTSTOOLARGE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
