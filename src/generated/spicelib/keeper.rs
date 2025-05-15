//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const FILSIZ: i32 = 255;
pub const FTSIZE: i32 = 5000;
pub const MAXTXT: i32 = 300;
pub const MAXFIL: i32 = (FTSIZE + MAXTXT);
const TYPLEN: i32 = 8;
const WDSIZE: i32 = 32;
const NKNOWN: i32 = 3;
const LNSIZE: i32 = 80;
const MSGSIZ: i32 = 500;

struct SaveVars {
    FILES: ActualCharArray,
    HANDLS: ActualArray<i32>,
    SRCES: ActualArray<i32>,
    TYPES: ActualCharArray,
    LOADED: i32,
    KNOWN: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FILES = ActualCharArray::new(FILSIZ, 1..=MAXFIL);
        let mut HANDLS = ActualArray::<i32>::new(1..=MAXFIL);
        let mut SRCES = ActualArray::<i32>::new(1..=MAXFIL);
        let mut TYPES = ActualCharArray::new(TYPLEN, 1..=MAXFIL);
        let mut LOADED: i32 = 0;
        let mut KNOWN = ActualCharArray::new(WDSIZE, 1..=NKNOWN);
        let mut FIRST: bool = false;

        FIRST = true;
        LOADED = 0;

        Self {
            FILES,
            HANDLS,
            SRCES,
            TYPES,
            LOADED,
            KNOWN,
            FIRST,
        }
    }
}

