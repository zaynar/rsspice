//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Quaternion to matrix
///
/// Find the rotation matrix corresponding to a specified unit
/// quaternion.
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
///  Q          I   A unit quaternion.
///  R          O   A rotation matrix corresponding to Q.
/// ```
///
/// # Detailed Input
///
/// ```text
///  Q        is a unit-length SPICE-style quaternion. Q has the
///           property that
///
///              || Q ||  =  1
///
///           See the discussion of quaternion styles in
///           $Particulars below.
/// ```
///
/// # Detailed Output
///
/// ```text
///  R        is a 3 by 3 rotation matrix representing the same
///           rotation as does Q. See the discussion titled
///           "Associating SPICE Quaternions with Rotation
///           Matrices" in $Particulars below.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If Q is not a unit quaternion, the output matrix R is
///      the rotation matrix that is the result of converting
///      normalized Q to a rotation matrix.
///
///  2)  If Q is the zero quaternion, the output matrix R is
///      the identity matrix.
/// ```
///
/// # Particulars
///
/// ```text
///  If a 4-dimensional vector Q satisfies the equality
///
///     || Q ||   =  1
///
///  or equivalently
///
///         2          2          2          2
///     Q(0)   +   Q(1)   +   Q(2)   +   Q(3)   =  1,
///
///  then we can always find a unit vector A and a scalar r such that
///
///     Q = ( cos(r/2), sin(r/2)A(1), sin(r/2)A(2), sin(r/2)A(3) ).
///
///  We can interpret A and r as the axis and rotation angle of a
///  rotation in 3-space. If we restrict r to the range [0, pi],
///  then r and A are uniquely determined, except if r = pi. In this
///  special case, A and -A are both valid rotation axes.
///
///  Every rotation is represented by a unique orthogonal matrix; this
///  routine returns that unique rotation matrix corresponding to Q.
///
///  The SPICELIB routine M2Q is a one-sided inverse of this routine:
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
///       .-                                                         -.
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
///       `-                                                         -.
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
///                .-             -.
///                |  0   -n3   n2 |
///                |               |
///      OMEGA  =  |  n3   0   -n1 |
///                |               |
///                | -n2   n1   0  |
///                `-             -'
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
///         To convert the quaternion
///
///            Q = ( sqrt(2)/2, 0, 0, -sqrt(2)/2 )
///
///         to a rotation matrix, we can use the code fragment
///
///            Q(0) =  DSQRT(2)/2.D0
///            Q(1) =  0.D0
///            Q(2) =  0.D0
///            Q(3) = -DSQRT(2)/2.D0
///
///            CALL Q2M ( Q, R )
///
///         The matrix R will be set equal to
///
///            +-              -+
///            |  0     1    0  |
///            |                |
///            | -1     0    0  |.
///            |                |
///            |  0     0    1  |
///            +-              -+
///
///         Why?  Well, Q represents a rotation by some angle r about
///         some axis vector A, where r and A satisfy
///
///            Q =
///
///            ( cos(r/2), sin(r/2)A(1), sin(r/2)A(2), sin(r/2)A(3) ).
///
///         In this example,
///
///            Q = ( sqrt(2)/2, 0, 0, -sqrt(2)/2 ),
///
///         so
///
///            cos(r/2) = sqrt(2)/2.
///
///         Assuming that r is in the interval [0, pi], we must have
///
///            r = pi/2,
///
///         so
///
///            sin(r/2) = sqrt(2)/2.
///
///         Since the second through fourth components of Q represent
///
///            sin(r/2) * A,
///
///         it follows that
///
///            A = ( 0, 0, -1 ).
///
///         So Q represents a transformation that rotates vectors by
///         pi/2 about the negative z-axis. This is equivalent to a
///         coordinate system rotation of pi/2 about the positive
///         z-axis; and we recognize R as the matrix
///
///            [ pi/2 ] .
///                    3
///
///
///  2)  Finding a set of Euler angles that represent a rotation
///      specified by a quaternion:
///
///         Suppose our rotation R is represented by the quaternion
///         Q. To find angles TAU, ALPHA, DELTA such that
///
///
///            R  =  [ TAU ]  [ pi/2 - DELTA ]  [ ALPHA ] ,
///                         3                 2          3
///
///         we can use the code fragment
///
///
///            CALL Q2M    ( Q, R )
///
///            CALL M2EUL  ( R,   3,      2,       3,
///           .                   TAU,    DELTA,   ALPHA  )
///
///            DELTA = HALFPI() - DELTA
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 12-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Corrected the
///         output argument name in $Exceptions section.
///
/// -    SPICELIB Version 1.1.2, 26-FEB-2008 (NJB)
///
///         Updated header; added information about SPICE
///         quaternion conventions.
///
/// -    SPICELIB Version 1.1.1, 13-JUN-2002 (FST)
///
///         Updated the $Exceptions section to clarify exceptions that
///         are the result of changes made in the previous version of
///         the routine.
///
/// -    SPICELIB Version 1.1.0, 04-MAR-1999 (WLT)
///
///         Added code to handle the case in which the input quaternion
///         is not of length 1.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 30-AUG-1990 (NJB)
/// ```
pub fn q2m(q: &[f64; 4], r: &mut [[f64; 3]; 3]) {
    Q2M(q, r.as_flattened_mut());
}

