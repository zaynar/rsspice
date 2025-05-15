//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBPOOL: i32 = -5;
const PREV: i32 = 2;
const NEXT: i32 = 1;

//$Procedure      ZZGPNM ( Get position of a name )
pub fn ZZGPNM(
    NAMLST: &mut [i32],
    NMPOOL: &mut [i32],
    NAMES: CharArrayMut,
    DATLST: &[i32],
    DPPOOL: &[i32],
    DPVALS: &[f64],
    CHPOOL: &[i32],
    CHVALS: CharArray,
    VARNAM: &[u8],
    FOUND: &mut bool,
    LOOKAT: &mut i32,
    NAMEAT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAMLST = DummyArrayMut::new(NAMLST, 1..);
    let mut NMPOOL = DummyArrayMut2D::new(NMPOOL, 1..=2, LBPOOL..);
    let mut NAMES = DummyCharArrayMut::new(NAMES, None, 1..);
    let mut NODE: i32 = 0;
    let mut FULL: bool = false;
    let mut HEAD: i32 = 0;
    let mut TAIL: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Parameters
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGPNM", ctx)?;

    *NAMEAT = 0;

    //
    //
    // Locate this variable name in the name pool or insert it
    // if it isn't there.  The location will be NAMEAT and
    // we will use the variable FOUND to indicate whether or
    // not it was already present.
    //
    *LOOKAT = ZZHASH(VARNAM, ctx)?;
    NODE = NAMLST[*LOOKAT];
    FULL = (LNKNFN(NMPOOL.as_slice()) <= 0);
    *FOUND = false;

    //
    // See if this name (or one colliding with it in the
    // hash scheme) has already been stored in the name list.
    //
    if (NODE > 0) {
        HEAD = NODE;
        TAIL = -NMPOOL[[PREV, HEAD]];

        while ((NODE > 0) && !*FOUND) {
            *FOUND = fstr::eq(NAMES.get(NODE), VARNAM);
            *NAMEAT = NODE;
            NODE = NMPOOL[[NEXT, NODE]];
        }

        if (!*FOUND && !FULL) {
            //
            // We didn't find this name on the conflict resolution
            // list. Allocate a new slot for it.
            //
            LNKAN(NMPOOL.as_slice_mut(), &mut NODE, ctx)?;
            LNKILA(TAIL, NODE, NMPOOL.as_slice_mut(), ctx)?;

            fstr::assign(NAMES.get_mut(NODE), VARNAM);
            *NAMEAT = NODE;
        }
    } else if !FULL {
        //
        // Nothing like this variable name (in the hashing sense)
        // has been loaded so far.  We need to allocate
        // a name slot for this variable.
        //
        LNKAN(NMPOOL.as_slice_mut(), &mut NODE, ctx)?;

        NAMLST[*LOOKAT] = NODE;
        fstr::assign(NAMES.get_mut(NODE), VARNAM);
        *NAMEAT = NODE;
    }

    //
    // If the name pool was full and we didn't find this name
    // we've got an error. Diagnose it and return.
    //
    if (FULL && !*FOUND) {
        SETMSG(
            b"The kernel pool does not have room for any more variables.",
            ctx,
        );
        SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
        CHKOUT(b"ZZGPNM", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGPNM", ctx)?;
    Ok(())
}
