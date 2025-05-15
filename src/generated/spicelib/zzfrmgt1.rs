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
    IDNT66: StackArray2D<f64, 36>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VERSN = vec![b' '; 6 as usize];
        let mut IDNT66 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut PASS1: bool = false;

        PASS1 = true;
        fstr::assign(&mut VERSN, b"4.0.0");

        Self {
            VERSN,
            IDNT66,
            PASS1,
        }
    }
}

//$Procedure      ZZFRMGT1 ( Frame get transformation )
pub fn ZZFRMGT1(
    INFRM: i32,
    ET: f64,
    XFORM: &mut [f64],
    OUTFRM: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROTATE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

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

    CHKIN(b"ZZFRMGT1", ctx)?;

    //
    // On the first pass, initialize the identity matrix.
    //
    if save.PASS1 {
        CLEARD(36, save.IDNT66.as_slice_mut());

        for I in 1..=6 {
            save.IDNT66[[I, I]] = 1.0;
        }

        save.PASS1 = false;
    }

    //
    // Get all the needed information about this frame.
    //
    FRINFO(INFRM, &mut CENTER, &mut TYPE, &mut TYPEID, FOUND, ctx)?;

    if !*FOUND {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;

        CHKOUT(b"ZZFRMGT1", ctx)?;
        return Ok(());
    }

    //
    // FOUND was set to true by the FRINFO call. Compute transformation
    // based on the frame class.
    //
    if (TYPE == INERTL) {
        IRFROT(INFRM, 1, ROTATE.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            for I in 1..=3 {
                for J in 1..=3 {
                    XFORM[[I, J]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), (J + 3)]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), J]] = 0.0;
                    XFORM[[I, (J + 3)]] = 0.0;
                }
            }

            *OUTFRM = 1;
        }
    } else if (TYPE == PCK) {
        TISBOD(b"J2000", TYPEID, ET, TSIPM.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            INVSTM(TSIPM.as_slice(), XFORM.as_slice_mut(), ctx)?;

            *OUTFRM = 1;
        }
    } else if (TYPE == CK) {
        CKFXFM(TYPEID, ET, XFORM.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == TK) {
        TKFRAM(TYPEID, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;

        if !FAILED(ctx) {
            for I in 1..=3 {
                for J in 1..=3 {
                    XFORM[[I, J]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), (J + 3)]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), J]] = 0.0;
                    XFORM[[I, (J + 3)]] = 0.0;
                }
            }
        }
    } else if (TYPE == DYN) {
        SETMSG(b"The reference frame # is a dynamic frame. Dynamic frames may not be used at recursion level 1.", ctx);
        ERRINT(b"#", INFRM, ctx);
        SIGERR(b"SPICE(RECURSIONTOODEEP)", ctx)?;
        CHKOUT(b"ZZFRMGT1", ctx)?;
        return Ok(());
    } else if (TYPE == SWTCH) {
        ZZSWFXFM(INFRM, ET, 6, XFORM.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;

        SETMSG(b"The reference frame # has class #. This form of reference frame is not supported in version # of ZZFRMGT1. You need to update your version of SPICELIB to the latest version in order to support this frame. ", ctx);

        ERRINT(b"#", INFRM, ctx);
        ERRINT(b"#", TYPE, ctx);
        ERRCH(b"#", &save.VERSN, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAMETYPE)", ctx)?;
        CHKOUT(b"ZZFRMGT1", ctx)?;
        return Ok(());
    }

    //
    // Make sure to clear outputs in case of a failure as defined in
    // in the header.
    //
    if (FAILED(ctx) || !*FOUND) {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;
    }

    CHKOUT(b"ZZFRMGT1", ctx)?;
    Ok(())
}
