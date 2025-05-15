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
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VERSN = vec![b' '; 6 as usize];

        fstr::assign(&mut VERSN, b"5.0.0");

        Self { VERSN }
    }
}

/// Frame get rotation
///
/// Find the rotation from a user specified frame to another frame at
/// a user specified epoch.
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
///  ROTATE     O   A rotation matrix.
///  OUTFRM     O   The frame that ROTATE transforms INFRM to.
///  FOUND      O   .TRUE. if a rotation can be found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INFRM    is the SPICE ID-code for some reference frame.
///
///  ET       is an epoch in ephemeris seconds past J2000 at which
///           the user wishes to retrieve a rotation matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROTATE   is a 3x3 matrix that rotates positions relative to
///           INFRM to positions relative to OUTFRM. (Assuming such
///           a rotation can be found.)
///
///  OUTFRM   is the SPICE ID-code of a reference frame. The 3x3
///           matrix ROTATE rotates positions relative to INFRM to
///           positions relative to OUTFRM. The positions
///           transformation is achieved by multiplying ROTATE on
///           the right by a position relative to INFRM. This is
///           easily accomplished via the subroutine call shown
///           below.
///
///              CALL MXV  ( ROTATE, INPOS,  OUTPOS )
///
///  FOUND    is a logical flag indicating whether or not a
///           rotation matrix could be found from INFRM to some
///           other frame. If a rotation matrix cannot be found
///           OUTFRM will be set to zero, FOUND will be set to
///           .FALSE. and ROTATE will be returned as the zero matrix.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a rotation matrix cannot be located, then FOUND will be set
///      to .FALSE., OUTFRM will be set to zero and ROTATE will be set
///      to the zero 3x3 matrix.
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
///  This is a low level routine used for determining a chain of
///  position transformation matrices from one frame to another.
/// ```
///
/// # Examples
///
/// ```text
///  See REFCHG.
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
///         **Updated shadow routines ZZROTGT0 and ZZROTGT1, as must be
///           done each time this routine is updated.**
///
///         Support for switch frames was added. VERSN is now
///         initialized via a DATA statement. Corrected long error
///         message to use the term "class" rather than "class id-code."
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 4.0.0, 21-MAR-2014 (BVS)
///
///         To prevent operations with un-initialized DP numbers, wrapped
///         IF ( .NOT. FAILED() ) ... END IF around output matrix
///         transposition operation in the PCK frame branch where the
///         routine returning the matrix might fail.
///
///         Incremented major version token by 2 to sync up versions with
///         FRMGET.
///
/// -    SPICELIB Version 2.1.0, 02-MAR-2010 (NJB)
///
///         Bug fix: frame ID rather than frame class ID
///         is now passed to dynamic frame evaluation
///         routine ZZDYNROT. Order of header sections was
///         corrected.
///
/// -    SPICELIB Version 2.0.0, 18-DEC-2004 (NJB)
///
///         Added the new frame type 'DYN' to the list of frame
///         types recognized by ROTGET.
///
/// -    SPICELIB Version 1.0.0, 03-MAR-1999 (WLT)
/// ```
pub fn rotget(
    ctx: &mut SpiceContext,
    infrm: i32,
    et: f64,
    rotate: &mut [[f64; 3]; 3],
    outfrm: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    ROTGET(
        infrm,
        et,
        rotate.as_flattened_mut(),
        outfrm,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ROTGET ( Frame get rotation )
pub fn ROTGET(
    INFRM: i32,
    ET: f64,
    ROTATE: &mut [f64],
    OUTFRM: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROTATE = DummyArrayMut2D::new(ROTATE, 1..=3, 1..=3);
    let mut CENTER: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

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

    CHKIN(b"ROTGET", ctx)?;

    //
    // Get all the needed information about this frame.
    //
    FRINFO(INFRM, &mut CENTER, &mut TYPE, &mut TYPEID, FOUND, ctx)?;

    if !*FOUND {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;

        CHKOUT(b"ROTGET", ctx)?;
        return Ok(());
    }

    //
    // FOUND was set to true by the FRINFO call. Compute rotation based
    // on the frame class.
    //
    if (TYPE == INERTL) {
        IRFROT(INFRM, 1, ROTATE.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            *OUTFRM = 1;
        }
    } else if (TYPE == PCK) {
        TIPBOD(b"J2000", TYPEID, ET, TIPM.as_slice_mut(), ctx)?;

        if !FAILED(ctx) {
            XPOSE(TIPM.as_slice(), ROTATE.as_slice_mut());

            *OUTFRM = 1;
        }
    } else if (TYPE == CK) {
        CKFROT(TYPEID, ET, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == TK) {
        TKFRAM(TYPEID, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else if (TYPE == DYN) {
        //
        // Unlike the other frame classes, the dynamic frame evaluation
        // routine ZZDYNROT requires the input frame ID rather than the
        // dynamic frame class ID. ZZDYNROT also requires the center ID
        // we found via the FRINFO call.

        ZZDYNROT(INFRM, CENTER, ET, ROTATE.as_slice_mut(), OUTFRM, ctx)?;

    //
    // The FOUND flag was set by FRINFO earlier; we don't touch
    // it here. If ZZDYNROT signaled an error, FOUND will be set
    // to .FALSE. at end of this routine.
    //
    } else if (TYPE == SWTCH) {
        ZZSWFXFM(INFRM, ET, 3, ROTATE.as_slice_mut(), OUTFRM, FOUND, ctx)?;
    } else {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;

        SETMSG(b"The reference frame # has class #. This form of reference frame is not supported in version # of ROTGET. You need to update your version of SPICELIB to the latest version in order to support this frame. ", ctx);

        ERRINT(b"#", INFRM, ctx);
        ERRINT(b"#", TYPE, ctx);
        ERRCH(b"#", &save.VERSN, ctx);
        SIGERR(b"SPICE(UNKNOWNFRAMETYPE)", ctx)?;
        CHKOUT(b"ROTGET", ctx)?;
        return Ok(());
    }

    //
    // Make sure to clear outputs in case of a failure as defined in
    // in the header.
    //
    if (FAILED(ctx) || !*FOUND) {
        CLEARD(9, ROTATE.as_slice_mut());
        *OUTFRM = 0;
        *FOUND = false;
    }

    CHKOUT(b"ROTGET", ctx)?;
    Ok(())
}
