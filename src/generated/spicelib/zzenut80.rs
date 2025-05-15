//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZENUT80 ( Earth nutation transformation, IAU 1980 model )
pub fn ZZENUT80(ET: f64, NUTXF: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NUTXF = DummyArrayMut2D::new(NUTXF, 1..=6, 1..=6);
    let mut DMOB: f64 = 0.0;
    let mut DVNUT = StackArray::<f64, 4>::new(1..=4);
    let mut EULANG = StackArray::<f64, 6>::new(1..=6);
    let mut MOB: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZENUT80", ctx)?;

    //
    // Get nutation angles and their rates.  We're expecting
    //
    //    DVNUT(1) = Psi------nutation in longitude (radians)
    //    DVNUT(2) = Epsilon--nutation in obliquity (radians)
    //    DVNUT(3) = dPsi/dt     (radians/second)
    //    DVNUT(4) = dEpsilon/dt (radians/second)
    //
    ZZWAHR(ET, DVNUT.as_slice_mut(), ctx);

    //
    // Get the mean obliquity of date.
    //
    // We're expecting the outputs to be as follows:
    //
    //     MOB      is the mean obliquity of the ecliptic at epoch
    //              ET. The mean obliquity of the ecliptic is the
    //              inclination of the ecliptic of date to the
    //              mean Earth equator of date.  Output units are
    //              radians.
    //
    //     DMOB     is the time derivative of MOB at ET, expressed
    //                    in radians per second.

    ZZMOBLIQ(ET, &mut MOB, &mut DMOB, ctx);

    //
    // The nutation rotation N is defined by
    //
    //
    //     N = [ -MOB - NUOBL ]  [ -NULON ]   [ MOB ]
    //                         1           3         1
    //
    // where MOBLIQ is the mean obliquity of the earth's ecliptic
    // at epoch, NUOB is nutation in obliquity at epoch, and
    // NULONG is nutation in longitude at epoch.  Using our
    // variable names, the Euler angle sequence is
    //
    //    [ -MOB - DVNUT(2) ]  [ -DVNUT(1) ]  [ MOB ]
    //                       1              3        1
    //
    // The rates corresponding to these angles are:
    //
    //    -DMOB - DVNUT(4),  -DVNUT(3),  DMOB
    //
    // We can use EUL2XF to form the state transformation from
    // the nutation base frame to the nutation frame.
    //

    EULANG[1] = (-MOB - DVNUT[2]);
    EULANG[2] = -DVNUT[1];
    EULANG[3] = MOB;
    EULANG[4] = (-DMOB - DVNUT[4]);
    EULANG[5] = -DVNUT[3];
    EULANG[6] = DMOB;

    EUL2XF(EULANG.as_slice(), 1, 3, 1, NUTXF.as_slice_mut(), ctx)?;

    CHKOUT(b"ZZENUT80", ctx)?;
    Ok(())
}
