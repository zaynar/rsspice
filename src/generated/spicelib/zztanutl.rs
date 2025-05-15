//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;

struct SaveVars {
    SVAXIS: StackArray<f64, 3>,
    SVET: f64,
    SVIRAD: f64,
    SVNRML: StackArray<f64, 3>,
    SVVRTX: StackArray<f64, 3>,
    SVCURV: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVAXIS = StackArray::<f64, 3>::new(1..=3);
        let mut SVET: f64 = 0.0;
        let mut SVIRAD: f64 = 0.0;
        let mut SVNRML = StackArray::<f64, 3>::new(1..=3);
        let mut SVVRTX = StackArray::<f64, 3>::new(1..=3);
        let mut SVCURV: i32 = 0;

        Self {
            SVAXIS,
            SVET,
            SVIRAD,
            SVNRML,
            SVVRTX,
            SVCURV,
        }
    }
}

//$Procedure ZZTANUTL ( DSK, tangent ray utilities )
pub fn ZZTANUTL(
    CURVE: i32,
    SRCRAD: f64,
    SHAPE: i32,
    TRGCDE: i32,
    NSURF: i32,
    SRFLST: &[i32],
    FIXFID: i32,
    ET: f64,
    PLNVEC: &[f64],
    AXIS: &[f64],
    ANGLE: f64,
    OCULTD: bool,
    POINT: &[f64],
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

    CHKIN(b"ZZTANUTL", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZTANUTL", ctx)?;
    Ok(())
}

//$Procedure ZZTANINI ( DSK, tangent utility initialization )
pub fn ZZTANINI(
    CURVE: i32,
    SRCRAD: f64,
    SHAPE: i32,
    TRGCDE: i32,
    NSURF: i32,
    SRFLST: &[i32],
    FIXFID: i32,
    ET: f64,
    PLNVEC: &[f64],
    AXIS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let PLNVEC = DummyArray::new(PLNVEC, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTANINI", ctx)?;

    //
    // Check for zero vectors on input.
    //
    if VZERO(AXIS.as_slice()) {
        SETMSG(b"Input axis vector is the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZTANINI", ctx)?;
        return Ok(());
    }

    if VZERO(PLNVEC.as_slice()) {
        SETMSG(b"Input reference vector is the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZTANINI", ctx)?;
        return Ok(());
    }

    //
    // Save the curve type.
    //
    if (((CURVE != LMBCRV) && (CURVE != UMBRAL)) && (CURVE != PNMBRL)) {
        SETMSG(b"Curve type code # was not recognized.", ctx);
        ERRINT(b"#", CURVE, ctx);
        SIGERR(b"SPICE(BADCURVETYPE)", ctx)?;
        CHKOUT(b"ZZTANINI", ctx)?;
        return Ok(());
    }

    save.SVCURV = CURVE;

    //
    // Save the illumination source radius.
    //
    if ((CURVE == UMBRAL) || (CURVE == PNMBRL)) {
        if (SRCRAD <= 0.0) {
            SETMSG(b"The source radius was #. The radius must be positive for a terminator computation.", ctx);
            ERRDP(b"#", SRCRAD, ctx);
            SIGERR(b"SPICE(BADSOURCERADIUS)", ctx)?;
            CHKOUT(b"ZZTANINI", ctx)?;
            return Ok(());
        }
    }

    save.SVIRAD = SRCRAD;

    //
    // Compute a normal vector to the plane defined by
    // AXIS and PLNVEC. The direction of positive rotation
    // about the normal is from AXIS toward PLNVEC.
    //
    VCRSS(
        AXIS.as_slice(),
        PLNVEC.as_slice(),
        save.SVNRML.as_slice_mut(),
    );

    if VZERO(save.SVNRML.as_slice()) {
        SETMSG(
            b"Input reference vector and axis vector are linearly dependent.",
            ctx,
        );
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZTANINI", ctx)?;
        return Ok(());
    }

    //
    // Scale the normal vector to unit length.
    //
    VHATIP(save.SVNRML.as_slice_mut());

    //
    // Save a unit-length copy of the input axis.
    // Save the original axis as the ray's vertex; this
    // will be used directly in the limb computation and
    // will be used, after addition of an offset, in the
    // terminator computations. Save the evaluation epoch.
    //
    VEQU(AXIS.as_slice(), save.SVVRTX.as_slice_mut());
    VHAT(AXIS.as_slice(), save.SVAXIS.as_slice_mut());

    save.SVET = ET;
    //
    // Prepare the DSK SINCPT utilities for a computation with
    // the input surface set.
    //
    if (SHAPE == ELLSHP) {
        //
        // This is the ellipsoid case.
        //
        ZZSUELIN(TRGCDE, ctx)?;
    } else if (SHAPE == DSKSHP) {
        //
        // This is the DSK case.
        //
        ZZSUDSKI(TRGCDE, NSURF, SRFLST.as_slice(), FIXFID, ctx)?;
    } else {
        SETMSG(b"Target shape code # was not recognized.", ctx);
        ERRINT(b"#", SHAPE, ctx);
        SIGERR(b"SPICE(BADSHAPE)", ctx)?;
        CHKOUT(b"ZZTANINI", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZTANINI", ctx)?;
    Ok(())
}

//$Procedure ZZTANSTA ( DSK, tangent ray state )
pub fn ZZTANSTA(
    ANGLE: f64,
    OCULTD: &mut bool,
    POINT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut POINT = DummyArrayMut::new(POINT, 1..=3);
    let mut APEX = StackArray::<f64, 3>::new(1..=3);
    let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
    let mut VRTOFF = StackArray::<f64, 3>::new(1..=3);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTANSTA", ctx)?;

    if (save.SVCURV == LMBCRV) {
        //
        // This is the limb case.
        //
        // We'll rotate SVAXIS by ANGLE to achieve the desired result.
        //
        VROTV(
            save.SVAXIS.as_slice(),
            save.SVNRML.as_slice(),
            ANGLE,
            RAYDIR.as_slice_mut(),
        );

        ZZRAYSFX(
            save.SVVRTX.as_slice(),
            RAYDIR.as_slice(),
            save.SVET,
            POINT.as_slice_mut(),
            OCULTD,
            ctx,
        )?;
    } else if (save.SVCURV == UMBRAL) {
        //
        // This is the umbral terminator case.
        //
        // Produce the ray's direction vector by rotating
        // the axis about the cutting half-plane normal by
        // the input angle.
        //
        VROTV(
            save.SVAXIS.as_slice(),
            save.SVNRML.as_slice(),
            ANGLE,
            RAYDIR.as_slice_mut(),
        );
        //
        // Produce the offset of the ray's vertex from the
        // center of the source by rotating the axis
        // vector by ANGLE-pi/2 radians. The length
        // of the vector must be SVIRAD. The saved axis
        // has unit length.
        //
        VROTV(
            save.SVAXIS.as_slice(),
            save.SVNRML.as_slice(),
            (ANGLE - (PI(ctx) / 2 as f64)),
            VRTOFF.as_slice_mut(),
        );
        VSCLIP(save.SVIRAD, VRTOFF.as_slice_mut());
        VADD(
            save.SVVRTX.as_slice(),
            VRTOFF.as_slice(),
            APEX.as_slice_mut(),
        );

        ZZRAYSFX(
            APEX.as_slice(),
            RAYDIR.as_slice(),
            save.SVET,
            POINT.as_slice_mut(),
            OCULTD,
            ctx,
        )?;
    } else if (save.SVCURV == PNMBRL) {
        //
        // This is the penumbral terminator case.
        //
        // Produce the ray's direction vector by rotating
        // the axis about the cutting half-plane normal by
        // the *negative* of the input angle.
        //
        VROTV(
            save.SVAXIS.as_slice(),
            save.SVNRML.as_slice(),
            -ANGLE,
            RAYDIR.as_slice_mut(),
        );
        //
        // Produce the ray's vertex by rotating the axis
        // vector about the normal, *not its negative,*
        // by 3*pi/2 - ANGLE radians. The length of the vector
        // must be SRCRAD. The saved axis has unit length.
        //
        VROTV(
            save.SVAXIS.as_slice(),
            save.SVNRML.as_slice(),
            ((1.5 * PI(ctx)) - ANGLE),
            VRTOFF.as_slice_mut(),
        );

        VSCLIP(save.SVIRAD, VRTOFF.as_slice_mut());

        VADD(
            save.SVVRTX.as_slice(),
            VRTOFF.as_slice(),
            APEX.as_slice_mut(),
        );

        ZZRAYSFX(
            APEX.as_slice(),
            RAYDIR.as_slice(),
            save.SVET,
            POINT.as_slice_mut(),
            OCULTD,
            ctx,
        )?;
    } else {
        //
        // This case should have been ruled out by a check in
        // ZZTANINI. Check again anyway.
        //
        SETMSG(b"Bad curve type code #.", ctx);
        ERRINT(b"#", save.SVCURV, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZTANSTA", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZTANSTA", ctx)?;
    Ok(())
}
