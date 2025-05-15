//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;

/// C-kernel, read pointing record, data type 4
///
/// Read a single data record from a type 4 CK segment.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  SCLKDP     I   Pointing request time.
///  TOL        I   Time tolerance.
///  NEEDAV     I   Angular velocity request flag.
///  RECORD     O   Pointing data record.
///  FOUND      O   .TRUE. when a record covering SCLKDP is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the integer handle of the CK file containing the
///           segment.
///
///  DESCR    is the descriptor of the segment.
///
///  SCLKDP   is the encoded spacecraft clock time for which
///           pointing is being requested.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock.
///
///           When SCLKDP falls within the bounds of one of the
///           interpolation intervals then the tolerance has no
///           effect because pointing will be returned at the
///           request time.
///
///           However, if the request time is not in one of the
///           intervals, then the tolerance is used to determine
///           if pointing at one of the interval endpoints should
///           be returned.
///
///  NEEDAV   is .TRUE. if angular velocity is requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record that CKE04 will evaluate to determine
///           the pointing and it includes parameters:
///
///           ---------------------------------------------------
///           |    Encoded onboard time which is the closest    |
///           |  to SCLKDP and belongs to one of approximation  |
///           |                   intervals                     |
///           ---------------------------------------------------
///           |       encoded SCLK time of the midpoint of      |
///           |             interpolation interval              |
///           ---------------------------------------------------
///           |          radii of interpolation interval        |
///           |    expressed as double precision SCLK ticks     |
///           ---------------------------------------------------
///           |         Number of coefficients for q0           |
///           ---------------------------------------------------
///           |         Number of coefficients for q1           |
///           ---------------------------------------------------
///           |         Number of coefficients for q2           |
///           ---------------------------------------------------
///           |         Number of coefficients for q3           |
///           ---------------------------------------------------
///           |         Number of coefficients for AV1          |
///           ---------------------------------------------------
///           |         Number of coefficients for AV2          |
///           ---------------------------------------------------
///           |         Number of coefficients for AV3          |
///           ---------------------------------------------------
///           |               q0 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q1 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q2 Cheby coefficients             |
///           ---------------------------------------------------
///           |               q3 Cheby coefficients             |
///           ---------------------------------------------------
///           |         AV1 Cheby coefficients (optional)       |
///           ---------------------------------------------------
///           |         AV2 Cheby coefficients (optional)       |
///           ---------------------------------------------------
///           |         AV3 Cheby coefficients (optional)       |
///           ---------------------------------------------------
///
///  FOUND    is .TRUE. if a record was found to satisfy the pointing
///           request. This occurs when the time for which pointing
///           is requested falls inside one of the interpolation
///           intervals, or when the request time is within the
///           tolerance of an interval endpoint.
/// ```
///
/// # Parameters
///
/// ```text
///  See 'ckparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to an open DAF file,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the specified descriptor does not belong to a segment data
///      format organized in accordance with generic segment
///      architecture, an error is signaled by a routine in the call
///      tree of this routine.
///
///  3)  If DESCR is not a valid descriptor of a segment in the CK
///      file specified by HANDLE, the results of this routine are
///      unpredictable.
///
///  4)  If the segment is not of data type 4, as specified in the
///      third integer component of the segment descriptor,
///      the error SPICE(WRONGDATATYPE) is signaled.
///
///  5)  If angular velocity data was requested but the segment
///      contains no such data, the error SPICE(NOAVDATA) is
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
///  See the CK Required Reading file for a detailed description of
///  the structure of a type 4 pointing segment.
///
///  When the time for which pointing was requested falls within an
///  interpolation interval, then FOUND will be true and RECORD will
///  contain the set of Chebyshev polynomial coefficients for the
///  time interval that brackets the request time. CKE04 will
///  evaluate RECORD to give pointing at the request time.
///
///  However, when the request time is not within any of the
///  interpolation intervals, then FOUND will be true only if the
///  interval endpoint closest to the request time is within the
///  tolerance specified by the user. In this case RECORD will
///  contain the set of Chebyshev polynomial coefficients for the
///  time interval one of the ends of which was within tolerance
///  from the request time, and CKE04 will evaluate RECORD to give
///  pointing at the time associated with that interval end time.
/// ```
///
/// # Examples
///
/// ```text
///  The CKRnn routines are usually used in tandem with the CKEnn
///  routines, which evaluate the record returned by CKRnn to give
///  the pointing information and output time.
///
///  The following code fragment searches backwards through all of the
///  segments in a file applicable to the Mars Global Surveyor
///  spacecraft bus that are of data type 4, for a particular
///  spacecraft clock time. It then evaluates the pointing for that
///  epoch and prints the result.
///
///  The search performed here does not mimic the behavior of the CK
///  reader APIs CKGP and CKGPAV, which consider data from multiple CK
///  files, when available. See the CK Required reading for details.
///
///  C
///  C     CK parameters include file.
///  C
///        INCLUDE               'ckparam.inc'
///
///  C
///  C     Local variables
///  C
///        CHARACTER*(20)        SCLKCH
///        CHARACTER*(20)        SCTIME
///        CHARACTER*(40)        IDENT
///
///        DOUBLE PRECISION      AV     ( 3 )
///        DOUBLE PRECISION      CLKOUT
///        DOUBLE PRECISION      CMAT   ( 3, 3 )
///        DOUBLE PRECISION      DCD    ( 2 )
///        DOUBLE PRECISION      DESCR  ( 5 )
///        DOUBLE PRECISION      RECORD ( CK4RSZ )
///        DOUBLE PRECISION      SCLKDP
///        DOUBLE PRECISION      TOL
///
///        INTEGER               HANDLE
///        INTEGER               I
///        INTEGER               ICD    ( 6 )
///        INTEGER               INST
///        INTEGER               SC
///
///        LOGICAL               FND
///        LOGICAL               NEEDAV
///        LOGICAL               SFND
///
///  C
///  C     Initial values.
///  C
///        SC     = -94
///        INST   = -94000
///        NEEDAV = .FALSE.
///
///  C
///  C     Load the MGS SCLK kernel and the C-kernel.
///  C
///        CALL FURNSH( 'MGS_SCLK.TSC' )
///        CALL DAFOPR( 'MGS_CK4.BC', HANDLE )
///
///  C
///  C     Get the spacecraft clock time. Then encode it for use
///  C     in the C-kernel.
///  C
///        CALL PROMPT( 'Enter SCLK string: ', SCLKCH )
///        CALL SCENCD( SC, SCLKCH, SCLKDP )
///
///  C
///  C     Use a tolerance of 2 seconds (half of the nominal
///  C     separation between MGS pointing instances ).
///  C
///        CALL SCTIKS ( SC, '0000000002:000', TOL )
///
///  C
///  C     Search backwards from the end of the CK file through all
///  C     of the segments.
///  C
///        CALL DAFBBS( HANDLE )
///        CALL DAFFPA( SFND   )
///
///        FND = .FALSE.
///
///        DO WHILE ( ( SFND ) .AND. ( .NOT. FND ) )
///
///  C
///  C        Get the segment identifier and descriptor.
///  C
///           CALL DAFGN( IDENT )
///           CALL DAFGS( DESCR )
///
///  C
///  C        Unpack the segment descriptor into its integer and
///  C        double precision components.
///  C
///           CALL DAFUS( DESCR, 2, 6, DCD, ICD )
///
///  C
///  C        Determine if this segment should be processed.
///  C
///           IF ( ( INST          .EQ. ICD( 1 ) ) .AND.
///       .        ( SCLKDP + TOL  .GE. DCD( 1 ) ) .AND.
///       .        ( SCLKDP - TOL  .LE. DCD( 2 ) ) .AND.
///       .        ( CK4DTP        .EQ. ICD( 3 ) )      ) THEN
///
///  C
///  C           Find CK 4 record covering requested time.
///  C
///              CALL CKR04( HANDLE, DESCR, SCLKDP, TOL, NEEDAV,
///       .                  RECORD, FND )
///
///              IF ( FND ) THEN
///
///  C
///  C              Compute pointing using found CK 4 record.
///  C
///                 CALL CKE04( NEEDAV, RECORD, CMAT, AV, CLKOUT)
///
///                 CALL SCDECD( SC, CLKOUT, SCTIME )
///
///                 WRITE (*,*)
///                 WRITE (*,*) 'Segment identifier: ', IDENT
///                 WRITE (*,*)
///                 WRITE (*,*) 'Pointing returned for time: ',
///       .                      SCTIME
///                 WRITE (*,*)
///                 WRITE (*,*) 'C-matrix:'
///                 WRITE (*,*)
///                 WRITE (*,*) ( CMAT(1,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(2,I), I = 1, 3 )
///                 WRITE (*,*) ( CMAT(3,I), I = 1, 3 )
///                 WRITE (*,*)
///
///              END IF
///
///           END IF
///
///           CALL DAFFPA ( SFND )
///
///        END DO
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The file containing the segment should be opened for read
///      or write access either by CKLPF, DAFOPR, or DAFOPW.
///
///  2)  The record returned by this routine is intended to be
///      evaluated by CKE04.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
///  Y.K. Zaiko         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.3, 17-AUG-2021 (NJB) (JDR)
///
///         Updated code example to use backwards search. Added
///         note regarding difference between this search and those
///         performed by the CK reader APIs CKGP and CKGPAV.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.1, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn ckr04(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    sclkdp: f64,
    tol: f64,
    needav: bool,
    record: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    CKR04(
        handle,
        descr,
        sclkdp,
        tol,
        needav,
        record,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKR04 ( C-kernel, read pointing record, data type 4 )
pub fn CKR04(
    HANDLE: i32,
    DESCR: &[f64],
    SCLKDP: f64,
    TOL: f64,
    NEEDAV: bool,
    RECORD: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut VALUE: f64 = 0.0;
    let mut MIDPT1: f64 = 0.0;
    let mut MIDPT2: f64 = 0.0;
    let mut RAD1: f64 = 0.0;
    let mut RAD2: f64 = 0.0;
    let mut LBND1: f64 = 0.0;
    let mut LBND2: f64 = 0.0;
    let mut RBND1: f64 = 0.0;
    let mut DCD = StackArray::<f64, 2>::new(1..=2);
    let mut CLKOUT: f64 = 0.0;
    let mut NUMALL: i32 = 0;
    let mut NUMCFT = StackArray::<i32, 7>::new(1..=QAVSIZ);
    let mut ICD = StackArray::<i32, 6>::new(1..=6);
    let mut ENDS: i32 = 0;
    let mut INDX: i32 = 0;
    let mut NREC: i32 = 0;
    let mut EXIST: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKR04", ctx)?;
    }

    //
    // Set initial value of the found flag to "NOT FOUND".
    //
    *FOUND = false;

    //
    // We need to unpack and analyze descriptor components. The
    // unpacked descriptor contains the following information
    // about the segment:
    //
    //    DCD(1)  Initial encoded SCLK
    //    DCD(2)  Final encoded SCLK
    //    ICD(1)  Instrument
    //    ICD(2)  Inertial reference frame
    //    ICD(3)  Data type
    //    ICD(4)  Angular velocity flag
    //    ICD(5)  Initial address of segment data
    //    ICD(6)  Final address of segment data
    //
    DAFUS(
        DESCR.as_slice(),
        2,
        6,
        DCD.as_slice_mut(),
        ICD.as_slice_mut(),
    );

    //
    // Check if the segment is type 4. Signal an error if it's not.
    //
    if (ICD[3] != CK4DTP) {
        SETMSG(b"The segment is not a type 4 segment.  Type is #", ctx);
        ERRINT(b"#", ICD[3], ctx);
        SIGERR(b"SPICE(WRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKR04", ctx)?;
        return Ok(());
    }

    if NEEDAV {
        //
        // Signal an error if angular velocities are required but
        // they are not present in the segment.
        //
        if (ICD[4] != 1) {
            SETMSG(b"Segment does not contain angular velocity data.", ctx);
            SIGERR(b"SPICE(NOAVDATA)", ctx)?;
            CHKOUT(b"CKR04", ctx)?;
            return Ok(());
        }
    }

    //
    // Get number of records (packets) in the segment.
    //
    CKNR04(HANDLE, DESCR.as_slice(), &mut NREC, ctx)?;

    //
    // Locate the last time in the set of reference epochs less than or
    // equal to the input SCLKDP.
    //
    SGFRVI(
        HANDLE,
        DESCR.as_slice(),
        SCLKDP,
        &mut VALUE,
        &mut INDX,
        &mut EXIST,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKR04", ctx)?;
        return Ok(());
    }

    if !EXIST {
        //
        // We didn't find reference value with means that SCLKDP is
        // less than the left bound of the first interpolation interval.
        // Fetch the first record.
        //
        INDX = 1;
        SGFPKT(
            HANDLE,
            DESCR.as_slice(),
            INDX,
            INDX,
            RECORD.as_slice_mut(),
            std::slice::from_mut(&mut ENDS),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CKR04", ctx)?;
            return Ok(());
        }

        MIDPT1 = RECORD[1];
        RAD1 = RECORD[2];

        //
        // Check whether SCLKDP is within TOL of the left bound of the
        // first interval.
        //
        LBND1 = ((MIDPT1 - RAD1) - TOL);

        if (SCLKDP >= LBND1) {
            *FOUND = true;
            CLKOUT = (MIDPT1 - RAD1);
        }
    } else {
        //
        // We found reference value.
        //
        if (INDX >= NREC) {
            //
            // The SCLKDP is greater than the left bound of the last
            // interpolation interval. Fetch the last record.
            //
            INDX = NREC;

            SGFPKT(
                HANDLE,
                DESCR.as_slice(),
                INDX,
                INDX,
                RECORD.as_slice_mut(),
                std::slice::from_mut(&mut ENDS),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"CKR04", ctx)?;
                return Ok(());
            }

            MIDPT1 = RECORD[1];
            RAD1 = RECORD[2];

            //
            // Check whether SCLKDP is within TOL of the right bound of
            // the last interval.
            //
            RBND1 = ((MIDPT1 + RAD1) + TOL);

            if (SCLKDP <= RBND1) {
                *FOUND = true;

                //
                // Check whether SCLKDP falls between right bound of the
                // last interval and right bound + TOL.
                //
                RBND1 = (MIDPT1 + RAD1);

                if (SCLKDP >= RBND1) {
                    CLKOUT = (MIDPT1 + RAD1);
                } else {
                    //
                    // SCLKDP belongs to the last interval
                    //
                    CLKOUT = SCLKDP;
                }
            }
        } else if ((INDX >= 1) && (INDX < NREC)) {
            //
            // The SCLKDP lies between left bound of the first interval
            // and the right bound of the interval before the last
            // interval. Fetch the found record.
            //
            SGFPKT(
                HANDLE,
                DESCR.as_slice(),
                INDX,
                INDX,
                RECORD.as_slice_mut(),
                std::slice::from_mut(&mut ENDS),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"CKR04", ctx)?;
                return Ok(());
            }

            MIDPT1 = RECORD[1];
            RAD1 = RECORD[2];

            //
            // Check whether SCLKDP belongs to current interval.
            //
            RBND1 = (MIDPT1 + RAD1);

            if (SCLKDP <= RBND1) {
                *FOUND = true;
                CLKOUT = SCLKDP;
            } else {
                //
                // SCLKDP doesn't belong to current interval. Fetch the
                // next packet.
                //
                SGFPKT(
                    HANDLE,
                    DESCR.as_slice(),
                    (INDX + 1),
                    (INDX + 1),
                    RECORD.as_slice_mut(),
                    std::slice::from_mut(&mut ENDS),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"CKR04", ctx)?;
                    return Ok(());
                }

                MIDPT2 = RECORD[1];
                RAD2 = RECORD[2];

                //
                // Find the closest interval bound for SCLKDP.
                //
                RBND1 = (MIDPT1 + RAD1);
                LBND2 = (MIDPT2 - RAD2);

                if ((SCLKDP - RBND1) <= (LBND2 - SCLKDP)) {
                    //
                    // SCLKDP is closer to the right bound of current
                    // interval. Check whether it's within TOL of it.
                    //
                    RBND1 = ((MIDPT1 + RAD1) + TOL);

                    if (SCLKDP <= RBND1) {
                        *FOUND = true;
                        CLKOUT = (MIDPT1 + RAD1);

                        //
                        // At this point we need to re-read our current
                        // record because it was overwritten by the next
                        // record. No FAILED() check here -- we already
                        // fetched this packet successfully one call to
                        // SGFPKT ago.
                        //
                        SGFPKT(
                            HANDLE,
                            DESCR.as_slice(),
                            INDX,
                            INDX,
                            RECORD.as_slice_mut(),
                            std::slice::from_mut(&mut ENDS),
                            ctx,
                        )?;
                    }
                } else {
                    //
                    // SCLKDP is closer to the left bound of the next
                    // interval. Check whether it's within TOL of it.
                    //
                    LBND2 = ((MIDPT2 - RAD2) - TOL);

                    if (SCLKDP >= LBND2) {
                        *FOUND = true;
                        INDX = (INDX + 1);
                        CLKOUT = (MIDPT2 - RAD2);
                    }
                }
            }
        }
    }

    //
    // If we found the interval on segment the SCLKDP belongs to, then
    //
    if *FOUND {
        //
        // Decode numbers of polynomial coefficients.
        //
        ZZCK4D2I(&mut RECORD[3], QAVSIZ, CK4PCD, NUMCFT.as_slice_mut());

        //
        // Count total number of coefficients.
        //
        NUMALL = 0;

        for K in 1..=QAVSIZ {
            NUMALL = (NUMALL + NUMCFT[K]);
        }

        //
        // Move coefficients to the right and insert numbers of
        // coefficients into output RECORD.
        //
        for K in intrinsics::range(NUMALL, 1, -1) {
            RECORD[(K + CK4SFT)] = RECORD[(K + 3)];
        }

        for K in 1..=QAVSIZ {
            RECORD[(K + 3)] = NUMCFT[K] as f64;
        }

        RECORD[3] = RECORD[2];
        RECORD[2] = RECORD[1];

        //
        // Insert CLKOUT into output RECORD
        //
        RECORD[1] = CLKOUT;
    }

    //
    // All done.
    //
    CHKOUT(b"CKR04", ctx)?;

    Ok(())
}