/// Keeps track of SPICE kernels
///
/// This routine is an umbrella for a collection of entry points
/// that manage the loading and unloading of SPICE kernels from
/// an application program.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  WHICH      I   KDATA
///  KIND       I   KTOTAL, KDATA
///  FILE      I-O  FURNSH, KDATA, UNLOAD, KINFO
///  COUNT      O   KTOTAL
///  FILTYP    I-O  KTOTAL, KDATA, KINFO
///  HANDLE     O   KDATA, KINFO
///  SRCFIL     O   KDATA, KINFO
///  FOUND      O   KDATA, KINFO
///  FILSIZ     P   Maximum file name length.
///  MAXFIL     P   Maximum number of files that can be loaded.
/// ```
///
/// # Detailed Input
///
/// ```text
///  See Individual Entry points.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See Individual Entry points.
/// ```
///
/// # Parameters
///
/// ```text
///  FILSIZ   is the maximum file name length that can be
///           accommodated by this set of routines.
///
///
///  MAXFIL   is the number of entries that can be stored in KEEPER's
///           kernel database. In this version of the toolkit MAXFIL
///           is set to 5300. Each time a kernel is loaded via
///           FURNSH, a database entry is created for that kernel.
///           If a meta-kernel is loaded, a database entry is created
///           for the meta-kernel itself and for all files referenced
///           in the meta-kernel's KERNELS_TO_LOAD specification.
///           Unloading a kernel or meta-kernel deletes database
///           entries created when the file was loaded.
///
///           The parameter MAXFIL is an upper bound on number of
///           SPICE kernels that can be loaded at any time via the
///           KEEPER interface, but the number of kernels that can be
///           loaded may be smaller, since re-loading a loaded kernel
///           or meta-kernel results in creation of additional
///           database entries.
///
///           Kernels loaded into the KEEPER system are subject to
///           constraints imposed by lower-level subsystems. The
///           binary kernel systems (SPK, CK, binary PCK, and EK)
///           have their own limits on the maximum number of kernels
///           that may be loaded.
///
///           The total number of DAF-based files (this set includes
///           SPKs, CKs, and binary PCKs) and DAS-based files (this
///           set includes EKs and DSKs) that may be loaded at any
///           time may not exceed 5000. This limit applies whether
///           the files are loaded via FURNSH or lower-level loaders
///           such as SPKLEF or DAFOPR. File access performance
///           normally will degrade as the number of loaded kernels
///           increases.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the main routine KEEPER is called, the error
///      SPICE(BOGUSENTRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine serves as an umbrella for a collection of
///  entry points that unify the task of loading, tracking,
///  and unloading SPICE kernels. A description of each entry
///  point is given below:
///
///  FURNSH    Furnish a kernel to a program. This entry point
///            provides a single interface for loading kernels into
///            your application program. All SPICE kernels (Text
///            kernels, SPK, CK, Binary PCK, and EK) can be loaded
///            through this entry point. In addition, special text
///            kernels, called meta-Text kernels, that contain a list
///            of other kernels to load can be processed by FURNSH.
///
///            Meta-text kernels allow you to easily control which
///            kernels will be loaded by your program without having
///            to write your own kernel managing routines.
///
///  KTOTAL    returns the number of kernels that are currently
///            available to your program as a result of previous calls
///            to FURNSH and UNLOAD.
///
///  KDATA     provides an interface for retrieving (in order of their
///            specification through FURNSH) kernels that are active in
///            your application.
///
///  KINFO     allows you to retrieve information about a loaded
///            kernel using the name of that kernel.
///
///  KCLEAR    Unloads all kernels that were loaded via the KEEPER
///            system, clears the kernel pool, and re-initializes the
///            KEEPER system.
///
///  UNLOAD    provides an interface for unloading kernels that have
///            been loaded via the routine FURNSH.
///
///  For more details concerning any particular entry point, see the
///  header for that entry point.
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
///  1) Load a meta-kernel with a PCK, an LSK and an SPK, and
///     separately, a text kernel and a binary PCK. Loop over the
///     loaded kernels, outputting file information for each of
///     them.
///
///     Then unload the text kernels, check that they have been
///     unloaded, and finally unload all remaining kernels
///     and clear the kernel pool using KCLEAR.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: keeper_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls',
///                               'pck00009.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Use the PCK kernel below as the binary PCK required for the
///     example.
///
///        earth_latest_high_prec.bpc
///
///
///     Use the FK kernel below as the text kernel required for the
///     example.
///
///        RSSD0002.TF
///
///
///     Example code begins here.
///
///
///           PROGRAM KEEPER_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER               FNAMLN
///           PARAMETER           ( FNAMLN = 256 )
///
///           INTEGER               FTYPLN
///           PARAMETER           ( FTYPLN = 33 )
///
///           INTEGER               SRCLEN
///           PARAMETER           ( SRCLEN = 256 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FNAMLN)    FILE
///           CHARACTER*(FTYPLN)    FILTYP
///           CHARACTER*(SRCLEN)    SRCFIL
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               WHICH
///
///           LOGICAL               FOUND
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'keeper_ex1.tm'              )
///           CALL FURNSH ( 'RSSD0002.TF'                )
///           CALL FURNSH ( 'earth_latest_high_prec.bpc' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'final FURNSH: ', COUNT
///           WRITE(*,*) ' '
///
///     C
///     C     Retrieve the data for all the loaded kernels and
///     C     test an index for which there should be no kernel.
///     C
///           WRITE(*,'(A)') 'Overview of loaded kernels, by index:'
///           WRITE(*,*) ' '
///
///           DO WHICH= 1, COUNT + 1
///
///              CALL KDATA ( WHICH, 'ALL',   FILE, FILTYP,
///          .                SRCFIL, HANDLE, FOUND        )
///
///              IF ( FOUND ) THEN
///
///                 WRITE(*,*) '  Index : ', WHICH
///                 WRITE(*,*) '  File  : ', FILE
///                 WRITE(*,*) '  Type  : ', FILTYP
///                 WRITE(*,*) '  Source: ', SRCFIL
///                 WRITE(*,*) '  Handle: ', HANDLE
///                 WRITE(*,*) ' '
///
///              ELSE
///
///                 WRITE(*,*) '  No kernel found with index: ', WHICH
///
///              END IF
///
///           END DO
///
///     C
///     C     Unload the text kernels.
///     C
///           CALL KTOTAL ( 'TEXT', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2,A)') 'Unloading', COUNT,
///          .                    ' text kernels...'
///           WRITE(*,*) ' '
///
///           DO WHILE ( COUNT .NE. 0 )
///
///              CALL KDATA (      1, 'TEXT',  FILE, FILTYP,
///          .                SRCFIL, HANDLE, FOUND        )
///
///     C
///     C        If the kernel is found in the pool, unload it.
///     C
///              IF ( FOUND ) THEN
///
///                 CALL UNLOAD ( FILE )
///
///     C
///     C           Check if the file has been unloaded.
///     C
///                 CALL KINFO ( FILE, FILTYP, SRCFIL, HANDLE, FOUND )
///
///                 IF ( FOUND ) THEN
///
///                    WRITE(*,'(A)') '  Error unloading ' // FILE
///
///                 ELSE
///
///                    WRITE(*,'(A)') '  Success unloading ' // FILE
///
///                 END IF
///
///     C
///     C        Something is not working. Inform NAIF.
///     C
///              ELSE
///
///                 WRITE(*,*) ' ERROR: No kernel found with index: ',
///          .                 WHICH
///
///              END IF
///
///     C
///     C        Check if we have more text kernels to unload from
///     C        the kernel pool. Note that unloading a text kernel
///     C        or meta-kernel implies that the kernel pool is
///     C        cleared, and any kernel(s) that were not to be
///     C        unloaded are re-loaded. Therefore the COUNT value
///     C        changes, and the indexing of the files within the
///     C        kernel pool too.
///     C
///              CALL KTOTAL ( 'TEXT', COUNT )
///
///           END DO
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'UNLOAD calls: ', COUNT
///
///     C
///     C     Clear the KEEPER system, retrieve the number of loaded
///     C     after the clear.
///     C
///           CALL KCLEAR()
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'KCLEAR      : ', COUNT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The total number of kernels after final FURNSH:  6
///
///     Overview of loaded kernels, by index:
///
///        Index :            1
///        File  : keeper_ex1.tm
///        Type  : META
///        Source:
///        Handle:            0
///
///        Index :            2
///        File  : de421.bsp
///        Type  : SPK
///        Source: keeper_ex1.tm
///        Handle:            1
///
///        Index :            3
///        File  : naif0012.tls
///        Type  : TEXT
///        Source: keeper_ex1.tm
///        Handle:            0
///
///        Index :            4
///        File  : pck00009.tpc
///        Type  : TEXT
///        Source: keeper_ex1.tm
///        Handle:            0
///
///        Index :            5
///        File  : RSSD0002.TF
///        Type  : TEXT
///        Source:
///        Handle:            0
///
///        Index :            6
///        File  : earth_latest_high_prec.bpc
///        Type  : PCK
///        Source:
///        Handle:            2
///
///        No kernel found with index:            7
///
///     Unloading 3 text kernels...
///
///       Success unloading naif0012.tls
///       Success unloading pck00009.tpc
///       Success unloading RSSD0002.TF
///
///     The total number of kernels after UNLOAD calls:  3
///
///     The total number of kernels after KCLEAR      :  0
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.0, 29-DEC-2021 (JDR) (NJB)
///
///         Changed argument name SOURCE to SRCFIL for consistency with
///         other routines.
///
///         Updated KEEPER umbrella routine and all entry points' headers
///         to comply with NAIF standard.
///
///         Updated description of input argument KIND in headers of entry
///         points KDATA and KTOTAL. Updated $Brief_I/O table to include
///         WHICH short description and sort arguments in the order they
///         are declared.
///
///         Added a restriction about specifying kernels using relative
///         paths to the FURNSH entry point header $Restrictions section.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Bug fix: now unloads binary kernels via low-level
///         unload routines only when those kernels have just
///         one entry in the KEEPER database.
///
///         Updated description of MAXFIL in the header.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 4.1.0, 01-JUL-2014 (NJB) (BVS)
///
///         Updated the discussion of kernel variable watches in entry
///         points KCLEAR and UNLOAD. Added to the FURNSH header mention
///         of the effects of failure during text kernel or meta-kernel
///         loading.
///
///         Last update was 12-APR-2012 (BVS)
///
///            Increased FTSIZE (from 1000 to 5000).
///
///            Changed to use SEPOOL instead of STPOOL to reduce loading
///            time for large meta-kernels due to n^2 delay in STPOOL.
///
/// -    SPICELIB Version 4.0.2, 13-APR-2011 (EDW)
///
///         Trivial edit to KCLEAR $Restrictions, replaced P*POOL with
///         PXPOOL. The "*" character causes the HTML documentation
///         script to create a link for the "POOL" substring.
///
/// -    SPICELIB Version 4.0.1, 10-FEB-2010 (EDW)
///
///         Added mention of the restriction on kernel pool variable
///         names to MAXLEN (defined in pool.f) characters or less.
///
/// -    SPICELIB Version 4.0.0, 02-APR-2009 (NJB)
///
///         Continued path values are now supported. FURNSH now rejects
///         file names longer than FILSIZ characters.
///
///         Deleted references to unneeded variable DOALL. Made
///         THSTYP declaration compatible with TYPES array.
///
/// -    SPICELIB Version 3.0.1, 27-APR-2007 (NJB)
///
///         Fixed header typo: added quotes to literal string
///         input arguments in example FURNSH calls.
///
/// -    SPICELIB Version 3.0.0, 15-NOV-2006 (NJB)
///
///         Added entry point KCLEAR. Bug fix: meta-kernel unloading bug
///         in UNLOAD was corrected. Some header updates were made.
///
/// -    SPICELIB Version 2.0.2, 29-JUL-2003 (NJB) (CHA)
///
///         Only the header of the entry point FURNSH was modified.
///         Numerous updates were made to improve clarity. Some
///         corrections were made.
///
/// -    SPICELIB Version 2.0.1, 06-DEC-2002 (NJB)
///
///         Typo in header example was corrected.
///
/// -    SPICELIB Version 2.0.0, 07-JAN-2002 (WLT)
///
///         Added a call to CVPOOL in FURNSH so that watches that are
///         triggered are triggered by loading Meta-kernels and not by
///         some external interaction with the kernel pool.
///
///         Added code to make sure that UNLOAD has the effect of
///         loading all remaining kernels in the order they were first
///         introduced.
///
/// -    SPICELIB Version 1.1.0, 19-SEP-2000 (WLT)
///
///         Corrected the error message template used
///         by ZZLDKER
///
/// -    SPICELIB Version 1.0.1, 16-DEC-1999 (NJB)
///
///         Documentation fix: corrected second code example in the
///         header of the entry point FURNSH. The example previously used
///         the kernel variable PATH_NAMES; that name has been replaced
///         with the correct name PATH_VALUES.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn keeper(
    ctx: &mut SpiceContext,
    which: i32,
    kind: &str,
    file: &str,
    count: i32,
    filtyp: &str,
    handle: i32,
    srcfil: &str,
    found: bool,
) -> crate::Result<()> {
    KEEPER(
        which,
        kind.as_bytes(),
        file.as_bytes(),
        count,
        filtyp.as_bytes(),
        handle,
        srcfil.as_bytes(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure KEEPER ( Keeps track of SPICE kernels )
pub fn KEEPER(
    WHICH: i32,
    KIND: &[u8],
    FILE: &[u8],
    COUNT: i32,
    FILTYP: &[u8],
    HANDLE: i32,
    SRCFIL: &[u8],
    FOUND: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Here we set up the database of loaded kernels
    //
    // The name of every file loaded through this interface will
    // be stored in the array FILES.
    //
    //
    // The handle of every loaded file will be stored in the array
    // HANDLS.  If the file is a text kernel it will be assigned the
    // handle 0.
    //
    //
    // The source of each file specified will be stored in the integer
    // array SRCFIL.  If the file is loaded directly, its source
    // will be zero.  If it is loaded as the result of meta-information
    // in a text kernel, the index of the source file in FILES will
    // be stored in SRCES.
    //
    //
    // The file type of every loaded kernel will be stored in the array
    // TYPES.
    //

    //
    // The number of files loaded through this interfaces is kept in the
    // integer LOADED.
    //

    CHKIN(b"KEEPER", ctx)?;
    SETMSG(b"The routine KEEPER is an umbrella for a collection of entry points that manage the loading, tracking and unloading of SPICE kernels.  KEEPER should not be called directly. It is likely that a programming error has been made. ", ctx);
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"KEEPER", ctx)?;
    Ok(())
}

/// Furnish a program with SPICE kernels
///
/// Load one or more SPICE kernels into a program.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   SPICE kernel file (text or binary).
///  FILSIZ     P   Maximum file name length.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is a SPICE kernel file. The file may be either binary
///           or text. If the file is a binary SPICE kernel it will
///           be loaded into the appropriate SPICE subsystem. If
///           FILE is a SPICE text kernel it will be loaded into the
///           kernel pool. If FILE is a SPICE meta-kernel containing
///           initialization instructions (through use of the
///           correct kernel pool variables), the files specified in
///           those variables will be loaded into the appropriate
///           SPICE subsystem.
///
///           The SPICE text kernel format supports association of
///           names and data values using a "keyword = value"
///           format. The keyword-value pairs thus defined are
///           called "kernel variables."
///
///           While any information can be placed in a text kernel
///           file, the following string valued kernel variables are
///           recognized by SPICE as meta-kernel keywords:
///
///                KERNELS_TO_LOAD
///                PATH_SYMBOLS
///                PATH_VALUES
///
///           Each kernel variable is discussed below.
///
///           KERNELS_TO_LOAD   is a list of SPICE kernels to be
///                             loaded into a program. If file
///                             names do not fit within the kernel
///                             pool 80 character limit, they may be
///                             continued to subsequent array
///                             elements by placing the continuation
///                             character ('+') at the end of an
///                             element and then placing the
///                             remainder of the file name in the
///                             next array element. (See the
///                             examples below for an illustration
///                             of this technique or consult the
///                             routine STPOOL for further details.)
///
///                             You may use one or more PATH_SYMBOL
///                             assignments (see below) to specify
///                             strings to be substituted for some
///                             part of a file name.
///
///           PATH_SYMBOLS      is a list of strings (without
///                             embedded blanks) which if
///                             encountered following the '$'
///                             character will be replaced with the
///                             corresponding PATH_VALUES string.
///                             Note that PATH_SYMBOLS are
///                             interpreted only in values
///                             associated with the KERNELS_TO_LOAD
///                             variable. There must be a one-to-one
///                             correspondence between the values
///                             supplied for PATH_SYMBOLS and
///                             PATH_VALUES. For the purpose of
///                             determining this correspondence, any
///                             path value that is continued over
///                             multiple array elements counts as a
///                             single value.
///
///           PATH_VALUES       is a list of expansions to use when
///                             PATH_SYMBOLS are encountered. If
///                             path values do not fit within the
///                             kernel pool 80 character limit, they
///                             may be continued in the same way as
///                             file names (see the KERNELS_TO_LOAD
///                             description above).
///
///            These kernel pool variables persist within the kernel
///            pool only until all kernels associated with the
///            variable KERNELS_TO_LOAD have been loaded. Once all
///            specified kernels have been loaded, the variables
///            KERNELS_TO_LOAD, PATH_SYMBOLS and PATH_VALUES are
///            removed from the kernel pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. The routine loads various SPICE kernels for use by your
///  application.
/// ```
///
/// # Parameters
///
/// ```text
///  FILSIZ   is the maximum file name length that can be accommodated
///           by the kernel pool.
///
///  MAXFIL   is the number of entries that can be stored in KEEPER's
///           kernel database. In this version of the toolkit MAXFIL
///           is set to 5300. Each time a kernel is loaded via
///           FURNSH, a database entry is created for that kernel.
///           If a meta-kernel is loaded, a database entry is created
///           for the meta-kernel itself and for all files referenced
///           in the meta-kernel's KERNELS_TO_LOAD specification.
///           Unloading a kernel or meta-kernel deletes database
///           entries created when the file was loaded.
///
///           The parameter MAXFIL is an upper bound on number of
///           SPICE kernels that can be loaded at any time via the
///           KEEPER interface, but the number of kernels that can be
///           loaded may be smaller, since re-loading a loaded kernel
///           or meta-kernel results in creation of additional
///           database entries.
///
///           Kernels loaded into the KEEPER system are subject to
///           constraints imposed by lower-level subsystems. The
///           binary kernel systems (SPK, CK, binary PCK, and EK)
///           have their own limits on the maximum number of kernels
///           that may be loaded.
///
///           The total number of DAF-based files (this set includes
///           SPKs, CKs, and binary PCKs) and DAS-based files (this
///           set includes EKs and DSKs) that may be loaded at any
///           time may not exceed 5000. This limit applies whether
///           the files are loaded via FURNSH or lower-level loaders
///           such as SPKLEF or DAFOPR. File access performance
///           normally will degrade as the number of loaded kernels
///           increases.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a problem is encountered while trying to load FILE, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If the input FILE is a meta-kernel and some file in the
///      KERNELS_TO_LOAD assignment cannot be found, or if an error
///      occurs while trying to load a file specified by this
///      assignment, the error is signaled by a routine in the call
///      tree of this routine, and this routine will return. Any files
///      loaded prior to encountering the failure, including those
///      referenced by the KERNELS_TO_LOAD assignment, will remain
///      loaded.
///
///  3)  If an attempt to load a text kernel fails while the kernel is
///      being parsed, any kernel variable assignments made before
///      the failure occurred will be retained in the kernel pool.
///
///  4)  If a PATH_SYMBOLS assignment is specified without a
///      corresponding PATH_VALUES assignment, the error
///      SPICE(NOPATHVALUE) is signaled.
///
///  5)  If a meta-text kernel is supplied to FURNSH that contains
///      instructions specifying that another meta-text kernel be
///      loaded, the error SPICE(RECURSIVELOADING) is signaled.
///
///  6)  If the input file name has non-blank length exceeding FILSIZ
///      characters, the error SPICE(FILENAMETOOLONG) is signaled.
///
///  7)  If the input file is a meta-kernel and some file in the
///      KERNELS_TO_LOAD assignment has name length exceeding FILSIZ
///      characters, the error SPICE(FILENAMETOOLONG) is signaled.
///
///  8)  If the input file is a meta-kernel and some value in the
///      PATH_VALUES assignment has length exceeding FILSIZ
///      characters, the error SPICE(PATHTOOLONG) is signaled.
///
///  9)  If the input file is a meta-kernel and some file in the
///      KERNELS_TO_LOAD assignment has, after symbol substitution,
///      combined name and path length exceeding FILSIZ characters,
///      the error SPICE(FILENAMETOOLONG) is signaled.
///
///  10) If a kernel pool variable name length exceeds its maximum
///      allowed length (see Kernel Required Reading, kernel.req), an
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  The input FILE is examined and loaded into the appropriate SPICE
///  subsystem. If the file is a meta-kernel, any kernels specified
///  by the KERNELS_TO_LOAD keyword (and if present, the PATH_SYMBOLS
///  and PATH_VALUES keywords) are loaded as well.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a uniform interface to the SPICE kernel
///  loading systems. It allows you to easily assemble a list of
///  SPICE kernels required by your application and to modify that set
///  without modifying the source code of programs that make use of
///  these kernels.
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
///  1) Load the leapseconds kernel naif0007.tls and the planetary
///     ephemeris SPK file de405s.bsp.
///
///        CALL FURNSH ( 'naif0007.tls' )
///        CALL FURNSH ( 'de405s.bsp'   )
///
///
///  2) This example illustrates how you could create a meta-kernel
///     file for a program that requires several text and binary
///     kernels.
///
///     First create a list of the kernels you need in a text file as
///     shown below.
///
///
///        KPL/MK
///
///        File name: furnsh_ex2.tm
///
///        Here are the SPICE kernels required for my application
///        program.
///
///        Note that kernels are loaded in the order listed. Thus
///        we need to list the highest priority kernel last.
///
///
///        \begindata
///
///        KERNELS_TO_LOAD = (
///
///           '/home/mydir/kernels/spk/lowest_priority.bsp',
///           '/home/mydir/kernels/spk/next_priority.bsp',
///           '/home/mydir/kernels/spk/highest_priority.bsp',
///           '/home/mydir/kernels/text/leapsecond.ker',
///           '/home/mydir/kernels+',
///           '/custom+',
///           '/kernel_data/constants.ker',
///           '/home/mydir/kernels/text/sclk.tsc',
///           '/home/mydir/kernels/ck/c-kernel.bc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Note that the file name
///
///        /home/mydir/kernels/custom/kernel_data/constants.ker
///
///     is continued across several lines in the right hand side of
///     the assignment of the kernel variable KERNELS_TO_LOAD.
///
///     Once you've created your list of kernels, call FURNSH near the
///     beginning of your application program to load the meta-kernel
///     automatically at program start up.
///
///        CALL FURNSH ( 'furnsh_ex2.tm' )
///
///     This will cause each of the kernels listed in your meta-kernel
///     to be loaded.
///
///
///  3) This example illustrates how you can simplify the previous
///     kernel list by using PATH_SYMBOLS.
///
///
///        KPL/MK
///
///        File name: furnsh_ex3.tm
///
///        Here are the SPICE kernels required for my application
///        program.
///
///
///        We are going to let A substitute for the directory that
///        contains SPK files; B substitute for the directory that
///        contains C-kernels; and C substitute for the directory that
///        contains text kernels. And we'll let D substitute for
///        a "custom" directory that contains a special planetary
///        constants kernel made just for our mission.
///
///        Note that our PATH_VALUES and the corresponding
///        PATH_SYMBOLS must be listed in the same order.
///
///
///        \begindata
///
///        PATH_VALUES  = ( '/home/mydir/kernels/spk',
///                         '/home/mydir/kernels/ck',
///                         '/home/mydir/kernels/text',
///                         '/home/mydir/kernels/custom/kernel_data' )
///
///        PATH_SYMBOLS = ( 'A',
///                         'B',
///                         'C',
///                         'D'  )
///
///        KERNELS_TO_LOAD = (  '$A/lowest_priority.bsp',
///                             '$A/next_priority.bsp',
///                             '$A/highest_priority.bsp',
///                             '$C/leapsecond.ker',
///                             '$D/constants.ker',
///                             '$C/sclk.tsc',
///                             '$B/c-kernel.bc'         )
///
///        \begintext
///
///        End of meta-kernel
///
///
///  4) This example illustrates continuation of path values. The
///     meta-kernel shown here is a modified version of that from
///     example 3.
///
///
///        KPL/MK
///
///        File name: furnsh_ex4.tm
///
///        Here are the SPICE kernels required for my application
///        program.
///
///        We are going to let A substitute for the directory that
///        contains SPK files; B substitute for the directory that
///        contains C-kernels; and C substitute for the directory that
///        contains text kernels. And we'll let D substitute for
///        a "custom" directory that contains a special planetary
///        constants kernel made just for our mission.
///
///        Note that our PATH_VALUES and the corresponding
///        PATH_SYMBOLS must be listed in the same order.
///
///        The values for path symbols A and D are continued over
///        multiple lines.
///
///        \begindata
///
///        PATH_VALUES  = ( '/very_long_top_level_path_name/mydir/+',
///                         'kernels/spk',
///                         '/home/mydir/kernels/ck',
///                         '/home/mydir/kernels/text',
///                         '/very_long_top_level_path_name+',
///                         '/mydir/kernels/custom+',
///                         '/kernel_data'                )
///
///        PATH_SYMBOLS = ( 'A',
///                         'B',
///                         'C',
///                         'D'  )
///
///        KERNELS_TO_LOAD = (  '$A/lowest_priority.bsp',
///                             '$A/next_priority.bsp',
///                             '$A/highest_priority.bsp',
///                             '$C/leapsecond.ker',
///                             '$D/constants.ker',
///                             '$C/sclk.tsc',
///                             '$B/c-kernel.bc'         )
///
///        \begintext
///
///        End of meta-kernel
///
///
///  5) Load a meta-kernel containing three kernels, and separately,
///     a text kernel and a binary PCK. Count the number of loaded
///     files before and after calling KCLEAR.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: furnsh_ex5.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls',
///                               'pck00009.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Use the PCK kernel below as the binary PCK required for the
///     example.
///
///        earth_latest_high_prec.bpc
///
///
///     Use the FK kernel below as the text kernel required for the
///     example.
///
///        RSSD0002.TF
///
///
///     Example code begins here.
///
///
///           PROGRAM FURNSH_EX5
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           INTEGER               COUNT
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'furnsh_ex5.tm'              )
///           CALL FURNSH ( 'RSSD0002.TF'                )
///           CALL FURNSH ( 'earth_latest_high_prec.bpc' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'final FURNSH: ', COUNT
///
///     C
///     C     Clear the KEEPER system, retrieve the number of loaded
///     C     after the clear.
///     C
///           CALL KCLEAR()
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'KCLEAR      : ', COUNT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The total number of kernels after final FURNSH:  6
///     The total number of kernels after KCLEAR      :  0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A meta-kernel cannot reference another meta-kernel.
///
///  2)  Failure during an attempt to load a text kernel or a
///      meta-kernel can result in a subset of the intended kernel
///      variables being set or a subset of the intended files
///      being loaded. FURNSH does not "clean up" so as to undo the
///      effects of a failed load operation.
///
///  3)  When a kernel is specified with a relative path, this path
///      should be valid at the time when FURNSH is called and stay
///      valid for the rest of the application run. This is required
///      because SPICE stores kernel names as provided by the caller
///      and uses them to open and close binary kernels as needed
///      by the DAF/DAS handle manager subsystem (behind the scenes,
///      to allow reading many more binary kernels than available
///      logical units), and to automatically reload into the POOL
///      the rest of text kernels that should stay loaded when a
///      particular text kernel is unloaded.
///
///      Changing the working directory from within an application
///      during an application run after calling FURNSH to load
///      kernels specified using relative paths is likely to
///      invalidate stored paths and prevent open/close and unload
///      operations mentioned above. A simple workaround when this
///      is needed is to specify kernels using absolute paths.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.1, 08-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Created complete code example from existing code fragments.
///
///         Added KERNEL to $Required_Reading section.
///
///         Added FILSIZ to the $Declarations section.
///
///         Added a restriction about specifying kernels using relative
///         paths to the $Restrictions section.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Updated description of MAXFIL in the header.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 4.1.0, 01-JUL-2014 (NJB) (BVS)
///
///         Updated discussion of partially completed kernel loading.
///
///      Last update was 12-APR-2012 (BVS)
///
///         Changed to use SEPOOL instead of STPOOL to reduce loading time
///         for large meta-kernels due to n^2 delay in STPOOL.
///
/// -    SPICELIB Version 4.0.1, 10-FEB-2010 (EDW)
///
///         Added mention of the restriction on kernel pool variable
///         names to MAXLEN (defined in pool.f) characters or less.
///
/// -    SPICELIB Version 4.0.0, 02-APR-2009 (NJB)
///
///         Continued path values are now supported. FURNSH now rejects
///         file names longer than FILSIZ characters.
///
/// -    SPICELIB Version 2.0.3, 27-APR-2007 (NJB)
///
///         Fixed header typo: added quotes to literal string
///         input arguments in example FURNSH calls.
///
/// -    SPICELIB Version 2.0.2, 15-NOV-2006 (NJB)
///
///         Added description of parameter MAXFIL to header.
///
/// -    SPICELIB Version 2.0.1, 29-JUL-2003 (NJB) (CHA)
///
///         Numerous updates to improve clarity. Some corrections were
///         made.
///
/// -    SPICELIB Version 2.0.0, 23-AUG-2001 (WLT)
///
///         Added a call to CVPOOL in FURNSH so that watches that are
///         triggered are triggered by loading Meta-kernels and not by
///         some external interaction with the kernel pool.
///
/// -    SPICELIB Version 1.1.0, 19-SEP-2000 (WLT)
///
///         Corrected the error message template used
///         by ZZLDKER
///
/// -    SPICELIB Version 1.0.1, 16-DEC-1999 (NJB)
///
///         Documentation fix: corrected second code example in the
///         header of this entry point. The example previously used the
///         kernel variable PATH_NAMES; that name has been replaced with
///         the correct name PATH_VALUES.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn furnsh(ctx: &mut SpiceContext, file: &str) -> crate::Result<()> {
    FURNSH(file.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FURNSH ( Furnish a program with SPICE kernels )
pub fn FURNSH(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NORC = [b' '; 1 as usize];
    let mut FIL2LD = [b' '; FILSIZ as usize];
    let mut PVALUE = [b' '; FILSIZ as usize];
    let mut SYMBOL = [b' '; LNSIZE as usize];
    let mut NOFILE = [b' '; MSGSIZ as usize];
    let mut THSTYP = [b' '; TYPLEN as usize];
    let mut CURSRC: i32 = 0;
    let mut D: i32 = 0;
    let mut DOLLAR: i32 = 0;
    let mut FIDX: i32 = 0;
    let mut FILNUM: i32 = 0;
    let mut FNMLEN: i32 = 0;
    let mut I: i32 = 0;
    let mut LIDX: i32 = 0;
    let mut MYHAND: i32 = 0;
    let mut N: i32 = 0;
    let mut NPATHS: i32 = 0;
    let mut NPVALS: i32 = 0;
    let mut R: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut START: i32 = 0;
    let mut USE: i32 = 0;
    let mut FND: bool = false;
    let mut OK: bool = false;
    let mut PATHS: bool = false;
    let mut UPDATE: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"FURNSH", ctx)?;

    if save.FIRST {
        save.FIRST = false;
        fstr::assign(save.KNOWN.get_mut(1), b"KERNELS_TO_LOAD");
        fstr::assign(save.KNOWN.get_mut(2), b"PATH_SYMBOLS");
        fstr::assign(save.KNOWN.get_mut(3), b"PATH_VALUES");
        save.LOADED = 0;

        SWPOOL(b"FURNSH", NKNOWN, save.KNOWN.as_arg(), ctx)?;
        CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;
    }

    //
    // Reject excessively long file names.
    //
    if (RTRIM(FILE) > FILSIZ) {
        SETMSG(b"Input file name <#> has length @ characters. The limit on the length of file names stored by FURNSH is @ characters.", ctx);
        ERRCH(b"#", FILE, ctx);
        ERRINT(b"@", RTRIM(FILE), ctx);
        ERRINT(b"@", FILSIZ, ctx);
        SIGERR(b"SPICE(FILENAMETOOLONG)", ctx)?;
        CHKOUT(b"FURNSH", ctx)?;
        return Ok(());
    }

    //
    // Make sure we have room to load at least one more file.
    //
    if (save.LOADED == MAXFIL) {
        SETMSG(b"There is no room left in KEEPER to load another SPICE kernel.  The current limit on the number of files that can be loaded is #.  If you really need more than this many files, you should increase the parameter MAXFIL in the subroutine KEEPER. ", ctx);

        ERRINT(b"#", MAXFIL, ctx);
        SIGERR(b"SPICE(NOMOREROOM)", ctx)?;
        CHKOUT(b"FURNSH", ctx)?;
        return Ok(());
    }
    //
    // We don't want external interactions with the kernel pool to
    // have any affect on FURNSH's watch so we check the watcher
    // here prior to the call to ZZLDKER.
    //
    CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

    //
    // Set a preliminary value for the error message in case the
    // call to ZZLDKER doesn't succeed.
    //
    fstr::assign(
        &mut NOFILE,
        b"The attempt to load \"#\" by the routine FURNSH failed. It #",
    );

    ZZLDKER(FILE, &NOFILE, &mut THSTYP, &mut MYHAND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"FURNSH", ctx)?;
        return Ok(());
    }

    save.LOADED = (save.LOADED + 1);
    CURSRC = save.LOADED;

    fstr::assign(save.FILES.get_mut(save.LOADED), FILE);
    fstr::assign(save.TYPES.get_mut(save.LOADED), &THSTYP);
    save.HANDLS[save.LOADED] = MYHAND;
    save.SRCES[save.LOADED] = 0;

    CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

    if !UPDATE {
        //
        // Nothing to do.  None of the control variables
        // were set in FILE.
        //
        CHKOUT(b"FURNSH", ctx)?;
        return Ok(());
    }

    //
    // See what is present in the kernel pool: Are any path symbols
    // defined?
    //
    DTPOOL(b"PATH_SYMBOLS", &mut PATHS, &mut NPATHS, &mut NORC, ctx)?;

    if (PATHS && fstr::eq(&NORC, b"C")) {
        //
        // Make sure that the values are equal in number. We need to
        // use STPOOL to count the path values, since some of them
        // might span multiple array elements.
        //
        I = 1;
        STPOOL(
            b"PATH_VALUES",
            I,
            b"+",
            &mut PVALUE,
            &mut SIZE,
            &mut OK,
            ctx,
        )?;

        while (OK && !FAILED(ctx)) {
            //
            // Reject excessively long path names.
            //
            if (SIZE > FILSIZ) {
                SETMSG(b"In meta-kernel <#>, the path at index # in the PATH_VALUES list has length # characters; the limit is # characters.", ctx);
                ERRCH(b"#", FILE, ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", SIZE, ctx);
                ERRINT(b"#", FILSIZ, ctx);
                SIGERR(b"SPICE(PATHTOOLONG)", ctx)?;
                CHKOUT(b"FURNSH", ctx)?;
                return Ok(());
            }

            I = (I + 1);
            STPOOL(
                b"PATH_VALUES",
                I,
                b"+",
                &mut PVALUE,
                &mut SIZE,
                &mut OK,
                ctx,
            )?;
        }

        if FAILED(ctx) {
            CHKOUT(b"FURNSH", ctx)?;
            return Ok(());
        }

        NPVALS = (I - 1);

        if (NPVALS != NPATHS) {
            SETMSG(
                b"Number of path symbols is #; number of path values is #; counts must match.",
                ctx,
            );
            ERRINT(b"#", NPATHS, ctx);
            ERRINT(b"#", NPVALS, ctx);
            SIGERR(b"SPICE(PATHMISMATCH)", ctx)?;
            CHKOUT(b"FURNSH", ctx)?;
            return Ok(());
        }
    } else {
        PATHS = false;
    }

    //
    // This kernel appears to be a legitimate meta-text kernel. Mark
    // it as such and then process its contents.
    //
    fstr::assign(save.TYPES.get_mut(save.LOADED), b"META");

    //
    // Now load all kernels specified in the KERNELS_TO_LOAD variable.
    //
    FILNUM = 1;
    FIDX = 1;

    SEPOOL(
        b"KERNELS_TO_LOAD",
        FIDX,
        b"+",
        &mut FIL2LD,
        &mut FNMLEN,
        &mut LIDX,
        &mut OK,
        ctx,
    )?;

    while (OK && !FAILED(ctx)) {
        //
        // Reject excessively long file names.
        //
        if (FNMLEN > FILSIZ) {
            SETMSG(b"In meta-kernel <#>, the file name at index # in the KERNELS_TO_LOAD list has length # characters; the limit is # characters.", ctx);
            ERRCH(b"#", FILE, ctx);
            ERRINT(b"#", FILNUM, ctx);
            ERRINT(b"#", FNMLEN, ctx);
            ERRINT(b"#", FILSIZ, ctx);
            SIGERR(b"SPICE(FILENAMETOOLONG)", ctx)?;
            CHKOUT(b"FURNSH", ctx)?;
            return Ok(());
        }

        //
        // Make sure we have room to load at least one more file.
        //
        if (save.LOADED == MAXFIL) {
            SETMSG(b"There is no room left in KEEPER to load another SPICE kernel. The current limit on the number of files that can be loaded is #.", ctx);

            ERRINT(b"#", MAXFIL, ctx);
            SIGERR(b"SPICE(NOMOREROOM)", ctx)?;
            CHKOUT(b"FURNSH", ctx)?;
            return Ok(());
        }

        //
        // Resolve any path symbols that may be present.
        // Make sure we have room to load at least one more file.
        //
        if PATHS {
            START = 1;
            DOLLAR = POS(&FIL2LD, b"$", START);

            while (DOLLAR > 0) {
                //
                // Determine the longest path symbol that fits into the
                // current file name.  We fetch path symbols one at a
                // time and see if they match the portion of the
                // string that follows the '$'.  The longest match
                // is the one we use as a symbol.
                //
                SIZE = 0;
                USE = 0;
                D = DOLLAR;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = NPATHS;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        GCPOOL(
                            b"PATH_SYMBOLS",
                            I,
                            1,
                            &mut N,
                            CharArrayMut::from_mut(&mut SYMBOL),
                            &mut FND,
                            ctx,
                        )?;

                        R = RTRIM(&SYMBOL);

                        if ((R > SIZE) && SAMSUB(&SYMBOL, 1, R, &FIL2LD, (D + 1), (D + R))) {
                            USE = I;
                            SIZE = R;
                        }

                        I += m3__;
                    }
                }
                //
                // If we found a matching path symbol, get the corresponding
                // value and put it into the file name.
                //
                if (USE > 0) {
                    //
                    // Get the path value having index USE in the set of
                    // path values. Note that we've already checked that
                    // the path value will fit in PVALUE.
                    //
                    STPOOL(
                        b"PATH_VALUES",
                        USE,
                        b"+",
                        &mut PVALUE,
                        &mut N,
                        &mut FND,
                        ctx,
                    )?;

                    //
                    // When the path is substituted for the symbol, the
                    // total length of the path and file name must fit in
                    // the name buffer.
                    //
                    if ((((FNMLEN + N) - SIZE) - 1) > FILSIZ) {
                        SETMSG(b"In meta-kernel <#>, the path at index # in the PATH_SYMBOLS list has # characters and the file name at index # has # characters. The combined path and file name has # characters; the limit is # characters.", ctx);
                        ERRCH(b"#", FILE, ctx);
                        ERRINT(b"#", USE, ctx);
                        ERRINT(b"#", N, ctx);
                        ERRINT(b"#", FILNUM, ctx);
                        ERRINT(b"#", FNMLEN, ctx);
                        ERRINT(b"#", (FNMLEN + N), ctx);
                        ERRINT(b"#", FILSIZ, ctx);
                        SIGERR(b"SPICE(FILENAMETOOLONG)", ctx)?;
                        CHKOUT(b"FURNSH", ctx)?;
                        return Ok(());
                    }

                    REPSUB(
                        &FIL2LD.clone(),
                        D,
                        (D + SIZE),
                        fstr::substr(&PVALUE, 1..=N),
                        &mut FIL2LD,
                        ctx,
                    )?;
                }
                //
                // Look for the next occurrence of a '$' after the last
                // place we found one.
                //
                START = (DOLLAR + 1);
                DOLLAR = POS(&FIL2LD, b"$", START);
            }
        }
        //
        // If any path symbols were present, they have now been
        // resolved.  Let ZZLDKER handle the task of loading this
        // kernel.  Make up a message template for use if ZZLDKER
        // runs into a problem.
        //
        fstr::assign(
            &mut NOFILE,
            b"The @ file \'#\' specified by KERNELS_TO_LOAD in the file @ #",
        );

        REPMOT(&NOFILE.clone(), b"@", FILNUM, b"L", &mut NOFILE, ctx)?;
        REPMC(&NOFILE.clone(), b"@", FILE, &mut NOFILE);

        ZZLDKER(&FIL2LD, &NOFILE, &mut THSTYP, &mut MYHAND, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"FURNSH", ctx)?;
            return Ok(());
        }

        if fstr::eq(&THSTYP, b"TEXT") {
            //
            // See if we stepped on any of the recognized variables.  If
            // we did, there's no point in trying to continue.
            //
            CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

            if UPDATE {
                //
                // First clean up the debris created by this attempt
                // at recursion.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = NKNOWN;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        DVPOOL(&save.KNOWN[I], ctx)?;
                        I += m3__;
                    }
                }
                //
                // Take care of any watcher activation caused by the
                // mop-up of the preceding loop.
                //
                CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

                SETMSG(b"Hmmm.  This is interesting. In the meta-text kernel \'#\' you\'ve requested that the text kernel \'#\' be loaded. This second file is also a \"meta-text\" kernel and specifies new kernel loading instructions. Although you receive high marks for creativity, this path is fraught with peril and can not be supported by FURNSH. ", ctx);

                ERRCH(b"#", FILE, ctx);
                ERRCH(b"#", &FIL2LD, ctx);
                SIGERR(b"SPICE(RECURSIVELOADING)", ctx)?;
                CHKOUT(b"FURNSH", ctx)?;
                return Ok(());
            }
        }
        //
        // Add the latest file loaded to our database of loaded
        // files.
        //
        save.LOADED = (save.LOADED + 1);
        fstr::assign(save.FILES.get_mut(save.LOADED), &FIL2LD);
        fstr::assign(save.TYPES.get_mut(save.LOADED), &THSTYP);
        save.HANDLS[save.LOADED] = MYHAND;
        save.SRCES[save.LOADED] = CURSRC;

        //
        // Get the name of the next file to load.
        //
        FILNUM = (FILNUM + 1);
        FIDX = (LIDX + 1);

        SEPOOL(
            b"KERNELS_TO_LOAD",
            FIDX,
            b"+",
            &mut FIL2LD,
            &mut FNMLEN,
            &mut LIDX,
            &mut OK,
            ctx,
        )?;
    }

    //
    // Last Step.  Remove the special variables from the kernel pool.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NKNOWN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DVPOOL(&save.KNOWN[I], ctx)?;
            I += m3__;
        }
    }

    CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

    CHKOUT(b"FURNSH", ctx)?;
    Ok(())
}

