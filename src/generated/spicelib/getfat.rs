//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const LBCELL: i32 = -5;
const IDLEN: i32 = 12;
const FILSIZ: i32 = 255;
const WDSIZE: i32 = 32;
const MINPCH: i32 = 32;
const MAXPCH: i32 = 126;

/// Get file architecture and type
///
/// Determine the architecture and type of SPICE kernels.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   The name of a file to be examined.
///  ARCH       O   The architecture of the kernel file.
///  KERTYP     O   The type of the kernel file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of a SPICE kernel file whose architecture
///           and type are desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARCH     is the file architecture of the SPICE kernel file
///           specified by FILE. If the architecture cannot be
///           determined or is not recognized the value '?' is
///           returned.
///
///           Architectures currently recognized are:
///
///              DAF -- The file is based on the DAF architecture.
///              DAS -- The file is based on the DAS architecture.
///              XFR -- The file is in a SPICE transfer file format.
///              DEC -- The file is an old SPICE decimal text file.
///              ASC -- An ASCII text file.
///              KPL -- Kernel Pool File (i.e., a text kernel)
///              TXT -- An ASCII text file.
///              TE1 -- Text E-Kernel type 1.
///              ?   -- The architecture could not be determined.
///
///           This variable must be at least 3 characters long.
///
///  KERTYP   is the type of the SPICE kernel file. If the type
///           can not be determined the value '?' is returned.
///
///           Kernel file types may be any sequence of at most four
///           printing characters. NAIF has reserved for its use
///           types which contain all upper case letters.
///
///           A file type of 'PRE' means that the file is a
///           pre-release file.
///
///           This variable may be at most 4 characters long.
/// ```
///
/// # Parameters
///
/// ```text
///  RECL     is the record length of a binary kernel file. Each
///           record must be large enough to hold 128 double
///           precision numbers. The units in which the record
///           length must be specified vary from environment to
///           environment. For example, VAX Fortran requires
///           record lengths to be specified in longwords,
///           where two longwords equal one double precision
///           number.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the filename specified is blank, the error
///      SPICE(BLANKFILENAME) is signaled.
///
///  2)  If any inquire on the filename specified by FILE, required to
///      obtain information about the physical file, fails for some
///      reason, the error SPICE(INQUIREERROR) is signaled.
///
///  3)  If the file specified by FILE does not exist, the error
///      SPICE(FILENOTFOUND) is signaled.
///
///  4)  If the file specified by FILE is already open but not through
///      SPICE interfaces, the error SPICE(EXTERNALOPEN) is signaled.
///
///  5)  If an attempt to open the file specified by FILE fails when
///      this routine requires that it succeed, the error
///      SPICE(FILEOPENFAILED) is signaled.
///
///  6)  If an attempt to read the file specified by FILE fails when
///      this routine requires that it succeed, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  7)  If an issue is detected during the opening the input file or
///      the process to determine its architecture and type, an error
///      is signaled by a routine in the call tree of this routine.
///
///  8)  If the ID word in a DAF based kernel is 'NAIF/DAF', then the
///      algorithm GETFAT uses to distinguish between CK and SPK
///      kernels may result in an indeterminate KERTYP if the SPK or
///      CK files have invalid first segments.
/// ```
///
/// # Files
///
/// ```text
///  The SPICE kernel file specified by FILE is opened and then
///  closed by this routine to determine its file architecture and
///  type. Filenames of open files should not be passed to this
///  routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is a support utility routine that determines the
///  architecture and type of a SPICE kernel file.
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
///  1) Determine the file architecture and file type of all the
///     kernels loaded through a meta-kernel, and of a kernel in
///     transfer format.
///
///     Use the SPK kernel below to provide an example of a kernel in
///     transfer format.
///
///        earthstns_itrf93_050714.xsp
///
///
///     Use the meta-kernel shown below to load the other types of
///     SPICE kernels.
///
///
///        KPL/MK
///
///        File: getfat_ex1.tm
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
///           File name                        Contents
///           ---------                        --------
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           pck00010.tpc                     Planet orientation and
///                                            radii
///           naif0011.tls                     Leapseconds
///           mgs_moc_v20.ti                   MGS MOC instrument
///                                            parameters
///           mgs_sclkscet_00061.tsc           MGS SCLK coefficients
///           mgs_sc_ext12.bc                  MGS s/c bus attitude
///           mgs_ext12_ipng_mgs95j.bsp        MGS ephemeris
///           megr90n000cb_plate.bds           Plate model based on
///                                            MEGDR DEM, resolution
///                                            4 pixels/degree.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'mgs_moc_v20.ti',
///                               'mgs_sclkscet_00061.tsc',
///                               'mgs_sc_ext12.bc',
///                               'mgs_ext12_ipng_mgs95j.bsp',
///                               'megr90n000cb_plate.bds'      )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM GETFAT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               ARCHLN
///           PARAMETER           ( ARCHLN = 4 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 256 )
///
///           INTEGER               KTYPLN
///           PARAMETER           ( KTYPLN = 5 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(ARCHLN)    ARCH
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(28)        FNAME1
///           CHARACTER*(KTYPLN)    KTYPE
///           CHARACTER*(FILSIZ)    SOURCE
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               I
///
///           LOGICAL               FOUND
///
///     C
///     C     Check the file architecture and type of an SPK
///     C     in transfer format.
///     C
///           FNAME1 = 'earthstns_itrf93_050714.xsp'
///
///           CALL GETFAT ( FNAME1, ARCH, KTYPE )
///
///           WRITE(*,*) 'File name     : ', FNAME1
///           WRITE(*,*) '  Architecture: ', ARCH
///           WRITE(*,*) '  Kernel type : ', KTYPE
///           WRITE(*,*) ' '
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( 'getfat_ex1.tm' )
///
///     C
///     C     Get the file architecture and kernel type for each of
///     C     the kernels in the kernel pool.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           DO I= 1, COUNT
///
///              CALL KDATA ( I,     'ALL', FNAME, KTYPE, SOURCE,
///          .                HANDLE, FOUND                      )
///
///              CALL GETFAT ( FNAME, ARCH, KTYPE )
///
///              WRITE(*,*) 'File name     : ', FNAME
///              WRITE(*,*) '  Source      : ', SOURCE
///              WRITE(*,*) '  Architecture: ', ARCH
///              WRITE(*,*) '  Kernel type : ', KTYPE
///              WRITE(*,*) ' '
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
///      File name     : earthstns_itrf93_050714.xsp
///        Architecture: XFR
///        Kernel type : DAF
///
///      File name     : getfat_ex1.tm
///        Source      :
///        Architecture: KPL
///        Kernel type : MK
///
///      File name     : de430.bsp
///        Source      : getfat_ex1.tm
///        Architecture: DAF
///        Kernel type : SPK
///
///      File name     : mar097.bsp
///        Source      : getfat_ex1.tm
///        Architecture: DAF
///        Kernel type : SPK
///
///      File name     : pck00010.tpc
///        Source      : getfat_ex1.tm
///        Architecture: KPL
///        Kernel type : PCK
///
///      File name     : naif0011.tls
///        Source      : getfat_ex1.tm
///        Architecture: KPL
///        Kernel type : LSK
///
///      File name     : mgs_moc_v20.ti
///        Source      : getfat_ex1.tm
///        Architecture: KPL
///        Kernel type : IK
///
///      File name     : mgs_sclkscet_00061.tsc
///        Source      : getfat_ex1.tm
///        Architecture: KPL
///        Kernel type : SCLK
///
///      File name     : mgs_sc_ext12.bc
///        Source      : getfat_ex1.tm
///        Architecture: DAF
///        Kernel type : CK
///
///      File name     : mgs_ext12_ipng_mgs95j.bsp
///        Source      : getfat_ex1.tm
///        Architecture: DAF
///        Kernel type : SPK
///
///      File name     : megr90n000cb_plate.bds
///        Source      : getfat_ex1.tm
///        Architecture: DAS
///        Kernel type : DSK
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  In order to properly determine the type of DAF based binary
///      kernels, the routine requires that their first segments and
///      the meta data necessary to address them are valid.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 5.0.1, 07-AUG-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example to $Examples section.
///
/// -    SPICELIB Version 5.0.0, 05-FEB-2015 (NJB)
///
///         Updated to support integration of DAS into the
///         handle manager subsystem. Now opened DAS files
///         must be known to that subsystem; if this routine
///         encounters an open, unrecognized DAS file, an
///         error is signaled.
///
///         Corrected various typos in comments.
///
/// -    SPICELIB Version 4.25.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 4.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 4.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 4.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 4.21.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 4.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 4.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 4.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 4.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 4.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 4.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 4.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 4.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 4.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 4.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 4.10.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 4.9.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 4.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 4.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 4.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 4.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 4.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 4.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 4.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 4.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 4.0.2, 24-APR-2003 (EDW)
///
///         Added MAC-OSX-F77 to the list of platforms
///         that require READONLY to read write protected
///         kernels.
///
/// -    SPICELIB Version 4.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 4.0.0, 22-AUG-2001 (WLT) (FST) (EDW)
///
///         Added code so that the architecture and type of open binary
///         SPICE kernels can be determined.
///
///         Added exception for MACPPC_C (CodeWarrior Mac classic).
///         Reduced RECL value to 12 to prevent expression of
///         the fseek bug.
///
/// -    SPICELIB Version 3.2.0, 06-DEC-1999 (WLT)
///
///         The heuristics for distinguishing between CK and SPK have
///         been enhanced so that the routine is no longer requires
///         that TICKS in C-kernels be positive or integral.
///
/// -    SPICELIB Version 3.1.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 3.1.3, 22-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
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
/// -    SPICELIB Version 3.1.0, 11-FEB-1999 (FST)
///
///         Added an integrality check to Test 3. If LASTDP is not
///         an integral value, then GETFAT simply returns KERTYP = '?',
///         since it is of an indeterminate type.
///
/// -    SPICELIB Version 3.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 19-DEC-1995 (KRG)
///
///         Added several new features to the subroutine:
///
///          - Error handling has been enhanced.
///          - Several new file architectures have been added.
///
///         Removed the mention of 1000 characters as a candidate for the
///         record length of a file.
///
///         Added the exception for a blank filename to the header. The
///         error is signaled, but it was not listed in the header.
///
///         Added IOSTAT values to the appropriate error messages.
///
///         Non-printing characters are replaced with blanks in the ID
///         word when it is read. This deals with the case where a
///         platform allows a text file to be opened as an unformatted
///         file and the ID word does not completely fill 8 characters.
///
/// -    SPICELIB Version 1.4.0, 05-JAN-1995 (HAN)
///
///         Removed ENV11 since it is now the same as ENV2.
///         Removed ENV10 since it is the same as the VAX environment.
///
/// -    SPICELIB Version 1.3.0, 30-AUG-1994 (HAN)
///
///         Added two new environments, DEC Alpha/OpenVMS and
///         Sun/Solaris, to the source master file.
///
/// -    SPICELIB Version 1.2.0, 25-MAR-1994 (HAN)
///
///         Added two new environments, DEC Alpha/OpenVMS and
///         Sun/Solaris, to the source master file.
///
/// -    SPICELIB Version 1.1.0, 25-MAR-1994 (HAN)
///
///         Modified master source code file to use READONLY on platforms
///         that support it. Also, changed some local declaration comment
///         lines to match the standard NAIF template.
///
/// -    SPICELIB Version 1.0.0, 24-JUL-1993 (WLT) (HAN) (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.0.0, 22-AUG-2001 (WLT) (FST) (EDW)
///
///         Added code so that the architecture and type of open binary
///         SPICE kernels can be determined. This uses the new DAF/DAS
///         handle manager as well as examination of handles of open DAS
///         files. Currently the handle manager deals only with DAF
///         files. This routine should be updated again when the DAS
///         system is integrated with the handle manager.
///
///         Some slight changes were required to support ZZDDHFNH on
///         the VAX environment. This resulted in the addition of
///         the logical USEFNH that is set to true in most
///         environments, and never used again other than to allow
///         the invocation of the ZZDDHFNH module.
///
/// -    SPICELIB Version 2.0.0, 19-DEC-1995 (KRG)
///
///         Added several new features to the subroutine:
///
///          - Error handling has been enhanced.
///          - Several new file architectures have been added.
///
///         Removed the mention of 1000 characters as a candidate for the
///         record length of a file. It seems unlikely that we will
///         encounter an environment where 1000 characters of storage is
///         larger than the storage necessary for 128 double precision
///         numbers; typically there are 8 characters per double precision
///         number, yielding 1024 characters.
///
///         Added the exception for a blank filename to the header. The
///         error is signaled, but it was not listed in the header.
///
///         Added IOSTAT values to the appropriate error messages.
///
///         Non-printing characters are replaced with blanks in the ID
///         word when it is read. This deals with the case where a
///         platform allows a text file to be opened as an unformatted
///         file and the ID word does not completely fill 8 characters.
/// ```
pub fn getfat(
    ctx: &mut SpiceContext,
    file: &str,
    arch: &mut str,
    kertyp: &mut str,
) -> crate::Result<()> {
    GETFAT(
        file.as_bytes(),
        fstr::StrBytes::new(arch).as_mut(),
        fstr::StrBytes::new(kertyp).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GETFAT ( Get file architecture and type )
pub fn GETFAT(
    FILE: &[u8],
    ARCH: &mut [u8],
    KERTYP: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FILARC = [b' '; WDSIZE as usize];
    let mut FNAME = [b' '; FILSIZ as usize];
    let mut IDWORD = [b' '; IDLEN as usize];
    let mut TMPWRD = [b' '; IDLEN as usize];
    let mut HANDLE: i32 = 0;
    let mut INTAMN: i32 = 0;
    let mut INTARC: i32 = 0;
    let mut INTBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut DIROPN: bool = false;
    let mut EXIST: bool = false;
    let mut FOUND: bool = false;
    let mut OPENED: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Set the length of a SPICE kernel file ID word.
    //

    //
    // Set minimum and maximum values for the range of ASCII printing
    // characters.
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"GETFAT", ctx)?;
    }

    //
    // Initialize the temporary storage variables that we use.
    //
    fstr::assign(&mut IDWORD, b" ");

    //
    // If the filename we have is blank, signal an error and return.
    //
    if fstr::eq(FILE, b" ") {
        SETMSG(b"The file name is blank.", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"GETFAT", ctx)?;
        return Ok(());
    }

    //
    // See if this is a binary file that is currently open
    // within the SPICE binary file management subsystem.  At
    // the moment, as far as we know, the file is not opened.
    //
    OPENED = false;

    ZZDDHFNH(FILE, &mut HANDLE, &mut FOUND, ctx)?;

    if FOUND {
        //
        // If the file was recognized, we need to get the unit number
        // associated with it.
        //
        ZZDDHNFO(
            HANDLE,
            &mut FNAME,
            &mut INTARC,
            &mut INTBFF,
            &mut INTAMN,
            &mut FOUND,
            ctx,
        )?;

        //
        // Translate the architecture ID to a string and retrieve the
        // logical unit to use with this file.
        //
        ZZDDHGSD(b"ARCH", INTARC, &mut FILARC, ctx);
        ZZDDHHLU(HANDLE, &FILARC, false, &mut NUMBER, ctx)?;

        OPENED = true;
    } else {
        //
        // We'll do a bit of inquiring before we try opening anything.
        //
        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                file: Some(FILE),
                exist: Some(&mut EXIST),
                opened: Some(&mut OPENED),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
        }

        //
        // Not too likely, but if the INQUIRE statement fails...
        //
        if (IOSTAT != 0) {
            SETMSG(b"IOSTAT error in INQUIRE statement. IOSTAT = #.", ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(INQUIREERROR)", ctx)?;
            CHKOUT(b"GETFAT", ctx)?;
            return Ok(());
        }

        //
        // Note: the following two tests MUST be performed in the order
        // in which they appear, since in some environments files that do
        // not exist are considered to be open.
        //
        if !EXIST {
            SETMSG(b"The kernel file \'#\' does not exist.", ctx);
            ERRCH(b"#", FILE, ctx);
            SIGERR(b"SPICE(FILENOTFOUND)", ctx)?;
            CHKOUT(b"GETFAT", ctx)?;
            return Ok(());
        }

        //
        // Reject open files not known to the handle manager subsystem.
        //
        if OPENED {
            //
            // Open files that are not opened within the SPICE
            // binary file management subsystem are forbidden fruit.
            // All we can do is signal an error letting the caller
            // know that we are helpless in this case.
            //
            SETMSG(b"The file \'#\' is already open.", ctx);
            ERRCH(b"#", FILE, ctx);
            SIGERR(b"SPICE(EXTERNALOPEN)", ctx)?;
            CHKOUT(b"GETFAT", ctx)?;
            return Ok(());
        }
    }

    //
    // Open the file with a record length of RECL (the length of the
    // DAF and DAS records). We assume, for now, that opening the file as
    // a direct access file will work.
    //
    DIROPN = true;

    //
    // If the file is not already open (probably the case that
    // happens most frequently) we try opening it for direct access
    // and see if we can locate the idword.
    //
    if !OPENED {
        GETLUN(&mut NUMBER, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(NUMBER),
                file: Some(FILE),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        //
        // If we had trouble opening the file, try opening it as a
        // sequential file.
        //
        if (IOSTAT != 0) {
            DIROPN = false;

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(NUMBER),
                    file: Some(FILE),
                    access: Some(b"SEQUENTIAL"),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }

            //
            // If we still have problems opening the file, we don't have a
            // clue about the file architecture and type.
            //
            if (IOSTAT != 0) {
                fstr::assign(ARCH, b"?");
                fstr::assign(KERTYP, b"?");
                SETMSG(b"Attempt to open the file \'#\' failed. IOSTAT = #.", ctx);
                ERRCH(b"#", FILE, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
                CHKOUT(b"GETFAT", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // We opened the file successfully, so let's try to read from the
    // file. We need to be sure to use the correct form of the read
    // statement, depending on whether the file was opened with direct
    // access or sequential access.
    //
    if DIROPN {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(NUMBER)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut TMPWRD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        //
        // If we couldn't read from the file as a direct access file with
        // a fixed record length, then try to open the file as a
        // sequential file and read from it.
        //
        if (IOSTAT != 0) {
            if OPENED {
                //
                // Something has gone wrong here.  The file was opened
                // as either a DAF or DAS prior to the call to GETFAT.
                // We retrieved the unit number maintained by the
                // underlying binary file management system, but we
                // were unable to read the file as direct access.
                // There's nothing we can do but abandon our quest to
                // determine the type of the file.
                //
                SETMSG(b"The file \'#\' is opened as a binary SPICE kernel.  But it cannot be read using a direct access read. The value of IOSTAT returned by the attempted READ is #. ", ctx);
                ERRCH(b"#", FILE, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
                CHKOUT(b"GETFAT", ctx)?;
                return Ok(());
            }

            //
            // If we reach this point, the file was opened locally
            // as a direct access file.  We could not read it that
            // way, so we'll try using a sequential read.   However,
            // we first need to close the file and then reopen it
            // for sequential reading.
            //
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(NUMBER),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(NUMBER),
                    file: Some(FILE),
                    access: Some(b"SEQUENTIAL"),
                    status: Some(b"OLD"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }

            //
            // If we could not open the file, we don't have a clue about
            // the file architecture and type.
            //
            if (IOSTAT != 0) {
                fstr::assign(ARCH, b"?");
                fstr::assign(KERTYP, b"?");
                SETMSG(b"Attempt to open the file \'#\' failed. IOSTAT = #.", ctx);
                ERRCH(b"#", FILE, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
                CHKOUT(b"GETFAT", ctx)?;
                return Ok(());
            }

            //
            // Try to read from the file.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::FormattedReader::new(ctx.io_unit(NUMBER)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut TMPWRD)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }
        }
    } else {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(NUMBER)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut TMPWRD)?;
                reader.finish()?;
                Ok(())
            })?;
        }
    }

    //
    // If we had an error while reading, we don't recognize this file.
    //
    if (IOSTAT != 0) {
        fstr::assign(ARCH, b"?");
        fstr::assign(KERTYP, b"?");
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        SETMSG(b"Attempt to read from file \'#\' failed. IOSTAT = #.", ctx);
        ERRCH(b"#", FILE, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"GETFAT", ctx)?;
        return Ok(());
    }

    //
    // Close the file (if we opened it here), as we do not need it
    // to be open any more.
    //
    if !OPENED {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(NUMBER),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // At this point, we have a candidate for an ID word. To avoid
    // difficulties with Fortran I/O and other things, we will now
    // replace any non printing ASCII characters with blanks.
    //
    for I in 1..=IDLEN {
        if ((intrinsics::ICHAR(fstr::substr(&TMPWRD, I..=I)) < MINPCH)
            || (intrinsics::ICHAR(fstr::substr(&TMPWRD, I..=I)) > MAXPCH))
        {
            fstr::assign(fstr::substr_mut(&mut TMPWRD, I..=I), b" ");
        }
    }

    //
    // Identify the architecture and type, if we can.
    //
    LJUST(&TMPWRD.clone(), &mut TMPWRD);
    UCASE(&TMPWRD.clone(), &mut TMPWRD, ctx);
    NEXTWD(&TMPWRD.clone(), &mut IDWORD, &mut TMPWRD);

    if fstr::eq(&IDWORD, b"DAFETF") {
        //
        // We have a DAF encoded transfer file.
        //
        fstr::assign(ARCH, b"XFR");
        fstr::assign(KERTYP, b"DAF");
    } else if fstr::eq(&IDWORD, b"DASETF") {
        //
        // We have a DAS encoded transfer file.
        //
        fstr::assign(ARCH, b"XFR");
        fstr::assign(KERTYP, b"DAS");
    } else if fstr::eq(fstr::substr(&IDWORD, 1..=10), b"\'NAIF/DAF\'") {
        //
        // We have an old DAF decimal text file.
        //
        fstr::assign(ARCH, b"DEC");
        fstr::assign(KERTYP, b"DAF");
    } else if fstr::eq(fstr::substr(&IDWORD, 1..=8), b"NAIF/DAS") {
        //
        // We have a pre release DAS binary file.
        //
        fstr::assign(ARCH, b"DAS");
        fstr::assign(KERTYP, b"PRE");
    } else {
        //
        // Get the architecture and type from the ID word, if we can.
        //
        IDW2AT(fstr::substr(&IDWORD, 1..=8), ARCH, KERTYP, ctx)?;
    }

    //
    // If the architecture is DAF and the type is unknown, '?', then we
    // have either an SPK file, a CK file, or something we don't
    // understand. Let's check it out.
    //
    if (fstr::eq(ARCH, b"DAF") && fstr::eq(KERTYP, b"?")) {
        //
        // We have a DAF file and we do not know what the type is. This
        // situation can occur for older SPK and CK files, before the ID
        // word was used to store type information.
        //
        // We use Bill's (WLT'S) magic heuristics to determine the type
        // of the file.
        //
        // Open the file and pass the handle to the private routine
        // that deals with the dirty work.
        //
        DAFOPR(FILE, &mut HANDLE, ctx)?;
        ZZCKSPK(HANDLE, KERTYP, ctx)?;
        DAFCLS(HANDLE, ctx)?;
    }

    CHKOUT(b"GETFAT", ctx)?;
    Ok(())
}
