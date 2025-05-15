//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NLAT: i32 = 500;
const NLON: i32 = 1000;

//$Procedure ZZT_BODDSK (Make type 2 DSK for body's reference ellipsoid)
pub fn ZZT_BODDSK(
    DSK: &[u8],
    BODY: &[u8],
    FIXREF: &[u8],
    LOAD: bool,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BODCDE: i32 = 0;
    let mut FRCENT: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut FRCLID: i32 = 0;
    let mut SURFID: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZT_BODDSK", ctx)?;

    //
    // Get body ID code.
    //
    spicelib::BODS2C(BODY, &mut BODCDE, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"The body name # could not be mapped to an ID code.", ctx);
        spicelib::ERRCH(b"#", BODY, ctx);
        spicelib::SIGERR(b"SPICE(BODYIDNOTFOUND)", ctx)?;
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    //
    // Map frame name to code.
    //
    spicelib::NAMFRM(FIXREF, &mut FRCODE, ctx)?;

    if (FRCODE == 0) {
        spicelib::SETMSG(b"The frame name # could not be mapped to an ID code.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    //
    // Get frame info and check frame center.
    //
    spicelib::FRINFO(
        FRCODE,
        &mut FRCENT,
        &mut FRCLSS,
        &mut FRCLID,
        &mut FOUND,
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    if !FOUND {
        //
        // We really don't expect to get here, but....
        //
        spicelib::SETMSG(b"The frame name # could not be mapped to an ID code.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    if (FRCENT != BODCDE) {
        spicelib::SETMSG(b"The frame # is centered at #; the center must coincide with the body # which has ID #.", ctx);
        spicelib::ERRCH(b"#", FIXREF, ctx);
        spicelib::ERRINT(b"#", FRCENT, ctx);
        spicelib::ERRCH(b"#", BODY, ctx);
        spicelib::ERRINT(b"#", BODCDE, ctx);
        spicelib::SIGERR(b"SPICE(BADFRAMECENTER)", ctx)?;
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    //
    // Create DSK file.
    //
    // Some of the *PL02 routines require the surface ID to
    // match the body ID.
    //
    SURFID = BODCDE;

    T_ELDS2Z(BODCDE, SURFID, FIXREF, NLON, NLAT, DSK, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
        return Ok(());
    }

    //
    // Open the file for read access if so directed.
    //
    if LOAD {
        spicelib::DASOPR(DSK, HANDLE, ctx)?;
    } else {
        //
        // Ensure that the stale handle can't be used.
        //
        *HANDLE = 0;
    }

    spicelib::CHKOUT(b"ZZT_BODDSK", ctx)?;
    Ok(())
}
