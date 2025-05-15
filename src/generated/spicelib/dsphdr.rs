//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivative of spherical w.r.t. rectangular
///
/// Compute the Jacobian matrix of the transformation from
/// rectangular to spherical coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   X-coordinate of point.
///  Y          I   Y-coordinate of point.
///  Z          I   Z-coordinate of point.
///  JACOBI     O   Matrix of partial derivatives.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X,
///  Y,
///  Z        are the rectangular coordinates of the point at
///           which the Jacobian of the map from rectangular
///           to spherical coordinates is desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  JACOBI   is the matrix of partial derivatives of the conversion
///           between rectangular and spherical coordinates. It
///           has the form
///
///              .-                                  -.
///              |  DR/DX       DR/DY      DR/DZ      |
///              |  DCOLAT/DX   DCOLAT/DY  DCOLAT/DZ  |
///              |  DLONG/DX    DLONG/DY   DLONG/DZ   |
///              `-                                  -'
///
///           evaluated at the input values of X, Y, and Z.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input point is on the Z-axis (X and Y = 0), the
///      Jacobian is undefined, the error SPICE(POINTONZAXIS) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  When performing vector calculations with velocities it is
///  usually most convenient to work in rectangular coordinates.
///  However, once the vector manipulations have been performed
///  it is often desirable to convert the rectangular representations
///  into spherical coordinates to gain insights about phenomena
///  in this coordinate frame.
///
///  To transform rectangular velocities to derivatives of coordinates
///  in a spherical system, one uses the Jacobian of the
///  transformation between the two systems.
///
///  Given a state in rectangular coordinates
///
///       ( x, y, z, dx, dy, dz )
///
///  the corresponding spherical coordinate derivatives are given by
///  the matrix equation:
///
///                        t          |                    t
///     (dr, dcolat, dlong)   = JACOBI|      * (dx, dy, dz)
///                                   |(x,y,z)
///
///  This routine computes the matrix
///
///           |
///     JACOBI|
///           |(x, y, z)
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
///  1) Find the spherical state of the Earth as seen from
///     Mars in the IAU_MARS reference frame at January 1, 2005 TDB.
///     Map this state back to rectangular coordinates as a check.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: dsphdr_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                     Contents
///           ---------                     --------
///           de421.bsp                     Planetary ephemeris
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00010.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM DSPHDR_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,E18.8)' )
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      COLAT
///           DOUBLE PRECISION      DRECTN ( 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      JACOBI ( 3, 3 )
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      SPHVEL ( 3 )
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      SLON
///           DOUBLE PRECISION      STATE  ( 6 )
///
///     C
///     C     Load SPK, PCK and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'dsphdr_ex1.tm' )
///
///     C
///     C     Look up the apparent state of earth as seen from Mars at
///     C     January 1, 2005 TDB, relative to the IAU_MARS reference
///     C     frame.
///     C
///           CALL STR2ET ( 'January 1, 2005 TDB', ET )
///
///           CALL SPKEZR ( 'Earth', ET,    'IAU_MARS', 'LT+S',
///          .              'Mars',  STATE, LT                )
///
///     C
///     C     Convert position to spherical coordinates.
///     C
///           CALL RECSPH ( STATE, R, COLAT, SLON )
///
///     C
///     C     Convert velocity to spherical coordinates.
///     C
///
///           CALL DSPHDR ( STATE(1), STATE(2), STATE(3), JACOBI )
///
///           CALL MXV ( JACOBI, STATE(4), SPHVEL )
///
///     C
///     C     As a check, convert the spherical state back to
///     C     rectangular coordinates.
///     C
///           CALL SPHREC ( R, COLAT, SLON, RECTAN )
///
///           CALL DRDSPH ( R, COLAT, SLON, JACOBI )
///
///           CALL MXV ( JACOBI, SPHVEL, DRECTN )
///
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X (km)                  = ', STATE(1)
///           WRITE(*,FMT1) '  Y (km)                  = ', STATE(2)
///           WRITE(*,FMT1) '  Z (km)                  = ', STATE(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular velocity:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  dX/dt (km/s)            = ', STATE(4)
///           WRITE(*,FMT1) '  dY/dt (km/s)            = ', STATE(5)
///           WRITE(*,FMT1) '  dZ/dt (km/s)            = ', STATE(6)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Spherical coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius     (km)         = ', R
///           WRITE(*,FMT1) '  Colatitude (deg)        = ',
///          .                                             COLAT/RPD()
///           WRITE(*,FMT1) '  Longitude  (deg)        = ', SLON/RPD()
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Spherical velocity:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  d Radius/dt     (km/s)  = ', SPHVEL(1)
///           WRITE(*,FMT1) '  d Colatitude/dt (deg/s) = ',
///          .                                        SPHVEL(2)/RPD()
///           WRITE(*,FMT1) '  d Longitude/dt  (deg/s) = ',
///          .                                        SPHVEL(3)/RPD()
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates from inverse ' //
///          .           'mapping:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X (km)                  = ', RECTAN(1)
///           WRITE(*,FMT1) '  Y (km)                  = ', RECTAN(2)
///           WRITE(*,FMT1) '  Z (km)                  = ', RECTAN(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular velocity from inverse mapping:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  dX/dt (km/s)            = ', DRECTN(1)
///           WRITE(*,FMT1) '  dY/dt (km/s)            = ', DRECTN(2)
///           WRITE(*,FMT1) '  dZ/dt (km/s)            = ', DRECTN(3)
///           WRITE(*,*) ' '
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Rectangular coordinates:
///
///       X (km)                  =    -0.76096183E+08
///       Y (km)                  =     0.32436380E+09
///       Z (km)                  =     0.47470484E+08
///
///      Rectangular velocity:
///
///       dX/dt (km/s)            =     0.22952075E+05
///       dY/dt (km/s)            =     0.53760111E+04
///       dZ/dt (km/s)            =    -0.20881149E+02
///
///      Spherical coordinates:
///
///       Radius     (km)         =     0.33653522E+09
///       Colatitude (deg)        =     0.81891013E+02
///       Longitude  (deg)        =     0.10320290E+03
///
///      Spherical velocity:
///
///       d Radius/dt     (km/s)  =    -0.11211601E+02
///       d Colatitude/dt (deg/s) =     0.33189930E-05
///       d Longitude/dt  (deg/s) =    -0.40539288E-02
///
///      Rectangular coordinates from inverse mapping:
///
///       X (km)                  =    -0.76096183E+08
///       Y (km)                  =     0.32436380E+09
///       Z (km)                  =     0.47470484E+08
///
///      Rectangular velocity from inverse mapping:
///
///       dX/dt (km/s)            =     0.22952075E+05
///       dY/dt (km/s)            =     0.53760111E+04
///       dZ/dt (km/s)            =    -0.20881149E+02
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.0, 19-JUL-2001 (WLT)
/// ```
pub fn dsphdr(
    ctx: &mut SpiceContext,
    x: f64,
    y: f64,
    z: f64,
    jacobi: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    DSPHDR(x, y, z, jacobi.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSPHDR ( Derivative of spherical w.r.t. rectangular )
pub fn DSPHDR(
    X: f64,
    Y: f64,
    Z: f64,
    JACOBI: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut JACOBI = DummyArrayMut2D::new(JACOBI, 1..=3, 1..=3);
    let mut RECTAN = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut COLAT: f64 = 0.0;
    let mut LONG: f64 = 0.0;
    let mut INJACB = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

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
        CHKIN(b"DSPHDR", ctx)?;
    }

    //
    // There is a singularity of the jacobian for points on the z-axis.
    //
    if ((X == 0 as f64) && (Y == 0 as f64)) {
        SETMSG(b"The Jacobian of the transformation from rectangular to spherical coordinates is not defined for points on the z-axis.", ctx);
        SIGERR(b"SPICE(POINTONZAXIS)", ctx)?;
        CHKOUT(b"DSPHDR", ctx)?;
        return Ok(());
    }

    //
    // We will get the Jacobian of the transformation from rectangular
    // to spherical coordinates by implicit differentiation.
    //
    // First move the X,Y and Z coordinates into a vector.
    //
    VPACK(X, Y, Z, RECTAN.as_slice_mut());

    //
    // Convert from rectangular to spherical coordinates.
    //
    RECSPH(RECTAN.as_slice(), &mut R, &mut COLAT, &mut LONG);

    //
    // Get the Jacobian of the transformation from spherical to
    // rectangular coordinates at R, COLAT, LONG.
    //
    DRDSPH(R, COLAT, LONG, INJACB.as_slice_mut());

    //
    // Now invert INJACB to get the Jacobian of the transformation from
    // rectangular to spherical coordinates.
    //
    INVORT(INJACB.as_slice(), JACOBI.as_slice_mut(), ctx)?;

    CHKOUT(b"DSPHDR", ctx)?;
    Ok(())
}
