//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
//---    The user defined functions required by GFUDS.
//
//      gfq    for udfunc
//      gfdecr for udqdec
//      gfrrdc for udqdec
//

//$Procedure GFQ ( Scalar function - range rate )
pub fn GFQ(ET: &mut f64, VALUE: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TARG: i32 = 0;
    let mut OBS: i32 = 0;
    let mut REF = [b' '; 12 as usize];
    let mut ABCORR = [b' '; 12 as usize];
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut LT: f64 = 0.0;

    //
    // Local variables.
    //

    //
    // Initialization. Retrieve the vector from the Sun to
    // the Moon in the J2000 frame, without aberration
    // correction.
    //
    TARG = 301;
    fstr::assign(&mut REF, b"J2000");
    fstr::assign(&mut ABCORR, b"NONE");
    OBS = 10;

    spicelib::SPKEZ(
        TARG,
        *ET,
        &REF,
        &ABCORR,
        OBS,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Calculate the scalar range rate corresponding the
    // STATE vector.
    //
    *VALUE = spicelib::DVNORM(STATE.as_slice());

    Ok(())
}
