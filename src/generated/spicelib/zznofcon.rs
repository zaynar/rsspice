//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 35;
const PHRSLN: i32 = 600;

//$Procedure  ZZNOFCON ( Create frame connection long error message )
pub fn ZZNOFCON(
    ET: f64,
    FRAME1: i32,
    ENDP1: i32,
    FRAME2: i32,
    ENDP2: i32,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BNAME = ActualCharArray::new(FRNMLN, 1..=2);
    let mut NAME = ActualCharArray::new(FRNMLN, 1..=2);
    let mut PHRASE = [b' '; PHRSLN as usize];
    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut ENDPS = StackArray::<i32, 2>::new(1..=2);
    let mut CENTER: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRAMES = StackArray::<i32, 2>::new(1..=2);
    let mut SCLKID: i32 = 0;
    let mut CKMISS: bool = false;
    let mut FOUND: bool = false;
    let mut HAVNAM = StackArray::<bool, 2>::new(1..=2);
    let mut SCMISS: bool = false;

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
    // Because this routine might cause a SPICE error to be
    // signaled, we have to check in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZNOFCON", ctx)?;

    //
    // Capture input IDs in arrays.
    //
    FRAMES[1] = FRAME1;
    FRAMES[2] = FRAME2;
    ENDPS[1] = ENDP1;
    ENDPS[2] = ENDP2;

    //
    // The flags CKMISS and SCMISS are used, respectively, to
    // record whether any CK lookup failed due to missing CK
    // data or missing SCLK data. Each of these flags is turned
    // on if at least one lookup failed due to the indicated
    // cause.
    //
    CKMISS = false;
    SCMISS = false;

    //
    // Get a string representation of the transformation epoch.
    //
    ETCAL(ET, &mut TIMSTR, ctx);

    //
    // Get the names of the participating frames, if available.
    //
    FRMNAM(FRAMES[1], &mut NAME[1], ctx)?;
    FRMNAM(FRAMES[2], &mut NAME[2], ctx)?;
    FRMNAM(ENDPS[1], &mut BNAME[1], ctx)?;
    FRMNAM(ENDPS[2], &mut BNAME[2], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZNOFCON", ctx)?;
        return Ok(());
    }

    for I in 1..=2 {
        if fstr::eq(NAME.get(I), b" ") {
            fstr::assign(NAME.get_mut(I), b"Name not available");
            HAVNAM[I] = false;
        } else {
            HAVNAM[I] = true;
        }

        if fstr::eq(BNAME.get(I), b" ") {
            fstr::assign(BNAME.get_mut(I), b"Name not available");
        }
    }

    fstr::assign(ERRMSG, b"At epoch # TDB (# TDB), there is insufficient information available to transform from reference frame # (@) to reference frame # (@).");

    REPMF(&ERRMSG.to_vec(), b"#", ET, 14, b"E", ERRMSG, ctx);
    REPMC(&ERRMSG.to_vec(), b"#", &TIMSTR, ERRMSG);

    for I in 1..=2 {
        REPMI(&ERRMSG.to_vec(), b"#", FRAMES[I], ERRMSG, ctx);
        REPMC(&ERRMSG.to_vec(), b"@", &NAME[I], ERRMSG);
    }

    //
    // For any frame graph longer than a single point, tell the user
    // the endpoint of the frame connection graph originating
    // at that frame.
    //
    for I in 1..=2 {
        if (FRAMES[I] != ENDPS[I]) {
            fstr::assign(&mut PHRASE, b"Frame # could be transformed to frame # (@).");

            if HAVNAM[I] {
                REPMC(&PHRASE.clone(), b"#", &NAME[I], &mut PHRASE);
            } else {
                REPMI(&PHRASE.clone(), b"#", FRAMES[I], &mut PHRASE, ctx);
            }

            REPMI(&PHRASE.clone(), b"#", ENDPS[I], &mut PHRASE, ctx);
            REPMC(&PHRASE.clone(), b"@", &BNAME[I], &mut PHRASE);

            SUFFIX(&PHRASE, 1, ERRMSG);

            //
            // The error messages below are appended only if they're not
            // redundant.
            //
            if ((I == 1) || (ENDPS[2] != ENDPS[1])) {
                //
                // For each endpoint frame, if that frame is of CK type,
                // indicate the instrument ID for which CK data are needed.
                //
                FRINFO(
                    ENDPS[I],
                    &mut CENTER,
                    &mut CLASS,
                    &mut CLSSID,
                    &mut FOUND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZNOFCON", ctx)?;
                    return Ok(());
                }

                if FOUND {
                    if (CLASS == CK) {
                        fstr::assign(&mut PHRASE, b"The latter is a CK frame; a CK file containing data for instrument or structure # at the epoch shown above, as well as a corresponding SCLK kernel, must be loaded in order to use this frame.");

                        REPMI(&PHRASE.clone(), b"#", CLSSID, &mut PHRASE, ctx);

                        SUFFIX(&PHRASE, 1, ERRMSG);

                        //
                        // Find out whether we have SCLK data for this
                        // CK ID.
                        //
                        CKMETA(CLSSID, b"SCLK", &mut SCLKID, ctx)?;

                        if !ZZSCLK(CLSSID, SCLKID, ctx)? {
                            SCMISS = true;

                            fstr::assign(&mut PHRASE, b"No SCLK kernel for instrument or structure #, with corresponding SCLK ID #, is currently loaded.");

                            REPMI(&PHRASE.clone(), b"#", CLSSID, &mut PHRASE, ctx);
                            REPMI(&PHRASE.clone(), b"#", SCLKID, &mut PHRASE, ctx);

                            SUFFIX(&PHRASE, 1, ERRMSG);
                        } else {
                            //
                            // If we got here and have the SCLK data, then
                            // we don't have CK data.
                            //
                            CKMISS = true;
                        }
                    }
                    //
                    // End of CK frame case.
                    //
                }
                //
                // End of "info found" case.
                //
            }
        //
        // End of distinct frame case.
        //
        } else if ((I == 1) || (ENDPS[2] != ENDPS[1])) {
            //
            // The error messages below are appended only if they're not
            // redundant.
            //
            // This graph has length one. If the frame comprising
            // this graph is a CK frame, generate a phrase
            // indicating the needed CK data.
            //
            FRINFO(
                FRAMES[I],
                &mut CENTER,
                &mut CLASS,
                &mut CLSSID,
                &mut FOUND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZNOFCON", ctx)?;
                return Ok(());
            }

            if FOUND {
                if (CLASS == CK) {
                    fstr::assign(&mut PHRASE, b"# is a CK frame; a CK file containing data for instrument or structure # at the epoch shown above, as well as a corresponding SCLK kernel, must be loaded in order to use this frame.");

                    if HAVNAM[I] {
                        REPMC(&PHRASE.clone(), b"#", &NAME[I], &mut PHRASE);
                    } else {
                        REPMI(&PHRASE.clone(), b"#", FRAMES[I], &mut PHRASE, ctx);
                    }

                    REPMI(&PHRASE.clone(), b"#", CLSSID, &mut PHRASE, ctx);

                    SUFFIX(&PHRASE, 1, ERRMSG);

                    //
                    // Find out whether we have SCLK data for this
                    // CK ID.
                    //
                    CKMETA(CLSSID, b"SCLK", &mut SCLKID, ctx)?;

                    if !ZZSCLK(CLSSID, SCLKID, ctx)? {
                        SCMISS = true;

                        fstr::assign(&mut PHRASE, b"No SCLK kernel for instrument or structure #, with corresponding SCLK ID #, is currently loaded.");

                        REPMI(&PHRASE.clone(), b"#", CLSSID, &mut PHRASE, ctx);
                        REPMI(&PHRASE.clone(), b"#", SCLKID, &mut PHRASE, ctx);

                        SUFFIX(&PHRASE, 1, ERRMSG);
                    } else {
                        //
                        // If we got here and have the SCLK data, then
                        // we don't have CK data.
                        //
                        CKMISS = true;
                    }
                }
                //
                // End of CK frame case.
                //
            }
            //
            // End of "info found" case.
            //
        }
        //
        // End of path length case.
        //
    }
    //
    // End of path loop.
    //

    if CKMISS {
        //
        // At least one lookup failed due to missing CK data.
        //
        // The informational message we include depends on whether we
        // also lack SCLK data.
        //
        if SCMISS {
            //
            // We lack SCLK data for one frame and CK data for another.
            //
            fstr::assign(&mut PHRASE, b"For a CK frame for which the corresponding SCLK kernel has been loaded, failure to find required CK data could be due to one or more CK files not having been loaded, or to the epoch shown above lying within a coverage gap or beyond the coverage bounds of the loaded CK files. It is also possible that no loaded CK file has required angular velocity data for the input epoch, even if a loaded CK does have attitude data for that epoch. You can use CKBRIEF with the -dump option to display coverage intervals of a CK file.");
        } else {
            //
            // We have SCLK data but lack CK data.
            //
            fstr::assign(&mut PHRASE, b"Failure to find required CK data could be due to one or more CK files not having been loaded, or to the epoch shown above lying within a coverage gap or beyond the coverage bounds of the loaded CK files. It is also possible that no loaded CK file has required angular velocity data for the input epoch, even if a loaded CK does have attitude data for that epoch. You can use CKBRIEF with the -dump option to display coverage intervals of a CK file.");
        }

        SUFFIX(&PHRASE, 1, ERRMSG);
    }

    CHKOUT(b"ZZNOFCON", ctx)?;
    Ok(())
}
