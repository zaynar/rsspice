//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Element of a character set
///
/// Determine whether an item is an element of a character set.
///
/// # Required Reading
///
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   Item to be tested.
///  A          I   Set to be tested.
///
///  The function returns .TRUE. if ITEM is an element of set A.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is an item which may or may not be an element of the
///           input set. Trailing blanks in ITEM are not significant.
///
///  A        is a SPICE set. Trailing blanks in the members of A
///           are not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if ITEM is a member of the set A,
///  and returns .FALSE. otherwise.
///
///  The comparison between ITEM and members of A is case-sensitive.
///  Trailing blanks are ignored.
/// ```
///
/// # Particulars
///
/// ```text
///  The LOGICAL functions ELEMC, ELEMD and ELEMI correspond to the
///  set operator IN in the Pascal language.
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
///  1) Check if the elements of a list of body names belong to the
///     Solar System planets set.
///
///
///     Example code begins here.
///
///
///           PROGRAM ELEMC_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL                 ELEMC
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 LBCELL
///           PARAMETER             ( LBCELL = -5 )
///
///           INTEGER                 LISTSZ
///           PARAMETER             ( LISTSZ   = 6   )
///
///           INTEGER                 PNAMSZ
///           PARAMETER             ( PNAMSZ   = 7 )
///
///           INTEGER                 SETDIM
///           PARAMETER             ( SETDIM   = 9   )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(PNAMSZ)      ITEMS  ( LISTSZ        )
///           CHARACTER*(PNAMSZ)      PLNETS ( LBCELL:SETDIM )
///
///           INTEGER                 I
///
///     C
///     C     Create the original planets set and a list of body
///     C     names.
///     C
///           DATA                  ( PLNETS(I), I=1,SETDIM)  /
///          .                       'MERCURY', 'VENUS',   'EARTH',
///          .                       'MARS',    'JUPITER', 'SATURN',
///          .                       'URANUS',  'NEPTUNE', 'PLUTO'  /
///
///           DATA                    ITEMS /
///          .                            'EARTH', 'APOLLO', 'MARS',
///          .                            'PLUTO', 'VENUS', 'CERES' /
///
///     C
///     C     Validate the set: Initialize the non-empty set, remove
///     C     duplicates and sort the elements.
///     C
///           CALL VALIDC ( SETDIM, SETDIM, PLNETS )
///
///     C
///     C     Check if the items in the list belong to the set.
///     C
///           DO I = 1, LISTSZ
///
///              IF ( ELEMC ( ITEMS(I), PLNETS ) ) THEN
///
///                 WRITE(*,*) 'Item ', ITEMS(I), ' is in the set.'
///
///              ELSE
///
///                 WRITE(*,*) 'Item ', ITEMS(I),
///          .                 ' is NOT in the set.'
///
///              END IF
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
///      Item EARTH   is in the set.
///      Item APOLLO  is NOT in the set.
///      Item MARS    is in the set.
///      Item PLUTO   is in the set.
///      Item VENUS   is in the set.
///      Item CERES   is NOT in the set.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated description of arguments ITEM and A to indicate that
///         trailing blanks are not significant and extended
///         $Detailed_Output to indicate that comparison is case-sensitive.
///
///         Removed unnecessary $Revisions section.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
pub fn elemc(ctx: &mut SpiceContext, item: &str, a: CharArray) -> crate::Result<bool> {
    let ret = ELEMC(item.as_bytes(), a, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure ELEMC ( Element of a character set )
pub fn ELEMC(ITEM: &[u8], A: CharArray, ctx: &mut Context) -> f2rust_std::Result<bool> {
    let A = DummyCharArray::new(A, None, LBCELL..);
    let mut ELEMC: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Standard error handling:
    //
    if RETURN(ctx) {
        ELEMC = false;
        return Ok(ELEMC);
    } else {
        CHKIN(b"ELEMC", ctx)?;
    }
    //
    // Just a binary search.
    //
    ELEMC = (BSRCHC(ITEM, CARDC(A.as_arg(), ctx)?, A.subarray(1)) != 0);

    CHKOUT(b"ELEMC", ctx)?;
    Ok(ELEMC)
}