/// Kernel Totals
///
/// Return the number of kernels of a specified type that are
/// currently loaded via the FURNSH interface.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  KIND       I   A list of kinds of kernels to count.
///  COUNT      O   The number of kernels of type KIND.
/// ```
///
/// # Detailed Input
///
/// ```text
///  KIND     is a list of types of kernels to count when computing
///           loaded kernels. KIND should consist of a list of words of
///           kernels to examine. Recognized types are
///
///              SPK  --- All SPK files are counted in the total.
///              CK   --- All CK files are counted in the total.
///              PCK  --- All binary PCK files are counted in the
///                       total.
///              DSK  --- All DSK files are counted in the total.
///              EK   --- All EK files are counted in the total.
///              TEXT --- All text kernels that are not meta-text
///                       kernels are included in the total.
///              META --- All meta-text kernels are counted in the
///                       total.
///              ALL  --- Every type of kernel is counted in the
///                       total.
///
///           KIND is case insensitive. If a word appears in KIND
///           that is not one of those listed above, it is ignored.
///
///           When KIND consists of multiple words, the words must
///           be separated by blanks. Examples of valid lists are the
///           strings
///
///              'SPK CK TEXT'
///              'SPK CK text'
///              'PCK DSK'
///              'CK'
///              'ALL'
///
///           See the $Examples section for illustrations of the
///           use of KIND.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COUNT    is the number of kernels loaded through FURNSH that
///           belong to the list specified by KIND.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a word on the list specified by KIND is not recognized,
///      it is ignored.
///
///  2)  If KIND is blank, or none of the words in KIND is on the
///      list specified above, COUNT will be returned as zero.
/// ```
///
/// # Particulars
///
/// ```text
///  KTOTAL allows you to easily determine the number of kernels
///  loaded via the interface FURNSH that are of a type of interest.
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
///  1) Load a meta-kernel with a PCK, an LSK and an SPK, and
///     separately, a text kernel and a binary PCK. Show the
///     total number of kernels and meta-kernels loaded. Determine the
///     number of text kernels loaded, and the number of binary
///     kernels.
///
///     Unload all kernels and clear the kernel pool using
///     KCLEAR, and check that no kernels are loaded.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ktotal_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls',
///                               'pck00009.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Use the PCK kernel below as the binary PCK required for the
///     example.
///
///        earth_latest_high_prec.bpc
///
///
///     Use the FK kernel below as the text kernel required for the
///     example.
///
///        RSSD0002.TF
///
///
///     Example code begins here.
///
///
///           PROGRAM KTOTAL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           INTEGER               COUNT
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'ktotal_ex1.tm'              )
///           CALL FURNSH ( 'RSSD0002.TF'                )
///           CALL FURNSH ( 'earth_latest_high_prec.bpc' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'final FURNSH: ', COUNT
///
///     C
///     C     Count the number of meta-kernels.
///     C
///           CALL KTOTAL ( 'META', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of meta-kernels  '
///          . //               '            : ', COUNT
///
///     C
///     C     Count the number of text kernels.
///     C
///           CALL KTOTAL ( 'TEXT', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of text kernels  '
///          . //               '            : ', COUNT
///
///     C
///     C     Count the number of binary kernels. These kernels
///     C     are of type CK, DSK, EK, PCK or SPK.
///     C
///           CALL KTOTAL ( 'CK DSK EK PCK SPK', COUNT )
///           WRITE(*,'(A,I2)') 'The total number of binary kernels'
///          . //               '            : ', COUNT
///
///     C
///     C     Clear the KEEPER system, retrieve the number of loaded
///     C     after the clear.
///     C
///           CALL KCLEAR()
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'KCLEAR      : ', COUNT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The total number of kernels after final FURNSH:  6
///     The total number of meta-kernels              :  1
///     The total number of text kernels              :  3
///     The total number of binary kernels            :  2
///     The total number of kernels after KCLEAR      :  0
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.1, 25-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///         Created complete code example from existing code fragments.
///
///         Updated $Detailed_Input description of input argument KIND to
///         illustrate use of multi-word lists. Added KERNEL to the list
///         of required readings.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 1.1.0, 02-APR-2009 (NJB)
///
///         Deleted reference to unneeded variable DOALL.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn ktotal(ctx: &mut SpiceContext, kind: &str, count: &mut i32) -> crate::Result<()> {
    KTOTAL(kind.as_bytes(), count, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure KTOTAL ( Kernel Totals )
pub fn KTOTAL(KIND: &[u8], COUNT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut I: i32 = 0;
    let mut START: i32 = 0;
    let mut ADD: bool = false;
    let mut DOCK: bool = false;
    let mut DODSK: bool = false;
    let mut DOEK: bool = false;
    let mut DOMETA: bool = false;
    let mut DOPCK: bool = false;
    let mut DOSPK: bool = false;
    let mut DOTEXT: bool = false;

    if (save.LOADED == 0) {
        *COUNT = 0;
        return Ok(());
    }

    CHKIN(b"KTOTAL", ctx)?;
    //
    // Parse KIND to see which kernels are of interest.
    //
    DOSPK = false;
    DOCK = false;
    DODSK = false;
    DOTEXT = false;
    DOMETA = false;
    DOEK = false;
    DOPCK = false;
    START = 1;

    FNDNWD(KIND, START, &mut B, &mut E);

    while (B > 0) {
        if EQSTR(fstr::substr(KIND, B..=E), b"ALL") {
            *COUNT = save.LOADED;
            CHKOUT(b"KTOTAL", ctx)?;
            return Ok(());
        } else {
            DOCK = (DOCK || EQSTR(fstr::substr(KIND, B..=E), b"CK"));
            DODSK = (DODSK || EQSTR(fstr::substr(KIND, B..=E), b"DSK"));
            DOEK = (DOEK || EQSTR(fstr::substr(KIND, B..=E), b"EK"));
            DOMETA = (DOMETA || EQSTR(fstr::substr(KIND, B..=E), b"META"));
            DOPCK = (DOPCK || EQSTR(fstr::substr(KIND, B..=E), b"PCK"));
            DOSPK = (DOSPK || EQSTR(fstr::substr(KIND, B..=E), b"SPK"));
            DOTEXT = (DOTEXT || EQSTR(fstr::substr(KIND, B..=E), b"TEXT"));
        }

        START = (E + 1);
        FNDNWD(KIND, START, &mut B, &mut E);
    }

    *COUNT = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.LOADED;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ADD = (((((((fstr::eq(save.TYPES.get(I), b"CK") && DOCK)
                || (fstr::eq(save.TYPES.get(I), b"DSK") && DODSK))
                || (fstr::eq(save.TYPES.get(I), b"EK") && DOEK))
                || (fstr::eq(save.TYPES.get(I), b"META") && DOMETA))
                || (fstr::eq(save.TYPES.get(I), b"PCK") && DOPCK))
                || (fstr::eq(save.TYPES.get(I), b"SPK") && DOSPK))
                || (fstr::eq(save.TYPES.get(I), b"TEXT") && DOTEXT));

            if ADD {
                *COUNT = (*COUNT + 1);
            }

            I += m3__;
        }
    }

    CHKOUT(b"KTOTAL", ctx)?;
    Ok(())
}

