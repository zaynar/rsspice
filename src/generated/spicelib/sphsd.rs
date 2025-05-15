//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Spherical surface distance
///
/// Return the distance between two points on a sphere, measured
/// along the shortest great circle arc connecting them.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RADIUS     I   Radius of sphere.
///  LON1,
///  LAT1       I   Longitude and latitude of first point in radians.
///  LON2,
///  LAT2       I   Longitude and latitude of second point in radians.
///
///  The function returns the distance between the two input points,
///  measured along the shortest great circle arc connecting them.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RADIUS   is the radius of the sphere on which the points are
///           located.
///
///  LON1,
///  LAT1     are, respectively, the longitude and latitude of the
///           first point. The units are radians.
///
///  LON2,
///  LAT2     are, respectively, the longitude and latitude of the
///           second point. The units are radians.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the distance between the two input points,
///  measured along the shortest great circle arc connecting them.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If RADIUS is negative, the error SPICE(INPUTOUTOFRANGE)
///      is signaled. SPHSD is set to zero. RADIUS may be zero;
///      this case is not treated as an exception.
///
///  2)  Latitudes out of the range [-pi/2, pi/2] are NOT treated
///      as errors, although they are not valid in the latitudinal
///      coordinate system and so may be considered to be exceptional
///      inputs. All latitude values are used in the same way in the
///      computation, regardless of whether or not they are in range.
///      See the code for the equation used.
///
///  3)  Longitudes out of the range (-pi, pi] are NOT treated
///      as errors, although they are not valid in the latitudinal
///      coordinate system and so may be considered to be exceptional
///      inputs. All longitude values are used in the same way in the
///      computation, regardless of whether or not they are in range.
///      See the code for the equation used.
/// ```
///
/// # Particulars
///
/// ```text
///  You may need to consider whether a spherical model is adequate
///  for your application; some bodies may be more accurately modeled
///  by an oblate or prolate spheroid, or by a triaxial ellipsoid.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Find the distance along a sphere of radius 1000 km between
///     the points at
///
///        longitude = pi/2 radians,
///        latitude  = pi/4 radians
///
///     and
///
///        longitude = 0    radians,
///        latitude  = pi/4 radians
///
///
///     Example code begins here.
///
///
///           PROGRAM SPHSD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION        SPHSD
///           DOUBLE PRECISION        PI
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION        DIST
///
///
///           DIST = SPHSD ( 1.D3, PI()/2.D0, PI()/4.D0,
///          .                          0.D0, PI()/4.D0 )
///
///           WRITE(*,*) 'Spherical surface distance:', DIST
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Spherical surface distance:   1047.1975511965979
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 13-AUG-2021 (JDR)
///
///         Changed the input argument names LONG1 and LONG2 to LON1 and
///         LON2 for consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example from existing fragment.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-NOV-1990 (NJB)
/// ```
pub fn sphsd(
    ctx: &mut SpiceContext,
    radius: f64,
    lon1: f64,
    lat1: f64,
    lon2: f64,
    lat2: f64,
) -> crate::Result<f64> {
    let ret = SPHSD(radius, lon1, lat1, lon2, lat2, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SPHSD ( Spherical surface distance )
pub fn SPHSD(
    RADIUS: f64,
    LON1: f64,
    LAT1: f64,
    LON2: f64,
    LAT2: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<f64> {
    let mut SPHSD: f64 = 0.0;
    let mut SL1SL2: f64 = 0.0;
    let mut COSANG: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Check RETURN but do not check in unless an error is detected.
    //
    if RETURN(ctx) {
        SPHSD = 0.0;
        return Ok(SPHSD);
    }

    //
    // Make sure that RADIUS is ok; check in only if it isn't.
    //
    if (RADIUS < 0 as f64) {
        SPHSD = 0.0;

        CHKIN(b"SPHSD", ctx)?;
        SETMSG(b"Radius was #.", ctx);
        ERRDP(b"#", RADIUS, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"SPHSD", ctx)?;
        return Ok(SPHSD);
    }

    //
    // The usual equation for the distance between points, measured
    // along a great circle, is:
    //
    //               -1
    //   DIST  =  COS  (   ( COS(LON1-LON2) * COS(LAT1) * COS(LAT2) )
    //                   + (                  SIN(LAT1) * SIN(LAT2) ) )
    //
    //          * RADIUS
    //
    // To arrive at this equation, we find the cartesian coordinates of
    // the input surface points and take the dot product of the two
    // points.
    //
    // To save a trig function reference, however, we implement this
    // calculation slightly differently.
    //

    //
    // COSANG is the cosine of the angle between the two position
    // vectors.  We bracket COSANG 'tween -1 and 1 to make sure
    // round-off error doesn't take it out of the domain of arc
    // cosine...
    //
    SL1SL2 = (f64::sin(LAT1) * f64::sin(LAT2));

    COSANG = ((f64::cos((LON1 - LON2)) * (f64::cos((LAT1 - LAT2)) - SL1SL2)) + SL1SL2);

    SPHSD = (RADIUS * f64::acos(BRCKTD(COSANG, -1.0, 1.0)));

    Ok(SPHSD)
}
