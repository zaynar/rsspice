//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const AXTOL: f64 = 0.00000000000001;
const MSGLEN: i32 = 240;
const NEASY: i32 = 3;
const NSIMPL: i32 = 8;
const NRANDM: i32 = 1000;
const NUMSCL: i32 = 5;
const UBEL: i32 = 9;
const UBPL: i32 = 4;

struct SaveVars {
    EASYA: StackArray<f64, 3>,
    EASYB: StackArray<f64, 3>,
    EASYC: StackArray<f64, 3>,
    EASYPT: StackArray2D<f64, 9>,
    SMPC: StackArray<f64, 8>,
    SMPN: StackArray2D<f64, 24>,
    SMPRAD: StackArray2D<f64, 24>,
    XFOUND: StackArray<bool, 8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut EASYA = StackArray::<f64, 3>::new(1..=NEASY);
        let mut EASYB = StackArray::<f64, 3>::new(1..=NEASY);
        let mut EASYC = StackArray::<f64, 3>::new(1..=NEASY);
        let mut EASYPT = StackArray2D::<f64, 9>::new(1..=3, 1..=NEASY);
        let mut SMPC = StackArray::<f64, 8>::new(1..=NSIMPL);
        let mut SMPN = StackArray2D::<f64, 24>::new(1..=3, 1..=NSIMPL);
        let mut SMPRAD = StackArray2D::<f64, 24>::new(1..=3, 1..=NSIMPL);
        let mut XFOUND = StackArray::<bool, 8>::new(1..=NSIMPL);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(4.0)].into_iter();
            EASYA
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(10.0), Val::D(2.0), Val::D(40.0)].into_iter();
            EASYB
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(20.0), Val::D(400.0)].into_iter();
            EASYC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(2.0),
                Val::D(0.0),
                Val::D(4.0),
                Val::D(0.0),
                Val::D(-8.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            EASYPT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(200000000000000000000.0),
                Val::D(200000000000000000000.0),
                Val::D(200000000000000000000.0),
                Val::D(0.00000000000000000004),
                Val::D(0.00000000000000000004),
                Val::D(0.00000000000000000004),
                Val::D(0.5),
                Val::D(1.0),
                Val::D(4.0),
                Val::D(1000000.0),
                Val::D(2000000.0),
                Val::D(4000000.0),
                Val::D(1000000.0),
                Val::D(1000000.0),
                Val::D(1000000.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
            ]
            .into_iter();
            SMPRAD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(100000000000000000000.0),
                Val::D(-1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.00000000000000000002),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(2.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(2000000.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(500000.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0000000001),
            ]
            .into_iter();
            for I in intrinsics::range(1, NSIMPL, 1) {
                for J in intrinsics::range(1, 3, 1) {
                    SMPN[[J, I]] = clist.next().unwrap().into_f64();
                }
                SMPC[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(true), 7 as usize))
                .chain([Val::L(false)]);

            XFOUND
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            EASYA,
            EASYB,
            EASYC,
            EASYPT,
            SMPC,
            SMPN,
            SMPRAD,
            XFOUND,
        }
    }
}

