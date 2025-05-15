//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXREC: i32 = 198;
const NSTATE: i32 = 6;

/// S/P Kernel, position and velocity in native frame
///
/// Return, for a specified SPK segment and time, the state (position
/// and velocity) of the segment's target body relative to its center
/// of motion.
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
///  REF        O   Target reference frame.
///  STATE      O   Position, velocity.
///  CENTER     O   Center of state.
///  MAXREC     P   Maximum length of records returned by SPKRnn.
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
/// ```
///
/// # Detailed Output
///
/// ```text
///  REF      is the id-code of the reference frame to
///           which the vectors returned by the routine belong.
///
///  STATE    is a 6-dimensional vector that contains the position and
///           velocity, at epoch ET, for whatever body is covered by
///           the specified segment. STATE has six elements: the first
///           three contain the body's position; the last three contain
///           the body's velocity. These vectors are rotated into the
///           specified  reference frame, the origin of which is
///           located at the center of motion for the body (see CENTER,
///           below). Units are always km and km/sec.
///
///  CENTER   is the integer ID code of the center of motion for
///           the state.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXREC   is the maximum length of a record returned by any of
///           data type-specific routines SPKRnn, which are called
///           by SPKPVN (see $Particulars).
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment type is not supported by the current
///      version of SPKPVN, the error SPICE(SPKTYPENOTSUPP)
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
///  SPKPVN is the most basic of the SPK readers, the reader upon
///  which SPKPV and SPKGEO, etc. are built. It should not normally
///  be called directly except in cases where some optimization is
///  required. (That is, where the calling program has prior knowledge
///  of the center-barycenter shifts to be performed, or a non-standard
///  method of determining the files and segments to be used when
///  computing states.)
///
///  This is the only reader which makes distinctions between the
///  various segment types in the SPK format. The complete list
///  of types currently supported is shown below.
///
///     Type   Description
///     ----   -----------------------
///        1   Difference Lines
///        2   Chebyshev (P)
///        3   Chebyshev (P,V)
///        5   Two body propagation between discrete states
///        8   Lagrange interpolation, equally spaced discrete states
///        9   Lagrange interpolation, unequally spaced discrete states
///       12   Hermite interpolation, equally spaced discrete states
///       13   Hermite interpolation, unequally spaced discrete states
///       14   Chebyshev Unequally spaced
///       15   Precessing Ellipse
///       17   Equinoctial Elements
///       18   ESOC/DDID Hermite/Lagrange Interpolation
///       19   ESOC/DDID Piecewise Interpolation
///       20   Chebyshev (V)
///       21   Extended Modified Difference Array
///
///  SPKPVN is the only reader that needs to be changed in order to
///  add a new segment type to the SPK format. If a new data type is
///  added, the following steps should be taken:
///
///  1) Write two new routines, SPKRnn and SPKEnn, to read and
///     evaluate, respectively, a record from a data type nn segment.
///
///  2) Insert a new case into the body of SPKPVN to accommodate the
///     new type.
///
///  3) If necessary, adjust the parameter MAXREC, above, so that it
///     is large enough to encompass the maximum size of a record
///     returned by SPKRnn and passed to SPKEnn.
///
///     The maximum record lengths for each data type currently
///     supported are as follows:
///
///               Data type       Maximum record length
///               ---------       ---------------------
///                   1                    71
///                   2                    87
///                   3                   171
///                   5                    15
///                   8                   171
///                   9                   197
///                  12                    87
///                  13                    99
///                  14                 Variable
///                  15                    16
///                  17                    12
///                  18                   198
///                  19                   198
///                  20                   159
///                  21                   112
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
///
///        CALL DAFGS ( DESCR )
///        CALL DAFUS ( DESCR, 2, 6, DC, IC )
///
///        IF ( DC(1) .LE. ET  .AND.  ET .LE. DC(2) ) THEN
///           CALL SPKPVN ( HANDLE, DESCR, ET, REF, STATE, CENTER )
///           CALL DAFGN  ( IDENT )
///           CALL FRMNAM ( REF, FRAME )
///           WRITE (*,*)
///           WRITE (*,*) 'Body   = ', IC(1)
///           WRITE (*,*) 'Center = ', CENTER,
///           WRITE (*,*) 'ID     = ', IDENT
///           WRITE (*,*) 'Frame  = ', FRAME
///           WRITE (*,*) 'Range  = ', VNORM ( STATE )
///        END IF
///
///        CALL DAFFNA ( FOUND )
///
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.0.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Improved $Abstract section.
///
/// -    SPICELIB Version 4.0.0, 23-DEC-2013 (NJB)
///
///         Added support for types 19, 20 and 21. Added header
///         comments giving description for types 18, 19,
///         and 21. Removed header reference to type 4.
///
/// -    SPICELIB Version 3.0.0, 16-AUG-2002 (NJB)
///
///         Added support for type 18. This routine now uses the
///         include file spkrec.inc to declare the record size.
///
///         Corrected header comments giving record sizes for types
///         8, 9, 12, 13.
///
/// -    SPICELIB Version 2.0.0, 06-NOV-1999 (NJB)
///
///         Added support for types 12 and 13.
///
/// -    SPICELIB Version 1.1.0, 07-JAN-1997 (WLT)
///
///         Added support for type 17.
///
/// -    SPICELIB Version 1.0.0, 19-SEP-1995 (WLT)
/// ```
pub fn spkpvn(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    ref_: &mut i32,
    state: &mut [f64; 6],
    center: &mut i32,
) -> crate::Result<()> {
    SPKPVN(handle, descr, et, ref_, state, center, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKPVN ( S/P Kernel, position and velocity in native frame )
pub fn SPKPVN(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    REF: &mut i32,
    STATE: &mut [f64],
    CENTER: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut RECORD = StackArray::<f64, 198>::new(1..=MAXREC);
    let mut DC = StackArray::<f64, 2>::new(1..=2);
    let mut IC = StackArray::<i32, 6>::new(1..=6);
    let mut RECSIZ: i32 = 0;
    let mut TYPE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Some local space is needed in which to return records, and
    // into which to unpack the segment descriptor.
    //

    //
    // Local Parameters
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKPVN", ctx)?;
    }

    //
    // Unpacking the segment descriptor will tell us the center,
    // reference frame, and data type for this segment.
    //
    DAFUS(DESCR.as_slice(), 2, 6, DC.as_slice_mut(), IC.as_slice_mut());

    *CENTER = IC[2];
    *REF = IC[3];
    TYPE = IC[4];

    //
    // Each data type has a pair of routines to read and evaluate
    // records for that data type. These routines are the only ones
    // that actually look inside the segments.
    //
    // By the time we have more than 100 data types, we should be
    // allowed to use longer variable names.
    //
    if (TYPE == 1) {
        SPKR01(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE01(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx);
    } else if (TYPE == 2) {
        SPKR02(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE02(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 3) {
        SPKR03(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE03(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;

    //
    // Type 04 is not officially part of the library.
    //
    // ELSE IF ( TYPE .EQ. 04 ) THEN
    //     CALL SPKR04 ( HANDLE, DESCR, ET, RECORD         )
    //     CALL SPKE04 (                ET, RECORD, STATE  )
    } else if (TYPE == 5) {
        SPKR05(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE05(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 8) {
        SPKR08(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE08(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 9) {
        SPKR09(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE09(ET, RECORD.as_slice_mut(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 10) {
        SPKR10(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE10(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 12) {
        SPKR12(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE12(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 13) {
        SPKR13(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE13(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 14) {
        //
        // Fetch the number of Chebyshev coefficients, compute the record
        // size needed, and signal an error if there is not enough storage
        // in RECORD. The number of coefficients is the first constant
        // value in the generic segment.
        //
        SGFCON(HANDLE, DESCR.as_slice(), 1, 1, RECORD.subarray_mut(1), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKPVN", ctx)?;
            return Ok(());
        }

        RECSIZ = ((NSTATE * (RECORD[1] as i32)) + 3);

        if (RECSIZ > MAXREC) {
            SETMSG(b"Storage for # double precision numbers is needed for an SPK data record and only # locations were available. Update the parameter MAXREC in the subroutine SPKPVN and notify the NAIF group of this problem.", ctx);
            ERRINT(b"#", RECSIZ, ctx);
            ERRINT(b"#", MAXREC, ctx);
            SIGERR(b"SPICE(SPKRECTOOLARGE)", ctx)?;
            CHKOUT(b"SPKPVN", ctx)?;
            return Ok(());
        }

        SPKR14(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE14(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 15) {
        SPKR15(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE15(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 17) {
        SPKR17(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE17(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 18) {
        SPKR18(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE18(ET, RECORD.as_slice_mut(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 19) {
        SPKR19(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE19(ET, RECORD.as_slice_mut(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 20) {
        SPKR20(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE20(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else if (TYPE == 21) {
        SPKR21(HANDLE, DESCR.as_slice(), ET, RECORD.as_slice_mut(), ctx)?;
        SPKE21(ET, RECORD.as_slice(), STATE.as_slice_mut(), ctx)?;
    } else {
        SETMSG(b"SPK type # is not supported in your version of the SPICE library.  You will need to upgrade your version of the library to make use of ephemerides that contain this SPK data type. ", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(SPKTYPENOTSUPP)", ctx)?;
        CHKOUT(b"SPKPVN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SPKPVN", ctx)?;
    Ok(())
}
