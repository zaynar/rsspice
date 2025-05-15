//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Inside Tetrahedral Angle
///
/// Determine if a given vector lies inside the solid tetrahedral
/// angle determined by 3 vectors. If it does, return the
/// point where the scale factor such that SCALE*V lies in the
/// plane spanned by E1, E2, and E3.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V          I   Vector to test for "betweenness"
///  E1         I   First edge of the tetrahedral angle
///  E2         I   Second edge of the tetrahedral angle
///  E3         I   Third edge of the tetrahedral angle
///  FOUND      O   Indicates whether V lies in the solid angle
///  SCALE      O   Scale times V is in the triangle E1,E2,E3
/// ```
///
/// # Detailed Input
///
/// ```text
///  V        is a 3-vector. This is the vector to test to see
///           if it lies between the 3 vectors E1, E2 and E3
///
///  E1,
///  E2,
///  E3       are the three edges of a solid tetrahedral angle. (See
///           particulars for a discussion of the solid angle).
/// ```
///
/// # Detailed Output
///
/// ```text
///  FOUND    indicates that V lies inside the solid tetrahedral
///           angle determined by E1, E2 and E3.
///
///
///  SCALE    if V lies inside the solid tetrahedral angle given
///           by E1, E2 and E3, SCALE*V is the point is the positive
///           scalar multiple of V that pierces the triangle
///           determined by the points E1, E2, E3.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If E1, E2 and E3 are not linearly independent, the routine
///      returns .FALSE. SCALE will be set to 0.
///
///  2)  If V is the zero vector, the routine returns .FALSE.
///      SCALE will be set to 0.
/// ```
///
/// # Particulars
///
/// ```text
///  Given 3 linearly independent vectors E1, E2, and E3 the
///  set of vectors a*E1 + b*E2 + c*E3  where a, b, and c
///  are non-negative form a region of space that is a tetrahedral
///  solid angle. If you cut this solid angle with a plane
///  that intersects all three rays from the origin determined
///  by E1, E2 and E3 you will get a tetrahedron (a 4-sided
///  solid with each face a triangle).
///
///  This routine determines whether the ray associated with
///  a vector V lies inside the tetrahedral angle E1,E2,E3.
///  Moreover, if V lies inside this angle, this routine returns
///  the scale factor SCALE such that the point SCALE*V
///  lies in the plane containing the points E1, E2 and E3.
///  This is necessarily a point in the triangle determined by
///  E1, E2 and E3.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you have a triangle in space specified by three
///  vertices P1, P2 and P3 and that an observer at location
///  OBS is looking along the ray emanating from OBS with
///  direction V. Does this ray intersect the triangle
///  P1, P2, P3?  Using this routine, you can answer this
///  question and give the point of intersection if there is
///  one. Here's how.
///
///  First construct the vectors from OBS to the corners of
///  the triangle.
///
///  CALL VSUB ( P1, OBS, E1 )
///  CALL VSUB ( P2, OBS, E2 )
///  CALL VSUB ( P3, OBS, E3 )
///
///  Now see if V lies between the vectors E1, E2, E3 and return
///  the intersection point if it does.
///
///  CALL INSANG ( V, E1, E2, E3, FOUND, SCALE )
///
///  If there was an intersection, add SCALE*V to OBS to get the
///  point of intersection. Otherwise say there was no intersection.
///
///  IF ( FOUND ) THEN
///
///     CALL VLCOM ( 1.0D0, OBS, SCALE, V, POINT )
///
///     WRITE (*,*) 'The ray intersects the triangle at:
///     WRITE (*,*) POINT(1)
///     WRITE (*,*) POINT(2)
///     WRITE (*,*) POINT(3)
///
///  ELSE
///
///     WRITE (*,*) 'There is no intersection.'
///
///  END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine can suffer from extreme loss of precision if the
///      vectors E1, E2, E3 are too long compared to the lengths of the
///      line segments formed by their pairwise differences.
///
///      The user of this routine must ensure that the inputs are
///      suitable.
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
/// -    SPICELIB Version 1.0.3, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 02-FEB-2016 (NJB)
///
///         Fixed comment typos. Updated $Restrictions.
///
/// -    SPICELIB Version 1.0.1, 08-OCT-2009 (NJB)
///
///         Updated header.
///
/// -    SPICELIB Version 1.0.0, 09-JUN-1996 (WLT)
/// ```
pub fn insang(
    v: &[f64; 3],
    e1: &[f64; 3],
    e2: &[f64; 3],
    e3: &[f64; 3],
    found: &mut bool,
    scale: &mut f64,
) {
    INSANG(v, e1, e2, e3, found, scale);
}

