//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const LBCELL: i32 = -5;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const ACCLEN: i32 = 10;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const LBPOOL: i32 = -5;
const MAXPC: i32 = 126;
const MINPC: i32 = 32;
const TYPLEN: i32 = 4;
const READ: i32 = 1;
const WRITE: i32 = (READ + 1);
const RRCIDX: i32 = 1;
const RCHIDX: i32 = (RRCIDX + 1);
const CRCIDX: i32 = (RCHIDX + 1);
const CCHIDX: i32 = (CRCIDX + 1);
const FREIDX: i32 = (CCHIDX + 1);
const LLABAS: i32 = FREIDX;
const LRCBAS: i32 = (LLABAS + 3);
const LWDBAS: i32 = (LRCBAS + 3);
const SUMSIZ: i32 = (LWDBAS + 3);
const FWDLOC: i32 = 2;
const RNGBAS: i32 = 2;
const BEGDSC: i32 = 9;
const FMTLEN: i32 = 8;
const TAILEN: i32 = 932;

struct SaveVars {
    FTHAN: ActualArray<i32>,
    FTACC: ActualArray<i32>,
    FTLNK: ActualArray<i32>,
    FTSUM: ActualArray2D<i32>,
    POOL: ActualArray2D<i32>,
    FTHEAD: i32,
    NEXT: StackArray<i32, 3>,
    PREV: StackArray<i32, 3>,
    ACC: Vec<u8>,
    FORMAT: Vec<u8>,
    IDWORD: Vec<u8>,
    LOCDAS: Vec<u8>,
    LOCIFN: Vec<u8>,
    LOCFMT: Vec<u8>,
    TAIL: Vec<u8>,
    TTYPE: Vec<u8>,
    CURTYP: i32,
    DIRREC: StackArray<i32, 256>,
    DSCTYP: i32,
    ENDREC: i32,
    FNB: i32,
    FHLIST: ActualArray<i32>,
    FINDEX: i32,
    INQSTA: i32,
    IOSTAT: i32,
    LAST: i32,
    LDREC: StackArray<i32, 3>,
    LDRMAX: i32,
    LOC: i32,
    LOCCCH: i32,
    LOCCRC: i32,
    LOCRRC: i32,
    LOCRCH: i32,
    MAXADR: i32,
    NEW: i32,
    NREC: i32,
    NUMBER: i32,
    NW: StackArray<i32, 3>,
    NXTDIR: i32,
    NXTREC: i32,
    POS: i32,
    PRVTYP: i32,
    FOUND: bool,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTACC = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTLNK = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTSUM = ActualArray2D::<i32>::new(1..=SUMSIZ, 1..=FTSIZE);
        let mut POOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=FTSIZE);
        let mut FTHEAD: i32 = 0;
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut PREV = StackArray::<i32, 3>::new(1..=3);
        let mut ACC = vec![b' '; ACCLEN as usize];
        let mut FORMAT = vec![b' '; FMTLEN as usize];
        let mut IDWORD = vec![b' '; IDWLEN as usize];
        let mut LOCDAS = vec![b' '; FILEN as usize];
        let mut LOCIFN = vec![b' '; IFNLEN as usize];
        let mut LOCFMT = vec![b' '; FMTLEN as usize];
        let mut TAIL = vec![b' '; TAILEN as usize];
        let mut TTYPE = vec![b' '; TYPLEN as usize];
        let mut CURTYP: i32 = 0;
        let mut DIRREC = StackArray::<i32, 256>::new(1..=NWI);
        let mut DSCTYP: i32 = 0;
        let mut ENDREC: i32 = 0;
        let mut FNB: i32 = 0;
        let mut FHLIST = ActualArray::<i32>::new(LBCELL..=FTSIZE);
        let mut FINDEX: i32 = 0;
        let mut INQSTA: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut LAST: i32 = 0;
        let mut LDREC = StackArray::<i32, 3>::new(1..=3);
        let mut LDRMAX: i32 = 0;
        let mut LOC: i32 = 0;
        let mut LOCCCH: i32 = 0;
        let mut LOCCRC: i32 = 0;
        let mut LOCRRC: i32 = 0;
        let mut LOCRCH: i32 = 0;
        let mut MAXADR: i32 = 0;
        let mut NEW: i32 = 0;
        let mut NREC: i32 = 0;
        let mut NUMBER: i32 = 0;
        let mut NW = StackArray::<i32, 3>::new(1..=3);
        let mut NXTDIR: i32 = 0;
        let mut NXTREC: i32 = 0;
        let mut POS: i32 = 0;
        let mut PRVTYP: i32 = 0;
        let mut FOUND: bool = false;
        let mut PASS1: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), FTSIZE as usize))
                .chain([]);

            FTACC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), FTSIZE as usize))
                .chain([]);

            FTHAN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(-1), FTSIZE as usize))
                .chain([]);

            FTLNK
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        FTHEAD = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2)].into_iter();
            PREV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NWC), Val::I(NWD), Val::I(NWI)].into_iter();
            NW.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FTHAN,
            FTACC,
            FTLNK,
            FTSUM,
            POOL,
            FTHEAD,
            NEXT,
            PREV,
            ACC,
            FORMAT,
            IDWORD,
            LOCDAS,
            LOCIFN,
            LOCFMT,
            TAIL,
            TTYPE,
            CURTYP,
            DIRREC,
            DSCTYP,
            ENDREC,
            FNB,
            FHLIST,
            FINDEX,
            INQSTA,
            IOSTAT,
            LAST,
            LDREC,
            LDRMAX,
            LOC,
            LOCCCH,
            LOCCRC,
            LOCRRC,
            LOCRCH,
            MAXADR,
            NEW,
            NREC,
            NUMBER,
            NW,
            NXTDIR,
            NXTREC,
            POS,
            PRVTYP,
            FOUND,
            PASS1,
        }
    }
}

