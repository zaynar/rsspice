//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Smallest DP number
///
/// Return the value of the smallest (negative) number representable
/// in a double precision variable.
///
/// # Brief I/O
///
/// ```text
///  The function returns the value of the smallest (negative) number
///  that can be represented in a double precision variable.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value of the smallest (negative) number
///  that can be represented in a double precision variable.
///
///  This varies from machine to machine. Values for several popular
///  machines are shown below.
///
///  Environment: VAX/VMS, VAX FORTRAN
///  Value:       -1.70141183460469229D+38
///
///  Environment: Sun, Sun FORTRAN
///  Value:       -1.7976931348623D+308
///
///  Environment: IBM PC, MS (Microsoft) FORTRAN 5.0
///  Value:       -1.79769313486231D+308
///
///  Environment: Macintosh, Language Systems FORTRAN 1.2.1
///  Value:       -1.7D+308
///
///  Environment: PC/Linux, Fort77
///  Value:       -1.79769313486231D+308
///
///  Environment: PC, Lahey F77 EM/32 Version 4.0
///  Value:       -1.79769313486231D+308
///
///  Environment: HP-UX 9000/750, FORTRAN/9000 Series 700 computers
///  Value:       -1.79769313486231D+308
///
///  Environment: Silicon Graphics, IRIX OS, SGI FORTRAN 77
///  Value:       -1.7976931348623158D+308
///
///  Environment: DEC Alpha, OSF/1, DEC FORTRAN-77
///  Value:       -1.70141183460469229D+38
///
///  Environment: NeXT, Mach OS, Absoft Fortran 77
///  Value:       (refer to the code below)
///
///
///  References for the values above are listed in the
///  $Literature_References section of the header.
///
///  IBM PC Note:
///
///     Although the Microsoft FORTRAN documentation lists
///     -1.797693134862316D+308 as the minimum negative double
///     precision number, it produced a compilation error. As a result,
///     we changed the number to -1.79769313486231D+308, which did not
///     produce a compilation error.
///
///  DEC Alpha OSF/1 Note:
///
///     NAIF does not own an Alpha nor its accompanying documentation.
///     The value in this routine was supplied by a SPICELIB user who
///     looked the value up in his manual.
///
///  PC/Linux Fort77 Note:
///
///     Value was validated by experiment. The value is identical to
///     that for the PC/MS Fortran and HP-UX 9000/750, FORTRAN/9000
///     platforms.
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
///  1) Return the value of the smallest negative number representable
///     in a double precision variable.
///
///     Example code begins here.
///
///
///           PROGRAM DPMIN_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      DPMIN
///
///     C
///     C     Display the smallest negative number representable
///     C     in a double precision variable.
///     C
///           WRITE(*,'(A,E22.14)')
///          .               'Minimum Double Precision Number: ',
///          .               DPMIN()
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Minimum Double Precision Number:  -0.17976931348623+309
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Programming in VAX FORTRAN", Digital Equipment Corporation,
///       September 1984, Appendix C, FORTRAN Data Representation,
///       page C-3.
///
///  [2]  "Microsoft FORTRAN Reference", Microsoft Corporation
///       1989, Section 1.3.1, page 13.
///
///  [3]  "Sun FORTRAN Programmer's Guide", Sun Microsystems,
///       Revision A of 6 May 1988, Appendix F, Manual Pages for
///       FORTRAN, page 306 (LIBM_DOUBLE). This routine includes
///       the function D_MAX_NORMAL.
///
///  [4]  "FORTRAN Programmer's Guide for the Sun Workstation",
///       Sun Microsystems, Revision E of 17 February 1986,
///       Appendix E, Manual Pages for FORTRAN, page 184 (RANGE).
///       This routine includes the function DFLMAX.
///
///  [5]  "Language Systems FORTRAN Reference Manual", Language Systems
///       Corporation, version 1.2.1, page 3-12.
///
///  [6]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       page 95.
///
///  [7]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers",
///       First Edition, June 1991, Hewlett Packard Company, page 4-5.
///
///  [8]  "SGI Fortran 77 Programmer's Guide", Document number
///       007-0711-030, page 2-2.
///
///  [9]  "Language Reference Manual", Absoft Fortran V3.2, 1993,
///       page 3-16, section 3.6.1.5. (for the NeXT)
///
///  [10] "Unix/VMS Compatibility Libraries", Absoft Fortran V3.2,
///       1993; Chapter 3, Support Libraries, page 3-5, dflmax. (for
///       the NeXT)
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
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.15.1, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 2.15.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 2.14.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.13.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 2.12.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.11.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 2.10.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 2.9.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.8.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.7.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.6.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 2.5.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 2.4.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 2.3.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 2.2.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 2.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 2.0.4, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 2.0.3, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 2.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 2.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 05-APR-1998 (NJB)
///
///         Added references to the PC-LINUX environment.
///
/// -    SPICELIB Version 1.7.0, 07-AUG-1996 (WLT)
///
///         The master program was updated to SAVE the minimum
///         d.p. for the NeXT edition of the toolkit.
///
/// -    SPICELIB Version 1.6.0, 05-JAN-1995 (HAN)
///
///         Module was revised to include one assigned value for all
///         Sun operating systems instead of calling different Sun
///         functions to return the value.
///
/// -    SPICELIB Version 1.5.0, 03-NOV-1993 (HAN) (NJB)
///
///         Module was updated to include the function value
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms.
///
/// -    SPICELIB Version 1.4.0, 06-OCT-1992 (HAN)
///
///         Module was updated to include the value of DPMIN for the
///         Hewlett Packard UX 9000/750 environment.
///
/// -    SPICELIB Version 1.3.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.3.0, 13-NOV-1991 (MJS)
///
///         Module was updated to include the value of DPMIN
///         for the Lahey F77L EM/32 environment (PC).
///
/// -    SPICELIB Version 1.2.0, 07-DEC-1990 (MJS)
///
///         Module was updated to include the value of DPMIN for
///         the Macintosh.
///
/// -    SPICELIB Version 1.1.0, 09-MAR-1990 (HAN)
///
///         Module was changed to include the function value for
///         the Sun, IBM PC value was changed, and references
///         were added to specify the sources of the function
///         values on different machines.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 05-APR-1998 (NJB)
///
///         Added references to the PC-LINUX environment.
///
/// -    SPICELIB Version 1.6.0, 5-JAN-1995 (HAN)
///
///         Module was revised to include one assigned value for all
///         Sun operating systems instead of calling different Sun
///         functions to return the value.
///
/// -    SPICELIB Version 1.5.0, 3-NOV-1993 (HAN) (NJB)
///
///         Module was updated to include the function value
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms.
///
/// -    SPICELIB Version 1.4.0, 6-OCT-1992 (HAN)
///
///         Module was updated to include the value of DPMIN for the
///         Hewlett Packard UX 9000/750 environment.
///
///         The code was also reformatted so that a utility program
///         can create the source file for a specific environment
///         given a master source file.
///
/// -    SPICELIB Version 1.2.0, 7-DEC-1990 (MJS)
///
///         Module was updated to include the value of DPMIN for
///         the Macintosh.
///
/// -    SPICELIB Version 1.1.0, 9-MAR-1990 (HAN)
///
///         Code was changed to include the function value
///         for Sun. Documentation in the $Particulars
///         section was changed to include the value, the
///         example was updated and corrected.
///
///         Code was changed to update the function value for
///         the IBM PC. The previous value did not compile
///         under the Microsoft FORTRAN Version 5.0. The last
///         digit was deleted from the value, and the module then
///         compiled.
///
///         All sources for the values contained in this module
///         are now specified in the $Literature_References section.
/// ```
pub fn dpmin() -> f64 {
    let ret = DPMIN();
    ret
}

//$Procedure DPMIN ( Smallest DP number )
pub fn DPMIN() -> f64 {
    let mut DPMIN: f64 = 0.0;

    //
    // Numbers are provided in a variety of formats: decimal, hex,
    // and binary. These last two formats are not portable; but then,
    // neither are the values.
    //

    //
    // IBM PC, Microsoft FORTRAN
    // PC,     Lahey F77 EM/32 FORTRAN
    // HP-UX 9000/750, FORTRAN/9000 Series 700 computers
    // PC/Linux, Fort77
    //
    DPMIN = -179769313486231000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    DPMIN
}
