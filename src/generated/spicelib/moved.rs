//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Move a double precision array to another
///
/// Copy the elements of one double precision array into another
/// array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRFRM     I   Double precision array to be moved.
///  NDIM       I   Number of elements to copy.
///  ARRTO      O   Destination array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRFRM   is an array from which to copy items.
///
///  NDIM     is the number of items to copy. i.e. the dimension of
///           ARRFRM and ARRTO.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRTO    is the array to which items should be copied.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NDIM < 1, the output array is returned unchanged.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is simply shorthand for the following 3 lines of
///  code.
///
///     DO I = 1, NDIM
///        ARRTO(I) = ARRFRM(I)
///     END DO
/// ```
///
/// # Examples
///
/// ```text
///  Often one needs to make a temporary copy of an array so that
///  it can be manipulated without altering the original array.
///  As pointed out in particulars, you could just do this within
///  the code that needs the copy. However, if you have several
///  arrays to copy, you can cut the number of lines of code that
///  are needed by a third.
///
///  For example:
///
///     DO I = 1, 19
///        TEMPA(I) = A(I)
///     END DO
///
///     DO I = 1, 38
///        TEMPB(I) = B(I)
///     END DO
///
///  Can be rewritten as
///
///     CALL MOVED ( A, 19, TEMPA )
///     CALL MOVED ( B, 38, TEMPB )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.M. Owen          (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.17.0, 04-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.16.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 2.15.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.14.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 2.13.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.12.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 2.11.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 2.10.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.9.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.8.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.7.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 2.6.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 2.5.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 2.4.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 2.3.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 2.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 2.1.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 2.1.0, 09-NOV-2001 (NJB)
///
///         Bug fix: code for PC-LINUX and PC-DIGITAL environments
///         was changed to prevent corruption of bit patterns inserted
///         into double precision numbers via EQUIVALENCE statements.
///         Now the routine memmove is called to effect the double
///         precision assignments performed by this routine.
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
/// -    SPICELIB Version 2.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform. Added
///         Copyright section to header.
///
/// -    SPICELIB Version 1.1.0, 21-FEB-1996 (KRG)
///
///         This subroutine was turned into a machine dependent master
///         file due to problems with its use with arrays containing
///         INTEGERs that had been EQUIVALENCEd to DOUBLE PRECISION
///         numbers on the NeXT with the Absoft Fortran compiler, V3.2.
///
///         The DO loop has been replaced with a call to the C memmove
///         function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn moved(arrfrm: &[f64], ndim: i32, arrto: &mut [f64]) {
    MOVED(arrfrm, ndim, arrto);
}

//$Procedure MOVED ( Move a double precision array to another )
pub fn MOVED(ARRFRM: &[f64], NDIM: i32, ARRTO: &mut [f64]) {
    let ARRFRM = DummyArray::new(ARRFRM, 1..);
    let mut ARRTO = DummyArrayMut::new(ARRTO, 1..);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        ARRTO[I] = ARRFRM[I];
    }
}
