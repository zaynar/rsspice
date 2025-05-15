//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    SVTCDE: i32,
    SVCCDE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVTCDE: i32 = 0;
        let mut SVCCDE: i32 = 0;

        Self { SVTCDE, SVCCDE }
    }
}

//*****************************************************************
//*                                                               *
//*                                                               *
//*    Utility Routines                                           *
//*                                                               *
//*                                                               *
//*****************************************************************

//
// Subroutine package for referencing SPK ephemerides
// via external calls:
//
pub fn F_ZZEXTSPK(TRGCDE: i32, ET: f64, REF: &[u8], INCTR: i32, OUTCTR: i32, STATE: &[f64]) {

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
}

//
// Save target and center ID codes.
//
pub fn F_ZZSPKSET(TRGCDE: i32, INCTR: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.SVTCDE = TRGCDE;
    save.SVCCDE = INCTR;
}

//
// Look up state of target relative to center.
//
pub fn F_ZZSPKGET(
    ET: f64,
    REF: &[u8],
    OUTCTR: &mut i32,
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut LT: f64 = 0.0;

    spicelib::CHKIN(b"F_ZZSPKGET", ctx)?;

    *OUTCTR = save.SVCCDE;

    spicelib::SPKGEO(
        save.SVTCDE,
        ET,
        REF,
        *OUTCTR,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::CHKOUT(b"F_ZZSPKGET", ctx)?;
    Ok(())
}