//$Procedure Q2M ( Quaternion to matrix )
pub fn Q2M(Q: &[f64], R: &mut [f64]) {
    let Q = DummyArray::new(Q, 0..=3);
    let mut R = DummyArrayMut2D::new(R, 1..=3, 1..=3);
    let mut Q01: f64 = 0.0;
    let mut Q02: f64 = 0.0;
    let mut Q03: f64 = 0.0;
    let mut Q12: f64 = 0.0;
    let mut Q13: f64 = 0.0;
    let mut Q23: f64 = 0.0;
    let mut Q1S: f64 = 0.0;
    let mut Q2S: f64 = 0.0;
    let mut Q3S: f64 = 0.0;
    let mut L2: f64 = 0.0;
    let mut SHARPN: f64 = 0.0;

    //
    // Local variables
    //

    //
    // If a matrix R represents a rotation of r radians about the unit
    // vector n, we know that R can be represented as
    //
    //                                       2
    //    I  +  sin(r) N  +  [ 1 - cos(r) ] N ,
    //
    // where N is the matrix that satisfies
    //
    //    Nv = n x v
    //
    // for all vectors v, namely
    //
    //         +-                -+
    //         |  0    -n     n   |
    //         |         3     2  |
    //         |                  |
    //    N =  |  n     0    -n   |.
    //         |   3           1  |
    //         |                  |
    //         | -n     n     0   |
    //         |   2     1        |
    //         +-                -+
    //
    //
    //  Define S as
    //
    //     sin(r/2) N,
    //
    //  and let our input quaternion Q be
    //
    //     ( q ,  q ,  q ,  q ).
    //        0    1    2    3
    //
    //  Using the facts that
    //
    //                         2
    //     1 - cos(r)  =  2 sin (r/2)
    //
    //  and
    //
    //     sin(r)      =  2 cos(r/2) sin(r/2),
    //
    //
    //  we can express R as
    //
    //                                  2
    //     I  +  2 cos(r/2) S    +   2 S,
    //
    //  or
    //                            2
    //     I  +  2 q  S    +   2 S.
    //              0
    //
    //  Since S is just
    //
    //     +-                -+
    //     |  0    -q     q   |
    //     |         3     2  |
    //     |                  |
    //     |  q     0    -q   |,
    //     |   3           1  |
    //     |                  |
    //     | -q     q     0   |
    //     |   2     1        |
    //     +-                -+
    //
    //  our expression for R comes out to
    //
    //     +-                                                         -+
    //     |          2   2                                            |
    //     | 1 - 2 ( q + q  )    2( q q  -  q q )     2 ( q q  + q q ) |
    //     |          2   3          1 2     0 3           1 3    0 2  |
    //     |                                                           |
    //     |                              2   2                        |
    //     | 2( q q  +  q q )    1 - 2 ( q + q  )     2 ( q q  - q q ) |.
    //     |     1 2     0 3              1   3            2 3    0 1  |
    //     |                                                           |
    //     |                                                   2   2   |
    //     | 2( q q  -  q q )    2 ( q q  + q q )     1 - 2 ( q + q  ) |
    //     |     1 3     0 2          2 3    0 1               1   2   |
    //     +-                                                         -+
    //
    //
    //  For efficiency, we avoid duplicating calculations where possible.
    //

    Q01 = (Q[0] * Q[1]);
    Q02 = (Q[0] * Q[2]);
    Q03 = (Q[0] * Q[3]);

    Q12 = (Q[1] * Q[2]);
    Q13 = (Q[1] * Q[3]);

    Q23 = (Q[2] * Q[3]);

    Q1S = (Q[1] * Q[1]);
    Q2S = (Q[2] * Q[2]);
    Q3S = (Q[3] * Q[3]);

    //
    // We sharpen the computation by effectively converting Q to
    // a unit quaternion if it isn't one already.
    //
    L2 = ((((Q[0] * Q[0]) + Q1S) + Q2S) + Q3S);

    if ((L2 != 1.0) && (L2 != 0.0)) {
        SHARPN = (1.0 / L2);

        Q01 = (Q01 * SHARPN);
        Q02 = (Q02 * SHARPN);
        Q03 = (Q03 * SHARPN);

        Q12 = (Q12 * SHARPN);
        Q13 = (Q13 * SHARPN);

        Q23 = (Q23 * SHARPN);

        Q1S = (Q1S * SHARPN);
        Q2S = (Q2S * SHARPN);
        Q3S = (Q3S * SHARPN);
    }

    R[[1, 1]] = (1.0 - (2.0 * (Q2S + Q3S)));
    R[[2, 1]] = (2.0 * (Q12 + Q03));
    R[[3, 1]] = (2.0 * (Q13 - Q02));

    R[[1, 2]] = (2.0 * (Q12 - Q03));
    R[[2, 2]] = (1.0 - (2.0 * (Q1S + Q3S)));
    R[[3, 2]] = (2.0 * (Q23 + Q01));

    R[[1, 3]] = (2.0 * (Q13 + Q02));
    R[[2, 3]] = (2.0 * (Q23 - Q01));
    R[[3, 3]] = (1.0 - (2.0 * (Q1S + Q2S)));
}
