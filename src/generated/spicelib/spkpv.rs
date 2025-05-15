//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVCTR1: StackArray<i32, 2>,
    SVREF: Vec<u8>,
    SVIRFR: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVREF = vec![b' '; FRNMLN as usize];
        let mut SVIRFR: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SVCTR1,
            SVREF,
            SVIRFR,
            FIRST,
        }
    }
}

/// S/P Kernel, position and velocity
///
/// Return the state (position and velocity) of a target body
/// relative to some center of motion in a specified frame.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  ET         I   Target epoch.
///  REF        I   Target reference frame.
///  STATE      O   Position, velocity.
///  CENTER     O   Center of state.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle assigned to a SPK file, and the
///           descriptor for a segment within the file. Together
///           they determine the ephemeris data from which the
///           state of the body is to be computed.
///
///  ET       is the epoch (ephemeris time) at which the state
///           is to be computed.
///
///  REF      is the name of the reference frame to
///           which the vectors returned by the routine should
///           be rotated. This may be any frame supported by
///           the SPICELIB subroutine FRMCHG.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    contains the position and velocity, at epoch ET,
///           for whatever body is covered by the specified segment.
///           STATE has six elements: the first three contain the
///           body's position; the last three contain the body's
///           velocity. These vectors are rotated into the
///           specified reference frame, the origin of
///           which is located at the center of motion for the
///           body (see CENTER, below).  Units are always km and
///           km/sec.
///
///  CENTER   is the integer ID code of the center of motion for
///           the state.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the requested reference frame is not supported by the
///      current version of CHGIRF, the error SPICE(SPKREFNOTSUPP)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  Once SPKPV was the most basic of the SPK readers, the reader upon
///  which SPKSSB, SPKAPP, and SPKEZ were built. However, its function
///  has now largely been replaced by SPKPVN. SPKPV should not normally
///  be called except by old software written before the release of
///  SPKPVN. This routine should be considered obsolete.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, an entire SPK file is searched
///  for segments containing a particular epoch. For each one found,
///  the body, center, segment identifier, and range at the epoch
///  are printed out.
///
///     CALL DAFOPR ( 'TEST.SPK', HANDLE )
///     CALL DAFBFS (             HANDLE )
///
///     CALL DAFFNA ( FOUND  )
///
///     DO WHILE ( FOUND )
///        CALL DAFGS ( DESCR )
///        CALL DAFUS ( DESCR, 2, 6, DC, IC )
///
///        IF ( DC(1) .LE. ET  .AND.  ET .LE. DC(2) ) THEN
///           CALL SPKPV ( HANDLE, DESCR, ET, 'J2000', STATE, CENTER )
///           CALL DAFGN ( IDENT )
///
///           WRITE (*,*)
///           WRITE (*,*) 'Body   = ', IC(1)
///           WRITE (*,*) 'Center = ', CENTER,
///           WRITE (*,*) 'ID     = ', IDENT
///           WRITE (*,*) 'Range  = ', VNORM ( STATE )
///        END IF
///
///        CALL DAFFNA ( FOUND )
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.1.1, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revision section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 6.1.0, 06-DEC-2013 (BVS) (NJB)
///
///         Updated to save the input frame name and POOL state counter
///         and to do frame name-ID conversion only if the counter has
///         changed. In-line comment regarding frame change was edited.
///
/// -    SPICELIB Version 6.0.0, 19-SEP-1995 (WLT)
///
///         The routine was updated to handle non-inertial frames.
///
/// -    SPICELIB Version 5.0.0, 13-MAR-1995 (KRG)
///
///         The routine was updated to handle type 14.
///
///         A new exception, 3, was also added.
///
/// -    SPICELIB Version 4.0.0, 04-NOV-1994 (WLT)
///
///         The routine was updated to handle type 15.
///
/// -    SPICELIB Version 3.0.0, 04-AUG-1993 (NJB)
///
///         The routine was updated to handle types 08 and 09.
///
/// -    SPICELIB Version 2.0.0, 01-APR-1992 (JML)
///
///         The routine was updated to handle type 05.
///
/// -    SPICELIB Version 1.0.2, 18-JUL-1991 (NJB)
///
///         The description of the output STATE was expanded slightly.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (RET)
/// ```
pub fn spkpv(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    ref_: &str,
    state: &mut [f64; 6],
    center: &mut i32,
) -> crate::Result<()> {
    SPKPV(
        handle,
        descr,
        et,
        ref_.as_bytes(),
        state,
        center,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKPV ( S/P Kernel, position and velocity )
pub fn SPKPV(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    REF: &[u8],
    STATE: &mut [f64],
    CENTER: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut IRF: i32 = 0;
    let mut IRFREQ: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut TSTATE = StackArray::<f64, 6>::new(1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local parameters.
    //

    //
    // Saved frame name length.
    //

    //
    // Some local space is needed in which to return records, and
    // into which to unpack the segment descriptor.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved frame name/ID items.
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKPV", ctx)?;
    }

    //
    // Initialization.
    //
    if save.FIRST {
        //
        // Initialize counter.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    *CENTER = IC[2];
    IRF = IC[3];

    //
    // Rotate the raw state from its native frame to the one requested
    // by the user only if the two frames differ.
    //
    ZZNAMFRM(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVREF,
        &mut save.SVIRFR,
        REF,
        &mut IRFREQ,
        ctx,
    )?;

    if (IRFREQ == 0) {
        SETMSG(b"No support for frame #.", ctx);
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(SPKREFNOTSUPP)", ctx)?;
    } else if (IRFREQ != IRF) {
        SPKPVN(
            HANDLE,
            DESCR.as_slice(),
            ET,
            &mut IRF,
            TSTATE.as_slice_mut(),
            CENTER,
            ctx,
        )?;
        FRMCHG(IRF, IRFREQ, ET, XFORM.as_slice_mut(), ctx)?;
        MXVG(
            XFORM.as_slice(),
            TSTATE.as_slice(),
            6,
            6,
            STATE.as_slice_mut(),
        );
    } else {
        SPKPVN(
            HANDLE,
            DESCR.as_slice(),
            ET,
            &mut IRF,
            STATE.as_slice_mut(),
            CENTER,
            ctx,
        )?;
    }

    CHKOUT(b"SPKPV", ctx)?;
    Ok(())
}
