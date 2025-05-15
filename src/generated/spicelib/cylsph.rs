//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cylindrical to spherical
///
/// Convert from cylindrical to spherical coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  R          I   Distance of point from Z axis.
///  CLON       I   Angle (radians) of point from XZ plane.
///  Z          I   Height of point above XY plane.
///  RADIUS     O   Distance of point from origin.
///  COLAT      O   Polar angle (co-latitude in radians) of point.
///  SLON       O   Azimuthal angle (longitude) of point (radians).
/// ```
///
/// # Detailed Input
///
/// ```text
///  R        is the distance of the point of interest from Z axis.
///
///  CLON     is the cylindrical angle (radians) of the point from
///           the XZ plane.
///
///  Z        is the height of the point above XY plane.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RADIUS   is the distance of the point from origin.
///
///  COLAT    is the polar angle (co-latitude in radians) of the
///           point. The range of COLAT is [-pi, pi].
///
///  SLON     is the azimuthal angle (longitude) of the point
///           (radians). SLON is set equal to CLON.
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
///  This returns the spherical coordinates of a point whose position
///  is input through cylindrical coordinates.
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
///     Moon as seen from the Earth, and convert them to spherical
///     and rectangular coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: cylsph_ex1.tm
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
///           PROGRAM CYLSPH_EX1
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
///           DOUBLE PRECISION      COLAT
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      SLON
///           DOUBLE PRECISION      Z
///
///     C
///     C     Load SPK and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'cylsph_ex1.tm' )
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
///     C     Convert the cylindrical coordinates to spherical.
///     C
///           CALL CYLSPH ( R, CLON, Z, RADIUS, COLAT, SLON )
///
///     C
///     C     Convert the spherical coordinates to rectangular.
///     C
///           CALL SPHREC ( RADIUS, COLAT, SLON, RECTAN )
///
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Original rectangular coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  X           (km): ', POS(1)
///           WRITE(*,FMT1) '  Y           (km): ', POS(2)
///           WRITE(*,FMT1) '  Z           (km): ', POS(3)
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Cylindrical coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius      (km): ', R
///           WRITE(*,FMT1) '  Longitude  (deg): ', CLON*DPR()
///           WRITE(*,FMT1) '  Z           (km): ', Z
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Spherical coordinates:'
///           WRITE(*,*) ' '
///           WRITE(*,FMT1) '  Radius      (km): ', RADIUS
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
///      Cylindrical coordinates:
///
///       Radius      (km):      383289.01777726
///       Longitude  (deg):         261.65040211
///       Z           (km):     -126505.93063865
///
///      Spherical coordinates:
///
///       Radius      (km):      403626.33912495
///       Colatitude (deg):         108.26566077
///       Longitude  (deg):         261.65040211
///
///      Rectangular coordinates from SPHREC:
///
///       X           (km):      -55658.44323296
///       Y           (km):     -379226.32931475
///       Z           (km):     -126505.93063865
///
///
///  2) Create a table showing a variety of cylindrical coordinates
///     and the corresponding spherical coordinates.
///
///     Corresponding spherical and cylindrical coordinates are
///     listed to three decimal places. All input and output angles
///     are in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM CYLSPH_EX2
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
///           DOUBLE PRECISION      COLAT
///           DOUBLE PRECISION      R      ( NREC )
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RCLON
///           DOUBLE PRECISION      SLON
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
///          . //        '  RADIUS   COLAT     SLON  '
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
///              CALL CYLSPH( R(I), RCLON, Z(I), RADIUS, COLAT, SLON )
///
///              WRITE (*,'(6F9.3)') R(I), CLON(I), Z(I), RADIUS,
///          .                       COLAT * DPR(), SLON  * DPR()
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
///          R       CLON      Z      RADIUS   COLAT     SLON
///       -------  -------  -------  -------  -------  -------
///         0.000    0.000    0.000    0.000    0.000    0.000
///         1.000    0.000    0.000    1.000   90.000    0.000
///         1.000   90.000    0.000    1.000   90.000   90.000
///         0.000    0.000    1.000    1.000    0.000    0.000
///         1.000  180.000    1.000    1.414   45.000  180.000
///         1.000  -90.000    0.000    1.000   90.000  -90.000
///         0.000    0.000   -1.000    1.000  180.000    0.000
///         1.000   45.000    0.000    1.000   90.000   45.000
///         1.000  180.000   -1.000    1.414  135.000  180.000
///         0.000  180.000    1.000    1.000    0.000  180.000
///         0.000   33.000    0.000    0.000    0.000   33.000
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
/// -    SPICELIB Version 1.2.0, 06-JUL-2021 (JDR)
///
///         Changed the argument names LONGC and LONG to CLON and SLON for
///         consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete code examples.
///
/// -    SPICELIB Version 1.1.1, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
///
/// -    SPICELIB Version 1.1.0, 30-MAR-2016 (BVS)
///
///         A cosmetic change: replaced '0.0 D0's with '0.0D0's.
///         Re-arranged header sections.
///
/// -    SPICELIB Version 1.0.2, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF. Obsolete $Revisions section
///         deleted.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn cylsph(r: f64, clon: f64, z: f64, radius: &mut f64, colat: &mut f64, slon: &mut f64) {
    CYLSPH(r, clon, z, radius, colat, slon);
}

//$Procedure CYLSPH ( Cylindrical to spherical )
pub fn CYLSPH(R: f64, CLON: f64, Z: f64, RADIUS: &mut f64, COLAT: &mut f64, SLON: &mut f64) {
    let mut BIG: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut RH: f64 = 0.0;
    let mut TH: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Convert to spherical, storing in temporary variables
    //
    BIG = intrinsics::DMAX1(&[f64::abs(R), f64::abs(Z)]);

    if (BIG == 0.0) {
        TH = 0.0;
        RH = 0.0;
    } else {
        X = (R / BIG);
        Y = (Z / BIG);

        RH = (BIG * f64::sqrt(((X * X) + (Y * Y))));
        TH = f64::atan2(R, Z);
    }

    //
    // Move the results to output variables
    //
    *SLON = CLON;
    *RADIUS = RH;
    *COLAT = TH;
}
