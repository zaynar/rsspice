//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const DIFF: i32 = 15;

/// Read SPK record from segment, type 15
///
/// Read a single SPK data record from a segment of type 15
/// (Precessing Conic Propagation).
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
///  RECORD     O   Data record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           a SPK segment of type 15.
///
///  ET       is a target epoch, for which a data record from
///           a specific segment is required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment specified by DESCR is not a type 15 segment,
///      the error SPICE(WRONGSPKTYPE) is signaled.
///
///  2)  A type 15 segment should have exactly 16 values. If this
///      is not the case, the error SPICE(MALFORMEDSEGMENT) is
///      signaled.
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
///  This routine reads all of the data from a type 15 SPK segment.
///
///  The structure of the data retrieved in RECORD is:
///
///      RECORD(1)             epoch of the orbit elements at periapse
///                            in ephemeris seconds past J2000.
///      RECORD(2)-RECORD(4)   unit trajectory pole vector
///      RECORD(5)-RECORD(7)   unit periapsis vector
///      RECORD(8)             semi-latus rectum---p in the
///                            equation:
///
///                            r = p/(1 + ECC*COS(Nu))
///
///      RECORD(9)             eccentricity
///      RECORD(10)            J2 processing flag describing
///                            what J2 corrections are to be
///                            applied when the orbit is
///                            propagated.
///
///                             Value       Meaning
///                             -----  -----------------------------
///                             1      Regress line of nodes only.
///                             2      Precess line of apsides only.
///                             3      Don't use J2 corrections.
///                             Other  Regress line of nodes
///                                    and precess line of apsides.
///
///      RECORD(11)-RECORD(13) unit central body pole vector
///      RECORD(14)            central body GM
///      RECORD(15)            central body J2
///      RECORD(16)            central body radius
///
///  Except for J2, units are radians, km, seconds.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRnn
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 15 ) THEN
///           CALL SPKR15 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  S. Schlaifer       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 15-NOV-1994 (WLT) (SS)
/// ```
pub fn spkr15(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR15(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR15 ( Read SPK record from segment, type 15 )
pub fn SPKR15(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut TYPE: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // The difference between the first and last address of a type 15
    // segment should be 15.
    //

    //
    // Standard Spice Error Handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR15", ctx)?;
    //
    // Unpack the segment descriptor.
    //
    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );

    TYPE = IC[4];
    BEGIN = IC[5];
    END = IC[6];
    //
    // Make sure that this really is a type 15 data segment.
    //
    if (TYPE != 15) {
        SETMSG(
            b"You are attempting to locate type 15 data in a type # data segment.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR15", ctx)?;
        return Ok(());
    }

    //
    // Since it doesn't cost much we make sure that the segment has
    // the correct amount of data.
    //
    if ((END - BEGIN) != DIFF) {
        SETMSG(b"A type 15 segment should contain exactly 16 double precision values.  The segment supplied had #.  The segment is badly formed. ", ctx);

        ERRINT(b"#", ((END - BEGIN) + 1), ctx);
        SIGERR(b"SPICE(MALFORMEDSEGMENT)", ctx)?;
        CHKOUT(b"SPKR15", ctx)?;
        return Ok(());
    }

    //
    // Read the data for the record.
    //
    DAFGDA(HANDLE, BEGIN, END, RECORD.as_slice_mut(), ctx)?;
    CHKOUT(b"SPKR15", ctx)?;
    Ok(())
}