//$Procedure F_SLICE ( Ellipsoid/plane intersection routine tests )
pub fn F_SLICE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut ALT: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CONST: f64 = 0.0;
    let mut ELLIPS = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut NEAR = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PFACTR: f64 = 0.0;
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut PT = StackArray::<f64, 3>::new(1..=3);
    let mut SFACTR: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut XL = StackArray2D::<f64, 16>::new(1..=2, 1..=NSIMPL);
    let mut SEED: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISELVL: bool = false;
    let mut VALELL: bool = false;

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
    //    T_ISPXED
    //
    // test whether an ellipse is the intersection of a specified
    // plane and ellipsoid.  It does this by testing the four
    // endpoints of the ellipse's axes to see whether they are
    //
    //    - On the ellipsoid:  a point P = (x,y,z) is "on" the
    //      ellipsoid with semi-axis lengths a, b, c if
    //      the "level surface parameter"
    //
    //         lambda = x**2/a**2 + y**2/b**2 + z**2/c**2
    //
    //      is sufficiently close to 1.
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
    // The function
    //
    //    T_ISLIMB
    //
    // test whether an ellipse is the limb defined by a specified
    // viewing point and ellipsoid.  It does this by testing the
    // four endpoints of the ellipse's axes to see whether, for each
    // endpoint, the ray from the viewing point to the endpoint is
    // orthogonal to the ellipsoid's outward normal direction at the
    // endpoint.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SLICE", ctx)?;

    //
    // Perform a number of simple INEDPL tests for which we can compute
    // the intersection ellipse in advance.
    //
    //
    // Set up expected semi-axis lengths for the "simple" tests.
    //
    XL[[1, 1]] = 1.0;
    XL[[2, 1]] = 1.0;

    XL[[1, 2]] = (f64::powf(3.0, 0.5) * 100000000000000000000.0);
    XL[[2, 2]] = (f64::powf(3.0, 0.5) * 100000000000000000000.0);

    XL[[1, 3]] = ((2.0 * f64::powf(3.0, 0.5)) * 0.00000000000000000001);
    XL[[2, 3]] = ((2.0 * f64::powf(3.0, 0.5)) * 0.00000000000000000001);

    XL[[1, 4]] = (0.5 * f64::powf(3.0, 0.5));
    XL[[2, 4]] = f64::powf(3.0, 0.5);

    XL[[1, 5]] = (f64::powf(3.0, 0.5) * 1000000.0);
    XL[[2, 5]] = ((0.5 * f64::powf(3.0, 0.5)) * 1000000.0);

    XL[[1, 6]] = ((0.5 * f64::powf(3.0, 0.5)) * 1000000.0);
    XL[[2, 6]] = ((0.5 * f64::powf(3.0, 0.5)) * 1000000.0);

    XL[[1, 7]] = 0.0;
    XL[[2, 7]] = 0.0;

    for I in 1..=NSIMPL {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            b"An INEDPL case that can be checked by inspection; case #.",
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::VHAT(save.SMPN.subarray([1, I]), NORMAL.as_slice_mut());

        CONST = save.SMPC[I];

        A = save.SMPRAD[[1, I]];
        B = save.SMPRAD[[2, I]];
        C = save.SMPRAD[[3, I]];

        spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INEDPL(
            A,
            B,
            C,
            PLANE.as_slice(),
            ELLIPS.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I != 7) {
            //
            // Check the FOUND flag on all cases but case 7.  Case
            // 7 is a boundary case that may fail on some platforms.
            //
            testutil::CHCKSL(b"FOUND", FOUND, save.XFOUND[I], OK, ctx)?;
        }

        if FOUND {
            //
            // Check the ellipse to make sure it is in the plane
            // and on the surface of the ellipsoid.
            //
            VALELL = T_ISPXED(ELLIPS.as_slice(), A, B, C, PLANE.as_slice(), OK, ctx)?;

            spicelib::EL2CGV(
                ELLIPS.as_slice(),
                CENTER.as_slice_mut(),
                SMAJOR.as_slice_mut(),
                SMINOR.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the lengths of the semi-axes.
            //
            testutil::CHCKSD(
                b"||SMAJOR||",
                spicelib::VNORM(SMAJOR.as_slice()),
                b"~/",
                XL[[1, I]],
                AXTOL,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Now for some more difficult cases.  We'll generate the ellipsoids
    // and planes using random numbers.  There are four components to
    // generate:
    //
    //    - random plane normal vectors
    //    - random ellipsoid axes
    //    - random plane constants
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
        A = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);
        B = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);
        C = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);

        CONST = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);

        //
        // We gotta have a plane normal vector.
        //
        NORMAL[1] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        NORMAL[2] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;
        NORMAL[3] = testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?;

        spicelib::VHATIP(NORMAL.as_slice_mut());

        fstr::assign(
            &mut TITLE,
            b"INEDPL Random case #.  A, B, C = # # #; CONST = #; NORMAL = (#, #, #)",
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", A, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", B, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", C, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", CONST, 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[1], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[2], 14, &mut TITLE, ctx);
        spicelib::REPMD(&TITLE.clone(), b"#", NORMAL[3], 14, &mut TITLE, ctx);

        testutil::TCASE(&TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::NVC2PL(NORMAL.as_slice(), CONST, PLANE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::INEDPL(
            A,
            B,
            C,
            PLANE.as_slice(),
            ELLIPS.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::EL2CGV(
            ELLIPS.as_slice(),
            CENTER.as_slice_mut(),
            SMAJOR.as_slice_mut(),
            SMINOR.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if FOUND {
            //
            // Make sure the intersection ellipse we found is
            // contained in both the plane and the surface of the
            // ellipsoid.
            //
            VALELL = T_ISPXED(ELLIPS.as_slice(), A, B, C, PLANE.as_slice(), OK, ctx)?;

            testutil::CHCKSL(b"Is ellipse valid?", VALELL, true, OK, ctx)?;
        }
    }

    //
    //     INELPL error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INEDPL: invalid plane", ctx)?;

    spicelib::CLEARD(4, PLANE.as_slice_mut());

    spicelib::INEDPL(
        1.0,
        1.0,
        1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDPLANE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INEDPL: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::FILLD(1.0, 3, NORMAL.as_slice_mut());

    spicelib::NVC2PL(NORMAL.as_slice(), 0.0, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INEDPL(
        0.0,
        1.0,
        1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    spicelib::INEDPL(
        1.0,
        0.0,
        1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    spicelib::INEDPL(
        1.0,
        1.0,
        0.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"INEDPL: ellipsoid has one negative-length axis.", ctx)?;

    spicelib::INEDPL(
        -1.0,
        1.0,
        1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    spicelib::INEDPL(
        1.0,
        -1.0,
        1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    spicelib::INEDPL(
        1.0,
        1.0,
        -1.0,
        PLANE.as_slice(),
        ELLIPS.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Now for the EDLIMB tests.
    //
    //
    // Start out with easy cases for which the answers can be found
    // by inspection.
    //
    for I in 1..=NEASY {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            b"An EDLIMB case that can be checked by inspection; case #.",
        );
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        A = save.EASYA[I];
        B = save.EASYB[I];
        C = save.EASYC[I];

        spicelib::VEQU(save.EASYPT.subarray([1, I]), PT.as_slice_mut());

        spicelib::EDLIMB(A, B, C, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        ISELVL = T_ISLIMB(LIMB.as_slice(), A, B, C, PT.as_slice(), OK, ctx)?;

        if *OK {
            VALELL = (ISELVL && *OK);

            testutil::CHCKSL(b"Is ellipse valid?", VALELL, true, OK, ctx)?;
        }
    }

    //
    // Random cases for EDLIMB follow:
    //
    for I in 1..=NRANDM {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"EDLIMB Random case #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Get a scale factor.
        //
        SFACTR = f64::powf(10.0, testutil::T_RANDD(-20.0, 20.0, &mut SEED, ctx)?);

        //
        // Make up ellipsoid axes and viewing point.
        //
        A = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);
        B = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);
        C = (SFACTR * testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?);

        //
        // The viewing point must be outside the ellipsoid.
        //
        PT[1] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        PT[2] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);
        PT[3] = (SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut SEED, ctx)?);

        //
        // In half of the cases, we'll scale PT by widely varying
        // scale factors.  In the other half, we'll keep PT close to
        // the ellipsoid, since these cases are more demanding.
        //
        if (I <= (NRANDM / 2)) {
            PFACTR = f64::powf(10.0, testutil::T_RANDD(0.1, 10.0, &mut SEED, ctx)?);
        } else {
            PFACTR = 1.0;
        }

        spicelib::VSCLIP(PFACTR, PT.as_slice_mut());

        spicelib::NEARPT(PT.as_slice(), A, B, C, NEAR.as_slice_mut(), &mut ALT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // If ALT is negative, just reflect PT about the tangent
        // plane at NEAR.
        //
        if (ALT < 0 as f64) {
            spicelib::SURFNM(A, B, C, NEAR.as_slice(), NORMAL.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VHATIP(NORMAL.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VLCOM(
                1.0,
                NEAR.as_slice(),
                -ALT,
                NORMAL.as_slice(),
                PT.as_slice_mut(),
            );
        }

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::EDLIMB(A, B, C, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        ISELVL = T_ISLIMB(LIMB.as_slice(), A, B, C, PT.as_slice(), OK, ctx)?;

        if *OK {
            VALELL = (ISELVL && *OK);

            testutil::CHCKSL(b"Is ellipse valid?", VALELL, true, OK, ctx)?;
        }
    }

    //
    //     EDLIMB error cases:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"EDLIMB: view point inside ellipsoid.", ctx)?;

    spicelib::CLEARD(3, PT.as_slice_mut());

    spicelib::EDLIMB(1.0, 1.0, 1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"EDLIMB: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::FILLD(10.0, 3, PT.as_slice_mut());

    spicelib::EDLIMB(0.0, 1.0, 1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::EDLIMB(1.0, 0.0, 1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::EDLIMB(1.0, 1.0, 0.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"EDLIMB: ellipsoid has one negative-length axis.", ctx)?;

    spicelib::EDLIMB(-1.0, 1.0, 1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::EDLIMB(1.0, -1.0, 1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::EDLIMB(1.0, 1.0, -1.0, PT.as_slice(), LIMB.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    //     This error may not be detected on some systems.  We exclude
    //     it for now.
    //
    //CC   CALL TCASE  ( 'EDLIMB: squared, scaled axis length underflows.' )

    //CC      PT(1) = 1.D256
    //CC      PT(2) = 1.D0
    //CC      PT(3) = 1.D0

    //CC      CALL EDLIMB ( 1.D0, 1.D0, 1.D255, PT, LIMB )
    //CC      CALL CHCKXC ( .TRUE., 'SPICE(DEGENERATECASE)', OK )

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
