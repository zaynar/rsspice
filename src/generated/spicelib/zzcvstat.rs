//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FRNMLN: i32 = 32;

struct SaveVars {
    SVREF: Vec<u8>,
    SVET: f64,
    SVSTAT: StackArray<f64, 6>,
    SVCTR: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVET: f64 = 0.0;
        let mut SVSTAT = StackArray::<f64, 6>::new(1..=6);
        let mut SVCTR: i32 = 0;

        Self {
            SVREF,
            SVET,
            SVSTAT,
            SVCTR,
        }
    }
}

//$Procedure ZZCVSTAT ( Constant velocity state )
pub fn ZZCVSTAT(
    ET: f64,
    REF: &[u8],
    CENTER: i32,
    STATE: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
    // Saved variables
    //

    //
    // This routine should never be called.
    //
    CHKIN(b"ZZCVSTAT", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZCVSTAT", ctx)?;
    Ok(())
}

//$Procedure ZZCVXSTA ( Constant velocity state, fetch state )
pub fn ZZCVXSTA(
    ET: f64,
    REF: &[u8],
    CENTER: &mut i32,
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut DELTA: f64 = 0.0;
    let mut XSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut XF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZCVXSTA", ctx)?;

    //
    // Extrapolate the saved state to the input time.
    //
    DELTA = (ET - save.SVET);

    VLCOM(
        1.0,
        save.SVSTAT.subarray(1),
        DELTA,
        save.SVSTAT.subarray(4),
        XSTATE.as_slice_mut(),
    );

    VEQU(save.SVSTAT.subarray(4), XSTATE.subarray_mut(4));

    //
    // Convert the extrapolated state to the requested frame
    // at ET.
    //
    SXFORM(&save.SVREF, REF, ET, XF.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZCVXSTA", ctx)?;
        return Ok(());
    }

    MXVG(XF.as_slice(), XSTATE.as_slice(), 6, 6, STATE.as_slice_mut());

    //
    // Set the output center of motion argument as well.
    //
    *CENTER = save.SVCTR;

    CHKOUT(b"ZZCVXSTA", ctx)?;
    Ok(())
}

//$Procedure ZZCVSSTA ( Constant velocity state, store parameters )
pub fn ZZCVSSTA(STATE: &[f64], CENTER: i32, ET: f64, REF: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STATE = DummyArray::new(STATE, 1..=6);

    if RETURN(ctx) {
        return;
    }

    //
    // No SPICE errors are detected here, so we don't check in.
    //
    //
    // Save all inputs.
    //
    MOVED(STATE.as_slice(), 6, save.SVSTAT.as_slice_mut());

    save.SVCTR = CENTER;
    save.SVET = ET;
    fstr::assign(&mut save.SVREF, REF);
}
