//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    SAVCTR: StackArray<f64, 3>,
    SAVR: f64,
    SAVX: StackArray<f64, 3>,
    SAVY: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVCTR = StackArray::<f64, 3>::new(1..=3);
        let mut SAVR: f64 = 0.0;
        let mut SAVX = StackArray::<f64, 3>::new(1..=3);
        let mut SAVY = StackArray::<f64, 3>::new(1..=3);

        SAVR = -1.0;

        Self {
            SAVCTR,
            SAVR,
            SAVX,
            SAVY,
        }
    }
}

//$Procedure T_TORUTL ( Torus utility package )
pub fn T_TORUTL(
    R: f64,
    CENTER: &[f64],
    NORMAL: &[f64],
    T: f64,
    CURVE: &[f64],
    DERIV: &[f64],
    TWIST: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    spicelib::CHKIN(b"T_TORUTL", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_TORUTL", ctx)?;
    Ok(())
}

//$Procedure T_TORCRV ( Torus curve )
pub fn T_TORCRV(
    T: f64,
    CURVE: &mut [f64],
    DERIV: &mut [f64],
    TWIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CURVE = DummyArrayMut::new(CURVE, 1..=3);
    let mut DERIV = DummyArrayMut::new(DERIV, 1..=3);
    let mut C: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut THETA: f64 = 0.0;

    //
    // Use discovery check-in.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    if ((T < 0.0) || (T > 1.0)) {
        spicelib::CHKIN(b"T_TORCRV", ctx)?;
        spicelib::SETMSG(b"Parameter T was # but must be in the range [0, 1].", ctx);
        spicelib::ERRDP(b"#", T, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_TORCRV", ctx)?;
        return Ok(());
    }

    THETA = (T * spicelib::TWOPI(ctx));

    C = (save.SAVR * f64::cos(THETA));
    S = (save.SAVR * f64::sin(THETA));

    spicelib::VLCOM3(
        C,
        save.SAVX.as_slice(),
        S,
        save.SAVY.as_slice(),
        1.0,
        save.SAVCTR.as_slice(),
        CURVE.as_slice_mut(),
    );
    spicelib::VLCOM(
        -S,
        save.SAVX.as_slice(),
        C,
        save.SAVY.as_slice(),
        DERIV.as_slice_mut(),
    );

    *TWIST = 0.0;

    Ok(())
}

//$Procedure T_TORSET ( Torus set-up )
pub fn T_TORSET(
    R: f64,
    CENTER: &[f64],
    NORMAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CENTER = DummyArray::new(CENTER, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_TORSET", ctx)?;

    save.SAVR = R;
    spicelib::VEQU(CENTER.as_slice(), save.SAVCTR.as_slice_mut());

    //
    // Generate basis vectors for the plane containing
    // the central curve of the torus.
    //
    spicelib::FRAME(
        NORMAL.as_slice_mut(),
        save.SAVX.as_slice_mut(),
        save.SAVY.as_slice_mut(),
    );

    spicelib::CHKOUT(b"T_TORSET", ctx)?;
    Ok(())
}
