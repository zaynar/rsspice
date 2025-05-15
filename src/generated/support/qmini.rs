//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure    QMINI ( Quaternion linear interpolation )
pub fn QMINI(INIT: &[f64], FINAL: &[f64], FRAC: f64, QINTRP: &mut [f64]) {
    let INIT = DummyArray::new(INIT, 0..=3);
    let FINAL = DummyArray::new(FINAL, 0..=3);
    let mut QINTRP = DummyArrayMut::new(QINTRP, 0..=3);
    let mut ANGLE: f64 = 0.0;
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut INSTAR = StackArray::<f64, 4>::new(0..=3);
    let mut INTANG: f64 = 0.0;
    let mut Q = StackArray::<f64, 4>::new(0..=3);
    let mut QSCALE = StackArray::<f64, 4>::new(0..=3);
    let mut VMAG: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    //
    // Find the conjugate INSTAR of the input quaternion INIT.
    //
    INSTAR[0] = INIT[0];

    spicelib::VMINUS(INIT.subarray(1), INSTAR.subarray_mut(1));

    //
    // Find the quotient quaternion Q that maps INIT to FINAL.
    //
    spicelib::QXQ(FINAL.as_slice(), INSTAR.as_slice(), Q.as_slice_mut());

    //
    // Extract the rotation angle from Q. Use arccosine for
    // speed, sacrificing some accuracy.
    //
    ANGLE = (2.0 * f64::acos(spicelib::BRCKTD(Q[0], -1.0, 1.0)));

    //
    // Create a quaternion QSCALE from the rotation axis of the quotient
    // and the scaled rotation angle.
    //
    INTANG = ((FRAC * ANGLE) / 2.0);

    QSCALE[0] = f64::cos(INTANG);

    //
    // Get the unit vector parallel to the vector part of Q.
    // UNORM does exactly what we want here, because if the vector
    // part of Q is zero, the returned "unit" vector will be the
    // zero vector.
    //
    spicelib::UNORM(Q.subarray(1), AXIS.as_slice_mut(), &mut VMAG);

    //
    // Form the vector part of QSCALE.
    //
    spicelib::VSCL(f64::sin(INTANG), AXIS.as_slice(), QSCALE.subarray_mut(1));

    //
    // Apply QSCALE to INIT to produce the interpolated quaternion we
    // seek.
    //
    spicelib::QXQ(QSCALE.as_slice(), INIT.as_slice(), QINTRP.as_slice_mut());
}
