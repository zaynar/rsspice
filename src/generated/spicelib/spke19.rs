//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 27;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const S19TP0: i32 = 0;
const S19TP1: i32 = (S19TP0 + 1);
const S19TP2: i32 = (S19TP1 + 1);
const S19PS0: i32 = 12;
const S19PS1: i32 = 6;
const S19PS2: i32 = 6;
const S19NST: i32 = 3;
const S19MXZ: i32 = S19PS0;
const S19MNZ: i32 = S19PS1;
const MAXRSZ: i32 = (2 + ((MAXDEG + 1) * (S19PS1 + 1)));
const MAXREC: i32 = 198;
const SBTIDX: i32 = 1;
const CNTIDX: i32 = 2;
const PKTIDX: i32 = 3;

/// SPK, evaluate record, type 19
///
/// Evaluate a single data record from a type 19 SPK segment.
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
///  MAXREC     P   Maximum size of SPK record.
///  ET         I   Epoch for which a state is desired.
///  RECORD     I   Record from a type 19 SPK segment valid for ET.
///  STATE      O   State (position and velocity) at epoch ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch for which a state vector is desired,
///           expressed as seconds past J2000 TDB.
///
///  RECORD   is a record from a type 19 SPK segment which, when
///           evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative to
///           some center, in the reference frame associated
///           with the data packets. Usually the body, center
///           and frame are those of the SPK segment from which
///           the packets were read.
///
///           The structure of the record is as follows:
///
///              +----------------------+
///              | subtype code         |
///              +----------------------+
///              | number of packets (n)|
///              +----------------------+
///              | packet 1             |
///              +----------------------+
///                       .
///                       .
///                       .
///              +----------------------+
///              | packet n             |
///              +----------------------+
///              | epoch 1              |
///              +----------------------+
///                       .
///                       .
///                       .
///              +----------------------+
///              | epoch n              |
///              +----------------------+
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state vector at epoch ET. Its contents are, in
///           order, X, Y, Z, X', Y', and Z'. Units are km and km/sec.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXREC   is the maximum size of SPK record. See the SPICELIB
///           routine SPKPVN for details.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  Most types of errors in the input record cannot be diagnosed
///      by this routine. This routine assumes that the input record
///      is valid.
///
///  2)  If the subtype code in the input record is invalid, an error
///      is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 19 (ESOC/DDID piecewise
///  interpolation) SPK segments is described in the SPK Required
///  Reading.
/// ```
///
/// # Examples
///
/// ```text
///  The SPKEnn routines are almost always used in conjunction with
///  the corresponding SPKRnn routines, which read the records from
///  SPK files.
///
///  The data returned by the SPKRnn routine is in a raw form, taken
///  directly from the segment. As such, it will be not be directly
///  useful to a user unless they have a complete understanding of the
///  structure of the data type. Given that understanding, however,
///  the SPKRnn routines could be used to "dump" and check segment data
///  for a particular epoch before evaluating the record to obtain a
///  state vector, as in the example that follows.
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
///
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 19 ) THEN
///
///           CALL SPKR19 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE19 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the input record is valid. Any
///      checking of the input data is assumed to have been performed
///      when the source SPK file was created.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 11-MAY-2015 (NJB)
///
///         Updated to support subtype 2. Now performs
///         computations in-line, rather than calling
///         SPKE18.
///
/// -    SPICELIB Version 1.0.0, 14-MAR-2014 (NJB) (BVS)
/// ```
pub fn spke19(
    ctx: &mut SpiceContext,
    et: f64,
    record: &mut [f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE19(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE19 ( SPK, evaluate record, type 19 )
pub fn SPKE19(
    ET: f64,
    RECORD: &mut [f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut LOCREC = StackArray::<f64, 198>::new(1..=MAXREC);
    let mut WORK = ActualArray2D::<f64>::new(1..=(MAXREC * 2), 1..=2);
    let mut VBUFF = StackArray::<f64, 6>::new(1..=6);
    let mut FROM: i32 = 0;
    let mut N: i32 = 0;
    let mut PACKSZ: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut TO: i32 = 0;
    let mut XSTART: i32 = 0;
    let mut YSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Index of subtype code in record:
    //

    //
    // Index of packet count in record:
    //

    //
    // Index at which packets start:
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKE19", ctx)?;

    //
    // Capture the subtype from the record and set the packet size
    // accordingly.
    //
    SUBTYP = intrinsics::IDNINT(RECORD[SBTIDX]);

    if (SUBTYP == S19TP0) {
        PACKSZ = S19PS0;
    } else if (SUBTYP == S19TP1) {
        PACKSZ = S19PS1;
    } else if (SUBTYP == S19TP2) {
        PACKSZ = S19PS2;
    } else {
        SETMSG(
            b"Unexpected SPK type 19 subtype found in type 19 record.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKE19", ctx)?;
        return Ok(());
    }

    //
    // Get the packet count.
    //
    N = intrinsics::IDNINT(RECORD[CNTIDX]);

    if (SUBTYP == S19TP0) {
        //
        // We interpolate each state component in turn.  Position and
        // velocity are interpolated separately.
        //
        XSTART = (3 + (PACKSZ * N));

        for I in 1..=3 {
            for J in 1..=N {
                //
                // For the Jth input packet, copy the Ith position and
                // velocity components into the local record buffer LOCREC.
                //
                FROM = ((2 + (PACKSZ * (J - 1))) + I);
                TO = ((2 * J) - 1);

                LOCREC[TO] = RECORD[FROM];
                LOCREC[(TO + 1)] = RECORD[(FROM + 3)];
            }

            //
            // Interpolate the Ith position and velocity components of the
            // state.  We'll keep the position and overwrite the velocity.
            //
            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 3)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            HRMINT(
                N,
                RECORD.subarray(XSTART),
                LOCREC.as_slice(),
                ET,
                WORK.as_slice_mut(),
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // Now interpolate velocity, using separate velocity data and
        // acceleration.
        //
        for I in 1..=3 {
            for J in 1..=N {
                //
                // For the Jth input packet, copy the Ith position and
                // velocity components into the local record buffer LOCREC.
                //
                FROM = (((2 + (PACKSZ * (J - 1))) + (PACKSZ / 2)) + I);
                TO = ((2 * J) - 1);

                LOCREC[TO] = RECORD[FROM];
                LOCREC[(TO + 1)] = RECORD[(FROM + 3)];
            }

            //
            // Interpolate the Ith velocity and acceleration components of
            // the state.  We'll capture the result in a temporary buffer,
            // then transfer the velocity to the output state array.
            //
            let [arg5, arg6] = VBUFF
                .get_disjoint_mut([I, (I + 3)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            HRMINT(
                N,
                RECORD.subarray(XSTART),
                LOCREC.as_slice(),
                ET,
                WORK.as_slice_mut(),
                arg5,
                arg6,
                ctx,
            )?;
        }

        //
        // Fill in the velocity in the output state using the results of
        // interpolating velocity and acceleration.
        //
        VEQU(VBUFF.as_slice(), STATE.subarray_mut(4));
    } else if (SUBTYP == S19TP1) {
        //
        // We perform Lagrange interpolation on each state component.
        //
        // We'll transpose the state information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRINT.
        //
        XPSGIP(PACKSZ, N, RECORD.subarray_mut(PKTIDX));

        //
        // We interpolate each state component in turn.
        //
        XSTART = (3 + (N * PACKSZ));

        for I in 1..=PACKSZ {
            YSTART = (3 + (N * (I - 1)));

            STATE[I] = LGRINT(
                N,
                RECORD.subarray(XSTART),
                RECORD.subarray(YSTART),
                LOCREC.as_slice_mut(),
                ET,
                ctx,
            )?;
        }
    } else if (SUBTYP == S19TP2) {
        //
        // We perform Hermite interpolation on each position component
        // and corresponding velocity component.
        //
        XSTART = (3 + (N * PACKSZ));

        for I in 1..=3 {
            for J in 1..=N {
                //
                // For the Jth input packet, copy the Ith position and
                // velocity components into the local record buffer LOCREC.
                //
                FROM = ((2 + (PACKSZ * (J - 1))) + I);
                TO = ((2 * J) - 1);

                LOCREC[TO] = RECORD[FROM];
                LOCREC[(TO + 1)] = RECORD[(FROM + 3)];
            }

            //
            // Interpolate the Ith position and velocity components of the
            // state.
            //
            let [arg5, arg6] = STATE
                .get_disjoint_mut([I, (I + 3)])
                .expect("mutable array elements passed to function must have disjoint indexes");
            HRMINT(
                N,
                RECORD.subarray(XSTART),
                LOCREC.as_slice(),
                ET,
                WORK.as_slice_mut(),
                arg5,
                arg6,
                ctx,
            )?;
        }
    } else {
        //
        // This is a backstop case.
        //
        SETMSG(
            b"Unexpected SPK type 19 subtype found in type 19 record.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKE19", ctx)?;
        return Ok(());
    }

    CHKOUT(b"SPKE19", ctx)?;
    Ok(())
}