/// Kernel Data
///
/// Return data for the nth kernel that is among a list of specified
/// kernel types.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  WHICH      I   Index of kernel to fetch from the list of kernels.
///  KIND       I   The kind of kernel to which fetches are limited.
///  FILE       O   The name of the kernel file.
///  FILTYP     O   The type of the kernel.
///  SRCFIL     O   Name of the source file used to load FILE.
///  HANDLE     O   The handle attached to FILE.
///  FOUND      O   .TRUE. if the specified file could be located.
/// ```
///
/// # Detailed Input
///
/// ```text
///  WHICH    is the number of the kernel to fetch (matching the type
///           specified by KIND) from the list of kernels that have
///           been loaded through the routine FURNSH but that have not
///           been unloaded through the routine UNLOAD.
///
///           The range of WHICH is 1 to COUNT, where COUNT is the
///           number of kernels loaded via FURNSH of type KIND. This
///           count may be obtained by calling KTOTAL. See the
///           $Examples section for an illustrative example.
///
///  KIND     is a list of types of kernels to be considered when
///           fetching kernels from the list of loaded kernels. KIND
///           should consist of words from list of kernel types
///           given below.
///
///              SPK  --- All SPK files are counted in the total.
///              CK   --- All CK files are counted in the total.
///              DSK  --- All DSK files are counted in the total.
///              PCK  --- All binary PCK files are counted in the
///                       total.
///              EK   --- All EK files are counted in the total.
///              TEXT --- All text kernels that are not meta-text
///                       kernels are included in the total.
///              META --- All meta-text kernels are counted in the
///                       total.
///              ALL  --- Every type of kernel is counted in the
///                       total.
///
///           KIND is case insensitive. If a word appears in KIND
///           that is not one of those listed above, it is ignored.
///
///           When KIND consists of multiple words, the words must
///           be separated by blanks. Examples of valid lists are the
///           strings
///
///              'SPK CK TEXT'
///              'SPK CK text'
///              'PCK DSK'
///              'CK'
///              'ALL'
///
///           See the routine KTOTAL for examples of the use of KIND.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FILE     is the name of the file having index WHICH in the
///           sequence of files of type KIND that is currently loaded
///           via FURNSH. FILE will be blank if there is not such
///           kernel loaded.
///
///  FILTYP   is the type of the kernel specified by FILE. FILE
///           will be blank if there is no file matching the
///           specification of WHICH and KIND.
///
///  SRCFIL   is the name of the source file that was used to
///           specify FILE as one to load. If FILE was loaded
///           directly via a call to FURNSH, SRCFIL will be blank.
///           If there is no file matching the specification of
///           WHICH and KIND, SRCFIL will be blank.
///
///  HANDLE   is the handle attached to FILE if it is a binary
///           kernel. If FILE is a text kernel or meta-text kernel
///           HANDLE will be zero. If there is no file matching
///           the specification of WHICH and KIND, HANDLE will be
///           set to zero.
///
///  FOUND    is returned .TRUE. if a FILE matching the specification
///           of WHICH and KIND exists. If there is no such file,
///           FOUND will be set to .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If a file is not loaded matching the specification of WHICH
///      and KIND, FOUND will be .FALSE., FILE, FILTYP, and SRCFIL will
///      be blank and HANDLE will be set to zero.
///
///  2)  If any of FILE, FILTYP or SRCFIL output strings has length
///      too short to contain the corresponding output string, the
///      string is truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to determine which kernels have been
///  loaded via FURNSH and to obtain information sufficient to directly
///  query those files.
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
///  1) Load a meta-kernel with a PCK, an LSK and an SPK and loop over
///     the loaded kernels, outputting file information for each of
///     them.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: kdata_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00009.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM KDATA_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER               FNAMLN
///           PARAMETER           ( FNAMLN = 256 )
///
///           INTEGER               FTYPLN
///           PARAMETER           ( FTYPLN = 33 )
///
///           INTEGER               SRCLEN
///           PARAMETER           ( SRCLEN = 256 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FNAMLN)    FILE
///           CHARACTER*(FTYPLN)    FILTYP
///           CHARACTER*(SRCLEN)    SRCFIL
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               WHICH
///
///           LOGICAL               FOUND
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'kdata_ex1.tm' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           DO WHICH= 1, COUNT + 1
///
///              CALL KDATA ( WHICH, 'ALL',   FILE, FILTYP,
///          .                SRCFIL, HANDLE, FOUND        )
///
///              IF ( FOUND ) THEN
///
///                 WRITE(*,*) 'Index : ', WHICH
///                 WRITE(*,*) 'File  : ', FILE
///                 WRITE(*,*) 'Type  : ', FILTYP
///                 WRITE(*,*) 'Source: ', SRCFIL
///                 WRITE(*,*) 'Handle: ', HANDLE
///                 WRITE(*,*) ' '
///
///              ELSE
///
///                 WRITE(*,*) 'No kernel found with index: ', WHICH
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
///      Index :            1
///      File  : kdata_ex1.tm
///      Type  : META
///      Source:
///      Handle:            0
///
///      Index :            2
///      File  : de421.bsp
///      Type  : SPK
///      Source: kdata_ex1.tm
///      Handle:            1
///
///      Index :            3
///      File  : pck00009.tpc
///      Type  : TEXT
///      Source: kdata_ex1.tm
///      Handle:            0
///
///      Index :            4
///      File  : naif0009.tls
///      Type  : TEXT
///      Source: kdata_ex1.tm
///      Handle:            0
///
///      No kernel found with index:            5
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.0, 08-AUG-2021 (JDR) (NJB)
///
///         Changed argument name SOURCE to SRCFIL for consistency with
///         other routines.
///
///         Edited the header to comply with NAIF standard.
///         Created complete code example from existing code fragments.
///
///         Updated $Detailed_Input description of input arguments KIND, to
///         illustrate use of multi-word lists, and WHICH, to describe its
///         range.
///
///         Added entry #2 to $Exceptions section. Added KERNEL to the list
///         of required readings.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 1.1.0, 02-APR-2009 (NJB)
///
///         Deleted reference to unneeded variable DOALL.
///
/// -    SPICELIB Version 1.0.1, 06-DEC-2002 (NJB)
///
///         Typo in header example was corrected.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn kdata(
    ctx: &mut SpiceContext,
    which: i32,
    kind: &str,
    file: &mut str,
    filtyp: &mut str,
    srcfil: &mut str,
    handle: &mut i32,
    found: &mut bool,
) {
    KDATA(
        which,
        kind.as_bytes(),
        fstr::StrBytes::new(file).as_mut(),
        fstr::StrBytes::new(filtyp).as_mut(),
        fstr::StrBytes::new(srcfil).as_mut(),
        handle,
        found,
        ctx.raw_context(),
    );
}

