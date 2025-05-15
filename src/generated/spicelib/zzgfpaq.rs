//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZGFPAQ ( Private --- GF, phase angle between bodies )
pub fn ZZGFPAQ(
    ET: f64,
    TARG: i32,
    ILLMN: i32,
    OBS: i32,
    ABCORR: &[u8],
    VALUE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LT: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut PV1 = StackArray::<f64, 3>::new(1..=3);
    let mut PV2 = StackArray::<f64, 3>::new(1..=3);
    let mut REF = [b' '; 5 as usize];

    //
    // SPICELIB functions.
    //

    //
    // Local Variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFPAQ", ctx)?;

    //
    // This calculation is invariant with respect to reference frame.
    // Use J2000 for convenience.
    //
    fstr::assign(&mut REF, b"J2000");

    //
    // Get the position of the TARG object relative to OBS at ET.
    //
    SPKEZP(
        TARG,
        ET,
        &REF,
        ABCORR,
        OBS,
        PV1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFPAQ", ctx)?;
        return Ok(());
    }

    //
    // Get the state of the ILLMN object relative to TARG at ET
    // for no aberration correction, or ET - LT otherwise.
    //
    if EQSTR(ABCORR, b"NONE") {
        SPKEZP(
            ILLMN,
            ET,
            &REF,
            ABCORR,
            TARG,
            PV2.as_slice_mut(),
            &mut LT,
            ctx,
        )?;
    } else {
        SPKEZP(
            ILLMN,
            (ET - LT),
            &REF,
            ABCORR,
            TARG,
            PV2.as_slice_mut(),
            &mut LT,
            ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZGFPAQ", ctx)?;
        return Ok(());
    }

    //
    //                   ILLMN      OBS
    //   ILLMN as seen      ^       /
    //   from TARG at       |      /
    //   ET - LT.           |     /
    //                     >|..../< phase angle
    //                      |   /
    //                    . |  /
    //                  .   | /
    //                 .     v     TARG as seen from OBS
    //           SEP   .   TARG    at ET
    //                  .  /
    //                    /
    //                   v
    //
    //    PI = SEP + PHASE
    //
    //    so
    //
    //    PHASE = PI - SEP
    //
    // Calculate the angle separating the vectors relative to TARG
    //
    SEP = VSEP(PV1.as_slice(), PV2.as_slice(), ctx);

    //
    // The angle of interest is that between -PV1 and PV2 measured from
    // TARG. Subtract SEP from PI to calculate this angle.
    //
    *VALUE = (PI(ctx) - SEP);

    //
    // All done.
    //
    CHKOUT(b"ZZGFPAQ", ctx)?;
    Ok(())
}
