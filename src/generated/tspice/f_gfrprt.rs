//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const REPFIL: &[u8] = b"zzgfrpwk.txt";
const VTIGHT: f64 = 0.000000000001;
const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const MAXWIN: i32 = 400;

struct SaveVars {
    BEGMSG: Vec<u8>,
    ENDMSG: Vec<u8>,
    QNAME: Vec<u8>,
    XBEGMS: Vec<u8>,
    XENDMS: Vec<u8>,
    DELTA: f64,
    FINISH: f64,
    FREQ: f64,
    INCR: f64,
    SPAN: f64,
    START: f64,
    T: f64,
    TOTAL: f64,
    XFREQ: f64,
    WINDOW: ActualArray<f64>,
    XINCR: f64,
    XTOTAL: f64,
    NSAMP: i32,
    TCHECK: i32,
    UNIT: i32,
    XCHECK: i32,
    XUNIT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGMSG = vec![b' '; MXBEGM as usize];
        let mut ENDMSG = vec![b' '; MXENDM as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut XBEGMS = vec![b' '; MXBEGM as usize];
        let mut XENDMS = vec![b' '; MXENDM as usize];
        let mut DELTA: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut FREQ: f64 = 0.0;
        let mut INCR: f64 = 0.0;
        let mut SPAN: f64 = 0.0;
        let mut START: f64 = 0.0;
        let mut T: f64 = 0.0;
        let mut TOTAL: f64 = 0.0;
        let mut XFREQ: f64 = 0.0;
        let mut WINDOW = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut XINCR: f64 = 0.0;
        let mut XTOTAL: f64 = 0.0;
        let mut NSAMP: i32 = 0;
        let mut TCHECK: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut XCHECK: i32 = 0;
        let mut XUNIT: i32 = 0;

        Self {
            BEGMSG,
            ENDMSG,
            QNAME,
            XBEGMS,
            XENDMS,
            DELTA,
            FINISH,
            FREQ,
            INCR,
            SPAN,
            START,
            T,
            TOTAL,
            XFREQ,
            WINDOW,
            XINCR,
            XTOTAL,
            NSAMP,
            TCHECK,
            UNIT,
            XCHECK,
            XUNIT,
        }
    }
}