//$Procedure KDATA ( Kernel Data )
pub fn KDATA(
    WHICH: i32,
    KIND: &[u8],
    FILE: &mut [u8],
    FILTYP: &mut [u8],
    SRCFIL: &mut [u8],
    HANDLE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut HITS: i32 = 0;
    let mut I: i32 = 0;
    let mut START: i32 = 0;
    let mut ADD: bool = false;
    let mut DOCK: bool = false;
    let mut DODSK: bool = false;
    let mut DOEK: bool = false;
    let mut DOMETA: bool = false;
    let mut DOPCK: bool = false;
    let mut DOSPK: bool = false;
    let mut DOTEXT: bool = false;

    fstr::assign(FILE, b" ");
    fstr::assign(FILTYP, b" ");
    fstr::assign(SRCFIL, b" ");
    *HANDLE = 0;
    *FOUND = false;

    if ((WHICH < 1) || (WHICH > save.LOADED)) {
        return;
    }
    //
    // Parse KIND to see which kernels are of interest.
    //
    DOSPK = false;
    DOCK = false;
    DODSK = false;
    DOTEXT = false;
    DOMETA = false;
    DOEK = false;
    DOPCK = false;
    START = 1;
    FNDNWD(KIND, START, &mut B, &mut E);

    while (B > 0) {
        if EQSTR(fstr::substr(KIND, B..=E), b"ALL") {
            //
            // There's no point in going on, we can fill in the output
            // variables right now.
            //
            *FOUND = true;
            fstr::assign(FILE, save.FILES.get(WHICH));
            fstr::assign(FILTYP, save.TYPES.get(WHICH));
            *HANDLE = save.HANDLS[WHICH];

            if (save.SRCES[WHICH] != 0) {
                fstr::assign(SRCFIL, save.FILES.get(save.SRCES[WHICH]));
            }
            return;
        } else {
            DOCK = (DOCK || EQSTR(fstr::substr(KIND, B..=E), b"CK"));
            DODSK = (DODSK || EQSTR(fstr::substr(KIND, B..=E), b"DSK"));
            DOEK = (DOEK || EQSTR(fstr::substr(KIND, B..=E), b"EK"));
            DOMETA = (DOMETA || EQSTR(fstr::substr(KIND, B..=E), b"META"));
            DOPCK = (DOPCK || EQSTR(fstr::substr(KIND, B..=E), b"PCK"));
            DOSPK = (DOSPK || EQSTR(fstr::substr(KIND, B..=E), b"SPK"));
            DOTEXT = (DOTEXT || EQSTR(fstr::substr(KIND, B..=E), b"TEXT"));
        }

        START = (E + 1);
        FNDNWD(KIND, START, &mut B, &mut E);
    }

    //
    // Examine the loaded kernels one at a time until we match
    // WHICH files of the specified KIND.
    //
    HITS = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.LOADED;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            ADD = (((((((fstr::eq(save.TYPES.get(I), b"CK") && DOCK)
                || (fstr::eq(save.TYPES.get(I), b"DSK") && DODSK))
                || (fstr::eq(save.TYPES.get(I), b"EK") && DOEK))
                || (fstr::eq(save.TYPES.get(I), b"META") && DOMETA))
                || (fstr::eq(save.TYPES.get(I), b"PCK") && DOPCK))
                || (fstr::eq(save.TYPES.get(I), b"SPK") && DOSPK))
                || (fstr::eq(save.TYPES.get(I), b"TEXT") && DOTEXT));

            if ADD {
                HITS = (HITS + 1);
                //
                // If we've reached the specified number, fill in the
                // requested information and return.
                //
                if (HITS == WHICH) {
                    *FOUND = true;
                    fstr::assign(FILE, save.FILES.get(I));
                    fstr::assign(FILTYP, save.TYPES.get(I));
                    *HANDLE = save.HANDLS[I];

                    if (save.SRCES[I] != 0) {
                        fstr::assign(SRCFIL, save.FILES.get(save.SRCES[I]));
                    }
                    return;
                }
            }

            I += m3__;
        }
    }
}

