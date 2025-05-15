//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const SCALE: f64 = 5.0;
const MAXV: i32 = (3 * 500);
const MAXP: i32 = (2 * MAXV);

//$Procedure ZZT_CG ( Make plate set for shape like comet C-G )
pub fn ZZT_CG(VOUT: &mut [f64], POUT: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut A: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP0 = ActualArray::<f64>::new(LBCELL..=MAXV);
    let mut VTEMP1 = ActualArray::<f64>::new(LBCELL..=MAXV);
    let mut PTEMP0 = ActualArray::<i32>::new(LBCELL..=MAXP);
    let mut PTEMP1 = ActualArray::<i32>::new(LBCELL..=MAXP);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZT_CG", ctx)?;

    spicelib::SSIZED(MAXV, VTEMP0.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXV, VTEMP1.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXP, PTEMP0.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXP, PTEMP1.as_slice_mut(), ctx)?;

    A = SCALE;

    ZZPSBOX(
        A,
        ((3 as f64) * A),
        ((4 as f64) * A),
        VOUT.as_slice_mut(),
        POUT.as_slice_mut(),
        ctx,
    )?;
    ZZPSBOX(A, A, A, VTEMP0.as_slice_mut(), PTEMP0.as_slice_mut(), ctx)?;

    OFFSET[1] = (1.5 * A);
    OFFSET[2] = A;
    OFFSET[3] = 0.0;

    ZZPSXLAT(
        VTEMP0.as_slice(),
        OFFSET.as_slice(),
        VTEMP1.as_slice_mut(),
        ctx,
    )?;
    spicelib::COPYD(VTEMP1.as_slice(), VTEMP0.as_slice_mut(), ctx)?;

    ZZPSUN(
        VTEMP0.as_slice(),
        PTEMP0.as_slice(),
        VOUT.as_slice(),
        POUT.as_slice(),
        VTEMP1.as_slice_mut(),
        PTEMP1.as_slice_mut(),
        ctx,
    )?;

    spicelib::COPYD(VTEMP1.as_slice(), VOUT.as_slice_mut(), ctx)?;
    spicelib::COPYI(PTEMP1.as_slice(), POUT.as_slice_mut(), ctx)?;

    ZZPSBOX(
        A,
        ((5 as f64) * A),
        ((3 as f64) * A),
        VTEMP0.as_slice_mut(),
        PTEMP0.as_slice_mut(),
        ctx,
    )?;

    OFFSET[1] = ((3 as f64) * A);
    OFFSET[2] = A;
    OFFSET[3] = A;

    ZZPSXLAT(
        VTEMP0.as_slice(),
        OFFSET.as_slice(),
        VTEMP1.as_slice_mut(),
        ctx,
    )?;
    spicelib::COPYD(VTEMP1.as_slice(), VTEMP0.as_slice_mut(), ctx)?;

    ZZPSUN(
        VTEMP0.as_slice(),
        PTEMP0.as_slice(),
        VOUT.as_slice(),
        POUT.as_slice(),
        VTEMP1.as_slice_mut(),
        PTEMP1.as_slice_mut(),
        ctx,
    )?;

    spicelib::COPYD(VTEMP1.as_slice(), VOUT.as_slice_mut(), ctx)?;
    spicelib::COPYI(PTEMP1.as_slice(), POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZT_CG", ctx)?;
    Ok(())
}
