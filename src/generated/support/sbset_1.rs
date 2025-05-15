//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure SBSET ( String buffer, set value )
pub fn SBSET_1(
    NAME: &[u8],
    STR: &[u8],
    NAMES: CharArrayMut,
    PTRS: &mut [i32],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAMES = DummyCharArrayMut::new(NAMES, None, LBCELL..);
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..);
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..);
    let mut MAXSTR: i32 = 0;
    let mut NSTR: i32 = 0;
    let mut POS: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SBSET_1", ctx)?;
    }

    //
    // If the buffer already contains a string with this name, remove it.
    //
    SBREM_1(
        NAME,
        NAMES.as_arg_mut(),
        PTRS.as_slice_mut(),
        BUFFER.as_arg_mut(),
        ctx,
    )?;

    //
    // Recover the (new) essential control information.
    //
    MAXSTR = spicelib::SIZEC(NAMES.as_arg(), ctx)?;
    NSTR = spicelib::CARDC(NAMES.as_arg(), ctx)?;

    //
    // Where should the name be inserted?
    //
    if (NSTR == MAXSTR) {
        spicelib::SETMSG(b"Current limit is #.", ctx);
        spicelib::ERRINT(b"#", MAXSTR, ctx);
        spicelib::SIGERR(b"SPICE(SBTOOMANYSTRS)", ctx)?;
    } else {
        POS = (spicelib::LSTLEC(NAME, NSTR, NAMES.subarray(1)) + 1);

        //
        // Store only the non-blank part of the string. (Store a blank
        // string as a single blank character.)
        //
        F = intrinsics::MAX0(&[1, spicelib::FRSTNB(STR)]);
        L = intrinsics::MAX0(&[1, spicelib::LASTNB(STR)]);

        //
        // Add the name of the string to the name list, and the string
        // itself to the LB.
        //
        spicelib::INSLAC(
            CharArray::from_ref(NAME),
            1,
            POS,
            NAMES.subarray_mut(1),
            &mut NSTR,
            ctx,
        )?;
        spicelib::SCARDC(NSTR, NAMES.as_arg_mut(), ctx)?;

        LBINS_1(
            POS,
            fstr::substr(STR, F..=L),
            PTRS.as_slice_mut(),
            BUFFER.as_arg_mut(),
            ctx,
        )?;
    }

    spicelib::CHKOUT(b"SBSET_1", ctx)?;
    Ok(())
}
