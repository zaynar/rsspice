//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const COMMA: &[u8] = b",";
const DQUOTE: &[u8] = b"\"";
const EQUALS: &[u8] = b"=";
const SLASH: &[u8] = b"/";
const NMAX: i32 = 100;
const METLEN: i32 = 1000;

//$Procedure ZZPRSMET ( Private: parse method string )
pub fn ZZPRSMET(
    BODYID: i32,
    METHOD: &[u8],
    MXNSRF: i32,
    SHAPE: &mut [u8],
    SUBTYP: &mut [u8],
    PRI: &mut bool,
    NSURF: &mut i32,
    SRFLST: &mut [i32],
    PNTDEF: &mut [u8],
    TRMTYP: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SRFLST = DummyArrayMut::new(SRFLST, 1..);
    let mut CHR = [b' '; 1 as usize];
    let mut LOCMTH = [b' '; METLEN as usize];
    let mut BEGS = StackArray::<i32, 102>::new(1..=(NMAX + 2));
    let mut CODE: i32 = 0;
    let mut ENDS = StackArray::<i32, 102>::new(1..=(NMAX + 2));
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut NPRI: i32 = 0;
    let mut NSHAPE: i32 = 0;
    let mut NLMBTP: i32 = 0;
    let mut NSUBTP: i32 = 0;
    let mut NSRFLS: i32 = 0;
    let mut NTRMTP: i32 = 0;
    let mut DONE: bool = false;
    let mut FND: bool = false;

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

    CHKIN(b"ZZPRSMET", ctx)?;

    //
    // No shape or surfaces have been specified yet.
    //
    fstr::assign(SHAPE, b" ");
    fstr::assign(SUBTYP, b" ");
    fstr::assign(PNTDEF, b" ");
    fstr::assign(TRMTYP, b" ");
    *PRI = true;
    *NSURF = 0;

    //
    // Initialize clause counts.
    //
    NPRI = 0;
    NSHAPE = 0;
    NSUBTP = 0;
    NLMBTP = 0;
    NTRMTP = 0;
    NSRFLS = 0;

    fstr::assign(fstr::substr_mut(&mut LOCMTH, 1..=1), SLASH);
    BEGS[1] = 1;
    ENDS[1] = 1;

    LJUCRS(1, METHOD, fstr::substr_mut(&mut LOCMTH, 2..), ctx);

    //
    // Tokenize the input method string.
    //
    ZZLEXMET(
        fstr::substr(&LOCMTH, 2..),
        NMAX,
        &mut N,
        BEGS.subarray_mut(2),
        ENDS.subarray_mut(2),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZPRSMET", ctx)?;
        return Ok(());
    }

    N = (N + 1);

    {
        let m1__: i32 = 2;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            BEGS[I] = (BEGS[I] + 1);
            ENDS[I] = (ENDS[I] + 1);
            I += m3__;
        }
    }

    //
    // Identify the target shape.
    //
    DONE = false;
    I = 1;

    while !DONE {
        if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"ELLIPSOID") {
            NSHAPE = (NSHAPE + 1);
            fstr::assign(SHAPE, b"ELLIPSOID");
        } else if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"DSK") {
            NSHAPE = (NSHAPE + 1);
            fstr::assign(SHAPE, b"DSK");
        }

        if (I == N) {
            DONE = true;
        } else {
            I = (I + 1);
        }
    }

    if (NSHAPE != 1) {
        SETMSG(b"The \"method\" or \"shape\" string must contain exactly one instance of a shape keyword: ELLIPSOID or DSK, or additionally, in the case of occultation computations, POINT. The number of shape keywords was #. The input string was <#>.", ctx);
        ERRINT(b"#", NSHAPE, ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
        CHKOUT(b"ZZPRSMET", ctx)?;
        return Ok(());
    }

    if fstr::eq(SHAPE, b"DSK") {
        //
        // Traverse the tokens; identify clauses in the method string.
        //
        I = 1;

        while (I < N) {
            fstr::assign(&mut CHR, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));

            if fstr::eq(&CHR, SLASH) {
                //
                // If the method string is correct, the next token should
                // be a keyword.
                //
                I = (I + 1);

                if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"SURFACES") {
                    //
                    // Normally, we're looking at the start of a surface
                    // list at this point. We need at least an assignment
                    // operator ('=') and a surface name or ID code following
                    // the SURFACES keyword.
                    //
                    NSRFLS = (NSRFLS + 1);

                    if (NSRFLS > 1) {
                        //
                        // We have too many surface specifications.
                        //
                        SETMSG(
                            b"Extra SURFACES keyword was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    if (I > (N - 2)) {
                        SETMSG(b"The surface list in the method string appears to be truncated. The method string is <#>.", ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    I = (I + 1);

                    fstr::assign(&mut CHR, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));

                    if fstr::ne(&CHR, EQUALS) {
                        SETMSG(b"The surface list in the method string lacks an \"=\" sign after the SURFACES keyword. The method string is <#>.", ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    //
                    // Prepare to read a list of names, quoted
                    // strings, or ID codes. The list must be non-empty. If
                    // there are multiple list entries, they're delimited by
                    // commas.
                    //
                    I = (I + 1);
                    DONE = false;

                    while !DONE {
                        fstr::assign(&mut CHR, fstr::substr(&LOCMTH, BEGS[I]..=BEGS[I]));

                        if fstr::eq(&CHR, DQUOTE) {
                            //
                            // We expect the current token to be a quoted
                            // string.
                            //
                            if (ENDS[I] > (BEGS[I] + 1)) {
                                BEGS[I] = (BEGS[I] + 1);
                                ENDS[I] = (ENDS[I] - 1);
                            } else {
                                //
                                // We have an invalid quoted string.
                                //
                                SETMSG(b"The surface list in the method string contains a double quote character that is not a delimiter of a valid quoted string. The method string is <#>.", ctx);
                                ERRCH(b"#", METHOD, ctx);
                                SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                                CHKOUT(b"ZZPRSMET", ctx)?;
                                return Ok(());
                            }
                        }

                        // We have either a name or an integer in string form.
                        // Convert the token to a surface ID code.
                        //
                        SRFSCC(
                            fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]),
                            BODYID,
                            &mut CODE,
                            &mut FND,
                            ctx,
                        )?;

                        if FAILED(ctx) {
                            CHKOUT(b"ZZPRSMET", ctx)?;
                            return Ok(());
                        }

                        if !FND {
                            SETMSG(b"The surface name <#> could not be translated to an ID code. The method string is <#>.", ctx);
                            ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                            ERRCH(b"#", METHOD, ctx);
                            SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
                            CHKOUT(b"ZZPRSMET", ctx)?;
                            return Ok(());
                        }

                        *NSURF = (*NSURF + 1);
                        //
                        // Make sure there's room in the surface ID array.
                        //
                        if (*NSURF > MXNSRF) {
                            SETMSG(b"The surface name <#> could not be stored in the surface list due to lack of room. The maximum number of surfaces that can be specified is #. The method string is <#>.", ctx);
                            ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                            ERRINT(b"#", MXNSRF, ctx);
                            ERRCH(b"#", METHOD, ctx);
                            SIGERR(b"SPICE(TOOMANYSURFACES)", ctx)?;
                            CHKOUT(b"ZZPRSMET", ctx)?;
                            return Ok(());
                        }

                        //
                        // Append the surface ID to the surface ID array.
                        //
                        SRFLST[*NSURF] = CODE;

                        if (I == N) {
                            //
                            // We're at the end of the method string.
                            //
                            DONE = true;
                        } else {
                            //
                            // There are more tokens; the surface list may
                            // contain more surface names or ID codes.
                            //
                            I = (I + 1);
                            fstr::assign(&mut CHR, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));

                            if fstr::eq(&CHR, SLASH) {
                                //
                                // We're at the end of the surface list.
                                //
                                DONE = true;
                                //
                                // Decrement the token pointer so that the
                                // slash will be seen as the next token.
                                //
                                I = (I - 1);
                            } else if fstr::eq(&CHR, COMMA) {
                                //
                                // We expect to find another surface name or
                                // ID in the list.
                                //
                                if (I < N) {
                                    I = (I + 1);
                                } else {
                                    //
                                    // We have a syntax error in the surface
                                    // list.
                                    //
                                    SETMSG(b"The surface list in the method string contains a comma that is not followed by a surface name or ID code. The method string is <#>.", ctx);
                                    ERRCH(b"#", METHOD, ctx);
                                    SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                                    CHKOUT(b"ZZPRSMET", ctx)?;
                                    return Ok(());
                                }
                            } else {
                                //
                                // We have a syntax error in the surface list.
                                //
                                SETMSG(b"The surface list in the method string is followed by the unexpected token <#>. The method string is <#>.", ctx);
                                ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                                ERRCH(b"#", METHOD, ctx);
                                SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                                CHKOUT(b"ZZPRSMET", ctx)?;
                                return Ok(());
                            }
                        }
                    }
                } else if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"UNPRIORITIZED") {
                    NPRI = (NPRI + 1);

                    if (NPRI > 1) {
                        //
                        // We have too many prioritization specifications.
                        //
                        SETMSG(
                            b"Extra prioritization keyword was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    *PRI = false;
                } else if (fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"INTERCEPT")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"NADIR"))
                {
                    //
                    // This is a sub-point type specification.
                    //
                    NSUBTP = (NSUBTP + 1);

                    if (NSUBTP > 1) {
                        //
                        // We have too many sub-point specifications.
                        //
                        SETMSG(
                            b"Extra sub-point type <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    fstr::assign(SUBTYP, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                } else if (fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"TANGENT")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"GUIDED"))
                {
                    //
                    // This is a point definition specification.
                    //
                    NLMBTP = (NLMBTP + 1);

                    if (NLMBTP > 1) {
                        //
                        // We have too many point definition specifications.
                        //
                        SETMSG(
                            b"Extra point definition <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    fstr::assign(PNTDEF, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                } else if (fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"UMBRAL")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"PENUMBRAL"))
                {
                    //
                    // This is a terminator type specification.
                    //
                    NTRMTP = (NTRMTP + 1);

                    if (NTRMTP > 1) {
                        //
                        // We have too many terminator type specifications.
                        //
                        SETMSG(
                            b"Extra terminator type <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    fstr::assign(TRMTYP, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                } else if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), SLASH) {
                    //
                    // Adjust the token index in the message to account
                    // for the insertion of a slash at the beginning.
                    //
                    SETMSG(b"An unexpected slash character was found at index # in the method string <#>.", ctx);
                    ERRINT(b"#", (BEGS[I] - 1), ctx);
                    ERRCH(b"#", METHOD, ctx);
                    SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                    CHKOUT(b"ZZPRSMET", ctx)?;
                    return Ok(());
                } else if fstr::ne(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"DSK") {
                    //
                    // 'DSK' was the only valid token that could appear
                    // at this point.
                    //
                    SETMSG(
                        b"Unexpected token <#> was found in the method string <#>.",
                        ctx,
                    );
                    ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                    ERRCH(b"#", METHOD, ctx);
                    SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                    CHKOUT(b"ZZPRSMET", ctx)?;
                    return Ok(());
                }
            }

            I = (I + 1);
        }

        //
        // Currently, the UNPRIORITIZED keyword must be provided
        // for DSK shapes.
        //
        if *PRI {
            SETMSG(b"The keyword UNPRIORITIZED must be present in the method string <#> when the target shape is DSK. Prioritized DSK segment searches are not currently supported.", ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(BADPRIORITYSPEC)", ctx)?;
            CHKOUT(b"ZZPRSMET", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(SHAPE, b"ELLIPSOID") {
        //
        // Check for legacy string inputs.
        //
        if EQSTR(METHOD, b"ELLIPSOID") {
            //
            // There are no other outputs to set.
            //
            CHKOUT(b"ZZPRSMET", ctx)?;
            return Ok(());
        } else if (((EQSTR(METHOD, b"NEARPOINT:ELLIPSOID")
            || EQSTR(METHOD, b"ELLIPSOID:NEARPOINT"))
            || EQSTR(METHOD, b"NEARPOINT/ELLIPSOID"))
            || EQSTR(METHOD, b"ELLIPSOID/NEARPOINT"))
        {
            fstr::assign(SUBTYP, b"NEAR POINT");

            CHKOUT(b"ZZPRSMET", ctx)?;
            return Ok(());
        } else if (((EQSTR(METHOD, b"INTERCEPT:ELLIPSOID")
            || EQSTR(METHOD, b"ELLIPSOID:INTERCEPT"))
            || EQSTR(METHOD, b"INTERCEPT/ELLIPSOID"))
            || EQSTR(METHOD, b"ELLIPSOID/INTERCEPT"))
        {
            fstr::assign(SUBTYP, b"INTERCEPT");

            CHKOUT(b"ZZPRSMET", ctx)?;
            return Ok(());
        }

        //
        // At this point, we should have a "modern" style of method
        // specification for an ellipsoidal shape model.
        // Parse the method string.
        //
        // Traverse the tokens; identify clauses in the method string.
        //
        I = 1;

        while (I <= N) {
            fstr::assign(&mut CHR, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));

            if fstr::eq(&CHR, SLASH) {
                //
                // If the method string is correct, the next token should
                // be a keyword. There had better be a next token.
                //
                if (I == N) {
                    SETMSG(
                        b"Expected more tokens after final <#> in the method string <#>.",
                        ctx,
                    );
                    ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                    ERRCH(b"#", METHOD, ctx);
                    SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                    CHKOUT(b"ZZPRSMET", ctx)?;
                    return Ok(());
                }

                I = (I + 1);

                if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"ELLIPSOID") {
                    //
                    // This case is a no-op.
                    //
                } else if ((fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"INTERCEPT")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"NADIR"))
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"NEAR"))
                {
                    //
                    // This is a sub-point type specification.
                    //
                    NSUBTP = (NSUBTP + 1);

                    if (NSUBTP > 1) {
                        //
                        // We have too many sub-point specifications.
                        //
                        SETMSG(
                            b"Extra sub-point type <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    if fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"NEAR") {
                        //
                        // This may be a combination of old style and
                        // new syntax, e.g. 'ELLIPSOID/NEAR POINT'.
                        //
                        if (fstr::eq(SHAPE, b"ELLIPSOID") && (I < N)) {
                            //
                            // We allow the "near point" sub-point type
                            // for ellipsoids but not for DSK surfaces.
                            //
                            if fstr::eq(
                                fstr::substr(&LOCMTH, BEGS[(I + 1)]..=ENDS[(I + 1)]),
                                b"POINT",
                            ) {
                                fstr::assign(SUBTYP, b"NADIR");
                            }
                        }
                    } else {
                        fstr::assign(SUBTYP, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                    }
                } else if (fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"TANGENT")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"GUIDED"))
                {
                    //
                    // This is a point definition specification.
                    //

                    NLMBTP = (NLMBTP + 1);

                    if (NLMBTP > 1) {
                        //
                        // We have too many point definition specifications.
                        //
                        SETMSG(
                            b"Extra point definition <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    fstr::assign(PNTDEF, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                } else if (fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"UMBRAL")
                    || fstr::eq(fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), b"PENUMBRAL"))
                {
                    //
                    // This is a terminator type specification.
                    //
                    NTRMTP = (NTRMTP + 1);

                    if (NTRMTP > 1) {
                        //
                        // We have too many terminator type specifications.
                        //
                        SETMSG(
                            b"Extra terminator type <#> was found in the method string <#>.",
                            ctx,
                        );
                        ERRCH(b"#", fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]), ctx);
                        ERRCH(b"#", METHOD, ctx);
                        SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                        CHKOUT(b"ZZPRSMET", ctx)?;
                        return Ok(());
                    }

                    fstr::assign(TRMTYP, fstr::substr(&LOCMTH, BEGS[I]..=ENDS[I]));
                } else {
                    //
                    // Sorry, no other strings are allowed.
                    //
                    SETMSG(
                        b"Unexpected method string <#> was found specifying an ellipsoid shape.",
                        ctx,
                    );
                    ERRCH(b"#", METHOD, ctx);
                    SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                    CHKOUT(b"ZZPRSMET", ctx)?;
                    return Ok(());
                }
            } else {
                SETMSG(
                    b"Unexpected method string <#> was found specifying an ellipsoid shape.",
                    ctx,
                );
                ERRCH(b"#", METHOD, ctx);
                SIGERR(b"SPICE(BADMETHODSYNTAX)", ctx)?;
                CHKOUT(b"ZZPRSMET", ctx)?;
                return Ok(());
            }

            I = (I + 1);
        }
    } else {
        //
        // This is a backstop error check.
        //
        SETMSG(b"Unexpected shape value # was found. This is due to a coding error, not to user input.", ctx);
        ERRCH(b"#", SHAPE, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZPRSMET", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZPRSMET", ctx)?;
    Ok(())
}
