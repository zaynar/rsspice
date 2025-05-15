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

struct SaveVars {
    FINISH: Vec<u8>,
    START: Vec<u8>,
    DONE: f64,
    ENTIRE: f64,
    SVINCR: f64,
    LSTSEC: f64,
    STEP: f64,
    CALLS: i32,
    CHECK: i32,
    LS: i32,
    STDOUT: i32,
    SVUNIT: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FINISH = vec![b' '; MXENDM as usize];
        let mut START = vec![b' '; MXBEGM as usize];
        let mut DONE: f64 = 0.0;
        let mut ENTIRE: f64 = 0.0;
        let mut SVINCR: f64 = 0.0;
        let mut LSTSEC: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut CALLS: i32 = 0;
        let mut CHECK: i32 = 0;
        let mut LS: i32 = 0;
        let mut STDOUT: i32 = 0;
        let mut SVUNIT: i32 = 0;
        let mut FIRST: bool = false;

        CALLS = 0;
        CHECK = 1;
        DONE = 0.0;
        ENTIRE = 0.0;
        fstr::assign(&mut FINISH, b" ");
        FIRST = true;
        LS = 1;
        LSTSEC = 0.0;
        fstr::assign(&mut START, b" ");
        STDOUT = 6;
        STEP = 0.0;
        SVINCR = 0.0;
        SVUNIT = 6;

        Self {
            FINISH,
            START,
            DONE,
            ENTIRE,
            SVINCR,
            LSTSEC,
            STEP,
            CALLS,
            CHECK,
            LS,
            STDOUT,
            SVUNIT,
            FIRST,
        }
    }
}

//$Procedure    ZZGFRPWK ( Geometry finder report work done on a task )
pub fn ZZGFRPWK(
    UNIT: i32,
    TOTAL: f64,
    WAIT: f64,
    TCHECK: i32,
    BEGIN: &[u8],
    END: &[u8],
    INCR: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
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

    CHKIN(b"ZZGFRPWK", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZGFRPWK", ctx)?;
    Ok(())
}

//$Procedure ZZGFTSWK ( Geometry finder total sum of work to be done. )
pub fn ZZGFTSWK(
    TOTAL: f64,
    WAIT: f64,
    TCHECK: i32,
    BEGIN: &[u8],
    END: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut MESSGE = [b' '; MXMSG as usize];
    let mut TVEC = StackArray::<f64, 6>::new(1..=6);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFTSWK", ctx)?;

    //
    // On the first pass, obtain the logical unit for
    // standard output.
    //
    if save.FIRST {
        STDIO(b"STDOUT", &mut save.STDOUT, ctx)?;

        //
        // The output unit is STDOUT unless the caller
        // sets it to something else.
        //
        save.SVUNIT = save.STDOUT;

        save.FIRST = false;
    }

    //
    // Save the inputs and set the amount of work done to 0
    //
    save.ENTIRE = TOTAL;
    save.STEP = intrinsics::DMIN1(&[3600.0, intrinsics::DMAX1(&[0.0, WAIT])]);
    save.CHECK = intrinsics::MAX0(&[1, TCHECK]);

    fstr::assign(&mut save.START, BEGIN);
    fstr::assign(&mut save.FINISH, END);

    save.DONE = 0.0;

    //
    // Set the timer.
    //
    ZZCPUTIM(TVEC.as_slice_mut(), ctx)?;

    save.LSTSEC = (((TVEC[4] * 3600.0) + (TVEC[5] * 60.0)) + TVEC[6]);

    //
    // Set the increment counter
    //
    save.CALLS = 0;

    //
    // Compose the output message.
    //
    save.LS = RTRIM(&save.START);
    fstr::assign(
        &mut MESSGE,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(fstr::substr(&save.START, 1..=save.LS), b" "),
                    b"  0.00%",
                ),
                b" ",
            ),
            &save.FINISH,
        ),
    );

    //
    // Display a blank line, make sure we don't overwrite anything
    // at the bottom of the screen. The display the message.
    //
    if (save.SVUNIT == save.STDOUT) {
        ZZGFDSPS(1, &MESSGE, b"A", 0, ctx)?;
    } else {
        //
        // Write the message without special carriage control.
        //
        WRITLN(b" ", save.SVUNIT, ctx)?;
        WRITLN(b" ", save.SVUNIT, ctx)?;
        WRITLN(&MESSGE, save.SVUNIT, ctx)?;
    }

    CHKOUT(b"ZZGFTSWK", ctx)?;
    Ok(())
}

