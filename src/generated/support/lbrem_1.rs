//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure LBREM ( Line buffer, remove )
pub fn LBREM_1(
    POS: i32,
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
    let mut POSCOM: i32 = 0;
    let mut PTR = StackArray::<i32, 2>::new(1..=2);
    let mut OFFSET: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Other Functions
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
        spicelib::CHKIN(b"LBREM_1", ctx)?;
    }
    //
    // We touch the input buffer so that compilers will not complain
    // that BUFFER is an unused argument.  It really is unused, but
    // it's in the calling sequence for the sake of uniformity of
    // the calling sequences for the line buffer routines.
    //
    spicelib::TOUCHC(
        &BUFFER[LBCBUF].to_vec(),
        fstr::substr_mut(&mut BUFFER[LBCBUF], 1..=1),
    );

    //
    // Recover the essential control information.
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
    // No way to remove a line that's not in the table.
    //
    if ((POS < 1) || (POS > NLINE)) {
        spicelib::SETMSG(b"Tried to access line # of #.", ctx);
        spicelib::ERRINT(b"#", POS, ctx);
        spicelib::ERRINT(b"#", NLINE, ctx);
        spicelib::SIGERR(b"SPICE(LBNOSUCHLINE)", ctx)?;
        spicelib::CHKOUT(b"LBREM_1", ctx)?;
        return Ok(());
    }

    //
    // Save the bounds of the stored line before removing the name
    // and pointers from their respective tables.
    //
    POSPTR = ((2 * POS) - 1);
    PTR[1] = PTRS[POSPTR];
    PTR[2] = PTRS[(POSPTR + 1)];
    NLINE = (NLINE - 1);

    spicelib::REMLAI(2, POSPTR, PTRS.subarray_mut(1), &mut PCARD, ctx)?;

    //
    // Add the interval to the complement. Insert it directly, then
    // do any merges required.
    //
    OFFSET = (NLINE * 2);
    POSCOM = (OFFSET + 1);

    for I in intrinsics::range((OFFSET + 2), PCARD, 2) {
        if (PTR[1] > PTRS[I]) {
            POSCOM = (I + 1);
        }
    }

    spicelib::INSLAI(
        PTR.as_slice(),
        2,
        POSCOM,
        PTRS.subarray_mut(1),
        &mut PCARD,
        ctx,
    )?;

    for I in intrinsics::range((PCARD - 2), (OFFSET + 2), -2) {
        if (PTRS[(I + 1)] == (PTRS[I] + 1)) {
            spicelib::REMLAI(2, I, PTRS.subarray_mut(1), &mut PCARD, ctx)?;
        }
    }

    NCOM = ((PCARD / 2) - NLINE);

    LBUPD_1(NLINE, NCOM, PTRS.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"LBREM_1", ctx)?;
    Ok(())
}
