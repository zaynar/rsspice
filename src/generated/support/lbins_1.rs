//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure LBINS ( Line buffer, insert )
pub fn LBINS_1(
    POS: i32,
    LINE: &[u8],
    PTRS: &mut [i32],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..);
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..);
    let mut MAXLN: i32 = 0;
    let mut NLINE: i32 = 0;
    let mut NCOM: i32 = 0;
    let mut PCARD: i32 = 0;
    let mut POSPTR: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut LNLEN: i32 = 0;
    let mut AVAIL: i32 = 0;
    let mut PTR = StackArray::<i32, 2>::new(1..=2);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Equivalences
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"LBINS_1", ctx)?;
    }

    //
    // Recover all the essential control information.
    //
    LBDES_1(
        PTRS.as_slice(),
        &mut MAXLN,
        &mut NLINE,
        &mut NCOM,
        &mut PCARD,
        ctx,
    )?;

    //
    // Where should this line be inserted, if at all?
    //
    if (NLINE == MAXLN) {
        spicelib::SETMSG(b"Current line limit is #.", ctx);
        spicelib::ERRINT(b"#", MAXLN, ctx);
        spicelib::SIGERR(b"SPICE(LBTOOMANYLINES)", ctx)?;
    } else if ((POS < 1) || ((POS - NLINE) > 1)) {
        spicelib::SETMSG(b"Tried to access line # of #.", ctx);
        spicelib::ERRINT(b"#", POS, ctx);
        spicelib::ERRINT(b"#", NLINE, ctx);
        spicelib::SIGERR(b"SPICE(LBNOSUCHLINE)", ctx)?;
    } else {
        POSPTR = ((2 * POS) - 1);

        //
        // Leading blanks are significant; trailing blanks are history.
        // (Store a blank string as a single blank character.)
        //
        F = 1;
        L = intrinsics::MAX0(&[1, spicelib::LASTNB(LINE)]);
        LNLEN = ((L - F) + 1);

        //
        // Store each new string at the end of the end of the CB.
        // If the final interval in the complement isn't large enough
        // to hold the new string, pack the CB and try again.
        //
        AVAIL = ((PTRS[PCARD] - PTRS[(PCARD - 1)]) + 1);

        if (AVAIL < LNLEN) {
            LBPACK_1(PTRS.as_slice_mut(), BUFFER.as_arg_mut(), ctx)?;

            LBDES_1(
                PTRS.as_slice(),
                &mut MAXLN,
                &mut NLINE,
                &mut NCOM,
                &mut PCARD,
                ctx,
            )?;
            AVAIL = ((PTRS[PCARD] - PTRS[(PCARD - 1)]) + 1);
        }

        //
        // If there still isn't enough room? Well, those are the breaks.
        //
        if (AVAIL < LNLEN) {
            spicelib::SIGERR(b"SPICE(LBLINETOOLONG)", ctx)?;

        //
        // If there is room, allocate just enough of the final interval
        // in the complement to contain the new string; store the string;
        // and insert the name and pointers at their proper locations.
        //
        } else {
            PTR[1] = PTRS[(PCARD - 1)];
            PTR[2] = ((PTR[1] + LNLEN) - 1);
            PTRS[(PCARD - 1)] = (PTR[2] + 1);

            CBPUT_1(
                PTR[1],
                PTR[2],
                fstr::substr(LINE, F..=L),
                BUFFER.as_arg_mut(),
                ctx,
            )?;

            spicelib::INSLAI(
                PTR.as_slice(),
                2,
                POSPTR,
                PTRS.subarray_mut(1),
                &mut PCARD,
                ctx,
            )?;

            LBUPD_1((NLINE + 1), NCOM, PTRS.as_slice_mut(), ctx)?;
        }
    }

    spicelib::CHKOUT(b"LBINS_1", ctx)?;
    Ok(())
}
