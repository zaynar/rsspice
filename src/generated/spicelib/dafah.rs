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
const INTEOC: i32 = 4;
const LBCELL: i32 = -5;
const ACCLEN: i32 = 10;
const ARCLEN: i32 = 3;
const CRLEN: i32 = 1000;
const DPRSIZ: i32 = 128;
const FMTLEN: i32 = 8;
const IDLEN: i32 = 8;
const IFNLEN: i32 = 60;
const MAXND: i32 = 124;
const MAXNI: i32 = 250;
const MAXSUM: i32 = 125;
const MAXPC: i32 = 126;
const MINPC: i32 = 32;
const TYPLEN: i32 = 4;

struct SaveVars {
    NFT: i32,
    FTHAN: ActualArray<i32>,
    FTLNK: ActualArray<i32>,
    FTND: ActualArray<i32>,
    FTNI: ActualArray<i32>,
    FINDEX: i32,
    ACC: Vec<u8>,
    CREC: Vec<u8>,
    DAFNAM: Vec<u8>,
    FORMAT: Vec<u8>,
    IDWORD: Vec<u8>,
    IFN: Vec<u8>,
    TTYPE: Vec<u8>,
    DREC: StackArray<f64, 128>,
    BWARD: i32,
    FHLIST: ActualArray<i32>,
    FND: i32,
    FNI: i32,
    FREE: i32,
    FNB: i32,
    FWARD: i32,
    IARC: i32,
    IBFF: i32,
    IAMH: i32,
    IOSTAT: i32,
    LUN: i32,
    FIRST: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NFT: i32 = 0;
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTLNK = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTND = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTNI = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FINDEX: i32 = 0;
        let mut ACC = vec![b' '; ACCLEN as usize];
        let mut CREC = vec![b' '; CRLEN as usize];
        let mut DAFNAM = vec![b' '; FILEN as usize];
        let mut FORMAT = vec![b' '; FMTLEN as usize];
        let mut IDWORD = vec![b' '; IDLEN as usize];
        let mut IFN = vec![b' '; IFNLEN as usize];
        let mut TTYPE = vec![b' '; TYPLEN as usize];
        let mut DREC = StackArray::<f64, 128>::new(1..=DPRSIZ);
        let mut BWARD: i32 = 0;
        let mut FHLIST = ActualArray::<i32>::new(LBCELL..=FTSIZE);
        let mut FND: i32 = 0;
        let mut FNI: i32 = 0;
        let mut FREE: i32 = 0;
        let mut FNB: i32 = 0;
        let mut FWARD: i32 = 0;
        let mut IARC: i32 = 0;
        let mut IBFF: i32 = 0;
        let mut IAMH: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut LUN: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;

        FIRST = true;
        NFT = 0;

        Self {
            NFT,
            FTHAN,
            FTLNK,
            FTND,
            FTNI,
            FINDEX,
            ACC,
            CREC,
            DAFNAM,
            FORMAT,
            IDWORD,
            IFN,
            TTYPE,
            DREC,
            BWARD,
            FHLIST,
            FND,
            FNI,
            FREE,
            FNB,
            FWARD,
            IARC,
            IBFF,
            IAMH,
            IOSTAT,
            LUN,
            FIRST,
            FOUND,
        }
    }
}