/// DAS, file manager
///
/// Manage open DAS files.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  FNAME     I-O  OPR, OPW, ONW, OPN (Obsolete), HFN, FNH
///  FTYPE      I   ONW
///  IFNAME     I   ONW, OPN (Obsolete)
///  HANDLE    I-O  OPR, OPW, ONW, OPN (Obsolete), OPS, LLC, HLU, LUH,
///                 HFN, FNH, HAM, SIH
///  UNIT      I-O  HLU, LUH
///  FREE      I-O  HFS, UFS
///  LASTLA    I-O  HFS, UFS
///  LASTRC    I-O  HFS, UFS
///  LASTWD    I-O  HFS, UFS
///  NRESVR     O   HFS
///  NRESVC     O   HFS
///  NCOMR      O   HFS
///  NCOMC      O   HFS
///  FHSET      O   HOF
///  ACCESS    I-O  SIH, HAM
///  RECL       P   OPR, OPW, ONW, OPN (Obsolete)
///  FTSIZE     P   OPR, OPW, ONW, OPN (Obsolete), LLC, HLU, LUH, HFN,
///                 FNH
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    on input is the name of a DAS file to be opened, or
///           the name of a DAS file about which some information
///           (handle, logical unit) is requested.
///
///  FTYPE    on input is a code for the type of data that is
///           contained in the DAS file. This code has no meaning or
///           interpretation at the level of the DAS file
///           architecture, but is provided as a convenience for
///           higher level software. The maximum length for the file
///           type is four (4) characters. If the input string is
///           longer than four characters, the first nonblank
///           character and its three, at most, immediate successors
///           will be used as the file type. The file type may not
///           contain nonprinting characters, and it IS case
///           sensitive.
///
///  IFNAME   is the internal file name for a DAS file to be
///           created.
///
///  HANDLE   on input is the handle of a DAS file about which some
///           information (file name, logical unit) is requested,
///           or the handle of a DAS file to be closed.
///
///  UNIT     on input is the logical unit connected to a DAS file
///           about which some information (file name, handle) is
///           requested.
///
///  FREE     is the Fortran record number of the first free record
///           in a specified DAS file.
///
///  LASTLA   is an array containing the highest current logical
///           addresses, in the specified DAS file, of data of
///           character, double precision, and integer types, in
///           that order.
///
///  LASTRC   is an array containing the Fortran record numbers, in
///           the specified DAS file, of the directory records
///           containing the current last descriptors of clusters
///           of character, double precision, and integer data
///           records, in that order.
///
///  LASTWD   is an array containing the word positions, in the
///           specified DAS file, of the current last descriptors
///           of clusters of character, double precision, and
///           integer data records, in that order.
///
///  ACCESS   is the type of access for which a DAS file is open.
///           The values of ACCESS may be
///
///              'READ'
///              'WRITE'
///
///           Leading and trailing blanks are ignored, and case
///           is not significant.
///
///           DAS files that are open for writing may also be read.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FNAME    on output is the name of a DAS file for which
///           the corresponding handle or logical unit has been
///           supplied.
///
///  HANDLE   on output is the handle of a DAS file for which
///           the corresponding file name or logical unit has been
///           supplied.
///
///  UNIT     on output is the logical unit connected to a DAS file
///           for which the corresponding file name or handle has
///           been supplied.
///
///  FREE     is the Fortran record number of the first free record
///           in a specified DAS file.
///
///  LASTLA   is an array containing the highest current logical
///           addresses, in the specified DAS file, of data of
///           character, double precision, and integer types, in
///           that order.
///
///  LASTRC   is an array containing the Fortran record numbers, in
///           the specified DAS file, of the directory records
///           containing the current last descriptors of clusters
///           of character, double precision, and integer data
///           records, in that order.
///
///  LASTWD   is an array containing the word positions, in the
///           specified DAS file, of the current last descriptors
///           of clusters of character, double precision, and
///           integer data records, in that order.
///
///  NRESVR   is the number of reserved records in a specified DAS
///           file.
///
///  NRESVC   is the number of characters in use in the reserved
///           record area of a specified DAS file.
///
///  NCOMR    is the number of comment records in a specified DAS
///           file.
///
///  NCOMC    is the number of characters in use in the comment area
///           of a specified DAS file.
///
///  FHSET    is a SPICE set containing the handles of the
///           currently open DAS files.
/// ```
///
/// # Parameters
///
/// ```text
///  RECL     is the record length of a DAS file. Each record
///           must be large enough to hold the greatest of NWI
///           integers, NWD double precision numbers, or NWC
///           characters, whichever is greater. The units in which
///           the record length must be specified vary from
///           environment to environment. For example, VAX Fortran
///           requires record lengths to be specified in longwords,
///           where two longwords equal one double precision
///           number.
///
///  FTSIZE   is the maximum number of DAS files that a user can
///           have open simultaneously. This includes any files used
///           by the DAS system when closing files opened with write
///           access. Currently, DASCLS (via DASSDR) opens a scratch
///           DAS file using DASOPS to segregate (sort by data
///           type) the records in the DAS file being closed.
///           Segregating the data by type improves the speed of
///           access to the data.
///
///           In order to avoid the possibility of overflowing the
///           DAS file table we recommend, when at least one DAS
///           file is open with write access, that users of this
///           software limit themselves to at most FTSIZE - 2  other
///           open DAS files. If no files are to be open with write
///           access, then users may open FTSIZE files with no
///           possibility of overflowing the DAS file table.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DASFM is called directly, the error SPICE(BOGUSENTRY)
///      is signaled.
///
///  2)  See entry points DASOPR, DASOPW, DASONW, DASOPN, DASOPS,
///      DASLLC, DASHFS, DASUFS, DASHLU, DASLUH, DASHFN, DASFNH,
///      DASHOF, and DASSIH for exceptions specific to those entry
///      points.
/// ```
///
/// # Files
///
/// ```text
///  This set of routines is intended to support the creation,
///  updating, and reading of Fortran direct access files that
///  conform to the DAS file format. This format is described in
///  detail in the DAS Required Reading.
///
///  See FTSIZE in the $Parameters section for a description of a
///  potential problem with overflowing the DAS file table when at
///  least one DAS file is opened with write access.
/// ```
///
/// # Particulars
///
/// ```text
///  As of the N0066 version of the SPICE Toolkit, DASFM uses the
///  SPICE handle manager subsystem and makes use of run-time
///  translation.
///
///  DASFM serves as an umbrella, allowing data to be shared by its
///  entry points:
///
///     DASOPR         Open for read.
///     DASOPW         Open for write.
///     DASONW         Open new.
///     DASOPN         Open new. (Obsolete: Use DASONW instead.)
///     DASOPS         Open as scratch file.
///
///     DASLLC         Low-level close.
///
///     DASHFS         Handle to file summary.
///     DASUFS         Update file summary.
///
///     DASHLU         Handle to logical unit.
///     DASLUH         Logical to handle.
///
///     DASHFN         Handle to name.
///     DASFNH         File name to handle.
///
///     DASHAM         Handle to access method.
///
///     DASHOF         Handles of open files.
///     DASSIH         Signal invalid handles.
///
///
///  Before a DAS file  can be used, it must be opened. Entry points
///  DASOPR and DASOPW provide the only means for opening an
///  existing DAS file.
///
///  Several files may be opened for use simultaneously. (This makes
///  it convenient to combine data from several files to produce a
///  single result, or to route subsets of data from a single source
///  to multiple DAS files.) As each DAS file is opened, it is
///  assigned a file handle, which is used to keep track of the file
///  internally, and which is used by the calling program to refer to
///  the file in all subsequent calls to DAS routines.
///
///  DAS files may be opened for either read or write access. Files
///  open for read access may not be changed in any way. Files opened
///  for write access may be both read from and written to.
///
///  DASONW is used to open a new DAS file. This routine extends the
///  functionality of DASOPN by providing a mechanism for associating a
///  type with the data in the DAS file. The use of this entry over
///  DASOPN is highly recommended.
///
///  Since the only reason for creating a new file is to write
///  something in it, all new files are opened for write access.
///
///  Entry point DASOPN, for opening a new DAS file, has been rendered
///  obsolete by the new entry point DASONW. The entry point DASOPN
///  will continue to be supported for purposes of backward
///  compatibility, but its use in new software development is strongly
///  discouraged.
///
///  Entry point DASOPS creates a new scratch DAS file. As with new
///  permanent files, these files are opened for write access.  DAS
///  files opened by DASOPS are automatically deleted when they are
///  closed.
///
///  Entry point DASLLC is used by DASCLS ( DAS, close file ) to close
///  an open DAS file and update DASFM's bookkeeping information
///  accordingly. DASCLS provides the only official means of closing
///  a DAS file that is currently open. Closing a DAS file any other
///  way (for example, by determining its logical unit and using the
///  Fortran CLOSE statement directly) may affect your calling program
///  in mysterious ways. Normally, DASLLC should not be called by
///  non-SPICELIB routines; these should call DASCLS instead.
///
///  Entry point DASHFS allows you to obtain a file summary for any
///  DAS file that is currently open, without calling DASRFR to
///  re-read the file record. Entry point DASUFS can be used to
///  update a file summary at run-time. Normally, there is no
///  need for routines outside of SPICELIB to modify a DAS file's
///  summary.
///
///  Entry point DASHAM allows you to determine which access method
///  a DAS file has been opened for.
///
///  Entry point DASHOF allows you to determine which DAS files are
///  open at any time. In particular, you can use DASHOF to determine
///  whether any file handle points to an open DAS file.
///
///  Entry point DASSIH signals errors when it is supplied with invalid
///  handles, so it serves to centralize error handling associated
///  with invalid handles.
///
///  The remaining entry points exist mainly to translate between
///  alternative representations of DAS files. There are three ways to
///  identify any open DAS file: by name, by handle, and by logical
///  unit. Given any one of these, you may use these entry points to
///  find the other two.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in entry points DASOPR, DASOPW, DASONW,
///  DASOPN (Obsolete), DASLLC, DASHFS, DASUFS, DASHLU, DASLUH,
///  DASHFN, DASFNH, DASHAM, DASHOF, and DASSIH for examples specific
///  to those entry points.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The value of parameter RECL may need to be changed when DASFM
///      and its entry points are ported to a new environment (CPU and
///      compiler).
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
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 8.0.1, 24-AUG-2021 (JDR)
///
///         Edited the header of the DASFM umbrella routine and all its
///         entry points.
///
///         Removed non-existent argument "SUM" from the umbrella's
///         $Brief_I/O section.
///
/// -    SPICELIB Version 8.0.0, 06-APR-2016 (NJB)
///
///         Corrected call to ZZDDHCLS in DASUFS.
///
///         26-FEB-2015 (NJB)
///
///            Now uses DAF/DAS handle manager subsystem.
///
///            FTSIZE is now 5000.
///
///            Corrected miscellaneous spelling errors in comments
///            throughout this file.
///
/// -    SPICELIB Version 7.24.0, 10-APR-2014 (NJB)
///
///         Added initializers for file table arrays. This was done
///         to suppress compiler warnings. Deleted declaration of
///         unused parameter BWDLOC.
///
///         Corrected header comments in entry point DASLLC: routine that
///         flushes written, buffered records is DASWBR, not DASWUR.
///
/// -    SPICELIB Version 7.23.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 7.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 7.21.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 7.20.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 7.19.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 7.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 7.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 7.16.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 7.15.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 7.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 7.13.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 7.12.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 7.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 7.10.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 7.9.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 7.8.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 7.7.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 7.6.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 7.5.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 7.4.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 7.3.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 7.2.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 7.1.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added to
///         entry points DASOPR and DASOPW.
///
///         Bug in code for constructing long error message in entry
///         point DASUFS was corrected.
///
/// -    SPICELIB Version 6.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 6.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 6.0.3, 24-APR-2003 (EDW)
///
///         Added MAC-OSX-F77 to the list of platforms
///         that require READONLY to read write protected
///         kernels.
///
/// -    SPICELIB Version 6.0.2, 21-FEB-2003 (NJB)
///
///         Corrected inline comment in DASLLC: determination of
///         whether file is open is done by searching the handle column of
///         the file table, not the unit column.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (NJB) (FST)
///
///         To accommodate future updates to the DAS system, including
///         integration with the handle manager and FTP validation
///         checks, the following entry points were modified:
///
///            DASONW, DASOPN
///
///         See their headers and code for the details of the changes.
///
///         Bug fix: removed local buffering of the DAS file ID word
///         and the internal file name, as this was causing DASWFR
///         to exhibit improper behavior.
///
///         Bug fix: missing call to CHKIN was added to an error
///         handling branch in entry point DASUFS. This call is
///         required because DASUFS uses discovery check-in.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 5.0.0, 05-APR-1998 (NJB)
///
///         Added references to the PC-LINUX environment. Repaired some
///         format errors involving placement of comment markers in
///         column 1.
///
/// -    SPICELIB Version 4.0.1, 19-DEC-1995 (NJB)
///
///         Added permuted index entry section.
///
/// -    SPICELIB Version 4.0.0, 31-AUG-1995 (NJB)
///
///         Changed argument list of the entry point DASONW. The input
///         argument NCOMR, which indicates the number of comment records
///         to reserve, was added to the argument list.
///
/// -    SPICELIB Version 3.1.0, 05-JAN-1995 (HAN)
///
///         Removed Sun Solaris environment since it is now the same
///         as the Sun OS 4.1.x environment.
///         Removed DEC Alpha/OpenVMS environment since it is now the same
///         as the VAX environment.
///         Entry points affected are: DASFM, DASOPR.
///
/// -    SPICELIB Version 3.0.0, 15-JUN-1994 (KRG)
///
///         Modified the umbrella routine DASFM to allow the inclusion of
///         a file type in the creation and manipulation of DAS files.
///
/// -    SPICELIB Version 2.0.0, 11-APR-1994 (HAN)
///
///         Updated module to include values for the Silicon Graphics/IRIX,
///         DEC Alpha-OSF/1, and Next/Absoft Fortran platforms. Entry
///         points affected are: DASFM, DASOPR.
///
/// -    SPICELIB Version 1.0.0, 15-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added to
///         entry points DASOPR and DASOPW.
///
///         Bug in code for constructing long error message in entry
///         point DASUFS was corrected.
///
///         Local variable DAS was renamed to DASFIL in DASSIH.
///
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (NJB) (FST)
///
///         Binary File Format Identification:
///
///         The file record now contains an 8 character string that
///         identifies the binary file format utilized by DAS files.
///         The purpose of this string's inclusion in the file record
///         is preparatory in nature, to accelerate the migration to
///         files that support the runtime translation update that
///         is scheduled.
///
///         FTP Validation:
///
///         The file record now contains a sequence of characters
///         commonly corrupted by improper FTP transfers. These
///         characters will be examined by the handle manager when
///         existing files are opened.
///
///         FTIDW and FTIFN have been removed from the elements of
///         the DAS file table. Their presence and use in DASUFS
///         was causing DASWFR difficulties in updating the internal
///         filename under situations where changes to the comment and
///         reserved record parameters in the file record were updated.
///         This change effects DASOPR, DASOPN, DASONW, DASOPW, and
///         DASUFS.
///
/// -    SPICELIB Version 3.0.0, 15-JUN-1994 (KRG)
///
///         Modified the umbrella routine DASFM to allow the inclusion of
///         a file type in the creation and manipulation of DAS files. In
///         particular, the following changes were made:
///
///            1) Added variable FTYPE to the SUBROUTINE declaration, and
///               added appropriate entries for this variable in the
///               $Brief_I/O and $Detailed_Input sections of the header.
///
///            2) Removed erroneous references to OPC from the $Brief_I/O
///               section.
///
///            3) Added a new entry point, DASONW, which will support the
///               ability to associate a data type with a new DAS file
///               when it is created. The addition of this new entry point
///               makes the entry point DASOPN obsolete.
///
///            4) Added a description of the new entry point DASONW to the
///               $Particulars section. Also added a statement that the
///               entry point DASOPN has been made obsolete by this new
///               entry point, and its use in new code development is
///               discouraged.
///
///            5) Added a new variable to the file table, FTIDW, which
///               will be used to store the ID words from successfully
///               opened DAS files. We need to maintain this information
///               when writing the file record, as we do not want to
///               modify the ID word in the file.
///
///            6) Removed the parameter DASID as it is no longer needed.
///
///            7) Added new variables TARCH and TTYPE for temporary
///               storage of the file architecture and type. Also added a
///               new variable FNB for storing the position of the first
///               nonblank in a string.
///
///            8) Added new parameters:
///
///                  ARCLEN The maximum length of a file architecture
///                  TYPLEN The maximum length of a file type
///                  MAXPC  Decimal value for the upper limit of printable
///                         ASCII characters.
///                  MINPC  Decimal value for the lower limit of printable
///                         ASCII characters.
///
///            9) Modified entry points which open DAS files: OPR, OPW,
///               OPS, OPN, ONW to support the new file ID word format.
///
///           10) Made all occurrences of error message formatting of
///               filenames consistent. All filenames will be single
///               quoted in output error messages.
///
///           11) Added a test for a blank filename before the inquire
///               to obtain information about a file in the entry points:
///               DASOPR, DASOPW, DASONW, and DASOPN.
///
///           12) Modified the description of FTSIZE in the $Parameters
///               section to reflect the possibility of overflowing the
///               DAS file table when at least one DAS file had been
///               opened with write access.
///
///               The problem occurs when the file table is full, the
///               number of open DAS files equals FTSIZE, and at least one
///               of the open files was opened with write access. If an
///               attempt to close a file opened with write access is made
///               under these conditions, by calling DASCLS, it will fail.
///               DASCLS (via DASSDR) calls DASOPS to open a scratch DAS
///               file, but the scratch file CANNOT be opened because the
///               file table is full. If this occurs, close a file open
///               for read access, or restrict the number of open files
///               in use to be at most FTSIZE - 1 when there will be at
///               least one file opened with write access.
/// ```
pub fn dasfm(
    ctx: &mut SpiceContext,
    fname: &str,
    ftype: &str,
    ifname: &str,
    handle: i32,
    unit: i32,
    free: i32,
    lastla: &[i32; 3],
    lastrc: &[i32; 3],
    lastwd: &[i32; 3],
    nresvr: i32,
    nresvc: i32,
    ncomr: i32,
    ncomc: i32,
    fhset: &[i32],
    access: &str,
) -> crate::Result<()> {
    DASFM(
        fname.as_bytes(),
        ftype.as_bytes(),
        ifname.as_bytes(),
        handle,
        unit,
        free,
        lastla,
        lastrc,
        lastwd,
        nresvr,
        nresvc,
        ncomr,
        ncomc,
        fhset,
        access.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASFM ( DAS, file manager )
pub fn DASFM(
    FNAME: &[u8],
    FTYPE: &[u8],
    IFNAME: &[u8],
    HANDLE: i32,
    UNIT: i32,
    FREE: i32,
    LASTLA: &[i32],
    LASTRC: &[i32],
    LASTWD: &[i32],
    NRESVR: i32,
    NRESVC: i32,
    NCOMR: i32,
    NCOMC: i32,
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
    // Access method parameters:
    //

    //
    // File summary parameters:
    //
    //    A DAS file summary has the following structure:
    //
    //       +----------------------------------------+
    //       | <number of reserved records>           |
    //       +----------------------------------------+
    //       | <number of characters in r.r. area>    |
    //       +----------------------------------------+
    //       | <number of comment records>            |
    //       +----------------------------------------+
    //       | <number of characters in comment area> |
    //       +----------------------------------------+
    //       | <first free record number>             |
    //       +----------------------------------------+
    //       | <last character logical address>       |
    //       +----------------------------------------+
    //       | <last d.p. logical address>            |
    //       +----------------------------------------+
    //       | <last integer logical address>         |
    //       +----------------------------------------+
    //       | <last character descriptor record>     |
    //       +----------------------------------------+
    //       | <last d.p. descriptor record>          |
    //       +----------------------------------------+
    //       | <last integer descriptor record>       |
    //       +----------------------------------------+
    //       | <last character descriptor word>       |
    //       +----------------------------------------+
    //       | <last d.p. descriptor word>            |
    //       +----------------------------------------+
    //       | <last integer descriptor word>         |
    //       +----------------------------------------+
    //

    //
    // Base indices for:
    //
    //    -- last logical addresses
    //    -- records containing last descriptor for a given type
    //    -- word containing last descriptor for a given type
    //
    // The offset into the file summary for any of these items
    // is obtained by adding the appropriate data type parameter
    // (DP, INT, or CHAR) to the base index for the item.
    //

    //
    // Descriptor record pointer locations (within descriptor records):
    //

    //
    // Directory address range location parameters:
    //

    //
    // First descriptor position in descriptor record:
    //

    //
    // Length of the Binary File Format string:
    //

    //
    // The parameter TAILEN determines the tail length of a DAS file
    // record.  This is the number of bytes (characters) that
    // occupy the portion of the file record that follows the
    // integer holding the first free address.  For environments
    // with a 32 bit word length, 1 byte characters, and DAS
    // record sizes of 1024 bytes, we have:
    //
    //       8 bytes - IDWORD
    //      60 bytes - IFNAME
    //       4 bytes - NRESVR (32 bit integer)
    //       4 bytes - NRESVC (32 bit integer)
    //       4 bytes - NCOMR  (32 bit integer)
    //     + 4 bytes - NCOMC  (32 bit integer)
    //      ---------
    //      84 bytes - (All file records utilize this space.)
    //
    // So the size of the remaining portion (or tail) of the DAS
    // file record for computing environments as described above
    // would be:
    //
    //    1024 bytes - DAS record size
    //  -    8 bytes - DAS Binary File Format Word
    //  -   84 bytes - (from above)
    //   ------------
    //     932 bytes - DAS file record tail length
    //
    // Note: environments that do not have a 32 bit word length,
    // 1 byte characters, and a DAS record size of 1024 bytes, will
    // require the adjustment of this parameter.
    //

    //
    // Local variables
    //

    //
    // The file table consists of a set of arrays which serve as
    // `columns' of the table.  The sets of elements having the same
    // index in the arrays form the `rows' of the table.  Each column
    // contains a particular type of information; each row contains
    // all of the information pertaining to a particular DAS file.
    //
    // All column names in the file table begin with `FT'.  The
    // columns are:
    //
    //    HAN      Handle
    //    ACC      Access method
    //    LNK      Number of links
    //    SUM      File summary
    //
    // The rows of the file table are indexed by a doubly linked
    // list pool.  The pool contains an active list and a free list.
    // when a file is opened, a pointer to the file (the pointer
    // is called a `node').  it is placed at the head of the active
    // list; when a file is closed, the node in the active list that
    // pointed to the file is placed on the free list.
    //
    // NEXT is incremented each time a file is opened to become the
    // next file handle assigned.
    //

    //
    // FTHEAD is a pointer to the head of the active file list.
    //

    //
    // NEXT and PREV map the DAS data type codes to their
    // successors and predecessors, respectively.
    //

    //
    // Other local variables
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
        CHKIN(b"DASFM", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"DASFM", ctx)?;
    }

    Ok(())
}

/// DAS, open for read
///
/// Open a DAS file for reading.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAS file to be opened.
///  HANDLE     O   Handle assigned to the opened DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a DAS file to be opened with read
///           access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle that is  associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAS routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input filename is blank, the error
///      SPICE(BLANKFILENAME) is signaled.
///
///  2)  If the specified file does not exist, the error
///      SPICE(FILENOTFOUND) is signaled.
///
///  3)  If the specified file has already been opened for read
///      access, the handle already associated with the file is
///      returned.
///
///  4)  If the specified file has already been opened for write
///      access, the error SPICE(DASRWCONFLICT) is signaled.
///
///  5)  If the specified file has already been opened by a non-DAS
///      routine, the error SPICE(DASIMPROPOPEN) is signaled.
///
///  6)  If the specified file cannot be opened without exceeding
///      the maximum allowed number of open DAS files, the error
///      SPICE(DASFTFULL) is signaled.
///
///  7)  If the named file cannot be opened properly, an error is
///      signaled by a routine in the call tree of this routine.
///
///  8)  If the file record cannot be read, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  9)  If the specified file is not a DAS file, as indicated by the
///      file's ID word, an error is signaled by a routine in the call
///      tree of this routine.
///
///  10) If no logical units are available, an error is signaled
///      by a routine in the call tree of this routine.
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
///  Most DAS files require only read access. If you do not need to
///  change the contents of a file, you should open it using DASOPR.
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
///  1) Dump several parameters from the first DLA segment of a DSK
///     file. Note that DSK files are based on DAS. The segment is
///     assumed to be of type 2.
///
///     Example code begins here.
///
///
///           PROGRAM DASOPR_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(LNSIZE)    OUTLIN
///
///           DOUBLE PRECISION      VOXORI ( 3 )
///           DOUBLE PRECISION      VOXSIZ
///           DOUBLE PRECISION      VTXBDS ( 2, 3 )
///
///           INTEGER               CGSCAL
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               NP
///           INTEGER               NV
///           INTEGER               NVXTOT
///           INTEGER               VGREXT ( 3 )
///           INTEGER               VOXNPL
///           INTEGER               VOXNPT
///           INTEGER               VTXNPL
///
///           LOGICAL               FOUND
///
///
///     C
///     C     Prompt for the name of the DSK to read.
///     C
///           CALL PROMPT ( 'Enter DSK name > ', DSK )
///     C
///     C     Open the DSK file for read access.
///     C     We use the DAS-level interface for
///     C     this function.
///     C
///           CALL DASOPR ( DSK, HANDLE )
///
///     C
///     C     Begin a forward search through the
///     C     kernel, treating the file as a DLA.
///     C     In this example, it's a very short
///     C     search.
///     C
///           CALL DLABFS ( HANDLE, DLADSC, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///     C
///     C        We arrive here only if the kernel
///     C        contains no segments.  This is
///     C        unexpected, but we're prepared for it.
///     C
///              CALL SETMSG ( 'No segments found '
///          .   //            'in DSK file #.'    )
///              CALL ERRCH  ( '#',  DSK           )
///              CALL SIGERR ( 'SPICE(NODATA)'     )
///
///           END IF
///
///     C
///     C     If we made it this far, DLADSC is the
///     C     DLA descriptor of the first segment.
///     C
///     C     Read and display type 2 bookkeeping data.
///     C
///           CALL DSKB02 ( HANDLE, DLADSC, NV,     NP,     NVXTOT,
///          .              VTXBDS, VOXSIZ, VOXORI, VGREXT, CGSCAL,
///          .              VTXNPL, VOXNPT, VOXNPL                 )
///
///     C
///     C     Show vertex and plate counts.
///     C
///           OUTLIN = 'Number of vertices:                 #'
///           CALL REPMI  ( OUTLIN, '#', NV, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of plates:                   #'
///           CALL REPMI  ( OUTLIN, '#', NP, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Voxel edge length (km):             #'
///           CALL REPMF  ( OUTLIN, '#', VOXSIZ, 6, 'E', OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of voxels:                   #'
///           CALL REPMI  ( OUTLIN, '#', NVXTOT, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///     C
///     C     Close the kernel.  This isn't necessary in a stand-
///     C     alone program, but it's good practice in subroutines
///     C     because it frees program and system resources.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds, the output
///     was:
///
///
///     Enter DSK name > phobos512.bds
///     Number of vertices:                 1579014
///     Number of plates:                   3145728
///     Voxel edge length (km):             1.04248E-01
///     Number of voxels:                   11914500
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
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added code example using example from DSKB02.
///
///         Updated $Exceptions entries #7 and #9: error handling for the
///         DAS open failure and "file is not a DAS file" cases is now
///         performed by lower level code (ZZDDHOPN).
///
/// -    SPICELIB Version 8.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 14-DEC-2001 (FST)
///
///         The DAS file ID word and internal file name are no longer
///         buffered by this routine. See DASFM's $Revisions section
///         for details.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 3.0.0, 15-JUN-1994 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type. Added error
///         checks on file names. Fixed bug involving use of sign of
///         file handles. Improved some error messages. (delete rest)
///
/// -    SPICELIB Version 2.0.0, 11-APR-1994 (HAN)
///
///         Updated module to include values for the Silicon Graphics/IRIX,
///         DEC Alpha-OSF/1, and Next/Absoft Fortran platforms. Entry
///         points affected are: DASFM, DASOPR.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added.
///
/// -    SPICELIB Version 3.0.1, 24-APR-2003 (EDW)
///
///         Added MAC-OSX-F77 to the list of platforms
///         that require READONLY to read write protected
///         kernels.
///
/// -    SPICELIB Version 3.0.0, 15-JUN-1994 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type.
///
///         Split an IF ... ELSE IF ... statement into 2 IF statements of
///         equivalent behavior to allow testing of the file architecture.
///
///         Added code to test the file architecture and to verify that the
///         file is a DAS file.
///
///         Removed the error SPICE(DASNOIDWORD) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADASFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAS file.
///
///         Added a test for a blank filename before attempting to use the
///         filename in the routine. If the filename is blank, the error
///         SPICE(BLANKFILENAME) will be signaled.
///
///         Fixed a bug when dealing with a read/write open conflict for
///         DAS files: the code used the DAF positive/negative handle
///         method to determine read/write access rather than the DAS file
///         table column FTACC. Replaced the code:
///
///            IF ( FTHAN(FINDEX) .LT. 0 ) THEN
///
///         with
///
///            IF ( FTACC(FINDEX) .EQ. WRITE ) THEN
///
///         Changed the long error message when the error
///         SPICE(NOTADASFILE) is signaled to suggest that a common error
///         is attempting to use a text version of the desired file rather
///         than the binary version.
///
/// -    SPICELIB Version 2.0.0, 11-APR-1994 (HAN)
///
///         Updated module to include values for the Silicon Graphics/IRIX,
///         DEC Alpha-OSF/1, and Next/Absoft Fortran platforms. Entry
///         points affected are: DASFM, DASOPR.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dasopr(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DASOPR(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASOPR ( DAS, open for read )
pub fn DASOPR(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASOPR", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Try to open the file for READ access.
    //
    //
    //    The file may or may not already be open. If so, it should have
    //    not been opened for writing (FTACC .EQ. WRITE). If opened for
    //    reading, ZZDDHOPN will just increment the number of links and
    //    return the handle.
    //
    ZZDDHOPN(FNAME, b"READ", b"DAS", HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPR", ctx)?;
        return Ok(());
    }

    //
    // Determine whether this file is already in the file table.
    // Search for a matching handle.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == *HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if save.FOUND {
        //
        // The file is already open for read access. Increment the number
        // of links to this file.
        //
        save.FTLNK[save.FINDEX] = (save.FTLNK[save.FINDEX] + 1);
        *HANDLE = save.FTHAN[save.FINDEX];
        //
        // There's nothing else to do.
        //
        CHKOUT(b"DASOPR", ctx)?;
        return Ok(());
    }

    //
    // This file has no entry in the file table. We need to
    // create a new entry. See whether there's room for it.
    //
    if (LNKNFN(save.POOL.as_slice()) == 0) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DASFTFULL)", ctx)?;
        CHKOUT(b"DASOPR", ctx)?;
        return Ok(());
    }

    //
    // Read the file's file record.
    //
    ZZDASRFR(
        *HANDLE,
        &mut save.IDWORD,
        &mut save.LOCIFN,
        &mut save.LOCRRC,
        &mut save.LOCRCH,
        &mut save.LOCCRC,
        &mut save.LOCCCH,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPR", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about our newly
    // opened DAS file. Link the information for this file at the
    // head of the file table list.
    //
    LNKAN(save.POOL.as_slice_mut(), &mut save.NEW, ctx)?;
    LNKILB(save.NEW, save.FTHEAD, save.POOL.as_slice_mut(), ctx)?;

    save.FTHEAD = save.NEW;
    save.FTHAN[save.FTHEAD] = *HANDLE;
    save.FTACC[save.FTHEAD] = READ;
    save.FTLNK[save.FTHEAD] = 1;

    //
    // Fill in the file summary. We already know how many reserved
    // records and comment records there are. To find the number of the
    // first free record, the last logical address of each type, and the
    // locations of the last descriptors of each type, we must examine
    // the directory records. We do not assume that the data records in
    // the DAS file have been segregated.
    //
    CLEARI(SUMSIZ, save.FTSUM.subarray_mut([1, save.FTHEAD]));

    save.FTSUM[[RRCIDX, save.FTHEAD]] = save.LOCRRC;
    save.FTSUM[[RCHIDX, save.FTHEAD]] = save.LOCRCH;
    save.FTSUM[[CRCIDX, save.FTHEAD]] = save.LOCCRC;
    save.FTSUM[[CCHIDX, save.FTHEAD]] = save.LOCCCH;

    //
    // We'll find the values for each data type separately.
    //
    for TYPE in 1..=3 {
        //
        // The first directory record is located right after the
        // last comment record.
        //
        save.NREC = ((save.LOCRRC + save.LOCCRC) + 2);

        //
        // Keep track of the record number of the last data
        // record of the current type.
        //
        save.LDREC[TYPE] = 0;

        //
        // Find the last directory containing a descriptor of a
        // record cluster of the current type.
        //
        ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

        save.MAXADR = save.DIRREC[(RNGBAS + (2 * TYPE))];
        save.NXTDIR = save.DIRREC[FWDLOC];

        while (save.NXTDIR > 0) {
            //
            // Read the directory record. If this record contains
            // descriptors for clusters we're interested in, update the
            // directory record number.
            //
            ZZDASGRI(*HANDLE, save.NXTDIR, save.DIRREC.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASOPR", ctx)?;
                return Ok(());
            }

            if (save.DIRREC[(RNGBAS + (2 * TYPE))] > 0) {
                save.MAXADR = save.DIRREC[(RNGBAS + (2 * TYPE))];
                save.NREC = save.NXTDIR;
            }

            save.NXTDIR = save.DIRREC[FWDLOC];
        }

        //
        // At this point, NREC is the record number of the directory
        // containing the last descriptor for clusters of TYPE, if
        // there are any such descriptors.
        //
        // MAXADR is the maximum logical address of TYPE.
        //
        save.FTSUM[[(LLABAS + TYPE), save.FTHEAD]] = save.MAXADR;

        if (save.MAXADR > 0) {
            save.FTSUM[[(LRCBAS + TYPE), save.FTHEAD]] = save.NREC;
        } else {
            save.FTSUM[[(LRCBAS + TYPE), save.FTHEAD]] = 0;
        }
        //
        // We still need to set the word location of the final
        // descriptor of TYPE, if there are any descriptors of TYPE.
        //
        if (save.MAXADR > 0) {
            //
            // Re-read the directory record containing the last descriptor
            // of the current type.
            //
            ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASOPR", ctx)?;
                return Ok(());
            }

            //
            // Traverse the directory record, looking for the last
            // descriptor of TYPE. We'll keep track of the maximum logical
            // address of TYPE for each cluster of TYPE whose descriptor
            // we examine. When this value is the maximum logical address
            // of TYPE, we've found the last descriptor of TYPE.
            //
            // Also keep track of the end record numbers for each
            // cluster.
            //
            save.LAST = (save.DIRREC[(RNGBAS + ((2 * TYPE) - 1))] - 1);
            save.DSCTYP = save.DIRREC[BEGDSC];
            save.PRVTYP = save.PREV[save.DSCTYP];
            save.ENDREC = save.NREC;
            save.POS = BEGDSC;

            while (save.LAST < save.MAXADR) {
                save.POS = (save.POS + 1);

                if (save.DIRREC[save.POS] > 0) {
                    save.CURTYP = save.NEXT[save.PRVTYP];
                } else {
                    save.CURTYP = save.PREV[save.PRVTYP];
                }

                if (save.CURTYP == TYPE) {
                    save.LAST = (save.LAST + (save.NW[TYPE] * i32::abs(save.DIRREC[save.POS])));
                }

                save.ENDREC = (save.ENDREC + i32::abs(save.DIRREC[save.POS]));
                save.PRVTYP = save.CURTYP;
            }
            //
            // At this point, POS is the word position of the last
            // descriptor of TYPE, and ENDREC is the record number
            // of the last data record of TYPE.
            //
            save.FTSUM[[(LWDBAS + TYPE), save.FTHEAD]] = save.POS;
            save.LDREC[TYPE] = save.ENDREC;
        } else {
            //
            // There's no data of TYPE in the file.
            //
            save.FTSUM[[(LWDBAS + TYPE), save.FTHEAD]] = 0;
            save.LDREC[TYPE] = 0;
        }
    }

    //
    // We're almost done; we need to find the number of the first free
    // record. This record follows all of the data records and all of
    // the directory records. It may happen that the last record in use
    // is an empty directory.
    //
    MAXAI(save.LDREC.as_slice(), 3, &mut save.LDRMAX, &mut save.LOC);

    save.NREC = ((save.LOCRRC + save.LOCCRC) + 2);

    ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

    save.NXTREC = save.DIRREC[FWDLOC];

    while (save.NXTREC != 0) {
        save.NREC = save.NXTREC;

        ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

        save.NXTREC = save.DIRREC[FWDLOC];
    }

    //
    // Now NREC is the last directory record.
    //
    save.FTSUM[[FREIDX, save.FTHEAD]] = (intrinsics::MAX0(&[save.LDRMAX, save.NREC]) + 1);

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DASOPR", ctx)?;
    Ok(())
}

