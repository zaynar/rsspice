//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Latitudinal to spherical coordinates
///
/// Convert from latitudinal coordinates to spherical coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RADIUS     I   Distance of a point from the origin.
///  LON        I   Angle of the point from the XZ plane in radians.
///  LAT        I   Angle of the point from the XY plane in radians.
///  RHO        O   Distance of the point from the origin.
///  COLAT      O   Angle of the point from positive Z axis (radians).
///  SLON       O   Angle of the point from the XZ plane (radians).
/// ```
///
/// # Detailed Input
///
/// ```text
///  RADIUS   is the distance of a point from the origin.
///
///  LON      is the angle of the point from the XZ plane in
///           radians.
///
///  LAT      is the angle of the point from the XY plane in
///           radians.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RHO      is the distance of the point from the origin.
///
///  COLAT    is the angle between the vector from the origin to the
///           point and the positive Z axis in radians. COLAT is
///           computed as PI/2 - LAT.
///
///  SLON     is the angle of the point from the XZ plane (radians).
///           SLON is set equal to LON.
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
///  This routine returns the spherical coordinates of a point
///  whose position is input in latitudinal coordinates.
///
///  Latitudinal coordinates are defined by a distance from a central
///  reference point, an angle from a reference meridian, and an angle
///  above the equator of a sphere centered at the central reference
///  point.
///
///  Spherical coordinates are defined by a distance from a central
///  reference point, an angle from a reference meridian, and an angle
///  from the z-axis.
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
///  1) Co-latitude is obtained by subtracting latitude from HALFPI()
///     Radius and longitude mean the same thing in both latitudinal
///     and spherical coordinates. The table below lists LAT and
///     corresponding COLAT in terms of degrees.
///
///          LAT     COLAT
///         -----    -----
///            0        90
///           20        70
///           45        45
///          -30       120
///           90         0
///          -45       135
///
///
///  2) Compute the latitudinal coordinates of the position of the Moon
///     as seen from the Earth, and convert them to spherical and
///     rectangular coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: latsph_ex2.tm
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
///           PROGRAM LATSPH_EX2
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
///           DOUBLE PRECISION      COLAT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      SLON
///
///     C
///     C     Load SPK and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'latsph_ex2.tm' )
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
///     C     Convert the position vector POS to latitudinal
///     C     coordinates.
///     C
///           CALL RECLAT ( POS, RADIUS, LON, LAT )
///
///     C
///     C     Convert the latitudinal coordinates to spherical.
///     C
///           CALL LATSPH ( RADIUS, LON, LAT, R, COLAT, SLON )
///
///     C
///     C     Convert the spherical coordinates to rectangular.
///     C
///           CALL SPHREC ( R, COLAT, SLON, RECTAN )
///
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Original rectangular coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X           (km): ', POS(1)
///           WRITE(*,FMT1) '  Y           (km): ', POS(2)
///           WRITE(*,FMT1) '  Z           (km): ', POS(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Latitudinal coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius      (km): ', RADIUS
///           WRITE(*,FMT1) '  Longitude  (deg): ', LON*DPR()
///           WRITE(*,FMT1) '  Latitude   (deg): ', LAT*DPR()
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Spherical coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius      (km): ', R
///           WRITE(*,FMT1) '  Colatitude (deg): ', COLAT*DPR()
///           WRITE(*,FMT1) '  Longitude  (deg): ', SLON*DPR()
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Rectangular coordinates from SPHREC:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X           (km): ', RECTAN(1)
///           WRITE(*,FMT1) '  Y           (km): ', RECTAN(2)
///           WRITE(*,FMT1) '  Z           (km): ', RECTAN(3)
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
///       X           (km):      -55658.44323296
///       Y           (km):     -379226.32931475
///       Z           (km):     -126505.93063865
///
///      Latitudinal coordinates:
///
///       Radius      (km):      403626.33912495
///       Longitude  (deg):         -98.34959789
///       Latitude   (deg):         -18.26566077
///
///      Spherical coordinates:
///
///       Radius      (km):      403626.33912495
///       Colatitude (deg):         108.26566077
///       Longitude  (deg):         -98.34959789
///
///      Rectangular coordinates from SPHREC:
///
///       X           (km):      -55658.44323296
///       Y           (km):     -379226.32931475
///       Z           (km):     -126505.93063865
///
///
///  3) Create a table showing a variety of latitudinal coordinates
///     and the corresponding spherical coordinates.
///
///     Corresponding latitudinal and spherical coordinates are
///     listed to three decimal places. Input and output angles are
///     in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM LATSPH_EX3
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
///           DOUBLE PRECISION      COLAT
///           DOUBLE PRECISION      LAT    ( NREC )
///           DOUBLE PRECISION      LON    ( NREC )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RADIUS ( NREC )
///           DOUBLE PRECISION      RLAT
///           DOUBLE PRECISION      RLON
///           DOUBLE PRECISION      SLON
///
///           INTEGER               I
///
///     C
///     C     Define the input latitudinal coordinates. Angles in
///     C     degrees.
///     C
///
///           DATA                 RADIUS / 0.D0, 1.D0,     1.D0,
///          .                              1.D0, 1.4142D0, 1.D0,
///          .                              1.D0, 1.D0,     1.4142D0,
///          .                              1.D0, 0.D0               /
///
///           DATA                 LON   /  0.D0,    0.D0,  90.D0,
///          .                              0.D0,  180.D0, -90.D0,
///          .                              0.D0,   45.D0, 180.D0,
///          .                             180.D0,  33.D0            /
///
///           DATA                 LAT   / 90.D0,    0.D0,   0.D0,
///          .                             90.D0,   45.D0,   0.D0,
///          .                            -90.D0,    0.D0, -45.D0,
///          .                             90.D0,    0.D0            /
///
///     C
///     C     Print the banner.
///     C
///           WRITE(*,*) '  RADIUS    LON      LAT   '
///          . //        '     R     COLAT     SLON  '
///           WRITE(*,*) ' -------  -------  ------- '
///          . //        ' -------  -------  ------- '
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           DO I = 1, NREC
///
///              RLON = LON(I) * RPD()
///              RLAT = LAT(I) * RPD()
///
///              CALL LATSPH( RADIUS(I), RLON, RLAT, R, COLAT, SLON )
///
///              WRITE (*,'(6F9.3)') RADIUS(I), LON(I), LAT(I),
///          .                       R, COLAT * DPR(), SLON * DPR()
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
///        RADIUS    LON      LAT        R     COLAT     SLON
///       -------  -------  -------  -------  -------  -------
///         0.000    0.000   90.000    0.000    0.000    0.000
///         1.000    0.000    0.000    1.000   90.000    0.000
///         1.000   90.000    0.000    1.000   90.000   90.000
///         1.000    0.000   90.000    1.000    0.000    0.000
///         1.414  180.000   45.000    1.414   45.000  180.000
///         1.000  -90.000    0.000    1.000   90.000  -90.000
///         1.000    0.000  -90.000    1.000  180.000    0.000
///         1.000   45.000    0.000    1.000   90.000   45.000
///         1.414  180.000  -45.000    1.414  135.000  180.000
///         1.000  180.000   90.000    1.000    0.000  180.000
///         0.000   33.000    0.000    0.000   90.000   33.000
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 04-JUL-2021 (JDR)
///
///         Changed the argument names LONG and LONGS to LON and SLON for
///         consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete code examples.
///
/// -    SPICELIB Version 1.0.2, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn latsph(
    ctx: &mut SpiceContext,
    radius: f64,
    lon: f64,
    lat: f64,
    rho: &mut f64,
    colat: &mut f64,
    slon: &mut f64,
) {
    LATSPH(radius, lon, lat, rho, colat, slon, ctx.raw_context());
}

//$Procedure LATSPH ( Latitudinal to spherical coordinates )
pub fn LATSPH(
    RADIUS: f64,
    LON: f64,
    LAT: f64,
    RHO: &mut f64,
    COLAT: &mut f64,
    SLON: &mut f64,
    ctx: &mut Context,
) {
    let mut TH: f64 = 0.0;
    let mut PH: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Convert to spherical coordinates, storing the results in
    // temporary variables
    //
    TH = (HALFPI(ctx) - LAT);
    PH = LON;

    //
    // Move results to output variables
    //
    *RHO = RADIUS;
    *COLAT = TH;
    *SLON = PH;
}
