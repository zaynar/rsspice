//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.00000000000001;
const MSGLEN: i32 = 400;
const NSIMPL: i32 = 3;
const NRANDM: i32 = 5000;

struct SaveVars {
    SMPA: StackArray<f64, 3>,
    SMPB: StackArray<f64, 3>,
    SMPC: StackArray<f64, 3>,
    SMPDIR: StackArray2D<f64, 9>,
    SMPPT: StackArray2D<f64, 9>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SMPA = StackArray::<f64, 3>::new(1..=NSIMPL);
        let mut SMPB = StackArray::<f64, 3>::new(1..=NSIMPL);
        let mut SMPC = StackArray::<f64, 3>::new(1..=NSIMPL);
        let mut SMPDIR = StackArray2D::<f64, 9>::new(1..=3, 1..=NSIMPL);
        let mut SMPPT = StackArray2D::<f64, 9>::new(1..=3, 1..=NSIMPL);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(10.0), Val::D(200.0), Val::D(3000.0)].into_iter();
            SMPA[1] = clist.next().unwrap().into_f64();
            SMPB[1] = clist.next().unwrap().into_f64();
            SMPC[1] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(20.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPPT[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(-1.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDIR[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(3000000000000000000000000000000.0),
                Val::D(3000000000000000000000000000000.0),
                Val::D(3000000000000000000000000000000.0),
            ]
            .into_iter();
            SMPA[2] = clist.next().unwrap().into_f64();
            SMPB[2] = clist.next().unwrap().into_f64();
            SMPC[2] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(4000000000000000000000000000000.0),
                Val::D(0.0),
                Val::D(4000000000000000000000000000000.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPPT[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(2.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDIR[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.01), Val::D(0.01), Val::D(0.005)].into_iter();
            SMPA[3] = clist.next().unwrap().into_f64();
            SMPB[3] = clist.next().unwrap().into_f64();
            SMPC[3] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPPT[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(-2.0), Val::D(-1.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDIR[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SMPA,
            SMPB,
            SMPC,
            SMPDIR,
            SMPPT,
        }
    }
}

//$Procedure F_NPEDLN ( NPEDLN tests )
pub fn F_NPEDLN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut LINEDR = StackArray::<f64, 3>::new(1..=3);
    let mut LINEPT = StackArray::<f64, 3>::new(1..=3);
    let mut NEGDIR = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut SFACTR: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut SEED: i32 = 0;
    let mut FOUND: bool = false;
    let mut FOUND2: bool = false;
    let mut ORTHOG: bool = false;
    let mut VALPT: bool = false;

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    // INTEGER               NEASY
    // PARAMETER           ( NEASY  =  3 )

    // INTEGER               NUMSCL
    // PARAMETER           ( NUMSCL =  5 )

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
    // The simple test cases.
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
    //    T_ISNPLN
    //
    // test whether the near point found by NPEDLN has the property
    // that the outward normal at the near point can be extended to
    // intersect the input line at right angles.
    //
    // T_ISNPLN also checks the distance of the near point from the line:
    // the distance is checked against the value obtained from NPLNPT.
    //
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_NPEDLN", ctx)?;

    //
    // Run some simple tests where the correct results can be
    // determined by inspection.
    //
    for I in 1..=NSIMPL {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"NPEDLN simple case #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::NPEDLN(
            save.SMPA[I],
            save.SMPB[I],
            save.SMPC[I],
            save.SMPPT.subarray([1, I]),
            save.SMPDIR.subarray([1, I]),
            PNEAR.as_slice_mut(),
            &mut DIST,
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the intersection ellipse we found is contained in
        // both the plane and the surface of the ellipsoid.
        //
        VALPT = T_ISEDPT(
            PNEAR.as_slice(),
            save.SMPA[I],
            save.SMPB[I],
            save.SMPC[I],
            OK,
            ctx,
        )?;

        testutil::CHCKSL(b"Is near point on surface?", VALPT, true, OK, ctx)?;

        ORTHOG = T_ISNPLN(
            save.SMPA[I],
            save.SMPB[I],
            save.SMPC[I],
            save.SMPPT.subarray([1, I]),
            save.SMPDIR.subarray([1, I]),
            PNEAR.as_slice(),
            DIST,
            OK,
            ctx,
        )?;

        testutil::CHCKSL(
            b"Does extension of outward normal hit line orthogonally?  Is distance correct?",
            ORTHOG,
            true,
            OK,
            ctx,
        )?;
    }

    //
    // Now for some more difficult cases.  We'll generate the ellipsoids
    // and lines using random numbers.  There are ten components to
    // generate:
    //
    //    - random line direction vectors
    //    - random line points
    //    - random ellipsoid axes
    //    - random scale factors for the ellipsoid and plane; these are
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
        // Make up ellipsoid axes and plane constant.
        //
        A = (SFACTR * testutil::T_RANDD(1.0, 2.0, &mut SEED, ctx)?);
        B = (SFACTR * testutil::T_RANDD(1.0, 2.0, &mut SEED, ctx)?);
        C = (SFACTR * testutil::T_RANDD(1.0, 2.0, &mut SEED, ctx)?);

        //
        // We gotta have a line direction vector.
        //
        LINEDR[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEDR[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEDR[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        spicelib::VHATIP(LINEDR.as_slice_mut());

        //
        // We also need a point on the line.  Scale the point up to
        // increase the likelihood of a non-intercept case.
        //
        LINEPT[1] = (((10 as f64) * SFACTR) * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEPT[2] = (((10 as f64) * SFACTR) * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        LINEPT[3] = (((10 as f64) * SFACTR) * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        fstr::assign(
            &mut TITLE,
            b"NPEDLN Random case #.  A, B, C = # # #; LINEDR = (#, #, #); LINEPT = (#, #, #)",
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", A, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", B, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", C, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEDR[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEDR[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEDR[3], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEPT[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEPT[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", LINEPT[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NPEDLN(
            A,
            B,
            C,
            LINEPT.as_slice(),
            LINEDR.as_slice(),
            PNEAR.as_slice_mut(),
            &mut DIST,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Make sure the intersection ellipse we found is contained in
        // both the plane and the surface of the ellipsoid.
        //
        VALPT = T_ISEDPT(PNEAR.as_slice(), A, B, C, OK, ctx)?;

        testutil::CHCKSL(b"Is near point on surface?", VALPT, true, OK, ctx)?;

        //
        // Check for surface intercept.
        //
        spicelib::SURFPT(
            LINEPT.as_slice(),
            LINEDR.as_slice(),
            A,
            B,
            C,
            XPT.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;

        if !FOUND {
            //
            // See whether the opposite ray hits the ellipsoid.
            //
            spicelib::VMINUS(LINEDR.as_slice(), NEGDIR.as_slice_mut());

            spicelib::SURFPT(
                LINEPT.as_slice(),
                NEGDIR.as_slice(),
                A,
                B,
                C,
                XPT.as_slice_mut(),
                &mut FOUND2,
                ctx,
            )?;
        }

        //
        // If we have an intersection case, test the near point
        // and distance.
        //
        if (FOUND || FOUND2) {
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

            testutil::CHCKSD(b"DIST", DIST, b"=", 0.0, 0.0, OK, ctx)?;
        } else {
            //
            // If we have a non-intersection case, test the solution
            // for the orthogonality condition.  Also check the distance
            // of the near point from the line.
            //
            ORTHOG = T_ISNPLN(
                A,
                B,
                C,
                LINEPT.as_slice(),
                LINEDR.as_slice(),
                PNEAR.as_slice(),
                DIST,
                OK,
                ctx,
            )?;

            testutil::CHCKSL(
                b"Does extension of outward normal hit line orthogonally?  Is distance correct?",
                ORTHOG,
                true,
                OK,
                ctx,
            )?;
        }
    }

    //
    //     NPEDLN error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NPEDLN: zero direction vector,", ctx)?;

    spicelib::FILLD(10.0, 3, LINEPT.as_slice_mut());
    spicelib::CLEARD(3, LINEDR.as_slice_mut());

    spicelib::NPEDLN(
        1.0,
        1.0,
        1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NPEDLN: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::FILLD(10.0, 3, LINEPT.as_slice_mut());
    spicelib::FILLD(-1.0, 3, LINEDR.as_slice_mut());

    spicelib::NPEDLN(
        0.0,
        1.0,
        1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::NPEDLN(
        1.0,
        0.0,
        1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::NPEDLN(
        1.0,
        1.0,
        0.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NPEDLN: ellipsoid has one negative-length axis.", ctx)?;

    spicelib::NPEDLN(
        -1.0,
        1.0,
        1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::NPEDLN(
        1.0,
        -1.0,
        1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::NPEDLN(
        1.0,
        1.0,
        -1.0,
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        PNEAR.as_slice_mut(),
        &mut DIST,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    //
    //     This error may not be detected on some systems.  We exclude
    //     it for now.

    //CC    CALL TCASE  ( 'NPEDLN: axis length underflow after squaring.' )

    //CC    CALL NPEDLN (  1.D255, 1.D0, 1.D0,  LINEPT, LINEDR, PNEAR, DIST )
    //CC    CALL CHCKXC ( .TRUE., 'SPICE(DEGENERATECASE)', OK )

    //CC    CALL NPEDLN (  1.D0, 1.D255, 1.D0,  LINEPT, LINEDR, PNEAR, DIST )
    //CC    CALL CHCKXC ( .TRUE., 'SPICE(DEGENERATECASE)', OK )

    //CC    CALL NPEDLN (  1.D0, 1.D0, 1.D255,  LINEPT, LINEDR, PNEAR, DIST )
    //CC    CALL CHCKXC ( .TRUE., 'SPICE(DEGENERATECASE)', OK )

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
