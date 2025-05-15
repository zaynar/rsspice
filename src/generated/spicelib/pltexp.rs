//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Plate expander
///
/// Expand a triangular plate by a specified amount. The expanded
/// plate is co-planar with, and has the same orientation as, the
/// original. The centroids of the two plates coincide.
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
///  IVERTS     I   Vertices of the plate to be expanded.
///  DELTA      I   Fraction by which the plate is to be expanded.
///  OVERTS     O   Vertices of the expanded plate.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IVERTS   is an array containing three vertices of a triangular
///           plate. Each vertex is a three-dimensional vector. The
///           elements
///
///              IVERTS(J,I), J = 1, 3
///
///           are, respectively, the X, Y, and Z components of the
///           Ith vertex.
///
///
///  DELTA    is a fraction by which the plate is to be scaled.
///           Scaling is done so that the scaled plate has the
///           following properties:
///
///              -  it is co-planar with the input plate
///
///              -  its centroid coincides with that of the input
///                 plate
///
///              -  its sides remain parallel to the corresponding
///                 sides of the input plate
///
///              -  the distance of each vertex from the centroid is
///                 (1+DELTA) times the corresponding distance for
///                 the input plate
/// ```
///
/// # Detailed Output
///
/// ```text
///  OVERTS   is an array containing three vertices of the triangular
///           plate resulting from scaling the input plate.
///
///           If CTROID is the centroid (the average of the vertices)
///           of the input plate, then the Ith vertex of OVERTS
///
///              OVERTS(J,I), J = 1, 3
///
///           is equal to
///
///              CTROID(J) + (1+DELTA)*( IVERTS(J,I) - CTROID(J) ),
///
///              J = 1, 3
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine supports "greedy" ray-plate intercept algorithms.
///  Such algorithms attempt to ensure that false negatives---in which
///  an intersection is not found due to round-off error---do not
///  occur. In such an algorithm, the plate of interest is expanded
///  slightly before the intersection test is performed.
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
///  1) Expand an equilateral triangle that lies in the plane
///
///        { (x,y,z) : z = 7 }
///
///     Use an expansion fraction of 1.D0; this doubles the size of
///     the plate.
///
///     Example code begins here.
///
///
///           PROGRAM PLTEXP_EX1
///           IMPLICIT NONE
///
///           DOUBLE PRECISION      DELTA
///           DOUBLE PRECISION      IVERTS ( 3, 3 )
///           DOUBLE PRECISION      OVERTS ( 3, 3 )
///           DOUBLE PRECISION      S
///
///           INTEGER               I
///
///           S = SQRT(3.D0)/2
///
///           CALL VPACK (    S,  -0.5D0,  7.D0, IVERTS(1,1) )
///           CALL VPACK ( 0.D0,    1.D0,  7.D0, IVERTS(1,2) )
///           CALL VPACK (   -S,  -0.5D0,  7.D0, IVERTS(1,3) )
///
///           DELTA = 1.D0
///
///           CALL PLTEXP ( IVERTS, DELTA, OVERTS )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Vertices of input plate: '
///
///           WRITE (*, '(1X,A,3E18.10)' ) ' I1 = ',
///          .          (IVERTS(I,1), I = 1, 3)
///           WRITE (*, '(1X,A,3E18.10)' ) ' I2 = ',
///          .          (IVERTS(I,2), I = 1, 3)
///           WRITE (*, '(1X,A,3E18.10)' ) ' I3 = ',
///          .          (IVERTS(I,3), I = 1, 3)
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Vertices of output plate: '
///
///           WRITE (*, '(1X,A,3E18.10)' ) ' O1 = ',
///          .          (OVERTS(I,1), I = 1, 3)
///           WRITE (*, '(1X,A,3E18.10)' ) ' O2 = ',
///          .          (OVERTS(I,2), I = 1, 3)
///           WRITE (*, '(1X,A,3E18.10)' ) ' O3 = ',
///          .          (OVERTS(I,3), I = 1, 3)
///           WRITE (*,*) ' '
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Vertices of input plate:
///       I1 =   0.8660254038E+00 -0.5000000000E+00  0.7000000000E+01
///       I2 =   0.0000000000E+00  0.1000000000E+01  0.7000000000E+01
///       I3 =  -0.8660254038E+00 -0.5000000000E+00  0.7000000000E+01
///
///      Vertices of output plate:
///       O1 =   0.1732050808E+01 -0.1000000000E+01  0.7000000000E+01
///       O2 =   0.0000000000E+00  0.2000000000E+01  0.7000000000E+01
///       O3 =  -0.1732050808E+01 -0.1000000000E+01  0.7000000000E+01
///
///
///     Note that the height of the plate is unchanged, but the vectors
///     from the centroid to the vertices have doubled in length.
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
///         Edited the header to comply with NAIF standard.
///
///         Edits to code example output format for the solution to fit
///         within the $Examples section without modifications.
///
/// -    SPICELIB Version 1.0.0, 29-FEB-2016 (NJB)
///
///         Based on original version 28-MAY-2014 (NJB)
/// ```
pub fn pltexp(iverts: &[[f64; 3]; 3], delta: f64, overts: &mut [[f64; 3]; 3]) {
    PLTEXP(iverts.as_flattened(), delta, overts.as_flattened_mut());
}

