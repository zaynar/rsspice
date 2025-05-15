//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Rectangular to cylindrical coordinates
///
/// Convert from rectangular to cylindrical coordinates.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  RECTAN     I   Rectangular coordinates of a point.
///  R          O   Distance of the point from Z axis.
///  CLON       O   Angle (radians) of the point from XZ plane
///  Z          O   Height of the point above XY plane.
/// ```
///
/// # Detailed Input
///
/// ```text
///  RECTAN   are the rectangular coordinates of the point of
///           interest.
/// ```
///
/// # Detailed Output
///
/// ```text
///  R        is the distance of the point of interest from Z-axis.
///
///  CLON     is the cylindrical angle (in radians) of the point of
///           interest from XZ plane. The CLON range is [0, 2pi].
///
///  Z        is the height of the point above XY plane.
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
///  This routine transforms the coordinates of a point from
///  rectangular to cylindrical coordinates.
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
///  1) Compute the cylindrical coordinates of the position of the Moon
///     as seen from the Earth, and convert them to rectangular
///     coordinates.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: reccyl_ex1.tm
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
///           PROGRAM RECCYL_EX1
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
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      RECTAN ( 3 )
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      Z
///
///     C
///     C     Load SPK and LSK kernels, use a meta kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'reccyl_ex1.tm' )
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
///     C     Convert the cylindrical to rectangular coordinates.
///     C
///
///           CALL CYLREC ( R, CLON, Z, RECTAN )
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
///           WRITE(*,*) 'Rectangular coordinates from CYLREC:'
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
///      Rectangular coordinates from CYLREC:
///
///       X          (km):      -55658.44323296
///       Y          (km):     -379226.32931475
///       Z          (km):     -126505.93063865
///
///
///  2) Create a table showing a variety of rectangular coordinates
///     and the corresponding cylindrical coordinates.
///
///     Corresponding rectangular and cylindrical coordinates are
///     listed to three decimal places. Output angles are in degrees.
///
///
///     Example code begins here.
///
///
///           PROGRAM RECCYL_EX2
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
///           DOUBLE PRECISION      CLON
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RECTAN ( 3, NREC )
///           DOUBLE PRECISION      Z
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
///          . //        '    R       CLON      Z'
///           WRITE(*,*) ' -------  -------  ------- '
///          . //        ' -------  -------  ------- '
///
///     C
///     C     Do the conversion. Output angles in degrees.
///     C
///           DO I = 1, NREC
///
///              CALL RECCYL( RECTAN(1,I), R, CLON, Z )
///
///              WRITE (*,'(6F9.3)') ( RECTAN(J,I), J=1,3 ),
///          .                         R, CLON * DPR(), Z
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
///       RECT(1)  RECT(2)  RECT(3)     R       CLON      Z
///       -------  -------  -------  -------  -------  -------
///         0.000    0.000    0.000    0.000    0.000    0.000
///         1.000    0.000    0.000    1.000    0.000    0.000
///         0.000    1.000    0.000    1.000   90.000    0.000
///         0.000    0.000    1.000    0.000    0.000    1.000
///        -1.000    0.000    0.000    1.000  180.000    0.000
///         0.000   -1.000    0.000    1.000  270.000    0.000
///         0.000    0.000   -1.000    0.000    0.000   -1.000
///         1.000    1.000    0.000    1.414   45.000    0.000
///         1.000    0.000    1.000    1.000    0.000    1.000
///         0.000    1.000    1.000    1.000   90.000    1.000
///         1.000    1.000    1.000    1.414   45.000    1.000
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
///         Changed the output argument name LONG to CLON for consistency
///         with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code examples.
///
/// -    SPICELIB Version 1.0.3, 26-JUL-2016 (BVS)
///
///         Minor headers edits.
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
pub fn reccyl(ctx: &mut SpiceContext, rectan: &[f64; 3], r: &mut f64, clon: &mut f64, z: &mut f64) {
    RECCYL(rectan, r, clon, z, ctx.raw_context());
}

//$Procedure RECCYL ( Rectangular to cylindrical coordinates )
pub fn RECCYL(RECTAN: &[f64], R: &mut f64, CLON: &mut f64, Z: &mut f64, ctx: &mut Context) {
    let RECTAN = DummyArray::new(RECTAN, 1..=3);
    let mut X: f64 = 0.0;
    let mut Y: f64 = 0.0;
    let mut BIG: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Use temporary variables for computing R.
    //
    BIG = intrinsics::DMAX1(&[f64::abs(RECTAN[1]), f64::abs(RECTAN[2])]);

    //
    // Convert to cylindrical coordinates
    //
    *Z = RECTAN[3];

    if (BIG == 0 as f64) {
        *R = 0.0;
        *CLON = 0.0;
    } else {
        X = (RECTAN[1] / BIG);
        Y = (RECTAN[2] / BIG);

        *R = (BIG * f64::sqrt(((X * X) + (Y * Y))));

        *CLON = f64::atan2(Y, X);
    }

    if (*CLON < 0.0) {
        *CLON = (*CLON + TWOPI(ctx));
    }
}
