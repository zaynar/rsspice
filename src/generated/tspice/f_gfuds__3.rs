//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure GFRRDC ( Derivative analytic/numeric )
pub fn GFRRDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    DECRES: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut N: i32 = 0;
    let mut TARG: i32 = 0;
    let mut OBS: i32 = 0;
    let mut DT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut DRVEL: f64 = 0.0;
    let mut SRHAT = StackArray::<f64, 6>::new(1..=6);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut DFDT = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut REF = [b' '; 12 as usize];
    let mut ABCORR = [b' '; 12 as usize];

    //
    // SPICELIB functions.
    //

    DT = 1.0;
    N = 6;
    TARG = 301;
    fstr::assign(&mut REF, b"J2000");
    fstr::assign(&mut ABCORR, b"NONE");
    OBS = 10;

    spicelib::SPKEZ(
        TARG,
        (*ET - DT),
        &REF,
        &ABCORR,
        OBS,
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKEZ(
        TARG,
        (*ET + DT),
        &REF,
        &ABCORR,
        OBS,
        STATE2.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    //
    // Approximate the derivative of the position and velocity by
    // finding the derivative of a quadratic approximating function.
    //
    //    DFDT(1) = Vx
    //    DFDT(2) = Vy
    //    DFDT(3) = Vz
    //    DFDT(4) = Ax
    //    DFDT(5) = Ay
    //    DFDT(6) = Az
    //
    spicelib::QDERIV(
        N,
        STATE1.as_slice(),
        STATE2.as_slice(),
        DT,
        DFDT.as_slice_mut(),
        ctx,
    )?;

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
    // d ||r||     ^
    // ------- = < r, v >
    // dt
    //
    //  2            ^          ^
    // d ||r||   < d r, v > + < r, d v >
    // ------- =   ---             ---
    //   2
    // dt          dt              dt
    //
    spicelib::DVHAT(STATE.as_slice(), SRHAT.as_slice_mut());

    DRVEL = (spicelib::VDOT(SRHAT.as_slice(), DFDT.subarray(4))
        + spicelib::VDOT(SRHAT.subarray(4), STATE.subarray(4)));
    *DECRES = (DRVEL < 0.0);

    Ok(())
}
//
//23456789012345678901234567890123456789012345678901234567890123456789012
//
