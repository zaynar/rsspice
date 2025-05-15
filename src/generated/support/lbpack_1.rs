//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure LBPACK ( Line buffer, pack )
pub fn LBPACK_1(
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
    let mut OFFSET: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut INTLEN: i32 = 0;
    let mut J: i32 = 0;

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
        spicelib::CHKIN(b"LBPACK_1", ctx)?;
    }

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
    // For each interval in the complement...
    //
    OFFSET = (NLINE * 2);

    for I in intrinsics::range((OFFSET + 1), PCARD, 2) {
        //
        // Remove the contents of the interval from the CB, pulling
        // the remaining contents forward.
        //
        BEGIN = PTRS[I];
        END = PTRS[(I + 1)];
        INTLEN = ((END - BEGIN) + 1);

        if (BEGIN <= END) {
            CBREM_1(BEGIN, END, BUFFER.as_arg_mut(), ctx)?;

            //
            // Adjust the pointers for both the lines and the complement
            // intervals that followed the purged interval.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = PCARD;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (PTRS[J] > END) {
                        PTRS[J] = (PTRS[J] - INTLEN);
                    }
                    J += m3__;
                }
            }
        }
    }

    //
    // There is only one interval in the complement now. It begins
    // just after the last line, and runs to the end of the buffer.
    //
    spicelib::MAXAI(PTRS.subarray(1), OFFSET, &mut END, &mut J);

    PTRS[(OFFSET + 1)] = (END + 1);
    PTRS[(OFFSET + 2)] = SIZECB_1(BUFFER.as_arg(), ctx)?;

    LBUPD_1(NLINE, 1, PTRS.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"LBPACK_1", ctx)?;
    Ok(())
}