/// DAS, open for write
///
/// Open a DAS file for writing.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAS file to be opened.
///  HANDLE     O   Handle assigned to the opened DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a DAS file to be opened with write
///           access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle that is associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAS routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input filename is blank, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If the specified file does not exist, an error is signaled by
///      a routine in the call tree of this routine.
///
///  3)  If the specified file has already been opened, either by the
///      DAS file routines or by other code, an error is signaled by a
///      routine in the call tree of this routine. Note that this
///      response is not paralleled by DASOPR, which allows you to open
///      a DAS file for reading even if it is already open for reading.
///
///  4)  If the specified file cannot be opened without exceeding
///      the maximum allowed number of open DAS files, the error
///      SPICE(DASFTFULL) is signaled.
///
///  5)  If the specified file cannot be opened properly, an error
///      is signaled by a routine in the call tree of this routine.
///
///  6)  If the file record cannot be read, an error is signaled by a
///      routine in the call tree of this routine.
///
///  7)  If the specified file is not a DAS file, as indicated by the
///      file's ID word, an error is signaled by a routine in the call
///      tree of this routine.
///
///  8)  If no logical units are available, an error is signaled
///      by a routine in the call tree of this routine.
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
///  Most DAS files require only read access. If you do not need to
///  change the contents of a file, you should open it with DASOPR.
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
///  1) Create a new DAS file containing 200 integer addresses set
///     to zero. Re-open the file for write access again, and write
///     to its addresses 1 through 200 in random-access fashion by
///     updating the file.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASOPW_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local Parameters
///     C
///           CHARACTER*(*)         DASNAM
///           PARAMETER           ( DASNAM   = 'dasopw_ex1.das' )
///
///           INTEGER               IDATLN
///           PARAMETER           ( IDATLN = 200 )
///
///           INTEGER               TYPELN
///           PARAMETER           ( TYPELN = 4   )
///
///     C
///     C     Local Variables
///     C
///           CHARACTER*(TYPELN)    TYPE
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IDATA   ( IDATLN )
///           INTEGER               J
///
///     C
///     C     Open a new DAS file. Reserve no comment records.
///     C
///           TYPE = 'TEST'
///           CALL DASONW ( DASNAM, TYPE,  'TEST/DASOPW_EX1',
///          .              0,      HANDLE                  )
///
///     C
///     C     Append 200 integers to the file; after the data are
///     C     present, we're free to update it in any order we
///     C     please. (CLEARI zeros out an integer array.)
///     C
///           CALL CLEARI (         IDATLN, IDATA )
///           CALL DASADI ( HANDLE, IDATLN, IDATA )
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Open the file again for writing.
///     C
///           CALL DASOPW ( DASNAM, HANDLE )
///
///     C
///     C     Reset the data array, and read the data into it.
///     C
///           CALL FILLI  ( -1, IDATLN, IDATA )
///           CALL DASRDI ( HANDLE,  1, IDATLN, IDATA )
///
///     C
///     C     Print the contents of the file before updating it.
///     C
///           WRITE(*,*) 'Contents of ' // DASNAM // ' before update:'
///           WRITE(*,*) ' '
///           DO I = 1, 20
///              WRITE (*,'(10I5)') (IDATA((I-1)*10+J), J = 1, 10)
///           END DO
///
///     C
///     C     Now the integer logical addresses 1:200 can be
///     C     written to in random-access fashion. We'll fill them
///     C     in reverse order.
///     C
///           DO I = IDATLN, 1, -1
///              CALL DASUDI ( HANDLE, I, I, I )
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now make sure that we updated the file properly.
///     C     Open the file for reading and dump the contents
///     C     of the integer logical addresses 1:200.
///     C
///           CALL DASOPR ( DASNAM, HANDLE )
///
///           CALL CLEARI (             IDATLN, IDATA )
///           CALL DASRDI ( HANDLE,  1, IDATLN, IDATA )
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Contents of ' // DASNAM // ' after update:'
///           WRITE(*,*) ' '
///           DO I = 1, 20
///              WRITE (*,'(10I5)') (IDATA((I-1)*10+J), J = 1, 10)
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Contents of dasopw_ex1.das before update:
///
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///         0    0    0    0    0    0    0    0    0    0
///
///      Contents of dasopw_ex1.das after update:
///
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///       101  102  103  104  105  106  107  108  109  110
///       111  112  113  114  115  116  117  118  119  120
///       121  122  123  124  125  126  127  128  129  130
///       131  132  133  134  135  136  137  138  139  140
///       141  142  143  144  145  146  147  148  149  150
///       151  152  153  154  155  156  157  158  159  160
///       161  162  163  164  165  166  167  168  169  170
///       171  172  173  174  175  176  177  178  179  180
///       181  182  183  184  185  186  187  188  189  190
///       191  192  193  194  195  196  197  198  199  200
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Updated $Exceptions #3 -- short error message is
///         SPICE(FILEOPENCONFLICT) -- #5 and #7 -- error handling for
///         the DAS open failure and "file is not a DAS file" cases is
///         now performed by lower level code (ZZDDHOPN).
///
/// -    SPICELIB Version 8.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 14-DEC-2001 (FST)
///
///         The DAS file ID word and internal file name are no longer
///         buffered by this routine. See DASFM's $Revisions section
///         for details.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 29-OCT-1993 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 7.0.0, 28-SEP-2005 (NJB)
///
///         Error handling for non-native files was added.
///
/// -    SPICELIB Version 2.0.0, 29-OCT-1993 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type.
///
///         Split an IF ... ELSE IF ... statement into 2 IF statements of
///         equivalent behavior to allow testing of the file architecture.
///
///         Added code to test the file architecture and to verify that the
///         file is a DAS file.
///
///         Removed the error SPICE(DASNOIDWORD) as it was no longer
///         relevant.
///
///         Added the error SPICE(NOTADASFILE) if this routine is called
///         with a file that does not contain an ID word identifying the
///         file as a DAF file.
///
///         Added a test for a blank filename before attempting to use the
///         filename in the routine. If the filename is blank, the error
///         SPICE(BLANKFILENAME) will be signaled.
///
///         Changed the long error message when the error
///         SPICE(NOTADASFILE) is signaled to suggest that a common error
///         is attempting to load a text version of the desired file rather
///         than the binary version.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dasopw(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DASOPW(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASOPW ( DAS, open for write )
pub fn DASOPW(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASOPW", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    if (LNKNFN(save.POOL.as_slice()) == 0) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DASFTFULL)", ctx)?;
        CHKOUT(b"DASOPW", ctx)?;
        return Ok(());
    }

    //
    // Open the file for writing:   open the file, get the
    // internal file name, and set the number of links to one.
    //
    ZZDDHOPN(FNAME, b"WRITE", b"DAS", HANDLE, ctx)?;

    //
    // Read the file record.
    //
    ZZDASRFR(
        *HANDLE,
        &mut save.IDWORD,
        &mut save.LOCIFN,
        &mut save.LOCRRC,
        &mut save.LOCRCH,
        &mut save.LOCCRC,
        &mut save.LOCCCH,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPW", ctx)?;
        return Ok(());
    }

    //
    // At this point, we know that we have a valid DAS file, and
    // we're set up to read from it, so ...
    //
    // Update the file table to include information about
    // our newly opened DAS file. Link the information
    // for this file at the head of the file table list.
    //
    // Set the output argument HANDLE as well.
    //
    LNKAN(save.POOL.as_slice_mut(), &mut save.NEW, ctx)?;
    LNKILB(save.NEW, save.FTHEAD, save.POOL.as_slice_mut(), ctx)?;

    save.FTHEAD = save.NEW;
    save.FTHAN[save.FTHEAD] = *HANDLE;
    save.FTACC[save.FTHEAD] = WRITE;
    save.FTLNK[save.FTHEAD] = 1;

    //
    // Fill in the file summary. We already know how many reserved
    // records and comment records there are. To find the number of the
    // first free record, the last logical address of each type, and the
    // locations of the last descriptors of each type, we must examine
    // the directory records. We do not assume that the data records in
    // the DAS file have been segregated.
    //
    CLEARI(SUMSIZ, save.FTSUM.subarray_mut([1, save.FTHEAD]));

    save.FTSUM[[RRCIDX, save.FTHEAD]] = save.LOCRRC;
    save.FTSUM[[RCHIDX, save.FTHEAD]] = save.LOCRCH;
    save.FTSUM[[CRCIDX, save.FTHEAD]] = save.LOCCRC;
    save.FTSUM[[CCHIDX, save.FTHEAD]] = save.LOCCCH;

    //
    // We'll need the logical unit connected to the file, so
    // we can read the file's directory records.
    //
    ZZDDHHLU(*HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPW", ctx)?;
        return Ok(());
    }

    //
    // We'll find the values for each data type separately.
    //
    for TYPE in 1..=3 {
        //
        // The first directory record is located right after the
        // last comment record.
        //
        save.NREC = ((save.LOCRRC + save.LOCCRC) + 2);

        //
        // Keep track of the record number of the last data
        // record of the current type.
        //
        save.LDREC[TYPE] = 0;

        //
        // Find the last directory containing a descriptor of a
        // record cluster of the current type.
        //
        ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

        save.MAXADR = save.DIRREC[(RNGBAS + (2 * TYPE))];
        save.NXTDIR = save.DIRREC[FWDLOC];

        while (save.NXTDIR > 0) {
            //
            // Read the directory record. If this record contains
            // descriptors for clusters we're interested in, update the
            // directory record number.
            //
            ZZDASGRI(*HANDLE, save.NXTDIR, save.DIRREC.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASOPW", ctx)?;
                return Ok(());
            }

            if (save.DIRREC[(RNGBAS + (2 * TYPE))] > 0) {
                save.MAXADR = save.DIRREC[(RNGBAS + (2 * TYPE))];
                save.NREC = save.NXTDIR;
            }

            save.NXTDIR = save.DIRREC[FWDLOC];
        }

        //
        // At this point, NREC is the record number of the directory
        // containing the last descriptor for clusters of TYPE, if
        // there are any such descriptors.
        //
        // MAXADR is the maximum logical address of TYPE.
        //
        save.FTSUM[[(LLABAS + TYPE), save.FTHEAD]] = save.MAXADR;

        if (save.MAXADR > 0) {
            save.FTSUM[[(LRCBAS + TYPE), save.FTHEAD]] = save.NREC;
        } else {
            save.FTSUM[[(LRCBAS + TYPE), save.FTHEAD]] = 0;
        }
        //
        // We still need to set the word location of the final
        // descriptor of TYPE, if there are any descriptors of TYPE.
        //
        if (save.MAXADR > 0) {
            //
            // Re-read the directory record containing the last descriptor
            // of the current type.
            //
            ZZDASGRI(*HANDLE, save.NREC, save.DIRREC.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASOPW", ctx)?;
                return Ok(());
            }

            //
            // Traverse the directory record, looking for the last
            // descriptor of TYPE. We'll keep track of the maximum logical
            // address of TYPE for each cluster of TYPE whose descriptor
            // we examine. When this value is the maximum logical address
            // of TYPE, we've found the last descriptor of TYPE.
            //
            // Also keep track of the end record numbers for each
            // cluster.
            //
            save.LAST = (save.DIRREC[(RNGBAS + ((2 * TYPE) - 1))] - 1);
            save.DSCTYP = save.DIRREC[BEGDSC];
            save.PRVTYP = save.PREV[save.DSCTYP];
            save.ENDREC = save.NREC;
            save.POS = BEGDSC;

            while (save.LAST < save.MAXADR) {
                save.POS = (save.POS + 1);

                if (save.DIRREC[save.POS] > 0) {
                    save.CURTYP = save.NEXT[save.PRVTYP];
                } else {
                    save.CURTYP = save.PREV[save.PRVTYP];
                }

                if (save.CURTYP == TYPE) {
                    save.LAST = (save.LAST + (save.NW[TYPE] * i32::abs(save.DIRREC[save.POS])));
                }

                save.ENDREC = (save.ENDREC + i32::abs(save.DIRREC[save.POS]));
                save.PRVTYP = save.CURTYP;
            }
            //
            // At this point, POS is the word position of the last
            // descriptor of TYPE, and ENDREC is the record number
            // of the last data record of TYPE.
            //
            save.FTSUM[[(LWDBAS + TYPE), save.FTHEAD]] = save.POS;
            save.LDREC[TYPE] = save.ENDREC;
        } else {
            //
            // There's no data of TYPE in the file.
            //
            save.FTSUM[[(LWDBAS + TYPE), save.FTHEAD]] = 0;
            save.LDREC[TYPE] = 0;
        }
    }

    //
    // We're almost done; we need to find the number of the first free
    // record. This record follows all of the data records and all of
    // the directory records. It may happen that the last record in use
    // is an empty directory.
    //
    MAXAI(save.LDREC.as_slice(), 3, &mut save.LDRMAX, &mut save.LOC);

    save.NREC = ((save.LOCRRC + save.LOCCRC) + 2);

    DASIOI(
        b"READ",
        save.NUMBER,
        save.NREC,
        save.DIRREC.as_slice_mut(),
        ctx,
    )?;

    save.NXTREC = save.DIRREC[FWDLOC];

    while (save.NXTREC != 0) {
        save.NREC = save.NXTREC;

        DASIOI(
            b"READ",
            save.NUMBER,
            save.NREC,
            save.DIRREC.as_slice_mut(),
            ctx,
        )?;

        save.NXTREC = save.DIRREC[FWDLOC];
    }

    //
    // Now NREC is the last directory record.
    //
    save.FTSUM[[FREIDX, save.FTHEAD]] = (intrinsics::MAX0(&[save.LDRMAX, save.NREC]) + 1);

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DASOPW", ctx)?;
    Ok(())
}

