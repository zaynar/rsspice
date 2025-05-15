//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Compute volume of plate model
///
/// Compute the volume of a three-dimensional region bounded by a
/// collection of triangular plates.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NV         I   Number of vertices.
///  VRTCES     I   Array of vertices.
///  NP         I   Number of triangular plates.
///  PLATES     I   Array of plates.
///
///  The function returns the volume of the spatial region bounded
///  by the plates.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NV       is the number of vertices comprising the plate
///           model.
///
///  VRTCES   is an array containing the plate model's vertices.
///           Elements
///
///              VRTCES( 1, I )
///              VRTCES( 2, I )
///              VRTCES( 3, I )
///
///           are, respectively, the X, Y, and Z components of
///           the Ith vertex.
///
///           This routine doesn't associate units with the
///           vertices.
///
///
///  NP       is the number of triangular plates comprising the
///           plate model.
///
///  PLATES   is an array containing 3-tuples of integers
///           representing the model's plates. The elements of
///           PLATES are vertex indices. The vertex indices are
///           1-based: vertices have indices ranging from 1 to
///           NV. The elements
///
///              PLATES( 1, I )
///              PLATES( 2, I )
///              PLATES( 3, I )
///
///           are, respectively, the indices of the vertices
///           comprising the Ith plate.
///
///           Note that the order of the vertices of a plate is
///           significant: the vertices must be ordered in the
///           positive (counterclockwise) sense with respect to
///           the outward normal direction associated with the
///           plate. In other words, if V1, V2, V3 are the
///           vertices of a plate, then
///
///              ( V2 - V1 )  x  ( V3 - V2 )
///
///           points in the outward normal direction. Here
///           "x" denotes the vector cross product operator.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the volume of the spatial region bounded
///  by the plates.
///
///  If the components of the vertex array have length unit L, then the
///  output volume has units
///
///      3
///     L
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The input plate model must define a spatial region with
///      a boundary. This routine does not check the inputs to
///      verify this condition. See the $Restrictions section below.
///
///  2)  If the number of vertices is less than 4, the error
///      SPICE(TOOFEWVERTICES) is signaled.
///
///  3)  If the number of plates is less than 4, the error
///      SPICE(TOOFEWPLATES) is signaled.
///
///  4)  If any plate contains a vertex index outside of the range
///
///         [1, NV]
///
///      the error SPICE(INDEXOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the volume of a spatial region bounded by
///  a set of triangular plates. If the plate set does not actually
///  form the boundary of a spatial region, the result of this routine
///  is invalid.
///
///  Examples:
///
///     Valid inputs
///     ------------
///     Tetrahedron
///     Box
///     Tiled ellipsoid
///     Two disjoint boxes
///
///     Invalid inputs
///     --------------
///     Single plate
///     Tiled ellipsoid with one plate removed
///     Two boxes with intersection having positive volume
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input
///  (if any), the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///
///  1) Compute the volume of the pyramid defined by the four
///     triangular plates whose vertices are the 3-element
///     subsets of the set of vectors
///
///        ( 0, 0, 0 )
///        ( 1, 0, 0 )
///        ( 0, 1, 0 )
///        ( 0, 0, 1 )
///
///
///     Example code begins here.
///
///
///     C
///     C     Compute the volume of a plate model representing the
///     C     pyramid with one vertex at the origin and the other
///     C     vertices coinciding with the standard basis vectors.
///     C
///           PROGRAM PLTVOL_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      PLTVOL
///     C
///     C     Local parameters
///     C
///           INTEGER               NVERT
///           PARAMETER           ( NVERT  = 4 )
///
///           INTEGER               NPLATE
///           PARAMETER           ( NPLATE = 4 )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      VRTCES ( 3, NVERT  )
///           DOUBLE PRECISION      VOL
///
///           INTEGER               PLATES ( 3, NPLATE )
///
///     C
///     C     Let the notation
///     C
///     C        < A, B >
///     C
///     C     denote the dot product of vectors A and B.
///     C
///     C     The plates defined below lie in the following planes,
///     C     respectively:
///     C
///     C        Plate 1:    { P :  < P, (-1,  0,  0) > = 0 }
///     C        Plate 2:    { P :  < P, ( 0, -1,  0) > = 0 }
///     C        Plate 3:    { P :  < P, ( 0,  0, -1) > = 0 }
///     C        Plate 4:    { P :  < P, ( 1,  1,  1) > = 1 }
///     C
///           DATA                  PLATES /    1,     4,     3,
///          .                                  1,     2,     4,
///          .                                  1,     3,     2,
///          .                                  2,     3,     4 /
///
///           DATA                  VRTCES / 0.D0,  0.D0,  0.D0,
///          .                               1.D0,  0.D0,  0.D0,
///          .                               0.D0,  1.D0,  0.D0,
///          .                               0.D0,  0.D0,  1.D0 /
///
///
///           VOL = PLTVOL ( NVERT, VRTCES, NPLATE, PLATES )
///
///           WRITE (*,*) 'Expected volume =    1/6'
///           WRITE (*,*) 'Computed volume = ', VOL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Expected volume =    1/6
///      Computed volume =   0.16666666666666666
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The plate collection must describe a surface and enclose a
///      volume such that the divergence theorem (see [1]) is
///      applicable.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  T. Apostol, "Calculus, Vol. II," John Wiley & Sons, 1969.
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
/// -    SPICELIB Version 1.0.1, 08-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added DSK to
///         $Required_Reading. Updated code example comments.
///
/// -    SPICELIB Version 1.0.0, 24-OCT-2016 (NJB)
///
///         Based on original 11-FEB-2011
/// ```
pub fn pltvol(
    ctx: &mut SpiceContext,
    nv: i32,
    vrtces: &[[f64; 3]],
    np: i32,
    plates: &[[i32; 3]],
) -> crate::Result<f64> {
    let ret = PLTVOL(
        nv,
        vrtces.as_flattened(),
        np,
        plates.as_flattened(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure PLTVOL ( Compute volume of plate model )
pub fn PLTVOL(
    NV: i32,
    VRTCES: &[f64],
    NP: i32,
    PLATES: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..=NV);
    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..=NP);
    let mut PLTVOL: f64 = 0.0;
    let mut M = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // The function must have an initial value.
    //
    PLTVOL = 0.0;

    //
    // This routine uses discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(PLTVOL);
    }

    //
    // Check the vertex and plate counts.
    //
    if (NV < 4) {
        CHKIN(b"PLTVOL", ctx)?;
        SETMSG(b"At least 4 vertices are needed, but NV = #.", ctx);
        ERRINT(b"#", NV, ctx);
        SIGERR(b"SPICE(TOOFEWVERTICES)", ctx)?;
        CHKOUT(b"PLTVOL", ctx)?;
        return Ok(PLTVOL);
    }

    if (NP < 4) {
        CHKIN(b"PLTVOL", ctx)?;
        SETMSG(b"At least 4 plates are needed, but NP = #.", ctx);
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(TOOFEWPLATES)", ctx)?;
        CHKOUT(b"PLTVOL", ctx)?;
        return Ok(PLTVOL);
    }

    //
    // Make sure the vertex indices are in the range [1, NV].
    //
    for I in 1..=NP {
        for J in 1..=3 {
            if ((PLATES[[J, I]] < 1) || (PLATES[[J, I]] > NV)) {
                CHKIN(b"PLTVOL", ctx)?;
                SETMSG(b"Vertex indices must be in the range [1, NV] for all SPICE language versions. The input value of NV was #. Vertex index # in plate # was #. (The vertex and plate numbers in this message are 1-based as well.)", ctx);
                ERRINT(b"#", NV, ctx);
                ERRINT(b"#", J, ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", PLATES[[J, I]], ctx);
                SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
                CHKOUT(b"PLTVOL", ctx)?;
                return Ok(PLTVOL);
            }
        }
    }

    //
    // The volume computation below requires only a few lines of code,
    // but it might not be obvious that it works. An explanation
    // follows.
    //
    //
    //    Notation
    //    --------
    //
    //    The expression
    //
    //       A x B
    //
    //    denotes the cross product of vectors A and B. The notation
    //
    //       < A, B >
    //
    //    denotes the dot product of these vectors.
    //
    //    The expression
    //
    //       ||A||
    //
    //    denotes the Euclidean (L2) norm of A, and
    //
    //       ^
    //       A
    //
    //    is the unit-length vector aligned with A, namely
    //
    //       A / ||A||
    //
    //    provided A is non-zero.
    //
    //
    //    Geometric assumptions
    //    ---------------------
    //
    //    The algorithm used here assumes, but does not attempt to
    //    verify, that the input plate model represents the boundary of
    //    a spatial region. The region has an "inside" and an "outside,"
    //    so it makes sense to speak of surface normal vectors that are
    //    either "inward pointing" or "outward pointing."

    //    The input plates are assumed to have vertex indices ordered in
    //    the positive sense about their respective outward normal
    //    vectors. So if a plate's vertices are A, B, C, then the
    //    plate's sides are
    //
    //       B - A,  C - B,  A - C
    //
    //    and all of
    //
    //       ( B - A ) x ( C - B )
    //       ( C - B ) x ( A - C )
    //       ( A - C ) x ( B - A )
    //
    //    are outward normal vectors.
    //
    //    Each plate lies in a plane, so all points on the plate satisfy
    //    an equation of the form
    //
    //       < P, N > = constant
    //
    //    where N is an outward normal vector for that plate. If the
    //    plane constant is positive, we say that the outward normal
    //    vector "points away" from the origin; if the constant is
    //    negative, we say that the outward normal "points toward" the
    //    origin. This is a linguistic shortcut for the unwieldy phrases
    //    "the component of the outward normal parallel to P points away
    //    from the origin" or ... (the negative counterpart phrase).
    //
    //
    //    Relationship of volume and surface
    //    ----------------------------------
    //
    //    Given a suitable 3-D spatial region, the divergence theorem
    //    (see [1]) says that the volume integral, over that region, of
    //    the divergence of a vector field F (div F) is equal to the
    //    surface integral of F over the boundary of the region. So if
    //    one picks a vector field F having divergence identically equal
    //    to 1, the surface integral of F equals the volume of the
    //    region.
    //
    //    We can use this fact to set up a surface integral that yields
    //    the volume enclosed by a set of triangular plates.
    //
    //
    //    A faster algorithm
    //    ------------------
    //
    //    However, there's a more efficient approach: we can compute the
    //    volume of the model by summing the signed volumes of the
    //    pyramids whose bases are the plates and whose apexes coincide
    //    with the origin. Plates with outward normal vectors pointing
    //    away from the origin contribute positive volume increments;
    //    plates with outward normal vectors pointing toward the origin
    //    contribute negative volume increments.
    //
    //    We'll show that these two methods are equivalent, and we'll
    //    use the more efficient method in our code.
    //
    //
    //    The surface integral
    //    --------------------
    //
    //    The field
    //
    //       F( x, y, z )  =  (1/3)( x, y, z )
    //
    //    has divergence 1. Let the vectors
    //
    //       A, B, C
    //
    //    be the vertices of a plate, and let
    //
    //       N = ( B - A )  x  ( C - B )
    //
    //    be an outward normal of the plate. Then
    //
    //       < N, X >
    //
    //    is constant for all X in the plane containing the plate, so we
    //    can pick the vertex A to stand in for X.
    //
    //    Then the surface integral of F over the plate is simply
    //
    //         ^
    //       < N,  A/3 > * plate_area
    //
    //
    //    Equivalence of the methods
    //    --------------------------
    //
    //    If we show that the above integral equals the volume
    //    contribution of the pyramid corresponding to the plate, the
    //    validity of summing the signed pyramid volumes is established.
    //
    //    Below, let
    //
    //       S1, S2 be the plate sides (B-A), (C-B).
    //
    //       N be the outward normal vector S1 x S2.
    //
    //       plate_area be...the area of the plate.
    //
    //    Then
    //
    //       N                  =   S1 x S2
    //                          =   ( B - A ) x ( C - B )
    //                          =   ( B x C ) - ( A x C ) + ( A x B )
    //
    //    and since A is orthogonal to both
    //
    //       A x C
    //       A x B
    //
    //    we have
    //
    //         ^
    //       < N,  A >          =   <  ( B x C )/||N||,  A  >
    //
    //
    //    Then
    //
    //       plate_area         =    (1/2) * ||( S1 x S2 )||
    //                          =    ||N|| / 2
    //
    //    So the surface integral of F over the plate is
    //
    //                        ^
    //       plate_area * ( < N, A/3 > )
    //
    //                          =  (1/6) * ||N||
    //                                   * < (B x C)/||N||, A >
    //
    //                          =  < ( B x C ), A > / 6
    //
    //                          =  < ( A X B ), C > / 6
    //
    //    On the other hand, letting
    //
    //       base_area
    //
    //    denote the area of the triangle defined by A, B, and the
    //    origin, the signed volume of the pyramid defined by A, B, and
    //    C is
    //
    //
    //       1/3 * base_area * height   =  1/3 *  < (A x B)/2,  C  >
    //
    //                                  =  < ( A X B ), C > / 6
    //
    //    This shows the signed pyramid volume is equal to the surface
    //    integral of F over the plate.
    //
    //
    //
    // We proceed to compute the signed volume of each pyramid whose
    // base is a plate and whose vertex is the origin.
    //
    // Note that PLTVOL has already been initialized to 0.D0.
    //
    for I in 1..=NP {
        //
        // Pack the vertices of the current plate into a 3x3 matrix.
        //
        for J in 1..=3 {
            VEQU(VRTCES.subarray([1, PLATES[[J, I]]]), M.subarray_mut([1, J]));
        }

        //
        // The determinant of M gives the volume of the parallelepiped
        // spanned by the origin-vertex vectors. The corresponding
        // pyramid has volume 1/3 * base_area * height, and
        // the area of the pyramid's base is half that of base of the
        // parallelepiped. So the determinant must be divided by 6.
        //
        PLTVOL = (PLTVOL + (DET(M.as_slice()) / 6.0));
    }

    //
    // No check-out required, since the routine is not checked in
    // at this point.
    //
    Ok(PLTVOL)
}
