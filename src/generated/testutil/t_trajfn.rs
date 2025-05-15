//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const FRNMLN: i32 = 32;

struct SaveVars {
    T_TRAJFN: f64,
    T_GETDLT: f64,
    T_GETDSX: f64,
    T_GETDSY: f64,
    T_GETDSZ: f64,
    T_GETDX: f64,
    T_GETDY: f64,
    T_GETDZ: f64,
    T_GETLT: f64,
    T_GETSX: f64,
    T_GETSY: f64,
    T_GETSZ: f64,
    T_GETX: f64,
    T_GETY: f64,
    T_GETZ: f64,
    T_STCINI: f64,
    SVCORR: Vec<u8>,
    SVREF: Vec<u8>,
    DLT: f64,
    DPCORR: StackArray<f64, 3>,
    LT: f64,
    PCORR: StackArray<f64, 3>,
    STATE: StackArray<f64, 6>,
    STOBS: StackArray<f64, 6>,
    SVOBS: i32,
    SVTARG: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut T_TRAJFN: f64 = 0.0;
        let mut T_GETDLT: f64 = 0.0;
        let mut T_GETDSX: f64 = 0.0;
        let mut T_GETDSY: f64 = 0.0;
        let mut T_GETDSZ: f64 = 0.0;
        let mut T_GETDX: f64 = 0.0;
        let mut T_GETDY: f64 = 0.0;
        let mut T_GETDZ: f64 = 0.0;
        let mut T_GETLT: f64 = 0.0;
        let mut T_GETSX: f64 = 0.0;
        let mut T_GETSY: f64 = 0.0;
        let mut T_GETSZ: f64 = 0.0;
        let mut T_GETX: f64 = 0.0;
        let mut T_GETY: f64 = 0.0;
        let mut T_GETZ: f64 = 0.0;
        let mut T_STCINI: f64 = 0.0;
        let mut SVCORR = vec![b' '; CORLEN as usize];
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut DLT: f64 = 0.0;
        let mut DPCORR = StackArray::<f64, 3>::new(1..=3);
        let mut LT: f64 = 0.0;
        let mut PCORR = StackArray::<f64, 3>::new(1..=3);
        let mut STATE = StackArray::<f64, 6>::new(1..=6);
        let mut STOBS = StackArray::<f64, 6>::new(1..=6);
        let mut SVOBS: i32 = 0;
        let mut SVTARG: i32 = 0;

        Self {
            T_TRAJFN,
            T_GETDLT,
            T_GETDSX,
            T_GETDSY,
            T_GETDSZ,
            T_GETDX,
            T_GETDY,
            T_GETDZ,
            T_GETLT,
            T_GETSX,
            T_GETSY,
            T_GETSZ,
            T_GETX,
            T_GETY,
            T_GETZ,
            T_STCINI,
            SVCORR,
            SVREF,
            DLT,
            DPCORR,
            LT,
            PCORR,
            STATE,
            STOBS,
            SVOBS,
            SVTARG,
        }
    }
}

//$Procedure T_TRAJFN ( Test utility, evaluate trajectory functions )
pub fn T_TRAJFN(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Entry points
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

    save.T_TRAJFN = 0.0;

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    Ok(save.T_TRAJFN)
}

//$Procedure T_STCINI ( Test utility, state computation initialization )
pub fn T_STCINI(TARG: i32, REF: &[u8], ABCORR: &[u8], OBS: i32, ctx: &mut Context) -> f64 {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Set the double precision return value, since one is
    // required.
    //
    save.T_STCINI = 0.0;

    //
    // Save the input arguments for later use.
    //
    save.SVTARG = TARG;
    fstr::assign(&mut save.SVREF, REF);
    fstr::assign(&mut save.SVCORR, ABCORR);
    save.SVOBS = OBS;

    save.T_STCINI
}

//$Procedure T_GETX ( Test utility, get position X component )
pub fn T_GETX(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the X component of position.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETX = save.STATE[1];

    Ok(save.T_GETX)
}

