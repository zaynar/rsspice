//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure SGCTDF ( Generic Segments Create Test DAF )
pub fn SGCTDF(
    FILE: &[u8],
    ND: i32,
    NI: i32,
    VALUE: f64,
    CASES: &[i32],
    NCASE: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CASES = DummyArray::new(CASES, 1..);
    let mut DESCR = StackArray::<f64, 125>::new(1..=125);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SGCTDF", ctx)?;
    }

    //
    // Initialize DESCR to 0.
    //
    for I in 1..=125 {
        DESCR[I] = 0.0;
    }

    //
    // Create the new DAF. First check to make certain no file of name
    // FILE exists. If it does remove it.
    //
    testutil::KILFIL(FILE, ctx)?;
    spicelib::DAFONW(
        FILE,
        b"DAF/TST",
        ND,
        NI,
        b"Test DAF F_SGMETA",
        0,
        HANDLE,
        ctx,
    )?;

    //
    // Create each of the segments.  Note this is terribly inefficient,
    // but will work just fine for test cases.
    //
    for I in 1..=NCASE {
        //
        // Start a new segment.  Note the contents of the descriptor are
        // irrelevant, since we are only trying to test the functionality
        // of SGMETA.
        //
        spicelib::DAFBNA(*HANDLE, DESCR.as_slice(), b"Test Segment", ctx)?;

        //
        // Now load up the array.  This is where the serious inefficieny
        // comes in.
        //
        for J in 1..=(CASES[I] - 1) {
            spicelib::DAFADA(&[VALUE], 1, ctx)?;
        }

        //
        // Place the number of elements into the last entry in the array
        // before ending it's creation.
        //
        spicelib::DAFADA(&[(CASES[I] as f64)], 1, ctx)?;

        spicelib::DAFENA(ctx)?;
    }

    spicelib::CHKOUT(b"SGCTDF", ctx)?;
    Ok(())
}
