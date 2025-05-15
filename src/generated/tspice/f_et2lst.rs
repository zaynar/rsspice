//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LSK: &[u8] = b"test.tls";
const PCK: &[u8] = b"test.tpc";
const SPK: &[u8] = b"test.bsp";
const AMP: f64 = 0.01;
const DTOL: f64 = 0.00000000000001;
const NTOL: f64 = 0.00000000000001;
const BDNMLN: i32 = 36;
const MSGLEN: i32 = 240;
const TIMLEN: i32 = 50;
const NBODS: i32 = 6;
const NLONG: i32 = 4;
const NTIMES: i32 = 25;
const NSYS: i32 = 2;
const SYNMLN: i32 = 30;

struct SaveVars {
    CORSYS: ActualCharArray,
    LONGS: StackArray<f64, 4>,
    BODIES: StackArray<i32, 6>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CORSYS = ActualCharArray::new(SYNMLN, 1..=NSYS);
        let mut LONGS = StackArray::<f64, 4>::new(1..=NLONG);
        let mut BODIES = StackArray::<i32, 6>::new(1..=NBODS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(199),
                Val::I(299),
                Val::I(399),
                Val::I(301),
                Val::I(599),
                Val::I(501),
            ]
            .into_iter();
            BODIES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"  PLANetocentRIC"), Val::C(b" planETOGRAPhic")].into_iter();
            CORSYS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-90.0), Val::D(0.0), Val::D(90.0), Val::D(180.0)].into_iter();
            LONGS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CORSYS,
            LONGS,
            BODIES,
        }
    }
}

