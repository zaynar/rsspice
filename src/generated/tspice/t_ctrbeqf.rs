//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const WDSIZE: i32 = 32;

//$Procedure T_CTRBEQF ( Set Equal ZZBODS2C and ZZNAMFRM Counters )
pub fn T_CTRBEQF(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FRMCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut FRNAME = [b' '; WDSIZE as usize];
    let mut FRCODE: i32 = 0;
    let mut FOUND: bool = false;
    let mut SAVNAM = [b' '; WDSIZE as usize];
    let mut SAVCDE: i32 = 0;
    let mut BODCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut BODY = [b' '; WDSIZE as usize];
    let mut BODYID: i32 = 0;
    let mut SVBODY = [b' '; WDSIZE as usize];
    let mut SVBDID: i32 = 0;
    let mut SVFND: bool = false;
    let mut SAVCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut UPDATE: bool = false;
    let mut IVALS = StackArray::<i32, 1>::new(1..=1);

    //
    // SPICELIB functions.
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_CTRBEQF", ctx)?;
    }

    //
    // Get the current body name/ID translation state counter.
    //
    fstr::assign(&mut BODY, b"SUN");
    BODYID = -10;
    FOUND = false;
    fstr::assign(&mut SVBODY, b"EARTH");
    SVBDID = 399;
    SVFND = true;
    spicelib::ZZCTRUIN(BODCTR.as_slice_mut(), ctx);

    spicelib::ZZBODS2C(
        BODCTR.as_slice_mut(),
        &mut SVBODY,
        &mut SVBDID,
        &mut SVFND,
        &BODY,
        &mut BODYID,
        &mut FOUND,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
        return Ok(());
    }

    if !FOUND {
        spicelib::SETMSG(
            b"ZZBODS2C error: expected FOUND = .TRUE., got FOUND = .FALSE.",
            ctx,
        );
        spicelib::SIGERR(b"SPICE(ZZBODS2CFAILED1)", ctx)?;
        spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
        return Ok(());
    }

    if (BODYID != 10) {
        spicelib::SETMSG(
            b"ZZBODS2C error: expected BODYID = 10, got BODYID = #.",
            ctx,
        );
        spicelib::ERRINT(b"#", BODYID, ctx);
        spicelib::SIGERR(b"SPICE(ZZBODS2CFAILED2)", ctx)?;
        spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
        return Ok(());
    }

    //
    // Get the current frame name/ID translation state counter.
    //
    fstr::assign(&mut FRNAME, b"J2000");
    FRCODE = -1;
    fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
    SAVCDE = 17;
    spicelib::ZZCTRUIN(FRMCTR.as_slice_mut(), ctx);

    spicelib::ZZNAMFRM(
        FRMCTR.as_slice_mut(),
        &mut SAVNAM,
        &mut SAVCDE,
        &FRNAME,
        &mut FRCODE,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
        return Ok(());
    }

    if (FRCODE != 1) {
        spicelib::SETMSG(b"ZZNAMFRM error: expected FRCODE = 1, got FRCODE = #.", ctx);
        spicelib::ERRINT(b"#", FRCODE, ctx);
        spicelib::SIGERR(b"SPICE(ZZNAMFRMFAILED1)", ctx)?;
        spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
        return Ok(());
    }

    //
    // Check counters against each other to see if one of them
    // needs to be run up to match the other.
    //
    spicelib::MOVEI(BODCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZCTRCHK(FRMCTR.as_slice(), SAVCTR.as_slice_mut(), &mut UPDATE, ctx);

    if UPDATE {
        //
        // They are different. Which one is behind? NOTE: this check
        // relies on the two-integer counter implementation as of N0065.
        //
        if ((FRMCTR[2] > BODCTR[2]) || ((FRMCTR[2] == BODCTR[2]) && (FRMCTR[1] > BODCTR[1]))) {
            //
            // The body counter is behind. Run it up to the frame
            // counter by repeatedly calling BODDEF.
            //
            while UPDATE {
                spicelib::BODDEF(b"SUN", 10, ctx)?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                fstr::assign(&mut BODY, b"SUN");
                BODYID = -10;
                FOUND = false;
                fstr::assign(&mut SVBODY, b"EARTH");
                SVBDID = 399;
                SVFND = true;
                spicelib::ZZCTRUIN(BODCTR.as_slice_mut(), ctx);

                spicelib::ZZBODS2C(
                    BODCTR.as_slice_mut(),
                    &mut SVBODY,
                    &mut SVBDID,
                    &mut SVFND,
                    &BODY,
                    &mut BODYID,
                    &mut FOUND,
                    ctx,
                )?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                if !FOUND {
                    spicelib::SETMSG(
                        b"ZZBODS2C error: expected FOUND = .TRUE., got FOUND = .FALSE.",
                        ctx,
                    );
                    spicelib::SIGERR(b"SPICE(ZZBODS2CFAILED3)", ctx)?;
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                if (BODYID != 10) {
                    spicelib::SETMSG(
                        b"ZZBODS2C error: expected BODYID = 10, got BODYID = #.",
                        ctx,
                    );
                    spicelib::ERRINT(b"#", BODYID, ctx);
                    spicelib::SIGERR(b"SPICE(ZZBODS2CFAILED4)", ctx)?;
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                spicelib::MOVEI(BODCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

                spicelib::ZZCTRCHK(FRMCTR.as_slice(), SAVCTR.as_slice_mut(), &mut UPDATE, ctx);
            }
        } else {
            //
            // The frame counter is behind. Run it up to the body
            // counter by repeatedly calling PIPOOL.
            //
            while UPDATE {
                IVALS[1] = 0;
                spicelib::PIPOOL(b"F_CTRBEQF_DUMMY", 1, IVALS.as_slice(), ctx)?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                fstr::assign(&mut FRNAME, b"J2000");
                FRCODE = -1;
                fstr::assign(&mut SAVNAM, b"ECLIPJ2000");
                SAVCDE = 17;
                spicelib::ZZCTRUIN(FRMCTR.as_slice_mut(), ctx);

                spicelib::ZZNAMFRM(
                    FRMCTR.as_slice_mut(),
                    &mut SAVNAM,
                    &mut SAVCDE,
                    &FRNAME,
                    &mut FRCODE,
                    ctx,
                )?;

                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                if (FRCODE != 1) {
                    spicelib::SETMSG(b"ZZNAMFRM error: expected FRCODE = 1, got FRCODE = #.", ctx);
                    spicelib::ERRINT(b"#", FRCODE, ctx);
                    spicelib::SIGERR(b"SPICE(ZZNAMFRMFAILED2)", ctx)?;
                    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
                    return Ok(());
                }

                spicelib::MOVEI(BODCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

                spicelib::ZZCTRCHK(FRMCTR.as_slice(), SAVCTR.as_slice_mut(), &mut UPDATE, ctx);
            }
        }
    }

    spicelib::CHKOUT(b"T_CTRBEQF", ctx)?;
    Ok(())
}
