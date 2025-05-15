//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;

struct SaveVars {
    VERSN: Vec<u8>,
    IDNT66: StackArray2D<f64, 36>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VERSN = vec![b' '; 6 as usize];
        let mut IDNT66 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut PASS1: bool = false;

        PASS1 = true;
        fstr::assign(&mut VERSN, b"5.0.0");

        Self {
            VERSN,
            IDNT66,
            PASS1,
        }
    }
}

/// Frame get transformation
///
/// Find the transformation from a user specified frame to
/// another frame at a user specified epoch.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INFRM      I   The integer code for a SPICE reference frame.
///  ET         I   An epoch in seconds past J2000.
///  XFORM      O   A state transformation matrix.
///  OUTFRM     O   The frame that XFORM transforms INFRM to.
///  FOUND      O   .TRUE. if a frame transformation can be found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INFRM    is the SPICE ID-code for some reference frame.
///
///  ET       is an epoch in ephemeris seconds past J2000 at
///           which the user wishes to retrieve a state
///           transformation matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XFORM    is a 6x6 matrix that transforms states relative to
///           INFRM to states relative to OUTFRM. (Assuming such a
///           transformation can be found.)
///
///  OUTFRM   is the SPICE ID-code of a reference frame. The 6x6
///           matrix XFORM transforms states relative to INFRM to
///           states relative to OUTFRM. The state transformation
///           is achieved by multiplying XFORM on the right by a
///           state relative to INFRM. This is easily accomplished
///           via the subroutine call shown below.
///
///              CALL MXVG ( XFORM, STATE, 6, 6, OSTATE )
///
///  FOUND    is a logical flag indicating whether or not a
///           transformation matrix could be found from INFRM to
///           some other frame. If a transformation matrix cannot
///           be found OUTFRM will be set to zero, FOUND will be
///           set to .FALSE. and XFORM will be returned as the zero
///           matrix.
/// ```
///
/// # Parameters
///
/// ```text
///  See include file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a transformation matrix cannot be located, then FOUND will
///      be set to .FALSE., OUTFRM will be set to zero and XFORM will
///      be set to the zero 6x6 matrix.
///
///  2)  If the class of the requested frame is not recognized the
///      exception, the error SPICE(UNKNOWNFRAMETYPE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  LSK, SCLK, PCK, FK, SPK, and/or CK kernels may need to be loaded
///  to provide the needed frame definition and transformation data.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a low level routine used for determining a chain of state
///  transformation matrices from one frame to another.
/// ```
///
/// # Examples
///
/// ```text
///  See FRMCHG.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.0, 15-MAR-2021 (NJB) (JDR)
///
///         **Updated shadow routines ZZFRMGT0 and ZZFRMGT1, as must be
///           done each time this routine is updated.**
///
///         Support for switch frames was added. VERSN is now
///         initialized via a DATA statement. Corrected long error
///         message to use the term "class" rather than "class id-code."
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 4.0.0, 05-JAN-2014 (BVS)
///
///         To prevent operations with un-initialized DP numbers, wrapped
///         IF ( .NOT. FAILED() ) ... END IF around output matrix
///         transposition/re-assignment operations in all branches where
///         the routine returning the matrix might fail.
///
///         Added zero-ing out the output matrix in cases of a failed or
///         .NOT. FOUND transformation look ups.
///
/// -    SPICELIB Version 3.0.0, 18-DEC-2004 (NJB)
///
///         Added the new frame type 'DYN' to the list of frame
///         types recognized by FRMGET.
///
/// -    SPICELIB Version 2.0.0, 03-APR-1997 (WLT)
///
///         Added the new frame type 'TK' to the list of frame
///         types recognized by FRMGET. In addition the routine
///         now checks FAILED after "getting" the frame transformation.
///
/// -    SPICELIB Version 1.0.0, 20-OCT-1994 (WLT)
/// ```
pub fn frmget(
    ctx: &mut SpiceContext,
    infrm: i32,
    et: f64,
    xform: &mut [[f64; 6]; 6],
    outfrm: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    FRMGET(
        infrm,
        et,
        xform.as_flattened_mut(),
        outfrm,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FRMGET ( Frame get transformation )
pub fn FRMGET(
    INFRM: i32,
    ET: f64,
    XFORM: &mut [f64],
    OUTFRM: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut ROTATE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Set output flag.
    //
    *FOUND = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"FRMGET", ctx)?;

    //
    // On the first pass, initialize the identity matrix.
    //
    if save.PASS1 {
        CLEARD(36, save.IDNT66.as_slice_mut());

        for I in 1..=6 {
            save.IDNT66[[I, I]] = 1.0;
        }

        save.PASS1 = false;
    }

    //
    // Get all the needed information about this frame.
    //
    FRINFO(INFRM, &mut CENTER, &mut TYPE, &mut TYPEID, FOUND, ctx)?;

    if !*FOUND {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;

        CHKOUT(b"FRMGET", ctx)?;
        return Ok(());
    }

    //
    // FOUND was set to true by the FRINFO call. Compute transformation
    // based on the frame class.
    //
    if (TYPE == INERTL) {
        IRFROT(INFRM, 1, ROTATE.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            for I in 1..=3 {
                for J in 1..=3 {
                    XFORM[[I, J]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), (J + 3)]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), J]] = 0.0;
                    XFORM[[I, (J + 3)]] = 0.0;
                }
            }

            *OUTFRM = 1;
        }
    } else if (TYPE == PCK) {
        TISBOD(b"J2000", TYPEID, ET, TSIPM.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            INVSTM(TSIPM.as_slice(), XFORM.as_slice_mut(), ctx)?;

            *OUTFRM = 1;
        }
    } else if (TYPE == CK) {
        CKFXFM(TYPEID, ET, XFORM.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == TK) {
        TKFRAM(TYPEID, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;

        if !FAILED(ctx) {
            for I in 1..=3 {
                for J in 1..=3 {
                    XFORM[[I, J]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), (J + 3)]] = ROTATE[[I, J]];
                    XFORM[[(I + 3), J]] = 0.0;
                    XFORM[[I, (J + 3)]] = 0.0;
                }
            }
        }
    } else if (TYPE == DYN) {
        //
        // Unlike the other frame classes, the dynamic frame evaluation
        // routine ZZDYNFRM requires the input frame ID rather than the
        // dynamic frame class ID. ZZDYNFRM also requires the center ID
        // we found via the FRINFO call.

        ZZDYNFRM(INFRM, CENTER, ET, XFORM.as_slice_mut(), OUTFRM, ctx)?;
    //
    // The FOUND flag was set by FRINFO earlier; we don't touch
    // it here. If ZZDYNFRM signaled an error, FOUND will be set
    // to .FALSE. at end of this routine.
    //
    } else if (TYPE == SWTCH) {
        ZZSWFXFM(INFRM, ET, 6, XFORM.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;

        SETMSG(b"The reference frame # has class #. This form of reference frame is not supported in version # of FRMGET. You need to update your version of SPICELIB to the latest version in order to support this frame. ", ctx);

        ERRINT(b"#", INFRM, ctx);
        ERRINT(b"#", TYPE, ctx);
        ERRCH(b"#", &save.VERSN, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAMETYPE)", ctx)?;
        CHKOUT(b"FRMGET", ctx)?;
        return Ok(());
    }

    //
    // Make sure to clear outputs in case of a failure as defined in
    // in the header.
    //
    if (FAILED(ctx) || !*FOUND) {
        CLEARD(36, XFORM.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;
    }

    CHKOUT(b"FRMGET", ctx)?;
    Ok(())
}
