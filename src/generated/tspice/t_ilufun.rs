//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const CORLEN: i32 = 10;
const MTHLEN: i32 = 50;

struct SaveVars {
    SVCORR: Vec<u8>,
    SVILUM: Vec<u8>,
    SVMETH: Vec<u8>,
    SVOBS: Vec<u8>,
    SVREF: Vec<u8>,
    SVTARG: Vec<u8>,
    SVNRML: StackArray<f64, 3>,
    SVPNT: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVILUM = vec![b' '; BDNMLN as usize];
        let mut SVMETH = vec![b' '; MTHLEN as usize];
        let mut SVOBS = vec![b' '; BDNMLN as usize];
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVTARG = vec![b' '; BDNMLN as usize];
        let mut SVNRML = StackArray::<f64, 3>::new(1..=3);
        let mut SVPNT = StackArray::<f64, 3>::new(1..=3);

        Self {
            SVCORR,
            SVILUM,
            SVMETH,
            SVOBS,
            SVREF,
            SVTARG,
            SVNRML,
            SVPNT,
        }
    }
}

//$Procedure T_ILUFUN ( TSPICE illumination angle functions )
pub fn T_ILUFUN(
    METHOD: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    NORMAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let mut T_ILUFUN: f64 = 0.0;

    //
    // Entry points
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
    // Saved variables
    //

    spicelib::CHKIN(b"T_ILUFUN", ctx)?;

    T_ILUFUN = 0.0;

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_ILUFUN", ctx)?;
    Ok(T_ILUFUN)
}

//$Procedure T_ILUINI ( TSPICE: initialize entry points. )
pub fn T_ILUINI(
    METHOD: &[u8],
    TARGET: &[u8],
    ILLUM: &[u8],
    FIXREF: &[u8],
    ABCORR: &[u8],
    OBSRVR: &[u8],
    SPOINT: &[f64],
    NORMAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SPOINT = DummyArray::new(SPOINT, 1..=3);
    let NORMAL = DummyArray::new(NORMAL, 1..=3);
    let mut T_ILUINI: f64 = 0.0;

    //
    // Give this function a return value.
    //
    T_ILUINI = 0.0;

    if spicelib::RETURN(ctx) {
        return Ok(T_ILUINI);
    }

    spicelib::CHKIN(b"T_ILUINI", ctx)?;

    fstr::assign(&mut save.SVMETH, METHOD);
    fstr::assign(&mut save.SVTARG, TARGET);
    fstr::assign(&mut save.SVILUM, ILLUM);
    fstr::assign(&mut save.SVREF, FIXREF);
    fstr::assign(&mut save.SVCORR, ABCORR);
    fstr::assign(&mut save.SVOBS, OBSRVR);

    spicelib::VEQU(SPOINT.as_slice(), save.SVPNT.as_slice_mut());
    spicelib::VEQU(NORMAL.as_slice(), save.SVNRML.as_slice_mut());

    spicelib::CHKOUT(b"T_ILUINI", ctx)?;
    Ok(T_ILUINI)
}

//$Procedure T_ILUPHS ( TSPICE: Compute phase angle )
pub fn T_ILUPHS(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut T_ILUPHS: f64 = 0.0;
    let mut LOCEMI = StackArray::<f64, 2>::new(1..=2);
    let mut LOCINC = StackArray::<f64, 2>::new(1..=2);
    let mut LOCPHS = StackArray::<f64, 2>::new(1..=2);

    T_ILUPHS = 0.0;

    if spicelib::RETURN(ctx) {
        return Ok(T_ILUPHS);
    }

    spicelib::CHKIN(b"T_ILUPHS", ctx)?;

    spicelib::ZZILUSTA(
        &save.SVMETH,
        &save.SVTARG,
        &save.SVILUM,
        ET,
        &save.SVREF,
        &save.SVCORR,
        &save.SVOBS,
        save.SVPNT.as_slice(),
        save.SVNRML.as_slice(),
        LOCPHS.as_slice_mut(),
        LOCINC.as_slice_mut(),
        LOCEMI.as_slice_mut(),
        ctx,
    )?;

    T_ILUPHS = LOCPHS[1];

    spicelib::CHKOUT(b"T_ILUPHS", ctx)?;
    Ok(T_ILUPHS)
}

//$Procedure T_ILUINC ( TSPICE: Compute incidence angle )
pub fn T_ILUINC(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut T_ILUINC: f64 = 0.0;
    let mut LOCEMI = StackArray::<f64, 2>::new(1..=2);
    let mut LOCINC = StackArray::<f64, 2>::new(1..=2);
    let mut LOCPHS = StackArray::<f64, 2>::new(1..=2);

    T_ILUINC = 0.0;

    if spicelib::RETURN(ctx) {
        return Ok(T_ILUINC);
    }

    spicelib::CHKIN(b"T_ILUINC", ctx)?;

    spicelib::ZZILUSTA(
        &save.SVMETH,
        &save.SVTARG,
        &save.SVILUM,
        ET,
        &save.SVREF,
        &save.SVCORR,
        &save.SVOBS,
        save.SVPNT.as_slice(),
        save.SVNRML.as_slice(),
        LOCPHS.as_slice_mut(),
        LOCINC.as_slice_mut(),
        LOCEMI.as_slice_mut(),
        ctx,
    )?;

    T_ILUINC = LOCINC[1];

    spicelib::CHKOUT(b"T_ILUINC", ctx)?;
    Ok(T_ILUINC)
}

//$Procedure T_ILUEMI ( TSPICE: Compute emission angle )
pub fn T_ILUEMI(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut T_ILUEMI: f64 = 0.0;
    let mut LOCEMI = StackArray::<f64, 2>::new(1..=2);
    let mut LOCINC = StackArray::<f64, 2>::new(1..=2);
    let mut LOCPHS = StackArray::<f64, 2>::new(1..=2);

    T_ILUEMI = 0.0;

    if spicelib::RETURN(ctx) {
        return Ok(T_ILUEMI);
    }

    spicelib::CHKIN(b"T_ILUEMI", ctx)?;

    spicelib::ZZILUSTA(
        &save.SVMETH,
        &save.SVTARG,
        &save.SVILUM,
        ET,
        &save.SVREF,
        &save.SVCORR,
        &save.SVOBS,
        save.SVPNT.as_slice(),
        save.SVNRML.as_slice(),
        LOCPHS.as_slice_mut(),
        LOCINC.as_slice_mut(),
        LOCEMI.as_slice_mut(),
        ctx,
    )?;

    T_ILUEMI = LOCEMI[1];

    spicelib::CHKOUT(b"T_ILUEMI", ctx)?;
    Ok(T_ILUEMI)
}
