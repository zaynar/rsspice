//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NTOL: f64 = 0.1;
const DTOL: f64 = 0.1;

/// Matrix to quaternion
///
/// Find a unit quaternion corresponding to a specified rotation
/// matrix.
///
/// # Required Reading
///
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  R          I   A rotation matrix.
///  Q          O   A unit quaternion representing R.
/// ```
///
/// # Detailed Input
///
/// ```text
///  R        is a rotation matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  Q        is a unit-length SPICE-style quaternion
///           representing R. See the discussion of quaternion
///           styles in $Particulars below.
///
///           Q is a 4-dimensional vector. If R rotates vectors
///           in the counterclockwise sense by an angle of theta
///           radians about a unit vector A, where
///
///              0 < theta < pi
///                -       -
///
///           then letting h = theta/2,
///
///              Q = ( cos(h), sin(h)A ,  sin(h)A ,  sin(h)A ).
///                                   1          2          3
///
///           The restriction that theta must be in the range
///           [0, pi] determines the output quaternion Q
///           uniquely except when theta = pi; in this special
///           case, both of the quaternions
///
///              Q = ( 0,  A ,  A ,  A  )
///                         1    2    3
///           and
///
///              Q = ( 0, -A , -A , -A  )
///                         1    2    3
///
///           are possible outputs.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If R is not a rotation matrix, the error SPICE(NOTAROTATION)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  A unit quaternion is a 4-dimensional vector for which the sum of
///  the squares of the components is 1. Unit quaternions can be used
///  to represent rotations in the following way: given a rotation
///  angle theta, where
///
///     0 < theta < pi
///       -       -
///
///  and a unit vector A, we can represent the transformation that
///  rotates vectors in the counterclockwise sense by theta radians
///  about A using the quaternion Q, where
///
///     Q =
///
///     ( cos(theta/2), sin(theta/2)a , sin(theta/2)a , sin(theta/2)a )
///                                  1               2               3
///
///  As mentioned in Detailed Output, our restriction on the range of
///  theta determines Q uniquely, except when theta = pi.
///
///  The SPICELIB routine Q2M is an one-sided inverse of this routine:
///  given any rotation matrix R, the calls
///
///     CALL M2Q ( R, Q )
///     CALL Q2M ( Q, R )
///
///  leave R unchanged, except for round-off error. However, the
///  calls
///
///     CALL Q2M ( Q, R )
///     CALL M2Q ( R, Q )
///
///  might preserve Q or convert Q to -Q.
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
///  1)  A case amenable to checking by hand calculation:
///
///         To convert the rotation matrix
///
///                  +-              -+
///                  |  0     1    0  |
///                  |                |
///            R  =  | -1     0    0  |
///                  |                |
///                  |  0     0    1  |
///                  +-              -+
///
///         also represented as
///
///            [ pi/2 ]
///                    3
///
///         to a quaternion, we can use the code fragment
///
///            CALL ROTATE (  HALFPI(),  3,  R  )
///            CALL M2Q    (  R,             Q  )
///
///         M2Q will return Q as
///
///            ( sqrt(2)/2, 0, 0, -sqrt(2)/2 )
///
///         Why?  Well, R is a reference frame transformation that
///         rotates vectors by -pi/2 radians about the axis vector
///
///            A  = ( 0, 0, 1 )
///
///         Equivalently, R rotates vectors by pi/2 radians in
///         the counterclockwise sense about the axis vector
///
///            -A = ( 0, 0, -1 )
///
///         so our definition of Q,
///
///            h = theta/2
///
///            Q = ( cos(h), sin(h)A , sin(h)A , sin(h)A  )
///                                 1         2         3
///
///         implies that in this case,
///
///            Q =  ( cos(pi/4),  0,  0, -sin(pi/4)  )
///
///              =  ( sqrt(2)/2,  0,  0, -sqrt(2)/2  )
///
///
///  2)  Finding a quaternion that represents a rotation specified by
///      a set of Euler angles:
///
///         Suppose our original rotation R is the product
///
///            [ TAU ]  [ pi/2 - DELTA ]  [ ALPHA ]
///                   3                 2          3
///
///         The code fragment
///
///            CALL EUL2M  ( TAU,   HALFPI() - DELTA,   ALPHA,
///           .              3,     2,                  3,      R )
///
///            CALL M2Q    ( R, Q )
///
///         yields a quaternion Q that represents R.
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
/// -    SPICELIB Version 2.1.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 27-FEB-2008 (NJB)
///
///         Updated header; added information about SPICE
///         quaternion conventions. Made various minor edits
///         throughout header.
///
/// -    SPICELIB Version 2.0.0, 17-SEP-1999 (WLT)
///
///         The routine was re-implemented to sharpen the numerical
///         stability of the routine and eliminate calls to SIN
///         and COS functions.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1990 (NJB)
/// ```
pub fn m2q(ctx: &mut SpiceContext, r: &[[f64; 3]; 3], q: &mut [f64; 4]) -> crate::Result<()> {
    M2Q(r.as_flattened(), q, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure M2Q ( Matrix to quaternion )
pub fn M2Q(R: &[f64], Q: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let R = DummyArray2D::new(R, 1..=3, 1..=3);
    let mut Q = DummyArrayMut::new(Q, 0..=3);
    let mut C: f64 = 0.0;
    let mut CC4: f64 = 0.0;
    let mut FACTOR: f64 = 0.0;
    let mut L2: f64 = 0.0;
    let mut MTRACE: f64 = 0.0;
    let mut POLISH: f64 = 0.0;
    let mut S = StackArray::<f64, 3>::new(1..=3);
    let mut S114: f64 = 0.0;
    let mut S224: f64 = 0.0;
    let mut S334: f64 = 0.0;
    let mut TRACE: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // NTOL and DETOL are used to determine whether R is a rotation
    // matrix.
    //
    // NTOL is the tolerance for the norms of the columns of R.
    //
    // DTOL is the tolerance for the determinant of a matrix whose
    // columns are the unitized columns of R.
    //
    //

    //
    // Local Variables
    //

    //
    // If R is not a rotation matrix, we can't proceed.
    //
    if !ISROT(R.as_slice(), NTOL, DTOL, ctx)? {
        CHKIN(b"M2Q", ctx)?;
        SETMSG(b"Input matrix was not a rotation.", ctx);
        SIGERR(b"SPICE(NOTAROTATION)", ctx)?;
        CHKOUT(b"M2Q", ctx)?;
        return Ok(());
    }
    //
    //
    // If our quaternion is C, S1, S2, S3 (the S's being the imaginary
    // part) and we let
    //
    //    CSi = C  * Si
    //    Sij = Si * Sj
    //
    // then the rotation matrix corresponding to our quaternion is:
    //
    //    R(1,1)      = 1.0D0 - 2*S22 - 2*S33
    //    R(2,1)      =         2*S12 + 2*CS3
    //    R(3,1)      =         2*S13 - 2*CS2
    //
    //    R(1,2)      =         2*S12 - 2*CS3
    //    R(2,2)      = 1.0D0 - 2*S11 - 2*S33
    //    R(3,2)      =         2*S23 + 2*CS1
    //
    //    R(1,3)      =         2*S13 + 2*CS2
    //    R(2,3)      =         2*S23 - 2*CS1
    //    R(3,3)      = 1.0D0 - 2*S11 - 2*S22
    //
    //    From the above we can see that
    //
    //       TRACE = 3 - 4*(S11 + S22 + S33)
    //
    //    so that
    //
    //
    //       1.0D0 + TRACE = 4 - 4*(S11 + S22 + S33)
    //                     = 4*(CC + S11 + S22 + S33)
    //                     - 4*(S11 + S22 + S33)
    //                     = 4*CC
    //
    //    Thus up to sign
    //
    //      C = 0.5D0 * DSQRT( 1.0D0 + TRACE )
    //
    //    But we also have
    //
    //      1.0D0 + TRACE - 2.0D0*R(i,i) = 4.0D0 - 4.0D0(Sii + Sjj + Skk)
    //                                   - 2.0D0 + 4.0D0(Sjj + Skk )
    //
    //                                   = 2.0D0 - 4.0D0*Sii
    //
    //    So that
    //
    //       1.0D0 - TRACE + 2.0D0*R(i,i) = 4.0D0*Sii
    //
    //    and so up to sign
    //
    //       Si = 0.5D0*DSQRT( 1.0D0 - TRACE + 2.0D0*R(i,i) )
    //
    //    in addition to this observation, we note that all of the
    //    product pairs can easily be computed
    //
    //     CS1 = (R(3,2) - R(2,3))/4.0D0
    //     CS2 = (R(1,3) - R(3,1))/4.0D0
    //     CS3 = (R(2,1) - R(1,2))/4.0D0
    //     S12 = (R(2,1) + R(1,2))/4.0D0
    //     S13 = (R(3,1) + R(1,3))/4.0D0
    //     S23 = (R(2,3) + R(3,2))/4.0D0
    //
    // But taking sums or differences of numbers that are nearly equal
    // or nearly opposite results in a loss of precision. As a result
    // we should take some care in which terms to select when computing
    // C, S1, S2, S3.  However, by simply starting with one of the
    // large quantities cc, S11, S22, or S33 we can make sure that we
    // use the best of the 6 quantities above when computing the
    // remaining components of the quaternion.
    //

    TRACE = ((R[[1, 1]] + R[[2, 2]]) + R[[3, 3]]);
    MTRACE = (1.0 - TRACE);

    CC4 = (1.0 + TRACE);
    S114 = (MTRACE + (2.0 * R[[1, 1]]));
    S224 = (MTRACE + (2.0 * R[[2, 2]]));
    S334 = (MTRACE + (2.0 * R[[3, 3]]));

    //
    // Note that if you simply add CC4 + S114 + S224 + S334
    // you get four. Thus at least one of the 4 terms is greater than 1.
    //
    if (1.0 <= CC4) {
        C = f64::sqrt((CC4 * 0.25));
        FACTOR = (1.0 / (C * 4.0));

        S[1] = ((R[[3, 2]] - R[[2, 3]]) * FACTOR);
        S[2] = ((R[[1, 3]] - R[[3, 1]]) * FACTOR);
        S[3] = ((R[[2, 1]] - R[[1, 2]]) * FACTOR);
    } else if (1.0 <= S114) {
        S[1] = f64::sqrt((S114 * 0.25));
        FACTOR = (1.0 / (S[1] * 4.0));

        C = ((R[[3, 2]] - R[[2, 3]]) * FACTOR);
        S[2] = ((R[[1, 2]] + R[[2, 1]]) * FACTOR);
        S[3] = ((R[[1, 3]] + R[[3, 1]]) * FACTOR);
    } else if (1.0 <= S224) {
        S[2] = f64::sqrt((S224 * 0.25));
        FACTOR = (1.0 / (S[2] * 4.0));

        C = ((R[[1, 3]] - R[[3, 1]]) * FACTOR);
        S[1] = ((R[[1, 2]] + R[[2, 1]]) * FACTOR);
        S[3] = ((R[[2, 3]] + R[[3, 2]]) * FACTOR);
    } else {
        S[3] = f64::sqrt((S334 * 0.25));
        FACTOR = (1.0 / (S[3] * 4.0));

        C = ((R[[2, 1]] - R[[1, 2]]) * FACTOR);
        S[1] = ((R[[1, 3]] + R[[3, 1]]) * FACTOR);
        S[2] = ((R[[2, 3]] + R[[3, 2]]) * FACTOR);
    }
    //
    // If the magnitude of this quaternion is not one, we polish it
    // up a bit.
    //
    L2 = ((((C * C) + (S[1] * S[1])) + (S[2] * S[2])) + (S[3] * S[3]));

    if (L2 != 1.0) {
        POLISH = (1.0 / f64::sqrt(L2));
        C = (C * POLISH);
        S[1] = (S[1] * POLISH);
        S[2] = (S[2] * POLISH);
        S[3] = (S[3] * POLISH);
    }

    if (C > 0.0) {
        Q[0] = C;
        Q[1] = S[1];
        Q[2] = S[2];
        Q[3] = S[3];
    } else {
        Q[0] = -C;
        Q[1] = -S[1];
        Q[2] = -S[2];
        Q[3] = -S[3];
    }

    Ok(())
}