//$Procedure PLTEXP ( Plate expander )
pub fn PLTEXP(IVERTS: &[f64], DELTA: f64, OVERTS: &mut [f64]) {
    let IVERTS = DummyArray2D::new(IVERTS, 1..=3, 1..=3);
    let mut OVERTS = DummyArrayMut2D::new(OVERTS, 1..=3, 1..=3);
    let mut SCLCTR = StackArray::<f64, 3>::new(1..=3);
    let mut D: f64 = 0.0;
    let mut S: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Compute the centroid of the input vertices. Scale the centroid
    // by DELTA, since we'll only use the scaled form.
    //
    // Unroll all loops to avoid loop overhead.
    //
    D = (DELTA / 3.0);

    SCLCTR[1] = (D * ((IVERTS[[1, 1]] + IVERTS[[1, 2]]) + IVERTS[[1, 3]]));
    SCLCTR[2] = (D * ((IVERTS[[2, 1]] + IVERTS[[2, 2]]) + IVERTS[[2, 3]]));
    SCLCTR[3] = (D * ((IVERTS[[3, 1]] + IVERTS[[3, 2]]) + IVERTS[[3, 3]]));

    //
    // Compute the offsets of the vertices from the centroid CTROID;
    // scale each offset by (1+DELTA). The Ith expanded vertex is
    //
    //    CTROID + (1+DELTA) * ( IVERTS(*,I) - CTROID )
    //
    // which can be re-written as
    //
    //    ( (1+DELTA) * IVERTS(*,I) )  -  ( DELTA * CTROID )
    //
    // or
    //
    //    ( (1+DELTA) * IVERTS(*,I) )  -  SCLCTR
    //
    //
    //
    S = (1.0 + DELTA);

    OVERTS[[1, 1]] = ((S * IVERTS[[1, 1]]) - SCLCTR[1]);
    OVERTS[[2, 1]] = ((S * IVERTS[[2, 1]]) - SCLCTR[2]);
    OVERTS[[3, 1]] = ((S * IVERTS[[3, 1]]) - SCLCTR[3]);

    OVERTS[[1, 2]] = ((S * IVERTS[[1, 2]]) - SCLCTR[1]);
    OVERTS[[2, 2]] = ((S * IVERTS[[2, 2]]) - SCLCTR[2]);
    OVERTS[[3, 2]] = ((S * IVERTS[[3, 2]]) - SCLCTR[3]);

    OVERTS[[1, 3]] = ((S * IVERTS[[1, 3]]) - SCLCTR[1]);
    OVERTS[[2, 3]] = ((S * IVERTS[[2, 3]]) - SCLCTR[2]);
    OVERTS[[3, 3]] = ((S * IVERTS[[3, 3]]) - SCLCTR[3]);
}