//$Procedure T_GETY ( Test utility, get position Y component )
pub fn T_GETY(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the Y component of position.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETY = save.STATE[2];

    Ok(save.T_GETY)
}

//$Procedure T_GETZ ( Test utility, get position Z component )
pub fn T_GETZ(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the Z component of position.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETZ = save.STATE[3];

    Ok(save.T_GETZ)
}

//$Procedure T_GETDX ( Test utility, get velocity X component )
pub fn T_GETDX(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the X component of velocity.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETDX = save.STATE[4];

    Ok(save.T_GETDX)
}

//$Procedure T_GETDY ( Test utility, get velocity Y component )
pub fn T_GETDY(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the Y component of velocity.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETDY = save.STATE[5];

    Ok(save.T_GETDY)
}

//$Procedure T_GETDZ ( Test utility, get velocity Z component )
pub fn T_GETDZ(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the state. Return the Z component of velocity.
    //
    spicelib::SPKEZ(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.STATE.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    save.T_GETDZ = save.STATE[6];

    Ok(save.T_GETDZ)
}

//$Procedure T_GETSX ( Test utility, get S.A. position X component )
pub fn T_GETSX(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the X component of
    // the stellar aberration correction position offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETSX = save.PCORR[1];

    Ok(save.T_GETSX)
}

//$Procedure T_GETSY ( Test utility, get S.A. position Y component )
pub fn T_GETSY(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the Y component of
    // the stellar aberration correction position offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETSY = save.PCORR[2];
    Ok(save.T_GETSY)
}

//$Procedure T_GETSZ ( Test utility, get S.A. position Z component )
pub fn T_GETSZ(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the Z component of
    // the stellar aberration correction position offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETSZ = save.PCORR[3];

    Ok(save.T_GETSZ)
}

//$Procedure T_GETDSX ( Test utility, get S.A. velocity X component )
pub fn T_GETDSX(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the X component of
    // the stellar aberration correction velocity offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETDSX = save.DPCORR[1];

    Ok(save.T_GETDSX)
}

//$Procedure T_GETDSY ( Test utility, get S.A. velocity Y component )
pub fn T_GETDSY(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the Y component of
    // the stellar aberration correction velocity offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETDSY = save.DPCORR[2];

    Ok(save.T_GETDSY)
}

//$Procedure T_GETDSZ ( Test utility, get S.A. velocity Z component )
pub fn T_GETDSZ(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the stellar aberration correction offset PCORR and its
    // derivative with respect to time DPCORR. Return the Z component of
    // the stellar aberration correction velocity offset.
    //
    T_GETSA(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.SVOBS,
        save.PCORR.as_slice_mut(),
        save.DPCORR.as_slice_mut(),
        ctx,
    )?;
    save.T_GETDSZ = save.DPCORR[3];

    Ok(save.T_GETDSZ)
}

//$Procedure T_GETLT ( Test utility, get light time )
pub fn T_GETLT(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the light time corrected state, along with
    // the light time and the light time rate. Return the light
    // time.
    //
    spicelib::SPKSSB(save.SVOBS, ET, &save.SVREF, save.STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.STOBS.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.LT,
        &mut save.DLT,
        ctx,
    )?;
    save.T_GETLT = save.LT;

    Ok(save.T_GETLT)
}

//$Procedure T_GETDLT ( Test utility, get light time rate )
pub fn T_GETDLT(ET: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Look up the light time corrected state, along with
    // the light time and the light time rate. Return the light
    // time rate.
    //
    spicelib::SPKSSB(save.SVOBS, ET, &save.SVREF, save.STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        save.SVTARG,
        ET,
        &save.SVREF,
        &save.SVCORR,
        save.STOBS.as_slice(),
        save.STATE.as_slice_mut(),
        &mut save.LT,
        &mut save.DLT,
        ctx,
    )?;
    save.T_GETDLT = save.DLT;
    Ok(save.T_GETDLT)
}
