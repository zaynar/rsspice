//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Rectangular coordinates to RA and DEC
///
/// Convert rectangular coordinates to range, right ascension,
/// and declination.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RECTAN     I   Rectangular coordinates of a point.
///  RANGE      O   Distance of the point from the origin.
///  RA         O   Right ascension in radians.
///  DEC        O   Declination in radians.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RECTAN   are the rectangular coordinates of a point.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RANGE    is the distance of the point from the origin.
///
///           The units associated with RANGE are those
///           associated with the input RECTAN.
///
///
///  RA       is the right ascension of RECTAN. This is the angular
///           distance measured toward the east from the prime
///           meridian to the meridian containing the input point.
///           The direction of increasing right ascension is from
///           the +X axis towards the +Y axis.
///
///           RA is output in radians. The range of RA is [0, 2*pi].
///
///
///  DEC      is the declination of RECTAN. This is the angle from
///           the XY plane of the ray from the origin through the
///           point.
///
///           DEC is output in radians. The range of DEC is
///           [-pi/2, pi/2].
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the X and Y components of RECTAN are both zero, the
///      right ascension is set to zero.
///
///  2)  If RECTAN is the zero vector, right ascension and declination
///      are both set to zero.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine returns the range, right ascension, and declination
///  of a point specified in rectangular coordinates.
///
///  The output is defined by a distance from a central reference
///  point, an angle from a reference meridian, and an angle above
///  the equator of a sphere centered at the central reference
///  point.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment converts right ascension and
///  declination from the B1950 reference frame to the J2000 frame.
///
///     C
///     C     Convert RA and DEC to a 3-vector expressed in
///     C     the B1950 frame.
///     C
///           CALL RADREC ( 1.D0, RA, DEC, V1950 )
///     C
///     C     We use the SPICELIB routine PXFORM to obtain the
///     C     transformation  matrix for converting vectors between
///     C     the B1950 and J2000 reference frames. Since
///     C     both frames are inertial, the input time value we
///     C     supply to PXFORM is arbitrary. We choose zero
///     C     seconds past the J2000 epoch.
///     C
///           CALL PXFORM ( 'B1950', 'J2000', 0.D0, MTRANS )
///     C
///     C     Transform the vector to the J2000 frame.
///     C
///           CALL MXV ( MTRANS, V1950, V2000 )
///     C
///     C     Find the RA and DEC of the J2000-relative vector.
///     C
///           CALL RECRAD ( V2000, R, RA, DEC )
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 30-JUL-2003 (NJB) (CHA)
///
///         Various header changes were made to improve clarity. Some
///         minor header corrections were made.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (HAN)
/// ```
pub fn recrad(
    ctx: &mut SpiceContext,
    rectan: &[f64; 3],
    range: &mut f64,
    ra: &mut f64,
    dec: &mut f64,
) {
    RECRAD(rectan, range, ra, dec, ctx.raw_context());
}

//$Procedure RECRAD ( Rectangular coordinates to RA and DEC )
pub fn RECRAD(RECTAN: &[f64], RANGE: &mut f64, RA: &mut f64, DEC: &mut f64, ctx: &mut Context) {
    let RECTAN = DummyArray::new(RECTAN, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Call the subroutine RECLAT to convert the rectangular coordinates
    // into latitudinal coordinates.  In RECLAT, the longitude ( which
    // is returned to this subroutine as RA ) ranges from - pi to pi
    // radians.   Because the right ascension ranges from zero to
    // two pi radians, whenever RA is negative two pi must be added to
    // it.
    //
    RECLAT(RECTAN.as_slice(), RANGE, RA, DEC);

    if (*RA < 0 as f64) {
        *RA = (*RA + TWOPI(ctx));
    }
}
