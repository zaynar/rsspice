//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// EK, return shadowing status \<STUB>
///
/// Return shadowing status of a specified EK file. THIS IS A
/// STUB ROUTINE.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle attached to EK file.
///  ISSHAD     O   Logical flag indicating whether EK is shadowed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an EK open for writing.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ISSHAD   is a logical flag that is returned .TRUE. if and
///           only if the EK file designated by HANDLE is
///           shadowed.
///
///           In this stub version of the routine, ISSHAD is
///           always returned .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  It is not an error to supply an input handle that does not
///      belong to an EK that is open for write access.
/// ```
///
/// # Files
///
/// ```text
///  See the EK Required Reading for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility that allows a program to test the
///  shadowing status of a specified EK file.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section of the umbrella routine EKSHAD.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This is a stub routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
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
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn ekshdw(handle: i32, isshad: &mut bool) {
    EKSHDW(handle, isshad);
}

//$Procedure EKSHDW ( EK, return shadowing status <STUB> )
pub fn EKSHDW(HANDLE: i32, ISSHAD: &mut bool) {
    let mut I: i32 = 0;

    I = HANDLE;
    *ISSHAD = false;
}