/// Kernel Information
///
/// Return information about a specific kernel
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   Name of a kernel to fetch information for
///  FILTYP     O   The type of the kernel
///  SRCFIL     O   Name of the source file used to load FILE.
///  HANDLE     O   The handle attached to FILE.
///  FOUND      O   .TRUE. if the specified file could be located.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of a kernel file for which KEEPER
///           information is desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FILTYP   is the type of the kernel specified by FILE.  FILE
///           will be blank if FILE is not on the list of loaded
///           kernels.
///
///  SRCFIL   is the name of the source file that was used to
///           specify FILE as one to load. If FILE was loaded
///           directly via a call to FURNSH, SRCFIL will be blank.
///           If FILE is not on the list of loaded kernels, SRCFIL
///           will be blank
///
///  HANDLE   is the handle attached to FILE if it is a binary
///           kernel. If FILE is a text kernel or meta-text kernel
///           HANDLE will be zero. If FILE is not on the list of
///           loaded kernels, HANDLE will be set to zero.
///
///  FOUND    is returned .TRUE. if FILE is on the KEEPER list of
///           loaded kernels. If there is no such file, FOUND will
///           be set to .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the specified file is not on the list of files that are
///      currently loaded via the interface FURNSH, FOUND will be
///      .FALSE., HANDLE will be set to zero and FILTYP and SRCFIL will
///      be set to blanks.
///
///  2)  If any of  FILTYP or SRCFIL output strings has length too
///      short to contain the corresponding output string, the string
///      is truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point allows you to request information directly
///  for a specific SPICE kernel.
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
///  1) Suppose you wish to determine the types of kernels loaded
///     by a given meta-kernel. The following code example shows
///     how you might use this routine to do this.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: kinfo_ex1.tm
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
///           pck00008.tpc                  Planet orientation and
///                                         radii
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'pck00008.tpc',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM KINFO_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           LOGICAL               EQSTR
///
///     C
///     C     Local constants.
///     C
///           INTEGER               FILLEN
///           PARAMETER           ( FILLEN = 32 )
///
///           INTEGER               TYPLEN
///           PARAMETER           ( TYPLEN = 33 )
///
///           INTEGER               SRCLEN
///           PARAMETER           ( SRCLEN = 256 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FILLEN)    FILE
///           CHARACTER*(TYPLEN)    FILTYP
///           CHARACTER*(SRCLEN)    SRCFIL
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               WHICH
///
///           LOGICAL               FOUND
///
///     C
///     C     Load the meta-kernel.
///     C
///           CALL FURNSH ( 'kinfo_ex1.tm' )
///
///     C
///     C     Find out the total number of kernels in the kernel pool.
///     C
///           CALL KTOTAL ( 'all', COUNT )
///
///           IF ( COUNT .EQ. 0 ) THEN
///
///              WRITE(*,*) 'No files loaded at this time.'
///
///           ELSE
///
///              WRITE(*,*) 'The loaded files files are: '
///              WRITE(*,*) ' '
///
///           END IF
///
///     C
///     C     Find the file name, type and source for each of the
///     C     kernels in the kernel pool and print its type.
///     C
///           DO WHICH= 1, COUNT
///
///              CALL KDATA ( WHICH,  'all',  FILE, FILTYP,
///          .                SRCFIL, HANDLE, FOUND        )
///
///              CALL KINFO ( FILE, FILTYP, SRCFIL, HANDLE, FOUND )
///
///              IF ( EQSTR( FILTYP, "SPK" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is an SPK file.'
///
///              ELSE IF ( EQSTR( FILTYP, "CK" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is a CK file.'
///
///              ELSE IF ( EQSTR( FILTYP, "PCK" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is a PCK file.'
///
///              ELSE IF ( EQSTR( FILTYP, "DSK" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is a DSK file.'
///
///              ELSE IF ( EQSTR( FILTYP, "EK" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is an EK file.'
///
///              ELSE IF ( EQSTR( FILTYP, "META" ) ) THEN
///
///                 WRITE(*,*) FILE, ' is a meta-kernel file.'
///
///              ELSE
///
///                 WRITE(*,*) FILE, ' is a text kernel.'
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
///      The loaded files files are:
///
///      kinfo_ex1.tm                     is a meta-kernel file.
///      de421.bsp                        is an SPK file.
///      pck00008.tpc                     is a text kernel.
///      naif0009.tls                     is a text kernel.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.0, 08-AUG-2021 (JDR)
///
///         Changed argument name SOURCE to SRCFIL for consistency with
///         other routines.
///
///         Edited the header to comply with NAIF standard.
///         Created complete code example from existing code fragments.
///
///         Added entry #2 to $Exceptions section.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn kinfo(
    ctx: &mut SpiceContext,
    file: &str,
    filtyp: &mut str,
    srcfil: &mut str,
    handle: &mut i32,
    found: &mut bool,
) {
    KINFO(
        file.as_bytes(),
        fstr::StrBytes::new(filtyp).as_mut(),
        fstr::StrBytes::new(srcfil).as_mut(),
        handle,
        found,
        ctx.raw_context(),
    );
}

//$Procedure KINFO ( Kernel Information )
pub fn KINFO(
    FILE: &[u8],
    FILTYP: &mut [u8],
    SRCFIL: &mut [u8],
    HANDLE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;

    fstr::assign(FILTYP, b" ");
    fstr::assign(SRCFIL, b" ");
    *HANDLE = 0;
    *FOUND = false;

    I = ISRCHC(FILE, save.LOADED, save.FILES.as_arg());

    if (I > 0) {
        *FOUND = true;
        fstr::assign(FILTYP, save.TYPES.get(I));
        *HANDLE = save.HANDLS[I];

        if (save.SRCES[I] != 0) {
            fstr::assign(SRCFIL, save.FILES.get(save.SRCES[I]));
        }
    }
}

/// Keeper clear
///
/// Clear the KEEPER subsystem: unload all kernels, clear the kernel
/// pool, and re-initialize the subsystem. Existing watches on kernel
/// variables are retained.
///
/// # Detailed Input
///
/// ```text
///  None. This routine operates by side effects. See $Particulars
///  below.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs when setting a kernel pool watch or
///      checking watched variables, the error is signaled by a routine
///      in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See $Particulars.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point allows you re-initialize the KEEPER system with
///  a single call.
///
///  This routine unloads all kernels from their kernel-type-specific
///  kernel management subsystems (SPKBSR, CKBSR, etc.), clears the
///  kernel pool, clears KEEPER's internal file database, and re-sets
///  the watch status for the kernel variables used to load kernels
///  via meta-kernels. As a side effect of clearing the kernel pool,
///  all watched variables are marked as updated. Note that clearing
///  the kernel pool does not delete watches (aka "watchers"). Watches
///  can be deleted by calling the POOL entry point DWPOOL.
///
///  This capability, though implemented in Fortran, is particularly
///  relevant to SPICE implementations such as Icy, for which the
///  state of the KEEPER system persists after any Icy-based IDL
///  script is run. Successive runs of Icy-based scripts may perform
///  in unexpected ways when scripts access data loaded during runs of
///  previous scripts.
///
///  Cleaning up after such programs using explicit UNLOAD commands is
///  tedious and error-prone. One call to this routine sets the
///  KEEPER system to its initial state, preventing unintentional
///  interaction between scripts via KEEPER's state.
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
///  1) Load a meta-kernel containing three kernels, and count the
///     number of files in the kernel pool before and after calling
///     KCLEAR.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: kclear_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls',
///                               'pck00009.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM KCLEAR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           INTEGER               COUNT
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'kclear_ex1.tm' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'FURNSH: ', COUNT
///
///     C
///     C     Clear the KEEPER system, retrieve the number of loaded
///     C     after the clear.
///     C
///           CALL KCLEAR()
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'KCLEAR: ', COUNT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The total number of kernels after FURNSH:  4
///     The total number of kernels after KCLEAR:  0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Calling this routine will wipe out any kernel pool data
///      inserted via the SPICELIB API routines to put data into the
///      kernel pool (PCPOOL, PDPOOL and PIPOOL).
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.1, 08-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example from existing code fragments.
///
///         Improved $Restrictions section.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 1.0.1, 01-JUL-2014 (NJB) (EDW)
///
///         Updated the discussion of kernel variable watchers.
///
///      Last update was 13-APR-2011 (EDW)
///
///         Trivial edit to $Restrictions, replaced P*POOL with
///         PXPOOL. The "*" character causes the HTML documentation
///         script to create a link for the "POOL" substring.
///
/// -    SPICELIB Version 1.0.0, 15-NOV-2006 (NJB)
/// ```
pub fn kclear(ctx: &mut SpiceContext) -> crate::Result<()> {
    KCLEAR(ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure KCLEAR ( Keeper clear )
pub fn KCLEAR(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut UPDATE: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"KCLEAR", ctx)?;

    //
    // Unloading all kernels is actually much less work than
    // unloading just a few of them.  We unload all of the
    // binary kernels via the "unload" routines for their
    // respective subsystems, then clear the kernel pool.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.LOADED;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if fstr::eq(save.TYPES.get(I), b"SPK") {
                SPKUEF(save.HANDLS[I], ctx)?;
            } else if fstr::eq(save.TYPES.get(I), b"CK") {
                CKUPF(save.HANDLS[I], ctx)?;
            } else if fstr::eq(save.TYPES.get(I), b"PCK") {
                PCKUOF(save.HANDLS[I], ctx)?;
            } else if fstr::eq(save.TYPES.get(I), b"EK") {
                EKUEF(save.HANDLS[I], ctx)?;
            } else if fstr::eq(save.TYPES.get(I), b"DSK") {
                ZZDSKUSF(save.HANDLS[I], ctx)?;
            }

            I += m3__;
        }
    }

    CLPOOL(ctx)?;

    //
    // Although it's not strictly necessary, we initialize
    // KEEPER's database arrays.  This step may occasionally
    // be helpful for debugging.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.LOADED;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.FILES.get_mut(I), b" ");
            save.HANDLS[I] = 0;
            save.SRCES[I] = 0;
            fstr::assign(save.TYPES.get_mut(I), b" ");

            I += m3__;
        }
    }

    //
    // There's just one counter that indicates the number of
    // database entries:  LOADED.  Set this counter to
    // its initial state.
    //
    save.LOADED = 0;

    //
    // Calling CLPOOL doesn't remove watches, but it does send a message
    // to each agent indicating that its variables have been touched.
    // Clear this indication by calling CVPOOL.  (This is done for
    // safety; the current implementation of FURNSH doesn't require it.)
    //
    CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;

    CHKOUT(b"KCLEAR", ctx)?;
    Ok(())
}

