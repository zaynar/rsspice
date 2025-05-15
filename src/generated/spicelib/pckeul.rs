//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 5;
const NR: i32 = 2;
const NS: i32 = 5;
const NT: i32 = 3;
const FILEN: i32 = 128;

/// PCK, get Euler angles at time from PCK file
///
/// This routine is obsolete. It supports only the type 02 binary
/// PCK format. It is maintained only for backward compatibility
///
/// Return Euler angles and their derivatives and their reference
/// frame, given an input time and body and reference frame from
/// a PCK binary file.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [ROTATION](crate::required_reading::rotation)
/// * [TIME](crate::required_reading::time)
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   ID code of body
///  ET         I   Epoch of transformation
///  FOUND      O   .TRUE. if ET, BODY found in a PCK file
///  REF        O   Name of inertial ref. frame of state
///  EULANG     O   Euler angles and their derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the integer ID code of the body for which the
///           state transformation matrix is requested. Bodies
///           are numbered according to the standard NAIF
///           numbering scheme. The numbering scheme is
///           explained in the NAIF_IDS required reading file.
///
///  ET       is the epoch at which the state transformation
///           matrix is requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FOUND    if the Euler angles for the requested time
///           and body are found in a PCK binary file,
///           FOUND is .TRUE. Otherwise, it's false.
///
///  REF      is the name of an inertial ref. frame.
///           (See the routine CHGIRF for a full list of names.)
///
///  EULANG   is the Euler angles and their derivatives at
///           time ET. The rotation matrix is
///           [ EULANG(3) ]  [EULANG(2)] [EULANG(1)]
///                        3            1           3
///
///           and   dEULANG(1)/dt = EULANG(4)
///                 dEULANG(2)/dt = EULANG(5)
///                 dEULANG(3)/dt = EULANG(6)
/// ```
///
/// # Examples
///
/// ```text
///  Here we load a binary PCK files and use PCKEUL to get the
///  Euler angles.
///
///  C
///  C  Load binary PCK file.
///  C
///     CALL PCKLOF ('example.pck', HANDLE)
///
///  C  Call routine to get Euler angles phi, delta, w.
///
///     CALL PCKEUL ( BODY, ET, FOUND, REF, EULANG )
///
///  The Euler angles and their derivatives are returned
///  in EULANG.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A binary PCK kernel must be loaded with PCKLOF before
///      calling this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Removed unneeded $Revisions section.
///
/// -    SPICELIB Version 2.0.0, 21-MAR-1995 (KSZ)
///
///         PCKEUL modified to check in. PCKMAT takes
///         over for PCKEUL in many cases. REF now a character.
///
/// -    SPICELIB Version 1.1.0, 18-OCT-1994 (KSZ)
///
///         Fixed bug which incorrectly modded DW by two pi.
///
/// -    SPICELIB Version 1.0.0, 11-MAR-1994 (KSZ)
/// ```
pub fn pckeul(
    ctx: &mut SpiceContext,
    body: i32,
    et: f64,
    found: &mut bool,
    ref_: &mut str,
    eulang: &mut [f64; 6],
) -> crate::Result<()> {
    PCKEUL(
        body,
        et,
        found,
        fstr::StrBytes::new(ref_).as_mut(),
        eulang,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKEUL ( PCK, get Euler angles at time from PCK file )
pub fn PCKEUL(
    BODY: i32,
    ET: f64,
    FOUND: &mut bool,
    REF: &mut [u8],
    EULANG: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut EULANG = DummyArrayMut::new(EULANG, 1..=6);
    let mut IDENT = [b' '; 40 as usize];
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut RECORD = StackArray::<f64, 130>::new(1..=130);
    let mut TYPE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut ICD = StackArray::<i32, 5>::new(1..=NI);
    let mut IREF: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Parameters
    //
    // ND    number of double precision components of descriptor
    // NI    number of integer components of descriptor
    // NR    component number of reference frame in integer
    //       portion of descriptor
    // NS    size of a packed PCK segment descriptor
    // NT    component number of data type in integer portion
    //       of descriptor
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE Error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PCKEUL", ctx)?;
    }

    //
    // Get a segment applicable to a specified body and epoch.
    //
    PCKSFS(
        BODY,
        ET,
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut IDENT,
        FOUND,
        ctx,
    )?;

    if *FOUND {
        //
        // Look at parts of the descriptor.
        //
        DAFUS(
            DESCR.as_slice(),
            ND,
            NI,
            DCD.as_slice_mut(),
            ICD.as_slice_mut(),
        );
        TYPE = ICD[NT];
        IREF = ICD[NR];
        IRFNAM(IREF, REF, ctx)?;

        if (TYPE == 2) {
            //
            // Read in Chebyshev coefficients from segment.
            //
            PCKR02(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;

            //
            // Call evaluation routine to get Euler angles
            // phi, delta, w.
            //
            PCKE02(ET, RECORD.as_slice(), EULANG.as_slice_mut(), ctx)?;
        } else {
            //
            // If appropriate data was not found, found is false.
            //
            *FOUND = false;
        }
    }

    CHKOUT(b"PCKEUL", ctx)?;
    Ok(())
}