/// DAS, open new file
///
/// Open a new DAS file and set the file type.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAS file to be opened.
///  FTYPE      I   Mnemonic code for type of data in the DAS file.
///  IFNAME     I   Internal file name.
///  NCOMR      I   Number of comment records to allocate.
///  HANDLE     O   Handle assigned to the opened DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new DAS file to be created (and
///           consequently opened for write access).
///
///  FTYPE    is a string indicating the type of data placed into a DAS
///           file. The first nonblank character and the three, or
///           fewer, characters immediately following it are stored as
///           the part of the file's ID word following the forward
///           slash. It is an error if FTYPE is blank.
///
///           The file type may not contain any nonprinting characters.
///           FTYPE is case sensitive.
///
///           NAIF has reserved for its own use file types
///           consisting of the upper case letters (A-Z) and the
///           digits 0-9. NAIF recommends lower case or mixed case
///           file types be used by all others in order to avoid any
///           conflicts with NAIF file types.
///
///  IFNAME   is a string containing the internal file name for the new
///           file. The name may contain as many as 60 characters. This
///           should uniquely identify the file.
///
///  NCOMR    is the number of comment records to allocate.
///           Allocating comment records at file creation time may
///           reduce the likelihood of having to expand the
///           comment area later.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAS routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input filename is blank, the error
///      SPICE(BLANKFILENAME) is signaled.
///
///  2)  If the specified file cannot be opened without exceeding
///      the maximum allowed number of open DAS files, the error
///      SPICE(DASFTFULL) is signaled. No file will be created.
///
///  3)  If the file cannot be opened properly, an error is signaled
///      by a routine in the call tree of this routine. No file will
///      be created.
///
///  4)  If the initial records in the file cannot be written, an
///      error is signaled by a routine in the call tree of this
///      routine. No file will be created.
///
///  5)  If the file type is blank, the error SPICE(BLANKFILETYPE) is
///      signaled.
///
///  6)  If the file type contains nonprinting characters---decimal
///      0-31 and 127-255---, the error SPICE(ILLEGALCHARACTER) is
///      signaled.
///
///  7)  If the number of comment records allocated NCOMR is negative,
///      the error SPICE(INVALIDCOUNT) is signaled.
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
///  The DAS files created by this routine have initialized file
///  records.
///
///  This entry point creates a new DAS file and sets the type of the
///  file to the mnemonic code passed to it.
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
///  1) Create a new DAS file and add 200 integers to it. Close the
///     file, then re-open it and read the data back out.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASONW_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasonw_ex1.das' )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(4)         TYPE
///
///           INTEGER               DATA   ( 200 )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Open a new DAS file. Use the file name as the internal
///     C     file name, and reserve no records for comments.
///     C
///           TYPE = 'TEST'
///           CALL DASONW ( FNAME, TYPE, FNAME, 0, HANDLE )
///
///     C
///     C     Fill the array DATA with the integers 1 through
///     C     100, and add this array to the file.
///     C
///           DO I = 1, 100
///              DATA(I) = I
///           END DO
///
///           CALL DASADI ( HANDLE, 100, DATA )
///
///     C
///     C     Now append the array DATA to the file again.
///     C
///           CALL DASADI ( HANDLE, 100, DATA )
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now verify the addition of data by opening the
///     C     file for read access and retrieving the data.
///     C
///           CALL DASOPR ( FNAME, HANDLE )
///           CALL DASRDI ( HANDLE, 1, 200, DATA )
///
///     C
///     C     Dump the data to the screen.  We should see the
///     C     sequence  1, 2, ..., 100, 1, 2, ... , 100.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Data from "', FNAME, '":'
///           WRITE (*,*) ' '
///           DO I = 1, 20
///              WRITE (*,'(10I5)') (DATA((I-1)*10+J), J = 1, 10)
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Data from "dasonw_ex1.das":
///
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Updated $Exceptions entry #3: error handling for the DAS open
///         failure is now performed by lower level code (ZZDDHOPN).
///
/// -    SPICELIB Version 7.0.0, 26-FEB-2015 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (FST)
///
///         The DAS file ID word and internal file name are no longer
///         buffered by this routine. See DASFM's $Revisions section
///         for details.
///
///         The entry point was modified to insert the FTP validation
///         string, as well as the binary file format into the file record.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 31-AUG-1995 (NJB)
///
///         Changed argument list of the entry point DASONW. The input
///         argument NCOMR, which indicates the number of comment records
///         to reserve, was added to the argument list.
///
/// -    SPICELIB Version 1.0.0, 29-OCT-1993 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (NJB) (FST)
///
///         See the $Revisions section under DASFM for a discussion of
///         the various changes made for this version.
/// ```
pub fn dasonw(
    ctx: &mut SpiceContext,
    fname: &str,
    ftype: &str,
    ifname: &str,
    ncomr: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    DASONW(
        fname.as_bytes(),
        ftype.as_bytes(),
        ifname.as_bytes(),
        ncomr,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASONW ( DAS, open new file )
pub fn DASONW(
    FNAME: &[u8],
    FTYPE: &[u8],
    IFNAME: &[u8],
    NCOMR: i32,
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
    }

    CHKIN(b"DASONW", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }
    //
    // Check to see whether the filename is blank. If it is, signal an
    // error, check out, and return.
    //
    if fstr::eq(FNAME, b" ") {
        SETMSG(b"The file name is blank. ", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"DASONW", ctx)?;
        return Ok(());
    }
    //
    // Check if the file type is blank.
    //
    if fstr::eq(FTYPE, b" ") {
        SETMSG(b"The file type is blank. ", ctx);
        SIGERR(b"SPICE(BLANKFILETYPE)", ctx)?;
        CHKOUT(b"DASONW", ctx)?;
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
            SETMSG(b"The file type contains nonprinting characters. ", ctx);
            SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
            CHKOUT(b"DASONW", ctx)?;
            return Ok(());
        }
    }

    //
    // Validate the comment record count.
    //
    if (NCOMR < 0) {
        SETMSG(
            b"The number of comment records allocated must be non-negative but was #.",
            ctx,
        );
        ERRINT(b"#", NCOMR, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"DASONW", ctx)?;
        return Ok(());
    }

    //
    // Set the value the file type in a temporary variable to be sure of
    // its length and then set the value of the ID word. Only 4
    // characters are allowed for the file type, and they are the first
    // nonblank character and its three (3) immediate successors in the
    // input string FTYPE.
    //
    fstr::assign(&mut save.TTYPE, fstr::substr(FTYPE, save.FNB..));
    fstr::assign(&mut save.IDWORD, &fstr::concat(b"DAS/", &save.TTYPE));

    //
    // The file can be opened only if there is room for another file.
    //
    if (LNKNFN(save.POOL.as_slice()) == 0) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DASFTFULL)", ctx)?;
        CHKOUT(b"DASONW", ctx)?;
        return Ok(());
    }

    //
    // Open a new file.
    //
    ZZDDHOPN(FNAME, b"NEW", b"DAS", HANDLE, ctx)?;

    //
    // Get a logical unit for the file.
    //
    ZZDDHHLU(*HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

    //
    // Fetch the system file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut save.FORMAT, ctx);

    //
    // Prepare to write the file record.
    //
    // Use a local variable for the internal file name to ensure
    // that IFNLEN characters are written. The remaining
    // elements of the file record are:
    //
    //    -- the number of reserved records
    //
    //    -- the number of characters in use in the reserved
    //       record area
    //
    //    -- the number of comment records
    //
    //    -- the number of characters in use in the comment area
    //
    // Initially, all of these counts are zero, except for the
    // comment record count, which is set by the caller.
    //
    fstr::assign(&mut save.LOCIFN, IFNAME);

    ZZDASNFR(
        save.NUMBER,
        &save.IDWORD,
        &save.LOCIFN,
        0,
        0,
        NCOMR,
        0,
        &save.FORMAT,
        ctx,
    )?;

    //
    // Check to see whether or not ZZDASNFR generated an error
    // writing the file record to the logical unit. In the event
    // an error occurs, checkout and return.
    //
    if FAILED(ctx) {
        CHKOUT(b"DASONW", ctx)?;
        return Ok(());
    }

    //
    // Zero out the first directory record in the file. If this
    // write fails, close the file with delete status and return
    // immediately. The first directory record follows the
    // comment records and reserved records. Currently there
    // are no reserved records, so the directory occupies record
    // NCOMR+2.
    //
    CLEARI(NWI, save.DIRREC.as_slice_mut());

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.NUMBER)?, Some((NCOMR + 2)))?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            for n in save.DIRREC.iter() {
                writer.write_i32(*n)?;
            }
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        //
        // We had a write error. Ask the handle manager to
        // close and delete the new file.
        //
        ZZDDHCLS(*HANDLE, b"DAS", true, ctx)?;

        CHKOUT(b"DASONW", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about
    // our newly opened DAS file. Link the information
    // for this file at the head of the file table list.
    //
    // Set the output argument HANDLE as well.
    //
    LNKAN(save.POOL.as_slice_mut(), &mut save.NEW, ctx)?;
    LNKILB(save.NEW, save.FTHEAD, save.POOL.as_slice_mut(), ctx)?;

    //
    // Clear out the file summary, except for the number of comment
    // records and the free record pointer. The free record pointer
    // should point to the first record AFTER the first directory.
    //
    save.FTHEAD = save.NEW;

    CLEARI(SUMSIZ, save.FTSUM.subarray_mut([1, save.FTHEAD]));

    save.FTHAN[save.FTHEAD] = *HANDLE;
    save.FTACC[save.FTHEAD] = WRITE;
    save.FTLNK[save.FTHEAD] = 1;
    save.FTSUM[[FREIDX, save.FTHEAD]] = (NCOMR + 3);
    save.FTSUM[[CRCIDX, save.FTHEAD]] = NCOMR;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DASONW", ctx)?;
    Ok(())
}

