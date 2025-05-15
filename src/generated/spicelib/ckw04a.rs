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
const SHFTAD: i32 = 6;

/// CK type 04: Add data to a segment
///
/// Add data to a type 4 CK segment currently being written to
/// the file associated with HANDLE. See also CKW04B and CKW04E.
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
///  HANDLE     I   The handle of an DAF file opened for writing.
///  NPKTS      I   Number of data packets to write to a segment.
///  PKTSIZ     I   The numbers of values in the data packets
///  PKTDAT     I   The data packets.
///  SCLKDP     I   The SCLK times associated with the data packets.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a CK file in which a CK type 4
///           segment is currently being written.
///
///  NPKTS    is the number of data packets to write to a segment.
///
///  PKTSIZ   is the number of values in all data packets.
///
///  PKTDAT   is the data packets. The data packets in this array
///           must be organized as described in the $Particulars
///           section of the header.
///
///  SCLKDP   contains the initial SCLK times corresponding to the
///           Chebyshev coefficients in PKTSIZ. The I'th time is
///           start time of the I'th packet coverage interval.
///           The times must form a strictly increasing sequence.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  Data is stored in a segment in the DAF file associated with
///  HANDLE.
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
///  1)  If the number of coefficient sets and epochs is not positive,
///      the error SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If size of any input packet is greater that maximum allowed
///      type 4 CK record size minus one, the error
///      SPICE(INVALIDARGUMENT) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine adds data to a type 4 CK segment that is currently
///  being written to the associated with HANDLE. The segment must
///  have been started by a call to the routine CKW04B, the routine
///  which begins a type 4 CK segment.
///
///  This routine is one of a set of three routines for creating and
///  adding data to type 4 CK segments. These routines are:
///
///     CKW04B: Begin a type 4 CK segment. This routine must be
///             called before any data may be added to a type 4
///             segment.
///
///     CKW04A: Add data to a type 4 CK segment. This routine may be
///             called any number of times after a call to CKW04B to
///             add type 4 records to the CK segment that was
///             started.
///
///     CKW04E: End a type 4 CK segment. This routine is called to
///             make the type 4 segment a permanent addition to the
///             DAF file. Once this routine is called, no further type
///             4 records may be added to the segment. A new segment
///             must be started.
///
///  A type 4 CK segment consists of coefficient sets for variable
///  order Chebyshev polynomials over consecutive time intervals of a
///  variable length. The gaps between intervals are allowed. The
///  Chebyshev polynomials represent individual SPICE-style quaternion
///  components q0, q1, q2 and q3 and individual angular velocities
///  AV1, AV2 and AV3 if they are included with the data.
///
///  See the discussion of quaternion styles below.
///
///  The pointing data supplied to the type 4 CK writer (CKW04A)
///  is packed into an array as a sequence of records,
///
///     ----------------------------------------------------
///     | Record 1 | Record 2 | .. | Record N-1 | Record N |
///     ----------------------------------------------------
///
///  with each record in data packets has the following format.
///
///     ----------------------------------------------------
///     | The midpoint of the approximation interval       |
///     ----------------------------------------------------
///     | The radius of the approximation interval         |
///     ----------------------------------------------------
///     | Number of coefficients for q0                    |
///     ----------------------------------------------------
///     | Number of coefficients for q1                    |
///     ----------------------------------------------------
///     | Number of coefficients for q2                    |
///     ----------------------------------------------------
///     | Number of coefficients for q3                    |
///     ----------------------------------------------------
///     | Number of coefficients for AV1                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV2                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV3                   |
///     ----------------------------------------------------
///     | q0 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q1 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q2 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q3 Cheby coefficients                            |
///     ----------------------------------------------------
///     | AV1 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV2 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV3 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///
///
///
///  Quaternion Styles
///  -----------------
///
///  There are different "styles" of quaternions used in
///  science and engineering applications. Quaternion styles
///  are characterized by
///
///  -  The order of quaternion elements
///
///  -  The quaternion multiplication formula
///
///  -  The convention for associating quaternions
///     with rotation matrices
///
///  Two of the commonly used styles are
///
///     - "SPICE"
///
///        > Invented by Sir William Rowan Hamilton
///        > Frequently used in mathematics and physics textbooks
///
///     - "Engineering"
///
///        > Widely used in aerospace engineering applications
///
///
///  SPICELIB subroutine interfaces ALWAYS use SPICE quaternions.
///  Quaternions of any other style must be converted to SPICE
///  quaternions before they are passed to SPICELIB routines.
///
///
///  Relationship between SPICE and Engineering Quaternions
///  ------------------------------------------------------
///
///  Let M be a rotation matrix such that for any vector V,
///
///     M*V
///
///  is the result of rotating V by theta radians in the
///  counterclockwise direction about unit rotation axis vector A.
///  Then the SPICE quaternions representing M are
///
///     (+/-) (  cos(theta/2),
///              sin(theta/2) A(1),
///              sin(theta/2) A(2),
///              sin(theta/2) A(3)  )
///
///  while the engineering quaternions representing M are
///
///     (+/-) ( -sin(theta/2) A(1),
///             -sin(theta/2) A(2),
///             -sin(theta/2) A(3),
///              cos(theta/2)       )
///
///  For both styles of quaternions, if a quaternion q represents
///  a rotation matrix M, then -q represents M as well.
///
///  Given an engineering quaternion
///
///     QENG   = ( q0,  q1,  q2,  q3 )
///
///  the equivalent SPICE quaternion is
///
///     QSPICE = ( q3, -q0, -q1, -q2 )
///
///
///  Associating SPICE Quaternions with Rotation Matrices
///  ----------------------------------------------------
///
///  Let FROM and TO be two right-handed reference frames, for
///  example, an inertial frame and a spacecraft-fixed frame. Let the
///  symbols
///
///     V    ,   V
///      FROM     TO
///
///  denote, respectively, an arbitrary vector expressed relative to
///  the FROM and TO frames. Let M denote the transformation matrix
///  that transforms vectors from frame FROM to frame TO; then
///
///     V   =  M * V
///      TO         FROM
///
///  where the expression on the right hand side represents left
///  multiplication of the vector by the matrix.
///
///  Then if the unit-length SPICE quaternion q represents M, where
///
///     q = (q0, q1, q2, q3)
///
///  the elements of M are derived from the elements of q as follows:
///
///       +-                                                         -+
///       |           2    2                                          |
///       | 1 - 2*( q2 + q3 )   2*(q1*q2 - q0*q3)   2*(q1*q3 + q0*q2) |
///       |                                                           |
///       |                                                           |
///       |                               2    2                      |
///   M = | 2*(q1*q2 + q0*q3)   1 - 2*( q1 + q3 )   2*(q2*q3 - q0*q1) |
///       |                                                           |
///       |                                                           |
///       |                                                   2    2  |
///       | 2*(q1*q3 - q0*q2)   2*(q2*q3 + q0*q1)   1 - 2*( q1 + q2 ) |
///       |                                                           |
///       +-                                                         -+
///
///  Note that substituting the elements of -q for those of q in the
///  right hand side leaves each element of M unchanged; this shows
///  that if a quaternion q represents a matrix M, then so does the
///  quaternion -q.
///
///  To map the rotation matrix M to a unit quaternion, we start by
///  decomposing the rotation matrix as a sum of symmetric
///  and skew-symmetric parts:
///
///                                     2
///     M = [ I  +  (1-cos(theta)) OMEGA  ] + [ sin(theta) OMEGA ]
///
///                  symmetric                   skew-symmetric
///
///
///  OMEGA is a skew-symmetric matrix of the form
///
///                +-             -+
///                |  0   -n3   n2 |
///                |               |
///      OMEGA  =  |  n3   0   -n1 |
///                |               |
///                | -n2   n1   0  |
///                +-             -+
///
///  The vector N of matrix entries (n1, n2, n3) is the rotation axis
///  of M and theta is M's rotation angle. Note that N and theta
///  are not unique.
///
///  Let
///
///     C = cos(theta/2)
///     S = sin(theta/2)
///
///  Then the unit quaternions Q corresponding to M are
///
///     Q = +/- ( C, S*n1, S*n2, S*n3 )
///
///  The mappings between quaternions and the corresponding rotations
///  are carried out by the SPICELIB routines
///
///     Q2M {quaternion to matrix}
///     M2Q {matrix to quaternion}
///
///  M2Q always returns a quaternion with scalar part greater than
///  or equal to zero.
///
///
///  SPICE Quaternion Multiplication Formula
///  ---------------------------------------
///
///  Given a SPICE quaternion
///
///     Q = ( q0, q1, q2, q3 )
///
///  corresponding to rotation axis A and angle theta as above, we can
///  represent Q using "scalar + vector" notation as follows:
///
///     s =   q0           = cos(theta/2)
///
///     v = ( q1, q2, q3 ) = sin(theta/2) * A
///
///     Q = s + v
///
///  Let Q1 and Q2 be SPICE quaternions with respective scalar
///  and vector parts s1, s2 and v1, v2:
///
///     Q1 = s1 + v1
///     Q2 = s2 + v2
///
///  We represent the dot product of v1 and v2 by
///
///     <v1, v2>
///
///  and the cross product of v1 and v2 by
///
///     v1 x v2
///
///  Then the SPICE quaternion product is
///
///     Q1*Q2 = s1*s2 - <v1,v2>  + s1*v2 + s2*v1 + (v1 x v2)
///
///  If Q1 and Q2 represent the rotation matrices M1 and M2
///  respectively, then the quaternion product
///
///     Q1*Q2
///
///  represents the matrix product
///
///     M1*M2
/// ```
///
/// # Examples
///
/// ```text
///  Assume that we have:
///
///     HANDLE   is the handle of an CK file opened with write
///              access.
///
///     SEGID    is a character string of no more than 40 characters
///              which provides a pedigree for the data in the CK
///              segment we will create.
///
///     INST     is the SPICE ID code for the instrument whose
///              pointing data is to be placed into the file.
///
///     AVFLAG   angular rates flag.
///
///     REFFRM   is the name of the SPICE reference frame for the
///              pointing data.
///
///     BEGTIM   is the starting encoded SCLK time for which the
///              segment is valid.
///
///     ENDTIM   is the ending encoded SCLK time for which the segment
///              is valid.
///
///     N        is the number of type 4 records that we want to
///              put into a segment in an CK file.
///
///     NPKTS    is integer array which contains the lengths of
///              variable size data packets
///
///     RECRDS   contains N type 4 records packaged for the CK
///              file.
///
///     SCSTRT   contains the initial encoded SC time for each of
///              the records contained in RECRDS, where
///
///                 SCSTRT(I) < SCSTRT(I+1), I = 1, N-1
///
///                 SCSTRT(1) <= FIRST, SCSTRT(N) < LAST
///
///  Then the following code fragment demonstrates how to create
///  a type 4 CK segment if all of the data for the segment is
///  available at one time.
///
///  C
///  C     Begin the segment.
///  C
///        CALL CKW04B ( HANDLE, BEGTIM, INST, REF, AVFLAG, SEGID )
///  C
///  C     Add the data to the segment all at once.
///  C
///        CALL CKW04A ( HANDLE, N, NPKTS, RECRDS, SCSTRT )
///  C
///  C     End the segment, making the segment a permanent
///  C     addition to the CK file.
///  C
///        CALL CKW04E ( HANDLE, ENDTIM )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The type 4 CK segment to which the data is added must have
///      been started by the routine CKW04B, the routine which begins
///      a type 4 CK segment.
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
/// -    SPICELIB Version 1.1.3, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.2, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.1.1, 26-FEB-2008 (NJB)
///
///         Updated header; added information about SPICE
///         quaternion conventions.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Removed DAFHLU call; replaced ERRFNM call with ERRHAN.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn ckw04a(
    ctx: &mut SpiceContext,
    handle: i32,
    npkts: i32,
    pktsiz: &mut [i32],
    pktdat: &mut [f64],
    sclkdp: &[f64],
) -> crate::Result<()> {
    CKW04A(handle, npkts, pktsiz, pktdat, sclkdp, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW04A ( CK type 04: Add data to a segment )
pub fn CKW04A(
    HANDLE: i32,
    NPKTS: i32,
    PKTSIZ: &mut [i32],
    PKTDAT: &mut [f64],
    SCLKDP: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PKTSIZ = DummyArrayMut::new(PKTSIZ, 1..);
    let mut PKTDAT = DummyArrayMut::new(PKTDAT, 1..);
    let SCLKDP = DummyArray::new(SCLKDP, 1..);
    let mut NUMCFT = StackArray::<i32, 7>::new(1..=QAVSIZ);
    let mut DISPLM: i32 = 0;
    let mut DISPM: i32 = 0;

    //
    // Spicelib functions.
    //

    //
    // Local parameters.
    //

    //
    // The number of elements by which coefficients in each packet
    // have to be shifted to the left after numbers of coefficients
    // were packed into a single integer.
    //

    //
    // Local Variables.
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKW04A", ctx)?;
    }

    //
    // First, check if the number of coefficient sets and epochs
    // is positive and whether each packet is smaller than the
    // maximum size of a record that CKPFS can handle.
    //
    for K in 1..=NPKTS {
        if (PKTSIZ[K] <= 0) {
            SETMSG(b"The number of coefficient sets and epochs in the # data packet (record) to be added to the DAF segment in the file \'#\' was not positive. Its value was: #.", ctx);
            ERRINT(b"#", K, ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            ERRINT(b"#", PKTSIZ[K], ctx);
            SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
            CHKOUT(b"CKW04A", ctx)?;
            return Ok(());
        }

        //
        // We do .GE. comparison because a type 4 CK record passed
        // inside CKPFS will have one more element -- time at which
        // the pointing will be evaluated.
        //
        if (PKTSIZ[K] >= CK4RSZ) {
            SETMSG(b"The total size of the # data packet (record) to be added to the DAF segment in the file \'#\' is greater than the maximum allowed type 4 record size #. Its value was: #.", ctx);
            ERRINT(b"#", K, ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            ERRINT(b"#", (CK4RSZ - 1), ctx);
            ERRINT(b"#", PKTSIZ[K], ctx);
            SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
            CHKOUT(b"CKW04A", ctx)?;
            return Ok(());
        }
    }

    DISPLM = 0;
    DISPM = 0;
    //
    // The cycle below encodes groups of numbers of coefficients in
    // data packets to single double precision numbers and shift
    // data in packets to the left to decrease the data packet
    // lengths.
    //
    for K in 1..=NPKTS {
        //
        // Encode integer numbers of coefficients for each component
        // to single double precision variable
        //
        for KK in 1..=QAVSIZ {
            NUMCFT[KK] = (PKTDAT[((KK + 2) + DISPLM)] as i32);
        }

        ZZCK4I2D(NUMCFT.as_slice(), QAVSIZ, CK4PCD, &mut PKTDAT[(3 + DISPM)]);

        //
        // Shift coefficients sets to the left to overwrite numbers of
        // packets
        //
        for KK in 4..=PKTSIZ[K] {
            PKTDAT[(KK + DISPM)] = PKTDAT[((KK + SHFTAD) + DISPLM)];
        }

        //
        // Shift middle value and radii of interval
        //
        PKTDAT[(1 + DISPM)] = PKTDAT[(1 + DISPLM)];
        PKTDAT[(2 + DISPM)] = PKTDAT[(2 + DISPLM)];

        DISPLM = (DISPLM + PKTSIZ[K]);

        //
        // Length of each data packet became less for 6 elements because
        // of encoding of 7 double precision numbers, which are the
        // numbers of polynomial coefficients, to one double precision
        // number
        //
        PKTSIZ[K] = (PKTSIZ[K] - SHFTAD);
        DISPM = (DISPM + PKTSIZ[K]);
    }

    //
    // Add the data.
    //
    SGWVPK(
        HANDLE,
        NPKTS,
        PKTSIZ.as_slice(),
        PKTDAT.as_slice(),
        NPKTS,
        SCLKDP.as_slice(),
        ctx,
    )?;

    //
    // No need to check FAILED() here, since all we do is check out.
    // Leave it up to the caller.
    //
    CHKOUT(b"CKW04A", ctx)?;

    Ok(())
}
