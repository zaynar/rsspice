//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const CTRSIZ: i32 = 2;
const RNAME: &[u8] = b"ZZSPKZP0";
const FRNMLN: i32 = 32;

struct SaveVars {
    LTCENT: f64,
    SOBS: StackArray<f64, 6>,
    POSTN: StackArray<f64, 3>,
    TEMP: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    CENTER: i32,
    FJ2000: i32,
    I: i32,
    REQFRM: i32,
    TYPE: i32,
    TYPEID: i32,
    FIRST: bool,
    FOUND: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVREQF: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LTCENT: f64 = 0.0;
        let mut SOBS = StackArray::<f64, 6>::new(1..=6);
        let mut POSTN = StackArray::<f64, 3>::new(1..=3);
        let mut TEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut CENTER: i32 = 0;
        let mut FJ2000: i32 = 0;
        let mut I: i32 = 0;
        let mut REQFRM: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut TYPEID: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVREQF: i32 = 0;

        FIRST = true;

        Self {
            LTCENT,
            SOBS,
            POSTN,
            TEMP,
            XFORM,
            CENTER,
            FJ2000,
            I,
            REQFRM,
            TYPE,
            TYPEID,
            FIRST,
            FOUND,
            XMIT,
            SVCTR1,
            SVREF,
            SVREQF,
        }
    }
}

//$Procedure ZZSPKZP0 ( S/P Kernel, easy position )
pub fn ZZSPKZP0(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    ABCORR: &[u8],
    OBS: i32,
    PTARG: &mut [f64],
    LT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PTARG = DummyArrayMut::new(PTARG, 1..=3);

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Saved frame name length.
    //

    //
    // Local variables
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(RNAME, ctx)?;
    }

    //
    // Get the frame id for J2000 on the first call to this routine.
    //
    if save.FIRST {
        NAMFRM(b"J2000", &mut save.FJ2000, ctx)?;

        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // Decide whether the aberration correction is for received or
    // transmitted radiation.
    //
    save.I = LTRIM(ABCORR);
    save.XMIT = EQCHR(fstr::substr(ABCORR, save.I..=save.I), b"X", ctx);

    //
    // If we only want geometric positions, then compute just that.
    //
    // Otherwise, compute the state of the observer relative to
    // the SSB.  Then feed that position into SPKAPO to compute the
    // apparent position of the target body relative to the observer
    // with the requested aberration corrections.
    //
    if EQSTR(ABCORR, b"NONE") {
        ZZSPKGP0(TARG, ET, REF, OBS, PTARG.as_slice_mut(), LT, ctx)?;
    } else {
        //
        // Get the auxiliary information about the requested output
        // frame.
        //
        ZZNAMFRM(
            save.SVCTR1.as_slice_mut(),
            &mut save.SVREF,
            &mut save.SVREQF,
            REF,
            &mut save.REQFRM,
            ctx,
        )?;

        if (save.REQFRM == 0) {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        FRINFO(
            save.REQFRM,
            &mut save.CENTER,
            &mut save.TYPE,
            &mut save.TYPEID,
            &mut save.FOUND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if !save.FOUND {
            SETMSG(b"The requested output frame \'#\' is not recognized by the reference frame subsystem. Please check that the appropriate kernels have been loaded and that you have correctly entered the name of the output frame. ", ctx);
            ERRCH(b"#", REF, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAME2)", ctx)?;
            CHKOUT(RNAME, ctx)?;

            return Ok(());
        }

        //
        // If we are dealing with an inertial frame, we can simply
        // call SPKSSB, SPKAPO and return.
        //
        if (save.TYPE == INERTL) {
            ZZSPKSB0(OBS, ET, REF, save.SOBS.as_slice_mut(), ctx)?;
            ZZSPKPA0(
                TARG,
                ET,
                REF,
                save.SOBS.as_slice(),
                ABCORR,
                PTARG.as_slice_mut(),
                LT,
                ctx,
            )?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Still here?
        //
        // We are dealing with a non-inertial frame.  But we need to
        // do light time and stellar aberration in an inertial frame.
        // Get the "apparent" position of TARG in the intermediary
        // inertial reference frame J2000.
        //
        // We also need the light time to the center of the frame.
        //
        ZZSPKSB0(OBS, ET, b"J2000", save.SOBS.as_slice_mut(), ctx)?;
        ZZSPKPA0(
            TARG,
            ET,
            b"J2000",
            save.SOBS.as_slice(),
            ABCORR,
            save.POSTN.as_slice_mut(),
            LT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (save.CENTER == OBS) {
            save.LTCENT = 0.0;
        } else if (save.CENTER == TARG) {
            save.LTCENT = *LT;
        } else {
            ZZSPKPA0(
                save.CENTER,
                ET,
                b"J2000",
                save.SOBS.as_slice(),
                ABCORR,
                save.TEMP.as_slice_mut(),
                &mut save.LTCENT,
                ctx,
            )?;
        }

        //
        // If something went wrong (like we couldn't get the position of
        // the center relative to the observer) now it is time to quit.
        //
        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the aberration corrections are for transmission, negate
        // the light time, since we wish to compute the orientation
        // of the non-inertial frame at an epoch later than ET by
        // the one-way light time.
        //
        if save.XMIT {
            save.LTCENT = -save.LTCENT;
        }

        //
        // Get the rotation from J2000 to the requested frame
        // and convert the position.
        //
        ZZREFCH0(
            save.FJ2000,
            save.REQFRM,
            (ET - save.LTCENT),
            save.XFORM.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        MXV(
            save.XFORM.as_slice(),
            save.POSTN.as_slice(),
            PTARG.as_slice_mut(),
        );
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
