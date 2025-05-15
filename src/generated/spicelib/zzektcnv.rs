//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LNSIZE: i32 = 80;
const MSGLEN: i32 = 160;
const NCOMP: i32 = 10;
const MAXMOD: i32 = 10;
const SHORT: i32 = 32;

//$Procedure  ZZEKTCNV ( Private: EK, time conversion )
pub fn ZZEKTCNV(
    TIMSTR: &[u8],
    ET: &mut f64,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LOCSTR = [b' '; LNSIZE as usize];
    let mut MODIFY = ActualCharArray::new(SHORT, 1..=MAXMOD);
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut SCLMSG = [b' '; MSGLEN as usize];
    let mut TYPE = [b' '; SHORT as usize];
    let mut SCLKDP: f64 = 0.0;
    let mut TVEC = StackArray::<f64, 10>::new(1..=NCOMP);
    let mut CLKID: i32 = 0;
    let mut LOC: i32 = 0;
    let mut NTVEC: i32 = 0;
    let mut FND: bool = false;
    let mut MODS: bool = false;
    let mut SUCCES: bool = false;
    let mut YABBRV: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEKTCNV", ctx)?;

    //
    // No error to start with.
    //
    *ERROR = false;
    fstr::assign(ERRMSG, b" ");

    //
    // Get a left-justified, compressed, upper-case copy of
    // the string, so we can easily search it for substrings
    // that would identify it as SCLK.  If we do find a
    // match, remove the identifying substring (of the form
    // 'MO SCLK', 'VGR1 SCLK', etc.).
    //
    CMPRSS(b" ", 1, TIMSTR, &mut LOCSTR);
    LJUST(&LOCSTR.clone(), &mut LOCSTR);
    UCASE(&LOCSTR.clone(), &mut LOCSTR, ctx);

    LOC = POSR(&LOCSTR, b"SCLK", RTRIM(&LOCSTR));

    if (LOC > 0) {
        //
        // It's a SCLK string.  Find the ID code, if we can.
        //
        SCN2ID(
            fstr::substr(&LOCSTR, 1..=(LOC + 3)),
            &mut CLKID,
            &mut FND,
            ctx,
        )?;

        if !FND {
            //
            // We don't recognize this SCLK type.
            //
            *ERROR = true;

            if (LOC > 1) {
                fstr::assign(
                    ERRMSG,
                    b"Time conversion failed; SCLK type <#> was not recognized.",
                );

                REPMC(
                    &ERRMSG.to_vec(),
                    b"#",
                    fstr::substr(TIMSTR, 1..=(LOC - 1)),
                    ERRMSG,
                );
            } else {
                fstr::assign(
                    ERRMSG,
                    b"Time conversion failed; SCLK name was not supplied.",
                );
            }

            CHKOUT(b"ZZEKTCNV", ctx)?;
            return Ok(());
        }

        //
        // If we got this far, we recognized the SCLK type.
        // Convert the time to ET.
        //
        SCPARS(
            CLKID,
            fstr::substr(&LOCSTR, (LOC + 4)..),
            ERROR,
            &mut SCLMSG,
            &mut SCLKDP,
            ctx,
        )?;

        if FAILED(ctx) {
            //
            // We'll arrive here if the required SCLK kernel hasn't
            // been loaded.
            //
            *ERROR = true;
            fstr::assign(
                ERRMSG,
                b"Unexpected SPICELIB error encountered while attempting to parse the string <",
            );
            SUFFIX(TIMSTR, 0, ERRMSG);
            SUFFIX(b">", 0, ERRMSG);
            CHKOUT(b"ZZEKTCNV", ctx)?;
            return Ok(());
        } else if *ERROR {
            fstr::assign(
                ERRMSG,
                b"The string <#> didn\'t parse as a spacecraft clock string.",
            );
            REPMC(&ERRMSG.to_vec(), b"#", TIMSTR, ERRMSG);
            SUFFIX(&SCLMSG, 3, ERRMSG);
            CHKOUT(b"ZZEKTCNV", ctx)?;
            return Ok(());
        } else {
            SCT2E(CLKID, SCLKDP, ET, ctx)?;

            if FAILED(ctx) {
                *ERROR = true;
                fstr::assign(
                    ERRMSG,
                    b"Unexpected SPICELIB error encountered while attempting to parse the string <",
                );
                SUFFIX(TIMSTR, 0, ERRMSG);
                SUFFIX(b">", 0, ERRMSG);
                CHKOUT(b"ZZEKTCNV", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // We could have a standard time string.  Make sure that the
        // time string is acceptable before actually calling STR2ET.
        //
        TPARTV(
            &LOCSTR,
            TVEC.as_slice_mut(),
            &mut NTVEC,
            &mut TYPE,
            MODIFY.as_arg_mut(),
            &mut MODS,
            &mut YABBRV,
            &mut SUCCES,
            &mut PICTUR,
            ERRMSG,
            ctx,
        );

        if SUCCES {
            //
            // It's safe to pass the string to STR2ET.
            //
            STR2ET(&LOCSTR, ET, ctx)?;

            if FAILED(ctx) {
                *ERROR = true;
                fstr::assign(
                    ERRMSG,
                    b"Unexpected SPICELIB error encountered while attempting to parse the string <",
                );
                SUFFIX(TIMSTR, 0, ERRMSG);
                SUFFIX(b">", 0, ERRMSG);
                CHKOUT(b"ZZEKTCNV", ctx)?;
                return Ok(());
            }
        } else {
            //
            // The string cannot be parsed by STR2ET.  The error message
            // was set by TPARTV.
            //
            *ERROR = true;
            CHKOUT(b"ZZEKTCNV", ctx)?;
            return Ok(());
        }
        //
        // We're done with the standard time string case.
        //
    }
    //
    // We've parsed a time string, if it was an SCLK or standard string.
    //
    CHKOUT(b"ZZEKTCNV", ctx)?;
    Ok(())
}
