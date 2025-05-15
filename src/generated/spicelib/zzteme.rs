//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZTEME ( J2000 to TEME at epoch )
pub fn ZZTEME(
    ET: f64,
    J2TM: &mut [f64],
    TM2J: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut J2TM = DummyArrayMut2D::new(J2TM, 1..=6, 1..=6);
    let mut TM2J = DummyArrayMut2D::new(TM2J, 1..=6, 1..=6);
    let mut M1 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut M1INV = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut M2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut M2INV = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XJ2000 = StackArray::<f64, 6>::new(1..=6);
    let mut Z = StackArray::<f64, 6>::new(1..=6);
    let mut ZJ2000 = StackArray::<f64, 6>::new(1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTEME", ctx)?;

    //
    // Extract the MEME +X vector and its derivative, both
    // expressed relative to the J2000 frame, into X.
    //
    ZZEPRC76(ET, M1.as_slice_mut(), ctx)?;
    INVSTM(M1.as_slice(), M1INV.as_slice_mut(), ctx)?;

    MOVED(M1INV.subarray([1, 1]), 6, XJ2000.as_slice_mut());

    //
    // Extract the TETE +Z vector and its derivative, both
    // expressed relative to the MEME frame, into Z.
    //
    ZZENUT80(ET, M2.as_slice_mut(), ctx)?;
    INVSTM(M2.as_slice(), M2INV.as_slice_mut(), ctx)?;

    MOVED(M2INV.subarray([1, 3]), 6, Z.as_slice_mut());

    //
    // Transform Z to the J2000 frame.
    //
    MXVG(M1INV.as_slice(), Z.as_slice(), 6, 6, ZJ2000.as_slice_mut());

    //
    // Compute the TEME to J2000 state transformation, and the
    // inverse.
    //
    ZZTWOVXF(
        ZJ2000.as_slice(),
        3,
        XJ2000.as_slice(),
        1,
        TM2J.as_slice_mut(),
        ctx,
    )?;
    INVSTM(TM2J.as_slice(), J2TM.as_slice_mut(), ctx)?;

    CHKOUT(b"ZZTEME", ctx)?;
    Ok(())
}
