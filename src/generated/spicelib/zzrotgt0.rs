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

struct SaveVars {
    VERSN: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VERSN = vec![b' '; 6 as usize];

        fstr::assign(&mut VERSN, b"4.0.0");

        Self { VERSN }
    }
}

//$Procedure      ZZROTGT0 ( Frame get rotation )
pub fn ZZROTGT0(
    INFRM: i32,
    ET: f64,
    ROTATE: &mut [f64],
    OUTFRM: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROTATE = DummyArrayMut2D::new(ROTATE, 1..=3, 1..=3);
    let mut CENTER: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Set output flag.
    //
    *FOUND = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZROTGT0", ctx)?;

    //
    // Get all the needed information about this frame.
    //
    FRINFO(INFRM, &mut CENTER, &mut TYPE, &mut TYPEID, FOUND, ctx)?;

    if !*FOUND {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;

        CHKOUT(b"ZZROTGT0", ctx)?;
        return Ok(());
    }

    //
    // FOUND was set to true by the FRINFO call. Compute rotation based
    // on the frame class.
    //
    if (TYPE == INERTL) {
        IRFROT(INFRM, 1, ROTATE.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            *OUTFRM = 1;
        }
    } else if (TYPE == PCK) {
        TIPBOD(b"J2000", TYPEID, ET, TIPM.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            XPOSE(TIPM.as_slice(), ROTATE.as_slice_mut());

            *OUTFRM = 1;
        }
    } else if (TYPE == CK) {
        CKFROT(TYPEID, ET, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == TK) {
        TKFRAM(TYPEID, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == DYN) {
        //
        // Unlike the other frame classes, the dynamic frame evaluation
        // routine ZZDYNROT requires the input frame ID rather than the
        // dynamic frame class ID. ZZDYNROT also requires the center ID
        // we found via the FRINFO call.
        //
        ZZDYNRT0(INFRM, CENTER, ET, ROTATE.as_slice_mut(), OUTFRM, ctx)?;

    //
    // The FOUND flag was set by FRINFO earlier; we don't touch
    // it here. If ZZDYNROT signaled an error, FOUND will be set
    // to .FALSE. at end of this routine.
    //
    } else if (TYPE == SWTCH) {
        ZZSWFXFM(INFRM, ET, 3, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;

        SETMSG(b"The reference frame # has class #. This form of reference frame is not supported in version # of ZZROTGT0. You need to update your version of SPICELIB to the latest version in order to support this frame. ", ctx);

        ERRINT(b"#", INFRM, ctx);
        ERRINT(b"#", TYPE, ctx);
        ERRCH(b"#", &save.VERSN, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAMETYPE)", ctx)?;
        CHKOUT(b"ZZROTGT0", ctx)?;
        return Ok(());
    }

    //
    // Make sure to clear outputs in case of a failure as defined in
    // in the header.
    //
    if (FAILED(ctx) || !*FOUND) {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;
    }

    CHKOUT(b"ZZROTGT0", ctx)?;
    Ok(())
}
