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
const LNSIZE: i32 = 80;
const MAXWIN: i32 = 400;

struct SaveVars {
    BEGMSG: Vec<u8>,
    ENDMSG: Vec<u8>,
    LINE: Vec<u8>,
    PCTSTR: Vec<u8>,
    QNAME: Vec<u8>,
    XBEGMS: Vec<u8>,
    XENDMS: Vec<u8>,
    XLINE: Vec<u8>,
    FRAC: f64,
    INCR: f64,
    TOTAL: f64,
    WAIT: f64,
    XINCR: f64,
    XTOTAL: f64,
    XWAIT: f64,
    IOSTAT: i32,
    NRPORT: i32,
    TCHECK: i32,
    UNIT: i32,
    XCHECK: i32,
    XNL: i32,
    XUNIT: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGMSG = vec![b' '; MXBEGM as usize];
        let mut ENDMSG = vec![b' '; MXENDM as usize];
        let mut LINE = vec![b' '; LNSIZE as usize];
        let mut PCTSTR = vec![b' '; LNSIZE as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut XBEGMS = vec![b' '; MXBEGM as usize];
        let mut XENDMS = vec![b' '; MXENDM as usize];
        let mut XLINE = vec![b' '; LNSIZE as usize];
        let mut FRAC: f64 = 0.0;
        let mut INCR: f64 = 0.0;
        let mut TOTAL: f64 = 0.0;
        let mut WAIT: f64 = 0.0;
        let mut XINCR: f64 = 0.0;
        let mut XTOTAL: f64 = 0.0;
        let mut XWAIT: f64 = 0.0;
        let mut IOSTAT: i32 = 0;
        let mut NRPORT: i32 = 0;
        let mut TCHECK: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut XCHECK: i32 = 0;
        let mut XNL: i32 = 0;
        let mut XUNIT: i32 = 0;

        Self {
            BEGMSG,
            ENDMSG,
            LINE,
            PCTSTR,
            QNAME,
            XBEGMS,
            XENDMS,
            XLINE,
            FRAC,
            INCR,
            TOTAL,
            WAIT,
            XINCR,
            XTOTAL,
            XWAIT,
            IOSTAT,
            NRPORT,
            TCHECK,
            UNIT,
            XCHECK,
            XNL,
            XUNIT,
        }
    }
}

