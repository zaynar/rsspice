//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PCK: &[u8] = b"f_recpgr.tpc";
const TIGHT: f64 = 0.00000000001;
const VTIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.0000000001;
const FILSIZ: i32 = 255;
const NBODS: i32 = 75;
const LNSIZE: i32 = 80;
const MAXCOF: i32 = 3;
const NCASE: i32 = 18;
const NAMLEN: i32 = 32;
const NSPEC: i32 = 3;

struct SaveVars {
    BODY: ActualCharArray,
    SPCIAL: ActualCharArray,
    RECTAN: StackArray2D<f64, 54>,
    XALT: StackArray<f64, 18>,
    XLAT: StackArray<f64, 18>,
    XLON: StackArray<f64, 18>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BODY = ActualCharArray::new(NAMLEN, 1..=NCASE);
        let mut SPCIAL = ActualCharArray::new(NAMLEN, 1..=NSPEC);
        let mut RECTAN = StackArray2D::<f64, 54>::new(1..=3, 1..=NCASE);
        let mut XALT = StackArray::<f64, 18>::new(1..=NCASE);
        let mut XLAT = StackArray::<f64, 18>::new(1..=NCASE);
        let mut XLON = StackArray::<f64, 18>::new(1..=NCASE);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
                Val::C(b"EARTH"),
            ]
            .into_iter();
            for I in intrinsics::range(1, 9, 1) {
                fstr::assign(BODY.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
                Val::C(b"MARS"),
            ]
            .into_iter();
            for I in intrinsics::range(10, NCASE, 1) {
                fstr::assign(BODY.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(6378.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-6378.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-6388.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-6368.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-6378.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(6378.14),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(6356.75),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-6356.75),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 9, 1) {
                for J in intrinsics::range(1, 3, 1) {
                    RECTAN[[J, I]] = clist.next().unwrap().into_f64();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(3397.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-3397.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-3407.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-3387.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-3397.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(3397.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(3375.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-3375.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(10, NCASE, 1) {
                for J in intrinsics::range(1, 3, 1) {
                    RECTAN[[J, I]] = clist.next().unwrap().into_f64();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(10.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(-10.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(-6356.75),
            ]
            .into_iter();
            for I in intrinsics::range(1, 9, 1) {
                XLON[I] = clist.next().unwrap().into_f64();
                XLAT[I] = clist.next().unwrap().into_f64();
                XALT[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(10.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(-10.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(-3375.0),
            ]
            .into_iter();
            for I in intrinsics::range(10, NCASE, 1) {
                XLON[I] = clist.next().unwrap().into_f64();
                XLAT[I] = clist.next().unwrap().into_f64();
                XALT[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"EARTH"), Val::C(b"MOON"), Val::C(b"SUN")].into_iter();
            SPCIAL
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BODY,
            SPCIAL,
            RECTAN,
            XALT,
            XLAT,
            XLON,
        }
    }
}

//$Procedure      F_PGR ( Planetographic coordinate tests )
pub fn F_PGR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; LNSIZE as usize];
    let mut ALT: f64 = 0.0;
    let mut COEFFS = StackArray::<f64, 3>::new(1..=MAXCOF);
    let mut F: f64 = 0.0;
    let mut GALT: f64 = 0.0;
    let mut GLAT: f64 = 0.0;
    let mut GLON: f64 = 0.0;
    let mut JAC2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;
    let mut REC = StackArray::<f64, 3>::new(1..=3);
    let mut RP: f64 = 0.0;
    let mut XREC = StackArray::<f64, 3>::new(1..=3);
    let mut XTEMP: f64 = 0.0;
    let mut N: i32 = 0;

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
    testutil::TOPEN(b"F_PGR", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create full text PCK file.", ctx)?;
    //
    // Create, load, and delete a PCK kernel.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Loop through the canned cases.
    //
    for I in 1..=NCASE {
        fstr::assign(&mut TITLE, b"Canned case #; BODY = #.");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMC(&TITLE.clone(), b"#", &save.BODY[I], &mut TITLE);

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::BODVRD(
            &save.BODY[I],
            b"RADII",
            3,
            &mut N,
            RADII.as_slice_mut(),
            ctx,
        )?;

        RE = RADII[1];
        RP = RADII[3];

        F = ((RE - RP) / RE);

        //
        // Test RECPGR.
        //
        spicelib::RECPGR(
            &save.BODY[I],
            save.RECTAN.subarray([1, I]),
            RE,
            F,
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        LON = (spicelib::DPR(ctx) * LON);
        LAT = (spicelib::DPR(ctx) * LAT);

        testutil::CHCKSD(b"LON", LON, b"~", save.XLON[I], VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"LAT", LAT, b"~", save.XLAT[I], VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"ALT", ALT, b"~", save.XALT[I], TIGHT, OK, ctx)?;

        //
        // Test PGRREC.
        //
        spicelib::PGRREC(
            &save.BODY[I],
            (spicelib::RPD(ctx) * save.XLON[I]),
            (spicelib::RPD(ctx) * save.XLAT[I]),
            save.XALT[I],
            RE,
            F,
            REC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Test the vector relative error if the test point is not
        // at the origin.
        //
        if !spicelib::VZERO(save.RECTAN.subarray([1, I])) {
            testutil::CHCKAD(
                b"REC",
                REC.as_slice(),
                b"~~/",
                save.RECTAN.subarray([1, I]),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;
        } else {
            //
            // Check component-wise differences.
            //
            testutil::CHCKAD(
                b"REC",
                REC.as_slice(),
                b"~",
                save.RECTAN.subarray([1, I]),
                3,
                VTIGHT,
                OK,
                ctx,
            )?;
        }
    }

    //
    //     Run a non-axis test case.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Test non-axis case against RECGEO for oblate body.", ctx)?;

    spicelib::VPACK(100000.0, 100000.0, 100000.0, XREC.as_slice_mut());

    spicelib::BODVRD(b"Mars", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RE = RADII[1];
    RP = RADII[3];

    F = ((RE - RP) / RE);

    //
    // Test RECPGR.
    //
    spicelib::RECPGR(
        b"MARS",
        XREC.as_slice(),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECGEO(XREC.as_slice(), RE, F, &mut GLON, &mut GLAT, &mut GALT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Mars is prograde, so flip the sign of the longitude.
    //
    GLON = -GLON;

    if (GLON < 0.0) {
        GLON = (GLON + spicelib::TWOPI(ctx));
    }

    testutil::CHCKSD(b"LON", LON, b"~", GLON, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"LAT", LAT, b"~", GLAT, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ALT", ALT, b"~", GALT, TIGHT, OK, ctx)?;

    //
    // Now test PGRREC.
    //
    spicelib::PGRREC(b"MARS", LON, LAT, ALT, RE, F, REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"REC",
        REC.as_slice(),
        b"~~/",
        XREC.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Repeat previous test with Mars longitude set to positive east.",
        ctx,
    )?;

    spicelib::PCPOOL(
        b"BODY499_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"EAST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test RECPGR.
    //
    spicelib::RECPGR(
        b"MARS",
        XREC.as_slice(),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECGEO(XREC.as_slice(), RE, F, &mut GLON, &mut GLAT, &mut GALT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if (GLON < 0.0) {
        GLON = (GLON + spicelib::TWOPI(ctx));
    }

    testutil::CHCKSD(b"LON", LON, b"~", GLON, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"LAT", LAT, b"~", GLAT, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ALT", ALT, b"~", GALT, TIGHT, OK, ctx)?;

    //
    // Now test PGRREC.
    //
    spicelib::PGRREC(b"MARS", LON, LAT, ALT, RE, F, REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"REC",
        REC.as_slice(),
        b"~~/",
        XREC.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Clear the override value from the pool.
    //
    spicelib::DVPOOL(b"BODY499_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Repeat previous test with Mars PM values absent from the kernel pool.",
        ctx,
    )?;

    //
    // Capture the correct values before starting.
    //
    spicelib::BODVRD(b"Mars", b"PM", 3, &mut N, COEFFS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY499_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(
        b"BODY499_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"EAST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Test RECPGR.
    //
    spicelib::RECPGR(
        b"MARS",
        XREC.as_slice(),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECGEO(XREC.as_slice(), RE, F, &mut GLON, &mut GLAT, &mut GALT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if (GLON < 0.0) {
        GLON = (GLON + spicelib::TWOPI(ctx));
    }

    testutil::CHCKSD(b"LON", LON, b"~", GLON, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"LAT", LAT, b"~", GLAT, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ALT", ALT, b"~", GALT, TIGHT, OK, ctx)?;

    //
    // Now test PGRREC.
    //
    spicelib::PGRREC(b"MARS", LON, LAT, ALT, RE, F, REC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"REC",
        REC.as_slice(),
        b"~~/",
        XREC.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // Clear the override value from the pool.
    //
    spicelib::DVPOOL(b"BODY499_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the Mars PM coefficients.
    //
    spicelib::PDPOOL(b"BODY499_PM", 3, COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check longitude sense for earth, moon, and sun.", ctx)?;

    for I in 1..=NSPEC {
        //
        // Test RECPGR.
        //
        spicelib::RECPGR(
            &save.SPCIAL[I],
            XREC.as_slice(),
            RE,
            F,
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::RECGEO(XREC.as_slice(), RE, F, &mut GLON, &mut GLAT, &mut GALT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (GLON < 0.0) {
            GLON = (GLON + spicelib::TWOPI(ctx));
        }

        testutil::CHCKSD(b"LON", LON, b"~", GLON, VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"LAT", LAT, b"~", GLAT, VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"ALT", ALT, b"~", GALT, TIGHT, OK, ctx)?;

        //
        // Now test PGRREC.
        //
        spicelib::PGRREC(
            &save.SPCIAL[I],
            LON,
            LAT,
            ALT,
            RE,
            F,
            REC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"REC",
            REC.as_slice(),
            b"~~/",
            XREC.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat tests with longitudes set to positive west.", ctx)?;

    spicelib::PCPOOL(
        b"BODY399_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(
        b"BODY301_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(
        b"BODY10_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NSPEC {
        //
        // Test RECPGR.
        //
        spicelib::RECPGR(
            &save.SPCIAL[I],
            XREC.as_slice(),
            RE,
            F,
            &mut LON,
            &mut LAT,
            &mut ALT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::RECGEO(XREC.as_slice(), RE, F, &mut GLON, &mut GLAT, &mut GALT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        GLON = -GLON;

        if (GLON < 0.0) {
            GLON = (GLON + spicelib::TWOPI(ctx));
        }

        testutil::CHCKSD(b"LON", LON, b"~", GLON, VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"LAT", LAT, b"~", GLAT, VTIGHT, OK, ctx)?;
        testutil::CHCKSD(b"ALT", ALT, b"~", GALT, TIGHT, OK, ctx)?;

        //
        // Now test PGRREC.
        //
        spicelib::PGRREC(
            &save.SPCIAL[I],
            LON,
            LAT,
            ALT,
            RE,
            F,
            REC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"REC",
            REC.as_slice(),
            b"~~/",
            XREC.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    spicelib::DVPOOL(b"BODY399_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY301_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DVPOOL(b"BODY10_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Error cases.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: equatorial radius is zero.", ctx)?;

    //
    // Capture the correct values before starting.
    //
    spicelib::BODVRD(b"Earth", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RE = RADII[1];
    RP = RADII[3];

    F = ((RE - RP) / RE);

    spicelib::RECPGR(
        b"EARTH",
        save.RECTAN.subarray([1, 1]),
        0.0,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"EARTH",
        LON,
        LAT,
        ALT,
        0.0,
        F,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: flattening factor is 1.", ctx)?;

    spicelib::RECPGR(
        b"EARTH",
        save.RECTAN.subarray([1, 1]),
        RE,
        1.0,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"EARTH",
        LON,
        LAT,
        ALT,
        RE,
        1.0,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: flattening factor is > 1.", ctx)?;

    spicelib::RECPGR(
        b"EARTH",
        save.RECTAN.subarray([1, 1]),
        RE,
        2.0,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"EARTH",
        LON,
        LAT,
        ALT,
        RE,
        2.0,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no Mars PM terms in pool.", ctx)?;
    //
    // Capture the correct values before starting.
    //
    spicelib::BODVRD(b"Mars", b"PM", 3, &mut N, COEFFS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete Mars' PM coefficients from the kernel pool.
    //
    spicelib::DVPOOL(b"BODY499_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECPGR(
        b"MARS",
        save.RECTAN.subarray([1, 1]),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"MARS",
        LON,
        LAT,
        ALT,
        RE,
        F,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no rate term for Mars PM.", ctx)?;
    //
    // Overwrite Mars' PM coefficients in the kernel pool.
    //
    spicelib::PDPOOL(b"BODY499_PM", 1, &[0.0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECPGR(
        b"MARS",
        save.RECTAN.subarray([1, 1]),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"MARS",
        LON,
        LAT,
        ALT,
        RE,
        F,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    //
    // Restore the good PM coefficients.
    //
    spicelib::PDPOOL(b"BODY499_PM", 3, COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: invalid override spec.", ctx)?;

    spicelib::PCPOOL(
        b"BODY499_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"EST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::RECPGR(
        b"MARS",
        save.RECTAN.subarray([1, 1]),
        RE,
        F,
        &mut LON,
        &mut LAT,
        &mut ALT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::PGRREC(
        b"MARS",
        LON,
        LAT,
        ALT,
        RE,
        F,
        save.RECTAN.subarray_mut([1, 1]),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::DVPOOL(b"BODY499_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // *****************************************************************
    // *****************************************************************
    // *****************************************************************
    //     Jacobian matrix routine tests
    // *****************************************************************
    // *****************************************************************
    // *****************************************************************

    //
    // Loop through the canned cases.
    //
    for I in 1..=NCASE {
        fstr::assign(&mut TITLE, b"Canned DPGRDR case #; BODY = #.");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMC(&TITLE.clone(), b"#", &save.BODY[I], &mut TITLE);

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::BODVRD(
            &save.BODY[I],
            b"RADII",
            3,
            &mut N,
            RADII.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        RE = RADII[1];
        RP = RADII[3];

        F = ((RE - RP) / RE);

        spicelib::DRDPGR(
            &save.BODY[I],
            save.XLON[I],
            save.XLAT[I],
            save.XALT[I],
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EQSTR(&save.BODY[I], b"Mars") {
            XTEMP = -save.XLON[I];
        } else {
            XTEMP = save.XLON[I];
        }

        spicelib::DRDGEO(
            XTEMP,
            save.XLAT[I],
            save.XALT[I],
            RE,
            F,
            JAC2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // For comparison, negate the partials w.r.t. geodetic longitude
        // for Mars.
        //
        if spicelib::EQSTR(&save.BODY[I], b"Mars") {
            for J in 1..=3 {
                JAC2[[J, 1]] = -JAC2[[J, 1]];
            }
        }

        testutil::CHCKAD(
            b"JACOBI",
            JACOBI.as_slice(),
            b"~",
            JAC2.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // Loop through the canned cases for DRDPGR.
    //
    for I in 1..=NCASE {
        fstr::assign(&mut TITLE, b"Canned DRDPGR case #; BODY = #.");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        spicelib::REPMC(&TITLE.clone(), b"#", &save.BODY[I], &mut TITLE);

        //
        // --- Case: ------------------------------------------------------
        //
        testutil::TCASE(&TITLE, ctx)?;

        spicelib::BODVRD(
            &save.BODY[I],
            b"RADII",
            3,
            &mut N,
            RADII.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        RE = RADII[1];
        RP = RADII[3];

        F = ((RE - RP) / RE);

        spicelib::DPGRDR(
            &save.BODY[I],
            save.RECTAN[[1, I]],
            save.RECTAN[[2, I]],
            save.RECTAN[[3, I]],
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;

        if spicelib::VZEROG(save.RECTAN.subarray([1, I]), 2) {
            testutil::CHCKXC(true, b"SPICE(POINTONZAXIS)", OK, ctx)?;
        } else {
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DGEODR(
                save.RECTAN[[1, I]],
                save.RECTAN[[2, I]],
                save.RECTAN[[3, I]],
                RE,
                F,
                JAC2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // For comparison, negate the gradient of geodetic longitude
            // for Mars.

            if spicelib::EQSTR(&save.BODY[I], b"Mars") {
                for J in 1..=3 {
                    JAC2[[1, J]] = -JAC2[[1, J]];
                }
            }

            testutil::CHCKAD(
                b"JACOBI",
                JACOBI.as_slice(),
                b"~",
                JAC2.as_slice(),
                9,
                VTIGHT,
                OK,
                ctx,
            )?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check longitude sense for earth, moon, and sun.", ctx)?;

    spicelib::VPACK(100000.0, 100000.0, 100000.0, XREC.as_slice_mut());

    for I in 1..=NSPEC {
        //
        // Test DPGRDR.
        //
        spicelib::DPGRDR(
            &save.SPCIAL[I],
            XREC[1],
            XREC[2],
            XREC[3],
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DGEODR(XREC[1], XREC[2], XREC[3], RE, F, JAC2.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"JACOBI",
            JACOBI.as_slice(),
            b"~",
            JAC2.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
        //
        // Now test DRDPGR.
        //
        spicelib::DRDPGR(
            &save.SPCIAL[I],
            LON,
            LAT,
            ALT,
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DRDGEO(LON, LAT, ALT, RE, F, JAC2.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            b"JACOBI",
            JACOBI.as_slice(),
            b"~",
            JAC2.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Repeat tests with longitudes set to positive west.", ctx)?;

    spicelib::PCPOOL(
        b"BODY399_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(
        b"BODY301_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(
        b"BODY10_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"WEST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NSPEC {
        //
        // Test DPGRDR.
        //
        spicelib::DPGRDR(
            &save.SPCIAL[I],
            XREC[1],
            XREC[2],
            XREC[3],
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DGEODR(XREC[1], XREC[2], XREC[3], RE, F, JAC2.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Negate the gradient of geodetic longitude.
        //
        for J in 1..=3 {
            JAC2[[1, J]] = -JAC2[[1, J]];
        }

        testutil::CHCKAD(
            b"JACOBI",
            JACOBI.as_slice(),
            b"~",
            JAC2.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;

        //
        // Now test DRDPGR.
        //
        spicelib::DRDPGR(
            &save.SPCIAL[I],
            LON,
            LAT,
            ALT,
            RE,
            F,
            JACOBI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DRDGEO(LON, LAT, ALT, RE, F, JAC2.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Negate all partials with respect to geodetic longitude.
        //
        for J in 1..=3 {
            JAC2[[J, 1]] = -JAC2[[J, 1]];
        }

        testutil::CHCKAD(
            b"JACOBI",
            JACOBI.as_slice(),
            b"~",
            JAC2.as_slice(),
            9,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    //     Error cases.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: equatorial radius is zero.", ctx)?;

    //
    // Capture the correct values before starting.
    //
    spicelib::BODVRD(b"Earth", b"RADII", 3, &mut N, RADII.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    RE = RADII[1];
    RP = RADII[3];

    F = ((RE - RP) / RE);

    spicelib::DPGRDR(
        b"EARTH",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        0.0,
        F,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"EARTH", LON, LAT, ALT, 0.0, F, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: flattening factor is 1.", ctx)?;

    spicelib::DPGRDR(
        b"EARTH",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        RE,
        1.0,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"EARTH", LON, LAT, ALT, RE, 1.0, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: flattening factor is > 1.", ctx)?;

    spicelib::DPGRDR(
        b"EARTH",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        RE,
        2.0,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"EARTH", LON, LAT, ALT, RE, 2.0, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no Mars PM terms in pool.", ctx)?;
    //
    // Capture the correct values before starting.
    //
    spicelib::BODVRD(b"Mars", b"PM", 3, &mut N, COEFFS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete Mars' PM coefficients from the kernel pool.
    //
    spicelib::DVPOOL(b"BODY499_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DPGRDR(
        b"MARS",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        RE,
        F,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"MARS", LON, LAT, ALT, RE, F, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: no rate term for Mars PM.", ctx)?;
    //
    // Overwrite Mars' PM coefficients in the kernel pool.
    //
    spicelib::PDPOOL(b"BODY499_PM", 1, &[0.0], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DPGRDR(
        b"MARS",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        RE,
        F,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"MARS", LON, LAT, ALT, RE, F, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(MISSINGDATA)", OK, ctx)?;

    //
    // Restore the good PM coefficients.
    //
    spicelib::PDPOOL(b"BODY499_PM", 3, COEFFS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error case: invalid override spec.", ctx)?;

    spicelib::PCPOOL(
        b"BODY499_PGR_POSITIVE_LON",
        1,
        CharArray::from_ref(b"EST"),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DPGRDR(
        b"MARS",
        save.RECTAN[[1, 1]],
        save.RECTAN[[2, 1]],
        save.RECTAN[[3, 1]],
        RE,
        F,
        JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    LON = 0.0;
    LAT = 0.0;
    ALT = 100000.0;
    spicelib::DRDPGR(b"MARS", LON, LAT, ALT, RE, F, JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::DVPOOL(b"BODY499_PGR_POSITIVE_LON", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