//$Procedure      F_GFRPRT ( Test GFRPRT entry points )
pub fn F_GFRPRT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved everything
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFRPRT", ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Test GFREPI: Make sure correct parameters are passed to ZZGFTSWK.",
        ctx,
    )?;

    //
    // Open a text file to capture the progress report.
    //
    if spicelib::EXISTS(REPFIL, ctx)? {
        spicelib::DELFIL(REPFIL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::TXTOPN(REPFIL, &mut save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWKUN(save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window.
    //
    spicelib::SSIZED(MAXWIN, save.WINDOW.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in intrinsics::range(1, (MAXWIN - 1), 2) {
        spicelib::WNINSD(
            (I as f64),
            ((I + 1) as f64),
            save.WINDOW.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XTOTAL = ((MAXWIN as f64) / 2.0);

    fstr::assign(
        &mut save.XBEGMS,
        b"@--------------------Message prefix-------------------@",
    );
    fstr::assign(&mut save.XENDMS, b"@**suffix***@");

    //
    // Call the progress report initializer.
    //
    spicelib::GFREPI(save.WINDOW.as_slice(), &save.XBEGMS, &save.XENDMS, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the current report parameters from the ZZGFRPWK monitor.
    //
    spicelib::ZZGFWKMO(
        &mut save.UNIT,
        &mut save.TOTAL,
        &mut save.FREQ,
        &mut save.TCHECK,
        &mut save.BEGMSG,
        &mut save.ENDMSG,
        &mut save.INCR,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the stored values from ZZGFWKMO against the expected
    // values.
    //
    // The default poll duration and call count are 1 second and
    // 4 calls, respectively. These values are hard-coded in GFREPI.
    //
    save.XFREQ = 1.0;
    save.XCHECK = 4;

    testutil::CHCKSI(b"UNIT", save.UNIT, b"=", save.XUNIT, 0, OK, ctx)?;
    testutil::CHCKSD(b"TOTAL", save.TOTAL, b"~", save.XTOTAL, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"FREQ", save.FREQ, b"=", save.XFREQ, 0.0, OK, ctx)?;
    testutil::CHCKSI(b"TCHECK", save.TCHECK, b"=", save.XCHECK, 0, OK, ctx)?;
    testutil::CHCKSC(b"BEGMSG", &save.BEGMSG, b"=", &save.XBEGMS, OK, ctx)?;
    testutil::CHCKSC(b"ENDMSG", &save.ENDMSG, b"=", &save.XENDMS, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Test GFREPU: Make sure correct work increments are passed to ZZGFWKIN.",
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.WINDOW.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.WINDOW.as_slice(),
            I,
            &mut save.START,
            &mut save.FINISH,
            ctx,
        )?;

        save.SPAN = (save.FINISH - save.START);

        //
        // Use progressively smaller samples.
        //
        save.NSAMP = (I + 2);

        save.DELTA = (save.SPAN / (save.NSAMP - 1) as f64);

        for J in 1..=save.NSAMP {
            save.T = (save.START + (((J - 1) as f64) * save.DELTA));

            spicelib::GFREPU(save.START, save.FINISH, save.T, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::ZZGFWKMO(
                &mut save.UNIT,
                &mut save.TOTAL,
                &mut save.FREQ,
                &mut save.TCHECK,
                &mut save.BEGMSG,
                &mut save.ENDMSG,
                &mut save.INCR,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect the last work increment to be DELTA for all
            // but the first value of T.
            //
            if (J == 1) {
                save.XINCR = 0.0;
            } else {
                save.XINCR = save.DELTA;
            }

            fstr::assign(&mut save.QNAME, b"INCR I=1*; J=*");

            spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", J, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.QNAME, save.INCR, b"~", save.XINCR, VTIGHT, OK, ctx)?;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Test GFREPF: Make sure correct parameters are passed to ZZGFWKAD and ZZGFWKIN.",
        ctx,
    )?;

    spicelib::GFREPF(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Fetch the current report parameters from the ZZGFRPWK monitor.
    //
    spicelib::ZZGFWKMO(
        &mut save.UNIT,
        &mut save.TOTAL,
        &mut save.FREQ,
        &mut save.TCHECK,
        &mut save.BEGMSG,
        &mut save.ENDMSG,
        &mut save.INCR,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the stored values from ZZGFWKMO against the expected
    // values.
    //
    // GFREPF is supposed to set the poll duration and call count to 0
    // seconds and 1 call, respectively. The work increment should be
    // set to 0.D0. These values are hard-coded in GFREPF.
    //
    save.XFREQ = 0.0;
    save.XCHECK = 1;

    testutil::CHCKSI(b"UNIT", save.UNIT, b"=", save.XUNIT, 0, OK, ctx)?;
    testutil::CHCKSD(b"TOTAL", save.TOTAL, b"~", save.XTOTAL, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"FREQ", save.FREQ, b"=", save.XFREQ, 0.0, OK, ctx)?;
    testutil::CHCKSI(b"TCHECK", save.TCHECK, b"=", save.XCHECK, 0, OK, ctx)?;
    testutil::CHCKSC(b"BEGMSG", &save.BEGMSG, b"=", &save.XBEGMS, OK, ctx)?;
    testutil::CHCKSC(b"ENDMSG", &save.ENDMSG, b"=", &save.XENDMS, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Clean up: delete log file.", ctx)?;

    if spicelib::EXISTS(REPFIL, ctx)? {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.XUNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        spicelib::DELFIL(REPFIL, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Call to umbrella routine", ctx)?;

    spicelib::GFRPRT(save.WINDOW.as_slice(), b" ", b" ", 0.0, 1.0, 0.5, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"GFREPI: report message prefix is too long", ctx)?;

    spicelib::GFREPI(save.WINDOW.as_slice(), b"@------------------------------------------------------------------------------------------#", b" ", ctx)?;

    testutil::CHCKXC(true, b"SPICE(MESSAGETOOLONG)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"GFREPI: report message suffix is too long", ctx)?;

    spicelib::GFREPI(save.WINDOW.as_slice(), b" ", b"@------------------------------------------------------------------------------------------#", ctx)?;

    testutil::CHCKXC(true, b"SPICE(MESSAGETOOLONG)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"GFREPI: report message prefix contains non-printing characters.",
        ctx,
    )?;

    spicelib::GFREPI(
        save.WINDOW.as_slice(),
        &fstr::concat(b"@-------#", &intrinsics::CHAR(4)),
        b" ",
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"GFREPI: report message suffix contains non-printing characters.",
        ctx,
    )?;

    spicelib::GFREPI(
        save.WINDOW.as_slice(),
        b" ",
        &fstr::concat(b"#-------!", &intrinsics::CHAR(5)),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"GFREPU: interval bounds out of order.", ctx)?;

    spicelib::GFREPU(1.0, 0.0, 2.0, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADENDPOINTS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"GFREPU: input time outside of interval.", ctx)?;

    spicelib::GFREPU(1.0, 1.5, 2.0, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
