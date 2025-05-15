//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

struct SaveVars {
    CTRHGH: i32,
    CTRLOW: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CTRHGH: i32 = 0;
        let mut CTRLOW: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            CTRHGH,
            CTRLOW,
            FIRST,
        }
    }
}

//$Procedure ZZCTR ( Manipulate Counter Array )
pub fn ZZCTR(
    NEWCTR: &[i32],
    OLDCTR: &[i32],
    UPDATE: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Save EVERYTHING.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // This routine should never be called. If this routine is called,
    // an error is signaled.
    //
    CHKIN(b"ZZCTR", ctx)?;

    SETMSG(b"ZZCTR: You have called an entry which performs performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine ZZCTR.", ctx);

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZCTR", ctx)?;

    Ok(())
}

//$Procedure ZZCTRUIN ( CounTeR array, User counter INitialization )
pub fn ZZCTRUIN(OLDCTR: &mut [i32], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut OLDCTR = DummyArrayMut::new(OLDCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return;
    }

    //
    // Initialize the high and low values.
    //
    if save.FIRST {
        save.CTRHGH = INTMAX();
        save.CTRLOW = INTMIN();

        save.FIRST = false;
    }

    //
    // Set counter.
    //
    OLDCTR[1] = save.CTRHGH;
    OLDCTR[2] = save.CTRHGH;
}

//$Procedure ZZCTRSIN ( CounTeR array, Subsystem counter INitialization )
pub fn ZZCTRSIN(OLDCTR: &mut [i32], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut OLDCTR = DummyArrayMut::new(OLDCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return;
    }

    //
    // Initialize the high and low values.
    //
    if save.FIRST {
        save.CTRHGH = INTMAX();
        save.CTRLOW = INTMIN();

        save.FIRST = false;
    }

    //
    // Set counter.
    //
    OLDCTR[1] = save.CTRLOW;
    OLDCTR[2] = save.CTRLOW;
}

//$Procedure ZZCTRINC ( CounTeR Array, INCrement counter )
pub fn ZZCTRINC(OLDCTR: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut OLDCTR = DummyArrayMut::new(OLDCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Initialize the high and low values.
    //
    if save.FIRST {
        save.CTRHGH = INTMAX();
        save.CTRLOW = INTMIN();

        save.FIRST = false;
    }

    //
    // Signal an error if both input counter array elements have high
    // values.
    //
    if ((OLDCTR[1] == save.CTRHGH) && (OLDCTR[2] == save.CTRHGH)) {
        CHKIN(b"ZZCTRINC", ctx)?;

        SETMSG(b"A subsystem state counter overflowed. For this to happen there must be a SPICE bug or you must have been running your SPICE-based application for a very long time. Please contact NAIF.and report the circumstances under which this happened.", ctx);

        SIGERR(b"SPICE(SPICEISTIRED)", ctx)?;

        CHKOUT(b"ZZCTRINC", ctx)?;

        return Ok(());
    }

    //
    // Increment counters.
    //
    if (OLDCTR[1] == save.CTRHGH) {
        OLDCTR[1] = save.CTRLOW;
        OLDCTR[2] = (OLDCTR[2] + 1);
    } else {
        OLDCTR[1] = (OLDCTR[1] + 1);
    }

    Ok(())
}

//$Procedure ZZCTRCHK ( CounTeR array, CHecK and update )
pub fn ZZCTRCHK(NEWCTR: &[i32], OLDCTR: &mut [i32], UPDATE: &mut bool, ctx: &mut Context) {
    let NEWCTR = DummyArray::new(NEWCTR, 1..=CTRSIZ);
    let mut OLDCTR = DummyArrayMut::new(OLDCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return;
    }

    //
    // Do counters differ?
    //
    *UPDATE = ((NEWCTR[1] != OLDCTR[1]) || (NEWCTR[2] != OLDCTR[2]));

    //
    // If they do, set old counter to new value.
    //
    if *UPDATE {
        OLDCTR[1] = NEWCTR[1];
        OLDCTR[2] = NEWCTR[2];
    }
}