//$Procedure      F_ZZGFRPWK ( Test ZZGFRPWK entry points )
pub fn F_ZZGFRPWK(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_ZZGFRPWK", ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test ZZGFTSWK parameter storage.", ctx)?;

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

    save.XTOTAL = ((MAXWIN as f64) / 2.0);

    fstr::assign(
        &mut save.XBEGMS,
        b"@-------------------Message prefix--------------------@",
    );
    fstr::assign(&mut save.XENDMS, b"@***suffix**@");

    save.XWAIT = 2.0;
    save.XCHECK = 10;

    //
    // Call the progress report display initializer.
    //
    spicelib::ZZGFTSWK(
        save.XTOTAL,
        save.XWAIT,
        save.XCHECK,
        &save.XBEGMS,
        &save.XENDMS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch the current report parameters from the ZZGFRPWK monitor.
    //
    spicelib::ZZGFWKMO(
        &mut save.UNIT,
        &mut save.TOTAL,
        &mut save.WAIT,
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

    testutil::CHCKSI(b"UNIT", save.UNIT, b"=", save.XUNIT, 0, OK, ctx)?;
    testutil::CHCKSD(b"TOTAL", save.TOTAL, b"~", save.XTOTAL, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"WAIT", save.WAIT, b"=", save.XWAIT, 0.0, OK, ctx)?;
    testutil::CHCKSI(b"TCHECK", save.TCHECK, b"=", save.XCHECK, 0, OK, ctx)?;
    testutil::CHCKSC(b"BEGMSG", &save.BEGMSG, b"=", &save.XBEGMS, OK, ctx)?;
    testutil::CHCKSC(b"ENDMSG", &save.ENDMSG, b"=", &save.XENDMS, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test the report file created by a call to ZZGFTSWK", ctx)?;

    //
    // Close and delete the existing log.
    //
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

    //
    // Open a new log file.
    //
    spicelib::TXTOPN(REPFIL, &mut save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWKUN(save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a search.
    //
    save.XTOTAL = ((MAXWIN as f64) / 2.0);

    fstr::assign(
        &mut save.XBEGMS,
        b"@-------------------Message prefix--------------------@",
    );
    fstr::assign(&mut save.XENDMS, b"@***suffix**@");

    save.XWAIT = 2.0;
    save.XCHECK = 10;

    //
    // Call the progress report display initializer.
    //
    spicelib::ZZGFTSWK(
        save.XTOTAL,
        save.XWAIT,
        save.XCHECK,
        &save.XBEGMS,
        &save.XENDMS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Rewind the file to set the record pointer to the start
    // of the file.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(save.XUNIT),
            ..Default::default()
        };
        save.IOSTAT = io::capture_iostat(|| ctx.rewind(specs))?;
    }
    testutil::CHCKSI(b"REWIND IOSTAT", save.IOSTAT, b"=", 0, 0, OK, ctx)?;

    if (save.IOSTAT == 0) {
        //
        // The first two lines of the file should be blank.
        //
        for I in 1..=2 {
            fstr::assign(&mut save.QNAME, b"Read IOSTAT no. *");
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

            if (save.IOSTAT == 0) {
                fstr::assign(&mut save.QNAME, b"Log file line no. *");
                spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSC(&save.QNAME, &save.LINE, b"=", b" ", OK, ctx)?;
            }
        }
    }

    //
    // If we haven't hit an error, read the third line.
    //
    if (save.IOSTAT == 0) {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
            save.IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut save.LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }
        testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

        if (save.IOSTAT == 0) {
            //
            // Check the report line.
            //
            fstr::assign(&mut save.XLINE, &save.XBEGMS);
            spicelib::SUFFIX(b"  0.00%", 1, &mut save.XLINE);
            spicelib::SUFFIX(&save.XENDMS, 1, &mut save.XLINE);

            testutil::CHCKSC(b"LINE", &save.LINE, b"=", &save.XLINE, OK, ctx)?;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test ZZGFKIN parameter storage.", ctx)?;

    for I in 1..=5 {
        save.XINCR = (I as f64);

        spicelib::ZZGFWKIN(save.XINCR, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZGFWKMO(
            &mut save.UNIT,
            &mut save.TOTAL,
            &mut save.WAIT,
            &mut save.TCHECK,
            &mut save.BEGMSG,
            &mut save.ENDMSG,
            &mut save.INCR,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // We expect the last work increment to be I.
        //

        fstr::assign(&mut save.QNAME, b"INCR I=1");

        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(&save.QNAME, save.INCR, b"~", save.XINCR, VTIGHT, OK, ctx)?;
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test ZZGFWKAD parameter storage.", ctx)?;

    save.XWAIT = 17.0;
    save.XCHECK = 32;
    fstr::assign(
        &mut save.XBEGMS,
        b"$-------Message prefix--------------------------------$",
    );
    fstr::assign(&mut save.XENDMS, b"#***suffix**#");

    spicelib::ZZGFWKAD(save.XWAIT, save.XCHECK, &save.XBEGMS, &save.XENDMS, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Fetch the current report parameters from the ZZGFRPWK monitor.
    //
    spicelib::ZZGFWKMO(
        &mut save.UNIT,
        &mut save.TOTAL,
        &mut save.WAIT,
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
    testutil::CHCKSI(b"UNIT", save.UNIT, b"=", save.XUNIT, 0, OK, ctx)?;
    testutil::CHCKSD(b"TOTAL", save.TOTAL, b"~", save.XTOTAL, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"WAIT", save.WAIT, b"=", save.XWAIT, 0.0, OK, ctx)?;
    testutil::CHCKSI(b"TCHECK", save.TCHECK, b"=", save.XCHECK, 0, OK, ctx)?;
    testutil::CHCKSC(b"BEGMSG", &save.BEGMSG, b"=", &save.XBEGMS, OK, ctx)?;
    testutil::CHCKSC(b"ENDMSG", &save.ENDMSG, b"=", &save.XENDMS, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Test a report with multiple status lines.", ctx)?;

    //
    // Close and delete the existing log.
    //
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

    //
    // Open a new log file.
    //
    spicelib::TXTOPN(REPFIL, &mut save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWKUN(save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a search.
    //
    save.XTOTAL = 100.0;

    fstr::assign(
        &mut save.XBEGMS,
        b"@-----------------------------------Message prefix----@",
    );
    fstr::assign(&mut save.XENDMS, b"@*****suffix@");

    //
    // Check the system time every 10 increment calls.
    //
    save.XCHECK = 10;

    //
    // Set the wait time for display to zero.
    //
    save.XWAIT = 0.0;

    //
    // Call the progress report display initializer.
    //
    spicelib::ZZGFTSWK(
        save.XTOTAL,
        save.XWAIT,
        save.XCHECK,
        &save.XBEGMS,
        &save.XENDMS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Report status 100 times. Each time, the increment is "1".
    //
    for I in 1..=100 {
        spicelib::ZZGFWKIN(1.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Rewind the file to set the record pointer to the start
    // of the file.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(save.XUNIT),
            ..Default::default()
        };
        save.IOSTAT = io::capture_iostat(|| ctx.rewind(specs))?;
    }
    testutil::CHCKSI(b"REWIND IOSTAT", save.IOSTAT, b"=", 0, 0, OK, ctx)?;

    if (save.IOSTAT == 0) {
        //
        // The first two lines of the file should be blank.
        //
        for I in 1..=2 {
            fstr::assign(&mut save.QNAME, b"Read IOSTAT no. *");
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

            if (save.IOSTAT == 0) {
                fstr::assign(&mut save.QNAME, b"Log file line no. *");
                spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSC(&save.QNAME, &save.LINE, b"=", b" ", OK, ctx)?;
            }
        }
    }

    //
    // If we haven't hit an error, read the rest of the file.
    //
    if (save.IOSTAT == 0) {
        for I in 1..=(intrinsics::IDNINT((save.XTOTAL / save.XCHECK as f64)) + 1) {
            fstr::assign(&mut save.QNAME, b"Log file line no. * IOSTAT");
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", (I + 2), &mut save.QNAME, ctx);

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

            if (save.IOSTAT == 0) {
                //
                // Check the report line.
                //
                spicelib::DPFMT(((10 * (I - 1)) as f64), b"XXX.XX", &mut save.PCTSTR, ctx)?;

                fstr::assign(&mut save.XLINE, &save.XBEGMS);
                spicelib::SUFFIX(&save.PCTSTR, 1, &mut save.XLINE);
                spicelib::SUFFIX(b"%", 0, &mut save.XLINE);
                spicelib::SUFFIX(&save.XENDMS, 1, &mut save.XLINE);

                testutil::CHCKSC(b"LINE", &save.LINE, b"=", &save.XLINE, OK, ctx)?;
            }
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Test a report with multiple status lines, this time using a positive time delay.",
        ctx,
    )?;

    //
    // Close and delete the existing log.
    //
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

    //
    // Open a new log file.
    //
    spicelib::TXTOPN(REPFIL, &mut save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZGFWKUN(save.XUNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a search. The search must take long enough
    // so that at least one XWAIT seconds pass per 20% of the
    // work.
    //
    save.XTOTAL = 250000000.0;

    fstr::assign(
        &mut save.XBEGMS,
        b"@-Message prefix--------------------------------------@",
    );
    fstr::assign(&mut save.XENDMS, b"@****Suffix*@");

    //
    // Check the system time once every XTOTAL/20 increment calls.
    //
    save.XCHECK = intrinsics::IDNINT((save.XTOTAL / 20 as f64));

    //
    // Set the wait time for display in seconds. The wait time is a
    // lower bound on the duration between progress report display
    // updates. Setting the wait to a small value won't cause more
    // updates to be displayed; in no event will there be more updates
    // than allowed by XCHECK. Setting the wait to a large value will
    // cause the test case to fail because some progress updates won't
    // get displayed.
    //
    save.XWAIT = 0.1;

    //
    // Call the progress report display initializer.
    //
    spicelib::ZZGFTSWK(
        save.XTOTAL,
        save.XWAIT,
        save.XCHECK,
        &save.XBEGMS,
        &save.XENDMS,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the progress increment to 5.
    //
    save.INCR = 5 as f64;

    //
    // Report status XTOTAL/INCR times. So every time the
    // system clock is checked, we expect that
    //
    //      FRAC = XCHECK * INCR / XTOTAL
    //
    //
    //           = 25%
    //
    // progress should have been made since the last check. We expect
    // 25% of the computation to take longer than XWAIT seconds, so we
    // should see a report line for every 25% progress.
    //
    save.FRAC = (((save.XCHECK as f64) * save.INCR) / save.XTOTAL);

    save.NRPORT = intrinsics::IDNINT((save.XTOTAL / save.INCR));

    for I in 1..=save.NRPORT {
        spicelib::ZZGFWKIN(save.INCR, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Rewind the file to set the record pointer to the start
    // of the file.
    //
    {
        use f2rust_std::io;

        let specs = io::PosSpecs {
            unit: Some(save.XUNIT),
            ..Default::default()
        };
        save.IOSTAT = io::capture_iostat(|| ctx.rewind(specs))?;
    }
    testutil::CHCKSI(b"REWIND IOSTAT", save.IOSTAT, b"=", 0, 0, OK, ctx)?;

    if (save.IOSTAT == 0) {
        //
        // The first two lines of the file should be blank.
        //
        for I in 1..=2 {
            fstr::assign(&mut save.QNAME, b"Read IOSTAT no. *");
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

            if (save.IOSTAT == 0) {
                fstr::assign(&mut save.QNAME, b"Log file line no. *");
                spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSC(&save.QNAME, &save.LINE, b"=", b" ", OK, ctx)?;
            }
        }
    }

    //
    // If we haven't hit an error, read the rest of the file.
    //
    // The expected number of status lines in the report is the
    // number of update calls divided by the call check ratio,
    // plus one, since the first line shows 0% progress.
    //
    save.XNL = (1 + (save.NRPORT / save.XCHECK));

    if (save.IOSTAT == 0) {
        for I in 1..=save.XNL {
            fstr::assign(&mut save.QNAME, b"Log file line no. * IOSTAT");
            spicelib::REPMI(&save.QNAME.to_vec(), b"*", (I + 2), &mut save.QNAME, ctx);

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(save.XUNIT)?, None, b"(A)")?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.LINE)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
            testutil::CHCKSI(&save.QNAME, save.IOSTAT, b"=", 0, 0, OK, ctx)?;

            if (save.IOSTAT == 0) {
                //
                // Check the report line.
                //
                spicelib::DPFMT(
                    ((((I - 1) * 100) as f64) * save.FRAC),
                    b"XXX.XX",
                    &mut save.PCTSTR,
                    ctx,
                )?;

                fstr::assign(&mut save.XLINE, &save.XBEGMS);
                spicelib::SUFFIX(&save.PCTSTR, 1, &mut save.XLINE);
                spicelib::SUFFIX(b"%", 0, &mut save.XLINE);
                spicelib::SUFFIX(&save.XENDMS, 1, &mut save.XLINE);

                testutil::CHCKSC(b"LINE", &save.LINE, b"=", &save.XLINE, OK, ctx)?;
            }
        }
    }

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

    spicelib::ZZGFRPWK(1, 2.0, 3.0, 7, b" ", b" ", 0.5, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