/// DAF, assign handles
///
/// Assign handles to DAFs as they are opened.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  FNAME     I-O  OPR, OPW, ONW, OPN (Obsolete), HFN, FNH
///  FTYPE      I   ONW
///  ND        I-O  ONW, OPN (Obsolete), HSF
///  NI        I-O  ONW, OPN (Obsolete), HSF
///  IFNAME     I   ONW, OPN (Obsolete)
///  RESV       I   ONW, OPN (Obsolete)
///  HANDLE    I-O  OPR, OPW, ONW, OPN (Obsolete), CLS, HLU, LUH, HFN,
///                 FNH, SIH
///  UNIT      I-O  HLU, LUH
///  FHSET      O   HOF
///  ACCESS     I   SIH
///  RECL       P   OPR, OPW, ONW, OPN (Obsolete)
///  FTSIZE     P   OPR, OPW, ONW, OPN (Obsolete), CLS, HLU, LUH, HFN,
///                 FNH
///  FILEN      P   SIH
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    on input, is the name of a DAF to be opened, or
///           the name of a DAF about which some information
///           (handle, logical unit) is requested.
///
///  FTYPE    on input, is a code for the type of data that is
///           contained in the DAF file. This code has no meaning or
///           interpretation at the level of the DAF file
///           architecture, but is provided as a convenience for
///           higher level software. The maximum length for the file
///           type is four (4) characters. If the input string is
///           longer than four characters, the first nonblank
///           character and its three, or fewer, immediate nonblank
///           successors will be used as the file type. The file
///           type may not contain nonprinting characters, and it IS
///           case sensitive.
///
///           NAIF has reserved for its own use file types
///           consisting of the upper case letters (A-Z) and the
///           digits 0-9. NAIF recommends lower case or mixed case
///           file types be used by all others in order to avoid
///           any conflicts with NAIF file types.
///
///  ND       on input, is the number of double precision components
///           in each array summary of a new file.
///
///  NI       on input, is the number of integer components in each
///           array summary in a new file.
///
///  IFNAME   is the internal file name for a DAF to be created.
///
///  RESV     is the number of records to be reserved in a DAF
///           to be created.
///
///  HANDLE   on input, is the handle of a DAF about which some
///           information (file name, logical unit) is requested,
///           or the handle of a DAF to be closed.
///
///  UNIT     on input, is the logical unit connected to a DAF
///           about which some information (file name, handle) is
///           requested.
///
///  ACCESS   is the type of access a DAF is open for, that is,
///           either reading or writing. The values of ACCESS
///           may be
///
///              'READ'
///              'WRITE'
///
///           Leading and trailing blanks are ignored, and case
///           is not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FNAME    on output, is the name of a DAF for which
///           the corresponding handle or logical unit has been
///           supplied.
///
///  ND       on output, is the number of double precision
///           components in each array summary of an existing file.
///
///  NI       on output, is the number of integer components in
///           each array summary in an existing file.
///
///  HANDLE   on output, is the handle of a DAF for which
///           the corresponding file name or logical unit has been
///           supplied.
///
///  UNIT     on output, is the logical unit connected to a DAF
///           for which the corresponding file name or handle has
///           been supplied.
///
///  FHSET    is a SPICE set containing the handles of the
///           currently open DAFs.
/// ```
///
/// # Parameters
///
/// ```text
///  RECL     is the record length of a DAF. Each record
///           must be large enough to hold 128 double
///           precision numbers or 1000 characters, whichever
///           is greater. The units in which the record length
///           must be specified vary from environment to
///           environment. For example, VAX Fortran requires
///           record lengths to be specified in longwords,
///           where two longwords equal one double precision
///           number. See the include file 'zzddhman.inc' for
///           details.
///
///  FTSIZE   is the size of the file table maintained internally
///           by DAFAH. In effect, FTSIZE is the maximum number
///           of DAFs that the DAF routines allow to be open
///           simultaneously. See the include file 'zzddhman.inc'
///           for details.
///
///  FILEN    is the maximum filename length. See the include file
///           'zzddhman.inc' for details.
///
///
///  INTEOC   is the ASCII decimal integer code of the character
///           recognized by SPICE as representing the end of the
///           comment data in the reserved record area.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DAFAH is called directly, the error SPICE(BOGUSENTRY)
///      is signaled.
///
///  2)  See entry points DAFOPR, DAFOPW, DAFONW, DAFOPN, DAFCLS,
///      DAFHSF, DAFHLU, DAFLUH, DAFHFN, DAFNFH, DAFHOF, and DAFSIH for
///      exceptions specific to those entry points.
/// ```
///
/// # Files
///
/// ```text
///  All DAFs opened by this routine are specified by name.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFAH serves as an umbrella, allowing data to be shared by its
///  entry points:
///
///     DAFOPR         Open for read.
///     DAFOPW         Open for write.
///     DAFONW         Open new.
///     DAFOPN         Open new. (Obsolete, use DAFONW )
///
///     DAFCLS         Close.
///
///     DAFHSF         Handle to summary format.
///
///     DAFHLU         Handle to logical unit.
///     DAFLUH         Logical to handle.
///
///     DAFHFN         Handle to name.
///     DAFFNH         File name to handle.
///
///     DAFHOF         Handles of open files.
///     DAFSIH         Signal invalid handles.
///
///  Before a DAF can be used, it must be opened. Entry points
///  DAFOPR and DAFOPW provide the only means for opening an
///  existing DAF.
///
///  Several files may be opened for use simultaneously. (This makes
///  it convenient to combine data from several files to produce a
///  single result.) As each DAF is opened, it is assigned a file
///  handle, which is used to keep track of the file internally, and
///  which is used by the calling program to refer to the file in all
///  subsequent calls to DAF routines.
///
///  DAFs may be opened for two kinds of access: read, and write.
///  Files opened for read access may not be changed in any way. Files
///  opened for write access may be both read and written.
///
///  DAFONW is used to open a new DAF file. This routine extends the
///  functionality of DAFOPN by providing a mechanism for associating a
///  type with the data in the DAF file. The use of this entry over
///  DAFOPN is highly recommended.
///
///  Since the only reason for creating a new file is to write
///  something in it, all new files are opened for write access.
///
///  Entry point DAFOPN, for opening a new DAF file, has been rendered
///  obsolete by the new entry point DAFONW. The entry point DAFOPN
///  will continue to be supported for purposes of backward
///  compatibility, but its use in new software development is
///  discouraged.
///
///  Entry point DAFCLS provides the only official means of closing
///  a DAF that is currently open. Closing a DAF any other way (for
///  example, by determining its logical unit and using the Fortran
///  CLOSE statement directly) may affect your calling program in
///  mysterious ways.
///
///  Entry point DAFHSF allows you to determine the summary format
///  of any DAF that is currently open, without calling DAFRFR to
///  re-read the file record.
///
///  Entry point DAFHOF allows you to determine which DAFs are open
///  at any time. In particular, you can use DAFHOF to determine
///  whether any file handle points to an open DAF.
///
///  Entry point DAFSIH signals errors when it is supplied with invalid
///  handles, so it serves to centralize error handling associated
///  with invalid handles.
///
///  The remaining entry points exist mainly to translate between
///  alternative representations of DAFs. There are three ways to
///  identify any open DAF: by name, by handle, and by logical
///  unit. Given any one of these, you may use these entry points to
///  find the other two.
/// ```
///
/// # Examples
///
/// ```text
///  See entry points DAFOPR, DAFOPW, DAFONW, DAFOPN, DAFCLS, DAFHSF,
///  DAFHLU, DAFLUH, DAFHFN, DAFNFH, DAFHOF, and DAFSIH for examples
///  specific to those entry points.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The value of parameter RECL may need to be changed when DAFAH
///      and its entry points are ported to a new environment (CPU and
///      compiler).
///
///  2)  An integer overflow may occur if the number of files opened
///      by a single program exceeds the maximum number that can be
///      stored in an integer variable.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Sun FORTRAN Programmer's Guide", Sun Microsystems, Rev. A, 6
///       May 1988.
///
///  [2]  "Microsoft Fortran Optimizing Compiler User's Guide",
///       Microsoft Corporation, 1987.
///
///  [3]  "Lahey F77L EM/32 Programmers Reference Manual", version 4.0,
///       p 144.
///
///  [4]  "Language Systems FORTRAN Reference Manual", Language Systems
///       Corporation, p 12-7.
///
///  [5]  "FORTRAN/9000 Reference HP 9000 Series 700 Computers", 1st
///       Edition, June 1991, Hewlett Packard Company, page 5-110.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  J.E. McLean        (JPL)
///  H.A. Neilan        (JPL)
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
/// -    SPICELIB Version 9.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header of DAFAH umbrella routine and all its entry
///         points to comply with NAIF standard.
///
///         Added DAFOPR, DAFOPW, DAFCLS and DAFHSF header examples.
///
///         Corrected spelling mistakes.
///
/// -    SPICELIB Version 9.0.1, 10-OCT-2012 (EDW)
///
///         Edited DAFOPN $Abstract section to use "Deprecated" keyword
///         and state replacement routine.
///
///         Corrected ordering of all header sections.
///
///         Added a functional code example to the $Examples section
///         in DAFOPN and DAFCLS.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 9.0.0, 09-NOV-2006 (NJB)
///
///         Updated the entry point DAFONW so that a non-empty reserved
///         record area will also be a valid empty comment area.  DAFONW
///         now writes a EOC character to the first byte of the second
///         record when the input number of reserved records NRESV is
///         greater than zero.
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         Updated the following entry points in response to changes
///         to the handle manager interfaces:
///
///            DAFCLS
///            DAFOPR
///            DAFOPW
///            DAFONW
///            DAFOPN
///
///         See the $Revisions section for details.
///
///         Minor bug fix to DAFFNH. An error was signaled but the
///         intended call to CHKOUT and RETURN statement were omitted.
///
/// -    SPICELIB Version 8.0.0, 14-NOV-2000 (FST)
///
///         Cleaned up entry point headers by removing duplicate
///         entries from the $Revisions section where appropriate.
///
///         Integrated the new handle manager code into this module.
///         The number of DAFs the system can load is now 1000,
///         and some supported environments can read non-native
///         binary DAFs. See the Convert User's Guide for details.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 22-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 7.0.0, 22-MAR-1999 (FST)
///
///         To accommodate the DAF FTP validation check, the following
///         entry points were modified:
///
///            DAFOPR, DAFOPW, DAFONW, DAFOPN.
///
///         See their headers and code for the details of the changes.
///
/// -    SPICELIB Version 6.0.0, 05-APR-1998 (NJB)
///
///         Added references to the PC-LINUX environment.
///
/// -    SPICELIB Version 5.1.0, 08-MAR-1996 (KRG)
///
///         The Following entry points have been modified: DAFONW and
///         DAFOPN.
///
///         The modifications support the notion of a DAF comment area,
///         and involve writing NULL filled reserved records when the
///         number of reserved records is greater than zero (0).
///
///         Some nested IF...THEN...ELSE IF...THEN...END IF constructs
///         were expanded to be independent IF...THEN...END IF tests.
///         The tests were for IOSTAT errors on cascading write statements
///         nested in the IF...ELSE IF... statements, and this was
///         confusing. These tests were restructured so that IOSTAT is
///         tested after each write statement which is equivalent to the
///         original intent and easier to read.
///
/// -    SPICELIB Version 5.0.0, 27-SEP-1993 (KRG)
///
///         The following entry points have had code modifications:
///         DAFOPR, DAFOPW and DAFOPN.
///
///         A new entry point has been added: DAFONW.
///
///         The modifications are to allow a type to be associated with a
///         DAF file.
///
///         A new parameter has been added to this subroutine's parameter
///         list, FTYPE, so that type information may be passed to the
///         entry point DAFONW. Two new variables were added to the
///         routine as well, TARCH and TTYPE, which provide temporary
///         storage for the file architecture and type.
///
///         Several new parameters have been added to the declarations for
///         this routine:
///
///            ARCLEN   The length of a file architecture.
///
///            MAXPC    The maximum decimal value for the range of
///                     printable characters.
///
///            MINPC    The minimum decimal value for the range of
///                     printable characters.
///
///            TYPLEN   The length of a file type.
///
///         See the individual entry points for detailed descriptions of
///         their modifications.
///
///         Removed the variables MINHAN and NIL, as they were not used in
///         any of the entry points, yet they had values assigned to them
///         through DATA statements.
///
///         Made all occurrences of error message formatting of filenames
///         consistent. All filenames will be single quoted in the output
///         error message.
///
/// -    SPICELIB Version 4.0.0, 25-FEB-1993 (JML)
///
///         In the entry points DAFOPR, DAFOPW, and DAFFNH, the INQUIRE
///         statement that checks if the file is already open now also
///         checks that the file exists.
///
///         IOSTAT is now checked after all INQUIRE statements.
///
///         A new variable LUN is used in DAFOPR, DAFOPW, and DAFOPN
///         for the logical unit number returned by GETLUN.
///
///         The IF-THEN statements in DAFOPR and DAFOPW were reorganized
///         to make the routines more readable.
///
///         In DAFOPR and DAFOPW, a long error message was added for the
///         case when the NAIF/DAF id word was not recognized. Also, the
///         file is closed when this error is signaled.
///
///         In DAFOPR and DAFOPW, IOSTAT is now checked after the file
///         record is read.
///
///         In DAFOPR, DAFOPW, DAFOPN, and DAFFNH, the file name is
///         checked to see if it is blank.
///
///         In DAFOPR, DAFOPW, DAFOPN, and DAFFNH, the file name passed
///         to the FORTRAN OPEN and INQUIRE statements has been chopped
///         at the last non-blank character.
///
///         A minor error in the $Particulars section of the header of
///         DAFCLS was corrected. It formerly stated that a file could be
///         open more than once for read or write access instead of just
///         read access.
///
/// -    SPICELIB Version 3.2.0, 06-OCT-1992 (HAN)
///
///         Module was updated to include the record length and source
///         for the Hewlett Packard UX 9000/750 environment. Moved FILEN
///         to the $Declarations section, and corrected $Revisions section
///         to include the last code change description, 3.1.0.
///
/// -    SPICELIB Version 3.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 3.1.0, 13-NOV-1991 (MJS)
///
///         Module was updated to operate in the Lahey F77 EM/32
///         PC environment.
///
/// -    SPICELIB Version 3.0.0, 03-SEP-1991 (NJB) (WLT)
///
///         DAFAH and its entry points were modified to permit multiple
///         DAFs to be open for writing at the same time. Also, the
///         entry points DAFHOF and DAFSIH were added.
///
/// -    SPICELIB Version 2.0.0, 25-MAR-1991 (JEM) (MJS)
///
///         The variable MINHAN was initialized to zero and the variable
///         NEXT was saved. DAFOPW now accepts the ID word 'NAIF/NIP'
///         as well 'NAIF/DAF'. Spelling mistakes were corrected.
///
/// -    SPICELIB Version 1.1.0, 05-NOV-1990 (HAN)
///
///         The parameter FTSIZE was increased from 4 to 20.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         The entry point ZZDDHCLS in the handle manager (ZZDDHMAN)
///         had its argument list augmented to allow files to be
///         deleted on close. This allows the removal of a series
///         of "raw" CLOSE statements in a few of the entry points
///         of this routine.
///
/// -    SPICELIB Version 8.0.0, 14-NOV-2000 (FST)
///
///         The DAF system now utilizes the handle manager umbrella
///         (ZZDDHMAN) and its entry points to provide most of the
///         handle and logical unit based operations that DAFAH
///         previously managed.
///
///         FTSIZE Files with UTSIZE Units:
///
///         In previous versions of the DAF system all files opened
///         through the DAFAH entry points were connected to logical
///         units. In contrast, the handle manager umbrella entry
///         points allow FTSIZE files to be loaded (opened), while
///         only utilizing UTSIZE (less than FTSIZE, see the include
///         file 'zzddhman.inc') logical units.  The entry points in
///         the handle manager automatically connect and disconnect
///         loaded files from their logical units as new files are
///         loaded and accessed.
///
///         Previously, one could buffer a logical unit associated
///         with a particular handle and access the file directly
///         with Fortran I/O statements.  To preserve this capability
///         invoking DAFHLU locks a handle to its assigned logical
///         unit, until that lock is removed (see ZZDDHUNL, an entry
///         point in ZZDDHMAN) or the file is closed.  See the
///         $Revisions section in the DAFHLU entry point for details.
///
///         Another consequence of the utilization of the handle
///         manager code is that the process of connecting a file
///         name to a HANDLE may require performing up to FTSIZE
///         INQUIRE statements.  This is necessary to insure that
///         different names referring to the same file return the
///         same handle.  This was the case previously with the DAF
///         system since an INQUIRE on a different, but equivalent,
///         file name would produce the same logical unit.
///
///         FTP Error Detection:
///
///         The FTP error detection software is now integrated into
///         the handle manager umbrella entry points, and as such
///         is no longer present in DAFAH.
///
///         Non-Native Files:
///
///         In addition to expanding the number of loaded files the
///         DAF system supports, the handle manager also detects and
///         tracks binary file formats.  This allows a layer of
///         private code that has been inserted between DAF routines
///         and the Fortran I/O statements to provide translation
///         services for DAF.  Some environments are now endowed with
///         the ability to read files created with certain non-native
///         binary file formats.  See the Convert User's Guide for
///         details.
///
/// -    SPICELIB Version 7.0.0, 22-MAR-1999 (FST)
///
///         Binary File Format Identification:
///
///         The file record now contains an 8 character string that
///         identifies the binary file format utilized by DAFs.
///         The purpose of this string's inclusion in the file record
///         is preparatory in nature, to accelerate the migration to
///         files that support the runtime translation update that
///         is scheduled.
///
///         FTP Validation:
///
///         The DAF system now employs a validation scheme to assist
///         users in detecting DAFs potentially corrupted via ASCII mode
///         FTP transfers.  A string that contains sequences of
///         characters commonly corrupted by improper FTP transfers is
///         inserted into the unused portion of the file record. When any
///         DAFAH entry point attempts to open a file, this string is
///         located and examined.  If the string indicates the file is
///         corrupted, the entry point signals an error.
///
///            Detection Scheme Implementation:
///
///            When a new DAF is created, the entry points DAFONW and
///            DAFOPN(obsolete) retrieve the FTP validation string from
///            the defining routine (ZZFTPSTR) and insert it into the
///            tail of the file record.  A diagram illustrating the new
///            file record for 32-bit environments with single byte
///            characters follows:
///
///               +=============+
///               | File Record |
///               |    Data     |
///               +=============+
///                      |
///                +=====|===+==========================+===+========+
///                |     |   |    603 bytes of nulls    | | | nulls  |
///                +=========+==========================+=|=+========+
///            Byte 1                                     |         1024
///                                                  +============+
///                                                  | FTP        |
///                                                  | Validation |
///                                                  | String     |
///                                                  +============+
///
///            As can be seen above, the file record is now null padded,
///            which was not the case previously.
///
///            When an existing DAF is opened, the entry points DAFOPR
///            and DAFOPW attempt to verify that the validation string is
///            intact.  This is accomplished by reading the file
///            record into a character string, and then passing the last
///            half of this string into the validation subroutine
///            ZZFTPCHK.  Only sending the latter half of the file record
///            into ZZFTPCHK is done to prevent other portions of the file
///            record from confusing the validation process.  The following
///            three abnormal situations may arise during validation:
///
///               (1) Older DAFs without the FTP validation string are
///                   not validated.  As far as the DAF open routines
///                   are concerned such files are valid by default. The
///                   only notable exception is that the garbage that
///                   resides in the unused portion of the file record may
///                   confuse ZZFTPCHK into thinking the validation
///                   string is present.  (The probability of this event
///                   is minimal and noted only for completeness.)
///
///               (2) Files with an older version of the validation
///                   string are examined for errors supported by the
///                   contemporaneous version of the Toolkit.
///
///               (3) Files with a newer version of the validation
///                   string are examined for errors supported by the
///                   current version of the Toolkit.
///
///            Updates to the FTP Validation String:
///
///            In the event that it becomes necessary to add additional
///            test characters to the validation string, refer to
///            ZZFTPSTR for the proper procedure.  The instructions
///            provided there ensure that the above behavior is properly
///            adhered to by the modifications.
///
///            FTP Validation Issues in Code Portability:
///
///            The scheme as currently implemented will function
///            properly in any computing environment whose character data
///            conforms to the single byte ASCII standards with a word
///            size that is between 32 and 64 bits inclusive.  Refer to
///            the above diagram that displays the new DAF file record
///            and the following discussion for details.
///
///            Since the DAF file record block contains integer data,
///            it may expand if the word size increases above the
///            currently supported 32 bits.  However, the FTP validation
///            string is extracted by reading in 1000 bytes of character
///            data and examining bytes 500-1000. (See the parameters
///            FTPBLK and FTPSTR if you need to alter these numbers).
///            So as long as the alteration in word size does not cause
///            the FTP string information to shift out of bytes 500-1000
///            in the file record, the existing code will function
///            properly.
///
/// -    SPICELIB Version 3.2.0, 6-OCT-1992 (HAN)
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 3.0.0, 03-SEP-1991 (NJB) (WLT)
///
///         DAFAH and the entry point DAFOPW were modified to permit
///         multiple DAFs to be open for writing at the same time.
///         Also, the entry points DAFHOF and DAFSIH were added.  DAFHOF
///         returns a set containing the handles of currently open DAFs.
///         To accommodate the addition of DAFHOF, the argument FHSET
///         was added to DAFAH's argument list, and local declarations
///         for DAFHOF were added to DAFAH's declaration section.  DAFSIH
///         signals an error if the file indicated by the handle is not
///         open for the specified type of access.
///
/// -    SPICELIB Version 2.0.0, 25-MAR-1991 (JEM) (MJS)
///
///         The entry point DAFOPW accepted only 'NAIF/DAF' as a valid
///         ID word. It now accepts 'NAIF/NIP' as well for
///         backwards compatibility. The entry point DAFOPR did not need
///         this fix because it already accepts both ID words.
///
/// -    SPICELIB Version 1.1.0,  5-NOV-1990 (HAN)
///
///         The parameter FTSIZE was increased from 4 to 20. The number
///         4 was chosen for testing purposes and was not removed.
/// ```
pub fn dafah(
    ctx: &mut SpiceContext,
    fname: &str,
    ftype: &str,
    nd: i32,
    ni: i32,
    ifname: &str,
    resv: i32,
    handle: i32,
    unit: i32,
    fhset: &[i32],
    access: &str,
) -> crate::Result<()> {
    DAFAH(
        fname.as_bytes(),
        ftype.as_bytes(),
        nd,
        ni,
        ifname.as_bytes(),
        resv,
        handle,
        unit,
        fhset,
        access.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFAH ( DAF, assign handles )
pub fn DAFAH(
    FNAME: &[u8],
    FTYPE: &[u8],
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    RESV: i32,
    HANDLE: i32,
    UNIT: i32,
    FHSET: &[i32],
    ACCESS: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // As each file is opened, it is assigned a handle, and the
    // internal file name is stored for comparison with other files.
    // All names in the file table begin with FT.
    //
    //    HAN      Handle
    //    LNK      Number of links
    //    ND,
    //    NI       Summary format
    //
    // The columns are stored in no particular order. New files are
    // added to the end of the list; the list is repacked whenever a
    // file is removed from the list.
    //
    // NFT is the number of files currently opened: this may not be
    // greater than FTSIZE. FINDEX refers to a file of interest within
    // the table.
    //
    // NEXT is incremented each time a file is opened to become the
    // next file handle assigned.
    //

    //
    // Other local variables
    //

    //
    // Saved variables
    //

    //
    // Save everything between calls.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFAH", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"DAFAH", ctx)?;
    }

    Ok(())
}

/// DAF, open for read
///
/// Open a DAF for subsequent read requests.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of DAF to be opened.
///  HANDLE     O   Handle assigned to DAF.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the file name of a DAF to be opened for read
///           access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAF routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file has already been opened for read
///      access, the handle already associated with the file is
///      returned.
///
///  2)  If the specified file has already been opened for write
///      access, an error is signaled by a routine in the call
///      tree of this routine.
///
///  3)  If the specified file has already been opened by a non-DAF
///      routine, an error is signaled by a routine in the call
///      tree of this routine.
///
///  4)  If the specified file cannot be opened without exceeding
///      the maximum number of files, the error SPICE(DAFFTFULL)
///      is signaled.
///
///  5)  If the attempt to read the file's file record fails,
///      the error SPICE(FILEREADFAILED) is signaled.
///
///  6)  If the specified file is not a DAF file, an error is
///      signaled by a routine in the call tree of this routine.
///
///  7)  If no logical units are available, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the file does not exist, an error is signaled by a routine
///      in the call tree of this routine.
///
///  9)  If an I/O error occurs in the process of opening the file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  10) If the file name is blank or otherwise inappropriate,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  11) If the file was transferred improperly via FTP, an error is
///      signaled by a routine in the call tree of this routine.
///
///  12) If the file utilizes a binary file format that is not
///      currently supported on this platform, an error is signaled by
///      a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME.
/// ```
///
/// # Particulars
///
/// ```text
///  Most DAFs require only read access. If you do not need to
///  change the contents of a file, you should open it with DAFOPR.
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
///
///  1) In the following code fragment, DAFOPR is used to open a file,
///     which is then searched for DAFs containing data for a
///     particular object.
///
///        CALL DAFOPR ( FNAME, HANDLE )
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( FOUND  )
///
///        DO WHILE ( FOUND )
///           CALL DAFGS ( SUM )
///           CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///           IF ( IC(1) .EQ. TARGET_OBJECT ) THEN
///            .
///            .
///
///           END IF
///
///           CALL DAFFNA ( FOUND )
///        END DO
///
///
///  2) Use a simple routine to output the double precision and integer
///     values stored in an SPK's segments descriptors. This function
///     opens a DAF for read, performs a forwards search for the DAF
///     arrays, prints segments description for each array found, then
///     closes the DAF.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        de421.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFOPR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Define the summary parameters appropriate
///     C     for an SPK file.
///     C
///           INTEGER               MAXSUM
///           PARAMETER           ( MAXSUM = 125 )
///
///           INTEGER               ND
///           PARAMETER           ( ND = 2 )
///
///           INTEGER               NI
///           PARAMETER           ( NI = 6 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(32)        KERNEL
///
///           DOUBLE PRECISION      DC     ( ND     )
///           DOUBLE PRECISION      SUM    ( MAXSUM )
///
///           INTEGER               HANDLE
///           INTEGER               IC     ( NI     )
///
///           LOGICAL               FOUND
///
///     C
///     C     Open a DAF for read. Return a HANDLE referring to the
///     C     file.
///     C
///           KERNEL = 'de421.bsp'
///           CALL DAFOPR ( KERNEL, HANDLE )
///
///     C
///     C     Begin a forward search on the file.
///     C
///           CALL DAFBFS ( HANDLE )
///
///     C
///     C     Search until a DAF array is found.
///     C
///           CALL DAFFNA ( FOUND )
///
///     C
///     C     Loop while the search finds subsequent DAF arrays.
///     C
///           DO WHILE ( FOUND )
///
///              CALL DAFGS ( SUM )
///              CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///              WRITE(*,*)                'Doubles:', DC(1:ND)
///              WRITE(*, FMT='(A,6I9)' ) 'Integers:', IC(1:NI)
///
///     C
///     C        Check for another segment.
///     C
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        1        0        1        2      641   310404
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        2        0        1        2   310405   423048
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        3        0        1        2   423049   567372
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        4        0        1        2   567373   628976
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        5        0        1        2   628977   674740
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        6        0        1        2   674741   715224
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        7        0        1        2   715225   750428
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        8        0        1        2   750429   785632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        9        0        1        2   785633   820836
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:       10        0        1        2   820837   944040
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      301        3        1        2   944041  1521324
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      399        3        1        2  1521325  2098608
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      199        1        1        2  2098609  2098620
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      299        2        1        2  2098621  2098632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      499        4        1        2  2098633  2098644
///
///
///     Note, the final entries in the integer array contain the
///     segment start/end indexes. The output indicates the search
///     proceeded from the start of the file (low value index) towards
///     the end (high value index).
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Files opened using this routine must be closed with DAFCLS.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Updated code example with IMPLICIT NONE, and declarations of
///         SUM variable and MAXSUM parameter.
///
///         Corrected minor typos in header.
///
/// -    SPICELIB Version 8.1.1, 10-OCT-2012 (EDW)
///
///         Added a functional code example to the $Examples section.
///
///         Removed the unneeded $Revisions section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 5.0.0, 03-MAR-1999 (FST)
///
///         This entry point now attempts to locate and validate the
///         FTP validation string contained in the file record.
///
///         See the $Revisions section under DAFAH for a discussion
///         of the impact of the changes made for this version.
///
/// -    SPICELIB Version 4.0.0, 27-SEP-1993 (KRG)
///
///         This routine was modified to use a subroutine to obtain the
///         architecture of the file rather than using hard coded values
///         for comparison with the file ID word. This was done in order to
///         isolate the code which checks to determine a file architecture
///         and to make the identification of file types easier through a
///         change to the file ID word.
///
///         In particular, the changes to this routine support the change
///         of the file ID word from 'NAIF/DAF' or 'NAIF/NIP' to 'DAF/xxxx'
///         where 'xxxx' represents a four character mnemonic code for the
///         type of data in the file.
///
///         Removed the error SPICE(DAFNOIDWORD) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADAFFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAF file.
///
///         Changed the long error message when the error
///         SPICE(NOTADAFFILE) is signaled to suggest that a common error
///         is attempting to load a text version of the desired file rather
///         than the binary version.
///
/// -    SPICELIB Version 3.0.0, 25-FEB-1993 (JML)
///
///         The INQUIRE statement that checks if the file is already open
///         now also checks that the file exists.
///
///         A new variable LUN is used for the logical unit number
///         returned by GETLUN.
///
///         The IF-THEN statements were reorganized to improve readability.
///
///         A long error message is now set when the DAF id word is not
///         recognized. Also, the file is closed when this error is
///         signaled.
///
///         IOSTAT is checked after the file record is read.
///
///         The file name is checked to see if it is blank.
///
///         The file name string that is passed to the FORTRAN OPEN and
///         INQUIRE statements has been chopped at the last non-blank
///         character.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 03-SEP-1991 (NJB) (WLT)
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
///         Some error messages were changed so that they specify
///         names of relevant DAFs.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafopr(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DAFOPR(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFOPR ( DAF, open for read )
pub fn DAFOPR(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFOPR", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Attempt to open the file; perform any appropriate checks.
    //
    ZZDDHOPN(FNAME, b"READ", b"DAF", HANDLE, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFOPR", ctx)?;
        return Ok(());
    }

    //
    // See if this file is already present in the file table.  If it
    // is simply increment its link count by one, check out and
    // return.
    //
    save.FINDEX = ISRCHI(*HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX != 0) {
        save.FTLNK[save.FINDEX] = (save.FTLNK[save.FINDEX] + 1);
        CHKOUT(b"DAFOPR", ctx)?;
        return Ok(());
    }

    //
    // Retrieve ND and NI from the file record.
    //
    ZZDAFGFR(
        *HANDLE,
        &mut save.IDWORD,
        &mut save.FND,
        &mut save.FNI,
        &mut save.IFN,
        &mut save.FWARD,
        &mut save.BWARD,
        &mut save.FREE,
        &mut save.FOUND,
        ctx,
    )?;

    if !save.FOUND {
        ZZDDHCLS(*HANDLE, b"DAF", false, ctx)?;
        SETMSG(
            b"Error reading the file record from the binary DAF file \'#\'.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DAFOPR", ctx)?;
        return Ok(());
    }

    //
    // At this point, we know that we have a valid DAF file, and we're
    // set up to read from it, so ...
    //
    // Update the file table to include information about our newly
    // opened DAF.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTND[save.NFT] = save.FND;
    save.FTNI[save.NFT] = save.FNI;
    save.FTLNK[save.NFT] = 1;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DAFOPR", ctx)?;
    Ok(())
}

/// DAF, open for write
///
/// Open a DAF for subsequent write requests.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of DAF to be opened.
///  HANDLE     O   Handle assigned to DAF.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a DAF to be opened with write access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This handle
///           is used to identify the file in subsequent calls to other
///           DAF routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file has already been opened, either by the
///      DAF routines or by other code, an error is signaled by a
///      routine in the call tree of this routine. Note that this
///      response is not paralleled by DAFOPR, which allows you to open
///      a DAF for reading even if it is already open for reading.
///
///  2)  If the specified file cannot be opened without exceeding the
///      maximum number of files, the error SPICE(DAFFTFULL) is
///      signaled.
///
///  3)  If the attempt to read the file's file record fails, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  4)  If the specified file is not a DAF file, an error is signaled
///      by a routine in the call tree of this routine.
///
///  5)  If no logical units are available, an error is signaled by a
///      routine in the call tree of this routine.
///
///  6)  If the file does not exist, an error is signaled by a routine
///      in the call tree of this routine.
///
///  7)  If an I/O error occurs in the process of opening the file, the
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  8)  If the file name is blank or otherwise inappropriate, an error
///      is signaled by a routine in the call tree of this routine.
///
///  9)  If the file was transferred improperly via FTP, an error is
///      signaled by a routine in the call tree of this routine.
///
///  10) If the file utilizes a non-native binary file format, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME.
/// ```
///
/// # Particulars
///
/// ```text
///  Most DAFs require only read access. If you do not need to
///  change the contents of a file, you should open it with DAFOPR.
///  Use DAFOPW when you need to
///
///     -- change (update) one or more summaries, names, or
///        arrays within a file; or
///
///     -- add new arrays to a file.
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
///  1) Delete the entire comment area of a DAF file. Note that this
///     action should only be performed if fresh new comments are to
///     be placed within the DAF file.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        earthstns_itrf93_201023.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFOPW_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         KERNEL
///           PARAMETER           ( KERNEL =
///          .                         'earthstns_itrf93_201023.bsp' )
///
///           INTEGER               BUFFSZ
///           PARAMETER           ( BUFFSZ = 10 )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 1000 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(LINLEN)    BUFFER ( BUFFSZ )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///
///           LOGICAL               DONE
///
///     C
///     C     Open a DAF for write. Return a HANDLE referring to the
///     C     file.
///     C
///           CALL DAFOPW ( KERNEL, HANDLE )
///
///     C
///     C     Print the first 10 lines of comments from the DAF file.
///     C
///           WRITE(*,'(A)') 'Comment area of input DAF file '
///          .            // '(max. 10 lines): '
///           WRITE(*,'(A)') '---------------------------------------'
///          .            // '-----------------------'
///
///           CALL DAFEC  ( HANDLE, BUFFSZ, N, BUFFER, DONE )
///
///           DO I = 1, N
///
///              WRITE (*,*) BUFFER(I)(:RTRIM(BUFFER(I)))
///
///           END DO
///
///           WRITE(*,'(A)') '---------------------------------------'
///          .            // '-----------------------'
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Deleting entire comment area...'
///
///     C
///     C     Delete all the comments from the DAF file.
///     C
///           CALL DAFDC ( HANDLE )
///
///     C
///     C     Close the DAF file and re-open it for read
///     C     access to work around the DAFEC restriction
///     C     on comments not to be modified while they are
///     C     being extracted.
///     C
///           CALL DAFCLS( HANDLE  )
///
///           CALL DAFOPR( KERNEL, HANDLE  )
///
///     C
///     C     Check if the comments have indeed been deleted.
///     C
///           CALL DAFEC  ( HANDLE, BUFFSZ, N, BUFFER, DONE )
///
///           IF ( DONE .AND. N .EQ. 0 ) THEN
///
///              WRITE(*,*) ' '
///              WRITE(*,*) '   Successful operation.'
///
///           ELSE
///
///              WRITE(*,*) ' '
///              WRITE(*,*) '   Operation failed.'
///
///           END IF
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Comment area of input DAF file (max. 10 lines):
///     --------------------------------------------------------------
///
///         SPK for DSN Station Locations
///         ========================================================***
///
///         Original file name:                   earthstns_itrf93_2***
///         Creation date:                        2020 October 28 12:30
///         Created by:                           Nat Bachman  (NAIF***
///
///
///         Introduction
///     --------------------------------------------------------------
///
///      Deleting entire comment area...
///
///         Successful operation.
///
///
///     Warning: incomplete output. 3 lines extended past the right
///     margin of the header and have been truncated. These lines are
///     marked by "***" at the end of each line.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Only files of the native binary file format may be opened
///      with this routine.
///
///  2)  Files opened using this routine must be closed with DAFCLS.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  J.E. McLean        (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Updated $Index_Entries.
///
/// -    SPICELIB Version 8.1.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 6.0.0, 03-MAR-1999 (FST)
///
///         This entry point now attempts to locate and validate the
///         FTP validation string contained in the file record.
///
/// -    SPICELIB Version 5.0.0, 27-SEP-1993 (KRG)
///
///         This routine was modified to use a subroutine to obtain the
///         architecture of the file rather than using hard coded values
///         for comparing to the file ID word. This was done in order to
///         isolate the code which checks to determine a file architecture,
///         and to make the identification of file types easier through a
///         change to the file ID word.
///
///         In particular, the changes to this routine support the change
///         of the file ID word from 'NAIF/DAF' or 'NAIF/NIP' to 'DAF/xxxx'
///         where 'xxxx' represents a four character mnemonic code for the
///         type of data in the file.
///
///         Removed the error SPICE(DAFNOIDWORD) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADAFFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAF file.
///
///         Changed the long error message when the error
///         SPICE(NOTADAFFILE) is signaled to suggest that a common error
///         is attempting to load a text version of the desired file rather
///         than the binary version.
///
/// -    SPICELIB Version 4.0.0, 25-FEB-1993 (JML)
///
///         The INQUIRE statement that checks if the file is already open
///         now also checks that the file exists.
///
///         A new variable LUN is used for the logical unit number
///         returned by GETLUN.
///
///         The IF-THEN statements were reorganized to improve readability.
///
///         A long error message is now set when the DAF id word is not
///         recognized. Also, the file is closed when this error is
///         signaled.
///
///         IOSTAT is now checked after the file record is read.
///
///         The file name is checked to see if it is blank.
///
///         The file name string that is passed to the FORTRAN OPEN and
///         INQUIRE statements has been chopped at the last non-blank
///         character.
///
/// -    SPICELIB Version 3.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 3.0.0, 03-SEP-1991 (NJB) (WLT)
///
///         DAFOPW now allows multiple files to be open for writing.
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
/// -    SPICELIB Version 2.0.0, 24-JAN-1991 (JEM)
///
///         DAFOPW now accepts the ID word 'NAIF/NIP' as well 'NAIF/DAF'.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.0.0, 03-MAR-1999 (FST)
///
///         See the $Revisions section under DAFAH for a discussion
///         of the impact of the changes made for this version.
///
/// -    SPICELIB Version 2.0.0, 24-JAN-1991 (NJB) (WLT)
///
///         DAFOPW now allows multiple files to be open for writing.
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
///         Some error messages were changed so that they specify
///         names of relevant DAFs.
/// ```
pub fn dafopw(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DAFOPW(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFOPW ( DAF, open for write )
pub fn DAFOPW(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFOPW", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Check to see if there is room in the file table.
    //
    if (save.NFT == FTSIZE) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DAFFTFULL)", ctx)?;
        CHKOUT(b"DAFOPW", ctx)?;
        return Ok(());
    }

    //
    // Attempt to open the file; perform any appropriate checks.
    //
    ZZDDHOPN(FNAME, b"WRITE", b"DAF", HANDLE, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFOPW", ctx)?;
        return Ok(());
    }

    //
    // Retrieve ND and NI from the file record.
    //
    ZZDAFGFR(
        *HANDLE,
        &mut save.IDWORD,
        &mut save.FND,
        &mut save.FNI,
        &mut save.IFN,
        &mut save.FWARD,
        &mut save.BWARD,
        &mut save.FREE,
        &mut save.FOUND,
        ctx,
    )?;

    if !save.FOUND {
        ZZDDHCLS(*HANDLE, b"DAF", false, ctx)?;
        SETMSG(
            b"Error reading the file record from the binary DAF file \'#\'.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DAFOPW", ctx)?;
        return Ok(());
    }

    //
    // At this point, we know that we have a valid DAF file, and we're
    // set up to write to it or read from it, so ...
    //
    // Update the file table to include information about our
    // newly opened DAF.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTND[save.NFT] = save.FND;
    save.FTNI[save.NFT] = save.FNI;
    save.FTLNK[save.NFT] = 1;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DAFOPW", ctx)?;
    Ok(())
}

