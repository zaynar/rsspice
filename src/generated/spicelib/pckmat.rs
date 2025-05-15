//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 5;
const NR: i32 = 2;
const NS: i32 = (ND + ((NI + 1) / 2));
const NT: i32 = 3;
const MAXREC: i32 = 130;
const NSTATE: i32 = 6;

/// PCK, get transformation matrix at time
///
/// Return the name of an inertial reference frame and the 6 x 6
/// state transformation matrix from that frame to the body fixed
/// frame of a given body at a specified epoch.
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
///  BODY       I   ID code of some body.
///  ET         I   Epoch of transformation.
///  REF        O   Integer code for inertial reference frame.
///  TSIPM      O   Transformation from Inertial to PM for BODY at ET.
///  FOUND      O   .TRUE. if data for BODY and ET are found.
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
///  REF      is the integer code for the inertial reference frame
///           of the state transformation matrix TSIPM. (See the
///           routine CHGIRF for a full list of inertial reference
///           frame names.)
///
///  TSIPM    is a 6x6 transformation matrix. It is used to
///           transform states from inertial coordinates to body
///           fixed (also called equator and prime meridian --- PM)
///           coordinates.
///
///           Given a state S in the inertial reference frame
///           specified by REF, the corresponding state in the body
///           fixed reference frame is given by the matrix vector
///           product:
///
///              TSIPM * S
///
///           See the PCK required reading for further details
///           concerning PCK reference frames.
///
///           NOTE: The inverse of TSIPM is NOT its transpose. The
///           matrix, TSIPM, has the structure shown below:
///
///                       -            -
///                      |       :      |
///                      |   R   :  0   |
///                      | ......:......|
///                      |       :      |
///                      | dR_dt :  R   |
///                      |       :      |
///                       -            -
///
///           where R is a time varying rotation matrix and dR_dt
///           is its derivative. The inverse of this matrix is:
///
///                       -              -
///                      |     T  :       |
///                      |    R   :  0    |
///                      | .......:.......|
///                      |        :       |
///                      |      T :   T   |
///                      | dR_dt  :  R    |
///                      |        :       |
///                       -              -
///
///           The SPICE routine INVSTM is available for producing
///           this inverse.
///
///   FOUND      if the data allowing the computation of a state
///           transformation matrix for the requested time and body
///           are found in a binary PCK file, FOUND will have the
///           value .TRUE., otherwise it will have the value
///           .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the size of the type 20 PCK record to be  retrieved is too
///      large to fit into RECORD, the error SPICE(PCKRECTOOLARGE)
///      is signaled.
///
///  2)  If any issue is detected while reading PCK data, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If the requested transformation matrix cannot be computed
///      using data from loaded binary PCK files, FOUND is returned
///      with the value .FALSE. This is not a SPICE error.
/// ```
///
/// # Files
///
/// ```text
///  This routine computes transformation matrices using data
///  provided by a loaded binary PCK kernel.
/// ```
///
/// # Particulars
///
/// ```text
///  The matrix for transforming an inertial state into a body fixed
///  states is the 6x6 matrix shown below as a block structured
///  matrix.
///
///              -            -
///             |       :      |
///             | TIPM  :  0   |
///             | ......:......|
///             |       :      |
///             | DTIPM : TIPM |
///             |       :      |
///              -            -
///
///  If a binary PCK file record can be found for the time and body
///  requested, it will be used. The most recently loaded binary PCK
///  file has first priority, followed by previously loaded binary PCK
///  files in backward time order. If no binary PCK file has been
///  loaded, the text P_constants kernel file is used.
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
///  C  Call routine to get transformation matrix.
///
///     CALL PCKMAT ( BODY, ET, REF, TIPM, FOUND )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 03-JAN-2014 (NJB) (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Removed unneeded $Revisions section.
///
///         Updated to support type 20. Changed long error message
///         for the case of RECORD having insufficient room: the
///         user is no longer advised to modify the record size.
///
/// -    SPICELIB Version 2.0.0, 22-MAR-1995 (KRG) (KSZ)
///
///         Added PCK type 03. Added a new exception. Made some minor
///         comment changes.
///
/// -    SPICELIB Version 1.0.0, 21-MAR-1995 (KSZ)
///
///         Replaces PCKEUL and returns the transformation
///         matrix rather than the Euler angles.
/// ```
pub fn pckmat(
    ctx: &mut SpiceContext,
    body: i32,
    et: f64,
    ref_: &mut i32,
    tsipm: &mut [[f64; 6]; 6],
    found: &mut bool,
) -> crate::Result<()> {
    PCKMAT(
        body,
        et,
        ref_,
        tsipm.as_flattened_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKMAT ( PCK, get transformation matrix at time )
pub fn PCKMAT(
    BODY: i32,
    ET: f64,
    REF: &mut i32,
    TSIPM: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TSIPM = DummyArrayMut2D::new(TSIPM, 1..=6, 1..=6);
    let mut IDENT = [b' '; 40 as usize];
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NS);
    let mut ESTATE = StackArray::<f64, 6>::new(1..=NSTATE);
    let mut EULANG = StackArray::<f64, 6>::new(1..=NSTATE);
    let mut RECORD = StackArray::<f64, 130>::new(1..=MAXREC);
    let mut HANDLE: i32 = 0;
    let mut ICD = StackArray::<i32, 5>::new(1..=NI);
    let mut RECSIZ: i32 = 0;
    let mut TYPE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // ND and NI values for a PCK file.
    //

    //
    // Index for the reference frame code in the integer summary.
    //
    //
    // Length of the descriptor for a PCK file.
    //
    //
    // Index for the data type code in the integer summary.
    //
    //
    // Maximum size allowed for a record in a segment of a binary PCK
    // file.
    //
    //
    // Number of components in a state vector.
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE Error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PCKMAT", ctx)?;

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

    if FAILED(ctx) {
        *FOUND = false;
        CHKOUT(b"PCKMAT", ctx)?;
        return Ok(());
    }

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
        *REF = ICD[NR];

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

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }
            //
            // From the PCK type two file the Euler angles are
            // retrieved in a particular order.  The routine to
            // get the TSIPM matrix from expects them in another
            // order.  Here we change from EULANG to ESTATE, which
            // has this proper order.
            //
            ESTATE[1] = EULANG[3];
            ESTATE[2] = EULANG[2];
            ESTATE[3] = EULANG[1];
            ESTATE[4] = EULANG[6];
            ESTATE[5] = EULANG[5];
            ESTATE[6] = EULANG[4];

            //
            // Call routine which takes Euler angles to transformation
            // matrix.
            //
            EUL2XF(ESTATE.as_slice(), 3, 1, 3, TSIPM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }
        } else if (TYPE == 3) {
            //
            // Fetch the number of Chebyshev coefficients, compute the
            // record size needed, and signal an error if there is not
            // enough storage in RECORD. The number of coefficients is the
            // first constant value in the generic segment.
            //
            SGFCON(HANDLE, DESCR.as_slice(), 1, 1, RECORD.subarray_mut(1), ctx)?;

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }

            RECSIZ = ((NSTATE * (RECORD[1] as i32)) + 2);

            if (RECSIZ > MAXREC) {
                SETMSG(b"Storage for # double precision numbers is needed for a PCK data record and only # locations were available. Notify the NAIF group of this problem.", ctx);
                ERRINT(b"#", RECSIZ, ctx);
                ERRINT(b"#", MAXREC, ctx);
                SIGERR(b"SPICE(PCKKRECTOOLARGE)", ctx)?;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }

            PCKR03(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
            PCKE03(ET, RECORD.as_slice(), TSIPM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }
        } else if (TYPE == 20) {
            //
            // Read in Chebyshev coefficients from segment.
            //
            PCKR20(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
            //
            // Call evaluation routine to get Euler angles
            // phi, delta, w.
            //
            PCKE20(ET, RECORD.as_slice(), EULANG.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }
            //
            // From the PCK type 20 file the Euler angles are
            // retrieved in a particular order. The routine to
            // get the TSIPM matrix from expects them in another
            // order. Here we change from EULANG to ESTATE, which
            // has this proper order.
            //
            ESTATE[1] = EULANG[3];
            ESTATE[2] = EULANG[2];
            ESTATE[3] = EULANG[1];
            ESTATE[4] = EULANG[6];
            ESTATE[5] = EULANG[5];
            ESTATE[6] = EULANG[4];

            //
            // Call routine which takes Euler angles to transformation
            // matrix.
            //
            EUL2XF(ESTATE.as_slice(), 3, 1, 3, TSIPM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                *FOUND = false;
                CHKOUT(b"PCKMAT", ctx)?;
                return Ok(());
            }
        } else {
            //
            // If data matching the requested body and time was not
            // found, FOUND is false.
            //
            *FOUND = false;
        }
    }

    CHKOUT(b"PCKMAT", ctx)?;
    Ok(())
}
