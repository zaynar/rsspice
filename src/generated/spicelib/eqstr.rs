//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Equivalent strings
///
/// Determine whether two strings are equivalent.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A,
///  B          I   Arbitrary character strings.
///
///  The function returns .TRUE. if A and B are equivalent.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are arbitrary character strings.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if A and B are equivalent: that is,
///  if A and B contain  the same characters in the same order,
///  when blanks are ignored and uppercase and lowercase characters
///  are considered equal.
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
///  This routine is provided for those cases in which two strings
///  must be compared, and in which allowances are to be made for
///  extra (leading, trailing, and embedded) blanks and differences
///  in case. For the most part,
///
///     EQSTR ( A, B )
///
///  is .TRUE. whenever
///
///     CALL CMPRSS ( ' ', 0, A, TEMPA )
///     CALL UCASE  (            TEMPA, TEMPA )
///
///     CALL CMPRSS ( ' ', 0, B, TEMPB )
///     CALL UCASE  (            TEMPB, TEMPB )
///
///     EQVLNT = TEMPA .EQ. TEMPB
///
///  is .TRUE. There are two important differences, however.
///
///     1) The single reference to EQSTR is much simpler to
///        write, and simpler to understand.
///
///     2) The reference to EQSTR does not require any temporary
///        storage, nor does it require that the strings A and B
///        be changed. This feature is especially useful when
///        comparing strings received as subprogram arguments
///        against strings stored internally within the subprogram.
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
///  1) This code provides examples of equivalent and non-equivalent
///     strings according to the algorithm implemented in EQSTR.
///
///     Example code begins here.
///
///
///           PROGRAM EQSTR_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL               EQSTR
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 9  )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 22 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRLEN)    STR1   ( SETSIZ )
///           CHARACTER*(STRLEN)    STR2   ( SETSIZ )
///
///           INTEGER               I
///
///     C
///     C     Initialize the two arrays of strings.
///     C
///           DATA                  STR1   / 'A short string   ',
///          .                               'Embedded        blanks',
///          .                               'Embedded        blanks',
///          .                               ' ',
///          .                               'One word left out',
///          .                               'Extra [] delimiters',
///          .                               'Testing 1, 2, 3',
///          .                               'Case insensitive',
///          .                               'Steve'  /
///
///           DATA                  STR2   / 'ashortstring',
///          .                               'Em be dd ed bl an ks',
///          .                               '   Embeddedblanks',
///          .                               '          ',
///          .                               'WORD LEFT OUT',
///          .                               'extradelimiters',
///          .                               'TESTING123',
///          .                               'Case Insensitive',
///          .                               '  S t E v E  '  /
///
///
///     C
///     C     Compare the two arrays.
///     C
///           DO I = 1, SETSIZ
///
///              WRITE(*,*)
///              WRITE(*,*) 'STR1 : ', STR1(I)
///              WRITE(*,*) 'STR2 : ', STR2(I)
///
///              IF ( EQSTR( STR1(I), STR2(I) ) ) THEN
///
///                 WRITE(*,*) 'EQSTR: equivalent.'
///
///              ELSE
///
///                 WRITE(*,*) 'EQSTR: NOT equivalent.'
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
///      STR1 : A short string
///      STR2 : ashortstring
///      EQSTR: equivalent.
///
///      STR1 : Embedded        blanks
///      STR2 : Em be dd ed bl an ks
///      EQSTR: equivalent.
///
///      STR1 : Embedded        blanks
///      STR2 :    Embeddedblanks
///      EQSTR: equivalent.
///
///      STR1 :
///      STR2 :
///      EQSTR: equivalent.
///
///      STR1 : One word left out
///      STR2 : WORD LEFT OUT
///      EQSTR: NOT equivalent.
///
///      STR1 : Extra [] delimiters
///      STR2 : extradelimiters
///      EQSTR: NOT equivalent.
///
///      STR1 : Testing 1, 2, 3
///      STR2 : TESTING123
///      EQSTR: NOT equivalent.
///
///      STR1 : Case insensitive
///      STR2 : Case Insensitive
///      EQSTR: equivalent.
///
///      STR1 : Steve
///      STR2 :   S t E v E
///      EQSTR: equivalent.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example fragments.
///
/// -    SPICELIB Version 1.2.0, 03-AUG-1994 (NJB)
///
///         Code changed to eliminate DO WHILE ( .TRUE. ) construct.
///         The purpose of the change was to eliminate compilation
///         diagnostics relating to unreachable statements. The code
///         ran just fine before this change.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 10-MAY-1990 (NJB)
///
///         Loop termination condition fixed.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 03-AUG-1994 (NJB)
///
///         Code changed to eliminate DO WHILE ( .TRUE. ) construct.
///         The purpose of the change was to eliminate compilation
///         diagnostics relating to unreachable statements.
///
///         Changed some statements of form
///
///            IF <condition> <statement>
///
///         to
///
///            IF <condition> THEN
///
///              <statement>
///
///            END IF
///
/// -    SPICELIB Version 1.1.0, 10-MAY-1990 (NJB)
///
///         Loop termination condition fixed. The routine now checks
///         the termination case where both string pointers are pointing
///         to blanks, and at least one pointer has a value greater than
///         the length of the string it corresponds to. Internal comments
///         were updated accordingly.
/// ```
pub fn eqstr(a: &str, b: &str) -> bool {
    let ret = EQSTR(a.as_bytes(), b.as_bytes());
    ret
}

