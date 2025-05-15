//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;

//$Procedure ZZTSTSPK ( Create an SPK file for use in testing )
pub fn ZZTSTSPK(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FRAME = [b' '; WDSIZE as usize];
    let mut SEGID = [b' '; WDSIZE as usize];
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut DEGREE: i32 = 0;
    let mut I: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut N: i32 = 0;
    let mut REF: i32 = 0;
    let mut GM: f64 = 0.0;
    let mut STATES = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut EPOCH: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut LAST: f64 = 0.0;
    let mut STEP: f64 = 0.0;

    //
    // Local Variables.
    //

    //
    // Wipe out any existing file with the target name.  Then open
    // a new SPC file for writing.
    //
    KILFIL(FILE, ctx)?;
    spicelib::SPCOPN(FILE, b"TestUtilitySPK", &mut HANDLE, ctx)?;
    //
    // Now just construct the state information needed  to create
    // segments for all of the various objects that we are going
    // to simulate.
    //
    EPOCH = -189345600.0;

    I = 1;
    TSTSTC(I, &mut BODY, ctx);

    while (BODY != 0) {
        TSTST(
            BODY,
            EPOCH,
            &mut SEGID,
            &mut REF,
            STATES.as_slice_mut(),
            &mut CENTER,
            &mut GM,
            ctx,
        )?;
        spicelib::FRMNAM(REF, &mut FRAME, ctx)?;

        if (GM > 0.0) {
            FIRST = -500000000.0;
            LAST = 500000000.0;
            N = 1;

            spicelib::SPKW05(
                HANDLE,
                BODY,
                CENTER,
                &FRAME,
                FIRST,
                LAST,
                &SEGID,
                GM,
                N,
                STATES.as_slice(),
                &[EPOCH],
                ctx,
            )?;
        } else {
            FIRST = -500000000.0;
            LAST = 500000000.0;
            N = 2;
            STEP = 1000000000.0;
            DEGREE = 1;

            STATES[[1, 2]] = STATES[[1, 1]];
            STATES[[2, 2]] = STATES[[2, 1]];
            STATES[[3, 2]] = STATES[[3, 1]];
            STATES[[4, 2]] = STATES[[4, 1]];
            STATES[[5, 2]] = STATES[[5, 1]];
            STATES[[6, 2]] = STATES[[6, 1]];

            spicelib::SPKW08(
                HANDLE,
                BODY,
                CENTER,
                &FRAME,
                FIRST,
                LAST,
                &SEGID,
                DEGREE,
                N,
                STATES.as_slice(),
                FIRST,
                STEP,
                ctx,
            )?;
        }

        I = (I + 1);
        TSTSTC(I, &mut BODY, ctx);
    }

    spicelib::DAFCLS(HANDLE, ctx)?;

    Ok(())
}
