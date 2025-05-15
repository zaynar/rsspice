//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector relative difference, general dimension
///
/// Return the relative difference between two vectors of general
/// dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1,
///  V2         I   Input vectors.
///  NDIM       I   Dimension of V1 and V2.
///
///  The function returns the relative difference between two vectors
///  of general dimension.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two vectors for which the relative difference is to
///           be computed.
///
///  NDIM     is the dimension of V1 and V2.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the relative difference between the two input
///  n-dimensional vectors V1 and V2.
///
///  It is defined as:
///
///                       || V1 - V2 ||
///     VRELG   =   ------------------------
///                  MAX ( ||V1||, ||V2|| )
///
///  where ||X|| indicates the Euclidean norm of the vector X.
///
///  VRELG assumes values in the range [0,2]. If both V1 and V2 are
///  zero vectors then VRELG is defined to be zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If both V1 and V2 are zero vectors, then VRELG is defined to
///      be zero.
/// ```
///
/// # Particulars
///
/// ```text
///  This function computes the relative difference between two vectors
///  of general dimension as defined above.
///
///  The function VREL may be used to find the relative difference
///  for two 3-dimensional vectors.
/// ```
///
/// # Examples
///
/// ```text
///  This example determines if the state of Jupiter, with respect
///  to Voyager 2, for a set of times is the same for two different
///  ephemeris files. Instead of insisting on absolute equality
///  between the state vectors, the program will check if the relative
///  difference between the vectors is greater than a fixed tolerance.
///
///  C
///  C     The NAIF code for Jupiter is 599 and for Voyager 2 is -32.
///  C     Set the tolerance to be 0.0005.
///  C
///        INTEGER               JUP
///        PARAMETER           ( JUP = 599 )
///
///        INTEGER               VG2
///        PARAMETER           ( VG2 = -32 )
///
///        INTEGER               NUM
///        PARAMETER           ( NUM = 500 )
///
///        DOUBLE PRECISION      TOL
///        PARAMETER           ( TOL = 5.D-04 )
///
///  C
///  C     SPICELIB function
///  C
///        DOUBLE PRECISION      VRELG
///  C
///  C     Local variables
///  C
///        DOUBLE PRECISION      STATE1 ( 6, NUM )
///        DOUBLE PRECISION      STATE2 ( 6, NUM )
///        DOUBLE PRECISION      ET     (    NUM )
///        DOUBLE PRECISION      LT
///        DOUBLE PRECISION      DIFF
///
///        INTEGER               HANDLE
///        INTEGER               I
///
///        .
///        .
///        .
///
///        C
///        C     Load  the first SPK file.
///        C
///              CALL SPKLEF ( 'VG2_SOURCE_1.BSP', HANDLE )
///        C
///        C     Find the states for each time in the array ET.
///        C     This example assumes that the SPK file can
///        C     provide states for all of the times in the array.
///        C
///              DO I = 1, NUM
///
///                 CALL SPKEZ ( JUP, ET(I),      'J2000', 'LT',
///             .                VG2, STATE1(1,I), LT           )
///
///              END DO
///        C
///        C     Unload the first file and load the second one.
///        C
///              CALL SPKUEF ( HANDLE )
///
///              CALL SPKLEF ( 'VG2_SOURCE_2.BSP', HANDLE )
///        C
///        C     Find the states from the new file.
///        C
///              DO I = 1, NUM
///
///                 CALL SPKEZ ( JUP, ET(I),      'J2000', 'LT',
///             .                VG2, STATE2(1,I), LT           )
///
///              END DO
///        C
///        C     Now compare the two state vectors for each time.
///        C
///              DO I = 1, NUM
///
///                 DIFF = VRELG ( STATE1(1,I), STATE2(1,I), 6 )
///
///                 IF ( DIFF .GT. TOL ) THEN
///
///                    .
///                    .
///                    .
///
///                 END IF
///
///              END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
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
/// -    SPICELIB Version 1.0.0, 15-JUN-1992 (JML)
/// ```
pub fn vrelg(v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
    let ret = VRELG(v1, v2, ndim);
    ret
}

//$Procedure VRELG ( Vector relative difference, general dimension )
pub fn VRELG(V1: &[f64], V2: &[f64], NDIM: i32) -> f64 {
    let V1 = DummyArray::new(V1, 1..);
    let V2 = DummyArray::new(V2, 1..);
    let mut VRELG: f64 = 0.0;
    let mut NUNORM: f64 = 0.0;
    let mut DENORM: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // If the numerator is zero then set VRELG equal to zero. Otherwise,
    // perform the rest of the calculation.
    //
    // This handles the case where both vectors are zero vectors since
    // the distance between them will be zero.
    //
    NUNORM = VDISTG(V1.as_slice(), V2.as_slice(), NDIM);

    if (NUNORM == 0.0) {
        VRELG = 0.0;
    } else {
        DENORM = intrinsics::DMAX1(&[VNORMG(V1.as_slice(), NDIM), VNORMG(V2.as_slice(), NDIM)]);

        VRELG = (NUNORM / DENORM);
    }

    VRELG
}
