//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Smallest integer number
///
/// Return the value of the smallest (negative) number representable
/// in an integer variable.
///
/// # Brief I/O
///
/// ```text
///  The function returns the value of the smallest (negative) number
///  that can be represented in an integer variable.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value of the smallest (negative) number
///  that can be represented in an integer variable.
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
///  The function always returns a constant value, set by the user
///  prior to compilation.
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
///  1) Obtain the integer part of a double precision number. If the
///     integer component of the number is out of range, avoid
///     overflow by making it as large or small as possible.
///
///
///     Example code begins here.
///
///
///           PROGRAM INTMIN_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               INTMAX
///           INTEGER               INTMIN
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      NUMBER  ( 3 )
///
///           INTEGER               I
///           INTEGER               IVALUE
///
///     C
///     C     Define a set of three numbers, two of them having an
///     C     integer component that is out of range.
///     C
///           DATA                  NUMBER  / 2.D40, -1.5D35, 1.6D0 /
///
///
///           DO I = 1, 3
///
///              WRITE(*,*) 'Double precision number: ', NUMBER(I)
///
///     C
///     C        If the integer component is out of range, avoid
///     C        overflow by making it as large as possible.
///     C
///              IF ( NUMBER(I) .GT. DBLE( INTMAX() ) ) THEN
///
///                 WRITE(*,*) '   Overflow! Greater than INTMAX.'
///                 IVALUE = INTMAX()
///
///              ELSE IF ( NUMBER(I) .LT. DBLE( INTMIN() ) ) THEN
///
///                 WRITE(*,*) '   Overflow! Smaller than INTMIN.'
///                 IVALUE = INTMIN()
///
///              ELSE
///
///                 IVALUE = INT( NUMBER(I) )
///
///              END IF
///
///              WRITE(*,*) '   Integer part        : ', IVALUE
///              WRITE(*,*)
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
///      Double precision number:    2.0000000000000001E+040
///         Overflow! Greater than INTMAX.
///         Integer part        :   2147483647
///
///      Double precision number:   -1.5000000000000000E+035
///         Overflow! Smaller than INTMIN.
///         Integer part        :  -2147483648
///
///      Double precision number:    1.6000000000000001
///         Integer part        :            1
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Programming in VAX FORTRAN", Digital Equipment Corporation,
///       September 1984, Appendix C, FORTRAN Data Representation,
///       page C-2.
///
///  [2]  "Microsoft FORTRAN Reference", Microsoft Corporation,
///       1989, Section 1.3.1, page 10.
///
///  [3]  "Sun FORTRAN Programmer's Guide", Sun Microsystems,
///       Revision A of 6 May 1988, Appendix F, Manual Pages for
///       FORTRAN, page 306 (RANGE).
///
///  [4]  "Language Systems FORTRAN Reference Manual", Language
///       Systems Corporation, version 1.2.1, page 3-2.
///
///  [5]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       page 95.
///
///  [6]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers",
///       First Edition, June 1991, Hewlett Packard Company, page 4-4.
///
///  [7]  "SGI Fortran 77 Programmer's Guide", Document number
///       007-0711-030, page 2-2.
///
///  [8]  "Language Reference Manual", Absoft Fortran V3.2, 1993,
///       page 3-14, section 3.6.1.5. (for the NeXT)
///
///  [9]  "Unix/VMS Compatibility Libraries", Absoft Fortran V3.2,
///       1993; Chapter 3, Support Libraries, page 3-14, inmax.
///       (for the NeXT)
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.17.1, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example from existing code fragment.
///
/// -    SPICELIB Version 3.17.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 3.16.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.15.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 3.14.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.13.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 3.12.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 3.11.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.10.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.9.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.8.0, 12-DEC-2008 (EDW)
///
///         Updated for PC-LINUX-GFORTRAN and MAC-OSX-GFORTRAN. Eliminated
///         environment descriptions. Most were out-of-date or wrong.
///         IMPLICIT NONE now included in all environments.
///
///         Corrected the PC-CYGWIN expression from
///
///            -2147483647
///
///         to
///
///            -2147483647 -1
///
/// -    SPICELIB Version 3.7.0, 14-MAR-2008 (EDW)
///
///         Changed the expression for
///
///            -2147483648
///
///         to
///
///            -2147483647 -1
///
///         to support those compilers that implement "-" as an operator.
///
/// -    SPICELIB Version 3.6.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 3.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 3.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 3.3.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 3.2.1, 31-AUG-2002 (BVS)
///
///         Changed value for SGI compiled with "-n32" from -2147483648 to
///         -2147483647.
///
/// -    SPICELIB Version 3.2.0, 30-JUN-2002 (EDW)
///
///         Added environments for Apple OS X G77 (MAC-OSX-G77),
///         and Absoft F77 (MAC-OSX-F77)
///
/// -    SPICELIB Version 3.1.3, 18-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 3.1.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 3.1.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 3.1.0, 03-AUG-1998 (FST)
///
///         The value for the PC under Microsoft Powerstation was
///         changed from -2147483647 to -2147483648. The Macintosh
///         value was changed back to -2147483648 as well.
///
/// -    SPICELIB Version 3.0.0, 05-APR-1998 (NJB)
///
///         Added reference to the PC-LINUX environment.
///
/// -    SPICELIB Version 2.0.0, 09-NOV-1993 (HAN) (MJS)
///
///         The value for the Macintosh has been changed from
///         -2147483648 to -2147483647.
///         Also, the module was updated to include the function value
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms.
///
/// -    SPICELIB Version 1.5.0, 09-OCT-1992 (HAN)
///
///         Module was updated to include the value of INTMIN for the
///         Hewlett Packard UX 9000/750 environment.
///
/// -    SPICELIB Version 1.4.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.4.0, 13-NOV-1991 (MJS)
///
///         Module was updated to include the value of INTMIN
///         for the Lahey F77L EM/32 environment (PC).
///
/// -    SPICELIB Version 1.3.0, 19-SEP-1991 (HAN) (NJB)
///
///         The minimum integer is -2147483648. Because the compiler
///         (version 1.3.1) does not accept this value in a direct
///         assignment, the value of INTMIN is specified as
///         ( -INMAX() ) - 1.
///
/// -    SPICELIB Version 1.2.0, 07-DEC-1990 (MJS)
///
///         Module was updated to include the value of INTMIN for
///         the Macintosh.
///
/// -    SPICELIB Version 1.1.0, 12-MAR-1990 (HAN)
///
///         Module was changed to include the function value for
///         the Sun. References were added to specify the sources
///         of the function values on different machines.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.1.0, 03-AUG-1998 (FST)
///
///         The value for the PC under Microsoft Powerstation was
///         changed from -2147483647 to -2147483648. The Macintosh
///         value was changed back to -2147483648 as well.
///
/// -    SPICELIB Version 3.0.0, 05-APR-1998 (NJB)
///
///         Added reference to the PC-LINUX environment.
///
/// -    SPICELIB Version 2.0.0, 9-NOV-1993 (HAN) (MJS)
///
///         The value for the Macintosh has been changed from
///         -2147483648 to -2147483647.
///         Also, the module was updated to include the function value
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms.
///
/// -    SPICELIB Version 1.5.0, 6-OCT-1992 (HAN)
///
///         Module was updated to include the value of INTMIN for the
///         Hewlett Packard UX 9000/750 environment.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 1.4.0, 13-NOV-1991 (MJS)
///
///         Module was updated to include the value of INTMIN
///         for the Lahey F77L EM/32 environment (PC).
///
/// -    SPICELIB Version 1.3.0, 19-SEP-1991 (HAN) (NJB)
///
///         The minimum integer is -2147483648. Because the compiler
///         (version 1.3.1) does not accept this value in a direct
///         assignment, the value of INTMIN is specified as
///         ( -INMAX() ) - 1.
///
/// -    SPICELIB Version 1.2.0, 7-DEC-1990 (MJS)
///
///         Module was updated to include the value of INTMIN for
///         the Macintosh. $Literature_References section was updated.
///
/// -    SPICELIB Version 1.1.0, 12-MAR-1990 (HAN)
///
///         Code was changed to include the function value
///         for the Sun. Documentation in the $Particulars
///         section was changed to include the value, and the
///         example was updated and corrected.
///
///         All sources for the values contained in this module
///         are now specified in the $Literature_References section.
///
/// -    Beta Version 1.1.0, 16-FEB-1989 (HAN) (NJB)
///
///         Contents of the $Exceptions section was changed
///         to "error free" to reflect the decision that the
///         module will never participate in error handling.
///
///         Parentheses added to function declaration.
/// ```
pub fn intmin() -> i32 {
    let ret = INTMIN();
    ret
}

//$Procedure INTMIN ( Smallest integer number )
pub fn INTMIN() -> i32 {
    let mut INTMIN: i32 = 0;

    //
    // Numbers are provided in a variety of formats: decimal, hex, sand
    // binary. These last two formats are not portable; but then,
    // neither are the values.
    //

    //
    INTMIN = (-2147483647 - 1);

    INTMIN
}