//$Procedure ZZGFWKIN ( Geometry finder work finished increment )
pub fn ZZGFWKIN(INCR: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PRCENT = [b' '; 10];
    let mut MESSGE = [b' '; MXMSG as usize];
    let mut CURSEC: f64 = 0.0;
    let mut FRACTN: f64 = 0.0;
    let mut TVEC = StackArray::<f64, 6>::new(1..=6);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFWKIN", ctx)?;

    save.SVINCR = INCR;
    save.DONE = (save.DONE + INCR);
    save.CALLS = (save.CALLS + 1);

    if (save.ENTIRE == 0 as f64) {
        CHKOUT(b"ZZGFWKIN", ctx)?;
        return Ok(());
    }

    if (save.CALLS >= save.CHECK) {
        save.CALLS = 0;

        ZZCPUTIM(TVEC.as_slice_mut(), ctx)?;

        CURSEC = (((TVEC[4] * 3600.0) + (TVEC[5] * 60.0)) + TVEC[6]);

        if (f64::abs((CURSEC - save.LSTSEC)) >= save.STEP) {
            save.LSTSEC = CURSEC;
            //
            // Report how much work has been done.
            //
            FRACTN = BRCKTD(((save.DONE / save.ENTIRE) * 100.0), 0.0, 100.0);

            DPFMT(FRACTN, b"xxx.xx", &mut PRCENT, ctx)?;

            fstr::assign(fstr::substr_mut(&mut PRCENT, 7..=7), b"%");

            fstr::assign(
                &mut MESSGE,
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(fstr::substr(&save.START, 1..=save.LS), b" "),
                            fstr::substr(&PRCENT, 1..=7),
                        ),
                        b" ",
                    ),
                    fstr::substr(&save.FINISH, 1..=RTRIM(&save.FINISH)),
                ),
            );

            if (save.SVUNIT == save.STDOUT) {
                ZZGFDSPS(0, &MESSGE, b"A", 0, ctx)?;
            } else {
                //
                // Write the message without special carriage control.
                //
                WRITLN(&MESSGE, save.SVUNIT, ctx)?;
            }
        }
    }

    CHKOUT(b"ZZGFWKIN", ctx)?;
    Ok(())
}

//$Procedure ZZGFWKAD ( Geometry finder work reporting adjustment )
pub fn ZZGFWKAD(WAIT: f64, TCHECK: i32, BEGIN: &[u8], END: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.STEP = intrinsics::DMIN1(&[3600.0, intrinsics::DMAX1(&[0.0, WAIT])]);
    save.CHECK = intrinsics::MAX0(&[1, TCHECK]);
    fstr::assign(&mut save.START, BEGIN);
    fstr::assign(&mut save.FINISH, END);
}

//$Procedure ZZGFWUN ( Geometry finder set work report output unit )
pub fn ZZGFWKUN(UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // On the first pass, obtain the logical unit for
    // standard output.
    //
    if save.FIRST {
        STDIO(b"STDOUT", &mut save.STDOUT, ctx)?;

        save.FIRST = false;
    }

    save.SVUNIT = UNIT;

    Ok(())
}

//$Procedure ZZGFWKMO ( Geometry finder work reporting monitor )
pub fn ZZGFWKMO(
    UNIT: &mut i32,
    TOTAL: &mut f64,
    WAIT: &mut f64,
    TCHECK: &mut i32,
    BEGIN: &mut [u8],
    END: &mut [u8],
    INCR: &mut f64,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *UNIT = save.SVUNIT;
    *TOTAL = save.ENTIRE;
    *WAIT = save.STEP;
    *TCHECK = save.CHECK;
    fstr::assign(BEGIN, &save.START);
    fstr::assign(END, &save.FINISH);
    *INCR = save.SVINCR;
}
