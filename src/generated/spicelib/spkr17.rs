//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const DIFF: i32 = 11;

/// Read SPK record from segment, type 17
///
/// Read a single SPK data record from a segment of type 17
/// (Equinoctial Elements).
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
///           a SPK segment of type 17.
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
///  1)  If the segment specified by DESCR is not a type 17 segment,
///      the error SPICE(WRONGSPKTYPE) is signaled.
///
///  2)  A type 17 segment should have exactly 16 values. If this
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
///  This routine reads all of the data from a type 17 SPK segment.
///
///  The structure of the data retrieved in RECORD is:
///
///      RECORD(1) is the epoch of the orbit elements at
///                in ephemeris seconds past J2000.
///
///      RECORD(2) is the semi-major axis (A) of the orbit.
///
///      RECORD(3) is the value of H at the specified epoch.
///                ( E*SIN(ARGP+NODE) ).
///
///      RECORD(4) is the value of K at the specified epoch
///                ( E*COS(ARGP+NODE) ).
///
///      RECORD(5) is the mean longitude (MEAN0+ARGP+NODE)at
///                the epoch of the elements.
///
///      RECORD(6) is the value of P (TAN(INC/2)*SIN(NODE))at
///                the specified epoch.
///
///      RECORD(7) is the value of Q (TAN(INC/2)*COS(NODE))at
///                     the specified epoch.
///
///      RECORD(8) is the rate of the longitude of periapse
///                (dARGP/dt + dNODE/dt ) at the epoch of
///                the elements. This rate is assumed to hold
///                for all time.
///
///      RECORD(9) is the derivative of the mean longitude
///                ( dM/dt + dARGP/dt + dNODE/dt ).  This
///                rate is assumed to be constant.
///
///      RECORD(10) is the rate of the longitude of the ascending
///                 node ( dNODE/dt).
///
///      RECORD(11) Right Ascension of the pole of the
///                 orbital reference system relative to the
///                 reference frame of the associated SPK segment.
///
///      RECORD(12) Declination of the pole of the
///                 orbital reference system relative to the
///                 reference frame of the associated SPK segment.
///
///  Units are km, radians and radians/second.
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
///        IF ( TYPE .EQ. 17 ) THEN
///           CALL SPKR17 ( HANDLE, DESCR, ET, RECORD )
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
///         Edited the header to comply with NAIF standard. Fixed textual
///         description of SPK type in $Abstract.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 03-JAN-1997 (WLT) (SS)
/// ```
pub fn spkr17(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR17(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR17 ( Read SPK record from segment, type 17 )
pub fn SPKR17(
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
    // The difference between the first and last address of a type 17
    // segment should be 11.
    //

    //
    // Standard Spice Error Handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR17", ctx)?;
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
    // Make sure that this really is a type 17 data segment.
    //
    if (TYPE != 17) {
        SETMSG(
            b"You are attempting to locate type 17 data in a type # data segment.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR17", ctx)?;
        return Ok(());
    }

    //
    // Since it doesn't cost much we make sure that the segment has
    // the correct amount of data.
    //
    if ((END - BEGIN) != DIFF) {
        SETMSG(b"A type 17 segment should contain exactly # double precision values.  The segment supplied had #.  The segment is badly formed. ", ctx);

        ERRINT(b"#", ((END - BEGIN) + 1), ctx);
        ERRINT(b"#", (DIFF + 1), ctx);
        SIGERR(b"SPICE(MALFORMEDSEGMENT)", ctx)?;
        CHKOUT(b"SPKR17", ctx)?;
        return Ok(());
    }

    //
    // Read the data for the record.
    //
    DAFGDA(HANDLE, BEGIN, END, RECORD.as_slice_mut(), ctx)?;
    CHKOUT(b"SPKR17", ctx)?;
    Ok(())
}