//$Procedure EQSTR ( Equivalent strings )
pub fn EQSTR(A: &[u8], B: &[u8]) -> bool {
    let mut EQSTR: bool = false;
    let mut PA: i32 = 0;
    let mut PB: i32 = 0;
    let mut LENA: i32 = 0;
    let mut LENB: i32 = 0;
    let mut CA: i32 = 0;
    let mut CB: i32 = 0;
    let mut LBOUND: i32 = 0;
    let mut UBOUND: i32 = 0;
    let mut DELTA: i32 = 0;
    let mut DONE: bool = false;

    //
    // Local variables
    //

    //
    // The general plan is to move a pair of pointers (PA, PB)
    // through strings A and B, skipping blank characters and
    // comparing others one-for-one.
    //
    //    Repeat:
    //
    //       If (A is blank) then
    //          Increment A
    //
    //       Else if (B is blank) then
    //          Increment B
    //
    //       Else
    //          If (A and B are equivalent) then
    //             Increment A and B
    //          Else
    //             Return .FALSE.
    //
    //       If (A and B are past end) then
    //          Return .TRUE.
    //
    //       Else if (A or B is past end and other is non-blank) then
    //          Return .FALSE.
    //
    //       Else if (A or B is past end and other is blank) then
    //          Return .TRUE.
    //
    // Note that no pointer gets incremented more than once on each
    // pass through the loop.
    //
    // On the other hand, in many cases the strings will be exactly
    // equal. If so, why knock ourselves out?
    //
    if fstr::eq(A, B) {
        EQSTR = true;
        return EQSTR;
    } else {
        PA = 1;
        PB = 1;

        LENA = intrinsics::LEN(A);
        LENB = intrinsics::LEN(B);

        LBOUND = intrinsics::ICHAR(b"a");
        UBOUND = intrinsics::ICHAR(b"z");
        DELTA = (intrinsics::ICHAR(b"A") - intrinsics::ICHAR(b"a"));

        DONE = false;

        while !DONE {
            //
            // At this point, we're guaranteed that
            //
            //   ( PA .LE. LENA )   and   ( PB .LE. LENB )
            //

            if fstr::eq(fstr::substr(A, PA..=PA), b" ") {
                PA = (PA + 1);
            } else if fstr::eq(fstr::substr(B, PB..=PB), b" ") {
                PB = (PB + 1);
            } else {
                CA = intrinsics::ICHAR(fstr::substr(A, PA..=PA));
                CB = intrinsics::ICHAR(fstr::substr(B, PB..=PB));

                if ((CA >= LBOUND) && (CA <= UBOUND)) {
                    CA = (CA + DELTA);
                }

                if ((CB >= LBOUND) && (CB <= UBOUND)) {
                    CB = (CB + DELTA);
                }

                if (CA == CB) {
                    PA = (PA + 1);
                    PB = (PB + 1);
                } else {
                    EQSTR = false;
                    DONE = true;
                    //
                    // We'll return from this point, having taken no further
                    // action.
                    //
                }
            }

            if !DONE {
                if (PA > LENA) {
                    //
                    // Whichever of the following tests passes, we're going
                    // to have a verdict at the end of the IF block below.
                    //
                    if (PB > LENB) {
                        EQSTR = true;
                    } else if fstr::ne(fstr::substr(B, PB..), b" ") {
                        EQSTR = false;
                    } else {
                        EQSTR = true;
                    }

                    DONE = true;
                //
                // We'll return from this point, having taken no further
                // action.
                //
                } else if (PB > LENB) {
                    //
                    // Whichever of the following tests passes, we're going
                    // to have a verdict at the end of the IF block below.
                    //
                    if fstr::ne(fstr::substr(A, PA..), b" ") {
                        EQSTR = false;
                    } else {
                        EQSTR = true;
                    }

                    DONE = true;
                    //
                    // We'll return from this point, having taken no further
                    // action.
                    //
                }
            }
        }
    }

    EQSTR
}
