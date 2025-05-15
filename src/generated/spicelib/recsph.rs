//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Rectangular to spherical coordinates
///
/// Convert from rectangular coordinates to spherical coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  RECTAN     I   Rectangular coordinates of a point.
///  R          O   Distance of the point from the origin.
///  COLAT      O   Angle of the point from the Z-axis (radians)
///  SLON       O   Longitude of the point (radians).
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
///  R        is the distance of the point from the origin.
///
///  COLAT    is the angle between the point and the positive Z-axis in
///           radians. The range of COLAT is [0, pi].
///
///  SLON     is the longitude of the point in radians. This is the
///           angle between the positive X-axis and the orthogonal
///           projection of the point onto the XY plane. SLON increases
///           in the counterclockwise sense about the positive Z-axis.
///           The range of SLON is [-pi, pi].
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
///  whose position is input in rectangular coordinates.
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
///  1) Compute the spherical coordinates of the position of the Moon
///     as seen from the Earth, and convert them to rectangular
///     coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: recsph_ex1.tm
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
///           PROGRAM RECSPH_EX1
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
///           CALL FURNSH ( 'recsph_ex1.tm' )
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
///     C     Convert the position vector POS to spherical
///     C     coordinates.
///     C
///           CALL RECSPH ( POS, RADIUS, COLAT, SLON )
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
///  2) Create a table showing a variety of rectangular coordinates
///     and the corresponding spherical coordinates.
///
///     Corresponding rectangular and spherical coordinates are
///     listed to three decimal places. Output angles in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM RECSPH_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
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
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      RECTAN ( 3, NREC )
///           DOUBLE PRECISION      SLON
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the input rectangular coordinates.
///     C
///           DATA                 RECTAN /
///          .                  0.D0,         0.D0,         0.D0,
///          .                  1.D0,         0.D0,         0.D0,
///          .                  0.D0,         1.D0,         0.D0,
///          .                  0.D0,         0.D0,         1.D0,
///          .                 -1.D0,         0.D0,         0.D0,
///          .                  0.D0,        -1.D0,         0.D0,
///          .                  0.D0,         0.D0,        -1.D0,
///          .                  1.D0,         1.D0,         0.D0,
///          .                  1.D0,         0.D0,         1.D0,
///          .                  0.D0,         1.D0,         1.D0,
///          .                  1.D0,         1.D0,         1.D0  /
///
///     C
///     C     Print the banner.
///     C
///           WRITE(*,*) ' RECT(1)  RECT(2)  RECT(3) '
///          . //        '  RADIUS   COLAT     SLON  '
///           WRITE(*,*) ' -------  -------  ------- '
///          . //        ' -------  -------  ------- '
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           DO I = 1, NREC
///
///              CALL RECSPH( RECTAN(1,I), RADIUS, COLAT, SLON )
///
///              WRITE (*,'(6F9.3)') ( RECTAN(J,I), J=1,3 ),
///          .                        RADIUS, COLAT * DPR(),
///          .                                SLON  * DPR()
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
///       RECT(1)  RECT(2)  RECT(3)   RADIUS   COLAT     SLON
///       -------  -------  -------  -------  -------  -------
///         0.000    0.000    0.000    0.000    0.000    0.000
///         1.000    0.000    0.000    1.000   90.000    0.000
///         0.000    1.000    0.000    1.000   90.000   90.000
///         0.000    0.000    1.000    1.000    0.000    0.000
///        -1.000    0.000    0.000    1.000   90.000  180.000
///         0.000   -1.000    0.000    1.000   90.000  -90.000
///         0.000    0.000   -1.000    1.000  180.000    0.000
///         1.000    1.000    0.000    1.414   90.000   45.000
///         1.000    0.000    1.000    1.414   45.000    0.000
///         0.000    1.000    1.000    1.414   45.000   90.000
///         1.000    1.000    1.000    1.732   54.736   45.000
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Changed the argument name LONG to SLON for consistency with
///         other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples. Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.3, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
///
/// -    SPICELIB Version 1.0.2, 07-JAN-2002 (NJB)
///
///         Fixed description of SLON in $Brief_I/O and Detailed_I/O
///         header sections.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn recsph(rectan: &[f64; 3], r: &mut f64, colat: &mut f64, slon: &mut f64) {
    RECSPH(rectan, r, colat, slon);
}

//$Procedure RECSPH ( Rectangular to spherical coordinates )
pub fn RECSPH(RECTAN: &[f64], R: &mut f64, COLAT: &mut f64, SLON: &mut f64) {
    let RECTAN = DummyArray::new(RECTAN, 1..=3);
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut Z: f64 = 0.0;
    let mut BIG: f64 = 0.0;

    //
    // Store rectangular coordinates in temporary variables
    //
    BIG = intrinsics::DMAX1(&[
        f64::abs(RECTAN[1]),
        f64::abs(RECTAN[2]),
        f64::abs(RECTAN[3]),
    ]);

    if (BIG > 0 as f64) {
        X = (RECTAN[1] / BIG);
        Y = (RECTAN[2] / BIG);
        Z = (RECTAN[3] / BIG);

        *R = (BIG * f64::sqrt((((X * X) + (Y * Y)) + (Z * Z))));

        *COLAT = f64::atan2(f64::sqrt(((X * X) + (Y * Y))), Z);

        X = RECTAN[1];
        Y = RECTAN[2];

        if ((X == 0.0) && (Y == 0.0)) {
            *SLON = 0.0;
        } else {
            *SLON = f64::atan2(Y, X);
        }
    } else {
        *R = 0.0;
        *COLAT = 0.0;
        *SLON = 0.0;
    }
}
