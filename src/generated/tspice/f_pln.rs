//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LIMIT: f64 = 0.000000000001;
const MSGLEN: i32 = 240;
const NUMCAS: i32 = 10;
const UBPL: i32 = 4;

struct SaveVars {
    CONST: StackArray<f64, 10>,
    NORMAL: StackArray2D<f64, 30>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CONST = StackArray::<f64, 10>::new(1..=NUMCAS);
        let mut NORMAL = StackArray2D::<f64, 30>::new(1..=3, 1..=NUMCAS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(100000000000000000000.0),
                Val::D(200000000000000000000.0),
                Val::D(300000000000000000000.0),
                Val::D(-100000000000000000000.0),
                Val::D(10000000000.0),
                Val::D(100000.0),
                Val::D(-100000.0),
                Val::D(-10000000000.0),
                Val::D(100000000000000000000.0),
                Val::D(-10000000000.0),
                Val::D(-100000.0),
                Val::D(-100000000000000000000.0),
                Val::D(-0.00000000000000000001),
                Val::D(0.0000000001),
                Val::D(-0.00001),
                Val::D(0.00001),
                Val::D(-0.00000000000000000001),
                Val::D(-0.0000000001),
                Val::D(100000000.0),
                Val::D(0.00000001),
                Val::D(-0.00000001),
            ]
            .into_iter();
            NORMAL
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(-100000000000000000000000000000000000.0),
                Val::D(0.00000000000000000000000000000000004),
                Val::D(2.0),
                Val::D(100000000000000000000.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(1.0),
                Val::D(0.0000000001),
                Val::D(0.00000000000000000001),
                Val::D(-1.0),
            ]
            .into_iter();
            CONST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { CONST, NORMAL }
    }
}

//$Procedure F_PLN ( SPICE plane constructor/decomposition tests )
pub fn F_PLN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut CON: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut NHAT = StackArray::<f64, 3>::new(1..=3);
    let mut NMAG: f64 = 0.0;
    let mut NORM = StackArray::<f64, 3>::new(1..=3);
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut POINT = StackArray::<f64, 3>::new(1..=3);
    let mut POINT2 = StackArray::<f64, 3>::new(1..=3);
    let mut SCLCON: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
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
    testutil::TOPEN(b"F_PLN", ctx)?;

    for CASE in 1..=NUMCAS {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(
            &mut TITLE,
            b"Pass a plane around and see if it comes back unchanged; case #.",
        );

        spicelib::REPMI(&TITLE.clone(), b"#", CASE, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::NVC2PL(
            save.NORMAL.subarray([1, CASE]),
            save.CONST[CASE],
            PLANE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The stored constant must be non-negative.
        //
        spicelib::PL2NVC(PLANE.as_slice(), NORM.as_slice_mut(), &mut CON);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"CON", CON, b">=", 0.0, 0.0, OK, ctx)?;

        spicelib::PL2PSV(
            PLANE.as_slice(),
            POINT.as_slice_mut(),
            V1.as_slice_mut(),
            V2.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The returned point should be the closest one to the origin.
        // Check against SCLCON below.
        //
        DIST = spicelib::VNORM(POINT.as_slice());

        //
        // While we've got the plane in this form, let's perturb POINT.
        // The perturbation shouldn't be too huge compared to POINT, or
        // we'll blow away the accuracy of POINT.
        //
        spicelib::UNORM(
            save.NORMAL.subarray([1, CASE]),
            NHAT.as_slice_mut(),
            &mut NMAG,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        SCLCON = (save.CONST[CASE] / NMAG);

        spicelib::VLCOM3(
            (1000.0 * SCLCON),
            V1.as_slice(),
            (1000.0 * SCLCON),
            V2.as_slice(),
            1.0,
            POINT.as_slice(),
            POINT2.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VEQU(POINT2.as_slice(), POINT.as_slice_mut());

        //
        // Test DIST while we're at it.
        //
        if (DIST != 0.0) {
            testutil::CHCKSD(b"DIST", DIST, b"~/", f64::abs(SCLCON), LIMIT, OK, ctx)?;
        }

        //
        // Ok, keep going.
        //
        spicelib::PSV2PL(
            POINT.as_slice(),
            V1.as_slice(),
            V2.as_slice(),
            PLANE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The stored constant must be non-negative.
        //
        spicelib::PL2NVC(PLANE.as_slice(), NORM.as_slice_mut(), &mut CON);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"CON", CON, b">=", 0.0, 0.0, OK, ctx)?;

        spicelib::PL2NVP(PLANE.as_slice(), NORM.as_slice_mut(), POINT.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The returned point should be the closest one to the origin.
        // Check against SCLCON below.
        //
        DIST = spicelib::VNORM(POINT.as_slice());

        if (DIST != 0.0) {
            testutil::CHCKSD(b"DIST", DIST, b"~/", f64::abs(SCLCON), LIMIT, OK, ctx)?;
        }

        spicelib::NVP2PL(NORM.as_slice(), POINT.as_slice(), PLANE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PL2NVC(PLANE.as_slice(), NORM.as_slice_mut(), &mut CON);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // CON must be non-negative.
        //
        testutil::CHCKSD(b"CON", CON, b">=", 0.0, 0.0, OK, ctx)?;

        //
        // The unit normal must be pretty close in length to 1.
        //
        testutil::CHCKSD(
            b"||NORMAL||",
            spicelib::VNORM(save.NORMAL.as_slice()),
            b"~",
            1.0,
            LIMIT,
            OK,
            ctx,
        )?;

        //
        // Check out the relative error in the plane constant and the
        // angular separation of the original and final normal vectors.
        //
        SEP = spicelib::VSEP(NORM.as_slice(), NHAT.as_slice(), ctx);
        if (save.CONST[CASE] >= 0.0) {
            testutil::CHCKSD(b"CON", CON, b"~/", SCLCON, LIMIT, OK, ctx)?;
            testutil::CHCKSD(b"SEP", SEP, b"~", 0.0, LIMIT, OK, ctx)?;
        } else {
            testutil::CHCKSD(b"-CON", -CON, b"~/", SCLCON, LIMIT, OK, ctx)?;
            testutil::CHCKSD(b"SEP", SEP, b"~", spicelib::PI(ctx), LIMIT, OK, ctx)?;
        }
    }

    //
    //     Now for some error handling tests.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NVC2PL:  pass in zero normal vector", ctx)?;

    spicelib::CLEARD(3, NORM.as_slice_mut());

    spicelib::NVC2PL(NORM.as_slice(), 0.0, PLANE.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"NVP2PL:  pass in zero normal vector", ctx)?;

    spicelib::NVP2PL(NORM.as_slice(), POINT.as_slice(), PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"PSV2PL error case:  dependent spanning vectors.", ctx)?;

    spicelib::PSV2PL(
        POINT.as_slice(),
        V1.as_slice(),
        V1.as_slice(),
        PLANE.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