/// Unload a kernel
///
/// Unload a SPICE kernel.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FILE       I   The name of a kernel to unload.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FILE     is the name of a file to unload. This file
///           should be one loaded through the interface FURNSH.
///           If the file is not on the list of loaded kernels
///           no action is taken.
///
///           Note that if FILE is a meta-text kernel, all of
///           the files loaded as a result of loading the meta-text
///           kernel will be unloaded.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the specified kernel is not on the list of loaded kernels
///      no action is taken.
/// ```
///
/// # Particulars
///
/// ```text
///  The call
///
///     CALL UNLOAD ( FILE )
///
///  has the effect of "erasing" the last previous call:
///
///     CALL FURNSH ( FILE )
///
///  This interface allows you to unload binary and text kernels.
///  Moreover, if you used a meta-text kernel to set up your
///  working environment, you can unload all of the kernels loaded
///  through the meta-kernel by unloading the meta-kernel.
///
///  The usual usage of FURNSH is to load each file needed by your
///  program exactly one time. However, it is possible to load a
///  kernel more than one time. (Usually, this is a result of loading
///  meta-kernels without taking the care needed to ensure that the
///  meta-kernels do not specify the same file more than once.) The
///  effect of unloading a kernel that has been loaded more than once
///  is to "undo" the last loading of the kernel. Depending upon the
///  kernel and its relationship to other loaded kernels, this may
///  have no visible effect on the working of your program. To
///  illustrate this behavior suppose that you have a collection of
///  files FILE1, FILE2, FILE3, FILE4, FILE5, FILE6, FILE7, FILE8,
///  META1, META2  where FILE1 ... FILE8 are SPICE kernels and META1
///  and META2 are meta-kernels with the specified kernels to load as
///  shown below.
///
///
///      META1:
///         KERNELS_TO_LOAD = ( FILE2,
///                             FILE3,
///                             FILE4,
///                             FILE5 )
///
///      META2:
///         KERNELS_TO_LOAD = ( FILE2,
///                             FILE3,
///                             FILE7,
///                             FILE8 )
///
///
///   The following sequence of calls
///
///       CALL FURNSH ( FILE1 )
///       CALL FURNSH ( FILE2 )
///       CALL FURNSH ( FILE3 )
///       CALL FURNSH ( META1 )
///       CALL FURNSH ( FILE6 )
///       CALL FURNSH ( META2 )
///
///   has the effect:
///
///       "Load" FILE1
///       "Load" FILE2
///       "Load" FILE3
///       "Load" META1 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META1)
///             "Load" FILE3 (note that it was loaded from META1)
///             "Load" FILE4 (note that it was loaded from META1)
///             "Load" FILE5 (note that it was loaded from META1)
///       "Load" FILE6
///       "Load" META2 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META2)
///             "Load" FILE3 (note that it was loaded from META2) *
///             "Load" FILE7 (note that it was loaded from META2)
///             "Load" FILE8 (note that it was loaded from META2)
///
///   If we  UNLOAD FILE3
///
///      CALL UNLOAD ( FILE3 )
///
///   we locate the last time FILE3 was loaded (* above) and modify the
///   state of loaded kernels so that it looks as if we had made the
///   following sequence of "load" operations.
///
///       "Load" FILE1
///       "Load" FILE2
///       "Load" FILE3
///       "Load" META1 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META1)
///             "Load" FILE3 (note that it was loaded from META1)
///             "Load" FILE4 (note that it was loaded from META1)
///             "Load" FILE5 (note that it was loaded from META1)
///       "Load" FILE6
///       "Load" META2 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META2)
///             "Load" FILE7 (note that it was loaded from META2)
///             "Load" FILE8 (note that it was loaded from META2)
///
///   As you can see, the data from FILE3 is still available to the
///   program. All that may have changed is the usage priority
///   associated with that data.
///
///   If we unload META2 (or META1) we remove all remaining files that
///   are noted as being loaded from META2 (or META1)
///
///       CALL UNLOAD ( META2 )
///
///   produces the following load state for the program:
///
///       "Load" FILE1
///       "Load" FILE2
///       "Load" FILE3
///       "Load" META1 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META1)
///             "Load" FILE3 (note that it was loaded from META1)
///             "Load" FILE4 (note that it was loaded from META1)
///             "Load" FILE5 (note that it was loaded from META1)
///       "Load" FILE6
///
///   If we had unloaded META1 instead, we would have this load state.
///
///       "Load" FILE1
///       "Load" FILE2
///       "Load" FILE3
///       "Load" FILE6
///       "Load" META2 as a text kernel and then...
///             "Load" FILE2 (note that it was loaded from META2)
///             "Load" FILE7 (note that it was loaded from META2)
///             "Load" FILE8 (note that it was loaded from META2)
///
///   So we see that unloading a file does not necessarily make its
///   data unavailable to your program. Unloading modifies the
///   precedence of the files loaded in your program. The data
///   associated with an unloaded file becomes unavailable only when
///   the file has been unloaded as many times as it was loaded.
///
///   When would you encounter such a scenario? The situation of
///   loading a file more than once might appear if you were trying to
///   contrast the results of computations performed with two
///   different meta-kernels. In such a scenario you might load a
///   "baseline" set of kernels early in your program and then load
///   and unload meta-kernels to compare results between the two
///   different sets of data.
///
///  Unloading Text Kernels or Meta-Kernels
///  --------------------------------------
///
///  Part of the action of unloading text (or meta-kernels) is
///  the clearing of the kernel pool and re-loading any kernels that
///  were not in the specified set of kernels to unload. Since
///  loading of text kernels is not a very fast process, unloading
///  text kernels takes considerably longer than unloading binary
///  kernels. Moreover, since the kernel pool is cleared, any kernel
///  pool variables you have set from your program by using one of the
///  interfaces PCPOOL, PDPOOL, PIPOOL, or LMPOOL will be removed from
///  the kernel pool. For this reason, if you plan to use this
///  feature in your program, together with one of the routines
///  specified above, you will need to take special precautions to
///  make sure kernel pool variables required by your program do not
///  inadvertently disappear.
///
///  As a side effect of unloading a text kernel, all watched kernel
///  variables are marked as updated. Note that unloading a text
///  kernel does not delete watchers. Watchers can be deleted by
///  calling the POOL entry point DWPOOL.
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
///  1) Load a meta-kernel with a PCK, an LSK and an SPK, and
///     separately, a text kernel and a binary PCK. Loop over the
///     loaded kernels, outputting file information for each of
///     them.
///
///     Then unload the text kernels, check that they have been
///     unloaded, and finally unload the meta-kernel.
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: unload_ex1.tm
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
///           pck00009.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0012.tls',
///                               'pck00009.tpc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Use the PCK kernel below as the binary PCK required for the
///     example.
///
///        earth_latest_high_prec.bpc
///
///
///     Use the FK kernel below as the text kernel required for the
///     example.
///
///        RSSD0002.TF
///
///
///     Example code begins here.
///
///
///           PROGRAM UNLOAD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER               FNAMLN
///           PARAMETER           ( FNAMLN = 256 )
///
///           INTEGER               FTYPLN
///           PARAMETER           ( FTYPLN = 33 )
///
///           INTEGER               SRCLEN
///           PARAMETER           ( SRCLEN = 256 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FNAMLN)    FILE
///           CHARACTER*(FTYPLN)    FILTYP
///           CHARACTER*(SRCLEN)    SRCFIL
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///
///           LOGICAL               FOUND
///
///     C
///     C     Load several kernel files.
///     C
///           CALL FURNSH ( 'unload_ex1.tm'              )
///           CALL FURNSH ( 'RSSD0002.TF'                )
///           CALL FURNSH ( 'earth_latest_high_prec.bpc' )
///
///     C
///     C     Count the number of loaded kernel files.
///     C
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'final FURNSH: ', COUNT
///           WRITE(*,*) ' '
///
///     C
///     C     Unload the text kernels.
///     C
///           CALL KTOTAL ( 'TEXT', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2,A)') 'Unloading', COUNT,
///          .                    ' text kernels...'
///           WRITE(*,*) ' '
///
///           DO WHILE ( COUNT .NE. 0 )
///
///              CALL KDATA (      1, 'TEXT',  FILE, FILTYP,
///          .                SRCFIL, HANDLE, FOUND        )
///
///     C
///     C        If the kernel is found in the pool, unload it.
///     C
///              IF ( FOUND ) THEN
///
///                 CALL UNLOAD ( FILE )
///
///     C
///     C           Check if the file has been unloaded.
///     C
///                 CALL KINFO ( FILE, FILTYP, SRCFIL, HANDLE, FOUND )
///
///                 IF ( FOUND ) THEN
///
///                    WRITE(*,'(A)') '  Error unloading ' // FILE
///
///                 ELSE
///
///                    WRITE(*,'(A)') '  Success unloading ' // FILE
///
///                 END IF
///
///     C
///     C        Something is not working. Inform NAIF.
///     C
///              ELSE
///
///                 WRITE(*,*) ' ERROR: No kernel found but KTOTAL '
///          .      //         'returns ', COUNT
///
///              END IF
///
///     C
///     C        Check if we have more text kernels to unload from
///     C        the kernel pool. Note that unloading a text kernel
///     C        or meta-kernel implies that the kernel pool is
///     C        cleared, and any kernel(s) that were not to be
///     C        unloaded are re-loaded. Therefore the COUNT value
///     C        changes, and the indexing of the files within the
///     C        kernel pool too.
///     C
///              CALL KTOTAL ( 'TEXT', COUNT )
///
///           END DO
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'UNLOAD calls: ', COUNT
///
///     C
///     C     Unload the meta-kernel and retrieve the number of loaded
///     C     after the clear.
///     C
///           CALL UNLOAD ( 'unload_ex1.tm' )
///
///           CALL KTOTAL ( 'ALL', COUNT )
///
///           WRITE(*,*) ' '
///           WRITE(*,'(A,I2)') 'The total number of kernels after '
///          . //               'final UNLOAD: ', COUNT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The total number of kernels after final FURNSH:  6
///
///
///     Unloading 3 text kernels...
///
///       Success unloading naif0012.tls
///       Success unloading pck00009.tpc
///       Success unloading RSSD0002.TF
///
///     The total number of kernels after UNLOAD calls:  3
///
///     The total number of kernels after final UNLOAD:  1
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See the note regarding the unloading of Text and meta-text
///      Kernels.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.1, 08-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Created complete code example from existing code fragments.
///
/// -    SPICELIB Version 5.0.0, 01-FEB-2017 (NJB) (BVS)
///
///         Updated to support use of DSKs.
///
///         Bug fix: now unloads binary kernels via low-level
///         unload routines only when those kernels have just
///         one entry in the KEEPER database.
///
///         Updated the $Author_and_Institution section.
///
/// -    SPICELIB Version 3.0.1, 01-JUL-2014 (NJB)
///
///         Updated discussion of kernel variable watchers.
///
/// -    SPICELIB Version 3.0.0, 15-NOV-2006 (NJB)
///
///         Bug fix: corrected update of source pointers when a
///         meta-kernel is unloaded. Previously source pointers
///         having higher indices than those of the files referenced
///         by the meta kernel were not adjusted when the database
///         was compressed.
///
/// -    SPICELIB Version 2.0.0, 23-AUG-2001 (WLT)
///
///         Added code to make sure that UNLOAD has the effect of
///         loading all remaining kernels in the order they were first
///         introduced.
///
/// -    SPICELIB Version 1.0.0, 01-JUL-1999 (WLT)
/// ```
pub fn unload(ctx: &mut SpiceContext, file: &str) -> crate::Result<()> {
    UNLOAD(file.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure UNLOAD ( Unload a kernel )
pub fn UNLOAD(FILE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut N1: i32 = 0;
    let mut N2: i32 = 0;
    let mut N3: i32 = 0;
    let mut NMULT: i32 = 0;
    let mut SRC: i32 = 0;
    let mut DIDCK: bool = false;
    let mut DIDDSK: bool = false;
    let mut DIDEK: bool = false;
    let mut DIDPCK: bool = false;
    let mut DIDSPK: bool = false;
    let mut DIDTXT: bool = false;
    let mut GOTIT: bool = false;
    let mut SINGLE: bool = false;
    let mut UPDATE: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"UNLOAD", ctx)?;

    DIDSPK = false;
    DIDPCK = false;
    DIDCK = false;
    DIDEK = false;
    DIDDSK = false;
    DIDTXT = false;

    //
    // First locate the file we need to unload, we search backward
    // through the list of loaded files so that we unload in the right
    // order.
    //
    GOTIT = false;
    I = save.LOADED;

    while (!GOTIT && (I > 0)) {
        if fstr::eq(save.FILES.get(I), FILE) {
            GOTIT = true;
        } else {
            I = (I - 1);
        }
    }

    //
    // If we didn't locate the requested file, there is nothing to do.
    //
    if !GOTIT {
        CHKOUT(b"UNLOAD", ctx)?;
        return Ok(());
    }

    //
    // We need to know what type of file we've got so that we
    // can take the correct "unload" action.
    //
    // If the kernel to be unloaded is binary, found out how
    // many instances of it are present in the database.
    //
    // We take advantage of the fact that all binary kernels
    // use the handle manager subsystem: handles are unique
    // across all file types. We don't need to rely on file
    // names.
    //
    if ((((fstr::eq(save.TYPES.get(I), b"SPK") || fstr::eq(save.TYPES.get(I), b"CK"))
        || fstr::eq(save.TYPES.get(I), b"DSK"))
        || fstr::eq(save.TYPES.get(I), b"EK"))
        || fstr::eq(save.TYPES.get(I), b"PCK"))
    {
        //
        // Count the occurrences of the file in the database.
        // Stop if we reach two occurrences.
        //
        NMULT = 0;
        J = 1;

        while ((J <= save.LOADED) && (NMULT < 2)) {
            if (save.HANDLS[J] == save.HANDLS[I]) {
                //
                // To be safe, make sure we're not looking at
                // a text kernel with a random, matching handle
                // value.
                //
                if (fstr::ne(save.TYPES.get(J), b"TEXT") && fstr::ne(save.TYPES.get(J), b"META")) {
                    NMULT = (NMULT + 1);
                }
            }

            J = (J + 1);
        }

        SINGLE = (NMULT == 1);
    }

    if fstr::eq(save.TYPES.get(I), b"SPK") {
        if SINGLE {
            SPKUEF(save.HANDLS[I], ctx)?;
        }

        DIDSPK = true;
    } else if fstr::eq(save.TYPES.get(I), b"CK") {
        if SINGLE {
            CKUPF(save.HANDLS[I], ctx)?;
        }

        DIDCK = true;
    } else if fstr::eq(save.TYPES.get(I), b"DSK") {
        if SINGLE {
            ZZDSKUSF(save.HANDLS[I], ctx)?;
        }

        DIDDSK = true;
    } else if fstr::eq(save.TYPES.get(I), b"PCK") {
        if SINGLE {
            PCKUOF(save.HANDLS[I], ctx)?;
        }

        DIDPCK = true;
    } else if fstr::eq(save.TYPES.get(I), b"EK") {
        if SINGLE {
            EKUEF(save.HANDLS[I], ctx)?;
        }

        DIDEK = true;
    } else if fstr::eq(save.TYPES.get(I), b"TEXT") {
        CLPOOL(ctx)?;
        DIDTXT = true;
    } else if fstr::eq(save.TYPES.get(I), b"META") {
        //
        // This is a special case, we need to undo the effect of loading
        // the meta-kernel.  This means we need to unload all kernels
        // that were loaded using this meta-kernel.
        //
        DIDTXT = true;
        SRC = I;

        {
            let m1__: i32 = save.LOADED;
            let m2__: i32 = (SRC + 1);
            let m3__: i32 = -1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (save.SRCES[J] == SRC) {
                    //
                    // This file was loaded by the meta-kernel of interest.
                    // We only need to unload the binary kernels as we
                    // will get rid of all text kernels by clearing the
                    // kernel pool.
                    //
                    // See whether the file we're about to process is
                    // binary, and if so, count the number of times
                    // it appears in the database. We have to repeat
                    // this test on each loop pass, since the count
                    // may have changed since the last pass.
                    //
                    if ((((fstr::eq(save.TYPES.get(J), b"SPK")
                        || fstr::eq(save.TYPES.get(J), b"CK"))
                        || fstr::eq(save.TYPES.get(J), b"DSK"))
                        || fstr::eq(save.TYPES.get(J), b"EK"))
                        || fstr::eq(save.TYPES.get(J), b"PCK"))
                    {
                        //
                        // Count the occurrences of the file in the database.
                        // Stop if we reach two occurrences.
                        //
                        NMULT = 0;
                        K = 1;

                        while ((K <= save.LOADED) && (NMULT < 2)) {
                            if (save.HANDLS[K] == save.HANDLS[J]) {
                                //
                                // To be safe, make sure we're not looking at a
                                // text kernel with a random, matching handle
                                // value.
                                //
                                if (fstr::ne(save.TYPES.get(K), b"TEXT")
                                    && fstr::ne(save.TYPES.get(K), b"META"))
                                {
                                    NMULT = (NMULT + 1);
                                }
                            }

                            K = (K + 1);
                        }

                        SINGLE = (NMULT == 1);
                    }

                    if fstr::eq(save.TYPES.get(J), b"SPK") {
                        if SINGLE {
                            SPKUEF(save.HANDLS[J], ctx)?;
                        }

                        DIDSPK = true;
                    } else if fstr::eq(save.TYPES.get(J), b"CK") {
                        if SINGLE {
                            CKUPF(save.HANDLS[J], ctx)?;
                        }

                        DIDCK = true;
                    } else if fstr::eq(save.TYPES.get(J), b"DSK") {
                        if SINGLE {
                            ZZDSKUSF(save.HANDLS[J], ctx)?;
                        }

                        DIDDSK = true;
                    } else if fstr::eq(save.TYPES.get(J), b"PCK") {
                        if SINGLE {
                            PCKUOF(save.HANDLS[J], ctx)?;
                        }

                        DIDPCK = true;
                    } else if fstr::eq(save.TYPES.get(J), b"EK") {
                        if SINGLE {
                            EKUEF(save.HANDLS[J], ctx)?;
                        }

                        DIDEK = true;
                    }

                    N1 = save.LOADED;
                    N2 = save.LOADED;
                    N3 = save.LOADED;
                    REMLAC(1, J, save.FILES.as_arg_mut(), &mut N1, ctx)?;
                    REMLAC(1, J, save.TYPES.as_arg_mut(), &mut N2, ctx)?;
                    REMLAI(1, J, save.SRCES.as_slice_mut(), &mut N3, ctx)?;
                    REMLAI(1, J, save.HANDLS.as_slice_mut(), &mut save.LOADED, ctx)?;

                    //
                    // Each time we delete an item from the database, any
                    // pointer to a location past the deletion point must be
                    // updated to reflect the compression of the database.
                    // Files loaded from meta kernels are always recorded
                    // in the database *after* their sources, so each pointer
                    // value is less than the index at which it occurs.
                    // So, we need examine only those entries from index J
                    // upwards.
                    //
                    {
                        let m1__: i32 = J;
                        let m2__: i32 = save.LOADED;
                        let m3__: i32 = 1;
                        K = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            if (save.SRCES[K] > J) {
                                //
                                // This pointer is affected by the deletion of
                                // the Jth database entry.
                                //
                                save.SRCES[K] = (save.SRCES[K] - 1);
                            }

                            K += m3__;
                        }
                    }
                }

                J += m3__;
            }
        }
        //
        // Now clear the kernel pool.
        //
        CLPOOL(ctx)?;
    }
    //
    // Remove the I'th kernel from our local database.
    //
    N1 = save.LOADED;
    N2 = save.LOADED;
    N3 = save.LOADED;
    REMLAC(1, I, save.FILES.as_arg_mut(), &mut N1, ctx)?;
    REMLAC(1, I, save.TYPES.as_arg_mut(), &mut N2, ctx)?;
    REMLAI(1, I, save.SRCES.as_slice_mut(), &mut N3, ctx)?;
    REMLAI(1, I, save.HANDLS.as_slice_mut(), &mut save.LOADED, ctx)?;

    //
    // Update any source pointers affected by the deletion of the Ith
    // database entry.
    //
    {
        let m1__: i32 = I;
        let m2__: i32 = save.LOADED;
        let m3__: i32 = 1;
        J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.SRCES[J] > I) {
                //
                // This pointer is affected by the deletion of the Ith
                // database entry.
                //
                save.SRCES[J] = (save.SRCES[J] - 1);
            }

            J += m3__;
        }
    }

    //
    // If we unloaded a text kernel, we now need to reload all
    // of the text kernels that were not unloaded.
    //
    if DIDTXT {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (fstr::eq(save.TYPES.get(I), b"TEXT") || fstr::eq(save.TYPES.get(I), b"META")) {
                    LDPOOL(&save.FILES[I], ctx)?;

                    if fstr::eq(save.TYPES.get(I), b"META") {
                        //
                        // Clean up any debris that may have been left lying
                        // around because we reloaded a meta-text kernel.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = NKNOWN;
                            let m3__: i32 = 1;
                            J = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                DVPOOL(&save.KNOWN[J], ctx)?;
                                J += m3__;
                            }
                        }

                        CVPOOL(b"FURNSH", &mut UPDATE, ctx)?;
                    }
                }

                I += m3__;
            }
        }
    }

    //
    // If any SPK files were unloaded, we need to reload everything
    // to establish the right priority sequence for segments.
    //
    if DIDSPK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::eq(save.TYPES.get(I), b"SPK") {
                    SPKLEF(&save.FILES[I], &mut save.HANDLS[I], ctx)?;
                }
                I += m3__;
            }
        }
    }
    //
    // If any CK files were unloaded, we need to reload all of the
    // C-kernels to make sure that we have the correct priorities
    // for the remaining C-kernels.
    //
    if DIDCK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::eq(save.TYPES.get(I), b"CK") {
                    CKLPF(&save.FILES[I], &mut save.HANDLS[I], ctx)?;
                }
                I += m3__;
            }
        }
    }

    //
    // If any DSK files were unloaded, we need to reload the remaining
    // ones to make sure that we have the correct priorities for the
    // remaining DSKs.
    //
    if DIDDSK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::eq(save.TYPES.get(I), b"DSK") {
                    ZZDSKLSF(&save.FILES[I], &mut save.HANDLS[I], ctx)?;
                }
                I += m3__;
            }
        }
    }

    //
    // If any binary PCK files were unloaded, we need to reload any
    // remaining ones to re-establish the correct priorities for
    // kernels.
    //
    if DIDPCK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::eq(save.TYPES.get(I), b"PCK") {
                    PCKLOF(&save.FILES[I], &mut save.HANDLS[I], ctx)?;
                }
                I += m3__;
            }
        }
    }
    //
    // Finally, if any E-kernels were unloaded, we reload the remaining
    // kernels to make sure the state is restored to the correct set
    // of loaded kernels.
    //
    if DIDEK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.LOADED;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::eq(save.TYPES.get(I), b"EK") {
                    EKLEF(&save.FILES[I], &mut save.HANDLS[I], ctx)?;
                }
                I += m3__;
            }
        }
    }

    CHKOUT(b"UNLOAD", ctx)?;
    Ok(())
}
