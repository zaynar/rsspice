//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Compute area of plate set
///
/// Compute the total area of a collection of triangular plates.
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
///  The function returns the total area of the set of plates.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NV       is the number of vertices comprising the plate
///           set.
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
///  NP       is the number of triangular plates comprising the
///           plate set.
///
///  PLATES   is an array containing 3-tuples of integers
///           representing the set of plates. The elements of
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
///  The function returns the total area of the input set of plates.
///  Each plate contributes the area of the triangle defined by the
///  plate's vertices.
///
///  If the components of the vertex array have length unit L, then the
///  output area has units
///
///      2
///     L
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of plates is less than 0, the error
///      SPICE(BADPLATECOUNT) is signaled.
///
///  2)  If the number of plates is positive and the number of vertices
///      is less than 3, the error SPICE(TOOFEWVERTICES) is signaled.
///
///  3)  If any plate contains a vertex index outside of the range
///
///         [1, NV]
///
///      the error SPICE(INDEXOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the total area of a set of triangular
///  plates. The plates need not define a closed surface.
///
///  Examples of valid plate sets:
///
///     Tetrahedron
///     Box
///     Tiled ellipsoid
///     Tiled ellipsoid with one plate removed
///     Two disjoint boxes
///     Two boxes with intersection having positive volume
///     Single plate
///     Empty plate set
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
///  1) Compute the area of the pyramid defined by the four
///     triangular plates whose vertices are the 3-element
///     subsets of the set of vectors:
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
///           PROGRAM PLTAR_EX1
///           IMPLICIT NONE
///     C
///     C     Compute the area of a plate model representing the
///     C     pyramid with one vertex at the origin and the other
///     C     vertices coinciding with the standard basis vectors.
///     C
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      PLTAR
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
///           DOUBLE PRECISION      AREA
///
///           INTEGER               PLATES ( 3, NPLATE )
///     C
///     C     Initial values
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
///           AREA = PLTAR ( NVERT, VRTCES, NPLATE, PLATES )
///
///           WRITE (*,*) 'Expected area   =    (3 + SQRT(3)) / 2'
///           WRITE (*,*) '                =    0.2366025403784438E+01'
///           WRITE (*,*) 'Computed area   = ', AREA
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Expected area   =    (3 + SQRT(3)) / 2
///                      =    0.2366025403784438E+01
///      Computed area   =    2.3660254037844384
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
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 21-OCT-2016 (NJB)
///
///         Original version 25-MAR-2016 (NJB)
/// ```
pub fn pltar(
    ctx: &mut SpiceContext,
    nv: i32,
    vrtces: &[[f64; 3]],
    np: i32,
    plates: &[[i32; 3]],
) -> crate::Result<f64> {
    let ret = PLTAR(
        nv,
        vrtces.as_flattened(),
        np,
        plates.as_flattened(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure PLTAR ( Compute area of plate set )
pub fn PLTAR(
    NV: i32,
    VRTCES: &[f64],
    NP: i32,
    PLATES: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..=NV);
    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..=NP);
    let mut PLTAR: f64 = 0.0;
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE1 = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE2 = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // The function must have an initial value.
    //
    PLTAR = 0.0;

    //
    // This routine uses discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(PLTAR);
    }

    //
    // Check the vertex and plate counts.
    //
    if (NP < 0) {
        CHKIN(b"PLTAR", ctx)?;
        SETMSG(b"Plate count must be non-negative but NP = #.", ctx);
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(BADPLATECOUNT)", ctx)?;
        CHKOUT(b"PLTAR", ctx)?;
        return Ok(PLTAR);
    }

    if (NP == 0) {
        //
        // The area has already been set to zero.
        //
        return Ok(PLTAR);
    }

    if (NV < 3) {
        CHKIN(b"PLTAR", ctx)?;
        SETMSG(b"At least 3 vertices are needed, but NV = #.", ctx);
        ERRINT(b"#", NV, ctx);
        SIGERR(b"SPICE(TOOFEWVERTICES)", ctx)?;
        CHKOUT(b"PLTAR", ctx)?;
        return Ok(PLTAR);
    }

    //
    // Make sure the vertex indices are in the range [1, NV].
    //
    for I in 1..=NP {
        for J in 1..=3 {
            if ((PLATES[[J, I]] < 1) || (PLATES[[J, I]] > NV)) {
                CHKIN(b"PLTAR", ctx)?;
                SETMSG(b"Vertex indices must be in the range [1, NV] for all SPICE language versions. The input value of NV was #. Vertex index # in plate # was #. (The vertex and plate numbers in this message are 1-based as well.)", ctx);
                ERRINT(b"#", NV, ctx);
                ERRINT(b"#", J, ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", PLATES[[J, I]], ctx);
                SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
                CHKOUT(b"PLTAR", ctx)?;
                return Ok(PLTAR);
            }
        }
    }

    for I in 1..=NP {
        //
        // Take the cross product of two edges of the Ith plate.
        //
        VSUB(
            VRTCES.subarray([1, PLATES[[2, I]]]),
            VRTCES.subarray([1, PLATES[[1, I]]]),
            EDGE1.as_slice_mut(),
        );

        VSUB(
            VRTCES.subarray([1, PLATES[[3, I]]]),
            VRTCES.subarray([1, PLATES[[2, I]]]),
            EDGE2.as_slice_mut(),
        );

        VCRSS(EDGE1.as_slice(), EDGE2.as_slice(), CP.as_slice_mut());

        //
        // The plate area is 1/2 of the magnitude of the
        // cross product.
        //
        PLTAR = (PLTAR + (0.5 * VNORM(CP.as_slice())));
    }

    //
    // No check-out required, since the routine is not checked in
    // at this point.
    //
    Ok(PLTAR)
}
