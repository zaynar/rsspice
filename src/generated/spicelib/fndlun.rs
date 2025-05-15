//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MINLUN: i32 = 1;
pub const MAXLUN: i32 = 99;
const NRESV: i32 = 3;

struct SaveVars {
    IOSTAT: i32,
    LAST: i32,
    RESNUM: StackArray<i32, 3>,
    FIRST: bool,
    OPENED: bool,
    RESVD: StackArray<bool, 99>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut IOSTAT: i32 = 0;
        let mut LAST: i32 = 0;
        let mut RESNUM = StackArray::<i32, 3>::new(1..=NRESV);
        let mut FIRST: bool = false;
        let mut OPENED: bool = false;
        let mut RESVD = StackArray::<bool, 99>::new(MINLUN..=MAXLUN);

        LAST = MINLUN;
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(5), Val::I(6), Val::I(7)].into_iter();
            RESNUM
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            IOSTAT,
            LAST,
            RESNUM,
            FIRST,
            OPENED,
            RESVD,
        }
    }
}

/// Find a free logical unit
///
/// Return the number of a free logical unit, if one is available.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       O   The number of a free logical unit.
///  MINLUN     P   Minimum logical unit number.
///  MAXLUN     P   Maximum logical unit number.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UNIT     is the number of a free logical unit (also called
///           an "external unit"). A "free" logical unit is one
///           that is not reserved and is not currently connected to
///           and open file. If no free units are available, the
///           value of UNIT is 0.
/// ```
///
/// # Parameters
///
/// ```text
///  MINLUN   is the minimum logical unit number. The Fortran
///           standard states that unit numbers must be zero or
///           positive. However, the value 0 is reserved as a
///           status code for this routine, so MINLUN must be
///           assigned a value greater than 0.
///
///  MAXLUN   is the maximum logical unit number allowed by the
///           VAX Fortran compiler. This may differ for other
///           machines.
///
///  Listed below are the values for several machines:
///
///  Environment: VAX/VMS, VAX FORTRAN
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: Sun, Sun FORTRAN
///  MINLUN:      1
///  MAXLUN:      63
///
///  Environment: PC, MS FORTRAN *
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: PC/Linux, Fort77
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: Macintosh, Language Systems FORTRAN
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: PC, Lahey F77 EM/32 Version 4.0
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: HP-UX 9000/750, FORTRAN/9000 Series 700 computers
///  MINLUN:      1
///  MAXLUN:      61
///
///  Environment: Silicon Graphics, SGI f77
///  MINLUN:      1
///  MAXLUN:      63
///
///  Environment: DEC Alpha OSF/1, DEC FORTRAN
///  MINLUN:      1
///  MAXLUN:      99
///
///  Environment: NeXT, Absoft Fortran
///  MINLUN:      1
///  MAXLUN:      99
///
///  * 32767 is the actual value a logical unit may be assigned to
///    on the IBM PC, however, using this value increases the memory
///    requirements of a program calling this routine by 128K.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If no logical units are available, UNIT is set equal
///      to 0.
///
///  2)  This routine performs a Fortran INQUIRE operation. If
///      the INQUIRE fails, UNIT is set equal to the negative
///      of the INQUIRE iostat ( UNIT will thus have a negative
///      value).
/// ```
///
/// # Particulars
///
/// ```text
///  FNDLUN returns the number of the first (unreserved) unit not
///  currently connected to a file. It thus frees the user from
///  having to maintain an accounting of which units are open, which
///  are closed, and which are available.
///
///  This routine is related to the routines GETLUN, RESLUN, and
///  FRELUN. Together, these routines support coordinated usage of
///  Fortran logical units.  FNDLUN (Find a free logical unit) and
///  GETLUN (Get a free logical unit) both have the function of
///  returning a logical unit number that is not reserved or already
///  in use. The principal difference between the functionality of
///  these routines is that GETLUN both returns a status code and
///  signals an error if a free unit is not found, while FNDLUN
///  merely returns a status code.
///
///  RESLUN is used to reserve logical unit numbers, so that they will
///  not be returned by GETLUN or FNDLUN; FRELUN frees logical units
///  previously reserved via calls to RESLUN.
///
///  On the VAX, SUN, PC, and HP logical units 5-7 are reserved by
///  default. On the Macintosh logical units 5,6 and 9 are reserved
///  by default. Other units may be reserved by calling RESLUN. Once
///  reserved, units (except ones reserved by default) may be
///  unreserved by calling FRELUN.
///
///  To reserve logical unit numbers for special use, refer to
///  RESLUN. To make reserved units available to FNDLUN and GETLUN,
///  refer to FRELUN.
///
///  A unit returned by FNDLUN does NOT automatically become a
///  reserved unit. If the user wishes to reserve a unit found by
///  FNDLUN, the call to FNDLUN must be followed by a call to RESLUN.
///
///  Note that although 0 is a valid logical unit number on some
///  systems, a value of 0 returned by FNDLUN indicates that no free
///  logical unit was available, rather than that logical unit 0 is
///  available. Similarly, negative values returned by FNDLUN are
///  status codes, not logical unit numbers.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of FNDLUN.
///
///     CALL FNDLUN ( UNIT )
///
///     IF ( UNIT .LT. 0 ) THEN
///        RETURN
///     END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine never returns logical unit numbers that are less
///      than or equal to 0.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Programming in VAX FORTRAN", Digital Equipment Corporation,
///       Section 11.1.1, p.11-2, September 1984.
///
///  [2]  "Microsoft FORTRAN Reference", Microsoft Corporation
///       Section 3.2.2, p.61, 1989,
///
///  [3]  "Sun FORTRAN Programmer's Guide", Sun Microsystems,
///       Revision A, Section 7.2, p.73, 6 May 1988
///
///  [4]  "Language Systems FORTRAN Reference Manual", Version 2.1,
///       page 193.
///
///  [5]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       page 94.
///
///  [6]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers", First
///       Edition, Hewlett Packard Company, pp.6-2 and 6-4, June 1991.
///
///  [7]  Silicon Graphics "Fortran 77 Programmer's Guide",
///       Document number 007-0711-030, page 1-20.
///
///  [8]  "Language Reference Manual", Absoft Fortran V3.2, section
///       7.3.1 (for the NeXT), p.7-4, 1993.
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.28.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 6.27.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.26.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 6.25.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 6.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 6.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.21.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 6.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 6.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 6.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 6.17.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 6.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 6.14.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 6.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 6.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 6.11.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 6.10.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 6.9.0, 16-MAR-2009 (BVS)
///
///         Changed MAXLUN from 99 to 61 for HP and HP_C environments. The
///         value 61 was determined by trial-n-error while preparing a
///         special HP toolkit delivery for GSFC in July 2008.
///
/// -    SPICELIB Version 6.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 6.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 6.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 6.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 6.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 6.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 6.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 6.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 6.0.3, 24-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 6.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 6.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 6.0.0, 05-APR-1998 (NJB)
///
///         References to the PC-LINUX environment were added.
///
/// -    SPICELIB Version 5.0.0, 09-NOV-1993 (HAN)
///
///          Module was updated to include the logical unit values
///          for the Silicon Graphics, DEC Alpha-OSF/1, and
///          NeXT platforms.
///
/// -    SPICELIB Version 4.0.0, 06-OCT-1992 (HAN)
///
///          Module was updated to include the logical unit values for
///          the Hewlett Packard UX 9000/750 environment.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1992 (MJS)
///
///          IOSTAT check now placed directly after the INQUIRE
///          statement.
///
/// -    SPICELIB Version 2.2.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 2.2.0, 13-NOV-1991 (MJS)
///
///          Module was updated to include the value of MAXLUN
///          for the Lahey F77L EM/32 environment (PC).
///
/// -    SPICELIB Version 2.1.0, 15-MAY-1991 (MJS)
///
///          Module was updated to allow portability to the Macintosh
///          environment.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///          The array RESNUM now contains the default reserved
///          logical units. All the elements of the array RESVD
///          were initialized. The value of MAXLUN for the IBM PC
///          was changed from 32767 to 99. Some header comments
///          were clarified.
///
/// -    SPICELIB Version 1.0.1, 20-MAR-1990 (HAN)
///
///          $Parameters section was updated to include the values
///          of MINLUN and MAXLUN for several machines. Sources of
///          these values are listed in the Literature References
///          section.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.0.0, 05-APR-1998 (NJB)
///
///         References to the PC-LINUX environment were added.
///
/// -    SPICELIB Version 5.0.0, 09-NOV-1993 (HAN)
///
///         Module was updated to include the logical unit values
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms.
///
///         The values used for the DEC Alpha worked in all of the
///         porting tests, but NAIF has no documentation for this
///         platform.
///
/// -    SPICELIB Version 1.4.0, 06-OCT-1992 (HAN)
///
///         Module was updated to include the logical unit values for
///         the Hewlett Packard UX 9000/750 environment.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1992 (MJS)
///
///         IOSTAT check now placed directly after the INQUIRE
///         statement. Previously, IOSTAT could have been checked
///         without first being assigned a value.
///
/// -    SPICELIB Version 2.1.0, 15-MAY-1991 (MJS)
///
///         Module was updated to allow portability to the Macintosh
///         environment. Literature References section was updated.
///         Some header comments were clarified.
///
/// -    SPICELIB Version 2.0.0, 26-MAR-1991 (MJS) (NJB)
///
///         The default reserved logical units are now declared in the
///         array RESNUM. All the elements of the array RESVD were
///         initialized. These two changes allow FNDLUN to be ported
///         to other platforms more easily. The value of MAXLUN for the
///         IBM PC was decreased from 32767 to 99.
///
///         Some cosmetic changes to variable declarations were made.
///         Also, some header comments were added to make the header's
///         discussion clearer.
///
/// -    Beta Version 1.1.0, 09-MAR-1989  (HAN)
///
///         Declaration of the variable RETURN was removed from the code.
///         The variable was declared, but not used.
/// ```
pub fn fndlun(ctx: &mut SpiceContext, unit: &mut i32) -> crate::Result<()> {
    FNDLUN(unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FNDLUN ( Find a free logical unit )
pub fn FNDLUN(UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Parameters
    //

    //
    // Local variables
    //

    //
    // Save everything between calls.
    //

    //
    // Initial values
    //

    //
    // VAX, SUN, PC, HP, SGI, DEC Alpha-OSF/1, and PC/Lunix
    // reserved units.
    //

    //
    // Initialize RESVD if it hasn't already been done.
    //
    if save.FIRST {
        for I in MINLUN..=MAXLUN {
            save.RESVD[I] = false;
        }

        for I in 1..=NRESV {
            save.RESVD[save.RESNUM[I]] = true;
        }

        save.FIRST = false;
    }

    //
    // Begin with the unit following the last one returned.
    // Cycle through the available units. Skip reserved units,
    // INQUIRE about others.
    //
    for I in (save.LAST + 1)..=MAXLUN {
        if save.RESVD[I] {
            save.OPENED = true;
        } else {
            {
                use f2rust_std::io;

                let specs = io::InquireSpecs {
                    unit: Some(I),
                    opened: Some(&mut save.OPENED),
                    ..Default::default()
                };
                save.IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
            }

            if (save.IOSTAT > 0) {
                *UNIT = -save.IOSTAT;
                return Ok(());
            }
        }

        if !save.OPENED {
            *UNIT = I;
            save.LAST = *UNIT;
            return Ok(());
        }
    }

    //
    // If we've come this far, we need to search the first part of
    // the list again, up to the last unit returned. Once again,
    // skip reserved units, INQUIRE about others.
    //
    for I in MINLUN..=save.LAST {
        if save.RESVD[I] {
            save.OPENED = true;
        } else {
            {
                use f2rust_std::io;

                let specs = io::InquireSpecs {
                    unit: Some(I),
                    opened: Some(&mut save.OPENED),
                    ..Default::default()
                };
                save.IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
            }

            if (save.IOSTAT > 0) {
                *UNIT = -save.IOSTAT;
                return Ok(());
            }
        }

        if !save.OPENED {
            *UNIT = I;
            save.LAST = *UNIT;
            return Ok(());
        }
    }

    //
    // If we've come this far, there are no free units to be had.
    // C'est la vie. Assign 0 to the unit number.
    //

    *UNIT = 0;

    Ok(())
}

/// Reserve a logical unit
///
/// Reserve a logical unit number. Reserved units are never returned
/// by FNDLUN or GETLUN.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Number of the logical unit to be reserved.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the number of the logical unit to be reserved.
///           Once reserved, the unit number will not be returned
///           by the routines FNDLUN or GETLUN, even if it is not
///           connected to a file.
///
///           On the VAX, SUN, PC, and HP logical units 5-7 are
///           reserved by default. On the Macintosh logical units
///           5,6 and 9 are reserved by default.
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
///  FNDLUN maintains an array of logical flags, one for each positive
///  unit number offered by the system. RESLUN sets the value of the
///  flag for UNIT to .TRUE.
///
///  Once reserved, units (except units reserved by default) may be
///  unreserved by calling FRELUN.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of RESLUN.
///
///        C
///        C     Units 17-23 are used by non-NAIF file readers.
///        C     Reserve these, so that they will not be returned
///        C     by FNDLUN or GETLUN.
///        C
///              DO I = 17, 23
///                 CALL RESLUN ( I )
///              END DO
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Programming in VAX FORTRAN", Digital Equipment Corporation,
///       Section 11.1.1, p.11-2, September 1984.
///
///  [2]  "Microsoft FORTRAN Reference", Microsoft Corporation
///       Section 3.2.2, p.61, 1989,
///
///  [3]  "Sun FORTRAN Programmer's Guide", Sun Microsystems,
///       Revision A, Section 7.2, p.73, 6 May 1988
///
///  [4]  "Language Systems FORTRAN Reference Manual", Version 2.1,
///       page 193.
///
///  [5]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       page 94.
///
///  [6]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers", First
///       Edition, Hewlett Packard Company, pp.6-2 and 6-4, June 1991.
///
///  [7]  Silicon Graphics "Fortran 77 Programmer's Guide",
///       Document number 007-0711-030, page 1-20.
///
///  [8]  "Language Reference Manual", Absoft Fortran V3.2, section
///       7.3.1 (for the NeXT), p.7-4, 1993.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
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
/// -    SPICELIB Version 6.1.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Literature_References.
///
/// -    SPICELIB Version 6.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 6.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 6.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 16-MAR-1992 (MJS)
///
///         RESVD is now initialized on entry to this routine if
///         it hasn't been done previously.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (IMU) (HAN) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-MAR-1992 (MJS)
///
///         RESVD is now initialized on entry to this routine if
///         it hasn't been done previously. Prior to this fix, any actions
///         taken by RESLUN or FRELUN before FNDLUN was called would have
///         been discarded. FIRST is now checked on entry to all entry
///         points.
///
/// -    Beta Version 1.1.0, 27-FEB-1989 (HAN) (NJB)
///
///         This routine is now an entry point of FNDLUN rather than
///         GETLUN. The code of this entry point itself has not changed
///         however. References to the routine FNDLUN were added to the
///         header. The $Restrictions section was updated to read "none."
///         This module was declared "error free", which means
///         that it will never participate in error handling.
/// ```
pub fn reslun(ctx: &mut SpiceContext, unit: i32) {
    RESLUN(unit, ctx.raw_context());
}

//$Procedure RESLUN ( Reserve a logical unit )
pub fn RESLUN(UNIT: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Initialize RESVD if it hasn't already been done.
    //
    if save.FIRST {
        for I in MINLUN..=MAXLUN {
            save.RESVD[I] = false;
        }

        for I in 1..=NRESV {
            save.RESVD[save.RESNUM[I]] = true;
        }

        save.FIRST = false;
    }

    //
    // If UNIT is in the proper range, set the corresponding flag
    // to TRUE.
    //
    if ((UNIT >= MINLUN) && (UNIT <= MAXLUN)) {
        save.RESVD[UNIT] = true;
    }
}

/// Free a reserved logical unit
///
/// Free a logical unit number reserved by RESLUN.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Number of the logical unit to be unreserved.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the number of the logical unit to be unreserved.
///           Once unreserved, the unit number may be returned by
///           the routines GETLUN or FNDLUN whenever not connected to
///           a file.
///
///           On the VAX, SUN, PC, and HP logical units 5-7 are
///           reserved by default. On the Macintosh logical units
///           5,6 and 9 are reserved by default. These may not be
///           unreserved.
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
///  FNDLUN maintains an array of logical flags, one for each unit
///  offered by the system. FRELUN sets the value of the flag for
///  UNIT to .FALSE.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of FRELUN.
///
///        C
///        C     Free the units used by the non-NAIF file readers,
///        C     so that they may be returned by FNDLUN or GETLUN.
///        C
///              DO I = 17, 23
///                 CALL FRELUN ( I )
///              END DO
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Programming in VAX FORTRAN", Digital Equipment Corporation,
///       Section 11.1.1, p.11-2, September 1984.
///
///  [2]  "Microsoft FORTRAN Reference", Microsoft Corporation
///       Section 3.2.2, p.61, 1989,
///
///  [3]  "Sun FORTRAN Programmer's Guide", Sun Microsystems,
///       Revision A, Section 7.2, p.73, 6 May 1988
///
///  [4]  "Language Systems FORTRAN Reference Manual", Version 2.1,
///       page 193.
///
///  [5]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       page 94.
///
///  [6]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers", First
///       Edition, Hewlett Packard Company, pp.6-2 and 6-4, June 1991.
///
///  [7]  Silicon Graphics "Fortran 77 Programmer's Guide",
///       Document number 007-0711-030, page 1-20.
///
///  [8]  "Language Reference Manual", Absoft Fortran V3.2, section
///       7.3.1 (for the NeXT), p.7-4, 1993.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
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
/// -    SPICELIB Version 6.1.0, 27-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Literature_References.
///
/// -    SPICELIB Version 6.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 6.0.3, 24-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 6.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 6.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 16-MAR-1992 (MJS)
///
///         RESVD is now initialized on entry to this routine if
///         it hasn't been done previously.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 12-MAR-1991 (MJS)
///
///         The array RESNUM now contains the default reserved
///         logical units. All the elements of the array RESVD
///         were initialized.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (IMU) (HAN) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-MAR-1992 (MJS)
///
///         RESVD is now initialized on entry to this routine if
///         it hasn't been done previously. Prior to this fix, any actions
///         taken by RESLUN or FRELUN before FNDLUN was called would have
///         been discarded. FIRST is now checked on entry to all entry
///         points.
///
/// -    Beta Version 1.1.0, 27-FEB-1989 (HAN) (NJB)
///
///         This routine is now an entry point of FNDLUN rather than
///         GETLUN. The code of this entry point itself has not changed
///         however. References to the routine FNDLUN were added to the
///         header. The $Restrictions section was updated to read "none."
///         This module was declared "error free", which means
///         that it will never participate in error handling.
/// ```
pub fn frelun(ctx: &mut SpiceContext, unit: i32) {
    FRELUN(unit, ctx.raw_context());
}

//$Procedure FRELUN ( Free a reserved logical unit )
pub fn FRELUN(UNIT: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Initialize RESVD if it hasn't already been done.
    //
    if save.FIRST {
        for I in MINLUN..=MAXLUN {
            save.RESVD[I] = false;
        }

        for I in 1..=NRESV {
            save.RESVD[save.RESNUM[I]] = true;
        }

        save.FIRST = false;
    }

    //
    // If UNIT is in the proper range and it has not been reserved by
    // default, set the corresponding flag to FALSE.
    //
    if ((UNIT >= MINLUN) && (UNIT <= MAXLUN)) {
        for I in 1..=NRESV {
            if (UNIT == save.RESNUM[I]) {
                return;
            }
        }

        save.RESVD[UNIT] = false;
    }
}