//$Procedure F_ET2LST ( Test the SPICELIB routine ET2LST )
pub fn F_ET2LST(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut AMPM = [b' '; TIMLEN as usize];
    let mut BODNAM = [b' '; BDNMLN as usize];
    let mut SYSNAM = [b' '; SYNMLN as usize];
    let mut TIME = [b' '; TIMLEN as usize];
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut XAMPM = [b' '; TIMLEN as usize];
    let mut XTIME = [b' '; TIMLEN as usize];
    let mut BPMKWD = [b' '; TIMLEN as usize];
    let mut DELTA: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LONG: f64 = 0.0;
    let mut SOLTIM: f64 = 0.0;
    let mut SRFLON: f64 = 0.0;
    let mut Q: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RANGE: f64 = 0.0;
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut SUNLAT: f64 = 0.0;
    let mut SUNLON: f64 = 0.0;
    let mut SUNRAD: f64 = 0.0;
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut RATE: f64 = 0.0;
    let mut BODY: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut HR: i32 = 0;
    let mut MN: i32 = 0;
    let mut SC: i32 = 0;
    let mut XHR: i32 = 0;
    let mut XMN: i32 = 0;
    let mut XSC: i32 = 0;
    let mut N: i32 = 0;
    let mut FOUND: bool = false;

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
    // Test case insensitivity while we're at it.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ET2LST", ctx)?;
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  load LSK, PCK, SPK kernels.", ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check time computations for each body, longitude, and supported
    // coordinate system.
    //

    for BODNO in 1..=NBODS {
        BODY = save.BODIES[BODNO];

        for SYSNO in 1..=NSYS {
            fstr::assign(&mut SYSNAM, save.CORSYS.get(SYSNO));

            for LONGNO in 1..=NLONG {
                LONG = (save.LONGS[LONGNO] * spicelib::RPD(ctx));

                for OFFSET in 1..=NTIMES {
                    //
                    // Set the ET value.
                    //
                    ET = (3600.0 * OFFSET as f64);

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(
                        &mut TITLE,
                        b"Normal case: body = #; system = #; longitude = #; ET = #.",
                    );

                    spicelib::REPMI(&TITLE.clone(), b"#", BODY, &mut TITLE, ctx);
                    spicelib::REPMC(&TITLE.clone(), b"#", &SYSNAM, &mut TITLE);
                    spicelib::REPMF(&TITLE.clone(), b"#", LONG, 6, b"F", &mut TITLE, ctx);
                    spicelib::REPMD(&TITLE.clone(), b"#", ET, 14, &mut TITLE, ctx);
                    testutil::TCASE(&TITLE, ctx)?;

                    //
                    // Compute the expected result directly.
                    //
                    // First, find the rectangular coordinates of
                    // the apparent sub-solar point at ET.
                    //
                    spicelib::BODC2N(BODY, &mut BODNAM, &mut FOUND, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::T_SUBSOL(
                        b"INTERCEPT",
                        &BODNAM,
                        ET,
                        b"LT+S",
                        &BODNAM,
                        SPOINT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Find planetocentric coordinates of the sub-solar
                    // point.
                    //
                    spicelib::RECLAT(SPOINT.as_slice(), &mut SUNRAD, &mut SUNLON, &mut SUNLAT);

                    //
                    //
                    // If the surface point's longitude is given in
                    // the planetographic system, convert it to the
                    // planetocentric system.
                    //
                    if spicelib::EQSTR(&SYSNAM, b"PLANETOGRAPHIC") {
                        spicelib::PGRREC(
                            &BODNAM,
                            LONG,
                            0.0,
                            0.0,
                            1000000.0,
                            0.0,
                            X.as_slice_mut(),
                            ctx,
                        )?;

                        //
                        // The output SRFLON is planetocentric longitude.
                        //
                        spicelib::RECLAT(X.as_slice(), &mut RANGE, &mut SRFLON, &mut LAT);
                    } else {
                        SRFLON = LONG;
                    }

                    //
                    // The offset from noon of solar time at the point
                    // is the planetocentric
                    // longitude offset from the sub-solar point, expressed
                    // as a fraction of 2*pi and scaled by 86400.
                    //
                    DELTA = (SRFLON - SUNLON);

                    if (DELTA > spicelib::PI(ctx)) {
                        DELTA = (DELTA - spicelib::TWOPI(ctx));
                    } else if (DELTA < -spicelib::PI(ctx)) {
                        DELTA = (DELTA + spicelib::TWOPI(ctx));
                    }

                    //
                    // Invert DELTA to account for the body rotation sense.
                    //
                    fstr::assign(&mut BPMKWD, b"BODY#_PM");
                    spicelib::REPMI(&BPMKWD.clone(), b"#", BODY, &mut BPMKWD, ctx);

                    spicelib::GDPOOL(
                        &BPMKWD,
                        2,
                        1,
                        &mut N,
                        std::slice::from_mut(&mut RATE),
                        &mut FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if (RATE < 0.0) {
                        DELTA = -DELTA;
                    }

                    //
                    // Compute SOLTIM representing a time past noon.
                    //
                    SOLTIM = ((86400.0 * DELTA) / spicelib::TWOPI(ctx));

                    //
                    // Convert SOLTIM to seconds past midnight.
                    //
                    SOLTIM = (43200.0 + SOLTIM);

                    //
                    // Convert the solar time to hours, minutes, and
                    // seconds.  These are our expected numeric values.
                    //
                    spicelib::RMAIND(SOLTIM, 3600.0, &mut Q, &mut R, ctx)?;

                    XHR = intrinsics::IDNINT(Q);

                    SOLTIM = (SOLTIM - ((intrinsics::IDNINT(Q) as f64) * 3600.0));

                    spicelib::RMAIND(SOLTIM, 60.0, &mut Q, &mut R, ctx)?;

                    XMN = intrinsics::IDNINT(Q);
                    XSC = (R as i32);

                    //
                    // See whether ET2LST agrees.
                    //
                    spicelib::ET2LST(
                        ET, BODY, LONG, &SYSNAM, &mut HR, &mut MN, &mut SC, &mut TIME, &mut AMPM,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Check results.
                    //
                    testutil::CHCKSI(b"HR", HR, b"=", XHR, 0, OK, ctx)?;
                    testutil::CHCKSI(b"MN", MN, b"=", XMN, 0, OK, ctx)?;
                    testutil::CHCKSI(b"SC", SC, b"=", XSC, 0, OK, ctx)?;

                    //
                    // Create the expected time strings.  Leading
                    // zeros are required.
                    //

                    fstr::assign(&mut XTIME, b"  :  :");

                    spicelib::DPFMT((HR as f64), b"0X", fstr::substr_mut(&mut XTIME, 1..=2), ctx)?;
                    spicelib::DPFMT((MN as f64), b"0X", fstr::substr_mut(&mut XTIME, 4..=5), ctx)?;
                    spicelib::DPFMT((SC as f64), b"0X", fstr::substr_mut(&mut XTIME, 7..=8), ctx)?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSC(b"TIME", &TIME, b"=", &XTIME, OK, ctx)?;

                    //
                    // Create the 12-hour clock string.
                    //
                    fstr::assign(&mut XAMPM, &XTIME);

                    if (HR < 12) {
                        fstr::assign(fstr::substr_mut(&mut XAMPM, 10..=13), b"A.M.");

                        if (HR == 0) {
                            fstr::assign(fstr::substr_mut(&mut XAMPM, 1..=2), b"12");
                        }
                    } else {
                        HR = (HR - 12);

                        if (HR == 0) {
                            fstr::assign(fstr::substr_mut(&mut XAMPM, 1..=2), b"12");
                        } else {
                            spicelib::DPFMT(
                                (HR as f64),
                                b"0X",
                                fstr::substr_mut(&mut XAMPM, 1..=2),
                                ctx,
                            )?;
                        }

                        fstr::assign(fstr::substr_mut(&mut XAMPM, 10..=13), b"P.M.");
                    }

                    testutil::CHCKSC(b"AMPM", &AMPM, b"=", &XAMPM, OK, ctx)?;
                }
            }
        }
    }

    //
    // Now for some exception handling tests.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exception:  body is sun.", ctx)?;

    ET = 0.0;
    BODY = 10;
    fstr::assign(&mut SYSNAM, b"PLANETOCENTRIC");
    LONG = (spicelib::PI(ctx) / 4.0);

    spicelib::ET2LST(
        ET, BODY, LONG, &SYSNAM, &mut HR, &mut MN, &mut SC, &mut TIME, &mut AMPM, ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // According to the routine's specification, the time is always
    // 12 noon on the sun.
    //
    testutil::CHCKSI(b"HR", HR, b"=", 12, 0, OK, ctx)?;
    testutil::CHCKSI(b"MN", MN, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSI(b"SC", SC, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKSC(b"TIME", &TIME, b"=", b"12:00:00", OK, ctx)?;
    testutil::CHCKSC(b"AMPM", &AMPM, b"=", b"12:00:00 P.M.", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error:  unrecognized coordinate system.", ctx)?;

    ET = 0.0;
    BODY = 399;
    fstr::assign(&mut SYSNAM, b"MAGNETOSPHERIC");
    LONG = (spicelib::PI(ctx) / 4.0);

    spicelib::ET2LST(
        ET, BODY, LONG, &SYSNAM, &mut HR, &mut MN, &mut SC, &mut TIME, &mut AMPM, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(UNKNOWNSYSTEM)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error:  no frame associated with body.", ctx)?;

    ET = 0.0;
    BODY = -77;
    fstr::assign(&mut SYSNAM, b"PLANETOCENTRIC");
    LONG = (spicelib::PI(ctx) / 4.0);

    spicelib::ET2LST(
        ET, BODY, LONG, &SYSNAM, &mut HR, &mut MN, &mut SC, &mut TIME, &mut AMPM, ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CANTFINDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete SPK file.", ctx)?;

    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
