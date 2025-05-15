//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const UBPL: i32 = 4;
const LVLTOL: f64 = 0.00000000000005;
const PLNTOL: f64 = 0.00000000000005;
const NPT: i32 = 4;

//
// Supporting function T_ISPXED
//
pub fn T_ISPXED(
    ELLIPS: &[f64],
    A: f64,
    B: f64,
    C: f64,
    PLANE: &[f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let mut T_ISPXED: bool = false;
    let mut ELPT = StackArray2D::<f64, 12>::new(1..=3, 1..=NPT);
    let mut LEVEL: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLC: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut CONST: f64 = 0.0;
    let mut PC: f64 = 0.0;

    //
    // Test whether an ellipse is the intersection of a specified
    // plane and ellipsoid.
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Scale ellipse and plane.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);
    SCLC = (C / SCALE);

    spicelib::EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSCLIP((1.0 / SCALE), CENTER.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMAJOR.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMINOR.as_slice_mut());

    //
    // Construct the endpoints of the major and minor axes.
    //
    spicelib::VADD(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        ELPT.subarray_mut([1, 1]),
    );
    spicelib::VSUB(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        ELPT.subarray_mut([1, 2]),
    );
    spicelib::VADD(
        CENTER.as_slice(),
        SMINOR.as_slice(),
        ELPT.subarray_mut([1, 3]),
    );
    spicelib::VSUB(
        CENTER.as_slice(),
        SMINOR.as_slice(),
        ELPT.subarray_mut([1, 4]),
    );
    //
    // Check the `level' of each point.
    //
    for I in 1..=NPT {
        LEVEL = (((f64::powi(ELPT[[1, I]], 2) / f64::powi(SCLA, 2))
            + (f64::powi(ELPT[[2, I]], 2) / f64::powi(SCLB, 2)))
            + (f64::powi(ELPT[[3, I]], 2) / f64::powi(SCLC, 2)));

        testutil::CHCKSD(b"LEVEL", LEVEL, b"~", 1.0, LVLTOL, OK, ctx)?;

        if !*OK {
            T_ISPXED = false;
            return Ok(T_ISPXED);
        }
    }

    //
    // Check each point to see if it's in the plane.
    //
    spicelib::PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CONST = (CONST / SCALE);

    for I in 1..=4 {
        PC = spicelib::VDOT(ELPT.subarray([1, I]), NORMAL.as_slice());

        testutil::CHCKSD(b"PC", PC, b"~/", CONST, PLNTOL, OK, ctx)?;

        if !*OK {
            T_ISPXED = false;
            return Ok(T_ISPXED);
        }
    }

    T_ISPXED = true;

    Ok(T_ISPXED)
}
