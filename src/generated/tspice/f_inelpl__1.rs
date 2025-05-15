//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const UBPL: i32 = 4;
const LVLTOL: f64 = 0.000000000001;
const PLNTOL: f64 = 0.00000000001;
const MSGLEN: i32 = 240;

//
// Supporting function T_ISPXEL
//
pub fn T_ISPXEL(
    ELLIPS: &[f64],
    PLANE: &[f64],
    N: i32,
    XPTS: &[f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let XPTS = DummyArray2D::new(XPTS, 1..=3, 1..=2);
    let mut T_ISPXEL: bool = false;
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut C: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CONST: f64 = 0.0;
    let mut LEVEL: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSET = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut PC: f64 = 0.0;
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLX = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut XAX = StackArray2D::<f64, 6>::new(1..=3, 1..=2);

    //
    // Test whether a set of points constitute the intersection of a
    // specified plane and ellipse.
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
    // Give the function an initial return value.
    //
    T_ISPXEL = false;

    //
    // Scale ellipse and plane.
    //
    spicelib::EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    A = spicelib::VNORM(SMAJOR.as_slice());
    B = spicelib::VNORM(SMINOR.as_slice());
    C = spicelib::VNORM(CENTER.as_slice());

    spicelib::PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if !*OK {
        return Ok(T_ISPXEL);
    }

    //
    // Scale the ellipse and plane constant for safer computation.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C), f64::abs(CONST)]);

    spicelib::VSCLIP((1.0 / SCALE), CENTER.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMAJOR.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMINOR.as_slice_mut());

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);

    CONST = (CONST / SCALE);

    //
    // Scale the input points.
    //
    for I in 1..=N {
        spicelib::VSCL(
            (1.0 / SCALE),
            XPTS.subarray([1, I]),
            SCLX.subarray_mut([1, I]),
        );
    }

    //
    // Subtract the scaled ellipse center from the intersection
    // points.
    //
    for I in 1..=N {
        spicelib::VSUB(
            SCLX.subarray([1, I]),
            CENTER.as_slice(),
            OFFSET.subarray_mut([1, I]),
        );
    }

    //
    // Transform the input points into the semi-axis frame.
    //
    spicelib::TWOVEC(
        SMAJOR.as_slice(),
        1,
        SMINOR.as_slice(),
        2,
        RMAT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=N {
        spicelib::MXV(
            RMAT.as_slice(),
            OFFSET.subarray([1, I]),
            XAX.subarray_mut([1, I]),
        );
    }

    //
    // Check the `level' of each point.
    //
    for I in 1..=N {
        LEVEL = ((f64::powi(XAX[[1, I]], 2) / f64::powi(SCLA, 2))
            + (f64::powi(XAX[[2, I]], 2) / f64::powi(SCLB, 2)));

        fstr::assign(&mut TITLE, b"Level of point #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::CHCKSD(&TITLE, LEVEL, b"~", 1.0, LVLTOL, OK, ctx)?;

        if !*OK {
            T_ISPXEL = false;
            return Ok(T_ISPXEL);
        }
    }

    //
    // Check each point to see if it's in the plane.
    //
    spicelib::PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    CONST = (CONST / SCALE);

    for I in 1..=N {
        PC = spicelib::VDOT(SCLX.subarray([1, I]), NORMAL.as_slice());

        fstr::assign(&mut TITLE, b"Plane constant of point #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);

        testutil::CHCKSD(&TITLE, PC, b"~/", CONST, PLNTOL, OK, ctx)?;

        if !*OK {
            T_ISPXEL = false;
            return Ok(T_ISPXEL);
        }
    }

    T_ISPXEL = true;

    Ok(T_ISPXEL)
}
