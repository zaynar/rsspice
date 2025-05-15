//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const S18TP0: i32 = 0;
const S18TP1: i32 = (S18TP0 + 1);
const S18PS0: i32 = 12;
const S18PS1: i32 = 6;
const MAXREC: i32 = 198;
const SBTIDX: i32 = 1;
const CNTIDX: i32 = 2;
const PKTIDX: i32 = 3;
const MAXDEG: i32 = 15;

/// S/P Kernel, evaluate, type 18
///
/// Evaluate a single data record from a type 18 SPK segment.
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
///  RECORD     I   Record from a type 18 SPK segment valid for ET.
///  STATE      O   State (position and velocity) at epoch ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch for which a state vector is desired.
///
///  RECORD   is a record from a type 18 SPK segment which, when
///           evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative to
///           some center, in some inertial reference frame.
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
///              | packet 2             |
///              +----------------------+
///                       .
///                       .
///                       .
///              +----------------------+
///              | packet n             |
///              +----------------------+
///              | epochs 1--n          |
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
///  1)  This routine assumes that the input record is valid.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 18 (MEX/Rosetta Orbit
///  file interpolation) SPK segments is described in the SPK
///  Required Reading.
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
///  state vector, as in the example which follows.
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
///        IF ( TYPE .EQ. 18 ) THEN
///
///           CALL SPKR18 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE18 ( ET, RECORD, STATE )
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 16-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 05-NOV-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in XPOSEG and LGRINT calls.
///
/// -    SPICELIB Version 1.0.0, 17-AUG-2002 (NJB)
/// ```
pub fn spke18(
    ctx: &mut SpiceContext,
    et: f64,
    record: &mut [f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE18(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE18 ( S/P Kernel, evaluate, type 18 )
pub fn SPKE18(
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
    // Maximum polynomial degree:
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

    CHKIN(b"SPKE18", ctx)?;

    //
    // Capture the subtype from the record and set the packet size
    // accordingly.
    //
    SUBTYP = intrinsics::IDNINT(RECORD[SBTIDX]);

    if (SUBTYP == S18TP0) {
        PACKSZ = S18PS0;
    } else if (SUBTYP == S18TP1) {
        PACKSZ = S18PS1;
    } else {
        SETMSG(
            b"Unexpected SPK type 18 subtype found in type 18 record.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
        CHKOUT(b"SPKE18", ctx)?;
        return Ok(());
    }
    //
    // Get the packet count.
    //
    N = intrinsics::IDNINT(RECORD[CNTIDX]);

    if (SUBTYP == S18TP1) {
        //
        // This is the easy case:  we perform Lagrange interpolation
        // on each state component.
        //
        // We'll transpose the state information in the input record so
        // that contiguous pieces of it can be shoved directly into the
        // interpolation routine LGRINT.
        //
        N = intrinsics::IDNINT(RECORD[CNTIDX]);

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
    } else {
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
    }

    CHKOUT(b"SPKE18", ctx)?;
    Ok(())
}