/// DAS, open new
///
/// Obsolete: This routine has been superseded by DASONW, and it is
/// supported for purposes of backward compatibility only.
///
/// Open a new DAS file for writing.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAS file to be opened.
///  IFNAME     I   Internal file name.
///  HANDLE     O   Handle assigned to the opened DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new DAS file to be created (and
///           consequently opened for write access).
///
///  IFNAME   is the internal file name for the new file. The name
///           may contain as many as 60 characters. This should
///           uniquely identify the file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DAS routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input filename is blank, the error
///      SPICE(BLANKFILENAME) is signaled.
///
///  2)  If the specified file cannot be opened without exceeding
///      the maximum allowed number of open DAS files, the error
///      SPICE(DASFTFULL) is signaled. No file will be created.
///
///  3)  If the file cannot be opened properly, an error is signaled
///      by a routine in the call tree of this routine. No file will
///      be created.
///
///  4)  If the initial records in the file cannot be written, an
///      error is signaled by a routine in the call tree of this
///      routine. No file will be created.
///
///  5)  If no logical units are available, an error is signaled by a
///      routine in the call tree of this routine. No file will be
///      created.
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
///  The DAS files created by this routine have initialized file
///  records.
///
///  This entry point has been made obsolete by the entry point DASONW,
///  and it is supported for reasons of backward compatibility only.
///  New software development should use the entry point DASONW.
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
///  1) Create a new DAS file and add 200 integers to it. Close the
///     file, then re-open it and read the data back out.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASOPN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dasopn_ex1.das' )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(4)         TYPE
///
///           INTEGER               DATA   ( 200 )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Open a new DAS file. Use the file name as the internal
///     C     file name.
///     C
///           TYPE = 'TEST'
///           CALL DASOPN ( FNAME, FNAME, HANDLE )
///
///     C
///     C     Fill the array DATA with the integers 1 through
///     C     100, and add this array to the file.
///     C
///           DO I = 1, 100
///              DATA(I) = I
///           END DO
///
///           CALL DASADI ( HANDLE, 100, DATA )
///
///     C
///     C     Now append the array DATA to the file again.
///     C
///           CALL DASADI ( HANDLE, 100, DATA )
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///     C
///     C     Now verify the addition of data by opening the
///     C     file for read access and retrieving the data.
///     C
///           CALL DASOPR ( FNAME, HANDLE )
///           CALL DASRDI ( HANDLE, 1, 200, DATA )
///
///     C
///     C     Dump the data to the screen.  We should see the
///     C     sequence  1, 2, ..., 100, 1, 2, ... , 100.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Data from "', FNAME, '":'
///           WRITE (*,*) ' '
///           DO I = 1, 20
///              WRITE (*,'(10I5)') (DATA((I-1)*10+J), J = 1, 10)
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Data from "dasopn_ex1.das":
///
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///         1    2    3    4    5    6    7    8    9   10
///        11   12   13   14   15   16   17   18   19   20
///        21   22   23   24   25   26   27   28   29   30
///        31   32   33   34   35   36   37   38   39   40
///        41   42   43   44   45   46   47   48   49   50
///        51   52   53   54   55   56   57   58   59   60
///        61   62   63   64   65   66   67   68   69   70
///        71   72   73   74   75   76   77   78   79   80
///        81   82   83   84   85   86   87   88   89   90
///        91   92   93   94   95   96   97   98   99  100
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Updated $Exceptions entry #3: error handling for the DAS open
///         failure is now performed by lower level code (ZZDDHOPN).
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (FST)
///
///         The DAS file ID word and internal file name are no longer
///         buffered by this routine. See DASFM's $Revisions section
///         for details.
///
///         This entry point was modified to insert the FTP validation
///         string, as well as the binary file format into the file record.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 29-OCT-1993 (KRG)
///
///         The effect of this routine is unchanged. It still uses the ID
///         word 'NAIF/DAS'. This is for backward compatibility only.
///
///         Added statements to the $Abstract and $Particulars sections
///         to document that this entry is now considered to be obsolete,
///         and that it has been superseded by the entry point DASONW.
///
///         Added a test for a blank filename before attempting to use the
///         filename in the routine. If the filename is blank, the error
///         SPICE(BLANKFILENAME) will be signaled.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.0.0, 11-DEC-2001 (FST)
///
///         See the $Revisions section under DASFM for a discussion
///         of the changes made for this version.
/// ```
pub fn dasopn(
    ctx: &mut SpiceContext,
    fname: &str,
    ifname: &str,
    handle: &mut i32,
) -> crate::Result<()> {
    DASOPN(
        fname.as_bytes(),
        ifname.as_bytes(),
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASOPN ( DAS, open new )
pub fn DASOPN(
    FNAME: &[u8],
    IFNAME: &[u8],
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
    }

    CHKIN(b"DASOPN", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }
    //
    // Check to see whether the filename is blank. If it is, signal an
    // error, check out, and return.
    //
    if fstr::eq(FNAME, b" ") {
        SETMSG(b"The file name is blank. ", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"DASOPN", ctx)?;
        return Ok(());
    }

    //
    // The file can be opened only if there is room for another file.
    //
    if (LNKNFN(save.POOL.as_slice()) == 0) {
        SETMSG(
            b"The file table is full, with # entries. Could not open \'#\'.",
            ctx,
        );
        ERRINT(b"#", FTSIZE, ctx);
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DASFTFULL)", ctx)?;
        CHKOUT(b"DASOPN", ctx)?;
        return Ok(());
    }

    //
    // To open a new file: get a free unit, open the file, write
    // the file record, and set the number of links to one.
    //
    // Look out for:
    //
    //    -- No free logical units.
    //
    //    -- Error opening the file.
    //
    //    -- Error writing to the file.
    //
    // If anything goes wrong after the file has been opened, delete
    // the file.
    //
    //
    ZZDDHOPN(FNAME, b"NEW", b"DAS", HANDLE, ctx)?;

    ZZDDHHLU(*HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPN", ctx)?;
        return Ok(());
    }

    //
    // Fetch the system file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut save.FORMAT, ctx);

    //
    // Prepare to write the file record.
    //
    // Use a local variable for the internal file name to ensure that
    // IFNLEN characters are written. The remaining elements of the file
    // record are:
    //
    //    -- the number of reserved records
    //
    //    -- the number of characters in use in the reserved
    //       record area
    //
    //    -- the number of comment records
    //
    //    -- the number of characters in use in the comment area
    //
    // Initially, all of these counts are zero.
    //
    //
    fstr::assign(&mut save.LOCIFN, IFNAME);
    fstr::assign(&mut save.IDWORD, b"NAIF/DAS");

    ZZDASNFR(
        save.NUMBER,
        &save.IDWORD,
        &save.LOCIFN,
        0,
        0,
        0,
        0,
        &save.FORMAT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPN", ctx)?;
        return Ok(());
    }

    //
    // Zero out the first directory record (record #2) in the file. If
    // this write fails, close the file with delete status and return
    // immediately.
    //

    //
    // NOTE: re-write this using ZZDDHCLS.
    //
    CLEARI(NWI, save.DIRREC.as_slice_mut());
    DASIOI(b"WRITE", save.NUMBER, 2, save.DIRREC.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.NUMBER),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        CHKOUT(b"DASOPN", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about
    // our newly opened DAS file. Link the information
    // for this file at the head of the file table list.
    //
    // Set the output argument HANDLE as well.
    //
    LNKAN(save.POOL.as_slice_mut(), &mut save.NEW, ctx)?;
    LNKILB(save.NEW, save.FTHEAD, save.POOL.as_slice_mut(), ctx)?;

    //
    // Clear out the file summary, except for the free record pointer.
    // The free record pointer should point to the first record AFTER
    // the first directory.
    //
    save.FTHEAD = save.NEW;

    CLEARI(SUMSIZ, save.FTSUM.subarray_mut([1, save.FTHEAD]));

    save.FTHAN[save.FTHEAD] = *HANDLE;
    save.FTACC[save.FTHEAD] = WRITE;
    save.FTLNK[save.FTHEAD] = 1;
    save.FTSUM[[FREIDX, save.FTHEAD]] = 3;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DASOPN", ctx)?;
    Ok(())
}

/// DAS, open scratch
///
/// Open a scratch DAS file for writing.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     O   Handle assigned to a scratch DAS file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the scratch file
///           opened by this routine. This handle is used to
///           identify the file in subsequent calls to other DAS
///           routines.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of DAS files that a user can have
///           open simultaneously. This includes any files used by the
///           DAS system when closing files opened with write access.
///           Currently, DASCLS (via the SPICELIB routine DASSDR) opens
///           a scratch DAS file using DASOPS to segregate (sort by
///           data type) the records in the DAS file being closed.
///           Segregating the data by type improves the speed of access
///           to the data.
///
///           In order to avoid the possibility of overflowing the
///           DAS file table we recommend, when at least one DAS
///           file is open with write access, that users of this
///           software limit themselves to at most FTSIZE - 2 other
///           open DAS files. If no files are to be open with write
///           access, then users may open FTSIZE files with no
///           possibility of overflowing the DAS file table.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified file cannot be opened without exceeding
///      the maximum allowed number of open DAS files, the error
///      SPICE(DASFTFULL) is signaled. No file will be created.
///
///  2)  If file cannot be opened properly, an error is signaled by a
///      routine in the call tree of this routine. No file will be
///      created.
///
///  3)  If the initial records in the file cannot be written, the
///      error SPICE(DASWRITEFAIL) is signaled. No file will be
///      created.
///
///  4)  If no logical units are available, an error is signaled by a
///      routine in the call tree of this routine. No file will be
///      created.
/// ```
///
/// # Files
///
/// ```text
///  See output argument HANDLE.
///
///  See FTSIZE in the $Parameters section for a description of a
///  potential problem with overflowing the DAS file table when at
///  least one DAS file is opened with write access.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility used by the DAS system to provide
///  work space needed when creating new DAS files.
///
///  The DAS files created by this routine have initialized file
///  records. The file type for a DAS scratch file is 'SCR ', so the
///  file type 'SCR ' is not available for general use. As with new
///  permanent files, these files are opened for write access. DAS
///  files opened by DASOPS are automatically deleted when they are
///  closed.
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
///  1) Create a DAS scratch file containing 10 integers, 5 double
///     precision numbers, and 4 characters, then print the logical
///     address ranges in use.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASOPS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               LASTC
///           INTEGER               LASTD
///           INTEGER               LASTI
///
///     C
///     C     Use a scratch file, since there's no reason to keep
///     C     the file.
///     C
///           CALL DASOPS ( HANDLE )
///
///           DO I = 1, 10
///              CALL DASADI ( HANDLE, 1, I )
///           END DO
///
///           DO I = 1, 5
///              CALL DASADD ( HANDLE, 1, DBLE(I) )
///           END DO
///
///     C
///     C     Add character data to the file. DAS character data are
///     C     treated as a character array, not as a string. The
///     C     following call adds only the first 4 characters to the
///     C     DAS file.
///     C
///           CALL DASADC ( HANDLE, 4, 1, 4, 'SPUDWXY' )
///
///     C
///     C     Now check the logical address ranges.
///     C
///           CALL DASLLA ( HANDLE, LASTC, LASTD, LASTI )
///
///           WRITE (*,*) 'Last character address in use: ', LASTC
///           WRITE (*,*) 'Last d.p. address in use     : ', LASTD
///           WRITE (*,*) 'Last integer address in use  : ', LASTI
///
///     C
///     C     Scratch files are automatically deleted when they are
///     C     closed.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Last character address in use:            4
///      Last d.p. address in use     :            5
///      Last integer address in use  :           10
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Extended $Particulars section to indicate that scratch files
///         are deleted when they are closed.
///
///         Updated $Exceptions entry #2: error handling for the DAS open
///         failure is now performed by lower level code (ZZDDHOPN), and
///         added FTSIZE parameter description.
///
/// -    SPICELIB Version 7.0.0, 07-APR-2016 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 29-OCT-1993 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type.
///
///         Put meaningful values into the type and internal filename
///         for a DAS scratch file, rather than leaving them blank.
///
///         Documented the potential problem of overflowing the DAS file
///         table when attempting to close a DAS file opened with write
///         access when the file table is full. Modified the long error
///         message to indicate this as a cause of the problem.
///
/// -    SPICELIB Version 1.1.0, 04-MAY-1993 (NJB)
///
///         Bug fix: removed file name variable from error message.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 29-OCT-1993 (KRG)
///
///         Modified the entry point to use the new file ID format which
///         contains a mnemonic code for the data type.
///
///         DAS scratch files use the type 'SCR ', so the ID word for a DAS
///         scratch file would be: 'DAS/SCR '
///
///         Changed the internal filename from blank to the string:
///
///            'DAS SCRATCH FILE'
///
///         It's probably better to have something written there than
///         nothing.
///
///         Documented the potential problem of overflowing the DAS file
///         table when attempting to close a DAS file opened with write
///         access when the file table is full. Modified the long error
///         message to indicate this as a cause of the problem.
///
///         The problem occurs when the file table is full, the number of
///         open DAS files equals FTSIZE, and at least one of the open
///         files was opened with write access. If an attempt to close a
///         file opened with write access is made under these conditions,
///         by calling DASCLS, it will fail. DASCLS (via DASSDR) calls
///         DASOPS to open a scratch DAS file, but the scratch file CANNOT
///         be opened because the file table is full. If this occurs, close
///         a file open for read access, or restrict the number of open
///         files in use to be at most FTSIZE - 1 when there will be at
///         least one file opened with write access.
///
/// -    SPICELIB Version 1.1.0, 04-MAY-1993 (NJB)
///
///         Bug fix: removed unneeded file name variable FNAME from
///         error message.
/// ```
pub fn dasops(ctx: &mut SpiceContext, handle: &mut i32) -> crate::Result<()> {
    DASOPS(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASOPS ( DAS, open scratch )
pub fn DASOPS(HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASOPS", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // The file can be opened only if there is room for another file.
    //
    if (LNKNFN(save.POOL.as_slice()) == 0) {
        SETMSG(b"The file table is full, with # entries. Could not open a scratch file. If a call to DASOPS was not made and this error occurred, it is likely that the DAS file table was full and an attempt to close a file opened with write access was made. See the DAS required reading and DASFM for details.", ctx);
        ERRINT(b"#", FTSIZE, ctx);
        SIGERR(b"SPICE(DASFTFULL)", ctx)?;
        CHKOUT(b"DASOPS", ctx)?;
        return Ok(());
    }

    //
    // Assign a name to the scratch file. This name is required
    // by the DDH subsystem.
    //
    fstr::assign(&mut save.LOCDAS, b"DAS SCRATCH FILE");

    //
    // Open a DAS file for scratch access.
    //
    ZZDDHOPN(&save.LOCDAS, b"SCRATCH", b"DAS", HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASOPS", ctx)?;
        return Ok(());
    }

    //
    // Get a logical unit for the file.
    //
    ZZDDHHLU(*HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

    //
    // Fetch the system file format.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut save.FORMAT, ctx);

    //
    // Prepare to write the file record.
    //
    // Use a local variable for the internal file name to ensure
    // that IFNLEN characters are written. The remaining
    // elements of the file record are:
    //
    //    -- the number of reserved records
    //
    //    -- the number of characters in use in the reserved
    //       record area
    //
    //    -- the number of comment records
    //
    //    -- the number of characters in use in the comment area
    //
    // Initially, all of these counts are zero, except for the
    // comment record count, which is set by the caller.
    //
    fstr::assign(&mut save.LOCIFN, fstr::substr(&save.LOCDAS, 1..=IFNLEN));

    ZZDASNFR(
        save.NUMBER,
        &save.IDWORD,
        &save.LOCIFN,
        0,
        0,
        0,
        0,
        &save.FORMAT,
        ctx,
    )?;

    //
    // Check to see whether or not ZZDASNFR generated an error
    // writing the file record to the logical unit. In the event
    // an error occurs, checkout and return.
    //
    if FAILED(ctx) {
        CHKOUT(b"DASOPS", ctx)?;
        return Ok(());
    }

    //
    // Update the file table to include information about
    // our newly opened DAS file. Link the information
    // for this file at the head of the file table list.
    //
    // Set the output argument HANDLE as well.
    //
    LNKAN(save.POOL.as_slice_mut(), &mut save.NEW, ctx)?;
    LNKILB(save.NEW, save.FTHEAD, save.POOL.as_slice_mut(), ctx)?;

    save.FTHEAD = save.NEW;

    //
    // Clear out the file summary, except for the free record pointer.
    // The free record pointer should point to the first record AFTER
    // the first directory.
    //
    CLEARI(SUMSIZ, save.FTSUM.subarray_mut([1, save.FTHEAD]));

    save.FTHAN[save.FTHEAD] = *HANDLE;
    save.FTACC[save.FTHEAD] = WRITE;
    save.FTLNK[save.FTHEAD] = 1;
    save.FTSUM[[FREIDX, save.FTHEAD]] = 3;

    //
    // Insert the new handle into our handle set.
    //
    INSRTI(*HANDLE, save.FHLIST.as_slice_mut(), ctx)?;

    CHKOUT(b"DASOPS", ctx)?;
    Ok(())
}

/// DAS, low-level close
///
/// Close the DAS file associated with a given handle, without
/// flushing buffered data or segregating the file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAS file to be closed.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  See $Particulars for a description of the effect of this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to a DAS file that is
///      currently open, this routine returns without signaling an
///      error.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Normally, routines outside of SPICELIB will not need to call this
///  routine. Application programs should close DAS files by calling
///  the SPICELIB routine DASCLS. This routine is a lower-level
///  routine that is called by DASCLS, but (obviously) does not have
///  the full functionality of DASCLS.
///
///  This routine closes a DAS file and updates the DAS file manager's
///  bookkeeping information on open DAS files. Because the DAS file
///  manager must keep track of which files are open at any given time,
///  it is important that DAS files be closed only with DASCLS or
///  DASLLC, to prevent the remaining DAS routines from failing,
///  sometimes mysteriously.
///
///  Note that when a file is opened more than once for read or write
///  access, DASOPR returns the same handle each time it is re-opened.
///  Each time the file is closed, DASLLC checks to see if any other
///  claims on the file are still active before physically closing
///  the file.
///
///  Unlike DASCLS, this routine does not force a write of updated,
///  buffered records to the indicated file, nor does it segregate the
///  data records in the file.
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
///  1) Write a DAS file by adding data to it over multiple passes.
///     Avoid spending time on file segregation between writes.
///
///     Each pass opens the file, adds character, double precision,
///     and integer data to the file, writes out buffered data by
///     calling DASWBR, and closes the file without segregating the
///     data by calling DASLLC.
///
///     The program also checks the file: after the final write,
///     the program reads the data and compares it to expected values.
///
///     Note that most user-oriented applications should segregate a
///     DAS file after writing it, since this greatly enhances file
///     reading efficiency. The technique demonstrated here may be
///     useful for cases in which a file will be written via many
///     small data additions, and in which the file is read between
///     write operations.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASLLC_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               FTYPLN
///           PARAMETER           ( FTYPLN = 3 )
///
///           INTEGER               CHRLEN
///           PARAMETER           ( CHRLEN = 50 )
///
///           INTEGER               IBUFSZ
///           PARAMETER           ( IBUFSZ = 20 )
///
///           INTEGER               DBUFSZ
///           PARAMETER           ( DBUFSZ = 30 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CHRLEN)    CHRBUF
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(FTYPLN)    FTYPE
///           CHARACTER*(CHRLEN)    XCHRBF
///
///           DOUBLE PRECISION      DPBUF  ( DBUFSZ )
///           DOUBLE PRECISION      XDPBUF ( DBUFSZ )
///
///           INTEGER               FIRSTC
///           INTEGER               FIRSTD
///           INTEGER               FIRSTI
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               INTBUF ( IBUFSZ )
///           INTEGER               J
///           INTEGER               LASTC
///           INTEGER               LASTD
///           INTEGER               LASTI
///           INTEGER               NCALL
///           INTEGER               NCOMR
///           INTEGER               NPASS
///           INTEGER               PASSNO
///           INTEGER               XINTBF ( IBUFSZ )
///
///
///     C
///     C     Initial values
///     C
///           DATA                  FNAME  / 'dasllc_ex1.das' /
///           DATA                  FTYPE  / 'ANG' /
///           DATA                  NCALL  / 1000  /
///           DATA                  NCOMR  / 10    /
///           DATA                  NPASS  / 3     /
///
///     C
///     C     Open a new DAS file. We'll allocate NCOMR records
///     C     for comments. The file type is not one of the standard
///     C     types recognized by SPICE; however it can be used to
///     C     ensure the database file is of the correct type.
///     C
///     C     We'll use the file name as the internal file name.
///     C
///           CALL DASONW ( FNAME, FTYPE, FNAME, NCOMR, HANDLE )
///
///     C
///     C     Add data of character, integer, and double precision
///     C     types to the file in interleaved fashion. We'll add to
///     C     the file over NPASS "passes," in each of which we close
///     C     the file after writing.
///     C
///           DO PASSNO = 1, NPASS
///
///              IF ( PASSNO .GT. 1 ) THEN
///
///                 WRITE (*,*) 'Opening file for write access...'
///
///                 CALL DASOPW( FNAME, HANDLE )
///
///              END IF
///
///              DO I = 1, NCALL
///     C
///     C           Add string data to the file.
///     C
///                 CHRBUF = 'Character value #'
///                 CALL REPMI( CHRBUF, '#', I, CHRBUF )
///
///                 CALL DASADC ( HANDLE, CHRLEN, 1, CHRLEN, CHRBUF )
///
///     C
///     C           Add double precision data to the file.
///     C
///                 DO J = 1, DBUFSZ
///                    DPBUF(J) = DBLE( 100000000*PASSNO + 100*I + J )
///                 END DO
///
///                 CALL DASADD ( HANDLE, DBUFSZ, DPBUF )
///
///     C
///     C           Add integer data to the file.
///     C
///                 DO J = 1, IBUFSZ
///                    INTBUF(J) = 100000000*PASSNO  +  100 * I  +  J
///                 END DO
///
///                 CALL DASADI ( HANDLE, IBUFSZ, INTBUF )
///
///              END DO
///
///     C
///     C        Write buffered data to the file.
///     C
///              WRITE (*,*) 'Writing buffered data...'
///              CALL DASWBR ( HANDLE )
///
///     C
///     C        Close the file without segregating it.
///     C
///              WRITE (*,*) 'Closing DAS file...'
///              CALL DASLLC ( HANDLE )
///
///           END DO
///
///           WRITE (*,*) 'File write is done.'
///
///     C
///     C     Check file contents.
///     C
///           CALL DASOPR( FNAME, HANDLE )
///
///     C
///     C     Read data from the file; compare to expected values.
///     C
///     C     Initialize end addresses.
///     C
///           LASTC = 0
///           LASTD = 0
///           LASTI = 0
///
///           DO PASSNO = 1, NPASS
///
///              DO I = 1, NCALL
///     C
///     C           Check string data.
///     C
///                 XCHRBF = 'Character value #'
///                 CALL REPMI( XCHRBF, '#', I, XCHRBF )
///
///                 FIRSTC = LASTC + 1
///                 LASTC  = LASTC + CHRLEN
///
///                 CALL DASRDC ( HANDLE, FIRSTC, LASTC,
///          .                    1,      CHRLEN, CHRBUF )
///
///                 IF ( CHRBUF .NE. XCHRBF ) THEN
///                    WRITE (*,*) 'Character data mismatch: '
///                    WRITE (*,*) 'PASS     = ', PASSNO
///                    WRITE (*,*) 'I        = ', I
///                    WRITE (*,*) 'Expected = ', XCHRBF
///                    WRITE (*,*) 'Actual   = ', CHRBUF
///                    STOP
///                 END IF
///
///     C
///     C           Check double precision data.
///     C
///                 DO J = 1, DBUFSZ
///                    XDPBUF(J) = DBLE(   100000000*PASSNO
///          .                           + 100*I + J        )
///                 END DO
///
///                 FIRSTD = LASTD + 1
///                 LASTD  = LASTD + DBUFSZ
///
///                 CALL DASRDD ( HANDLE, FIRSTD, LASTD, DPBUF )
///
///                 DO J = 1, DBUFSZ
///
///                    IF ( DPBUF(J) .NE. XDPBUF(J) ) THEN
///
///                       WRITE (*,*)
///          .                      'Double precision data mismatch: '
///                       WRITE (*,*) 'PASS     = ', PASSNO
///                       WRITE (*,*) 'I        = ', I
///                       WRITE (*,*) 'J        = ', J
///                       WRITE (*,*) 'Expected = ', XDPBUF(J)
///                       WRITE (*,*) 'Actual   = ', DPBUF(J)
///                       STOP
///
///                    END IF
///
///                 END DO
///
///     C
///     C           Check integer data.
///     C
///                 DO J = 1, IBUFSZ
///                    XINTBF(J) = 100000000*PASSNO  +  100 * I  +  J
///                 END DO
///
///                 FIRSTI = LASTI + 1
///                 LASTI  = LASTI + IBUFSZ
///
///                 CALL DASRDI ( HANDLE, FIRSTI, LASTI, INTBUF )
///
///                 DO J = 1, IBUFSZ
///
///                    IF ( INTBUF(J) .NE. XINTBF(J) ) THEN
///
///                       WRITE (*,*) 'Integer data mismatch: '
///                       WRITE (*,*) 'PASS     = ', PASSNO
///                       WRITE (*,*) 'I        = ', I
///                       WRITE (*,*) 'J        = ', J
///                       WRITE (*,*) 'Expected = ', XINTBF(J)
///                       WRITE (*,*) 'Actual   = ', INTBUF(J)
///                       STOP
///
///                    END IF
///
///                 END DO
///
///              END DO
///
///           END DO
///
///           WRITE (*,*) 'File check is done.'
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Writing buffered data...
///      Closing DAS file...
///      Opening file for write access...
///      Writing buffered data...
///      Closing DAS file...
///      Opening file for write access...
///      Writing buffered data...
///      Closing DAS file...
///      File write is done.
///      File check is done.
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (NJB) (JDR)
///
///         Updated the header to comply with NAIF standard. Added
///         complete code example.
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.0.3, 10-APR-2014 (NJB)
///
///         Corrected header comments: routine that flushes
///         written, buffered records is DASWBR, not DASWUR.
///
/// -    SPICELIB Version 6.0.2, 21-FEB-2003 (NJB)
///
///         Corrected inline comment: determination of whether file
///         is open is done by searching the handle column of the file
///         table, not the unit column.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dasllc(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DASLLC(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASLLC ( DAS, low-level close )
pub fn DASLLC(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASLLC", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Is this file even open?  Peruse the `handle' column of the file
    // table; see whether this handle is present.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    //
    // If the file is not open: no harm, no foul.  Otherwise, decrement
    // the number of links to the file.  If the number of links drops to
    // zero, physically close the file and remove it from the file
    // buffer.
    //
    if save.FOUND {
        save.FTLNK[save.FINDEX] = (save.FTLNK[save.FINDEX] - 1);

        if (save.FTLNK[save.FINDEX] == 0) {
            //
            // Close this file and delete it from the active list.
            // If this was the head node of the list, the head node
            // becomes the successor of this node (which may be NIL).
            // Delete the handle from our handle set.
            //
            ZZDDHCLS(HANDLE, b"DAS", false, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASLLC", ctx)?;
                return Ok(());
            }

            if (save.FINDEX == save.FTHEAD) {
                save.FTHEAD = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
            }

            LNKFSL(save.FINDEX, save.FINDEX, save.POOL.as_slice_mut(), ctx)?;

            REMOVI(HANDLE, save.FHLIST.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"DASLLC", ctx)?;
    Ok(())
}

/// DAS, handle to file summary
///
/// Return a file summary for a specified DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAS file.
///  NRESVR     O   Number of reserved records in file.
///  NRESVC     O   Number of characters in use in reserved rec. area.
///  NCOMR      O   Number of comment records in file.
///  NCOMC      O   Number of characters in use in comment area.
///  FREE       O   Number of first free record.
///  LASTLA     O   Array of last logical addresses for each data type.
///  LASTRC     O   Record number of last descriptor of each data type.
///  LASTWD     O   Word number of last descriptor of each data type.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file. The file
///           may be open for read or write access.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NRESVR   is the number of reserved records in a specified DAS
///           file.
///
///  NRESVC   is the number of characters in use in the reserved record
///           area of a specified DAS file.
///
///  NCOMR    is the number of comment records in a specified DAS file.
///
///  NCOMC    is the number of characters in use in the comment area of
///           a specified DAS file.
///
///  FREE     is the 1-based record number of the first free record in
///           a specified DAS file.
///
///  LASTLA   is an array containing the highest current 1-based
///           logical addresses, in the specified DAS file, of data of
///           character, double precision, and integer types, in that
///           order.
///
///  LASTRC   is an array containing the 1-based record numbers, in the
///           specified DAS file, of the directory records containing
///           the current last descriptors of clusters of character,
///           double precision, and integer data records, in that
///           order.
///
///  LASTWD   is an array containing the 1-based word indices, within
///           the respective descriptor records identified by the
///           elements of LASTRC, of the current last descriptors of
///           clusters of character, double precision, and integer data
///           records, in that order.
/// ```
///
/// # Parameters
///
/// ```text
///  See INCLUDE file das.inc for declarations and descriptions of
///  parameters used throughout the DAS system.
///
///  CHARDT,
///  DPDT,
///  INTDT    are data type specifiers that indicate CHARACTER,
///           DOUBLE PRECISION, and INTEGER respectively. These
///           parameters are used in all DAS routines that require a
///           data type specifier.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to any file that is
///      currently known to be open, the error SPICE(DASNOSUCHHANDLE)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The quantities
///
///     NRESVR
///     NRESVC
///     NCOMR
///     NCOMC
///     FREE
///     LASTLA
///     LASTRC
///     LASTWD
///
///  define the "state" of a DAS file, and in particular the state of
///  the directory structure of the file. This information is needed by
///  other DAS routines, but application programs will usually have no
///  need for it. The one exception is the array of "last" logical
///  addresses LASTLA: these addresses indicate how many words of data
///  of each type are contained in the specified DAS file. The elements
///  of LASTLA can be conveniently retrieved by calling DASLLA.
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
///  1) Create a DAS file containing 10 integers, 5 double precision
///     numbers, and 4 characters. Print the summary of the file and
///     dump its contents.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASHFS_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'das.inc'
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'dashfs_ex1.das' )
///
///           INTEGER               IFNMLN
///           PARAMETER           ( IFNMLN = 60 )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 2  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(IFNMLN)    IFNAME
///           CHARACTER*(4)         TYPE
///           CHARACTER*(LINLEN)    LINE
///
///           DOUBLE PRECISION      X
///
///           INTEGER               FIRST
///           INTEGER               FREE
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               LAST
///           INTEGER               LASTLA ( 3 )
///           INTEGER               LASTRC ( 3 )
///           INTEGER               LASTWD ( 3 )
///           INTEGER               N
///           INTEGER               NCOMC
///           INTEGER               NCOMR
///           INTEGER               NREAD
///           INTEGER               NRESVC
///           INTEGER               NRESVR
///           INTEGER               REMAIN
///
///     C
///     C     Open a new DAS file. Reserve no records for comments.
///     C
///           TYPE   = 'TEST'
///           IFNAME = 'TEST.DAS/NAIF/NJB/11-NOV-1992-20:12:20'
///
///           CALL DASONW ( FNAME, TYPE, IFNAME, 0, HANDLE )
///
///     C
///     C     Obtain the file summary.
///     C
///           CALL DASHFS ( HANDLE, NRESVR, NRESVC, NCOMR, NCOMC,
///          .              FREE,   LASTLA, LASTRC, LASTWD       )
///
///     C
///     C     Print the summary of the new file.
///     C
///           WRITE(*,*) 'Summary before adding data:'
///           WRITE(*,*) '   Number of reserved records     :', NRESVR
///           WRITE(*,*) '   Characters in reserved records :', NRESVC
///           WRITE(*,*) '   Number of comment records      :', NCOMR
///           WRITE(*,*) '   Characters in comment area     :', NCOMC
///           WRITE(*,*) '   Number of first free record    :', FREE
///           WRITE(*,*) '   Last logical character address :',
///          .                                          LASTLA(CHARDT)
///           WRITE(*,*) '   Last logical d.p. address      :',
///          .                                          LASTLA(DPDT)
///           WRITE(*,*) '   Last logical integer address   :',
///          .                                          LASTLA(INTDT)
///           WRITE(*,*) '   Last character descriptor      :',
///          .                                          LASTRC(CHARDT)
///           WRITE(*,*) '   Last d.p descriptor            :',
///          .                                          LASTRC(DPDT)
///           WRITE(*,*) '   Last integer descriptor        :',
///          .                                          LASTRC(INTDT)
///           WRITE(*,*) '   Character word position in desc:',
///          .                                          LASTWD(CHARDT)
///           WRITE(*,*) '   d.p. word position in desc     :',
///          .                                          LASTWD(DPDT)
///           WRITE(*,*) '   Integer word position in desc  :',
///          .                                          LASTWD(INTDT)
///           WRITE(*,*)
///
///     C
///     C     Add the data.
///     C
///           DO I = 1, 10
///              CALL DASADI ( HANDLE, 1, I )
///           END DO
///
///           DO I = 1, 5
///              CALL DASADD ( HANDLE, 1, DBLE(I) )
///           END DO
///
///     C
///     C     Add character data to the file. DAS character data are
///     C     treated as a character array, not as a string. The
///     C     following call adds only the first 4 characters to the
///     C     DAS file.
///     C
///           CALL DASADC ( HANDLE, 4, 1, 4, 'SPUDWXY' )
///
///     C
///     C     Close the file and open it for reading.
///     C
///           CALL DASCLS ( HANDLE        )
///           CALL DASOPR ( FNAME, HANDLE )
///
///     C
///     C     Obtain again the file summary.
///     C
///           CALL DASHFS ( HANDLE, NRESVR, NRESVC, NCOMR, NCOMC,
///          .              FREE,   LASTLA, LASTRC, LASTWD       )
///
///           WRITE(*,*) 'Summary after adding data:'
///           WRITE(*,*) '   Number of reserved records     :', NRESVR
///           WRITE(*,*) '   Characters in reserved records :', NRESVC
///           WRITE(*,*) '   Number of comment records      :', NCOMR
///           WRITE(*,*) '   Characters in comment area     :', NCOMC
///           WRITE(*,*) '   Number of first free record    :', FREE
///           WRITE(*,*) '   Last logical character address :',
///          .                                          LASTLA(CHARDT)
///           WRITE(*,*) '   Last logical d.p. address      :',
///          .                                          LASTLA(DPDT)
///           WRITE(*,*) '   Last logical integer address   :',
///          .                                          LASTLA(INTDT)
///           WRITE(*,*) '   Last character descriptor      :',
///          .                                          LASTRC(CHARDT)
///           WRITE(*,*) '   Last d.p descriptor            :',
///          .                                          LASTRC(DPDT)
///           WRITE(*,*) '   Last integer descriptor        :',
///          .                                          LASTRC(INTDT)
///           WRITE(*,*) '   Character word position in desc:',
///          .                                          LASTWD(CHARDT)
///           WRITE(*,*) '   d.p. word position in desc     :',
///          .                                          LASTWD(DPDT)
///           WRITE(*,*) '   Integer word position in desc  :',
///          .                                          LASTWD(INTDT)
///           WRITE(*,*)
///
///     C
///     C     Read the integers and dump them.
///     C
///           WRITE(*,*) 'Integer data in the DAS file:'
///           DO I = 1, LASTLA(INTDT)
///              CALL DASRDI ( HANDLE, I, I, N )
///              WRITE (*,*) '   ', N
///           END DO
///
///     C
///     C     Now the d.p. numbers:
///     C
///           WRITE(*,*)
///           WRITE(*,*) 'Double precision data in the DAS file:'
///           DO I = 1, LASTLA(DPDT)
///              CALL DASRDD ( HANDLE, I, I, X )
///              WRITE (*,*) '   ', X
///           END DO
///
///     C
///     C     Now the characters. In this case, we read the
///     C     data one line at a time.
///     C
///           FIRST   =  0
///           LAST    =  0
///           REMAIN  =  LASTLA(CHARDT)
///
///           WRITE(*,*)
///           WRITE(*,*) 'Character data in the DAS file:'
///           DO WHILE ( REMAIN .GT. 0 )
///
///              NREAD = MIN ( LINLEN, REMAIN )
///              FIRST = LAST + 1
///              LAST  = LAST + NREAD
///
///              CALL DASRDC ( HANDLE, FIRST, LAST, 1, LINLEN, LINE )
///
///              WRITE (*,*) '   ', LINE(:NREAD)
///
///              REMAIN = REMAIN - NREAD
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Summary before adding data:
///         Number of reserved records     :           0
///         Characters in reserved records :           0
///         Number of comment records      :           0
///         Characters in comment area     :           0
///         Number of first free record    :           3
///         Last logical character address :           0
///         Last logical d.p. address      :           0
///         Last logical integer address   :           0
///         Last character descriptor      :           0
///         Last d.p descriptor            :           0
///         Last integer descriptor        :           0
///         Character word position in desc:           0
///         d.p. word position in desc     :           0
///         Integer word position in desc  :           0
///
///      Summary after adding data:
///         Number of reserved records     :           0
///         Characters in reserved records :           0
///         Number of comment records      :           0
///         Characters in comment area     :           0
///         Number of first free record    :           6
///         Last logical character address :           4
///         Last logical d.p. address      :           5
///         Last logical integer address   :          10
///         Last character descriptor      :           2
///         Last d.p descriptor            :           2
///         Last integer descriptor        :           2
///         Character word position in desc:          10
///         d.p. word position in desc     :          11
///         Integer word position in desc  :          12
///
///      Integer data in the DAS file:
///                    1
///                    2
///                    3
///                    4
///                    5
///                    6
///                    7
///                    8
///                    9
///                   10
///
///      Double precision data in the DAS file:
///            1.0000000000000000
///            2.0000000000000000
///            3.0000000000000000
///            4.0000000000000000
///            5.0000000000000000
///
///      Character data in the DAS file:
///         SP
///         UD
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.0.2, 19-JUL-2021 (JDR)
///
///         Updated the header to comply with NAIF standard. Added
///         complete code example.
///
///         Described the public parameters used together with this module
///         in the $Parameters section.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUL-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUL-1992 (NJB) (WLT)
/// ```
pub fn dashfs(
    ctx: &mut SpiceContext,
    handle: i32,
    nresvr: &mut i32,
    nresvc: &mut i32,
    ncomr: &mut i32,
    ncomc: &mut i32,
    free: &mut i32,
    lastla: &mut [i32; 3],
    lastrc: &mut [i32; 3],
    lastwd: &mut [i32; 3],
) -> crate::Result<()> {
    DASHFS(
        handle,
        nresvr,
        nresvc,
        ncomr,
        ncomc,
        free,
        lastla,
        lastrc,
        lastwd,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASHFS ( DAS, handle to file summary )
pub fn DASHFS(
    HANDLE: i32,
    NRESVR: &mut i32,
    NRESVC: &mut i32,
    NCOMR: &mut i32,
    NCOMC: &mut i32,
    FREE: &mut i32,
    LASTLA: &mut [i32],
    LASTRC: &mut [i32],
    LASTWD: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut LASTLA = DummyArrayMut::new(LASTLA, 1..=3);
    let mut LASTRC = DummyArrayMut::new(LASTRC, 1..=3);
    let mut LASTWD = DummyArrayMut::new(LASTWD, 1..=3);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASHFS", ctx)?;
    }

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if save.FOUND {
        //
        // Give the caller the current summary from the file table.
        //
        *NRESVR = save.FTSUM[[RRCIDX, save.FINDEX]];
        *NRESVC = save.FTSUM[[RCHIDX, save.FINDEX]];
        *NCOMR = save.FTSUM[[CRCIDX, save.FINDEX]];
        *NCOMC = save.FTSUM[[CCHIDX, save.FINDEX]];
        *FREE = save.FTSUM[[FREIDX, save.FINDEX]];

        for I in 1..=3 {
            LASTLA[I] = save.FTSUM[[(LLABAS + I), save.FINDEX]];
            LASTRC[I] = save.FTSUM[[(LRCBAS + I), save.FINDEX]];
            LASTWD[I] = save.FTSUM[[(LWDBAS + I), save.FINDEX]];
        }
    } else {
        SETMSG(b"There is no DAS file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DASNOSUCHHANDLE)", ctx)?;
    }

    CHKOUT(b"DASHFS", ctx)?;
    Ok(())
}

/// DAS, update file summary
///
/// Update the file summary in a specified DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of an open DAS file.
///  NRESVR     I   Number of reserved records in file.
///  NRESVC     I   Number of characters in use in reserved rec. area.
///  NCOMR      I   Number of comment records in file.
///  NCOMC      I   Number of characters in use in comment area.
///  FREE       I   Number of first free record.
///  LASTLA     I   Array of last logical addresses for each data type.
///  LASTRC     I   Record number of last descriptor of each data type.
///  LASTWD     I   Word number of last descriptor of each data type.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file.
///
///  NRESVR   is the number of reserved records in a specified DAS
///           file.
///
///  NRESVC   is the number of characters in use in the reserved
///           record area of a specified DAS file.
///
///  NCOMR    is the number of comment records in a specified DAS
///           file.
///
///  NCOMC    is the number of characters in use in the comment area
///           of a specified DAS file.
///
///  FREE     is the Fortran record number of the first free record
///           in a specified DAS file.
///
///  LASTLA   is an array containing the highest current logical
///           addresses, in the specified DAS file, of data of
///           character, double precision, and integer types, in
///           that order.
///
///  LASTRC   is an array containing the Fortran record numbers, in
///           the specified DAS file, of the directory records
///           containing the current last descriptors of clusters
///           of character, double precision, and integer data
///           records, in that order.
///
///  LASTWD   is an array containing the word positions, in the
///           specified DAS file, of the current last descriptors
///           of clusters of character, double precision, and
///           integer data records, in that order.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to any file that is
///      currently known to be open, the error SPICE(DASNOSUCHHANDLE)
///      is signaled.
///
///  2)  If the specified handle is not open for WRITE access, the
///      error SPICE(DASINVALIDACCESS) is signaled.
///
///  3)  If this routine's attempts to read the DAS file record
///      fail before an update, the error SPICE(DASREADFAIL) is
///      signaled.
///
///  4)  If the attempt to write to the DAS file record fails, the
///      error SPICE(DASWRITEFAIL) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  The quantities NRESVR, NRESRC, NCOMR, NCOMC, FREE, LASTLA,
///  LASTRC, and LASTWD define the `state' of a DAS file, and in
///  particular the state of the directory structure of the file.
///  These quantities should normally be updated only by DAS routines.
///
///  The higher-level DAS routines that affect a DAS file's summary,
///  such as
///
///     DASADx
///     DASUDx
///     DASARR
///
///  automatically update the file summary, so there is no need for
///  the calling program to perform the update explicitly.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update the last d.p. logical address for a DAS file, leaving
///      the rest of the file summary intact.
///
///         C
///         C     Read the file summary.
///         C
///               CALL DASHFS ( HANDLE,
///              .              NRESVR,
///              .              RRESVC,
///              .              NCOMR,
///              .              NCOMC,
///              .              FREE,
///              .              LASTLA,
///              .              LASTRC,
///              .              LASTWD )
///
///         C
///         C     Update the d.p. component of the `last logical
///         C     address' array.
///         C
///               LASTLA(DP) = NEWVAL
///
///               CALL DASUFS ( HANDLE,
///              .              NRESVR,
///              .              RRESVC,
///              .              NCOMR,
///              .              NCOMC,
///              .              FREE,
///              .              LASTLA,
///              .              LASTRC,
///              .              LASTWD )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.1.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 7.1.0, 07-OCT-2015 (NJB)
///
///         Corrected call to ZZDDHCLS.
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///
/// -    SPICELIB Version 6.1.0, 26-SEP-2005 (NJB)
///
///         Bug fix: file name is now correctly inserted into long
///         error message generated when target file is not open for
///         write access.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 6.0.0, 15-OCT-2001 (FST) (NJB)
///
///         Bug fix: this routine now reads the file record
///         before attempting to update it. The buffered values
///         of IDWORD and IFN are no longer present.
///
///         Bug fix: missing call to CHKIN was added to an error
///         handling branch in entry point DASUFS. This call is
///         required because DASUFS uses discovery check-in.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUL-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.1.0, 26-SEP-2005 (NJB)
///
///         Bug fix: file name is now correctly inserted into long
///         error message generated when target file is not open for
///         write access.
///
/// -    SPICELIB Version 5.1.0, 15-OCT-2001 (NJB)
///
///         Bug fix: missing call to CHKIN was added to an error
///         handling branch in entry point DASUFS. This call is
///         required because DASUFS uses discovery check-in.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUL-1992 (NJB) (WLT)
/// ```
pub fn dasufs(
    ctx: &mut SpiceContext,
    handle: i32,
    nresvr: i32,
    nresvc: i32,
    ncomr: i32,
    ncomc: i32,
    free: i32,
    lastla: &[i32; 3],
    lastrc: &[i32; 3],
    lastwd: &[i32; 3],
) -> crate::Result<()> {
    DASUFS(
        handle,
        nresvr,
        nresvc,
        ncomr,
        ncomc,
        free,
        lastla,
        lastrc,
        lastwd,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASUFS ( DAS, update file summary )
pub fn DASUFS(
    HANDLE: i32,
    NRESVR: i32,
    NRESVC: i32,
    NCOMR: i32,
    NCOMC: i32,
    FREE: i32,
    LASTLA: &[i32],
    LASTRC: &[i32],
    LASTWD: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LASTLA = DummyArray::new(LASTLA, 1..=3);
    let LASTRC = DummyArray::new(LASTRC, 1..=3);
    let LASTWD = DummyArray::new(LASTWD, 1..=3);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASUFS", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Find the file table entries for this file.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if save.FOUND {
        //
        // Obtain a logical unit for this file.
        //
        ZZDDHHLU(HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASUFS", ctx)?;
            return Ok(());
        }

        //
        // Now check to see that HANDLE is open for write, as one has
        // no business updating a file summary for files that are
        // open for read access only.
        //
        if (save.FTACC[save.FINDEX] != WRITE) {
            SETMSG(
                b"DAS file not open for writing. Handle = #, file = \'#\'.",
                ctx,
            );
            ERRINT(b"#", HANDLE, ctx);
            ERRFNM(b"#", save.NUMBER, ctx)?;
            SIGERR(b"SPICE(DASINVALIDACCESS)", ctx)?;
            CHKOUT(b"DASUFS", ctx)?;
            return Ok(());
        }

        //
        // If any of the counts pertaining to the reserved record are or
        // the comment area were changed, we need to record the new
        // counts in the file record.  Otherwise, leave the file alone.
        //
        if ((((NRESVR != save.FTSUM[[RRCIDX, save.FINDEX]])
            || (NRESVC != save.FTSUM[[RCHIDX, save.FINDEX]]))
            || (NCOMR != save.FTSUM[[CRCIDX, save.FINDEX]]))
            || (NCOMC != save.FTSUM[[CCHIDX, save.FINDEX]]))
        {
            //
            // Read the file record.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(save.NUMBER)?, Some(1))?;
                save.IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut save.IDWORD)?;
                    reader.read_str(&mut save.LOCIFN)?;
                    save.LOCRRC = reader.read_i32()?;
                    save.LOCRCH = reader.read_i32()?;
                    save.LOCCRC = reader.read_i32()?;
                    save.LOCCCH = reader.read_i32()?;
                    reader.read_str(&mut save.LOCFMT)?;
                    reader.read_str(&mut save.TAIL)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (save.IOSTAT != 0) {
                SETMSG(b"Attempt to read file record failed. File was \'#\'.  Value of IOSTAT was \'#\'.", ctx);
                ERRFNM(b"#", save.NUMBER, ctx)?;
                ERRINT(b"#", save.IOSTAT, ctx);
                SIGERR(b"SPICE(DASREADFAIL)", ctx)?;
                CHKOUT(b"DASUFS", ctx)?;
                return Ok(());
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(save.NUMBER)?, Some(1))?;
                save.IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&save.IDWORD)?;
                    writer.write_str(&save.LOCIFN)?;
                    writer.write_i32(NRESVR)?;
                    writer.write_i32(NRESVC)?;
                    writer.write_i32(NCOMR)?;
                    writer.write_i32(NCOMC)?;
                    writer.write_str(&save.LOCFMT)?;
                    writer.write_str(&save.TAIL)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (save.IOSTAT != 0) {
                //
                // Try to obtain the DAS file's name.
                //
                {
                    use f2rust_std::io;

                    let specs = io::InquireSpecs {
                        unit: Some(save.NUMBER),
                        name: Some(&mut save.LOCDAS),
                        ..Default::default()
                    };
                    save.INQSTA = io::capture_iostat(|| ctx.inquire(specs))?;
                }

                ZZDDHCLS(HANDLE, b"DAS", false, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"DASUFS", ctx)?;
                    return Ok(());
                }

                SETMSG(b"Attempt to update file record failed. File was \'#\'.  Value of IOSTAT was \'#\'.", ctx);
                ERRCH(b"#", &save.LOCDAS, ctx);
                ERRINT(b"#", save.IOSTAT, ctx);
                SIGERR(b"SPICE(DASWRITEFAIL)", ctx)?;
                CHKOUT(b"DASUFS", ctx)?;
                return Ok(());
            }
        }

        //
        // Update the file table.
        //
        save.FTSUM[[RRCIDX, save.FINDEX]] = NRESVR;
        save.FTSUM[[RCHIDX, save.FINDEX]] = NRESVC;
        save.FTSUM[[CRCIDX, save.FINDEX]] = NCOMR;
        save.FTSUM[[CCHIDX, save.FINDEX]] = NCOMC;
        save.FTSUM[[FREIDX, save.FINDEX]] = FREE;

        for I in 1..=3 {
            save.FTSUM[[(LLABAS + I), save.FINDEX]] = LASTLA[I];
            save.FTSUM[[(LRCBAS + I), save.FINDEX]] = LASTRC[I];
            save.FTSUM[[(LWDBAS + I), save.FINDEX]] = LASTWD[I];
        }
    } else {
        SETMSG(b"There is no file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DASNOSUCHHANDLE)", ctx)?;
        CHKOUT(b"DASUFS", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DASUFS", ctx)?;
    Ok(())
}

/// DAS, handle to logical unit
///
/// Return the logical unit associated with a handle. The unit
/// is "locked" to the handle by the DAF/DAS handle manager
/// subsystem.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAS file.
///  UNIT       O   Corresponding logical unit.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file.
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
///  1)  If the specified handle does not belong to any file that is
///      currently known to be open, the error SPICE(DASNOSUCHHANDLE)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility used by the DAS system to support
///  file I/O. DASHLU may also prove useful to general SPICELIB
///  users for constructing error messages.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Obtain the logical unit associated with a DAS file having
///      a known handle.
///
///         CALL DASHLU ( HANDLE, UNIT )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Successfully invoking this module has the side effect of
///      locking UNIT to HANDLE. This 'lock' guarantees until
///      HANDLE is closed (or unlocked) that the file associated
///      with HANDLE is always open and attached to logical unit
///      UNIT. To unlock a handle without closing the file, use
///      ZZDDHUNL, an entry point in the handle manager umbrella,
///      ZZDDHMAN.
///
///      The system can lock at most UTSIZE-SCRUNT-RSVUNT
///      simultaneously (see the include file 'zzddhman.inc' for
///      specific values of these parameters), but unnecessarily
///      locking handles to their logical units may cause performance
///      degradation. The handle manager will have fewer logical
///      units to utilize when disconnecting and reconnecting
///      loaded files.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 7.0.0, 04-FEB-2015 (NJB) (FST)
///
///         Now uses DAF/DAS handle manager subsystem.
///         Note that this routine is now considered obsolete.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dashlu(ctx: &mut SpiceContext, handle: i32, unit: &mut i32) -> crate::Result<()> {
    DASHLU(handle, unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASHLU ( DAS, handle to logical unit )
pub fn DASHLU(HANDLE: i32, UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // We use discovery check-ins in this routine.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        CHKIN(b"DASHLU", ctx)?;

        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        CHKOUT(b"DASHLU", ctx)?;

        save.PASS1 = false;
    }

    //
    // Find the file table entries for this file.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if save.FOUND {
        //
        // For backward compatibility, the logical unit must be
        // locked to the file.
        //
        ZZDDHHLU(HANDLE, b"DAS", true, UNIT, ctx)?;
    } else {
        CHKIN(b"DASHLU", ctx)?;
        SETMSG(b"There is no file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DASNOSUCHHANDLE)", ctx)?;
        CHKOUT(b"DASHLU", ctx)?;
    }

    Ok(())
}

/// DAS, logical unit to handle
///
/// Return the handle associated with a logical unit.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Logical unit connected to a DAS file.
///  HANDLE     O   Corresponding handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the logical unit to which a DAS file has been
///           connected when it was opened.
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
///  1)  If the specified unit is not connected to any DAS file that is
///      currently known to be open, the error SPICE(DASNOSUCHUNIT)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  It is unlikely, but possible, that a calling program would know
///  the logical unit to which a file is connected without knowing the
///  handle associated with the file. DASLUH is provided mostly for
///  completeness.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the handle associated with
///  a DAS file is retrieved using the logical unit to which the
///  file is connected. The handle is then used to determine the
///  name of the file.
///
///     CALL DASLUH ( UNIT,   HANDLE )
///     CALL DASHFN ( HANDLE, FNAME  )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///         Note that this routine is now considered obsolete.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dasluh(ctx: &mut SpiceContext, unit: i32, handle: &mut i32) -> crate::Result<()> {
    DASLUH(unit, handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASLUH ( DAS, logical unit to handle )
pub fn DASLUH(UNIT: i32, HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASLUH", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Try to locate the handle associated with this unit.
    //
    ZZDDHLUH(UNIT, HANDLE, &mut save.FOUND, ctx)?;

    if !save.FOUND {
        SETMSG(b"There is no DAS file open with unit = #", ctx);
        ERRINT(b"#", UNIT, ctx);
        SIGERR(b"SPICE(DASNOSUCHUNIT)", ctx)?;
    }

    CHKOUT(b"DASLUH", ctx)?;
    Ok(())
}

/// DAS, handle to file name
///
/// Return the name of the DAS file associated with a handle.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAS file.
///  FNAME      O   Corresponding file name.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FNAME    is the name of the DAS file associated with the input
///           file handle.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified handle does not belong to any file that is
///      currently known to be open, the error SPICE(DASNOSUCHHANDLE)
///      is signaled.
///
///  2)  If FNAME string is too short to contain the output string,
///      the name is truncated on the right.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  It may be desirable to recover the names of one or more DAS
///  files in a different part of the program from the one in which
///  they were opened. Note that the names returned by DASHFN may
///  not be identical to the names used to open the files. Under
///  most operating systems, a particular file can be accessed using
///  many different names. DASHFN returns one of them.
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
///  1) In the following program, the name of a DAS file is
///     recovered using the handle associated with the file.
///
///
///     Use the DSK kernel below as input DAS file for the example.
///
///        phobos512.bds
///
///
///     Example code begins here.
///
///
///           PROGRAM DASHFN_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local constants
///     C
///           CHARACTER*(*)         DAS
///           PARAMETER           ( DAS    = 'phobos512.bds' )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 256 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    FNAME
///
///           INTEGER               HANDLE
///
///     C
///     C     Open the DAS file for read access.
///     C
///           CALL DASOPR ( DAS, HANDLE )
///
///     C
///     C     Map the handle to a file name.
///     C
///           CALL DASHFN ( HANDLE, FNAME )
///
///           WRITE(*,*) 'DAS file name = <', FNAME(:RTRIM(FNAME)),
///          .           '>.'
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      DAS file name = <phobos512.bds>.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Extended $Exceptions section to describe what
///         happens in case of output string being too short.
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///         Note that this routine is now considered obsolete.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dashfn(ctx: &mut SpiceContext, handle: i32, fname: &mut str) -> crate::Result<()> {
    DASHFN(
        handle,
        fstr::StrBytes::new(fname).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASHFN ( DAS, handle to file name )
pub fn DASHFN(HANDLE: i32, FNAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASHFN", ctx)?;
    }

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Find the file table entries for this file.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if save.FOUND {
        ZZDDHHLU(HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::InquireSpecs {
                unit: Some(save.NUMBER),
                name: Some(FNAME),
                ..Default::default()
            };
            ctx.inquire(specs)?;
        }
    } else {
        SETMSG(b"There is no DAS file open with handle = #", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DASNOSUCHHANDLE)", ctx)?;
    }

    CHKOUT(b"DASHFN", ctx)?;
    Ok(())
}

/// DAS, file name to handle
///
/// Return handle associated with a file name.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DAS file.
///  HANDLE     O   Corresponding handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a previously opened DAS file.
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
///  1)  If the specified name does not specify any DAS file currently
///      known to be open, the error SPICE(DASNOSUCHFILE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  It is sometimes easier to work with file names (which are
///  meaningful, and often predictable) than with file handles
///  (which are neither), especially in interactive situations.
///  However, nearly every DAS routine requires that you use file
///  handles to refer to files. DASFNH is provided to bridge the gap
///  between the two representations.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the handle associated with a
///  DAS file is recovered using the name of the file.
///
///     CALL DASOPR ( 'sample.DAS', HANDLE )
///      .
///      .
///
///     CALL DASFNH ( 'sample.DAS', HANDLE )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 7.0.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 7.0.0, 30-JUL-2014 (NJB)
///
///         Now uses DAF/DAS handle manager subsystem.
///         Note that this routine is now considered obsolete.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dasfnh(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    DASFNH(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASFNH ( DAS, file name to handle )
pub fn DASFNH(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASFNH", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    ZZDDHFNH(FNAME, HANDLE, &mut save.FOUND, ctx)?;

    if !save.FOUND {
        SETMSG(
            b"There is no DAS file in the table with file name = \'#\'",
            ctx,
        );
        ERRCH(b"#", FNAME, ctx);
        SIGERR(b"SPICE(DASNOSUCHFILE)", ctx)?;
    }

    CHKOUT(b"DASFNH", ctx)?;
    Ok(())
}

/// DAS, handles of open files
///
/// Return a SPICE set containing the handles of all currently
/// open DAS files.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FHSET      O   A set containing handles of currently open DAS
///                 files.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FHSET    is a SPICE set containing the file handles of
///           all currently open DAS files.
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
///  This routine allows subroutines to test DAS file handles for
///  validity before using them. Many DAS operations that rely on
///  handles to identify DAS files cause errors to be signaled if
///  the handles are invalid.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Find out how may DAS files are open for writing.
///
///         C
///         C    Find out which DAS files are open.
///         C
///              CALL DASHOF  ( FHSET )
///
///         C
///         C    Count the ones open for writing.
///         C
///              COUNT = 0
///
///              DO I = 1, CARDC(FHSET)
///
///                 CALL DASHAM ( FHSET(I), METHOD )
///
///                 IF ( METHOD .EQ. WRITE ) THEN
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
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.0.2, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dashof(ctx: &mut SpiceContext, fhset: &mut [i32]) -> crate::Result<()> {
    DASHOF(fhset, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASHOF ( DAS, handles of open files )
pub fn DASHOF(FHSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut FHSET = DummyArrayMut::new(FHSET, LBCELL..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASHOF", ctx)?;
    }

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Just stuff our local list into the set.
    //
    COPYI(save.FHLIST.as_slice(), FHSET.as_slice_mut(), ctx)?;

    CHKOUT(b"DASHOF", ctx)?;
    Ok(())
}

/// DAS, signal invalid handles
///
/// Signal an error if a DAS file file handle does not designate a
/// DAS file that is open for a specified type of access.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
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
///  HANDLE   is a DAS file handle to validate. For HANDLE to be
///           considered valid, it must specify a DAS file that
///           is open for the type of access specified by the
///           input argument ACCESS.
///
///
///  ACCESS   is a string indicating the type of access that
///           the DAS file specified by the input argument HANDLE
///           must be open for. The values of ACCESS may be
///
///              'READ'      File must be open for read access
///                          by DAS routines. DAS files opened
///                          for read or write access may be
///                          read.
///
///              'WRITE'     File must be open for write access
///                          by DAS routines. Note that files
///                          open for write access may be read as
///                          well as written.
///
///           Leading and trailing blanks in ACCESS are ignored,
///           and case is not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
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
///  This routine signals the error SPICE(DASINVALIDACCESS) if the
///  DAS designated by the input argument HANDLE is not open
///  for the specified type of access. If HANDLE does not designate
///  an open DAS file, the error SPICE(DASNOSUCHHANDLE) is signaled.
///
///  This routine allows subroutines to test file handles for
///  validity before attempting to access the files they designate,
///  or before performing operations on the handles themselves, such
///  as finding the name of the file designated by a handle. This
///  routine should be used in situations where the appropriate action
///  to take upon determining that a handle is invalid is to signal
///  an error. DASSIH centralizes the error response for this type of
///  error in a single routine.
///
///  In cases where it is necessary to determine the validity of a
///  file handle, but it is not an error for the handle to refer
///  to a closed file, the entry point DASHOF should be used instead
///  of DASSIH.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Make sure that a file handle designates a DAS file that can
///      be read. Signal an error if not.
///
///      Note that if a DAS file is open for reading or writing, read
///      access is allowed.
///
///               CALL DASSIH ( HANDLE, 'READ' )
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
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.1.1, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.1.0, 26-SEP-2005 (NJB)
///
///         Local variable DAS was renamed to DASFIL. This
///         was done to avoid future conflict with parameters
///         in zzddhman.inc.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 6.1.0, 26-SEP-2005 (NJB)
///
///         Local variable DAS was renamed to DASFIL. This
///         was done to avoid future conflict with parameters
///         in zzddhman.inc.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if these open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT) (IMU)
/// ```
pub fn dassih(ctx: &mut SpiceContext, handle: i32, access: &str) -> crate::Result<()> {
    DASSIH(handle, access.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASSIH ( DAS, signal invalid handles )
pub fn DASSIH(HANDLE: i32, ACCESS: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASSIH", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
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
        CHKOUT(b"DASSIH", ctx)?;
        return Ok(());
    }

    //
    // See whether the input handle is in our list at all.  It's
    // unlawful for the handle to be absent.
    //
    if !ELEMI(HANDLE, save.FHLIST.as_slice(), ctx)? {
        SETMSG(b"Handle # is not attached to an open DAS file.", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(DASNOSUCHHANDLE)", ctx)?;
        CHKOUT(b"DASSIH", ctx)?;
        return Ok(());
    } else {
        //
        // Find the file table entries for this file.  We know they
        // must exist.
        //
        save.FINDEX = save.FTHEAD;
        save.FOUND = false;

        while (!save.FOUND && (save.FINDEX > 0)) {
            if (save.FTHAN[save.FINDEX] == HANDLE) {
                save.FOUND = true;
            } else {
                save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
            }
        }
        //
        // At this point, FINDEX points to the file table entries
        // for this file.
        //

        if (fstr::eq(&save.ACC, b"WRITE") && (save.FTACC[save.FINDEX] != WRITE)) {
            //
            // If the access type is 'WRITE', the DAS file must be open
            // for writing.
            //
            ZZDDHHLU(HANDLE, b"DAS", false, &mut save.NUMBER, ctx)?;

            SETMSG(
                b"DAS file not open for writing. Handle = #, file = \'#\'.",
                ctx,
            );
            ERRINT(b"#", HANDLE, ctx);
            ERRFNM(b"#", save.NUMBER, ctx)?;
            SIGERR(b"SPICE(DASINVALIDACCESS)", ctx)?;
            CHKOUT(b"DASSIH", ctx)?;
            return Ok(());
        }
    }

    //
    // The DAS file's handle is o.k.
    //
    CHKOUT(b"DASSIH", ctx)?;
    Ok(())
}

/// DAS, handle to access method
///
/// Return the allowed access method for a specified DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   HANDLE of a DAS file.
///  ACCESS     O   String indicating allowed access method.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a previously opened DAS file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ACCESS   is a string indicating the type of access that
///           the DAS file specified by the input argument HANDLE
///           is open for. The values of ACCESS may be
///
///              'READ'      File is open for read access by DAS
///                          routines. Both the data area and
///                          the comment area may be read. The
///                          file may not be modified.
///
///              'WRITE'     File is open for write access by
///                          DAS routines. Files open for
///                          write access may be read as well as
///                          written.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is invalid, the error SPICE(INVALIDHANDLE)
///      is signaled. ACCESS is not modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows subroutines to determine the access methods
///  allowed for a given DAS file.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Make sure that a file handle designates a DAS file that can
///      be read. Signal an error if not.
///
///      Note that if a DAS file is open for reading or writing, read
///      access is allowed.
///
///               CALL DASHAM ( HANDLE, 'READ' )
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
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 6.0.2, 19-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 6.0.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 5.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 5.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 5.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 5.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input and $ Output sections of the header. This was
///         done in order to minimize documentation changes if these open
///         routines ever change.
///
/// -    SPICELIB Version 1.0.0, 01-FEB-1993 (NJB) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 01-NOV-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input and $ Output sections of the header. This was
///         done in order to minimize documentation changes if these open
///         routines ever change.
///
/// -    SPICELIB Version 1.0.0, 01-FEB-1993 (NJB) (WLT) (IMU)
/// ```
pub fn dasham(ctx: &mut SpiceContext, handle: i32, access: &mut str) -> crate::Result<()> {
    DASHAM(
        handle,
        fstr::StrBytes::new(access).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASHAM ( DAS, handle to access method )
pub fn DASHAM(HANDLE: i32, ACCESS: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASHAM", ctx)?;

    //
    // Initialize the file table pool and handle list, if necessary.
    //
    if save.PASS1 {
        LNKINI(FTSIZE, save.POOL.as_slice_mut(), ctx)?;
        SSIZEI(FTSIZE, save.FHLIST.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // See whether the input handle is in our list at all.  It's
    // unlawful for the handle to be absent.
    //
    save.FINDEX = save.FTHEAD;
    save.FOUND = false;

    while (!save.FOUND && (save.FINDEX > 0)) {
        if (save.FTHAN[save.FINDEX] == HANDLE) {
            save.FOUND = true;
        } else {
            save.FINDEX = LNKNXT(save.FINDEX, save.POOL.as_slice(), ctx)?;
        }
    }

    if !save.FOUND {
        SETMSG(b"The handle # does not designate a known DAS file ", ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(INVALIDHANDLE)", ctx)?;
        CHKOUT(b"DASHAM", ctx)?;
        return Ok(());
    }

    //
    // We know about the file if we got this far.  Set the output
    // argument accordingly.
    //
    if (save.FTACC[save.FINDEX] == READ) {
        fstr::assign(ACCESS, b"READ");
    } else {
        fstr::assign(ACCESS, b"WRITE");
    }

    CHKOUT(b"DASHAM", ctx)?;
    Ok(())
}