/// DAF, open new
///
/// Open a new DAF for subsequent write requests.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of DAF to be opened.
///  FTYPE      I   Mnemonic code for type of data in the DAF file.
///  ND         I   Number of double precision components in summaries.
///  NI         I   Number of integer components in summaries.
///  IFNAME     I   Internal file name.
///  RESV       I   Number of records to reserve.
///  HANDLE     O   Handle assigned to DAF.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new DAF to be created (and
///           consequently opened for write access).
///
///  FTYPE    is a code for type of data placed into a DAF file.
///           The first nonblank character and the three (3)
///           characters immediately following it, giving four (4)
///           characters, are used to represent the type of the data
///           placed in the DAF file. This is provided as a
///           convenience for higher level software. It is an error
///           if this string is blank. When written to the DAF file,
///           the value for the type IS case sensitive; what you put
///           in is what you get out, so be careful.
///
///           NAIF has reserved for its own use file types
///           consisting of the upper case letters (A-Z) and the
///           digits 0-9. NAIF recommends lower case or mixed case
///           file types be used by all others in order to avoid
///           any conflicts with NAIF file types.
///
///  ND       is the number of double precision components
///           in each array summary of the new file.
///
///  NI       is the number of integer components in each
///           array summary in the new file.
///
///  IFNAME   is the internal file name (containing as many as 60
///           characters) for the new file. This should uniquely
///           identify the file.
///
///  RESV     is the number of records in the new file to be
///           reserved; these records will not be used to store any
///           data belonging to DAF arrays subsequently written to
///           the file. The user may reserve records 2 through (2 +
///           RESV - 1) in the file. SPICE kernels based on the DAF
///           format use the reserved record area to store optional
///           textual information; for these kernels, the reserved
///           records contain the file's "comment area."
///
///           When RESV is non-zero, this routine writes an
///           end-of-comments character into the first byte of
///           record 2, and fills the rest of the allocated records
///           will null (ASCII code 0) characters.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAF routines.
/// ```
///
/// # Parameters
///
/// ```text
///  INTEOC   is the ASCII decimal integer code of the character
///           recognized by SPICE as representing the end of the
///           comment data in the reserved record area.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file cannot be opened without exceeding
///      the maximum number of files, the error SPICE(DAFFTFULL)
///      is signaled.
///
///  2)  If the input argument ND is out of the range [0, 124]
///      or if NI is out of the range [2, 250], the error
///      SPICE(DAFINVALIDPARAMS) is signaled.
///
///  3)  If
///
///         ND + ( NI + 1 ) / 2   >  125
///
///      the error SPICE(DAFINVALIDPARAMS) is signaled.
///
///  4)  If the number of records to be reserved is not zero or
///      positive, the error SPICE(DAFNORESV) is signaled.
///
///  5)  If an I/O error occurs in the process of opening the file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  6)  If (for some reason) the initial records in the file cannot
///      be written, the error SPICE(DAFWRITEFAIL) is signaled.
///
///  7)  If no logical units are available, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the file name is blank or otherwise inappropriate,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  9)  If the file type is blank, the error SPICE(BLANKFILETYPE)
///      is signaled.
///
///  10) If the file type contains nonprinting characters, decimal
///      0-31 and 127-255, the error SPICE(ILLEGALCHARACTER) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine supersedes DAFOPN as the method for opening a new DAF
///  file. It includes a data type identifier as part of the ID word of
///  a DAF file it creates.
///
///  The DAFs created by DAFONW have initialized file records but
///  do not yet contain any arrays. See the DAF Required Reading
///  for a discussion of file records.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, DAFONW is used to open a file,
///  to which a new array is then added. This file will have the data
///  type 'TEST' which may be used to distinguish production data from
///  test data at a user subroutine level.
///
///     FNAME = 'test.bin'
///     FTYPE = 'TEST'
///
///     CALL DAFONW   ( FNAME, FTYPE,  ND,  NI,  IFNAME, 0, HANDLE )
///
///     CALL DAFBNA   ( HANDLE, SUM, NAME  )
///     CALL GET_DATA ( DATA,   N,   FOUND )
///
///     DO WHILE ( FOUND )
///
///        CALL DAFADA   ( DATA, N        )
///        CALL GET_DATA ( DATA, N, FOUND )
///
///     END DO
///
///     CALL DAFENA
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Files opened using this routine must be closed with DAFCLS.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 9.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 9.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 9.0.0, 09-NOV-2006 (NJB)
///
///         DAFONW now writes a EOC character to the first byte
///         of the second record when NRESV > 0.
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 03-MAR-1999 (FST)
///
///         The entry point was modified to insert the FTP validation
///         string, as well as the binary file format into the file record.
///
/// -    SPICELIB Version 1.1.0, 08-MAR-1996 (KRG)
///
///         The modifications support the notion of a DAF comment area,
///         and involve writing NULL filled reserved records when the
///         number of reserved records is greater than zero (0).
///
///         Some nested IF...THEN...ELSE IF...THEN...END IF constructs
///         were expanded to be independent IF...THEN...END IF tests.
///         The tests were for IOSTAT errors on cascading write statements
///         nested in the IF...ELSE IF... statements, and this was
///         confusing. These tests were restructured so that IOSTAT is
///         tested after each write statement which is equivalent to the
///         original intent and easier to read.
///
/// -    SPICELIB Version 1.0.0, 29-SEP-1993 (KRG)
///
///         This routine implements the notion of a file type for DAF
///         files. It allows type information to be added to the file ID
///         word.
///
///         This routine is a modified version of DAFOPN. See the revision
///         history of that entry point for details of changes before the
///         creation of this entry point.
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 9.0.0, 09-NOV-2006 (NJB)
///
///         DAFONW now writes a EOC character to the first byte
///         of the second record when NRESV > 0.
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 03-MAR-1999 (FST)
///
///         The entry point was modified to insert the FTP validation
///         string, as well as the binary file format into the file record.
///
/// -    SPICELIB Version 1.1.0, 08-MAR-1996 (KRG)
///
///         The modifications support the notion of a DAF comment area,
///         and involve writing NULL filled reserved records when the
///         number of reserved records is greater than zero (0).
///
///         Some nested IF...THEN...ELSE IF...THEN...END IF constructs
///         were expanded to be independent IF...THEN...END IF tests.
///         The tests were for IOSTAT errors on cascading write statements
///         nested in the IF...ELSE IF... statements, and this was
///         confusing. These tests were restructured so that IOSTAT is
///         tested after each write statement which is equivalent to the
///         original intent and easier to read.
///
/// -    SPICELIB Version 2.0.0, 03-MAR-1999 (FST)
///
///         See the $Revisions section under DAFAH for a discussion
///         of the impact of the changes made for this version.
/// ```
pub fn dafonw(
    ctx: &mut SpiceContext,
    fname: &str,
    ftype: &str,
    nd: i32,
    ni: i32,
    ifname: &str,
    resv: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    DAFONW(
        fname.as_bytes(),
        ftype.as_bytes(),
        nd,
        ni,
        ifname.as_bytes(),
        resv,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFONW ( DAF, open new )
pub fn DAFONW(
    FNAME: &[u8],
    FTYPE: &[u8],
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    RESV: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFONW", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Check to see if there is room in the file table.
    //
    if (save.NFT == FTSIZE) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DAFFTFULL)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Check if the file type is blank.
    //
    if fstr::eq(FTYPE, b" ") {
        SETMSG(b"The file type is blank.", ctx);
        SIGERR(b"SPICE(BLANKFILETYPE)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Check for nonprinting characters in the file type.
    //
    save.FNB = LTRIM(FTYPE);

    for I in save.FNB..=RTRIM(FTYPE) {
        if ((intrinsics::ICHAR(fstr::substr(FTYPE, I..=I)) > MAXPC)
            || (intrinsics::ICHAR(fstr::substr(FTYPE, I..=I)) < MINPC))
        {
            SETMSG(b"The file type contains nonprinting characters.", ctx);
            SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
            CHKOUT(b"DAFONW", ctx)?;
            return Ok(());
        }
    }

    //
    // Set the value the file type in a temporary variable to be sure of
    // its length and then set the value of the ID word. Only 4
    // characters are allowed for the file type, and they are the first
    // nonblank character and its three (3), or fewer, immediate
    // successors in the input string FTYPE.
    //
    fstr::assign(&mut save.TTYPE, fstr::substr(FTYPE, save.FNB..));
    fstr::assign(&mut save.IDWORD, &fstr::concat(b"DAF/", &save.TTYPE));

    //
    // Make sure ND and NI are in range.
    //
    if ((ND < 0) || (ND > MAXND)) {
        SETMSG(b"ND was #, should be in range [0,#].", ctx);
        ERRINT(b"#", ND, ctx);
        ERRINT(b"#", MAXND, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    if ((NI < 2) || (NI > MAXNI)) {
        SETMSG(b"NI was #, should be in range [2,#].", ctx);
        ERRINT(b"#", NI, ctx);
        ERRINT(b"#", MAXNI, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    if ((ND + ((NI + 1) / 2)) > MAXSUM) {
        SETMSG(b"Summary size was #, should not exceed #.", ctx);
        ERRINT(b"#", (ND + ((NI + 1) / 2)), ctx);
        ERRINT(b"#", MAXSUM, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // The user must reserve some non-negative number of records.
    //
    if (RESV < 0) {
        SETMSG(
            b"An attempt was made to reserve a negative number (#) of records.",
            ctx,
        );
        ERRINT(b"#", RESV, ctx);
        SIGERR(b"SPICE(DAFNORESV)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Attempt to create the file; perform any appropriate checks.
    //
    ZZDDHOPN(FNAME, b"NEW", b"DAF", HANDLE, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut save.IFN, IFNAME);
    save.FND = ND;
    save.FNI = NI;
    save.FWARD = (RESV + 2);
    save.BWARD = save.FWARD;
    fstr::assign(&mut save.CREC, b" ");

    CLEARD(128, save.DREC.as_slice_mut());
    DAFRWA((save.FWARD + 2), 1, &mut save.FREE, ctx)?;

    //
    // Fetch a logical unit for HANDLE.
    //
    ZZDDHHLU(*HANDLE, b"DAF", false, &mut save.LUN, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Fetch the system file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut save.FORMAT, ctx);

    //
    // Write the new file record to the logical unit, LUN.
    //
    ZZDAFNFR(
        save.LUN,
        &save.IDWORD,
        save.FND,
        save.FNI,
        &save.IFN,
        save.FWARD,
        save.BWARD,
        save.FREE,
        &save.FORMAT,
        ctx,
    )?;

    //
    // Check to see whether or not ZZDAFNFR generated an error writing
    // the file record to the logical unit.  In the event an error
    // occurs, checkout and return.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Write NULL filled reserved records.
    //
    if (RESV > 0) {
        for I in 1..=CRLEN {
            fstr::assign(
                fstr::substr_mut(&mut save.CREC, I..=I),
                &intrinsics::CHAR(0),
            );
        }

        for I in 2..=(RESV + 1) {
            //
            // Place an end-of-comments marker in the first byte
            // of the first record.
            //
            if (I == 2) {
                fstr::assign(
                    fstr::substr_mut(&mut save.CREC, 1..=1),
                    &intrinsics::CHAR(INTEOC),
                );
            } else {
                fstr::assign(
                    fstr::substr_mut(&mut save.CREC, 1..=1),
                    &intrinsics::CHAR(0),
                );
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some(I))?;
                save.IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&save.CREC)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (save.IOSTAT != 0) {
                ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;
                SETMSG(
                    b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
                    ctx,
                );
                ERRCH(b"#", FNAME, ctx);
                ERRINT(b"#", save.IOSTAT, ctx);
                SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
                CHKOUT(b"DAFONW", ctx)?;
                return Ok(());
            }
        }
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some(save.FWARD))?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            for n in save.DREC.iter() {
                writer.write_f64(*n)?;
            }
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;
        SETMSG(
            b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer =
            io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some((save.FWARD + 1)))?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&save.CREC)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;
        SETMSG(
            b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFONW", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about our newly
    // opened DAF.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTND[save.NFT] = save.FND;
    save.FTNI[save.NFT] = save.FNI;
    save.FTLNK[save.NFT] = 1;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DAFONW", ctx)?;
    Ok(())
}

/// DAF, open new
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine DAFONW. NAIF supports this routine only to provide
/// backward compatibility.
///
/// Open a new DAF for subsequent write requests.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of DAF to be opened.
///  ND         I   Number of double precision components in summaries.
///  NI         I   Number of integer components in summaries.
///  IFNAME     I   Internal file name.
///  RESV       I   Number of records to reserve.
///  HANDLE     O   Handle assigned to DAF.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new DAF to be created (and
///           consequently open for write access).
///
///  ND       is the number of double precision components
///           in each array summary of the new file.
///
///  NI       is the number of integer components in each
///           array summary in the new file.
///
///  IFNAME   is the internal file name (containing as many as 60
///           characters) for the new file. This should uniquely
///           identify the file.
///
///  RESV     is the number of records in the new file to be
///           reserved for non-DAF use. The user may reserve
///           records 2 through (2 + RESV - 1) in the file.
///           These records are not used to store DAF data,
///           and are in fact invisible to all DAF routines.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAF routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file cannot be opened without exceeding
///      the maximum number of files, the error SPICE(DAFFTFULL)
///      is signaled.
///
///  2)  If the input argument ND is out of the range [0, 124]
///      or if NI is out of the range [2, 250], the error
///      SPICE(DAFINVALIDPARAMS) is signaled.
///
///  3)  If
///
///         ND + ( NI + 1 ) / 2   >  125
///
///      the error SPICE(DAFINVALIDPARAMS) is signaled.
///
///  4)  If the number of records to be reserved is not zero or
///      positive, the error SPICE(DAFNORESV) is signaled.
///
///  5)  If an I/O error occurs in the process of opening the file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  6)  If (for some reason) the initial records in the file cannot
///      be written, the error SPICE(DAFWRITEFAIL) is signaled.
///
///  7)  If no logical units are available, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the file name is blank or otherwise inappropriate,
///      an error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME.
/// ```
///
/// # Particulars
///
/// ```text
///  The DAFs created by DAFOPN have initialized file records but
///  do not yet contain any arrays. See the DAF Required Reading
///  for a discussion of file records.
///
///  This entry point has been made obsolete by the entry point DAFONW.
///  It is supported for reasons of backward compatibility only. New
///  software development should use the entry point DAFONW.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, DAFOPN is used to open a file,
///  to which a new array is then added.
///
///     CALL DAFOPN   ( FNAME,  ND,  NI,  IFNAME, 0, HANDLE )
///
///     CALL DAFBNA   ( HANDLE, SUM, NAME  )
///     CALL GET_DATA ( DATA,   N,   FOUND )
///
///     DO WHILE ( FOUND )
///
///        CALL DAFADA   ( DATA, N        )
///        CALL GET_DATA ( DATA, N, FOUND )
///
///     END DO
///
///     CALL DAFENA
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Files opened using this routine must be closed with DAFCLS.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.1, 10-OCT-2012 (EDW)
///
///         Edited $Abstract section to use "Deprecated" keyword
///         and state replacement routine.
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 4.0.0, 03-MAR-1999 (FST)
///
///         The entry point was modified to insert the FTP validation
///         string, as well as the binary file format into the file record.
///
/// -    SPICELIB Version 3.1.0, 08-MAR-1996 (KRG)
///
///         The modifications support the notion of a DAF comment area,
///         and involve writing NULL filled reserved records when the
///         number of reserved records is greater than zero (0).
///
///         Some nested IF...THEN...ELSE IF...THEN...END IF constructs
///         were expanded to be independent IF...THEN...END IF tests.
///         The tests were for IOSTAT errors on cascading write statements
///         nested in the IF...ELSE IF... statements, and this was
///         confusing. These tests were restructured so that IOSTAT is
///         tested after each write statement which is equivalent to the
///         original intent and easier to read.
///
/// -    SPICELIB Version 3.0.0, 29-SEP-1993 (KRG)
///
///         Modified the logical structure of some
///            IF ... THEN ... ELSE IF... END IF
///         statements which were testing different items in each ELSE IF
///         clause for failure into separate IF ... END IF statements. This
///         improved the readability and supportability of the code.
///
/// -    SPICELIB Version 2.1.0, 25-FEB-1993 (JML)
///
///         A new variable LUN is used for the logical unit number
///         returned by GETLUN.
///
///         The file name is checked to see if it is blank.
///
///         The file name string that is passed to the FORTRAN OPEN and
///         INQUIRE statements has been chopped at the last non-blank
///         character.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 03-SEP-1991 (NJB) (HAN) (WLT)
///
///         Updated to allow multiple DAFs to be open for write
///         access simultaneously. An error in a calling sequence
///         shown in the $Examples section was corrected.
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.0.0, 03-MAR-1999 (FST)
///
///         See the $Revisions section under DAFAH for a discussion
///         of the impact of the changes made for this version.
///
/// -    SPICELIB Version 2.0.0, 03-SEP-1991 (NJB) (HAN) (WLT)
///
///         Updated to allow multiple DAFs to be open for write
///         access simultaneously.
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
///         Invalid values of ND and NI are now screened; two new
///         exceptions were added to the $Exceptions header section.
///
///         The calling sequence of DAFADA shown in the first example
///         in the $Examples section was reversed; this was fixed.
///
///         Some error messages were changed so that they specify
///         names of relevant DAFs.
/// ```
pub fn dafopn(
    ctx: &mut SpiceContext,
    fname: &str,
    nd: i32,
    ni: i32,
    ifname: &str,
    resv: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    DAFOPN(
        fname.as_bytes(),
        nd,
        ni,
        ifname.as_bytes(),
        resv,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFOPN ( DAF, open new )
pub fn DAFOPN(
    FNAME: &[u8],
    ND: i32,
    NI: i32,
    IFNAME: &[u8],
    RESV: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFOPN", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Check to see if there is room in the file table.
    //
    if (save.NFT == FTSIZE) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DAFFTFULL)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // Make sure ND and NI are in range.
    //
    if ((ND < 0) || (ND > MAXND)) {
        SETMSG(b"ND was #, should be in range [0,#].", ctx);
        ERRINT(b"#", ND, ctx);
        ERRINT(b"#", MAXND, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    if ((NI < 2) || (NI > MAXNI)) {
        SETMSG(b"NI was #, should be in range [2,#].", ctx);
        ERRINT(b"#", NI, ctx);
        ERRINT(b"#", MAXNI, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    if ((ND + ((NI + 1) / 2)) > MAXSUM) {
        SETMSG(b"Summary size was #, should not exceed #.", ctx);
        ERRINT(b"#", (ND + ((NI + 1) / 2)), ctx);
        ERRINT(b"#", MAXSUM, ctx);
        SIGERR(b"SPICE(DAFINVALIDPARAMS)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // The user must reserve some non-negative number of records.
    //
    if (RESV < 0) {
        SETMSG(
            b"An attempt was made to reserve a negative number (#) of records.",
            ctx,
        );
        ERRINT(b"#", RESV, ctx);
        SIGERR(b"SPICE(DAFNORESV)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // Attempt to create the file; perform any appropriate checks.
    //
    ZZDDHOPN(FNAME, b"NEW", b"DAF", HANDLE, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut save.IFN, IFNAME);
    save.FND = ND;
    save.FNI = NI;
    save.FWARD = (RESV + 2);
    save.BWARD = save.FWARD;
    fstr::assign(&mut save.CREC, b" ");

    CLEARD(128, save.DREC.as_slice_mut());
    DAFRWA((save.FWARD + 2), 1, &mut save.FREE, ctx)?;

    //
    // Fetch a logical unit for HANDLE.
    //
    ZZDDHHLU(*HANDLE, b"DAF", false, &mut save.LUN, ctx)?;

    //
    // Check FAILED(); return if an error has occurred.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // Fetch the system file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut save.FORMAT, ctx);

    //
    // Write the new file record to the logical unit, LUN.
    //
    ZZDAFNFR(
        save.LUN,
        b"NAIF/DAF",
        save.FND,
        save.FNI,
        &save.IFN,
        save.FWARD,
        save.BWARD,
        save.FREE,
        &save.FORMAT,
        ctx,
    )?;
    //
    // Check to see whether or not ZZDAFNFR generated an error writing
    // the file record to the logical unit.  In the event an error
    // occurs, checkout and return.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // Write NULL filled reserved records.
    //
    if (RESV > 0) {
        for I in 1..=CRLEN {
            fstr::assign(
                fstr::substr_mut(&mut save.CREC, I..=I),
                &intrinsics::CHAR(0),
            );
        }

        for I in 2..=(RESV + 1) {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some(I))?;
                save.IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&save.CREC)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (save.IOSTAT != 0) {
                ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;

                SETMSG(
                    b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
                    ctx,
                );
                ERRCH(b"#", FNAME, ctx);
                ERRINT(b"#", save.IOSTAT, ctx);
                SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
                CHKOUT(b"DAFOPN", ctx)?;
                return Ok(());
            }
        }
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some(save.FWARD))?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            for n in save.DREC.iter() {
                writer.write_f64(*n)?;
            }
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;

        SETMSG(
            b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer =
            io::UnformattedWriter::new(ctx.io_unit(save.LUN)?, Some((save.FWARD + 1)))?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&save.CREC)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        ZZDDHCLS(*HANDLE, b"DAF", true, ctx)?;

        SETMSG(
            b"Attempt to write file \'#\' failed. Value of IOSTAT was #.",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", save.IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFOPN", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about
    // our newly opened DAF.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTND[save.NFT] = save.FND;
    save.FTNI[save.NFT] = save.FNI;
    save.FTLNK[save.NFT] = 1;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DAFOPN", ctx)?;
    Ok(())
}

/// DAF, close
///
/// Close the DAF associated with a given handle.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAF to be closed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a previously opened DAF file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to a DAF
///      that is currently open, nothing happens.
///
///  2)  If this routine is used to close a HANDLE not associated
///      with a DAF, an error is signaled by a routine in the call tree
///      of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  Because DAFAH and its entry points must keep track of what
///  files are open at any given time, it is important that DAF
///  files be closed only with DAFCLS, to prevent the remaining
///  DAF routines from failing, sometimes mysteriously.
///
///  Note that when a file is opened more than once for read access,
///  DAFOPR returns the same handle each time it is re-opened.
///  Each time the file is closed, DAFCLS checks to see if any other
///  claims on the file are still active before physically closing
///  the file.
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
///
///  1) In the following code fragment, the arrays in a file are
///     examined in order to determine whether the file contains
///     any arrays whose names begin with the word TEST.
///     The complete names for these arrays are printed to
///     the screen. The file is closed at the end of the search.
///
///        CALL DAFOPR ( FNAME, HANDLE )
///        CALL DAFBFS ( HANDLE )
///        CALL DAFFNA ( FOUND  )
///
///        DO WHILE ( FOUND )
///           CALL DAFGN ( NAME )
///
///           IF ( NAME(1:5) .EQ. 'TEST ' ) THEN
///              WRITE (*,*) NAME
///           END IF
///
///           CALL DAFFNA ( FOUND )
///        END DO
///
///        CALL DAFCLS ( HANDLE )
///
///     Note that if the file has been opened already by a DAF routine
///     at some other place in the calling program, it remains open.
///     This makes it possible to examine files that have been opened
///     for use by other modules without interfering with the operation
///     of those routines.
///
///
///  2) Use a simple routine to output the double precision and integer
///     values stored in an SPK's segments descriptors. This function
///     opens a DAF for read, performs a forwards search for the DAF
///     arrays, prints segments description for each array found, then
///     closes the DAF.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        de421.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFCLS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Define the summary parameters appropriate
///     C     for an SPK file.
///     C
///           INTEGER               MAXSUM
///           PARAMETER           ( MAXSUM = 125 )
///
///           INTEGER               ND
///           PARAMETER           ( ND = 2 )
///
///           INTEGER               NI
///           PARAMETER           ( NI = 6 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(32)        KERNEL
///
///           DOUBLE PRECISION      DC     ( ND     )
///           DOUBLE PRECISION      SUM    ( MAXSUM )
///
///           INTEGER               HANDLE
///           INTEGER               IC     ( NI     )
///
///           LOGICAL               FOUND
///
///     C
///     C     Open a DAF for read. Return a HANDLE referring to the
///     C     file.
///     C
///           KERNEL = 'de421.bsp'
///           CALL DAFOPR ( KERNEL, HANDLE )
///
///     C
///     C     Begin a forward search on the file.
///     C
///           CALL DAFBFS ( HANDLE )
///
///     C
///     C     Search until a DAF array is found.
///     C
///           CALL DAFFNA ( FOUND )
///
///     C
///     C     Loop while the search finds subsequent DAF arrays.
///     C
///           DO WHILE ( FOUND )
///
///              CALL DAFGS ( SUM )
///              CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///              WRITE(*,*)                'Doubles:', DC(1:ND)
///              WRITE(*, FMT='(A,6I9)' ) 'Integers:', IC(1:NI)
///
///     C
///     C        Check for another segment.
///     C
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        1        0        1        2      641   310404
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        2        0        1        2   310405   423048
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        3        0        1        2   423049   567372
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        4        0        1        2   567373   628976
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        5        0        1        2   628977   674740
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        6        0        1        2   674741   715224
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        7        0        1        2   715225   750428
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        8        0        1        2   750429   785632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        9        0        1        2   785633   820836
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:       10        0        1        2   820837   944040
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      301        3        1        2   944041  1521324
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      399        3        1        2  1521325  2098608
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      199        1        1        2  2098609  2098620
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      299        2        1        2  2098621  2098632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      499        4        1        2  2098633  2098644
///
///
///     Note, the final entries in the integer array contain the
///     segment start/end indexes. The output indicates the search
///     proceeded from the start of the file (low value index) towards
///     the end (high value index).
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Updated code example with IMPLICIT NONE, and declarations of
///         SUM variable and MAXSUM parameter.
///
///         Corrected minor typos in header.
///
/// -    SPICELIB Version 8.1.1, 10-OCT-2012 (EDW)
///
///         Added a functional code example to the $Examples section.
///
///         Removed the unneeded $Revisions section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         This routine was updated to accommodate changes to the
///         handle manager interface. See DAFAH's Revision section
///         for details.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.3, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 2.0.2, 25-FEB-1993 (JML)
///
///         A minor error in the $Particulars section of the header was
///         corrected. It formerly stated that a file could be open more
///         than once for read or write access instead of just read access.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 03-SEP-1991 (NJB) (WLT)
///
///         This routine was updated so that it now keeps current the set
///         of DAF handles returned by DAFHOF.
///
///         Upgraded to support file handle checking routines
///         DAFHOF and DAFSIH. DAFCLS now initializes the file
///         handle list if necessary, and removes from the list
///         the handles of files it closes.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafcls(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DAFCLS(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFCLS ( DAF, close )
pub fn DAFCLS(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFCLS", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Is this file even open? If so, decrement the number of links
    // to the file. If the number of links drops to zero, physically
    // close the file and remove it from the file buffer.
    //
    // If the file is not open: no harm, no foul.
    //
    save.FINDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX > 0) {
        save.FTLNK[save.FINDEX] = (save.FTLNK[save.FINDEX] - 1);

        if (save.FTLNK[save.FINDEX] == 0) {
            ZZDDHCLS(HANDLE, b"DAF", false, ctx)?;

            for I in save.FINDEX..=(save.NFT - 1) {
                save.FTHAN[I] = save.FTHAN[(I + 1)];
                save.FTLNK[I] = save.FTLNK[(I + 1)];
                save.FTND[I] = save.FTND[(I + 1)];
                save.FTNI[I] = save.FTNI[(I + 1)];
            }

            save.NFT = (save.NFT - 1);

            //
            // Delete the handle from our handle set.
            //
            REMOVI(HANDLE, save.FHLIST.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"DAFCLS", ctx)?;
    Ok(())
}

/// DAF, handle to summary format
///
/// Return the summary format associated with a handle.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF file.
///  ND         O   Number of double precision components in summaries.
///  NI         O   Number of integer components in summaries.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a previously opened
///           DAF file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ND,
///  NI       are the numbers of double precision and integer
///           components, respectively, in each array summary
///           in the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to any file that is
///      currently known to be open, the error SPICE(DAFNOSUCHHANDLE)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The summary format must be known in order to pack or unpack
///  an array summary. See the DAF Required Reading for a discussion
///  of summary formats.
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
///  1) Find the number of d.p. `words' in a DAF having an
///     arbitrary summary format.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        de421.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFHSF_EX1
///           IMPLICIT NONE
///
///     C
///     C     Count the number of d.p. words of data in a
///     C     DAF.  Exclude array summaries, reserved records,
///     C     the file record, and character records.
///     C
///           INTEGER               FILEN
///           PARAMETER           ( FILEN  = 128 )
///
///           INTEGER               MAXND
///           PARAMETER           ( MAXND  = 124 )
///
///           INTEGER               MAXNI
///           PARAMETER           ( MAXNI  = 250 )
///
///           INTEGER               MAXSUM
///           PARAMETER           ( MAXSUM = 125 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FILEN)     DAF
///
///           DOUBLE PRECISION      DC    ( MAXND  )
///           DOUBLE PRECISION      SUM   ( MAXSUM )
///
///           INTEGER               FA
///           INTEGER               HANDLE
///           INTEGER               IA
///           INTEGER               IC    ( MAXNI )
///           INTEGER               N
///           INTEGER               ND
///           INTEGER               NI
///
///           LOGICAL               FOUND
///
///
///           DAF = 'de421.bsp'
///
///     C
///     C     Open the DAF and find the summary format.
///     C
///           CALL DAFOPR ( DAF,    HANDLE )
///           CALL DAFHSF ( HANDLE, ND, NI )
///
///     C
///     C     Start a forward search and examine each array in
///     C     turn.
///     C
///           CALL DAFBFS ( HANDLE )
///           CALL DAFFNA ( FOUND  )
///
///           N = 0
///           DO WHILE ( FOUND )
///
///     C
///     C        Obtain the array summary, unpack it, and get
///     C        the initial and final array addresses from
///     C        the integer descriptor component.
///     C
///              CALL DAFGS ( SUM )
///              CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///              IA  =  IC ( NI - 1 )
///              FA  =  IC ( NI     )
///
///              N   =  FA - IA + 1 + N
///
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///           WRITE (*,*) 'Number of d.p. words is ', N
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Number of d.p. words is      2098004
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Changed example
///         code to hardcode the DAF file used as input.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.4, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.3, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.2, 03-SEP-1990 (NJB)
///
///         Example added to the $Examples section.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafhsf(
    ctx: &mut SpiceContext,
    handle: i32,
    nd: &mut i32,
    ni: &mut i32,
) -> crate::Result<()> {
    DAFHSF(handle, nd, ni, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFHSF ( DAF, handle to summary format )
pub fn DAFHSF(
    HANDLE: i32,
    ND: &mut i32,
    NI: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFHSF", ctx)?;
    }

    save.FINDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX > 0) {
        *ND = save.FTND[save.FINDEX];
        *NI = save.FTNI[save.FINDEX];
    } else {
        SETMSG(b"There is no DAF open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DAFNOSUCHHANDLE)", ctx)?;
    }

    CHKOUT(b"DAFHSF", ctx)?;
    Ok(())
}

/// DAF, handle to logical unit
///
/// Return the logical unit associated with a handle.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF file.
///  UNIT       O   Corresponding logical unit.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a previously opened
///           DAF file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UNIT     is the Fortran logical unit to which the file is
///           connected.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any error occurs while attempting to fetch a logical unit,
///      the error is signaled by a routine in the call tree of this
///      routine. The value of UNIT in this case is undefined.
/// ```
///
/// # Particulars
///
/// ```text
///  The best reason for knowing the logical unit to which a DAF
///  is connected is to read or write from the records reserved in a
///  file. Since these records are by definition invisible to the DAF
///  routines, you must read and write them directly.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the first reserved record in
///  a newly created DAF is used to store the name and address
///  of the person who created it.
///
///     FTYPE = 'TEST'
///     CALL DAFONW ( FNAME, FTYPE, 3, 6, IFNAME, 5, HANDLE )
///     CALL DAFHLU ( HANDLE, UNIT )
///
///     WRITE (UNIT,REC=2) 'Ellis Wyatt, JPL ',
///    .                   '4800 Oak Grove Drive ',
///    .                   'Room 301-125A ',
///    .                   'Pasadena, CA 91109'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine may only be used to retrieve logical units
///      for DAFs loaded or created using the interfaces available
///      in this entry point umbrella. Using this entry point to
///      retrieve units for files not loaded through these interfaces
///      may result in unexpected behavior.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.3, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
///         Changed the example to use the new entry point DAFONW.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         Successfully invoking this module has the side effect of
///         locking UNIT to HANDLE. This 'lock' guarantees until
///         HANDLE is closed (or unlocked) that the file associated
///         with HANDLE is always open and attached to logical unit
///         UNIT. To unlock a handle without closing the file, use
///         ZZDDHUNL, an entry point in the handle manager umbrella,
///         ZZDDHMAN.
///
///         The system can lock at most UTSIZE-SCRUNT-RSVUNT
///         simultaneously (see the include file 'zzddhman.inc' for
///         specific values of these parameters), but unnecessarily
///         locking handles to their logical units may cause performance
///         degradation. The handle manager will have less logical
///         units to utilize when disconnecting and reconnecting
///         loaded files.
///
/// -    Beta Version 1.1.0, 1-NOV-1989 (RET)
///
///         DAFHLU now only checks in and checks out if the one exception
///         occurs. The purpose of this change was to help speed up a
///         routine that gets called constantly by higher level DAF
///         routines.
/// ```
pub fn dafhlu(ctx: &mut SpiceContext, handle: i32, unit: &mut i32) -> crate::Result<()> {
    DAFHLU(handle, unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFHLU ( DAF, handle to logical unit )
pub fn DAFHLU(HANDLE: i32, UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFHLU", ctx)?;
    }

    ZZDDHHLU(HANDLE, b"DAF", true, UNIT, ctx)?;

    CHKOUT(b"DAFHLU", ctx)?;
    Ok(())
}

/// DAF, logical unit to handle
///
/// Return the handle associated with a logical unit.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Logical unit connected to a DAF.
///  HANDLE     O   Corresponding DAF file handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the logical unit to which a DAF has been
///           connected after it has been opened.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle associated with the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified unit is not connected to any file that is
///      currently loaded as a DAF, the error SPICE(DAFNOSUCHUNIT)
///      is signaled. The value of HANDLE returned is undefined in
///      this case.
/// ```
///
/// # Particulars
///
/// ```text
///  It is unlikely, but possible, that a calling program would know
///  the logical unit to which a file is connected without knowing the
///  handle associated with the file. DAFLUH is provided mostly for
///  completeness.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the handle associated with
///  a DAF is retrieved using the logical unit to which the
///  file is connected. The handle is then used to determine the
///  name of the file.
///
///     CALL DAFLUH ( UNIT,   HANDLE )
///     CALL DAFHFN ( HANDLE, FNAME )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine may only be used to retrieve handles for logical
///      units connected to DAFs loaded or created using the interfaces
///      available in this entry point umbrella. Using this entry point
///      to retrieve handles for files not loaded through these
///      interfaces may result in unexpected behavior.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.3, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafluh(ctx: &mut SpiceContext, unit: i32, handle: &mut i32) -> crate::Result<()> {
    DAFLUH(unit, handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFLUH ( DAF, logical unit to handle )
pub fn DAFLUH(UNIT: i32, HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFLUH", ctx)?;
    }

    ZZDDHLUH(UNIT, HANDLE, &mut save.FOUND, ctx)?;

    if !save.FOUND {
        *HANDLE = 0;
        SETMSG(b"There is no file open with unit = #", ctx);
        ERRINT(b"#", UNIT, ctx);
        SIGERR(b"SPICE(DAFNOSUCHUNIT)", ctx)?;
        CHKOUT(b"DAFLUH", ctx)?;
        return Ok(());
    }

    //
    // Now make certain that the HANDLE is associated with a DAF.
    //
    ZZDDHNFO(
        *HANDLE,
        &mut save.DAFNAM,
        &mut save.IARC,
        &mut save.IBFF,
        &mut save.IAMH,
        &mut save.FOUND,
        ctx,
    )?;

    if (save.IARC != DAF) {
        *HANDLE = 0;
        SETMSG(b"The file, \'#\', connected to unit # is not a DAF.", ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", UNIT, ctx);
        SIGERR(b"SPICE(DAFNOSUCHUNIT)", ctx)?;
        CHKOUT(b"DAFLUH", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DAFLUH", ctx)?;
    Ok(())
}

/// DAF, handle to file name
///
/// Return the name of the file associated with a handle.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF file.
///  FNAME      O   Corresponding file name.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a previously opened
///           DAF file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FNAME    is the name of the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to any file that is
///      currently known to be loaded as a DAF, the error
///      SPICE(DAFNOSUCHHANDLE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  It may be desirable to recover the names of one or more DAF
///  files in a different part of the program from the one in which
///  they were opened. Note that the names returned by DAFHFN may
///  not be identical to the names used to open the files. Under
///  most operating systems, a particular file can be accessed using
///  many different names. DAFHFN returns one of them.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the name of a DAF is
///  recovered using the handle associated with the file.
///
///     CALL DAFOPR ( 'sample.DAF', HANDLE )
///      .
///      .
///
///     CALL DAFHFN ( HANDLE, FNAME )
///
///  Depending on the circumstances (operating system, compiler,
///  default directory) the value of FNAME might resemble any of
///  the following:
///
///     'USER$DISK:[WYATT.IMAGES]SAMPLE.DAF;4'
///
///     '/wyatt/images/sample.DAF'
///
///     'A:\IMAGES\SAMPLE.DAF'
///
///  On the other hand, it might not.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine may only be used to retrieve the names of DAFs
///      loaded or created using the interfaces available in this entry
///      point umbrella. Using this entry point to retrieve names for
///      files not loaded through these interfaces may result in
///      unexpected behavior.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.1.1, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.1.0, 25-FEB-1993 (JML)
///
///         IOSTAT is checked after the INQUIRE statement.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafhfn(ctx: &mut SpiceContext, handle: i32, fname: &mut str) -> crate::Result<()> {
    DAFHFN(
        handle,
        fstr::StrBytes::new(fname).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFHFN ( DAF, handle to file name )
pub fn DAFHFN(HANDLE: i32, FNAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFHFN", ctx)?;
    }

    ZZDDHNFO(
        HANDLE,
        &mut save.DAFNAM,
        &mut save.IARC,
        &mut save.IBFF,
        &mut save.IAMH,
        &mut save.FOUND,
        ctx,
    )?;

    if (!save.FOUND || (save.IARC != DAF)) {
        SETMSG(b"There is no file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DAFNOSUCHHANDLE)", ctx)?;
        CHKOUT(b"DAFHFN", ctx)?;
        return Ok(());
    }

    fstr::assign(FNAME, &save.DAFNAM);

    CHKOUT(b"DAFHFN", ctx)?;
    Ok(())
}

/// DAF, file name to handle
///
/// Return handle associated with a file name.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAF file.
///  HANDLE     O   Corresponding DAF file handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a previously opened DAF file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle associated with the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified name does not specify any file currently
///      known to be loaded as a DAF, the error SPICE(DAFNOSUCHFILE) is
///      signaled. The value of HANDLE is undefined in this case.
///
///  2)  If the file does not exist, an error is signaled by a routine
///      in the call tree of this routine. The value of HANDLE is
///      undefined in this case.
///
///  3)  If an I/O error is generated in the process of connecting the
///      specified name with a handle, the error is signaled by a
///      routine in the call tree of this routine. The value of
///      HANDLE is undefined in this case.
/// ```
///
/// # Particulars
///
/// ```text
///  It is sometimes easier to work with file names (which are
///  meaningful, and often predictable) than with file handles
///  (which are neither), especially in interactive situations.
///  However, nearly every DAF routines requires that you use file
///  handles to refer to files. DAFFNH is provided to bridge the gap
///  between the two representations.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the handle associated with a
///  DAF is recovered using the name of the file.
///
///     CALL DAFOPR ( 'sample.DAF', HANDLE )
///      .
///      .
///
///     CALL DAFFNH ( 'sample.DAF', HANDLE )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Only file names of DAFs loaded with interfaces present in
///      this entry point umbrella should be passed into this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.1, 10-OCT-2012 (EDW)
///
///         Eliminated unneeded $Revisions section.
///
///         Corrected ordering of header section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
/// -    SPICELIB Version 8.1.0, 02-APR-2002 (FST)
///
///         Fixed a bug, where an error was signaled but the call to
///         CHKOUT and the RETURN statement were omitted.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
///         In previous version of DAFAH, this module simply
///         performed an INQUIRE on FNAME and looked in the
///         file table for the logical unit returned.
///
///         The integration of the new handle manager interfaces
///         into this entry point has the possibility of increasing
///         the complexity of this routine when more than UTSIZE
///         files are loaded. Essentially, when given an arbitrary
///         name, a total of FTSIZE INQUIRE statements may be executed
///         to accurately connect FNAME with HANDLE. See ZZDDHFNH and
///         ZZDDHF2H for details.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.1, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 2.0.0, 25-FEB-1993 (JML)
///
///         The INQUIRE statement that checks if the file is open now also
///         checks that the file exists. Two new exceptions were added as
///         a result of this change.
///
///         A RETURN statement was added after the error signaled when
///         the file is not open.
///
///         The file name is checked to see if it is blank.
///
///         The file name string that is passed to the FORTRAN INQUIRE
///         statement has been chopped at the last non-blank character.
///
/// -    SPICELIB Version 1.1.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.1, 18-SEP-1991 (HAN)
///
///         The $Revisions section was incorrectly named $Version. This has
///         been fixed.
///
/// -    SPICELIB Version 1.1.0, 05-NOV-1990 (HAN)
///
///         Call to CHKIN was corrected. The module was checking in
///         as 'DAFFHN'.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn daffnh(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DAFFNH(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFFNH ( DAF, file name to handle )
pub fn DAFFNH(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFFNH", ctx)?;
    }

    ZZDDHFNH(FNAME, HANDLE, &mut save.FOUND, ctx)?;

    if !save.FOUND {
        *HANDLE = 0;
        SETMSG(
            b"There is no file in the DAF table with file name = \'#\'",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DAFNOSUCHFILE)", ctx)?;
        CHKOUT(b"DAFFNH", ctx)?;
        return Ok(());
    }

    //
    // Now make certain that HANDLE is associated with a DAF.
    //
    ZZDDHNFO(
        *HANDLE,
        &mut save.DAFNAM,
        &mut save.IARC,
        &mut save.IBFF,
        &mut save.IAMH,
        &mut save.FOUND,
        ctx,
    )?;

    if (save.IARC != DAF) {
        *HANDLE = 0;
        SETMSG(b"The file, \'#\', is not a DAF.", ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DAFNOSUCHFILE)", ctx)?;
        CHKOUT(b"DAFFNH", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DAFFNH", ctx)?;
    Ok(())
}

/// DAF, handles of open files
///
/// Return a SPICE set containing the handles of all currently
/// open DAFS.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FHSET      O   A set containing handles of currently open DAFS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FHSET    is a SPICE set containing the file handles of
///           all currently open DAFs.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the set FHSET is not initialized, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If the set FHSET is too small to accommodate the set of
///      handles to be returned, an error is signaled by a routine in
///      the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows subroutines to test file handles for
///  validity before performing operations on them, such as
///  finding the name of the file designated by a handle. Many
///  DAF operations on handles cause errors to be signaled if
///  the handles are invalid.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Find out how may DAFs are open for writing.
///
///         C
///         C    Find out which DAFs are open.
///         C
///              CALL DAFHOF  ( FHSET )
///
///         C
///         C    Count the ones open for writing. These have
///         C    negative file handles.
///         C
///              COUNT = 0
///
///              DO I = 1, CARDC(FHSET)
///
///                 IF ( FHSET(I) .LT. 0 ) THEN
///                    COUNT = COUNT + 1
///                 END IF
///
///              END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1991 (NJB) (WLT)
/// ```
pub fn dafhof(ctx: &mut SpiceContext, fhset: &mut [i32]) -> crate::Result<()> {
    DAFHOF(fhset, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFHOF ( DAF, handles of open files )
pub fn DAFHOF(FHSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FHSET = DummyArrayMut::new(FHSET, LBCELL..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFHOF", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Just stuff our local list into the set.
    //
    COPYI(save.FHLIST.as_slice(), FHSET.as_slice_mut(), ctx)?;

    CHKOUT(b"DAFHOF", ctx)?;
    Ok(())
}

/// DAF, signal invalid handles
///
/// Signal an error if a DAF file handle does not designate a DAF
/// that is open for a specified type of access.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
/// * [ERROR](crate::required_reading::error)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   HANDLE to be validated.
///  ACCESS     I   String indicating access type.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a DAF handle to validate. For HANDLE to be
///           considered valid, it must specify a DAF that is
///           open for the type of access specified by the input
///           argument ACCESS.
///
///
///  ACCESS   is a string indicating the type of access that
///           the DAF specified by the input argument HANDLE
///           must be open for. The values of ACCESS may be
///
///
///              'READ'      File must be open for read access
///                          by DAF routines. All open DAFs
///                          may be read.
///
///              'WRITE'     File must be open for write access
///                          by DAF routines.
///
///                          Note that files open for write
///                          access may be read as well as
///                          written.
///
///
///           Leading and trailing blanks in ACCESS are ignored,
///           and case is not significant.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input argument ACCESS has an unrecognized value,
///      the error SPICE(INVALIDOPTION) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine signals the error SPICE(DAFINVALIDACCESS) if the
///  DAF designated by the input argument HANDLE is not open
///  for the specified type of access. If HANDLE does not designate
///  an open DAF, the error SPICE(DAFNOSUCHHANDLE) is signaled.
///
///  This routine allows subroutines to test file handles for
///  validity before performing operations on them, such as
///  finding the name of the file designated by a handle. Many
///  DAF operations on handles may cause unpredictable program
///  behavior if the handles are invalid. This routine should
///  be used in situations where the appropriate action to take upon
///  determining that a handle is invalid is to signal an error.
///  DAFSIH centralizes the error response for this type of error in a
///  single routine.
///
///  In cases where it is necessary to determine the validity of a
///  file handle, but it is not an error for the handle to refer
///  to a closed file, the entry point DAFHOF should be used instead
///  of DAFSIH.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Add data to a DAF specified by a file handle. Signal an
///      error if the file is not open for writing. Check the
///      SPICELIB error status function FAILED after calling
///      DAFSIH, so that the routine will return if DAFSIH
///      signaled an error (we're presuming that this code
///      fragment would be used in a subroutine).
///
///         C
///         C     Check that HANDLE is valid, then add data to the
///         C     file specified by HANDLE.
///         C
///               CALL DAFSIH  (  HANDLE, 'WRITE' )
///
///               IF ( FAILED() ) THEN
///                  RETURN
///               END IF
///
///               CALL DAFBNA (  HANDLE,  SUM,    NAME )
///               CALL DAFADA (  DATA,    N            )
///               CALL DAFENA
///
///  2)  Find the size of an array in a DAF specified by a file
///      handle. Signal an error if the file is not open for reading.
///
///         C
///         C     Check that HANDLE is valid, then obtain the
///         C     current array summary and compute the size of
///         C     the current array.
///         C
///               CALL DAFSIH  ( HANDLE, 'READ' )
///
///               IF ( FAILED() ) THEN
///                  RETURN
///               END IF
///
///         C
///         C     Obtain the summary format, then the integer and d.p.
///         C     components of the summary. Finally, compute the
///         C     array length.
///         C
///               CALL DAFHSF (  HANDLE, ND, NI          )
///               CALL DAFGS  (  SUMMRY                  )
///               CALL DAFUS  (  SUMMRY, ND, NI, DC, IC  )
///
///               IA      =  IC( NI - 1 )
///               FA      =  IC( NI     )
///               LENGTH  =  FA  -  IA  +  1
///
///  3)  Make sure that a file handle designates an open DAF. Signal
///      an error if it does not.
///
///      Note that if a DAF is open at all, read access is allowed.
///
///               CALL DAFSIH ( HANDLE, 'READ' )
///
///               IF ( FAILED() ) THEN
///                  RETURN
///               END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.2, 25-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.0.1, 10-OCT-2012 (EDW)
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 8.0.0, 13-NOV-2001 (FST)
///
///         This routine was updated to utilize the new handle manager
///         software to manage binary file formats and consolidated
///         I/O code.
///
/// -    SPICELIB Version 7.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 7.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 7.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 7.0.1, 17-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.2.1, 29-SEP-1993 (KRG)
///
///         Removed references to specific DAF file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.2.0, 25-FEB-1993 (JML)
///
///         IOSTAT is now checked after the INQUIRE statement.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 03-SEP-1991 (NJB) (WLT)
/// ```
pub fn dafsih(ctx: &mut SpiceContext, handle: i32, access: &str) -> crate::Result<()> {
    DAFSIH(handle, access.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFSIH ( DAF, signal invalid handles )
pub fn DAFSIH(HANDLE: i32, ACCESS: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFSIH", ctx)?;
    }

    //
    // Initialize the handle list, if necessary.
    //
    if save.FIRST {
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;
        save.FIRST = false;
    }

    //
    // Get an upper case, left-justified copy of ACCESS.
    //
    LJUST(ACCESS, &mut save.ACC);
    UCASE(&save.ACC.to_vec(), &mut save.ACC, ctx);

    //
    // Make sure we recognize the access type specified by the caller.
    //
    if (fstr::ne(&save.ACC, b"READ") && fstr::ne(&save.ACC, b"WRITE")) {
        SETMSG(b"Unrecognized access type.  Type was #. ", ctx);
        ERRCH(b"#", ACCESS, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"DAFSIH", ctx)?;
        return Ok(());
    }

    //
    // Retrieve information about this HANDLE.
    //
    ZZDDHNFO(
        HANDLE,
        &mut save.DAFNAM,
        &mut save.IARC,
        &mut save.IBFF,
        &mut save.IAMH,
        &mut save.FOUND,
        ctx,
    )?;

    //
    // See whether the input handle is in our list at all.  It's
    // unlawful for the handle to be absent.  All open DAFs are
    // readable, so in the case that ACC is 'READ', we're done if
    // the DAF is open.
    //
    if (!save.FOUND || !ELEMI(HANDLE, save.FHLIST.as_slice(), ctx)?) {
        SETMSG(b"There is no file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DAFNOSUCHHANDLE)", ctx)?;
        CHKOUT(b"DAFSIH", ctx)?;
        return Ok(());

    //
    // If the access type is 'WRITE', the DAF must be open for writing.
    // This is not the case if the value of IAMH returned from the handle
    // manager is not READ.
    //
    } else if (fstr::eq(&save.ACC, b"WRITE") && (save.IAMH == READ)) {
        SETMSG(b"DAF not open for write.  Handle = #, file = \'#\'", ctx);
        ERRINT(b"#", HANDLE, ctx);
        ERRCH(b"#", &save.DAFNAM, ctx);
        SIGERR(b"SPICE(DAFINVALIDACCESS)", ctx)?;
        CHKOUT(b"DAFSIH", ctx)?;
        return Ok(());
    }

    //
    // The DAF's handle is o.k.
    //
    CHKOUT(b"DAFSIH", ctx)?;
    Ok(())
}
