//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cylindrical to latitudinal
///
/// Convert from cylindrical to latitudinal coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  R          I   Distance of point from Z axis.
///  CLON       I   Cylindrical angle of point from XZ plane(radians).
///  Z          I   Height of point above XY plane.
///  RADIUS     O   Distance of point from origin.
///  LON        O   Longitude of point (radians).
///  LAT        O   Latitude of point (radians).
/// ```
///
/// # Detailed Input
///
/// ```text
///  R        is the distance of the input point from Z axis.
///
///  CLON     is the cylindrical angle of the point from XZ plane
///           (radians).
///
///  Z        is the height of the point above XY plane.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RADIUS   is the distance of the input point from origin.
///
///  LON      is the longitude (i.e. angle from the XZ plane) of
///           the input point (radians). LON is set equal to CLON.
///
///  LAT      is the latitude (i.e. angle above the XY plane) of the
///           input point (radians). The range of LAT is [-pi, pi].
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
///  This routine converts coordinates given in cylindrical
///  coordinates to coordinates in latitudinal coordinates.
///
///  Latitudinal coordinates are defined by a distance from a central
///  reference point, an angle from a reference meridian, and an angle
///  above the equator of a sphere centered at the central reference
///  point.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Compute the cylindrical coordinates of the position of the
///     Moon as seen from the Earth, and convert them to latitudinal
///     and rectangular coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: cyllat_ex1.tm
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
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM CYLLAT_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,F20.8)' )
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      CLON
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      Z
///
///     C
///     C     Load SPK and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'cyllat_ex1.tm' )
///
///     C
///     C     Look up the geometric state of the Moon as seen from
///     C     the Earth at 2017 Mar 20, relative to the J2000
///     C     reference frame.
///     C
///           CALL STR2ET ( '2017 Mar 20', ET )
///
///           CALL SPKPOS ( 'Moon',  ET,  'J2000', 'NONE',
///          .              'Earth', POS, LT               )
///
///     C
///     C     Convert the position vector POS to cylindrical
///     C     coordinates.
///     C
///           CALL RECCYL ( POS, R, CLON, Z )
///
///     C
///     C     Convert the cylindrical coordinates to latitudinal.
///     C
///           CALL CYLLAT ( R, CLON, Z, RADIUS, LON, LAT )
///
///     C
///     C     Convert the latitudinal coordinates to rectangular.
///     C
///           CALL LATREC ( RADIUS, LON, LAT, RECTAN )
///
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Original rectangular coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X          (km): ', POS(1)
///           WRITE(*,FMT1) '  Y          (km): ', POS(2)
///           WRITE(*,FMT1) '  Z          (km): ', POS(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Cylindrical coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius     (km): ', R
///           WRITE(*,FMT1) '  Longitude (deg): ', CLON*DPR()
///           WRITE(*,FMT1) '  Z          (km): ', Z
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Latitudinal coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius     (km): ', RADIUS
///           WRITE(*,FMT1) '  Longitude (deg): ', LON*DPR()
///           WRITE(*,FMT1) '  Latitude  (deg): ', LAT*DPR()
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates from LATREC:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X          (km): ', RECTAN(1)
///           WRITE(*,FMT1) '  Y          (km): ', RECTAN(2)
///           WRITE(*,FMT1) '  Z          (km): ', RECTAN(3)
///           WRITE(*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Original rectangular coordinates:
///
///       X          (km):      -55658.44323296
///       Y          (km):     -379226.32931475
///       Z          (km):     -126505.93063865
///
///      Cylindrical coordinates:
///
///       Radius     (km):      383289.01777726
///       Longitude (deg):         261.65040211
///       Z          (km):     -126505.93063865
///
///      Latitudinal coordinates:
///
///       Radius     (km):      403626.33912495
///       Longitude (deg):         261.65040211
///       Latitude  (deg):         -18.26566077
///
///      Rectangular coordinates from LATREC:
///
///       X          (km):      -55658.44323296
///       Y          (km):     -379226.32931475
///       Z          (km):     -126505.93063865
///
///
///  2) Create a table showing a variety of cylindrical coordinates
///     and the corresponding latitudinal coordinates.
///
///     Corresponding latitudinal and cylindrical coordinates are
///     listed to three decimal places. All input and output angles
///     are in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM CYLLAT_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      RPD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NREC
///           PARAMETER           ( NREC = 11 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      CLON   ( NREC )
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      R      ( NREC )
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RCLON
///           DOUBLE PRECISION      Z      ( NREC )
///
///           INTEGER               I
///
///     C
///     C     Define the input cylindrical coordinates. Angles
///     C     in degrees.
///     C
///
///           DATA                 R    /  0.D0, 1.D0, 1.D0,
///          .                             0.D0, 1.D0, 1.D0,
///          .                             0.D0, 1.D0, 1.D0,
///          .                             0.D0, 0.D0           /
///
///           DATA                 CLON /  0.D0,   0.D0,  90.D0,
///          .                             0.D0, 180.D0, -90.D0,
///          .                             0.D0,  45.D0, 180.D0,
///          .                           180.D0,  33.D0         /
///
///           DATA                 Z    /  0.D0,   0.D0,  0.D0,
///          .                             1.D0,   1.D0,  0.D0,
///          .                            -1.D0,   0.D0, -1.D0,
///          .                             1.D0,   0.D0         /
///
///     C
///     C     Print the banner.
///     C
///           WRITE(*,*) '    R       CLON      Z    '
///          . //        '  RADIUS     LON     LAT   '
///           WRITE(*,*) ' -------  -------  ------- '
///          . //        ' -------  -------  ------- '
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           DO I = 1, NREC
///
///              RCLON = CLON(I) * RPD()
///
///              CALL CYLLAT( R(I), RCLON, Z(I), RADIUS, LON, LAT )
///
///              WRITE (*,'(6F9.3)') R(I), CLON(I), Z(I),
///          .                       RADIUS, LON * DPR(), LAT * DPR()
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///          R       CLON      Z      RADIUS     LON     LAT
///       -------  -------  -------  -------  -------  -------
///         0.000    0.000    0.000    0.000    0.000    0.000
///         1.000    0.000    0.000    1.000    0.000    0.000
///         1.000   90.000    0.000    1.000   90.000    0.000
///         0.000    0.000    1.000    1.000    0.000   90.000
///         1.000  180.000    1.000    1.414  180.000   45.000
///         1.000  -90.000    0.000    1.000  -90.000    0.000
///         0.000    0.000   -1.000    1.000    0.000  -90.000
///         1.000   45.000    0.000    1.000   45.000    0.000
///         1.000  180.000   -1.000    1.414  180.000  -45.000
///         0.000  180.000    1.000    1.000  180.000   90.000
///         0.000   33.000    0.000    0.000   33.000    0.000
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Changed the argument names LONGC and LONG to CLON and LON for
///         consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete code examples.
///
/// -    SPICELIB Version 1.0.3, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
///
/// -    SPICELIB Version 1.0.2, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn cyllat(r: f64, clon: f64, z: f64, radius: &mut f64, lon: &mut f64, lat: &mut f64) {
    CYLLAT(r, clon, z, radius, lon, lat);
}

//$Procedure CYLLAT ( Cylindrical to latitudinal )
pub fn CYLLAT(R: f64, CLON: f64, Z: f64, RADIUS: &mut f64, LON: &mut f64, LAT: &mut f64) {
    let mut BIG: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut RHO: f64 = 0.0;
    let mut LATTUD: f64 = 0.0;

    //
    // Local Variables
    //

    //
    // Convert the input cylindrical coordinates to latitudinal
    // coordinates, storing in temporary variables.
    //
    BIG = intrinsics::DMAX1(&[f64::abs(R), f64::abs(Z)]);

    if (BIG > 0 as f64) {
        X = (R / BIG);
        Y = (Z / BIG);
        RHO = (BIG * f64::sqrt(((X * X) + (Y * Y))));
    } else {
        RHO = 0.0;
    }

    if (RHO == 0.0) {
        LATTUD = 0.0;
    } else {
        LATTUD = f64::atan2(Z, R);
    }
    //
    // Move results to output variables
    //
    *LON = CLON;
    *RADIUS = RHO;
    *LAT = LATTUD;
    //
}