//$Procedure INSANG ( Inside Tetrahedral Angle )
pub fn INSANG(V: &[f64], E1: &[f64], E2: &[f64], E3: &[f64], FOUND: &mut bool, SCALE: &mut f64) {
    let V = DummyArray::new(V, 1..=3);
    let E1 = DummyArray::new(E1, 1..=3);
    let E2 = DummyArray::new(E2, 1..=3);
    let E3 = DummyArray::new(E3, 1..=3);
    let mut DENOM: f64 = 0.0;
    let mut EN: f64 = 0.0;
    let mut NORM12 = StackArray::<f64, 3>::new(1..=3);
    let mut NORM23 = StackArray::<f64, 3>::new(1..=3);
    let mut NORM31 = StackArray::<f64, 3>::new(1..=3);
    let mut VN12: f64 = 0.0;
    let mut VN23: f64 = 0.0;
    let mut VN31: f64 = 0.0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //
    //
    // Our initial value for SCALE is zero.  When we have better
    // information, we'll change this.
    //
    *SCALE = 0.0;

    //
    // First we construct a normal to the plane spanned by E1 and E2
    // and make sure that we don't get a zero vector.  If we
    // get the zero vector, E1 and E2 are linearly dependent so we
    // set the value of FOUND to FALSE and return.
    //
    VCRSS(E1.as_slice(), E2.as_slice(), NORM12.as_slice_mut());

    //
    // First make sure V and E3 are in the same half space
    // bounded by E1 and E2.  If they are not, we can return.
    //
    VN12 = VDOT(V.as_slice(), NORM12.as_slice());
    EN = VDOT(E3.as_slice(), NORM12.as_slice());
    //
    // Determine whether NORML and E3 are perpendicular.  If they
    // are perpendicular, E3 is a linear combination of E1 and E2.
    // In this case set FOUND to FALSE and return.
    //
    if (EN == 0.0) {
        *FOUND = false;
        return;
    }
    //
    // Now check to see if V and E3 are in the same half space.  If
    // not, we can stop and return the value FALSE.
    //
    if ((EN > 0.0) && (VN12 < 0.0)) {
        *FOUND = false;
        return;
    } else if ((EN < 0.0) && (VN12 > 0.0)) {
        *FOUND = false;
        return;
    }

    //
    // Now check that V and E1 are on the same side of the plane
    // spanned by E2 and E3.  Note we don't have to compute EN
    // again <( E2 x E3 ), E1 >  because of the vector identity
    //
    //   < (E1 x E2), E3 > =  < (E2 x E3), E1 > = < (E3 x E1), E2 >
    //
    VCRSS(E2.as_slice(), E3.as_slice(), NORM23.as_slice_mut());
    VN23 = VDOT(V.as_slice(), NORM23.as_slice());

    //
    // The following tests are the same as in the previous case.
    //
    if ((EN > 0.0) && (VN23 < 0.0)) {
        *FOUND = false;
        return;
    } else if ((EN < 0.0) && (VN23 > 0.0)) {
        *FOUND = false;
        return;
    }

    //
    // Finally check to see if V and E2 are in the same half space
    // bounded by E3 and E2
    //
    VCRSS(E3.as_slice(), E1.as_slice(), NORM31.as_slice_mut());
    VN31 = VDOT(V.as_slice(), NORM31.as_slice());

    if ((EN > 0.0) && (VN31 < 0.0)) {
        *FOUND = false;
        return;
    } else if ((EN < 0.0) && (VN31 > 0.0)) {
        *FOUND = false;
        return;
    }

    //
    // If you get this far, we know that V is lies in the intersection
    // of the half spaces determined by the various combinations of
    // E1, E2 and E3.
    //
    *FOUND = true;

    //
    // Now find the intersection. First get a normal to the triangle.
    // One way to get the normal is to find the vector cross
    // product
    //
    //   NORML = ( E2 - E1 ) x ( E3 - E1 )
    //
    // However, this can be rewritten as:
    //
    //    NORML = E2 x E3 - E1 x E3 - E2 x E1 + E1 x E1
    //
    //          = E2 x E3 + E3 x E1 + E1 x E2
    //
    // But we already have the three components E2 x E3, ... etc.
    // in the vectors NORM12, NORM23, NORM31
    //
    // Now we need to find the scalar multiple t*V such that
    //
    //    < tV - E1, NORML > = 0
    //
    // But this can be rewritten as:
    //
    //    t < V, NORML > = < E1, NORML >
    //
    // Solving for t yields
    //
    //  t = < E1, NORML > / < V, NORML >
    //
    //    = < E1, E1xE2 + E2xE3 + E3xE1 > / <  V, E1xE2 + E2xE3 + E3xE1 >
    //
    //    = ( 0 + <E1, E2xE3> + 0 ) / (<V,E1xE2> + <V,E2xE3> + <V,E3xE1>)
    //
    //    =  EN / ( VN12 + VN23 + VN31 )
    //
    DENOM = ((VN12 + VN23) + VN31);

    if (DENOM == 0.0) {
        *FOUND = false;
    } else {
        *FOUND = true;
        *SCALE = (EN / DENOM);
    }
}
