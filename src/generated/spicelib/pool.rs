//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const MAXVAR: i32 = 26003;
const MAXVAL: i32 = 400000;
const MAXLIN: i32 = 15000;
const MAXCHR: i32 = 80;
const MAXLEN: i32 = 32;
const MAXAGT: i32 = 1000;
const MXNOTE: i32 = (MAXVAR * 5);
const LBCELL: i32 = -5;
const LBPOOL: i32 = -5;
const QUOTE: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"\'");
const LNSIZE: i32 = 132;
const PREV: i32 = 2;
const NEXT: i32 = 1;
const MRKLEN: i32 = 10;

struct SaveVars {
    BEGDAT: Vec<u8>,
    BEGTXT: Vec<u8>,
    PNAMES: ActualCharArray,
    NAMLST: ActualArray<i32>,
    NMPOOL: ActualArray2D<i32>,
    DATLST: ActualArray<i32>,
    CHPOOL: ActualArray2D<i32>,
    DPPOOL: ActualArray2D<i32>,
    CHVALS: ActualCharArray,
    DPVALS: ActualArray<f64>,
    WTVARS: ActualCharArray,
    WTPOOL: ActualArray2D<i32>,
    WTAGNT: ActualCharArray,
    WTPTRS: ActualArray<i32>,
    ACTIVE: ActualCharArray,
    AGENTS: ActualCharArray,
    NOTIFY: ActualCharArray,
    FIRST: bool,
    SUBCTR: StackArray<i32, 2>,
    FINISH: Vec<u8>,
    CVALUE: Vec<u8>,
    LINE: Vec<u8>,
    VARNAM: Vec<u8>,
    BIG: f64,
    DVALUE: f64,
    SMALL: f64,
    AGNODE: i32,
    AVAIL: i32,
    BEGIN: i32,
    CHNODE: i32,
    CODE: i32,
    DATAHD: i32,
    DNODE: i32,
    DPNODE: i32,
    FREE: i32,
    HEAD: i32,
    HITS: i32,
    I: i32,
    IOSTAT: i32,
    IQUOTE: i32,
    J: i32,
    K: i32,
    LINNUM: i32,
    LOOKAT: i32,
    MARGIN: i32,
    NAMEAT: i32,
    NEED: i32,
    NFETCH: i32,
    NNODE: i32,
    NNODES: i32,
    NODE: i32,
    NPTRS: i32,
    NVARS: i32,
    NW: i32,
    R: i32,
    SPACE: i32,
    TAIL: i32,
    TOFREE: i32,
    VARLEN: i32,
    CHR: bool,
    DP: bool,
    EOF: bool,
    GOTIT: bool,
    NOAGNT: bool,
    SUCCES: bool,
    VECTOR: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGDAT = vec![b' '; MRKLEN as usize];
        let mut BEGTXT = vec![b' '; MRKLEN as usize];
        let mut PNAMES = ActualCharArray::new(MAXLEN, 1..=MAXVAR);
        let mut NAMLST = ActualArray::<i32>::new(1..=MAXVAR);
        let mut NMPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXVAR);
        let mut DATLST = ActualArray::<i32>::new(1..=MAXVAR);
        let mut CHPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXLIN);
        let mut DPPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MAXVAL);
        let mut CHVALS = ActualCharArray::new(MAXCHR, 1..=MAXLIN);
        let mut DPVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut WTVARS = ActualCharArray::new(MAXLEN, LBCELL..=MAXVAR);
        let mut WTPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MXNOTE);
        let mut WTAGNT = ActualCharArray::new(MAXLEN, 1..=MXNOTE);
        let mut WTPTRS = ActualArray::<i32>::new(1..=MAXVAR);
        let mut ACTIVE = ActualCharArray::new(MAXLEN, LBCELL..=MXNOTE);
        let mut AGENTS = ActualCharArray::new(MAXLEN, LBCELL..=MXNOTE);
        let mut NOTIFY = ActualCharArray::new(MAXLEN, LBCELL..=MXNOTE);
        let mut FIRST: bool = false;
        let mut SUBCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FINISH = vec![b' '; 2 as usize];
        let mut CVALUE = vec![b' '; LNSIZE as usize];
        let mut LINE = vec![b' '; LNSIZE as usize];
        let mut VARNAM = vec![b' '; MAXLEN as usize];
        let mut BIG: f64 = 0.0;
        let mut DVALUE: f64 = 0.0;
        let mut SMALL: f64 = 0.0;
        let mut AGNODE: i32 = 0;
        let mut AVAIL: i32 = 0;
        let mut BEGIN: i32 = 0;
        let mut CHNODE: i32 = 0;
        let mut CODE: i32 = 0;
        let mut DATAHD: i32 = 0;
        let mut DNODE: i32 = 0;
        let mut DPNODE: i32 = 0;
        let mut FREE: i32 = 0;
        let mut HEAD: i32 = 0;
        let mut HITS: i32 = 0;
        let mut I: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut IQUOTE: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut LINNUM: i32 = 0;
        let mut LOOKAT: i32 = 0;
        let mut MARGIN: i32 = 0;
        let mut NAMEAT: i32 = 0;
        let mut NEED: i32 = 0;
        let mut NFETCH: i32 = 0;
        let mut NNODE: i32 = 0;
        let mut NNODES: i32 = 0;
        let mut NODE: i32 = 0;
        let mut NPTRS: i32 = 0;
        let mut NVARS: i32 = 0;
        let mut NW: i32 = 0;
        let mut R: i32 = 0;
        let mut SPACE: i32 = 0;
        let mut TAIL: i32 = 0;
        let mut TOFREE: i32 = 0;
        let mut VARLEN: i32 = 0;
        let mut CHR: bool = false;
        let mut DP: bool = false;
        let mut EOF: bool = false;
        let mut GOTIT: bool = false;
        let mut NOAGNT: bool = false;
        let mut SUCCES: bool = false;
        let mut VECTOR: bool = false;

        FIRST = true;

        Self {
            BEGDAT,
            BEGTXT,
            PNAMES,
            NAMLST,
            NMPOOL,
            DATLST,
            CHPOOL,
            DPPOOL,
            CHVALS,
            DPVALS,
            WTVARS,
            WTPOOL,
            WTAGNT,
            WTPTRS,
            ACTIVE,
            AGENTS,
            NOTIFY,
            FIRST,
            SUBCTR,
            FINISH,
            CVALUE,
            LINE,
            VARNAM,
            BIG,
            DVALUE,
            SMALL,
            AGNODE,
            AVAIL,
            BEGIN,
            CHNODE,
            CODE,
            DATAHD,
            DNODE,
            DPNODE,
            FREE,
            HEAD,
            HITS,
            I,
            IOSTAT,
            IQUOTE,
            J,
            K,
            LINNUM,
            LOOKAT,
            MARGIN,
            NAMEAT,
            NEED,
            NFETCH,
            NNODE,
            NNODES,
            NODE,
            NPTRS,
            NVARS,
            NW,
            R,
            SPACE,
            TAIL,
            TOFREE,
            VARLEN,
            CHR,
            DP,
            EOF,
            GOTIT,
            NOAGNT,
            SUCCES,
            VECTOR,
        }
    }
}

fn ISQUOT(CODE: i32, IQUOTE: i32) -> bool {
    (CODE == IQUOTE)
}

/// Maintain a pool of kernel variables
///
/// Maintain a pool of variables read from SPICE ASCII kernel files.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  FNAME      I   LDPOOL
///  UNIT       I   WRPOOL
///  NAME       I   RTPOOL, EXPOOL, GIPOOL, GDPOOL, GCPOOL, PCPOOL,
///                 PDPOOL, PIPOOL, DTPOOL, SZPOOL, DVPOOL, GNPOOL
///  NAMES      I   SWPOOL
///  NNAMES     I   SWPOOL
///  AGENT      I   CVPOOL, DWPOOL, SWPOOL
///  N         I-O  RTPOOL, GIPOOL, GCPOOL, GDPOOL, DTPOOL, PCPOOL,
///                 PDPOOL, PIPOOL, LMPOOL, SZPOOL, GNPOOL
///  VALUES    I-O  RTPOOL  GDPOOL, PDPOOL
///  FOUND      O   RTPOOL, EXPOOL, GIPOOL, GCPOOL, GDPOOL, DTPOOL,
///                 SZPOOL, GNPOOL
///  UPDATE     O   CVPOOL, ZZPCTRCK
///  START      I   GIPOOL, GDPOOL, GCPOOL, GNPOOL
///  ROOM       I   GIPOOL, GDPOOL, GCPOOL. GNPOOL
///  CVALS     I-O  GCPOOL, PCPOOL, LMPOOL, GNPOOL
///  IVALS     I-O  GIPOOL, PIPOOL
///  TYPE       O   DTPOOL
///  UWVARS     O   ZZVUPOOL
///  UWPTRS     O   ZZVUPOOL
///  UWPOOL     O   ZZVUPOOL
///  UWAGNT     O   ZZVUPOOL
///  USRCTR    I-O  ZZPCTRCK
///
///  MAXVAR     P   (All)
///  MAXLEN     P   (All)
///  MAXVAL     P   (All)
///  MAXAGT     P   (All)
///  MXNOTE     P   (All)
///  BEGDAT     P   WRPOOL
///  BEGTXT     P   WRPOOL
///  CTRSIZ     P   ZZPCTRCK
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the ENTRY points for a discussion of their arguments.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the ENTRY points for a discussion of their arguments.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXVAR   is the maximum number of variables that the
///           kernel pool may contain at any one time.
///           MAXVAR should be a prime number.
///
///           Here's a list of primes that should make
///           it easy to upgrade MAXVAR when/if the need arises.
///
///               103
///               199
///               307
///               401
///               503
///               601
///               701
///               751
///               811
///               911
///              1013
///              1213
///              1303
///              1511
///              1811
///              1913
///              2003
///              2203
///              2503
///              2803
///              3203
///              3607
///              4001
///              4507
///              4801
///              5003 Current Value
///              6007
///              6521
///              7001
///              7507
///              8009
///              8501
///              9001
///              9511
///             10007
///             10501
///             11003
///             11503
///
///
///  MAXLEN   is the maximum length of the variable names that
///           can be stored in the kernel pool (also set in
///           zzrvar.f).
///
///  MAXVAL   is the maximum number of distinct values that
///           may belong to the variables in the kernel pool.
///           Each variable must have at least one value, and
///           may have any number, so long as the total number
///           does not exceed MAXVAL. MAXVAL must be at least
///           as large as MAXVAR.
///
///  MAXAGT   is the maximum number of agents that can be
///           associated with a given kernel variable.
///
///  MAXCHR   is the maximum number of characters that can be
///           stored in a component of a string valued kernel
///           variable.
///
///  MXNOTE   is the maximum sum of the sizes of the sets of
///           agents in the range of the mapping that associates
///           with each watched kernel variable a set of agents
///           that "watch" that variable.
///
///  MAXLIN   is the maximum number of character strings that
///           can be stored as data for kernel pool variables.
///
///  CTRSIZ   is the dimension of the counter array used by
///           various SPICE subsystems to uniquely identify
///           changes in their states. This parameter is
///           defined in the private include file 'zzctr.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If POOL is called directly, the error SPICE(BOGUSENTRY) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the ENTRY points for a discussion of their arguments.
/// ```
///
/// # Particulars
///
/// ```text
///  POOL should never be called directly, but should instead be
///  accessed only through its entry points.
///
///  The purpose of this routine is to maintain a pool of variables
///  read from ASCII kernel files. The following entry points may be
///  used to access the pool.
///
///        CLPOOL         Clears the pool.
///
///        LDPOOL         Loads the variables from a kernel file into
///                       the pool.
///
///        RTPOOL         Returns the value of a variable from
///                       the pool. (Obsolete use GDPOOL)
///
///        EXPOOL         Confirms the existence of a numeric
///                       variable in the pool.
///
///        WRPOOL         Writes the contents of the pool to an
///                       ASCII kernel file.
///
///        SWPOOL         Sets up a "watcher" on a variable so that
///                       various "agents" can be notified when a
///                       variable has been updated.
///
///        CVPOOL         Indicates whether or not an agent's
///                       variable has been updated since the last
///                       time an agent checked with the pool.
///
///        GCPOOL         Returns the value of a string valued
///                       variable in the pool.
///
///        GDPOOL         Returns the d.p. value of a numeric valued
///                       variable in the pool.
///
///        GIPOOL         Returns the integer value of a numeric valued
///                       variable in the pool.
///
///        DTPOOL         Returns the attributes of a variable in the
///                       pool.
///
///        PCPOOL         Allows the insertion of a character variable
///                       directly into the kernel pool without
///                       supplying a text kernel.
///
///        PDPOOL         Allows the insertion of a double precision
///                       variable directly into the kernel pool
///                       without supplying a text kernel.
///
///        PIPOOL         Allows the insertion of an integer variable
///                       directly into the kernel pool without
///                       supplying a text kernel.
///
///        LMPOOL         Similar to LDPOOL, but the text kernel is
///                       stored in an array of strings instead of an
///                       external file.
///
///        SZPOOL         allows run time retrieval of kernel pool
///                       memory parameters.
///
///        DVPOOL         allows deletion of a specific variable from
///                       the kernel pool. (CLPOOL deletes all
///                       variables from the kernel pool.)
///
///        GNPOOL         assists in determining which variables are
///                       defined in the kernel pool via variable name
///                       template matching.
///
///        DWPOOL         deletes a watch from the watcher system.
///
///  Nominally, the kernel pool contains up to MAXVAR separate
///  variables, up to MAXVAL numeric values, and up to MAXLIN string
///  values. The names of the individual variables may contain up to
///  MAXLEN characters.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data from
///  several kernel files can be loaded into a kernel pool. After the
///  pool is loaded, the values in the pool are written to a kernel
///  file.
///
///  C
///  C     Store in an array the names of the kernel files whose
///  C     values will be loaded into the kernel pool.
///  C
///        FNAME (1) = 'AXES.KER'
///        FNAME (2) = 'GM.KER'
///        FNAME (3) = 'LEAP_SECONDS.KER'
///
///  C
///  C     Clear the kernel pool. (This is optional.)
///  C
///        CALL CLPOOL
///
///  C
///  C     Load the variables from the three kernel files into the
///  C     the kernel pool.
///  C
///        DO I = 1, 3
///          CALL LDPOOL ( FNAME (I) )
///        END DO
///
///  C
///  C     We can examine the values associated with any d.p. variable
///  C     in the kernel pool using GDPOOL.
///  C
///        CALL GDPOOL ( VARIABLE, START, ROOM, NVALS, VALUES, FOUND )
///
///  C
///  C     Open the text file 'NEWKERNEL.KER'.
///  C
///        CALL TXTOPN ( NEWKERNEL.KER', UNIT )
///
///  C
///  C     Write the values in the kernel pool to the file.
///  C
///        CALL WRPOOL ( UNIT )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 10.2.0, 27-AUG-2021 (JDR) (BVS) (NJB)
///
///         Changed input argument name KERNEL to FNAME in entry point
///         LDPOOL for consistency with other routines.
///
///         Edited the header of the umbrella routine and all its entry
///         points.
///
///         Updated SZPOOL $Detailed_Input section.
///
/// -    SPICELIB Version 10.1.0, 14-JUL-2014 (NJB) (BVS)
///
///         Updated header of WRPOOL to improve accuracy of
///         that routine's output description.
///
///         Updated header of CVPOOL to improve accuracy of
///         the description of the output argument UPDATE.
///
///         Updated header of CLPOOL to improve the description
///         of behavior of the watcher subsystem.
///
///         Last update was 17-JAN-2014 (BVS) (NJB)
///
///         Increased key POOL parameters as follows:
///
///            MAXVAR    5003 ->  26003
///            MAXVAL  200000 -> 400000
///            MAXLIN    4000 ->  15000
///
///         Decreased MXNOTE factor F (MXNOTE=MAXVAR*F) from 10 to 5.
///
///         Updated the main umbrella argument list to include the POOL
///         state counter. Added the private entry point ZZPCTRCK allowing
///         other routines to check their saved POOL state counter against
///         the current POOL state counter to detect and act on the POOL
///         state change.
///
///         Updated $Index_Entries sections of entry points PCPOOL, PDPOOL,
///         and PIPOOL.
///
/// -    SPICELIB Version 10.0.0, 24-MAY-2010 (EDW) (NJB)
///
///         Added an error check on the length of the kernel pool variable
///         name argument in:
///
///            PCPOOL
///            PDPOOL
///            PIPOOL
///
///         to enforce the variable name length does not exceed MAXLEN.
///
///         Increased MAXVAL to 200000.
///
/// -    SPICELIB Version 9.0.0, 19-MAR-2009 (NJB)
///
///         Added watch deletion entry point DWPOOL and private entry
///         point ZZVUPOOL. Re-implemented watcher system to improve
///         efficiency, particularly of watch deletion. Bug fix: corrected
///         watcher overflow detection logic in SWPOOL. Updated header
///         code examples to use TXTOPN instead of GETLUN and a Fortran
///         OPEN statement; also to use GDPOOL instead of RTPOOL, except in
///         the header of RTPOOL itself.
///
///         Code examples in SWPOOL and CVPOOL were updated to handle
///         kernel pool fetch failures.
///
///         Existing entry points modified as part of this update were:
///
///            POOL
///            CLPOOL
///            CVPOOL
///            DTPOOL
///            DVPOOL
///            EXPOOL
///            GCPOOL
///            GDPOOL
///            GIPOOL
///            GNPOOL
///            LDPOOL
///            LMPOOL
///            PCPOOL
///            PDPOOL
///            PIPOOL
///            RTPOOL
///            SWPOOL
///            WRPOOL
///
///         Code examples using RTPOOL were updated to use GDPOOL, except
///         in the header of RTPOOL itself. Code examples using GETLUN and
///         an in-line Fortran OPEN statement were updated to use TXTOPN.
///
///         Various typos in comments throughout this file were fixed.
///
///
/// -    SPICELIB Version 8.3.0, 22-DEC-2004 (NJB)
///
///         Fixed bug in DVPOOL. Made corrections to comments in
///         other entry points. The updated routines are DTPOOL,
///         DVPOOL, EXPOOL, GCPOOL, GDPOOL, GIPOOL, RTPOOL.
///
/// -    SPICELIB Version 8.2.0, 24-JAN-2003 (BVS)
///
///         Increased MAXVAL to 40000.
///
/// -    SPICELIB Version 8.1.0, 13-MAR-2001 (FST) (NJB)
///
///         Increased kernel pool size and agent parameters. MAXVAR is now
///         5003, MAXVAL is 10000, MAXLIN is 4000, MXNOTE is 2000, and
///         MAXAGT is 1000.
///
///         Modified Fortran output formats used in entry point WRPOOL to
///         remove list-directed formatting. This change was made to
///         work around problems with the way f2c translates list-
///         directed I/O.
///
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
///         The entry point RTPOOL should now be regarded as obsolete
///         and is maintained solely for backward compatibility with
///         existing routines that make use of it.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL and CVPOOL were added.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 2.0.0, 18-OCT-1989 (RET)
///
///        A FAILED test was inserted into the control of the DO-loop which
///        reads in each kernel variable in LDPOOL.
///
/// -    SPICELIB Version 1.2.0, 09-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.3.0, 22-DEC-2004 (NJB)
///
///         Fixed bug in DVPOOL. Made corrections to comments in
///         other entry points. The updated routines are DTPOOL,
///         DVPOOL, EXPOOL, GCPOOL, GDPOOL, GIPOOL, RTPOOL.
///
/// -    SPICELIB Version 8.2.0, 24-JAN-2003 (BVS)
///
///         Increased MAXVAL to 40000.
///
/// -    SPICELIB Version 8.1.0, 13-MAR-2001 (FST) (NJB)
///
///         Increased kernel pool size and agent parameters. MAXVAR is now
///         5003, MAXVAL is 10000, MAXLIN is 4000, MXNOTE is 2000, and
///         MAXAGT is 1000.
///
///         Modified Fortran output formats used in entry point WRPOOL to
///         remove list-directed formatting. This change was made to
///         work around problems with the way f2c translates list-
///         directed I/O.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
///         The entry point RTPOOL should now be regarded as obsolete
///         and is maintained solely for backward compatibility with
///         existing routines that make use of it.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL (set watch on a pool variable)
///         and CVPOOL (check variable for update) so that routines
///         that buffer data stored in the kernel pool can fetch
///         that data only when it is updated.
///
///         Also the control of initializations was modified to be
///         consistent with other SPICELIB practices.
///
///         Finally, the revision history was upgraded so that the
///         version number increases over time. This wasn't true
///         before. In addition some early revision data that referred to
///         pre-SPICELIB modifications were removed. This editing of
///         the version numbers makes it unlikely that anyone can track
///         down which previous version of this routine they have by
///         looking at the version number. The best way to determine
///         the routine you had previously is to compare the dates
///         stored in the Version line of the routine.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
///         Also, changed version number in previous revisions entry
///         from SPICELIB Version 2.0.0 to SPICELIB Version 2.0.0. The
///         last version entry in the $Version section had been
///         Version 1.0.0, dated later than the entry for `version 2'
///         in the $Revisions section!
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 2.0.0, 18-OCT-1989 (RET)
///
///        A FAILED test was inserted into the control of the DO-loop which
///        reads in each kernel variable.
///
///        Previously, if the error action 'RETURN' had been set by a
///        calling program, and the call to RDKNEW by LDPOOL failed,
///        then execution would continue through LDPOOL, with SPICELIB
///        routines returning upon entry. This meant that the routine
///        RDKVAR never got a chance to set the EOF flag, which was the
///        only control of the DO-loop. An infinite loop resulted in such
///        cases. The FAILED test resolves that situation.
///
/// -    SPICELIB Version 1.2.0, 9-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
///
/// -    SPICELIB Version 1.0.0, 8-JAN-1989 (IMU)
/// ```
pub fn pool(
    ctx: &mut SpiceContext,
    fname: &str,
    unit: i32,
    name: &str,
    names: CharArray,
    nnames: i32,
    agent: &str,
    n: i32,
    values: &[f64],
    found: bool,
    update: bool,
    start: i32,
    room: i32,
    cvals: CharArray,
    ivals: &[i32],
    type_: &str,
    uwvars: CharArray,
    uwptrs: &[i32],
    uwpool: &[[i32; 2]],
    uwagnt: CharArray,
    usrctr: &[i32; 2],
) -> crate::Result<()> {
    POOL(
        fname.as_bytes(),
        unit,
        name.as_bytes(),
        names,
        nnames,
        agent.as_bytes(),
        n,
        values,
        found,
        update,
        start,
        room,
        cvals,
        ivals,
        type_.as_bytes(),
        uwvars,
        uwptrs,
        uwpool.as_flattened(),
        uwagnt,
        usrctr,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure POOL ( Maintain a pool of kernel variables )
pub fn POOL(
    FNAME: &[u8],
    UNIT: i32,
    NAME: &[u8],
    NAMES: CharArray,
    NNAMES: i32,
    AGENT: &[u8],
    N: i32,
    VALUES: &[f64],
    FOUND: bool,
    UPDATE: bool,
    START: i32,
    ROOM: i32,
    CVALS: CharArray,
    IVALS: &[i32],
    TYPE: &[u8],
    UWVARS: CharArray,
    UWPTRS: &[i32],
    UWPOOL: &[i32],
    UWAGNT: CharArray,
    USRCTR: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Private SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // The next two variables are for use in traversing linked lists.
    //

    //
    // Local variables
    //

    //
    // Because some environments (such as the SUN) are too stupid to
    // treat the backslash character correctly we have to go through
    // some gyrations to put it into a variable in a "portable" way.
    // This is the reason for the following block of declarations.
    // Admittedly this is bizarre, but it works.
    //

    //
    //    The following is the hash table used for holding kernel pool
    //    variables.  Here's the basic structure:
    //
    //    The function ZZHASH computes the address of the head of a linked
    //    list that contains the collisions for the range of ZZHASH.
    //
    //    The head node of the collision lists is stored in NAMLST.
    //
    //    If NAMLST has a value zero then
    //
    //       there is no name corresponding to that value of the
    //       hash function.
    //
    //    If NAMLST is non-zero then
    //
    //       it is the head node of the list of names that have been
    //       stored so far.
    //
    //       The list of addresses of names is stored in NMPOOL.
    //       The names that have been stored so far are in PNAMES.
    //
    //    The data associated with  PNAMES is pointed to by DATLST
    //    and CHPOOL or DPPOOL.  If a name of interest is stored in
    //    PNAMES(I) then the DATLST(I) points to the first data node
    //    associated with the name.
    //
    //    If DATLST(I) is less than zero then
    //
    //       its opposite is the address of the first node of
    //       character data associated with PNAMES(I).
    //
    //    If DATLST(I) is positive then
    //
    //       it points to the address of the first node of numeric
    //       data associated with PNAMES(I).
    //
    //    If DATLST(I) is zero
    //
    //       there is no data associated with PNAMES(I).
    //
    //
    //    The arrays DPPOOL and CHPOOL are linked list pools that
    //    give the address lists of values associated with a name.
    //
    //    The actual data is stored in DPVALS and CHVALS.
    //
    //    Here's a picture of how this all works.
    //
    //
    //                                            Linked list Pool
    //                                            of HASH collisions
    //                      NAMLST                  NMPOOL         PNAME
    //                    +------------+          +---------+    +--------+
    //                    |            |          |         |    |        |
    //                    +------------+ if not 0 +---------+    +--------+
    // ZZHASH( NAME ) --->|  Head Node | ---.     |         |    |        |
    //                    +------------+    |     +---------+    +--------+
    //                                      |     |         |    |        |
    //                                      |     +---------+    +--------+
    //                                      `-->  |Head of  |    |Name    |
    //                                            |collision|    |corresp.|
    //                                            |list for | -. |to head |
    //                                            | NAME    |  | |of list |
    //                                            +---------+  | +--------+
    //                                            |         |  | |        |
    //                                            +---------+  | +--------+
    //                                            |         |  | |        |
    //                                            +---------+  | +--------+
    //                                            |Next Node|<-' |NextName|
    //                                            +---------+etc.+--------+
    //                                                 .              .
    //                                                 .              .
    //                                                 .              .
    //                                            +---------+    +--------+
    //                                            |         |    |        |
    //                                            +---------+    +--------+
    //
    //
    //
    //
    //     Linked       Variable    Heads of
    //     List Pool     Names      Data lists
    //      NMPOOL       PNAME       DATLST
    //    +--------+   +--------+   +---------+          Head of linked list
    //    |        |   |        |   |         |     .--> in DPPOOL linked
    //    +--------+   +--------+   +---------+    |     list pool
    //    |        |   |        |   |         |    |
    //    +--------+   +--------+   +---------+    | Positive Value
    //    |        |<->|        |<->|         |---<
    //    +--------+   +--------+   +---------+    |
    //    |        |   |        |   |         |    | Negative Value
    //    +--------+   +--------+   +---------+    |
    //    |        |   |        |   |         |    `--> Opposite of head
    //    +--------+   +--------+   +---------+          of linked list
    //    |        |   |        |   |         |          in CHPOOL linked
    //    +--------+   +--------+   +---------+          list pool.
    //
    //
    //
    //
    //
    //     Linked                Values
    //     List Pool             of data
    //      DPPOOL (CHPOOL)      DPVALS (CHVALS)
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    | HEAD       |--. <--> | head value |
    //    +------------+  |      +------------+
    //    |            |  |      |            |
    //    +------------+  |      +------------+
    //    |            |  |      |            |
    //    +------------+  |      +------------+
    //    | Node 2     |<-' <--> | 2nd value  |
    //    +------------+ etc.    +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //    |            |         |            |
    //    +------------+         +------------+
    //
    //

    //
    // The WT... variables make up the data structure that
    // maps variables to their associated agents (WTAGNT).
    // A diagram of the watcher data structure is shown below.
    //
    //  Watched     Heads of     Agent linked  Agent names
    //  variables   agent lists  list pool
    //   WTVARS       WTPTR        WTPOOL        WTAGNT
    // +--------+   +--------+   +---------+   +---------+
    // |        |   |        |   |         |   |         |
    // +--------+   +--------+   +---------+   +---------+
    // |        |   |        |   |         |   |         |
    // +--------+   +--------+   +---------+   +---------+
    // |        |<->|        |<->|         |<->|         |
    // +--------+   +--------+   +---------+   +---------+
    // |        |   |        |   |         |   |         |
    // +--------+   +--------+   +---------+   +---------+
    // |        |   |        |   |         |   |         |
    // +--------+   +--------+   +---------+   +---------+
    // |        |   |        |   |         |   |         |
    // +--------+   +--------+   +---------+   +---------+
    //
    //

    //
    // Agents contains the list of agents that need to be notified
    // about updates to their variables.  NOTIFY and ACTIVE are both
    // temporary sets.
    //
    // These variables are declared with the size MXNOTE because
    // they must be able to hold the largest possible number
    // of agents that could be associated with a kernel variable.
    //

    //
    // First is our initialization flag.
    //

    //
    // POOL state counter.
    //

    //
    // The remaining local variables...
    //

    //
    // Save EVERYTHING.
    //

    //
    // Initial values
    //

    //
    // Set up the definition of our in-line functions.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"POOL", ctx)?;
    }

    //
    // This routine should never be called. If this routine is called,
    // an error is signaled.
    //
    SETMSG(b"POOL: You have called an entry which performs performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine POOL.", ctx);

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"POOL", ctx)?;
    Ok(())
}

/// Clear the pool of kernel variables
///
/// Remove all kernel variables from the kernel pool. Watches
/// on kernel variables are retained.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  All known agents (those established through the SPICELIB
///      routine SWPOOL) will be "notified" that their watched
///      variables have been updated whenever CLPOOL is called.
/// ```
///
/// # Particulars
///
/// ```text
///  CLPOOL clears the pool of kernel variables maintained by
///  the kernel POOL subsystem. All the variables in the pool are
///  deleted. However, all watcher information is retained.
///
///  Each watched variable will be regarded as having been updated.
///  Any agent associated with that variable will have a notice
///  posted for it indicating that its watched variable has been
///  updated.
///
///  Application programs can delete watches by calling the SPICELIB
///  routine DWPOOL. See the header of that routine for details.
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
///  1) This code example demonstrates how to assign values to kernel
///     pool variables, how to check for the existence of kernel pool
///     variables and how to clear the kernel pool, i.e. how to delete
///     all variable assignments loaded into the kernel pool.
///
///     Place a value into the kernel pool and check for the variable
///     to which the value has been assigned. Clear the kernel pool
///     and check for that variable again.
///
///     Example code begins here.
///
///
///           PROGRAM CLPOOL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      DVALS ( 1 )
///
///           INTEGER               N
///
///           LOGICAL               FOUND
///
///
///     C
///     C     Place a value into the kernel pool. Recall the routines
///     C     for direct insertion of pool assignments have arrays for
///     C     input.
///     C
///           DVALS( 1 ) = -666.D0
///           CALL PDPOOL ( 'TEST_VAR', 1, DVALS )
///
///     C
///     C     Check for the variable assignment to TEST_VAR.
///     C
///           CALL GDPOOL ( 'TEST_VAR', 1, 1, N, DVALS, FOUND )
///
///           WRITE(*,'(A)') 'First call to GDPOOL:'
///
///           IF ( FOUND ) THEN
///
///              WRITE(*,'(A,F8.2)') '   TEST_VAR value:', DVALS(1)
///
///           ELSE
///
///              WRITE(*,'(A)') '   TEST_VAR not in kernel pool.'
///
///           END IF
///
///     C
///     C     Now clear the kernel pool.
///     C
///           CALL CLPOOL ()
///
///     C
///     C     Again, check for the TEST_VAR assignment.
///     C
///           CALL GDPOOL ( 'TEST_VAR', 1, 1, N, DVALS, FOUND )
///
///           WRITE(*,'(A)') 'Second call to GDPOOL:'
///
///           IF ( FOUND ) THEN
///
///              WRITE(*,'(A,F6.2)') '   TEST_VAR value:', DVALS(1)
///
///           ELSE
///
///              WRITE(*,'(A)') '   TEST_VAR not in kernel pool.'
///
///           END IF
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     First call to GDPOOL:
///        TEST_VAR value: -666.00
///     Second call to GDPOOL:
///        TEST_VAR not in kernel pool.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine should not be used to unload kernels that
///      have been loaded via FURNSH.
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
/// -    SPICELIB Version 8.2.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Routine declared error free.
///
/// -    SPICELIB Version 8.2.0, 01-JUL-2014 (NJB) (BVS)
///
///         Description of behavior of watcher subsystem was expanded.
///
///         Last update was 30-JUL-2013 (BVS)
///
///            Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation. Updated $Restrictions
///         header section. Updated code example to use TXTOPN.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         This entry point clears the string valued variables as well as
///         the numeric valued variables.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL and CVPOOL were added.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.2.0, 01-JUL-2014 (NJB) (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
///         ZZNWPOOL is called to update the list of agents
///         to notify of watched variable updates.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         This entry point clears the string valued variables as well as
///         the numeric valued variables.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL (set watch on a pool variable)
///         and CVPOOL (check variable for update) so that routines
///         that buffer data stored in the kernel pool can fetch
///         that data only when it is updated.
///
///
///         Also the control of initializations was modified to be
///         consistent with other SPICELIB practices.
///
///         Finally, the revision history was upgraded so that the
///         version number increases over time. This wasn't true
///         before. In addition some early revision data that referred to
///         pre-SPICELIB modifications were removed. This editing of
///         the version numbers makes it unlikely that anyone can track
///         down which previous version of this routine they have by
///         looking at the version number. The best way to determine
///         the routine you had previously is to compare the dates
///         stored in the Version line of the routine.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 1.0.0, 8-JAN-1989 (IMU)
/// ```
pub fn clpool(ctx: &mut SpiceContext) -> crate::Result<()> {
    CLPOOL(ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CLPOOL ( Clear the pool of kernel variables )
pub fn CLPOOL(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CLPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Wipe out all of the PNAMES data.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXVAR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.NAMLST[save.I] = 0;
            save.DATLST[save.I] = 0;
            fstr::assign(save.PNAMES.get_mut(save.I), b" ");
            save.I += m3__;
        }
    }
    //
    // Free up all of the space in all of the linked list pools, except
    // for the watcher pool.
    //
    LNKINI(MAXVAR, save.NMPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXVAL, save.DPPOOL.as_slice_mut(), ctx)?;
    LNKINI(MAXLIN, save.CHPOOL.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = CARDC(save.WTVARS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Union the update set AGENTS with the set of agents
            // associated with the Ith watched variable.
            //
            ZZNWPOOL(
                &save.WTVARS[save.I],
                save.WTVARS.as_arg(),
                save.WTPTRS.as_slice(),
                save.WTPOOL.as_slice(),
                save.WTAGNT.as_arg(),
                save.ACTIVE.as_arg_mut(),
                save.NOTIFY.as_arg_mut(),
                save.AGENTS.as_arg_mut(),
                ctx,
            )?;
            save.I += m3__;
        }
    }

    CHKOUT(b"CLPOOL", ctx)?;
    Ok(())
}

/// Load variables from a kernel file into the pool
///
/// Load the variables contained in a NAIF ASCII kernel file into the
/// kernel pool.
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
///  FNAME      I   Name of the text kernel file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of the text kernel file whose variables will
///           be loaded into the pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an I/O error occurs while opening or reading a text kernel,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If any text kernel parsing error occurs, the error is signaled
///      by a routine in the call tree of this routine.
///
///  3)  If a kernel pool overflow is detected, an error is signaled by
///      a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See FNAME in $Detailed_Input.
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
///  1) The following program demonstrates how to load the variables
///     contained in a NAIF ASCII kernel file into the kernel pool
///     and how to determine the properties of a stored kernel
///     variable.
///
///     The program prompts for text kernel name and for the name of
///     a kernel variable. If the variable is present in the kernel
///     pool, the dimension and type of the variable are displayed.
///
///
///     Example code begins here.
///
///
///           PROGRAM LDPOOL_EX1
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
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 256 )
///
///           INTEGER               KVNMLN
///           PARAMETER           ( KVNMLN = 33 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(KVNMLN)    VARNAM
///           CHARACTER*(1)         VTYPE
///
///           INTEGER               N
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for the name of a text-kernel file.
///     C
///           CALL PROMPT ( 'Enter text-kernel name        > ', FNAME )
///
///     C
///     C     Load the kernel. The same operation could be done using
///     C     a FURNSH call.
///     C
///           CALL LDPOOL ( FNAME )
///
///           CALL PROMPT ( 'Enter name of kernel variable > ',
///          .               VARNAM )
///
///           CALL DTPOOL ( VARNAM, FOUND, N, VTYPE )
///
///           IF ( FOUND ) THEN
///              WRITE(*,*) ' '
///              WRITE(*,*) 'Properties of variable ',
///          .               VARNAM(:RTRIM(VARNAM)), ':'
///              WRITE(*,*) ' '
///              WRITE(*,*) '   Size:   ', N
///
///              IF ( VTYPE .EQ. 'C' ) THEN
///
///                 WRITE(*,*) '   Type:   Character'
///
///              ELSE
///
///                 WRITE(*,*) '   Type:   Numeric'
///              END IF
///
///           ELSE
///
///              WRITE(*,*) VARNAM,
///          .              ' is not present in the kernel pool.'
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the PCK file gm_de431.tpc to ask for the
///     variable 'BODY000_GMLIST', the output was:
///
///
///     Enter text-kernel name        > gm_de431.tpc
///     Enter name of kernel variable > BODY000_GMLIST
///
///      Properties of variable BODY000_GMLIST:
///
///         Size:             65
///         Type:   Numeric
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Normally SPICE applications should load kernels via the
///      FURNSH routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.3.0, 17-AUG-2021 (JDR)
///
///         Changed input argument name KERNEL to FNAME for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 8.2.0, 30-JUL-2013 (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
///         Filled out $Exceptions section of header, which previously
///         contained only the word "None."
///
///         Updated code example to use TXTOPN.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         In addition much greater error checking is performed on
///         the input file to guarantee valid inputs.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL and CVPOOL were added.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 2.0.0, 18-OCT-1989 (RET)
///
///         A FAILED test was inserted into the control of the DO-loop
///         which reads in each kernel variable in LDPOOL.
///
/// -    SPICELIB Version 1.2.0, 09-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
///         The entry point RTPOOL should now be regarded as obsolete
///         and is maintained solely for backward compatibility with
///         existing routines that make use of it.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
///         In addition much greater error checking is performed on
///         the input file to guarantee valid inputs.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL (set watch on a pool variable)
///         and CVPOOL (check variable for update) so that routines
///         that buffer data stored in the kernel pool can fetch
///         that data only when it is updated.
///
///         In addition, the revision history was upgraded so that the
///         version number increases over time. This wasn't true
///         before. In addition some early revision data that referred to
///         pre-SPICELIB modifications were removed. This editing of
///         the version numbers makes it unlikely that anyone can track
///         down which previous version of this routine they have by
///         looking at the version number. The best way to determine
///         the routine you had previously is to compare the dates
///         stored in the Version line of the routine.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
///         Also, changed version number in previous revisions entry
///         from SPICELIB Version 2.0.0 to SPICELIB Version 2.0.0. The
///         last version entry in the $Version section had been
///         Version 1.0.0, dated later than the entry for `version 2'
///         in the $Revisions section!
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 2.0.0, 18-OCT-1989 (RET)
///
///        A FAILED test was inserted into the control of the DO-loop which
///        reads in each kernel variable.
///
///        Previously, if the error action 'RETURN' had been set by a
///        calling program, and the call to RDKNEW by LDPOOL failed,
///        then execution would continue through LDPOOL, with SPICELIB
///        routines returning upon entry. This meant that the routine
///        RDKVAR never got a chance to set the EOF flag, which was the
///        only control of the DO-loop. An infinite loop resulted in such
///        cases. The FAILED test resolves that situation.
///
/// -    SPICELIB Version 1.2.0, 9-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
///
/// -    SPICELIB Version 1.0.0, 8-JAN-1989 (IMU)
/// ```
pub fn ldpool(ctx: &mut SpiceContext, fname: &str) -> crate::Result<()> {
    LDPOOL(fname.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LDPOOL ( Load variables from a kernel file into the pool )
pub fn LDPOOL(FNAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LDPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Open the kernel file and read the first variable.
    //
    RDKNEW(FNAME, ctx)?;
    ZZRVAR(
        save.NAMLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.PNAMES.as_arg_mut(),
        save.DATLST.as_slice_mut(),
        save.DPPOOL.as_slice_mut(),
        save.DPVALS.as_slice_mut(),
        save.CHPOOL.as_slice_mut(),
        save.CHVALS.as_arg_mut(),
        &mut save.VARNAM,
        &mut save.EOF,
        ctx,
    )?;

    //
    // Read the variables in the file, one at a time.
    //
    while (!save.EOF && !FAILED(ctx)) {
        if fstr::ne(&save.VARNAM, b" ") {
            //
            // See if this variable is being watched; if it is, add its
            // associated agents to the list of AGENTS to be notified of a
            // watched variable update.
            //
            if ELEMC(&save.VARNAM, save.WTVARS.as_arg(), ctx)? {
                //
                // Union the update set AGENTS with the set of agents
                // associated with the variable NAME.
                //
                ZZNWPOOL(
                    &save.VARNAM,
                    save.WTVARS.as_arg(),
                    save.WTPTRS.as_slice(),
                    save.WTPOOL.as_slice(),
                    save.WTAGNT.as_arg(),
                    save.ACTIVE.as_arg_mut(),
                    save.NOTIFY.as_arg_mut(),
                    save.AGENTS.as_arg_mut(),
                    ctx,
                )?;
            }
        }

        ZZRVAR(
            save.NAMLST.as_slice_mut(),
            save.NMPOOL.as_slice_mut(),
            save.PNAMES.as_arg_mut(),
            save.DATLST.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.DPVALS.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.CHVALS.as_arg_mut(),
            &mut save.VARNAM,
            &mut save.EOF,
            ctx,
        )?;
    }
    //
    // We need to make sure that the kernel file gets closed.  Normally
    // the calling tree of ZZRVAR take care of this, but if a parsing
    // or syntax error occurs there,  ZZRVAR just returns and the
    // closing of the kernel is never handled.  This takes care
    // of the problem.  If the file has been closed already, this
    // doesn't hurt anything.
    //
    CLTEXT(FNAME, ctx)?;
    CHKOUT(b"LDPOOL", ctx)?;
    Ok(())
}

/// Return the value of a pooled kernel variable
///
/// Return the value of a kernel variable from the kernel pool.
///
/// This routine is maintained only for backward compatibility.
/// It should be regarded as obsolete. Use one of the entry points
/// GDPOOL, GIPOOL or GCPOOL in its place.
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
///  NAME       I   Name of the variable whose value is to be returned.
///  N          O   Number of values associated with NAME.
///  VALUES     O   Values associated with NAME.
///  FOUND      O   .TRUE. if variable is in pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the variable whose values are to be
///           returned. If the variable is not in the pool, FOUND
///           will be .FALSE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values associated with NAME.
///           If NAME is not in the pool, no value is given to
///           N.
///
///  VALUES   is the array of values associated with NAME.
///           If NAME is not in the pool, no values are given to
///           the elements of VALUES.
///
///  FOUND    is .TRUE. if the variable is in the pool, .FALSE. if it
///           is not.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the output argument VALUES is not large enough to hold all
///      of the values of the kernel variable designated by NAME,
///      then memory will be corrupted. RTPOOL cannot detect this
///      error.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data from
///  several kernel files can be loaded into a kernel pool. After the
///  pool is loaded, the values in the pool are written to a kernel
///  file.
///
///
///  C
///  C     Store in an array the names of the kernel files whose
///  C     values will be loaded into the kernel pool.
///  C
///        FNAME (1) = 'AXES.KER'
///        FNAME (2) = 'GM.KER'
///        FNAME (3) = 'LEAP_SECONDS.KER'
///
///  C
///  C     Clear the kernel pool. (This is optional.)
///  C
///        CALL CLPOOL
///
///  C
///  C     Load the variables from the three kernel files into the
///  C     the kernel pool.
///  C
///        DO I = 1, 3
///          CALL LDPOOL ( FNAME (I) )
///        END DO
///
///  C
///  C     We can examine the values associated with any variable
///  C     in the kernel pool using RTPOOL.
///  C
///        CALL RTPOOL ( VARIABLE, NUMVAL, VALUES, FOUND )
///
///  C
///  C     Open the new text file 'NEWKERNEL.KER'.
///  C
///        CALL TXTOPN ( 'NEWKERNEL.KER', UNIT )
///
///  C
///  C     Write the values in the kernel pool to the file.
///  C
///        CALL WRPOOL ( UNIT )
/// ```
///
/// # Restrictions
///
/// ```text
///  See $Exceptions section.
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
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
///         Updated code example to use TXTOPN.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
///         The entry point RTPOOL should now be regarded as obsolete
///         and is maintained solely for backward compatibility with
///         existing routines that make use of it.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
pub fn rtpool(
    ctx: &mut SpiceContext,
    name: &str,
    n: &mut i32,
    values: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    RTPOOL(name.as_bytes(), n, values, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RTPOOL ( Return the value of a pooled kernel variable )
pub fn RTPOOL(
    NAME: &[u8],
    N: &mut i32,
    VALUES: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut VALUES = DummyArrayMut::new(VALUES, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RTPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        *FOUND = false;
        CHKOUT(b"RTPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            *FOUND = false;
            CHKOUT(b"RTPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //
    // However, recall that RTPOOL can only return d.p. values.
    // DATLST(NODE) is the head of a d.p. list of values if it
    // is positive.  We use negative values to point to character
    // values.
    //
    if (save.DATLST[save.NODE] <= 0) {
        *FOUND = false;
    } else {
        *FOUND = true;
        *N = 0;
        save.NODE = save.DATLST[save.NODE];

        while (save.NODE > 0) {
            *N = (*N + 1);
            VALUES[*N] = save.DPVALS[save.NODE];
            save.NODE = save.DPPOOL[[NEXT, save.NODE]];
        }
    }

    CHKOUT(b"RTPOOL", ctx)?;
    Ok(())
}

/// Confirm the existence of a pooled kernel variable
///
/// Confirm the existence of a numeric kernel variable in the kernel
/// pool.
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
///  NAME       I   Name of a numeric kernel variable.
///  FOUND      O   .TRUE. when the variable is in the pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the numeric kernel variable whose
///           existence in the kernel pool is to be checked.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FOUND    is .TRUE. whenever the specified variable is included
///           in the pool.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines whether or not a numeric kernel pool
///  variable exists. It does not detect the existence of
///  string valued kernel pool variables.
///
///  A better routine for determining the existence of numeric kernel
///  pool variables is the routine DTPOOL which determines the
///  existence, size and type of kernel pool variables.
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
///  1) The following code example demonstrates how to use EXPOOL
///     to confirm the existence of numeric kernel pool variables.
///     In the example, we will look for different variables;
///     some of them numeric, some string valued and some not
///     present in the kernel pool.
///
///     Use the kernel shown below; an IK defining two keywords
///     used to provide data for an instrument with NAIF ID -999001.
///
///
///        KPL/IK
///
///        File name: expool_ex1.ti
///
///        The keyword below define the three frequencies used by a
///        hypothetical instrument (NAIF ID -999001). They correspond
///        to three filters: red, green and blue. Frequencies are
///        given in micrometers.
///
///        \begindata
///
///           INS-999001_FREQ_RGB   = (  0.65,  0.55, 0.475 )
///           INS-999001_FREQ_UNITS = ( 'MICROMETERS'       )
///
///        \begintext
///
///
///        End of IK
///
///
///     Example code begins here.
///
///
///           PROGRAM EXPOOL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         IKNAME
///           PARAMETER           ( IKNAME = 'expool_ex1.ti' )
///
///           INTEGER               KPVNLN
///           PARAMETER           ( KPVNLN = 32 )
///
///           INTEGER               NKPVNM
///           PARAMETER           ( NKPVNM = 3  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(KPVNLN)    KEYWRD ( NKPVNM )
///
///           INTEGER               I
///
///           LOGICAL               FOUND
///
///     C
///     C     Define the variable names
///     C
///           DATA                  KEYWRD   /
///          .                             'INS-999001_FREQ_RGB',
///          .                             'NOT_IN_THE_POOL',
///          .                             'INS-999001_FREQ_UNITS' /
///
///     C
///     C     Load the instrument kernel.
///     C
///           CALL FURNSH ( IKNAME )
///
///           DO I = 1, NKPVNM
///
///     C
///     C        Check if the variable is numeric and present
///     C        in the kernel pool.
///     C
///              CALL EXPOOL ( KEYWRD(I), FOUND )
///
///              WRITE(*,*) 'Variable name: ', KEYWRD(I)
///
///              IF ( FOUND ) THEN
///
///                 WRITE(*,*) '   It is numeric and exists in the '
///          .              // 'kernel pool.'
///
///              ELSE
///
///                 WRITE(*,*) '   Either it is not numeric or it is '
///          .              // 'not in the kernel pool.'
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
///      Variable name: INS-999001_FREQ_RGB
///         It is numeric and exists in the kernel pool.
///      Variable name: NOT_IN_THE_POOL
///         Either it is not numeric or it is not in the kernel pool.
///      Variable name: INS-999001_FREQ_UNITS
///         Either it is not numeric or it is not in the kernel pool.
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
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated the header to reflect that only numeric variables
///         present in the kernel pool will cause the routine to return
///         .TRUE.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
///         Fixed typos.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
///
///         The entry point RTPOOL should now be regarded as obsolete
///         and is maintained solely for backward compatibility with
///         existing routines that make use of it.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
/// ```
pub fn expool(ctx: &mut SpiceContext, name: &str, found: &mut bool) -> crate::Result<()> {
    EXPOOL(name.as_bytes(), found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EXPOOL ( Confirm the existence of a pooled kernel variable )
pub fn EXPOOL(NAME: &[u8], FOUND: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EXPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        *FOUND = false;
        CHKOUT(b"EXPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            *FOUND = false;
            CHKOUT(b"EXPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //
    // However, recall that EXPOOL indicates the existence only of
    // d.p. values.
    //
    *FOUND = (save.DATLST[save.NODE] > 0);

    CHKOUT(b"EXPOOL", ctx)?;
    Ok(())
}

/// Write the variables in pool to a specified unit
///
/// Write to a specified unit a set of "keyword = value" assignments
/// for all currently defined kernel variables. The assignments
/// constitute a text kernel from which the current state of the
/// kernel pool can be restored.
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
///  UNIT       I   Logical unit to which the variables in the pool
///                 will be written.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the logical unit to which the variables in the pool
///           will be written.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If writing a variable to the output file fails due to an I/O
///      error, the error SPICE(WRITEERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  This routine writes to the specified logical unit the kernel
///  variables present in the kernel pool. Each variable consists of a
///  name and a set of associated values. The variables are written in
///  the form of a series of "keyword = value" assignments. The
///  assignments are preceded by a SPICE text kernel "\begindata"
///  marker.
///
///  The output of this routine, if written to a text file, is a SPICE
///  text kernel. The current contents of the kernel pool can be
///  restored by clearing the pool and then loading this text kernel.
///
///  If the values are to be written to an output kernel file, the
///  file should be opened with a logical unit determined by the
///  calling program.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data from
///  several kernel files can be loaded into a kernel pool. After the
///  pool is loaded, the values in the pool are written to a kernel
///  file.
///
///
///  C
///  C     Store in an array the names of the kernel files whose
///  C     values will be loaded into the kernel pool.
///  C
///        FNAME (1) = 'AXES.KER'
///        FNAME (2) = 'GM.KER'
///        FNAME (3) = 'LEAP_SECONDS.KER'
///
///  C
///  C     Clear the kernel pool. (This is optional.)
///  C
///        CALL CLPOOL
///
///  C
///  C     Load the variables from the three kernel files into the
///  C     the kernel pool.
///  C
///        DO I = 1, 3
///          CALL LDPOOL ( FNAME (I) )
///        END DO
///
///  C
///  C     We can examine the values associated with any double
///  C     precision variable in the kernel pool using GDPOOL.
///  C
///        CALL GDPOOL ( VARIABLE, 1, NMAX, NUMVAL, VALUES, FOUND )
///
///  C
///  C     Open the new text file 'NEWKERNEL.KER'.
///  C
///        CALL TXTOPN ( 'NEWKERNEL.KER', UNIT )
///
///  C
///  C     Write the values in the kernel pool to the file.
///  C
///        CALL WRPOOL ( UNIT )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 17-AUG-2021 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated the $Exceptions section to document the actual error
///         handling of the routine.
///
/// -    SPICELIB Version 8.1.1, 30-JUN-2014 (NJB)
///
///         Updated header to more accurately describe the output
///         of this routine.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Updated code example to use TXTOPN.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued. Both types are supported
///         by WRPOOL.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 1.2.0, 09-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1989 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
/// -    SPICELIB Version 5.0.0, 22-AUG-1990 (NJB)
///
///         Increased value of parameter MAXVAL to 5000 to accommodate
///         storage of SCLK coefficients in the kernel pool.
///
/// -    SPICELIB Version 4.0.0, 12-JUN-1990 (IMU)
///
///         All entry points except POOL and CLPOOL now initialize the
///         pool if it has not been done yet.
///
/// -    SPICELIB Version 3.0.0, 23-OCT-1989 (HAN)
///
///         Added declaration of FAILED. FAILED is checked in the
///         DO-loops in LDPOOL and WRPOOL to prevent infinite looping.
///
/// -    SPICELIB Version 2.0.0, 18-OCT-1989 (RET)
///
///        A FAILED test was inserted into the control of the DO-loop which
///        reads in each kernel variable.
///
///        Previously, if the error action 'RETURN' had been set by a
///        calling program, and the call to RDKNEW by LDPOOL failed,
///        then execution would continue through LDPOOL, with SPICELIB
///        routines returning upon entry. This meant that the routine
///        RDKVAR never got a chance to set the EOF flag, which was the
///        only control of the DO-loop. An infinite loop resulted in such
///        cases. The FAILED test resolves that situation.
///
/// -    SPICELIB Version 1.2.0, 9-MAR-1989 (HAN)
///
///         Parameters BEGDAT and BEGTXT have been moved into the
///         $Declarations section.
///
/// -    SPICELIB Version 1.1.0, 16-FEB-1989 (IMU) (NJB)
///
///         Parameters MAXVAR, MAXVAL, MAXLEN moved into $Declarations.
///         (Actually, MAXLEN was implicitly 32 characters, and has only
///         now been made an explicit---and changeable---limit.)
///
///         Declaration of unused function FAILED removed.
/// ```
pub fn wrpool(ctx: &mut SpiceContext, unit: i32) -> crate::Result<()> {
    WRPOOL(unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRPOOL ( Write the variables in pool to a specified unit )
pub fn WRPOOL(UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"WRPOOL", ctx)?;
    }

    //
    // Indicate the beginning of a data section.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,A)")?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&save.BEGDAT)?;
            writer.finish()?;
            Ok(())
        })?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,A)")?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        IOERR(
            b"writing a variable to the output kernel file ",
            b" ",
            save.IOSTAT,
            ctx,
        );
        SIGERR(b"SPICE(WRITEERROR)", ctx)?;
        CHKOUT(b"WRPOOL", ctx)?;
        return Ok(());
    }

    //
    // Next prepare for writing out the data.
    //
    save.IQUOTE = intrinsics::ICHAR(QUOTE);
    save.MARGIN = (MAXLEN + 6);

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXVAR;
        let m3__: i32 = 1;
        save.K = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the head of this list.
            //
            save.NNODE = save.NAMLST[save.K];

            while (save.NNODE > 0) {
                fstr::assign(&mut save.LINE, save.PNAMES.get(save.NNODE));
                save.DATAHD = save.DATLST[save.NNODE];
                save.DP = (save.DATAHD > 0);
                save.CHR = (save.DATAHD < 0);
                save.DNODE = i32::abs(save.DATAHD);
                //
                // Determine whether or not this is a vector object.
                //
                if save.DP {
                    save.VECTOR = (save.DPPOOL[[NEXT, save.DNODE]] > 0);
                } else if save.CHR {
                    save.VECTOR = (save.CHPOOL[[NEXT, save.DNODE]] > 0);
                } else {
                    SETMSG(b"This error is never supposed to occur. No data was available for the variable \'#\'. ", ctx);

                    save.R = RTRIM(&save.PNAMES[save.NNODE]);
                    ERRCH(
                        b"#",
                        fstr::substr(&save.PNAMES[save.NNODE], 1..=save.R),
                        ctx,
                    );
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"WRPOOL", ctx)?;
                    return Ok(());
                }
                //
                // If still here, then we can set up the beginning of this
                // output line.
                //
                fstr::assign(fstr::substr_mut(&mut save.LINE, (MAXLEN + 2)..), b"= ");

                if save.VECTOR {
                    fstr::assign(fstr::substr_mut(&mut save.LINE, (MAXLEN + 4)..), b"( ");
                }
                //
                // Now fetch all of the data associated with this variable.
                // We'll write them out one per line.
                //
                while (save.DNODE > 0) {
                    //
                    // Get the next data value and the address of the next node.
                    //
                    if save.DP {
                        save.DVALUE = save.DPVALS[save.DNODE];
                        save.DNODE = save.DPPOOL[[NEXT, save.DNODE]];
                    } else {
                        fstr::assign(&mut save.CVALUE, QUOTE);
                        save.J = 1;
                        //
                        // We have to double up each of the quotes on output.
                        // For this reason we copy the letters one at a time
                        // into the output holding area CVALUE.
                        //
                        {
                            let m1__: i32 = 1;
                            let m2__: i32 = RTRIM(&save.CHVALS[save.DNODE]);
                            let m3__: i32 = 1;
                            save.I = m1__;
                            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                save.J = (save.J + 1);
                                fstr::assign(
                                    fstr::substr_mut(&mut save.CVALUE, save.J..=save.J),
                                    fstr::substr(save.CHVALS.get(save.DNODE), save.I..=save.I),
                                );

                                save.CODE = intrinsics::ICHAR(fstr::substr(
                                    &save.CHVALS[save.DNODE],
                                    save.I..=save.I,
                                ));

                                if ISQUOT(save.CODE, save.IQUOTE) {
                                    save.J = (save.J + 1);
                                    fstr::assign(
                                        fstr::substr_mut(&mut save.CVALUE, save.J..=save.J),
                                        fstr::substr(save.CHVALS.get(save.DNODE), save.I..=save.I),
                                    );
                                }
                                save.I += m3__;
                            }
                        }

                        save.J = (save.J + 1);
                        fstr::assign(fstr::substr_mut(&mut save.CVALUE, save.J..=save.J), QUOTE);
                        save.DNODE = save.CHPOOL[[NEXT, save.DNODE]];
                    }

                    //
                    // We will need to properly finish off this write with
                    // either a comma, a blank or a right parenthesis.
                    //
                    if (save.DNODE > 0) {
                        fstr::assign(&mut save.FINISH, b", ");
                    } else if save.VECTOR {
                        fstr::assign(&mut save.FINISH, b" )");
                    } else {
                        fstr::assign(&mut save.FINISH, b" ");
                    }
                    //
                    // Now write out our data.
                    //
                    if save.DP {
                        {
                            use f2rust_std::{
                                data::Val,
                                io::{self, Writer},
                            };

                            let mut writer = io::FormattedWriter::new(
                                ctx.io_unit(UNIT)?,
                                None,
                                b"(1X,A,D25.17,A)",
                            )?;
                            save.IOSTAT = io::capture_iostat(|| {
                                writer.start()?;
                                writer.write_str(fstr::substr(&save.LINE, 1..=save.MARGIN))?;
                                writer.write_f64(save.DVALUE)?;
                                writer.write_str(&save.FINISH)?;
                                writer.finish()?;
                                Ok(())
                            })?;
                        }
                    } else {
                        {
                            use f2rust_std::{
                                data::Val,
                                io::{self, Writer},
                            };

                            let mut writer =
                                io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,3A)")?;
                            save.IOSTAT = io::capture_iostat(|| {
                                writer.start()?;
                                writer.write_str(fstr::substr(&save.LINE, 1..=save.MARGIN))?;
                                writer.write_str(fstr::substr(&save.CVALUE, 1..=save.J))?;
                                writer.write_str(&save.FINISH)?;
                                writer.finish()?;
                                Ok(())
                            })?;
                        }
                    }

                    //
                    // Check the IOSTAT code.  After all, that's why it's there.
                    //
                    if (save.IOSTAT != 0) {
                        IOERR(
                            b"writing a variable to the output kernel file ",
                            b" ",
                            save.IOSTAT,
                            ctx,
                        );
                        SIGERR(b"SPICE(WRITEERROR)", ctx)?;
                        CHKOUT(b"WRPOOL", ctx)?;
                        return Ok(());
                    }

                    //
                    // Blank out the output line so that we'll have
                    // leading blanks for subsequent components of the
                    // vector (if we are in fact writing one).
                    //
                    fstr::assign(&mut save.LINE, b" ");
                }
                //
                // Get the next name for this node:
                //
                save.NNODE = save.NMPOOL[[NEXT, save.NNODE]];
            }
            //
            // Get the next node (if there is one).
            //
            save.K += m3__;
        }
    }

    //
    // Indicate the beginning of a text section. Data sections and
    // text sections must alternate, even if the text section is blank.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,A)")?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.finish()?;
            Ok(())
        })?;
    }
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,A)")?;
        save.IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&save.BEGTXT)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (save.IOSTAT != 0) {
        IOERR(
            b"writing a variable to the output kernel file ",
            b" ",
            save.IOSTAT,
            ctx,
        );
        SIGERR(b"SPICE(WRITEERROR)", ctx)?;
        CHKOUT(b"WRPOOL", ctx)?;
        return Ok(());
    }

    CHKOUT(b"WRPOOL", ctx)?;
    Ok(())
}

/// Set watch on a pool variable
///
/// Add a name to the list of agents to notify whenever a member of
/// a list of kernel variables is updated.
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
///  AGENT      I   The name of an agent to be notified after updates.
///  NNAMES     I   The number of variables to associate with AGENT.
///  NAMES      I   Variable names whose update causes the notice.
/// ```
///
/// # Detailed Input
///
/// ```text
///  AGENT    is the name of a routine or entry point (agency) that
///           will want to know when the kernel pool variables
///           designated by NAMES have been updated.
///
///  NNAMES   is the number of kernel pool variable names that will
///           be associated with AGENT.
///
///  NAMES    is an array of names of variables in the kernel pool.
///           Whenever any of these is updated, a notice will be
///           posted for AGENT so that one can quickly check
///           whether needed data has been modified.
///
///           Any kernel variable may be associated with multiple
///           agents; this call adds AGENT to each set of agents
///           associated with a member of NAMES.
///
///           The variables designated by NAMES need not exist in
///           the kernel pool at the time a watch is set.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If sufficient room is not available to hold a new kernel
///      variable name, the error SPICE(KERVARSETOVERFLOW) is signaled.
///
///  2)  If sufficient room is not available to hold a new agent
///      name, the error SPICE(TOOMANYWATCHES) is signaled.
///
///  3)  If any kernel variable in the array NAMES is already watched
///      by MAXAGT agents, and AGENT is not already associated with
///      that kernel variable, the error SPICE(AGENTLISTOVERFLOW) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The kernel pool is a convenient place to store a wide
///  variety of data needed by routines in SPICELIB and routines
///  that interface with SPICELIB routines. However, when
///  a single name has a large quantity of data associated with
///  it, it becomes inefficient to constantly query the kernel
///  pool for values that are not updated on a frequent basis.
///
///  This entry point allows a routine to instruct the kernel pool
///  to post a message whenever a particular value gets updated.
///  In this way, a routine can quickly determine whether or not
///  data it requires has been updated since the last time the
///  data was accessed. This makes it reasonable to buffer
///  the data in local storage and update it only when
///  a variable in the kernel pool that affects this data has
///  been updated.
///
///  Note that SWPOOL has a side effect. Whenever a call to
///  SWPOOL is made, the agent specified in the calling sequence
///  is added to the list of agents that should be notified that
///  an update of its variables has occurred. In other words
///  the code
///
///      CALL SWPOOL ( AGENT, NNAMES, NAMES  )
///      CALL CVPOOL ( AGENT,         UPDATE )
///
///  will always return UPDATE as .TRUE.
///
///  This feature allows for a slightly cleaner use of SWPOOL and
///  CVPOOL as shown in the example below. Because SWPOOL
///  automatically loads AGENT into the list of agents to notify of
///  a kernel pool update, you do not have to include the code for
///  fetching the initial values of the kernel variables in the
///  initialization portion of a subroutine. Instead, the code for
///  the first fetch from the pool is the same as the code for
///  fetching when the pool is updated.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have an application subroutine, MYTASK, that
///  needs to access a large data set in the kernel pool. If this
///  data could be kept in local storage and kernel pool queries
///  performed only when the data in the kernel pool has been
///  updated, the routine can perform much more efficiently.
///
///  The code fragment below illustrates how you might make use of this
///  feature.
///
///  C
///  C     On the first call to this routine establish those variables
///  C     that we will want to read from the kernel pool only when
///  C     new values have been established.
///  C
///        IF ( FIRST ) THEN
///
///           FIRST = .FALSE.
///           HAVE  = .FALSE.
///
///           CALL SWPOOL ( 'MYTASK', NNAMES, NAMES )
///
///        END IF
///
///  C
///  C     If any of the variables has been updated, fetch
///  C     it from the kernel pool. (Note that this also
///  C     handles getting variables for the first time.)
///  C     We use HAVE to indicate the fetch succeeded. If it
///  C     didn't, we need to attempt the fetch on the next
///  C     pass into this routine.
///  C
///        CALL CVPOOL ( 'MYTASK', UPDATE )
///
///        IF (  UPDATE  .OR (.NOT. HAVE ) ) THEN
///
///           CALL GDPOOL ( 'MYTASK_VAR_1', 1, M, N1, VALS1, FOUND(1) )
///           CALL GDPOOL ( 'MYTASK_VAR_2', 1, M, N2, VALS2, FOUND(2) )
///                   .
///                   .
///                   .
///           CALL GDPOOL ( 'MYTASK_VAR_N', 1, M, NN, VALSN, FOUND(N) )
///
///        END IF
///
///        IF ( FAILED() ) THEN
///              .
///              .
///           do something about the failure
///              .
///              .
///
///        ELSE
///
///           HAVE = .TRUE.
///
///        END IF
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
/// -    SPICELIB Version 8.2.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.2.0, 30-JUL-2013 (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         This routine was re-written to work with the new
///         watcher system implementation. Several bugs related
///         to watch system overflow were fixed.
///
///         The code example was updated to handle kernel pool
///         fetch failure.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL and CVPOOL were added.
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         This routine was re-written to work with the new
///         watcher system implementation.
///
///         Several bugs related to watch system overflow were fixed.
///         Now overflow error checks are performed *before* the
///         watcher system is updated, so a partial update won't
///         occur if there's not enough room for a full update.
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL (set watch on a pool variable)
///         and CVPOOL (check variable for update) so that routines
///         that buffer data stored in the kernel pool can fetch
///         that data only when it is updated.
///
///         In addition, the revision history was upgraded so that the
///         version number increases over time. This wasn't true
///         before. In addition some early revision data that referred to
///         pre-SPICELIB modifications were removed. This editing of
///         the version numbers makes it unlikely that anyone can track
///         down which previous version of this routine they have by
///         looking at the version number. The best way to determine
///         the routine you had previously is to compare the dates
///         stored in the Version line of the routine.
/// ```
pub fn swpool(
    ctx: &mut SpiceContext,
    agent: &str,
    nnames: i32,
    names: CharArray,
) -> crate::Result<()> {
    SWPOOL(agent.as_bytes(), nnames, names, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SWPOOL ( Set watch on a pool variable )
pub fn SWPOOL(
    AGENT: &[u8],
    NNAMES: i32,
    NAMES: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let NAMES = DummyCharArray::new(NAMES, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SWPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter. Although setting a watcher does not
    // change the POOL we will increment the POOL state counter to make
    // sure that the next call to CVPOOL with this watcher triggers the
    // initial update.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Do all of the error checking we need to do BEFORE touching
    // the watcher data structure. We don't want to end up with
    // a partial update due to running out of room in mid-update.
    //
    // First make sure we can handle any new kernel variable names.
    //
    save.NEED = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NNAMES;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if !ELEMC(&NAMES[save.I], save.WTVARS.as_arg(), ctx)? {
                save.NEED = (save.NEED + 1);
            }

            save.I += m3__;
        }
    }

    save.SPACE = (SIZEC(save.WTVARS.as_arg(), ctx)? - CARDC(save.WTVARS.as_arg(), ctx)?);

    if (save.NEED > save.SPACE) {
        SETMSG(b"The watched kernel variable name list WTVARS has room for # more elements, so the # new names (in a list of # names) associated with agent # cannot be inserted.", ctx);
        ERRINT(b"#", save.SPACE, ctx);
        ERRINT(b"#", save.NEED, ctx);
        ERRINT(b"#", NNAMES, ctx);
        ERRCH(b"#", AGENT, ctx);
        SIGERR(b"SPICE(KERVARSETOVERFLOW)", ctx)?;
        CHKOUT(b"SWPOOL", ctx)?;
        return Ok(());
    }

    //
    // If the input agent is a new one for any member of NAMES,
    // make sure we have enough room to store this agent. Also
    // check for kernel variables that would have more than
    // MAXAGT agents watching them if this watch were established.
    //
    save.NEED = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NNAMES;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the agents associated with NAMES(I). The output argument
            // ACTIVE is a SPICE set.
            //
            ZZGAPOOL(
                &NAMES[save.I],
                save.WTVARS.as_arg(),
                save.WTPTRS.as_slice(),
                save.WTPOOL.as_slice(),
                save.WTAGNT.as_arg(),
                save.ACTIVE.as_arg_mut(),
                ctx,
            )?;

            save.NFETCH = CARDC(save.ACTIVE.as_arg(), ctx)?;

            save.NOAGNT = ((save.NFETCH == 0) || !ELEMC(AGENT, save.ACTIVE.as_arg(), ctx)?);

            if save.NOAGNT {
                save.NEED = (save.NEED + 1);
                //
                // Check the number of agents already associated with the
                // current kernel variable.
                //
                if (save.NFETCH == MAXAGT) {
                    SETMSG(b"The list of agents to notify when # is updated is too big. The maximum number of agents that any kernelpool variable can activate is #.", ctx);
                    ERRCH(b"#", &NAMES[save.I], ctx);
                    ERRINT(b"#", MAXAGT, ctx);
                    SIGERR(b"SPICE(TOOMANYWATCHES)", ctx)?;
                    CHKOUT(b"SWPOOL", ctx)?;
                    return Ok(());
                }
            }

            save.I += m3__;
        }
    }

    //
    // See whether WTAGNT has enough room to set this watch.
    //
    save.SPACE = LNKNFN(save.WTPOOL.as_slice());

    if (save.NEED > save.SPACE) {
        SETMSG(b"The watched kernel variable agent list WTAGNT has room for # more elements, so the # new occurrences of agent # required for the input watch cannot be inserted.", ctx);
        ERRINT(b"#", save.SPACE, ctx);
        ERRINT(b"#", save.NEED, ctx);
        ERRCH(b"#", AGENT, ctx);
        SIGERR(b"SPICE(AGENTLISTOVERFLOW)", ctx)?;
        CHKOUT(b"SWPOOL", ctx)?;
        return Ok(());
    }

    //
    // All of the overflow checks have been done. We finally can
    // get on with setting the specified watch.
    //
    // For each variable specified by the array NAMES, put AGENT
    // into its list of guys to be notified when a variable change
    // occurs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NNAMES;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the agents associated with NAMES(I). The output argument
            // ACTIVE is a SPICE set.
            //
            ZZGAPOOL(
                &NAMES[save.I],
                save.WTVARS.as_arg(),
                save.WTPTRS.as_slice(),
                save.WTPOOL.as_slice(),
                save.WTAGNT.as_arg(),
                save.ACTIVE.as_arg_mut(),
                ctx,
            )?;

            save.NFETCH = CARDC(save.ACTIVE.as_arg(), ctx)?;

            //
            // Three things can happen now:
            //
            //    1) The kernel variable NAMES(I) is already watched by at
            //       least one agent, but not by AGENT. We need to add AGENT
            //       to the list of agents watching NAMES(I).
            //
            //    2) The kernel variable NAMES(I) isn't yet watched by any
            //       agent, so we need to insert NAMES(I) into WTVARS, as
            //       well as add AGENT to the (empty) list of agents watching
            //       NAMES(I).
            //
            //    3) The kernel variable NAMES(I) is already watched by AGENT.
            //       No action is needed.
            //
            // We could get fancy and try to minimize the number of lines of
            // code required to handle the first two cases...but we won't.
            // We'll just take them one at a time.
            //
            //
            if (save.NFETCH > 0) {
                if !ELEMC(AGENT, save.ACTIVE.as_arg(), ctx)? {
                    //
                    // Case 1: at least one agent is already watching NAMES(I),
                    // but AGENT is not watching NAMES(I). We need the head of
                    // the agent list for this kernel variable.
                    //
                    save.J = BSRCHC(
                        &NAMES[save.I],
                        CARDC(save.WTVARS.as_arg(), ctx)?,
                        save.WTVARS.subarray(1),
                    );

                    save.HEAD = save.WTPTRS[save.J];

                    //
                    // Allocate a free node in the watch pool; append this node
                    // to the tail of the agent list for the kernel variable;
                    // we know that list is non-empty.
                    //
                    LNKAN(save.WTPOOL.as_slice_mut(), &mut save.NODE, ctx)?;

                    save.TAIL = LNKTL(save.HEAD, save.WTPOOL.as_slice(), ctx)?;

                    LNKILA(save.TAIL, save.NODE, save.WTPOOL.as_slice_mut(), ctx)?;

                    //
                    // Store the agent name at index NODE in the agent list.
                    //
                    fstr::assign(save.WTAGNT.get_mut(save.NODE), AGENT);

                    //
                    // The insertion is complete. We update AGENTS, which is
                    // the set of agents to notify, at the end of this routine.
                    //
                }
            } else {
                //
                // Case 2: the kernel variable NAMES(I) isn't watched. Add it
                // the watcher system. We've already ensured that there's
                // room in WTVARS and WTAGNT and that the insertion won't give
                // NAMES(I) an excessive number of agents.
                //
                // Let J be the insertion index in WTVARS. Since NAMES(I)
                // isn't yet a member of WTWARS, the insertion index will
                // always follow that of the last element in WTVARS
                // less than NAMES(I).
                //
                save.J = (1 + LSTLTC(
                    &NAMES[save.I],
                    CARDC(save.WTVARS.as_arg(), ctx)?,
                    save.WTVARS.subarray(1),
                ));

                //
                // Note that we don't use INSRTC to add NAMES(I) to WTVARS
                // because we need the insertion index, and we don't want
                // to execute a redundant search to find it.
                //
                // We're now going to expand both the set WTVARS and the
                // parallel array WTPTRS by inserting new values at index J.
                // WTVARS(J) will receive the new kernel variable name
                // NAMES(I) and WTPTRS(J) will receive a new node in the watch
                // pool: this node provides an index into the agent list for
                // NAMES(I).
                //
                // Let NVARS be the size of the array WTVARS(1:*) prior to
                // the insertion. NVARS will be updated by INSLAC.
                //
                // NPTRS is the size of the associated pointer table WTPTRS.
                //
                save.NVARS = CARDC(save.WTVARS.as_arg(), ctx)?;
                save.NPTRS = save.NVARS;

                INSLAC(
                    NAMES.subarray(save.I),
                    1,
                    save.J,
                    save.WTVARS.subarray_mut(1),
                    &mut save.NVARS,
                    ctx,
                )?;

                //
                // WTVARS is actually a set, so we must update its cardinality.
                //
                SCARDC(save.NVARS, save.WTVARS.as_arg_mut(), ctx)?;

                //
                // Allocate a free node in the watch pool.
                //
                LNKAN(save.WTPOOL.as_slice_mut(), &mut save.NODE, ctx)?;

                //
                // Now insert NODE in the pointer table WTPTRS at index J.
                //
                INSLAI(
                    &[save.NODE],
                    1,
                    save.J,
                    save.WTPTRS.as_slice_mut(),
                    &mut save.NPTRS,
                    ctx,
                )?;

                //
                // Store the agent name at index NODE in the agent list.
                //
                fstr::assign(save.WTAGNT.get_mut(save.NODE), AGENT);

                //
                // The insertion is complete. We update AGENTS, which is the
                // set of agents to notify, at the end of this routine.
            }

            save.I += m3__;
        }
    }

    //
    // We ALWAYS put this agent into the list of agents to be notified.
    //
    INSRTC(AGENT, save.AGENTS.as_arg_mut(), ctx)?;

    //
    // That is all.
    //
    CHKOUT(b"SWPOOL", ctx)?;
    Ok(())
}

/// Check variable in the pool for update
///
/// Indicate whether or not any watched kernel variables that have a
/// specified agent on their notification list have been updated.
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
///  AGENT      I   Name of the agent to check for notices.
///  UPDATE     O   .TRUE. if variables for AGENT have been updated.
/// ```
///
/// # Detailed Input
///
/// ```text
///  AGENT    is the name of a subroutine, entry point, or significant
///           portion of code that needs to access variables in the
///           kernel pool. Generally this agent will buffer these
///           variables internally and fetch them from the kernel
///           pool only when they are updated.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UPDATE   is a logical flag that will be set to .TRUE. if the
///           variables in the kernel pool that are associated with
///           AGENT have been updated since the last call to CVPOOL.
///
///           UPDATE will be set to .TRUE. on the first call made for
///           the specified agent, whether or not the associated
///           variables have been updated since the agent was placed
///           on their notification list, as long as the agent is
///           associated with any watched variables.
/// ```
///
/// # Parameters
///
/// ```text
///  See the umbrella subroutine POOL.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point allows the calling program to determine
///  whether or not variables associated with with AGENT have
///  been updated. Making use of this entry point in conjunction
///  with the entry point SWPOOL (set watch on pool variables)
///  modules can buffer kernel pool variables they need and
///  fetch values from the kernel pool only when variables have
///  been updated.
///
///  Note that the call to CVPOOL has a side effect.
///  Two consecutive calls to CVPOOL with the same
///  AGENT will always result in the UPDATE being .FALSE.
///  on the second call. In other words, if you embed
///  the following two lines of code in a piece of code
///
///     CALL CVPOOL ( AGENT, UPDATE )
///     CALL CVPOOL ( AGENT, UPDATE )
///
///  and then test UPDATE, it will be .FALSE. The idea is
///  that once a call to CVPOOL has been made, the
///  kernel pool has performed its duty and notified the
///  calling routine that one of the AGENT's variables
///  has been updated. Consequently, on the second call
///  to CVPOOL above, the kernel pool will not have any
///  updates to report about any of AGENT's variables.
///
///  If, on the other hand, you have code such as
///
///     CALL CVPOOL ( AGENT, UPDATE )
///     CALL LDPOOL ( 'MYFILE.DAT'  )
///     CALL CVPOOL ( AGENT, UPDATE )
///
///  the value of UPDATE will be true if one of the variables
///  associated with AGENT was updated by the call to
///  LDPOOL (and that variable has been specified as one
///  to watch by call a call to SWPOOL).
///
///  It should also be noted that any call to CVPOOL that
///  occurs immediately after a call to SWPOOL will result in
///  UPDATE being returned as .TRUE. In other words, code
///  such as shown below, will always result in the value
///  of UPDATE as being returned .TRUE.
///
///     CALL SWPOOL ( AGENT, NNAMES, NAMES  )
///     CALL CVPOOL ( AGENT,         UPDATE )
///
///  See the header for SWPOOL for a full discussion of this
///  feature.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have an application subroutine, MYTASK, that
///  needs to access a large data set in the kernel pool. If this
///  data could be kept in local storage and kernel pool queries
///  performed only when the data in the kernel pool has been
///  updated, the routine can perform much more efficiently.
///
///  The code fragment below illustrates how you might make use of this
///  feature.
///
///  C
///  C     On the first call to this routine establish those variables
///  C     that we will want to read from the kernel pool only when
///  C     new values have been established.
///  C
///        IF ( FIRST ) THEN
///
///           FIRST = .FALSE.
///           HAVE  = .FALSE.
///
///           CALL SWPOOL ( 'MYTASK', NNAMES, NAMES )
///
///        END IF
///
///  C
///  C     If any of the variables has been updated, fetch
///  C     it from the kernel pool. (Note that this also
///  C     handles getting variables for the first time.)
///  C     We use HAVE to indicate the fetch succeeded. If it
///  C     didn't, we need to attempt the fetch on the next
///  C     pass into this routine.
///  C
///        CALL CVPOOL ( 'MYTASK', UPDATE )
///
///        IF (  UPDATE  .OR (.NOT. HAVE ) ) THEN
///
///           CALL GDPOOL ( 'MYTASK_VAR_1', 1, M, N1, VALS1, FOUND(1) )
///           CALL GDPOOL ( 'MYTASK_VAR_2', 1, M, N2, VALS2, FOUND(2) )
///                   .
///                   .
///                   .
///           CALL GDPOOL ( 'MYTASK_VAR_N', 1, M, NN, VALSN, FOUND(N) )
///
///        END IF
///
///        IF ( FAILED() ) THEN
///              .
///              .
///           do something about the failure
///              .
///              .
///
///        ELSE
///
///           HAVE = .TRUE.
///
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.1, 30-JUN-2014 (NJB)
///
///         Description of the output variable UPDATE now
///         mentions that the initial value of .TRUE. will
///         be returned after an agent is associated with
///         kernel variables.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
///         The code example was updated to handle kernel pool
///         fetch failure.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL and CVPOOL were added.
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The basic data structure used to maintain the list of
///         variable names and values was replaced with a hash table
///         implementation. Data and names are accessed by means
///         of a hash function and linked lists of pointers to existing
///         variable names and data values.
///
/// -    SPICELIB Version 6.0.0, 31-MAR-1992 (WLT)
///
///         The entry points SWPOOL (set watch on a pool variable)
///         and CVPOOL (check variable for update) so that routines
///         that buffer data stored in the kernel pool can fetch
///         that data only when it is updated.
///
///         In addition, the revision history was upgraded so that the
///         version number increases over time. This wasn't true
///         before. In addition some early revision data that referred to
///         pre-SPICELIB modifications were removed. This editing of
///         the version numbers makes it unlikely that anyone can track
///         down which previous version of this routine they have by
///         looking at the version number. The best way to determine
///         the routine you had previously is to compare the dates
///         stored in the Version line of the routine.
/// ```
pub fn cvpool(ctx: &mut SpiceContext, agent: &str, update: &mut bool) -> crate::Result<()> {
    CVPOOL(agent.as_bytes(), update, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CVPOOL ( Check variable in the pool for update)
pub fn CVPOOL(AGENT: &[u8], UPDATE: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CVPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Check to see if our agent is on the list of agents to be
    // notified.  If it is, we take this agent off the list---he's
    // now considered to have been notified.
    //
    *UPDATE = ELEMC(AGENT, save.AGENTS.as_arg(), ctx)?;

    if *UPDATE {
        REMOVC(AGENT, save.AGENTS.as_arg_mut(), ctx)?;
    }

    CHKOUT(b"CVPOOL", ctx)?;
    Ok(())
}

/// Get character data from the kernel pool
///
/// Return the character value of a kernel variable from the
/// kernel pool.
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
///  NAME       I   Name of the variable whose value is to be returned.
///  START      I   Which component to start retrieving for NAME
///  ROOM       I   The largest number of values to return.
///  N          O   Number of values returned for NAME.
///  CVALS      O   Values associated with NAME.
///  FOUND      O   .TRUE. if variable is in pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the variable whose values are to be
///           returned. If the variable is not in the pool with
///           character type, FOUND will be .FALSE.
///
///  START    is the index of the first component of NAME to return.
///           If START is less than 1, it will be treated as 1. If
///           START is greater than the total number of components
///           available for NAME, no values will be returned (N will
///           be set to zero). However, FOUND will still be set to
///           .TRUE.
///
///  ROOM     is the maximum number of components that should be
///           returned for this variable. (Usually it is the amount
///           of ROOM available in the array CVALS). If ROOM is
///           less than 1 the error SPICE(BADARRAYSIZE) will be
///           signaled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values associated with NAME that
///           are returned. It will always be less than or equal
///           to ROOM.
///
///           If NAME is not in the pool with character type, no
///           value is given to N.
///
///  CVALS    is the array of values associated with NAME.
///           If NAME is not in the pool with character type, no
///           values are given to the elements of CVALS.
///
///           If the length of CVALS is less than the length of
///           strings stored in the kernel pool (see MAXCHR) the
///           values returned will be truncated on the right.
///
///  FOUND    is .TRUE. if the variable is in the pool and has
///           character type, .FALSE. if it is not.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXCHR   is the maximum number of characters that can be
///           stored in a component of a string valued kernel
///           variable. This value is currently 80.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ROOM is less than one, the error
///      SPICE(BADARRAYSIZE) is signaled.
///
///  2)  If CVALS has declared length less than the size of a
///      string to be returned, the value will be truncated on
///      the right. See MAXCHR for the maximum stored size of
///      string variables.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user interface to retrieving
///  character data stored in the kernel pool. This interface
///  allows you to retrieve the data associated with a variable
///  in multiple accesses. Under some circumstances this alleviates
///  the problem of having to know in advance the maximum amount
///  of space needed to accommodate all kernel variables.
///
///  However, this method of access does come with a price. It is
///  always more efficient to retrieve all of the data associated
///  with a kernel pool data in one call than it is to retrieve
///  it in sections.
///
///  See also the entry points GDPOOL and GIPOOL.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data stored
///  in a kernel pool variable can be retrieved in pieces.
///
///  First we need some declarations.
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 3 )
///
///     CHARACTER*(8)         VARNAM
///     CHARACTER*(3)         INDENT
///     INTEGER               START
///     INTEGER               N
///     LOGICAL               FOUND
///     CHARACTER*(80)        CVALS(ROOM)
///
///
///  Next load the data in the file 'typical.ker' into the
///  kernel pool.
///
///     CALL LDPOOL ( 'typical.ker' )
///
///  Next we shall print the values stored for the kernel pool
///  variable 'MYDATA'
///
///     VARNAM = 'MYDATA'
///     INDENT = ' '
///     START  =  1
///
///     CALL GCPOOL ( VARNAM, START, ROOM, N, CVALS, FOUND )
///
///     IF ( .NOT. FOUND )
///        WRITE (*,*) 'There is no string data available for MYDATA.'
///     ELSE
///
///        WRITE (*,*) 'Values for MYDATA.'
///        WRITE (*,*)
///
///        DO I = 1, N
///           WRITE (*,*) INDENT, CVALS(I)
///        END DO
///
///        DO WHILE ( N .EQ. ROOM )
///
///           START = START + N
///           CALL GCPOOL ( VARNAM, START, ROOM, N, CVALS, FOUND )
///
///           DO I = 1, N
///              WRITE (*,*) INDENT, CVALS(I)
///           END DO
///
///        END DO
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added MAXCHR
///         description in $Parameters.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
/// ```
pub fn gcpool(
    ctx: &mut SpiceContext,
    name: &str,
    start: i32,
    room: i32,
    n: &mut i32,
    cvals: CharArrayMut,
    found: &mut bool,
) -> crate::Result<()> {
    GCPOOL(
        name.as_bytes(),
        start,
        room,
        n,
        cvals,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GCPOOL (Get character data from the kernel pool)
pub fn GCPOOL(
    NAME: &[u8],
    START: i32,
    ROOM: i32,
    N: &mut i32,
    CVALS: CharArrayMut,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CVALS = DummyCharArrayMut::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"GCPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Perform the one obvious error check first.
    //
    if (ROOM < 1) {
        SETMSG(b"The amount of room specified as available for output in the output array was: #.  The amount of room must be positive. ", ctx);

        ERRINT(b"#", ROOM, ctx);
        SIGERR(b"SPICE(BADARRAYSIZE)", ctx)?;
        CHKOUT(b"GCPOOL", ctx)?;
        return Ok(());
    }

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        *FOUND = false;
        CHKOUT(b"GCPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            *FOUND = false;
            CHKOUT(b"GCPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //
    save.DATAHD = save.DATLST[save.NODE];

    if (save.DATAHD > 0) {
        *N = 0;
        *FOUND = false;
        CHKOUT(b"GCPOOL", ctx)?;
        return Ok(());
    } else if (save.DATAHD == 0) {
        SETMSG(b"This is never supposed to happen.  The requested name, \'#\', was found in the name list, but the pointer to the head of the data for this variable is zero. Please note your activities and report this error to NAIF. ", ctx);
        ERRCH(b"#", fstr::substr(NAME, 1..=RTRIM(NAME)), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"GCPOOL", ctx)?;
        return Ok(());
    }

    *FOUND = true;
    save.K = 0;
    *N = 0;
    save.BEGIN = intrinsics::MAX0(&[START, 1]);
    save.NODE = -save.DATAHD;

    while (save.NODE > 0) {
        save.K = (save.K + 1);

        if (save.K >= save.BEGIN) {
            *N = (*N + 1);
            fstr::assign(CVALS.get_mut(*N), save.CHVALS.get(save.NODE));

            if (*N == ROOM) {
                CHKOUT(b"GCPOOL", ctx)?;
                return Ok(());
            }
        }

        save.NODE = save.CHPOOL[[NEXT, save.NODE]];
    }

    CHKOUT(b"GCPOOL", ctx)?;
    Ok(())
}

/// Get d.p. values from the kernel pool
///
/// Return the d.p. value of a kernel variable from the kernel pool.
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
///  NAME       I   Name of the variable whose value is to be returned.
///  START      I   Which component to start retrieving for NAME
///  ROOM       I   The largest number of values to return.
///  N          O   Number of values returned for NAME.
///  VALUES     O   Values associated with NAME.
///  FOUND      O   .TRUE. if variable is in pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the variable whose values are to be
///           returned. If the variable is not in the pool with
///           numeric type, FOUND will be .FALSE.
///
///  START    is the index of the first component of NAME to return.
///           If START is less than 1, it will be treated as 1. If
///           START is greater than the total number of components
///           available for NAME, no values will be returned (N will
///           be set to zero). However, FOUND will still be set to
///           .TRUE.
///
///  ROOM     is the maximum number of components that should be
///           returned for this variable. (Usually it is the amount
///           of ROOM available in the array VALUES). If ROOM is
///           less than 1 the error SPICE(BADARRAYSIZE) will be
///           signaled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values associated with NAME that
///           are returned. It will always be less than or equal
///           to ROOM.
///
///           If NAME is not in the pool with numeric type, no value
///           is given to N.
///
///  VALUES   is the array of values associated with NAME.
///           If NAME is not in the pool with numeric type, no
///           values are given to the elements of VALUES.
///
///  FOUND    is .TRUE. if the variable is in the pool and has numeric
///           type, .FALSE. if it is not.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ROOM is less than one, the error
///      SPICE(BADARRAYSIZE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user interface to retrieving
///  numeric data stored in the kernel pool. This interface
///  allows you to retrieve the data associated with a variable
///  in multiple accesses. Under some circumstances this alleviates
///  the problem of having to know in advance the maximum amount
///  of space needed to accommodate all kernel variables.
///
///  However, this method of access does come with a price. It is
///  always more efficient to retrieve all of the data associated
///  with a kernel pool data in one call than it is to retrieve
///  it in sections.
///
///  This routine should be used in place of RTPOOL when possible
///  as it avoids errors associated with writing data past the
///  end of an array.
///
///  See also the entry points GIPOOL and GCPOOL.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data stored
///  in a kernel pool variable can be retrieved in pieces.
///
///  First we need some declarations.
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 3 )
///
///     CHARACTER*(8)         VARNAM
///     CHARACTER*(3)         INDENT
///     INTEGER               START
///     INTEGER               N
///     LOGICAL               FOUND
///     DOUBLE PRECISION      VALUES(ROOM)
///
///
///  Next load the data in the file 'typical.ker' into the
///  kernel pool.
///
///
///
///     CALL LDPOOL ( 'typical.ker' )
///
///  Next we shall print the values stored for the kernel pool
///  variable 'MYDATA'
///
///     VARNAM = 'MYDATA'
///     INDENT = ' '
///     START  =  1
///
///     CALL GDPOOL ( VARNAM, START, ROOM, N, VALUES, FOUND )
///
///     IF ( .NOT. FOUND )
///        WRITE (*,*) 'There is no numeric data available for MYDATA.'
///     ELSE
///
///        WRITE (*,*) 'Values for MYDATA.'
///        WRITE (*,*)
///
///        DO I = 1, N
///           WRITE (*,*) INDENT, VALUES(I)
///        END DO
///
///        DO WHILE ( N .EQ. ROOM )
///
///           START = START + N
///           CALL GDPOOL ( VARNAM, START, ROOM, N, VALUES, FOUND )
///
///           DO I = 1, N
///              WRITE (*,*) INDENT, VALUES(I)
///           END DO
///
///        END DO
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
/// ```
pub fn gdpool(
    ctx: &mut SpiceContext,
    name: &str,
    start: i32,
    room: i32,
    n: &mut i32,
    values: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    GDPOOL(
        name.as_bytes(),
        start,
        room,
        n,
        values,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GDPOOL (Get d.p. values from the kernel pool)
pub fn GDPOOL(
    NAME: &[u8],
    START: i32,
    ROOM: i32,
    N: &mut i32,
    VALUES: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut VALUES = DummyArrayMut::new(VALUES, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"GDPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Perform the one obvious error check first.
    //
    if (ROOM < 1) {
        SETMSG(b"The amount of room specified as available for output in the output array was: #.  The amount of room must be positive. ", ctx);

        ERRINT(b"#", ROOM, ctx);
        SIGERR(b"SPICE(BADARRAYSIZE)", ctx)?;
        CHKOUT(b"GDPOOL", ctx)?;
        return Ok(());
    }

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        *FOUND = false;
        CHKOUT(b"GDPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            *FOUND = false;
            CHKOUT(b"GDPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //
    save.DATAHD = save.DATLST[save.NODE];

    if (save.DATAHD < 0) {
        *N = 0;
        *FOUND = false;
        CHKOUT(b"GDPOOL", ctx)?;
        return Ok(());
    } else if (save.DATAHD == 0) {
        SETMSG(b"This is never supposed to happen.  The requested name, \'#\', was found in the name list, but the pointer to the head of the data for this variable is zero. Please note your activities and report this error to NAIF. ", ctx);
        ERRCH(b"#", fstr::substr(NAME, 1..=RTRIM(NAME)), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"GDPOOL", ctx)?;
        return Ok(());
    }

    *FOUND = true;
    save.K = 0;
    *N = 0;
    save.BEGIN = intrinsics::MAX0(&[START, 1]);
    save.NODE = save.DATAHD;

    while (save.NODE > 0) {
        save.K = (save.K + 1);

        if (save.K >= save.BEGIN) {
            *N = (*N + 1);
            VALUES[*N] = save.DPVALS[save.NODE];

            if (*N == ROOM) {
                CHKOUT(b"GDPOOL", ctx)?;
                return Ok(());
            }
        }

        save.NODE = save.DPPOOL[[NEXT, save.NODE]];
    }

    CHKOUT(b"GDPOOL", ctx)?;
    Ok(())
}

/// Get integers from the kernel pool
///
/// Return the integer value of a kernel variable from the
/// kernel pool.
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
///  NAME       I   Name of the variable whose value is to be returned.
///  START      I   Which component to start retrieving for NAME
///  ROOM       I   The largest number of values to return.
///  N          O   Number of values returned for NAME.
///  IVALS      O   Values associated with NAME.
///  FOUND      O   .TRUE. if variable is in pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the variable whose values are to be
///           returned. If the variable is not in the pool with
///           numeric type, FOUND will be .FALSE.
///
///  START    is the index of the first component of NAME to return.
///           If START is less than 1, it will be treated as 1. If
///           START is greater than the total number of components
///           available for NAME, no values will be returned (N will
///           be set to zero). However, FOUND will still be set to
///           .TRUE.
///
///  ROOM     is the maximum number of components that should be
///           returned for this variable. (Usually it is the amount
///           of ROOM available in the array IVALS). If ROOM is
///           less than 1 the error SPICE(BADARRAYSIZE) will be
///           signaled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of values associated with NAME that
///           are returned. It will always be less than or equal
///           to ROOM.
///
///           If NAME is not in the pool with numeric type, no value
///           is given to N.
///
///  IVALS    is the array of values associated with NAME. Any
///           numeric value having non-zero fractional part is
///           rounded to the closest integer. If NAME is not in the
///           pool or does not have numeric type, no values are
///           assigned to the elements of IVALS.
///
///  FOUND    is .TRUE. if the variable is in the pool and has numeric
///           type, .FALSE. if it is not.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ROOM is less than one, the error
///      SPICE(BADARRAYSIZE) is signaled.
///
///  2)  If a value requested is outside the valid range
///      of integers, the error SPICE(INTOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user interface for retrieving
///  integer data stored in the kernel pool. This interface
///  allows you to retrieve the data associated with a variable
///  in multiple accesses. Under some circumstances this alleviates
///  the problem of having to know in advance the maximum amount
///  of space needed to accommodate all kernel variables.
///
///  However, this method of access does come with a price. It is
///  always more efficient to retrieve all of the data associated
///  with a kernel pool data in one call than it is to retrieve
///  it in sections.
///
///  See also the entry points GDPOOL and GCPOOL.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the data stored
///  in a kernel pool variable can be retrieved in pieces.
///
///  First we need some declarations.
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 3 )
///
///     CHARACTER*(8)         VARNAM
///     CHARACTER*(3)         INDENT
///     INTEGER               START
///     INTEGER               N
///     LOGICAL               FOUND
///     INTEGER               IVALS(ROOM)
///
///
///  Next load the data in the file 'typical.ker' into the
///  kernel pool.
///
///     CALL LDPOOL ( 'typical.ker' )
///
///  Next we shall print the values stored for the kernel pool
///  variable 'MYDATA'
///
///     VARNAM = 'MYDATA'
///     INDENT = ' '
///     START  =  1
///
///     CALL GIPOOL ( VARNAM, START, ROOM, N, IVALS, FOUND )
///
///     IF ( .NOT. FOUND )
///        WRITE (*,*) 'There is no numeric data available for MYDATA.'
///     ELSE
///
///        WRITE (*,*) 'Values for MYDATA.'
///        WRITE (*,*)
///
///        DO I = 1, N
///           WRITE (*,*) INDENT, IVALS(I)
///        END DO
///
///        DO WHILE ( N .EQ. ROOM )
///
///           START = START + N
///           CALL GIPOOL ( VARNAM, START, ROOM, N, IVALS, FOUND )
///
///           DO I = 1, N
///              WRITE (*,*) INDENT, IVALS(I)
///           END DO
///
///        END DO
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.2, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 8.1.1, 14-JUL-2014 (NJB)
///
///         Updated description of IVALS in $Detailed_Output
///         header section.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
/// ```
pub fn gipool(
    ctx: &mut SpiceContext,
    name: &str,
    start: i32,
    room: i32,
    n: &mut i32,
    ivals: &mut [i32],
    found: &mut bool,
) -> crate::Result<()> {
    GIPOOL(
        name.as_bytes(),
        start,
        room,
        n,
        ivals,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GIPOOL (Get integers from the kernel pool)
pub fn GIPOOL(
    NAME: &[u8],
    START: i32,
    ROOM: i32,
    N: &mut i32,
    IVALS: &mut [i32],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IVALS = DummyArrayMut::new(IVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"GIPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Perform the one obvious error check first.
    //
    if (ROOM < 1) {
        SETMSG(b"The amount of room specified as available for output in the output array was: #.  The amount of room must be positive. ", ctx);

        ERRINT(b"#", ROOM, ctx);
        SIGERR(b"SPICE(BADARRAYSIZE)", ctx)?;
        CHKOUT(b"GIPOOL", ctx)?;
        return Ok(());
    }

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        *FOUND = false;
        CHKOUT(b"GIPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            *FOUND = false;
            CHKOUT(b"GIPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //
    save.DATAHD = save.DATLST[save.NODE];

    if (save.DATAHD < 0) {
        *N = 0;
        *FOUND = false;
        CHKOUT(b"GIPOOL", ctx)?;
        return Ok(());
    } else if (save.DATAHD == 0) {
        SETMSG(b"This is never supposed to happen.  The requested name, \'#\', was found in the name list, but the pointer to the head of the data for this variable is zero. Please note your activities and report this error to NAIF. ", ctx);
        ERRCH(b"#", fstr::substr(NAME, 1..=RTRIM(NAME)), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"GIPOOL", ctx)?;
        return Ok(());
    }
    //
    // Prepare for fetching values.
    //
    save.BIG = (INTMAX() as f64);
    save.SMALL = (INTMIN() as f64);
    *FOUND = true;
    save.K = 0;
    *N = 0;
    save.BEGIN = intrinsics::MAX0(&[START, 1]);
    save.NODE = save.DATAHD;

    while (save.NODE > 0) {
        save.K = (save.K + 1);

        if (save.K >= save.BEGIN) {
            *N = (*N + 1);

            if ((save.DPVALS[save.NODE] >= save.SMALL) && (save.DPVALS[save.NODE] <= save.BIG)) {
                IVALS[*N] = intrinsics::IDNINT(save.DPVALS[save.NODE]);
            } else {
                SETMSG(b"The value associated with index # of the kernel variable # is outside the range of integers. The value stored was: # .", ctx);

                ERRINT(b"#", save.K, ctx);
                ERRCH(b"#", fstr::substr(NAME, 1..=RTRIM(NAME)), ctx);
                ERRDP(b"#", save.DPVALS[save.NODE], ctx);
                SIGERR(b"SPICE(INTOUTOFRANGE)", ctx)?;
                CHKOUT(b"GIPOOL", ctx)?;
                return Ok(());
            }
            if (*N == ROOM) {
                CHKOUT(b"GIPOOL", ctx)?;
                return Ok(());
            }
        }

        save.NODE = save.DPPOOL[[NEXT, save.NODE]];
    }

    CHKOUT(b"GIPOOL", ctx)?;
    Ok(())
}

/// Data for a kernel pool variable
///
/// Return the data about a kernel pool variable.
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
///  NAME       I   Name of the variable whose value is to be returned.
///  FOUND      O   .TRUE. if variable is in pool.
///  N          O   Number of values returned for NAME.
///  TYPE       O   Type of the variable 'C', 'N', 'X'
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the variable whose values are to be
///           returned.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FOUND    is .TRUE. if the variable is in the pool .FALSE. if it
///           is not.
///
///  N        is the number of values associated with NAME.
///           If NAME is not present in the pool N will be returned
///           with the value 0.
///
///  TYPE     is the type of the variable associated with NAME.
///
///               'C' if the data is character data
///               'N' if the data is numeric.
///               'X' if there is no variable NAME in the pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the name requested is not in the kernel pool, FOUND
///      will be set to .FALSE., N to zero and TYPE to 'X'.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to determine whether or not a kernel
///  pool variable is present and to determine its size and type
///  if it is.
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
///  1) The following program demonstrates how to determine the
///     properties of a stored kernel variable. The program prompts
///     for text kernel name and for the name of a kernel variable.
///     If the variable is present in the kernel pool, the dimension
///     and type of the variable are displayed.
///
///
///     Example code begins here.
///
///
///           PROGRAM DTPOOL_EX1
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
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 256 )
///
///           INTEGER               KVNMLN
///           PARAMETER           ( KVNMLN = 33 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(KVNMLN)    VARNAM
///           CHARACTER*(1)         VTYPE
///
///           INTEGER               N
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for the name of a text-kernel file.
///     C
///           CALL PROMPT ( 'Enter text-kernel name        > ', FNAME )
///
///     C
///     C     Load the kernel.
///     C
///           CALL FURNSH ( FNAME )
///
///           CALL PROMPT ( 'Enter name of kernel variable > ',
///          .               VARNAM )
///
///           CALL DTPOOL ( VARNAM, FOUND, N, VTYPE )
///
///           IF ( FOUND ) THEN
///              WRITE(*,*) ' '
///              WRITE(*,*) 'Properties of variable ',
///          .               VARNAM(:RTRIM(VARNAM)), ':'
///              WRITE(*,*) ' '
///              WRITE(*,*) '   Size:   ', N
///
///              IF ( VTYPE .EQ. 'C' ) THEN
///
///                 WRITE(*,*) '   Type:   Character'
///
///              ELSE
///
///                 WRITE(*,*) '   Type:   Numeric'
///              END IF
///
///           ELSE
///
///              WRITE(*,*) VARNAM,
///          .              ' is not present in the kernel pool.'
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the FK file named cas_v40.tf to ask for the
///     variable 'FRAME_-82104_NAME', the output was:
///
///
///     Enter text-kernel name        > cas_v40.tf
///     Enter name of kernel variable > FRAME_-82104_NAME
///
///      Properties of variable FRAME_-82104_NAME:
///
///         Size:              1
///         Type:   Character
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.1, 22-DEC-2004 (NJB)
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
///
/// -    SPICELIB Version 7.0.0, 20-SEP-1995 (WLT)
///
///         The implementation of the kernel pool was completely redone
///         to improve performance in loading and fetching data. In
///         addition the pool was upgraded so that variables may be
///         either string or numeric valued.
///
///         The entry points GCPOOL, GDPOOL, GIPOOL and DTPOOL were added
///         to the routine.
/// ```
pub fn dtpool(
    ctx: &mut SpiceContext,
    name: &str,
    found: &mut bool,
    n: &mut i32,
    type_: &mut str,
) -> crate::Result<()> {
    DTPOOL(
        name.as_bytes(),
        found,
        n,
        fstr::StrBytes::new(type_).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DTPOOL (Data for a kernel pool variable)
pub fn DTPOOL(
    NAME: &[u8],
    FOUND: &mut bool,
    N: &mut i32,
    TYPE: &mut [u8],
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
        CHKIN(b"DTPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Until we find otherwise, we shall assume there is no data
    // for this variable.
    //
    *FOUND = false;
    *N = 0;
    fstr::assign(TYPE, b"X");

    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        CHKOUT(b"DTPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NODE = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));

    while !save.SUCCES {
        save.NODE = save.NMPOOL[[NEXT, save.NODE]];

        if (save.NODE < 0) {
            CHKOUT(b"DTPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NODE));
    }
    //
    // If you get to this point, the variable NAME is present in the
    // list of names at PNAMES(NODE), ABS( DATLST(NODE) ) points to the
    // head of a linked list of values for this NAME.
    //

    save.DATAHD = save.DATLST[save.NODE];

    if (save.DATAHD < 0) {
        fstr::assign(TYPE, b"C");
        *FOUND = true;
        save.NODE = -save.DATAHD;

        while (save.NODE > 0) {
            *N = (*N + 1);
            save.NODE = save.CHPOOL[[NEXT, save.NODE]];
        }
    } else if (save.DATAHD > 0) {
        fstr::assign(TYPE, b"N");
        *FOUND = true;
        save.NODE = save.DATAHD;

        while (save.NODE > 0) {
            *N = (*N + 1);
            save.NODE = save.DPPOOL[[NEXT, save.NODE]];
        }
    } else if (save.DATAHD == 0) {
        SETMSG(b"This is never supposed to happen.  The requested name, \'#\', was found in the name list, but the pointer to the head of the data for this variable is zero. Please note your activities and report this error to NAIF. ", ctx);
        ERRCH(b"#", fstr::substr(NAME, 1..=RTRIM(NAME)), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"DTPOOL", ctx)?;

        return Ok(());
    }

    CHKOUT(b"DTPOOL", ctx)?;
    Ok(())
}

/// Put character strings into the kernel pool
///
/// Provide toolkit programmers a method for programmatically
/// inserting character data into the kernel pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   The kernel pool name to associate with CVALS.
///  N          I   The number of values to insert.
///  CVALS      I   An array of strings to insert into the kernel pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the kernel pool variable to associate
///           with the values supplied in the array CVALS
///
///  N        is the number of values to insert into the kernel pool.
///
///  CVALS    is an array of strings to insert into the kernel
///           pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NAME is already present in the kernel pool and there
///      is sufficient room to hold all values supplied in CVALS,
///      the old values associated with NAME will be overwritten.
///
///  2)  If there is not sufficient room to insert a new variable into
///      the kernel pool and NAME is not already present in the kernel
///      pool, an error is signaled by a routine in the call tree of
///      this routine.
///
///  3)  If there is not sufficient room to insert the values
///      associated with NAME, the error SPICE(NOMOREROOM) is signaled.
///
///  4)  If the kernel pool variable name length exceeds its maximum
///      allowed length (see Kernel Required Reading, kernel.req), the
///      error SPICE(BADVARNAME) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point provides a programmatic interface for inserting
///  character data into the SPICE kernel pool without reading an
///  external file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wish to supply default values for a program
///  so that it may function even in the absence of the appropriate
///  text kernels. You can use the entry points PCPOOL, PDPOOL
///  and PIPOOL to initialize the kernel pool with suitable
///  values at program initialization. The example below shows
///  how you might set up various kernel pool variables that might
///  be required by a program.
///
///
///     Set up the relationship between the EARTH_BODYFIXED frame
///     and the IAU_EARTH frame.
///
///     CALL IDENT  ( MATRIX )
///     CALL PCPOOL ( 'TKFRAME_EARTH_FIXED_SPEC',     1, 'MATRIX'    )
///     CALL PCPOOL ( 'TKFRAME_EARTH_FIXED_RELATIVE', 1, 'IAU_EARTH' )
///     CALL PDPOOL ( 'TKFRAME_EARTH_FIXED_MATRIX',   9,  MATRIX )
///
///
///     Load the IAU model for the earth's rotation and shape.
///
///
///     RA ( 1 ) =  0.0D0
///     RA ( 2 ) = -0.641D0
///     RA ( 3 ) =  0.0D0
///
///     DEC( 1 ) = 90.0D0
///     DEC( 2 ) = -0.557D0
///     DEC( 3 ) =  0.0D0
///
///     PM ( 1 ) = 190.16D0
///     PM ( 2 ) = 360.9856235D0
///     PM ( 3 ) =   0.0D0
///
///     R  ( 1 ) =  6378.140D0
///     R  ( 2 ) =  6378.140D0
///     R  ( 3 ) =  6356.75D0
///
///     CALL PDPOOL ( 'BODY399_POLE_RA',   3, RA  )
///     CALL PDPOOL ( 'BODY399_POLE_DEC',  3, DEC )
///     CALL PDPOOL ( 'BODY399_PM',        3, PM  )
///     CALL PDPOOL ( 'BODY399_RADII',     3, R   )
///
///
///     Set up a preliminary set of leapsecond values.
///
///     CALL PDPOOL ( 'DELTET/DELTA_T_A/',1, 32.184D0  )
///     CALL PDPOOL ( 'DELTET/K',         1,  1.657D-3 )
///     CALL PDPOOL ( 'DELTET/EB',        1,  1.671D-2 )
///
///     VALUES(1) = 6.23999600D0
///     VALUES(2) = 1.99096871D-7
///
///     CALL PDPOOL ( 'DELTET/M', 2, VALUES )
///
///
///     VALUES(  1 ) = 10
///     VALUES(  3 ) = 11
///     VALUES(  5 ) = 12
///     VALUES(  7 ) = 13
///     VALUES(  9 ) = 14
///     VALUES( 11 ) = 15
///     VALUES( 13 ) = 16
///     VALUES( 15 ) = 17
///     VALUES( 17 ) = 18
///     VALUES( 19 ) = 19
///     VALUES( 21 ) = 20
///     VALUES( 23 ) = 21
///     VALUES( 25 ) = 22
///     VALUES( 27 ) = 23
///     VALUES( 29 ) = 24
///     VALUES( 31 ) = 25
///     VALUES( 33 ) = 26
///     VALUES( 35 ) = 27
///     VALUES( 37 ) = 28
///     VALUES( 39 ) = 29
///     VALUES( 41 ) = 30
///     VALUES( 43 ) = 31
///
///     CALL TPARSE ( '1972-JAN-1', VALUES(2),  ERROR )
///     CALL TPARSE ( '1972-JUL-1', VALUES(4),  ERROR )
///     CALL TPARSE ( '1973-JAN-1', VALUES(6),  ERROR )
///     CALL TPARSE ( '1974-JAN-1', VALUES(8),  ERROR )
///     CALL TPARSE ( '1975-JAN-1', VALUES(10), ERROR )
///     CALL TPARSE ( '1976-JAN-1', VALUES(12), ERROR )
///     CALL TPARSE ( '1977-JAN-1', VALUES(14), ERROR )
///     CALL TPARSE ( '1978-JAN-1', VALUES(16), ERROR )
///     CALL TPARSE ( '1979-JAN-1', VALUES(18), ERROR )
///     CALL TPARSE ( '1980-JAN-1', VALUES(20), ERROR )
///     CALL TPARSE ( '1981-JUL-1', VALUES(22), ERROR )
///     CALL TPARSE ( '1982-JUL-1', VALUES(24), ERROR )
///     CALL TPARSE ( '1983-JUL-1', VALUES(26), ERROR )
///     CALL TPARSE ( '1985-JUL-1', VALUES(28), ERROR )
///     CALL TPARSE ( '1988-JAN-1', VALUES(30), ERROR )
///     CALL TPARSE ( '1990-JAN-1', VALUES(32), ERROR )
///     CALL TPARSE ( '1991-JAN-1', VALUES(34), ERROR )
///     CALL TPARSE ( '1992-JUL-1', VALUES(36), ERROR )
///     CALL TPARSE ( '1993-JUL-1', VALUES(38), ERROR )
///     CALL TPARSE ( '1994-JUL-1', VALUES(40), ERROR )
///     CALL TPARSE ( '1996-JAN-1', VALUES(42), ERROR )
///     CALL TPARSE ( '1997-JUL-1', VALUES(44), ERROR )
///
///     CALL PDPOOL ( 'DELTET/DELTA_AT',  44, VALUES )
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 9.1.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 9.1.0, 17-JAN-2014 (BVS) (NJB)
///
///         Updated to increment POOL state counter.
///         Updated $Index_Entries section.
///
/// -    SPICELIB Version 9.0.0, 24-MAY-2010 (EDW)
///
///         Added an error check on the length of the kernel pool variable
///         name argument to enforce the variable name length does not
///         exceed MAXLEN.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory instead
///         of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn pcpool(ctx: &mut SpiceContext, name: &str, n: i32, cvals: CharArray) -> crate::Result<()> {
    PCPOOL(name.as_bytes(), n, cvals, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCPOOL ( Put character strings into the kernel pool )
pub fn PCPOOL(NAME: &[u8], N: i32, CVALS: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CVALS = DummyCharArray::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if (N <= 0) {
        return Ok(());
    }

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PCPOOL", ctx)?;

    //
    // Check the variable name length; signal an error
    // if longer than MAXLEN.
    //
    save.VARLEN = intrinsics::LEN(fstr::substr(NAME, 1..=LASTNB(NAME)));

    if (save.VARLEN > MAXLEN) {
        SETMSG(b"The input kernel pool variable name exceeds the maximum allowed length of #1. The length of the variable name is #2, the offending variable name: \'#3\'.", ctx);

        ERRINT(b"#1", MAXLEN, ctx);
        ERRINT(b"#2", save.VARLEN, ctx);
        ERRCH(b"#3", NAME, ctx);
        SIGERR(b"SPICE(BADVARNAME)", ctx)?;
        CHKOUT(b"PCPOOL", ctx)?;
        return Ok(());
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Find out where the name for this item is located
    // in the data tables.
    //
    ZZGPNM(
        save.NAMLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.PNAMES.as_arg_mut(),
        save.DATLST.as_slice(),
        save.DPPOOL.as_slice(),
        save.DPVALS.as_slice(),
        save.CHPOOL.as_slice(),
        save.CHVALS.as_arg(),
        NAME,
        &mut save.GOTIT,
        &mut save.LOOKAT,
        &mut save.NAMEAT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"PCPOOL", ctx)?;
        return Ok(());
    }
    //
    // Determine how much room is available for inserting new d.p.s
    // values into the kernel pool.
    //
    save.AVAIL = LNKNFN(save.CHPOOL.as_slice());

    if save.GOTIT {
        //
        // If we found the specified variable in the kernel pool, we
        // may be able to free up some space before inserting data.
        // We need to take this into account when determining
        // the amount of free room in the pool.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];

        if (save.DATAHD > 0) {
            //
            // No extra strings will be freed.  We have whatever
            // free space is in the CHPOOL right now.
            //
        } else {
            //
            // Find out how many items are in the current
            // list of strings associated with the variable.
            //
            save.TOFREE = 0;
            save.NODE = -save.DATAHD;

            while (save.NODE > 0) {
                save.TOFREE = (save.TOFREE + 1);
                save.NODE = save.CHPOOL[[NEXT, save.NODE]];
            }
            //
            // Add the number we will free to the amount currently
            // free in the dp pool.
            //
            save.AVAIL = (save.AVAIL + save.TOFREE);
        }
    }

    //
    // If the AVAIL for new data is less than the number of items
    // to be added, we just bail out here.
    //
    if (save.AVAIL < N) {
        if !save.GOTIT {
            //
            // We need to perform some clean up.  We've allocated
            // a new name but it has nothing in it. On the other hand
            // if we found it don't need to do anything because we've
            // only read from the pool. We haven't altered anything.
            // But in that case we'll never get into this block of code.
            //
            ZZCLN(
                save.LOOKAT,
                save.NAMEAT,
                save.NAMLST.as_slice_mut(),
                save.DATLST.as_slice_mut(),
                save.NMPOOL.as_slice_mut(),
                save.CHPOOL.as_slice_mut(),
                save.DPPOOL.as_slice_mut(),
                ctx,
            )?;
        }

        SETMSG(b"There is not sufficient space available in the kernel pool to store the # items associated with the name #.  There is room to store only # items. ", ctx);
        ERRINT(b"#", N, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRINT(b"#", save.AVAIL, ctx);
        SIGERR(b"SPICE(NOMOREROOM)", ctx)?;
        CHKOUT(b"PCPOOL", ctx)?;
        return Ok(());
    }

    //
    // There is room to insert the data.  Free up any required
    // nodes.
    //
    if save.GOTIT {
        //
        // We need to free the data associated with this
        // variable.  But first make sure there will be room
        // to add data.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];
        save.DATLST[save.NAMEAT] = 0;

        if (save.DATAHD > 0) {
            //
            // This variable was character type we need to
            // free a linked list from the character data
            // pool.
            //
            save.HEAD = save.DATAHD;
            save.TAIL = -save.DPPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.DPPOOL.as_slice_mut(), ctx)?;
        } else {
            //
            // This variable was character type. We need to
            // free a linked list from the numeric pool.
            //
            save.HEAD = -save.DATAHD;
            save.TAIL = -save.CHPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.CHPOOL.as_slice_mut(), ctx)?;
        }
    }
    //
    // We have done all of the freeing and checking that
    // needs to be done.  Now add the data.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We are ready to go.  Allocate a node for this data
            // item. First make sure there is room to do so.
            //
            save.FREE = LNKNFN(save.CHPOOL.as_slice());

            if (save.FREE <= 0) {
                SETMSG(b"There is no room available for adding another character value to the kernel pool.", ctx);
                SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                CHKOUT(b"PCPOOL", ctx)?;
                return Ok(());
            }

            //
            // Allocate a node for storing this string value:
            //
            LNKAN(save.CHPOOL.as_slice_mut(), &mut save.CHNODE, ctx)?;

            if (save.DATLST[save.NAMEAT] == 0) {
                //
                // There was no data for this name yet.  We make
                // CHNODE be the head of the data list for this name.
                //
                save.DATLST[save.NAMEAT] = -save.CHNODE;
            } else {
                //
                // Put this node after the tail of the current list.
                //
                save.HEAD = -save.DATLST[save.NAMEAT];
                save.TAIL = -save.CHPOOL[[PREV, save.HEAD]];

                LNKILA(save.TAIL, save.CHNODE, save.CHPOOL.as_slice_mut(), ctx)?;
            }

            //
            // Finally insert this data item in the data buffer
            // at CHNODE.  Note any quotes will be doubled so we
            // have to undo this affect when we store the data.
            //
            fstr::assign(save.CHVALS.get_mut(save.CHNODE), CVALS.get(save.I));

            //
            // That's all for this value. It's now time to loop
            // back through and get the next value.
            //

            save.I += m3__;
        }
    }

    //
    // One last thing, see if this variable is being watched,
    // If it is, add its associated agents to the list of
    // AGENTS to be notified of a watched variable update.
    //
    if ELEMC(NAME, save.WTVARS.as_arg(), ctx)? {
        //
        // Union the update set AGENTS with the set of agents
        // associated with the variable NAME.
        //
        ZZNWPOOL(
            NAME,
            save.WTVARS.as_arg(),
            save.WTPTRS.as_slice(),
            save.WTPOOL.as_slice(),
            save.WTAGNT.as_arg(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"PCPOOL", ctx)?;
    Ok(())
}

/// Put d.p.'s into the kernel pool
///
/// Provide toolkit programmers a method for programmatically
/// inserting double precision data into the kernel pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   The kernel pool name to associate with VALUES.
///  N          I   The number of values to insert.
///  VALUES     I   An array of values to insert into the kernel pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the kernel pool variable to associate
///           with the values supplied in the array VALUES
///
///  N        is the number of values to insert into the kernel pool.
///
///  VALUES   is an array of d.p. values to insert into the kernel
///           pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NAME is already present in the kernel pool and there
///      is sufficient room to hold all values supplied in VALUES,
///      the old values associated with NAME will be overwritten.
///
///  2)  If there is not sufficient room to insert a new variable into
///      the kernel pool and NAME is not already present in the kernel
///      pool, an error is signaled by a routine in the call tree of
///      this routine.
///
///  3)  If there is not sufficient room to insert the values
///      associated with NAME, the error SPICE(NOMOREROOM) is signaled.
///
///  4)  If the kernel pool variable name length exceeds its maximum
///      allowed length (see Kernel Required Reading, kernel.req), the
///      error SPICE(BADVARNAME) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point provides a programmatic interface for inserting
///  data into the SPICE kernel pool without reading an external file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wish to supply default values for a program
///  so that it may function even in the absence of the appropriate
///  text kernels. You can use the entry points PCPOOL, PDPOOL
///  and PIPOOL to initialize the kernel pool with suitable
///  values at program initialization. The example below shows
///  how you might set up various kernel pool variables that might
///  be required by a program.
///
///
///     Set up the relationship between the EARTH_BODYFIXED frame
///     and the IAU_EARTH frame.
///
///     CALL IDENT  ( MATRIX )
///     CALL PCPOOL ( 'TKFRAME_EARTH_FIXED_SPEC',     1, 'MATRIX'    )
///     CALL PCPOOL ( 'TKFRAME_EARTH_FIXED_RELATIVE', 1, 'IAU_EARTH' )
///     CALL PDPOOL ( 'TKFRAME_EARTH_FIXED_MATRIX',   9,  MATRIX )
///
///
///     Load the IAU model for the earth's rotation and shape.
///
///
///     RA ( 1 ) =  0.0D0
///     RA ( 2 ) = -0.641D0
///     RA ( 3 ) =  0.0D0
///
///     DEC( 1 ) = 90.0D0
///     DEC( 2 ) = -0.557D0
///     DEC( 3 ) =  0.0D0
///
///     PM ( 1 ) = 190.16D0
///     PM ( 2 ) = 360.9856235D0
///     PM ( 3 ) =   0.0D0
///
///     R  ( 1 ) =  6378.140D0
///     R  ( 2 ) =  6378.140D0
///     R  ( 3 ) =  6356.75D0
///
///     CALL PDPOOL ( 'BODY399_POLE_RA',   3, RA  )
///     CALL PDPOOL ( 'BODY399_POLE_DEC',  3, DEC )
///     CALL PDPOOL ( 'BODY399_PM',        3, PM  )
///     CALL PDPOOL ( 'BODY399_RADII',     3, R   )
///
///
///     Set up a preliminary set of leapsecond values.
///
///     CALL PDPOOL ( 'DELTET/DELTA_T_A', 1, 32.184D0  )
///     CALL PDPOOL ( 'DELTET/K',         1,  1.657D-3 )
///     CALL PDPOOL ( 'DELTET/EB',        1,  1.671D-2 )
///
///     VALUES(1) = 6.23999600D0
///     VALUES(2) = 1.99096871D-7
///
///     CALL PDPOOL ( 'DELTET/M', 2, VALUES )
///
///
///     VALUES(  1 ) = 10
///     VALUES(  3 ) = 11
///     VALUES(  5 ) = 12
///     VALUES(  7 ) = 13
///     VALUES(  9 ) = 14
///     VALUES( 11 ) = 15
///     VALUES( 13 ) = 16
///     VALUES( 15 ) = 17
///     VALUES( 17 ) = 18
///     VALUES( 19 ) = 19
///     VALUES( 21 ) = 20
///     VALUES( 23 ) = 21
///     VALUES( 25 ) = 22
///     VALUES( 27 ) = 23
///     VALUES( 29 ) = 24
///     VALUES( 31 ) = 25
///     VALUES( 33 ) = 26
///     VALUES( 35 ) = 27
///     VALUES( 37 ) = 28
///     VALUES( 39 ) = 29
///     VALUES( 41 ) = 30
///     VALUES( 43 ) = 31
///
///     CALL TPARSE ( '1972-JAN-1', VALUES(2),  ERROR )
///     CALL TPARSE ( '1972-JUL-1', VALUES(4),  ERROR )
///     CALL TPARSE ( '1973-JAN-1', VALUES(6),  ERROR )
///     CALL TPARSE ( '1974-JAN-1', VALUES(8),  ERROR )
///     CALL TPARSE ( '1975-JAN-1', VALUES(10), ERROR )
///     CALL TPARSE ( '1976-JAN-1', VALUES(12), ERROR )
///     CALL TPARSE ( '1977-JAN-1', VALUES(14), ERROR )
///     CALL TPARSE ( '1978-JAN-1', VALUES(16), ERROR )
///     CALL TPARSE ( '1979-JAN-1', VALUES(18), ERROR )
///     CALL TPARSE ( '1980-JAN-1', VALUES(20), ERROR )
///     CALL TPARSE ( '1981-JUL-1', VALUES(22), ERROR )
///     CALL TPARSE ( '1982-JUL-1', VALUES(24), ERROR )
///     CALL TPARSE ( '1983-JUL-1', VALUES(26), ERROR )
///     CALL TPARSE ( '1985-JUL-1', VALUES(28), ERROR )
///     CALL TPARSE ( '1988-JAN-1', VALUES(30), ERROR )
///     CALL TPARSE ( '1990-JAN-1', VALUES(32), ERROR )
///     CALL TPARSE ( '1991-JAN-1', VALUES(34), ERROR )
///     CALL TPARSE ( '1992-JUL-1', VALUES(36), ERROR )
///     CALL TPARSE ( '1993-JUL-1', VALUES(38), ERROR )
///     CALL TPARSE ( '1994-JUL-1', VALUES(40), ERROR )
///     CALL TPARSE ( '1996-JAN-1', VALUES(42), ERROR )
///     CALL TPARSE ( '1997-JUL-1', VALUES(44), ERROR )
///
///     CALL PDPOOL ( 'DELTET/DELTA_AT',  44, VALUES )
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 9.1.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 9.1.0, 17-JAN-2014 (BVS) (NJB)
///
///         Updated to increment POOL state counter.
///         Updated $Index_Entries section.
///
/// -    SPICELIB Version 9.0.0, 24-MAY-2010 (EDW)
///
///         Added an error check on the length of the kernel pool variable
///         name argument to enforce the variable name length does not
///         exceed MAXLEN.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory instead
///         of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn pdpool(ctx: &mut SpiceContext, name: &str, n: i32, values: &[f64]) -> crate::Result<()> {
    PDPOOL(name.as_bytes(), n, values, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PDPOOL ( Put d.p.'s into the kernel pool )
pub fn PDPOOL(NAME: &[u8], N: i32, VALUES: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VALUES = DummyArray::new(VALUES, 1..);

    //
    // Standard SPICE error handling.
    //
    if (N <= 0) {
        return Ok(());
    }

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PDPOOL", ctx)?;

    //
    // Check the variable name length; signal an error
    // if longer than MAXLEN.
    //
    save.VARLEN = intrinsics::LEN(fstr::substr(NAME, 1..=LASTNB(NAME)));

    if (save.VARLEN > MAXLEN) {
        SETMSG(b"The input kernel pool variable name exceeds the maximum allowed length of #1. The length of the variable name is #2, the offending variable name: \'#3\'.", ctx);

        ERRINT(b"#1", MAXLEN, ctx);
        ERRINT(b"#2", save.VARLEN, ctx);
        ERRCH(b"#3", NAME, ctx);
        SIGERR(b"SPICE(BADVARNAME)", ctx)?;
        CHKOUT(b"PDPOOL", ctx)?;
        return Ok(());
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Find out where the name for this item is located
    // in the data tables.
    //
    ZZGPNM(
        save.NAMLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.PNAMES.as_arg_mut(),
        save.DATLST.as_slice(),
        save.DPPOOL.as_slice(),
        save.DPVALS.as_slice(),
        save.CHPOOL.as_slice(),
        save.CHVALS.as_arg(),
        NAME,
        &mut save.GOTIT,
        &mut save.LOOKAT,
        &mut save.NAMEAT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"PDPOOL", ctx)?;
        return Ok(());
    }
    //
    // Determine how much room is available for inserting new d.p.s
    // values into the kernel pool.
    //
    save.AVAIL = LNKNFN(save.DPPOOL.as_slice());

    if save.GOTIT {
        //
        // If we found the specified variable in the kernel pool, we
        // may be able to free up some space before inserting data.
        // We need to take this into account when determining
        // the amount of free room in the pool.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];

        if (save.DATAHD < 0) {
            //
            // No extra d.p.s will be freed.  We have whatever
            // free space is in the DPPOOL right now.
            //
        } else {
            //
            // Find out how many items are in the current
            // list of d.p. associated with the variable.
            //
            save.TOFREE = 0;
            save.NODE = save.DATAHD;

            while (save.NODE > 0) {
                save.TOFREE = (save.TOFREE + 1);
                save.NODE = save.DPPOOL[[NEXT, save.NODE]];
            }
            //
            // Add the number we will free to the amount currently
            // free in the dp pool.
            //
            save.AVAIL = (save.AVAIL + save.TOFREE);
        }
    }

    //
    // If the AVAIL for new data is less than the number of items
    // to be added, we just bail out here.
    //
    if (save.AVAIL < N) {
        if !save.GOTIT {
            //
            // We need to perform some clean up.  We've allocated
            // a new name but it has nothing in it. On the other hand
            // if we found it don't need to do anything because we've
            // only read from the pool. We haven't altered anything.
            // But in that case we'll never get into this block of code.
            //
            ZZCLN(
                save.LOOKAT,
                save.NAMEAT,
                save.NAMLST.as_slice_mut(),
                save.DATLST.as_slice_mut(),
                save.NMPOOL.as_slice_mut(),
                save.CHPOOL.as_slice_mut(),
                save.DPPOOL.as_slice_mut(),
                ctx,
            )?;
        }

        SETMSG(b"There is not sufficient space available in the kernel pool to store the # items associated with the name #.  There is room to store only # items. ", ctx);
        ERRINT(b"#", N, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRINT(b"#", save.AVAIL, ctx);
        SIGERR(b"SPICE(NOMOREROOM)", ctx)?;
        CHKOUT(b"PDPOOL", ctx)?;
        return Ok(());
    }

    //
    // There is room to insert the data.  Free up any required
    // nodes.
    //
    if save.GOTIT {
        //
        // We need to free the data associated with this
        // variable.  But first make sure there will be room
        // to add data.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];
        save.DATLST[save.NAMEAT] = 0;

        if (save.DATAHD < 0) {
            //
            // This variable was character type we need to
            // free a linked list from the character data
            // pool.
            //
            save.HEAD = -save.DATAHD;
            save.TAIL = -save.CHPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.CHPOOL.as_slice_mut(), ctx)?;
        } else {
            //
            // This variable was numeric type. We need to
            // free a linked list from the numeric pool.
            //
            save.HEAD = save.DATAHD;
            save.TAIL = -save.DPPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.DPPOOL.as_slice_mut(), ctx)?;
        }
    }
    //
    // We have done all of the freeing and checking that
    // needs to be done.  Now add the data.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // OK. See if there is room in
            // the numeric portion of the pool to store this value.
            //
            save.FREE = LNKNFN(save.DPPOOL.as_slice());

            if (save.FREE <= 0) {
                //
                // This branch of the code should never be exercised,
                // but it doesn't hurt to program in a redundant check.
                //
                ZZCLN(
                    save.LOOKAT,
                    save.NAMEAT,
                    save.NAMLST.as_slice_mut(),
                    save.DATLST.as_slice_mut(),
                    save.NMPOOL.as_slice_mut(),
                    save.CHPOOL.as_slice_mut(),
                    save.DPPOOL.as_slice_mut(),
                    ctx,
                )?;

                SETMSG(b"There is no room available for adding another numeric value to the kernel pool.", ctx);
                SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                CHKOUT(b"PDPOOL", ctx)?;
                return Ok(());
            }

            //
            // Allocate a node for storing this numeric value:
            //
            LNKAN(save.DPPOOL.as_slice_mut(), &mut save.DPNODE, ctx)?;

            if (save.DATLST[save.NAMEAT] == 0) {
                //
                // There was no data for this name yet.  We make
                // DPNODE be the head of the data list for this name.
                //
                save.DATLST[save.NAMEAT] = save.DPNODE;
            } else {
                //
                // Put this node after the tail of the current list.
                //
                save.HEAD = save.DATLST[save.NAMEAT];
                save.TAIL = -save.DPPOOL[[PREV, save.HEAD]];

                LNKILA(save.TAIL, save.DPNODE, save.DPPOOL.as_slice_mut(), ctx)?;
            }

            //
            // Finally insert this data item into the numeric buffer.
            //
            save.DPVALS[save.DPNODE] = VALUES[save.I];

            save.I += m3__;
        }
    }

    //
    // One last thing, see if this variable is being watched,
    // If it is, add its associated agents to the list of
    // AGENTS to be notified of a watched variable update.
    //
    if ELEMC(NAME, save.WTVARS.as_arg(), ctx)? {
        //
        // Union the update set AGENTS with the set of agents
        // associated with the variable NAME.
        //
        ZZNWPOOL(
            NAME,
            save.WTVARS.as_arg(),
            save.WTPTRS.as_slice(),
            save.WTPOOL.as_slice(),
            save.WTAGNT.as_arg(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"PDPOOL", ctx)?;
    Ok(())
}

/// Put integers into the kernel pool
///
/// Provide toolkit programmers a method for programmatically
/// inserting integer data into the kernel pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   The kernel pool name to associate with IVALS.
///  N          I   The number of values to insert.
///  IVALS      I   An array of integers to insert into the pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the kernel pool variable to associate
///           with the values supplied in the array IVALS
///
///  N        is the number of values to insert into the kernel pool.
///
///  IVALS    is an array of integers to insert into the kernel
///           pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NAME is already present in the kernel pool and there
///      is sufficient room to hold all values supplied in IVALS,
///      the old values associated with NAME will be overwritten.
///
///  2)  If there is not sufficient room to insert a new variable into
///      the kernel pool and NAME is not already present in the kernel
///      pool, an error is signaled by a routine in the call tree of
///      this routine.
///
///  3)  If there is not sufficient room to insert the values
///      associated with NAME, the error SPICE(NOMOREROOM) is signaled.
///
///  4)  If the kernel pool variable name length exceeds its maximum
///      allowed length (see Kernel Required Reading, kernel.req), the
///      error SPICE(BADVARNAME) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point provides a programmatic interface for inserting
///  data into the SPICE kernel pool without reading an external file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wish to supply default values for a program
///  so that it may function even in the absence of the appropriate
///  text kernels. You can use the entry points PCPOOL, PDPOOL
///  and PIPOOL to initialize the kernel pool with suitable
///  values at program initialization. The example below shows
///  how you might set up various kernel pool variables that might
///  be required by a program.
///
///
///     Set up the relationship between the EARTH_BODYFIXED frame
///     and the IAU_EARTH frame.
///
///     CALL IDENT ( MATRIX )
///     CALL PCPOOL ( 'TKFRAME_EARTH_FIXED_SPEC',     1, 'MATRIX' )
///     CALL PIPOOL ( 'TKFRAME_EARTH_FIXED_RELATIVE', 1,  10081   )
///     CALL PDPOOL ( 'TKFRAME_EARTH_FIXED_MATRIX',   9,  MATRIX  )
///
///
///     Load the IAU model for the earth's rotation and shape.
///
///
///     RA ( 1 ) =  0.0D0
///     RA ( 2 ) = -0.641D0
///     RA ( 3 ) =  0.0D0
///
///     DEC( 1 ) = 90.0D0
///     DEC( 2 ) = -0.557D0
///     DEC( 3 ) =  0.0D0
///
///     PM ( 1 ) = 190.16D0
///     PM ( 2 ) = 360.9856235D0
///     PM ( 3 ) =   0.0D0
///
///     R  ( 1 ) =  6378.140D0
///     R  ( 2 ) =  6378.140D0
///     R  ( 3 ) =  6356.75D0
///
///     CALL PDPOOL ( 'BODY399_POLE_RA',   3, RA  )
///     CALL PDPOOL ( 'BODY399_POLE_DEC',  3, DEC )
///     CALL PDPOOL ( 'BODY399_PM',        3, PM  )
///     CALL PDPOOL ( 'BODY399_RADII',     3, R   )
///
///
///     Set up a preliminary set of leapsecond values.
///
///     CALL PDPOOL ( 'DELTET/DELTA_T_A/',1, 32.184D0  )
///     CALL PDPOOL ( 'DELTET/K',         1,  1.657D-3 )
///     CALL PDPOOL ( 'DELTET/EB',        1,  1.671D-2 )
///
///     VALUES(1) = 6.23999600D0
///     VALUES(2) = 1.99096871D-7
///
///     CALL PDPOOL ( 'DELTET/M', 2, VALUES )
///
///
///     VALUES(  1 ) = 10
///     VALUES(  3 ) = 11
///     VALUES(  5 ) = 12
///     VALUES(  7 ) = 13
///     VALUES(  9 ) = 14
///     VALUES( 11 ) = 15
///     VALUES( 13 ) = 16
///     VALUES( 15 ) = 17
///     VALUES( 17 ) = 18
///     VALUES( 19 ) = 19
///     VALUES( 21 ) = 20
///     VALUES( 23 ) = 21
///     VALUES( 25 ) = 22
///     VALUES( 27 ) = 23
///     VALUES( 29 ) = 24
///     VALUES( 31 ) = 25
///     VALUES( 33 ) = 26
///     VALUES( 35 ) = 27
///     VALUES( 37 ) = 28
///     VALUES( 39 ) = 29
///     VALUES( 41 ) = 30
///     VALUES( 43 ) = 31
///
///     CALL TPARSE ( '1972-JAN-1', VALUES(2),  ERROR )
///     CALL TPARSE ( '1972-JUL-1', VALUES(4),  ERROR )
///     CALL TPARSE ( '1973-JAN-1', VALUES(6),  ERROR )
///     CALL TPARSE ( '1974-JAN-1', VALUES(8),  ERROR )
///     CALL TPARSE ( '1975-JAN-1', VALUES(10), ERROR )
///     CALL TPARSE ( '1976-JAN-1', VALUES(12), ERROR )
///     CALL TPARSE ( '1977-JAN-1', VALUES(14), ERROR )
///     CALL TPARSE ( '1978-JAN-1', VALUES(16), ERROR )
///     CALL TPARSE ( '1979-JAN-1', VALUES(18), ERROR )
///     CALL TPARSE ( '1980-JAN-1', VALUES(20), ERROR )
///     CALL TPARSE ( '1981-JUL-1', VALUES(22), ERROR )
///     CALL TPARSE ( '1982-JUL-1', VALUES(24), ERROR )
///     CALL TPARSE ( '1983-JUL-1', VALUES(26), ERROR )
///     CALL TPARSE ( '1985-JUL-1', VALUES(28), ERROR )
///     CALL TPARSE ( '1988-JAN-1', VALUES(30), ERROR )
///     CALL TPARSE ( '1990-JAN-1', VALUES(32), ERROR )
///     CALL TPARSE ( '1991-JAN-1', VALUES(34), ERROR )
///     CALL TPARSE ( '1992-JUL-1', VALUES(36), ERROR )
///     CALL TPARSE ( '1993-JUL-1', VALUES(38), ERROR )
///     CALL TPARSE ( '1994-JUL-1', VALUES(40), ERROR )
///     CALL TPARSE ( '1996-JAN-1', VALUES(42), ERROR )
///     CALL TPARSE ( '1997-JUL-1', VALUES(44), ERROR )
///
///     CALL PDPOOL ( 'DELTET/DELTA_AT',  44, VALUES )
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 9.1.1, 27-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 9.1.0, 17-JAN-2014 (BVS) (NJB)
///
///         Updated to increment POOL state counter.
///         Updated $Index_Entries section.
///
/// -    SPICELIB Version 9.0.0, 24-MAY-2010 (EDW)
///
///         Added an error check on the length of the kernel pool variable
///         name argument to enforce the variable name length does not
///         exceed MAXLEN.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory instead
///         of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn pipool(ctx: &mut SpiceContext, name: &str, n: i32, ivals: &[i32]) -> crate::Result<()> {
    PIPOOL(name.as_bytes(), n, ivals, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PIPOOL ( Put integers into the kernel pool )
pub fn PIPOOL(NAME: &[u8], N: i32, IVALS: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let IVALS = DummyArray::new(IVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if (N <= 0) {
        return Ok(());
    }

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PIPOOL", ctx)?;

    //
    // Check the variable name length; signal an error
    // if longer than MAXLEN.
    //
    save.VARLEN = intrinsics::LEN(fstr::substr(NAME, 1..=LASTNB(NAME)));

    if (save.VARLEN > MAXLEN) {
        SETMSG(b"The input kernel pool variable name exceeds the maximum allowed length of #1. The length of the variable name is #2, the offending variable name: \'#3\'.", ctx);

        ERRINT(b"#1", MAXLEN, ctx);
        ERRINT(b"#2", save.VARLEN, ctx);
        ERRCH(b"#3", NAME, ctx);
        SIGERR(b"SPICE(BADVARNAME)", ctx)?;
        CHKOUT(b"PIPOOL", ctx)?;
        return Ok(());
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Find out where the name for this item is located
    // in the data tables.
    //
    ZZGPNM(
        save.NAMLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.PNAMES.as_arg_mut(),
        save.DATLST.as_slice(),
        save.DPPOOL.as_slice(),
        save.DPVALS.as_slice(),
        save.CHPOOL.as_slice(),
        save.CHVALS.as_arg(),
        NAME,
        &mut save.GOTIT,
        &mut save.LOOKAT,
        &mut save.NAMEAT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"PIPOOL", ctx)?;
        return Ok(());
    }
    //
    // Determine how much room is available for inserting new d.p.s
    // values into the kernel pool.
    //
    save.AVAIL = LNKNFN(save.DPPOOL.as_slice());

    if save.GOTIT {
        //
        // If we found the specified variable in the kernel pool, we
        // may be able to free up some space before inserting data.
        // We need to take this into account when determining
        // the amount of free room in the pool.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];

        if (save.DATAHD < 0) {
            //
            // No extra d.p.s will be freed.  We have whatever
            // free space is in the DPPOOL right now.
            //
        } else {
            //
            // Find out how many items are in the current
            // list of d.p. associated with the variable.
            //
            save.TOFREE = 0;
            save.NODE = save.DATAHD;

            while (save.NODE > 0) {
                save.TOFREE = (save.TOFREE + 1);
                save.NODE = save.DPPOOL[[NEXT, save.NODE]];
            }
            //
            // Add the number we will free to the amount currently
            // free in the dp pool.
            //
            save.AVAIL = (save.AVAIL + save.TOFREE);
        }
    }

    //
    // If the AVAIL for new data is less than the number of items
    // to be added, we just bail out here.
    //
    if (save.AVAIL < N) {
        if !save.GOTIT {
            //
            // We need to perform some clean up.  We've allocated
            // a new name but it has nothing in it. On the other hand
            // if we found it don't need to do anything because we've
            // only read from the pool. We haven't altered anything.
            // But in that case we'll never get into this block of code.
            //
            ZZCLN(
                save.LOOKAT,
                save.NAMEAT,
                save.NAMLST.as_slice_mut(),
                save.DATLST.as_slice_mut(),
                save.NMPOOL.as_slice_mut(),
                save.CHPOOL.as_slice_mut(),
                save.DPPOOL.as_slice_mut(),
                ctx,
            )?;
        }

        SETMSG(b"There is not sufficient space available in the kernel pool to store the # items associated with the name #.  There is room to store only # items. ", ctx);
        ERRINT(b"#", N, ctx);
        ERRCH(b"#", NAME, ctx);
        ERRINT(b"#", save.AVAIL, ctx);
        SIGERR(b"SPICE(NOMOREROOM)", ctx)?;
        CHKOUT(b"PIPOOL", ctx)?;
        return Ok(());
    }

    //
    // There is room to insert the data.  Free up any required
    // nodes.
    //
    if save.GOTIT {
        //
        // We need to free the data associated with this
        // variable.  But first make sure there will be room
        // to add data.
        //
        save.DATAHD = save.DATLST[save.NAMEAT];
        save.DATLST[save.NAMEAT] = 0;

        if (save.DATAHD < 0) {
            //
            // This variable was character type we need to
            // free a linked list from the character data
            // pool.
            //
            save.HEAD = -save.DATAHD;
            save.TAIL = -save.CHPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.CHPOOL.as_slice_mut(), ctx)?;
        } else {
            //
            // This variable was numeric type. We need to
            // free a linked list from the numeric pool.
            //
            save.HEAD = save.DATAHD;
            save.TAIL = -save.DPPOOL[[PREV, save.HEAD]];

            LNKFSL(save.HEAD, save.TAIL, save.DPPOOL.as_slice_mut(), ctx)?;
        }
    }
    //
    // We have done all of the freeing and checking that
    // needs to be done.  Now add the data.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // OK. See if there is room in
            // the numeric portion of the pool to store this value.
            //
            save.FREE = LNKNFN(save.DPPOOL.as_slice());

            if (save.FREE <= 0) {
                //
                // This branch of the code should never be exercised,
                // but it doesn't hurt to program in a redundant check.
                //
                ZZCLN(
                    save.LOOKAT,
                    save.NAMEAT,
                    save.NAMLST.as_slice_mut(),
                    save.DATLST.as_slice_mut(),
                    save.NMPOOL.as_slice_mut(),
                    save.CHPOOL.as_slice_mut(),
                    save.DPPOOL.as_slice_mut(),
                    ctx,
                )?;

                SETMSG(b"There is no room available for adding another numeric value to the kernel pool.", ctx);
                SIGERR(b"SPICE(KERNELPOOLFULL)", ctx)?;
                CHKOUT(b"PIPOOL", ctx)?;
                return Ok(());
            }

            //
            // Allocate a node for storing this numeric value:
            //
            LNKAN(save.DPPOOL.as_slice_mut(), &mut save.DPNODE, ctx)?;

            if (save.DATLST[save.NAMEAT] == 0) {
                //
                // There was no data for this name yet.  We make
                // DPNODE be the head of the data list for this name.
                //
                save.DATLST[save.NAMEAT] = save.DPNODE;
            } else {
                //
                // Put this node after the tail of the current list.
                //
                save.HEAD = save.DATLST[save.NAMEAT];
                save.TAIL = -save.DPPOOL[[PREV, save.HEAD]];

                LNKILA(save.TAIL, save.DPNODE, save.DPPOOL.as_slice_mut(), ctx)?;
            }

            //
            // Finally insert this data item into the numeric buffer.
            //
            save.DPVALS[save.DPNODE] = (IVALS[save.I] as f64);

            save.I += m3__;
        }
    }

    //
    // One last thing, see if this variable is being watched,
    // If it is, add its associated agents to the list of
    // AGENTS to be notified of a watched variable update.
    //
    if ELEMC(NAME, save.WTVARS.as_arg(), ctx)? {
        //
        // Union the update set AGENTS with the set of agents
        // associated with the variable NAME.
        //
        ZZNWPOOL(
            NAME,
            save.WTVARS.as_arg(),
            save.WTPTRS.as_slice(),
            save.WTPOOL.as_slice(),
            save.WTAGNT.as_arg(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"PIPOOL", ctx)?;
    Ok(())
}

/// Load variables from memory into the pool
///
/// Load the variables contained in an internal buffer into the
/// kernel pool.
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
///  CVALS      I   An array that contains a SPICE text kernel
///  N          I   The number of entries in CVALS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CVALS    is an array that contains lines of text that
///           could serve as a SPICE text kernel.
///
///  N        is the number of entries in CVALS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the kernel pool variables names or their values, as
///      provided in the input CVALS array, cannot be parsed, an error
///      is signaled by a routine in the call tree of this routine.
///
///  2)  If there is no room left in the kernel pool to store all
///      variables present in the input CVALS array, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If the length of any kernel pool variable name present in the
///      input CVALS array exceeds its maximum allowed length (see
///      Kernel Required Reading, kernel.req), an error is signaled by
///      a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to store a text kernel in an internal
///  array of your program and load this array into the kernel pool
///  without first storing its contents as a text kernel.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that your application is not particularly sensitive
///  to the current number of leapseconds but that you would
///  still like to use a relatively recent leapseconds kernel
///  without requiring users to load a leapseconds kernel into
///  the program. The example below shows how you might set up
///  the initialization portion of your program.
///
///     INTEGER               LNSIZE
///     PARAMETER           ( LNSIZE = 80 )
///
///     CHARACTER*(LNSIZE)    TEXT ( 27 )
///
///     TEXT(  1 ) = 'DELTET/DELTA_T_A =   32.184'
///     TEXT(  2 ) = 'DELTET/K         =    1.657D-3'
///     TEXT(  3 ) = 'DELTET/EB        =    1.671D-2'
///     TEXT(  4 ) = 'DELTET/M = (  6.239996D0   1.99096871D-7 )'
///     TEXT(  5 ) = 'DELTET/DELTA_AT  = ( 10,   @1972-JAN-1'
///     TEXT(  6 ) = '                     11,   @1972-JUL-1'
///     TEXT(  7 ) = '                     12,   @1973-JAN-1'
///     TEXT(  8 ) = '                     13,   @1974-JAN-1'
///     TEXT(  9 ) = '                     14,   @1975-JAN-1'
///     TEXT( 10 ) = '                     15,   @1976-JAN-1'
///     TEXT( 11 ) = '                     16,   @1977-JAN-1'
///     TEXT( 12 ) = '                     17,   @1978-JAN-1'
///     TEXT( 13 ) = '                     18,   @1979-JAN-1'
///     TEXT( 14 ) = '                     19,   @1980-JAN-1'
///     TEXT( 15 ) = '                     20,   @1981-JUL-1'
///     TEXT( 16 ) = '                     21,   @1982-JUL-1'
///     TEXT( 17 ) = '                     22,   @1983-JUL-1'
///     TEXT( 18 ) = '                     23,   @1985-JUL-1'
///     TEXT( 19 ) = '                     24,   @1988-JAN-1'
///     TEXT( 20 ) = '                     25,   @1990-JAN-1'
///     TEXT( 21 ) = '                     26,   @1991-JAN-1'
///     TEXT( 22 ) = '                     27,   @1992-JUL-1'
///     TEXT( 23 ) = '                     28,   @1993-JUL-1'
///     TEXT( 24 ) = '                     29,   @1994-JUL-1'
///     TEXT( 25 ) = '                     30,   @1996-JAN-1'
///     TEXT( 26 ) = '                     31,   @1997-JUL-1'
///     TEXT( 27 ) = '                     32,   @1999-JAN-1 )'
///
///     CALL LMPOOL ( TEXT, 27 )
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 8.3.1, 17-AUG-2021 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated $Exceptions section to better describe the error
///         handling of this routine.
///
/// -    SPICELIB Version 8.3.0, 30-JUL-2013 (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.2.0, 10-FEB-2010 (EDW)
///
///         Added mention of the restriction on kernel pool variable
///         names to MAXLEN characters or less.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn lmpool(ctx: &mut SpiceContext, cvals: CharArray, n: i32) -> crate::Result<()> {
    LMPOOL(cvals, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LMPOOL ( Load variables from memory into the pool )
pub fn LMPOOL(CVALS: CharArray, N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CVALS = DummyCharArray::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"LMPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Read from the internal SPICE pool buffer
    //
    save.LINNUM = 1;

    ZZRVBF(
        CVALS.as_arg(),
        N,
        &mut save.LINNUM,
        save.NAMLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.PNAMES.as_arg_mut(),
        save.DATLST.as_slice_mut(),
        save.DPPOOL.as_slice_mut(),
        save.DPVALS.as_slice_mut(),
        save.CHPOOL.as_slice_mut(),
        save.CHVALS.as_arg_mut(),
        &mut save.VARNAM,
        &mut save.EOF,
        ctx,
    )?;

    //
    // Read the variables in the file, one at a time.
    //
    while (!save.EOF && !FAILED(ctx)) {
        if fstr::ne(&save.VARNAM, b" ") {
            if ELEMC(&save.VARNAM, save.WTVARS.as_arg(), ctx)? {
                //
                // The variable VARNAM is watched.
                //
                // Union the update set AGENTS with the set of agents
                // associated with the variable VARNAM.
                //
                ZZNWPOOL(
                    &save.VARNAM,
                    save.WTVARS.as_arg(),
                    save.WTPTRS.as_slice(),
                    save.WTPOOL.as_slice(),
                    save.WTAGNT.as_arg(),
                    save.ACTIVE.as_arg_mut(),
                    save.NOTIFY.as_arg_mut(),
                    save.AGENTS.as_arg_mut(),
                    ctx,
                )?;
            }
        }
        //
        // We've processed VARNAM if it was non-blank.
        //
        ZZRVBF(
            CVALS.as_arg(),
            N,
            &mut save.LINNUM,
            save.NAMLST.as_slice_mut(),
            save.NMPOOL.as_slice_mut(),
            save.PNAMES.as_arg_mut(),
            save.DATLST.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.DPVALS.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.CHVALS.as_arg_mut(),
            &mut save.VARNAM,
            &mut save.EOF,
            ctx,
        )?;
    }

    //
    // That's it, the buffer supplied has been completely parsed
    // and placed into the kernel pool.
    //
    CHKOUT(b"LMPOOL", ctx)?;
    Ok(())
}

/// Get size limitations of the kernel pool
///
/// Return the kernel pool size limitations.
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
///  NAME       I   Name of the parameter to be returned.
///  N          O   Value of parameter specified by NAME.
///  FOUND      O   .TRUE. if NAME is recognized.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of a kernel pool size parameter.
///           The following parameters may be specified.
///
///              'MAXVAR'    is the maximum number of variables that
///                          the kernel pool may contain at any one
///                          time. MAXVAR should be a prime number.
///
///              'MAXLEN'    is the maximum length of the variable
///                          names that can be stored in the kernel
///                          pool.
///
///              'MAXVAL'    is the maximum number of distinct values
///                          that may belong to the variables in the
///                          kernel pool. Each variable must have at
///                          least one value, and may have any number,
///                          so long as the total number does not
///                          exceed MAXVAL. MAXVAL must be at least as
///                          large as MAXVAR.
///
///              'MXNOTE'    is the maximum number of distinct
///                          variable-agents pairs that can be
///                          maintained by the kernel pool. (A variable
///                          is "paired" with an agent, if that agent
///                          is to be notified whenever the variable is
///                          updated.)
///
///              'MAXAGT'    is the maximum number of agents that can
///                          be kept on the distribution list for
///                          notification of updates to kernel
///                          variables.
///
///              'MAXCHR'    is the maximum number of characters that
///                          can be stored in a component of a string
///                          valued kernel variable.
///
///              'MAXLIN'    is the maximum number of character strings
///                          that can be stored as data for kernel pool
///                          variables.
///
///           Note that the case of NAME is insignificant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the value of the parameter specified by NAME. If
///           NAME is not one of the items specified above, N will
///           be returned with the value 0.
///
///  FOUND    is .TRUE. if the parameter is recognized .FALSE. if it
///           is not.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified parameter is not recognized, the value of N
///      returned will be zero and FOUND will be set to .FALSE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the a programmatic interface to the
///  parameters used to define the kernel pool. It is not
///  anticipated that most kernel pool users will need to use this
///  routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated NAME
///         description to not refer to the main routine.
///
/// -    SPICELIB Version 1.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn szpool(
    ctx: &mut SpiceContext,
    name: &str,
    n: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SZPOOL(name.as_bytes(), n, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SZPOOL (Get size limitations of the kernel pool)
pub fn SZPOOL(
    NAME: &[u8],
    N: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SZPOOL", ctx)?;

    *FOUND = true;

    if EQSTR(NAME, b"MAXVAR") {
        *N = MAXVAR;
    } else if EQSTR(NAME, b"MAXVAL") {
        *N = MAXVAL;
    } else if EQSTR(NAME, b"MAXLIN") {
        *N = MAXLIN;
    } else if EQSTR(NAME, b"MAXCHR") {
        *N = MAXCHR;
    } else if EQSTR(NAME, b"MXNOTE") {
        *N = MXNOTE;
    } else if EQSTR(NAME, b"MAXLEN") {
        *N = MAXLEN;
    } else if EQSTR(NAME, b"MAXAGT") {
        *N = MAXAGT;
    } else {
        *N = 0;
        *FOUND = false;
    }

    CHKOUT(b"SZPOOL", ctx)?;
    Ok(())
}

/// Delete a variable from the kernel pool
///
/// Delete a variable from the kernel pool.
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
///  NAME       I   Name of the variable to be deleted.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the kernel pool variable to delete.
///           The name and associated values are removed from the
///           kernel pool, freeing the occupied space.
///
///           If a watches are set on the variable designated by
///           NAME, the corresponding agents are placed on the list
///           of agents to be notified of a kernel variable update.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified variable is not present in the kernel pool,
///      this routine simply returns. No error is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine enables users to selectively remove variables from
///  the kernel pool, as opposed to having to clear the pool and
///  reload it.
///
///  Note that it is not necessary to remove kernel variables in order
///  to simply update them; this routine should be used only when
///  variables are to be removed.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Remove triaxial radii of Jupiter from the kernel pool.
///
///         CALL DVPOOL ( 'BODY599_RADII' )
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
/// -    SPICELIB Version 8.3.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Moved example from $Restrictions to $Examples section.
///
/// -    SPICELIB Version 8.3.0, 30-JUL-2013 (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.2.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.1.0, 22-DEC-2004 (NJB)
///
///         Bug fix: corrected logic for determining when a
///         conflict resolution list is non-empty.
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (NJB) (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 8.3.0, 30-JUL-2013 (BVS)
///
///         Updated to increment POOL state counter.
///
/// -    SPICELIB Version 8.2.0, 19-MAR-2009 (NJB)
///
///         Watcher update code was re-written for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.1.0, 22-DEC-2004 (NJB)
///
///         Bug fix: corrected logic for determining when a
///         conflict resolution list is non-empty. The test
///
///            IF ( NAMEAT .LT. 0 ) THEN
///
///         formerly tested the variable NODE instead of NAMEAT.
///
///
///         Corrected an in-line comment relating to finding the
///         head node of the conflict resolution list for NAME.
/// ```
pub fn dvpool(ctx: &mut SpiceContext, name: &str) -> crate::Result<()> {
    DVPOOL(name.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DVPOOL ( Delete a variable from the kernel pool )
pub fn DVPOOL(NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DVPOOL", ctx)?;
    }

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // Locate the variable name in the hash table.  If the variable
    // is not present, just return.
    //
    //
    // Compute the hash value of this name.
    //
    save.LOOKAT = ZZHASH(NAME, ctx)?;

    //
    // Now see if there is a non-empty conflict resolution list for the
    // input string NAME.  If so, NAMLST(LOOKAT) contains the head node
    // of the conflict resolution list; this node is a positive value.
    //
    if (save.NAMLST[save.LOOKAT] == 0) {
        CHKOUT(b"DVPOOL", ctx)?;
        return Ok(());
    }
    //
    // If were are still here NAMLST(LOOKAT) is the first node of
    // a conflict resolution list.  See if the NAME corresponding
    // to this node is the one we are looking for.
    //
    save.NAMEAT = save.NAMLST[save.LOOKAT];
    save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NAMEAT));

    while !save.SUCCES {
        save.NAMEAT = save.NMPOOL[[NEXT, save.NAMEAT]];

        if (save.NAMEAT < 0) {
            CHKOUT(b"DVPOOL", ctx)?;
            return Ok(());
        }

        save.SUCCES = fstr::eq(NAME, save.PNAMES.get(save.NAMEAT));
    }

    //
    // Ok, the variable's here.  The head node of its value list is
    // DATLST(NAMEAT).  Delete the list pointing to the associated
    // values.  This list is in the numeric pool DPPOOL if the head
    // node is positive; otherwise the list is in the character pool
    // CHPOOL.
    //
    //
    ZZCLN(
        save.LOOKAT,
        save.NAMEAT,
        save.NAMLST.as_slice_mut(),
        save.DATLST.as_slice_mut(),
        save.NMPOOL.as_slice_mut(),
        save.CHPOOL.as_slice_mut(),
        save.DPPOOL.as_slice_mut(),
        ctx,
    )?;

    //
    // For consistency with CLPOOL, blank out the PNAMES entry containing
    // the name of this variable.  This is a bit of a flourish since
    // when errors occur during the population of the kernel pool, PNAMES
    // is not cleaned out
    //
    fstr::assign(save.PNAMES.get_mut(save.NAMEAT), b" ");

    //
    // There may be agents watching the variable we just wiped out.  If
    // so, add these agents to the list of agents to be notified of a
    // watched variable update.
    //
    if ELEMC(NAME, save.WTVARS.as_arg(), ctx)? {
        //
        // Union the update set AGENTS with the set of agents
        // associated with the variable NAME.
        //
        ZZNWPOOL(
            NAME,
            save.WTVARS.as_arg(),
            save.WTPTRS.as_slice(),
            save.WTPOOL.as_slice(),
            save.WTAGNT.as_arg(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"DVPOOL", ctx)?;
    Ok(())
}

/// Get names of kernel pool variables
///
/// Return names of kernel variables matching a specified template.
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
///  NAME       I   Template that names should match.
///  START      I   Index of first matching name to retrieve.
///  ROOM       I   The largest number of values to return.
///  N          O   Number of values returned for NAME.
///  CVALS      O   Kernel pool variables whose names match NAME.
///  FOUND      O   .TRUE. if there is at least one match.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is a MATCHI template which will be used when searching
///           for variable names in the kernel pool. The characters
///           '*' and '%' are used for the wild string and wild
///           characters respectively. For details of string
///           pattern matching see the header of the routine MATCHI.
///
///
///  START    is the index of the first variable name to return that
///           matches the NAME template. The matching names are
///           assigned indices ranging from 1 to NVAR, where NVAR is
///           the number of matching names. The index of a name does
///           not indicate how it compares alphabetically to another
///           name.
///
///           If START is less than 1, it will be treated as 1. If
///           START is greater than the total number of matching
///           variable names, no values will be returned and N will
///           be set to zero. However, FOUND will still be set to
///           .TRUE.
///
///
///  ROOM     is the maximum number of variable names that should
///           be returned for this template. If ROOM is less than 1
///           the error SPICE(BADARRAYSIZE) will be signaled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of variable names matching NAME that are
///           returned. It will always be less than or equal to
///           ROOM.
///
///           If no variable names match NAME, N is set to zero.
///
///
///  CVALS    is an array of kernel pool variables whose names match
///           the template NAME and which have indices ranging from
///           START to START+N-1.
///
///           Note that in general the names returned in CVALS are
///           not sorted.
///
///           If no variables match NAME, no values are assigned to
///           the elements of CVALS.
///
///           If the length of CVALS is less than the length of the
///           variable names, the values returned will be truncated
///           on the right. To ensure that names are not truncated,
///           CVALS should be declared to be at least
///           CHARACTER*(32).
///
///
///  FOUND    is .TRUE. if the some variable name in the kernel pool
///           matches NAME, .FALSE. if it is not.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXLEN   is the maximum length of the variable names that
///           can be stored in the kernel pool. This value is
///           currently 32.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ROOM is less than one, the error
///      SPICE(BADARRAYSIZE) is signaled.
///
///  2)  If CVALS has declared length less than the size of a variable
///      name to be returned, the name will be truncated on the right.
///      See MAXLEN for the maximum size of variable names.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides the user interface for retrieving the names
///  of kernel pool variables. This interface allows you to retrieve
///  the names matching a template via multiple accesses. Under some
///  circumstances this alleviates the problem of having to know in
///  advance the maximum amount of space needed to accommodate all
///  matching names.
///
///  However, this method of access does come with a price. It is
///  always more efficient to retrieve all of the data associated with
///  a kernel pool variable in one call than it is to retrieve it in
///  sections. The parameter MAXVAR defines the upper bound on the
///  number of possible matching names.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment demonstrates how the names of kernel
///  pool variables matching a template can be retrieved in pieces.
///
///  First we need some declarations.
///
///     INTEGER               ROOM
///     PARAMETER           ( ROOM = 3 )
///
///     CHARACTER*(3)         INDENT
///     CHARACTER*(80)        CVALS  (ROOM)
///     CHARACTER*(8)         VARNAM
///
///     INTEGER               START
///     INTEGER               N
///
///     LOGICAL               FOUND
///
///
///  Next load the data in the file 'typical.ker' into the
///  kernel pool.
///
///     CALL LDPOOL ( 'typical.ker' )
///
///  Next we shall print the names of kernel variables that match the
///  template 'BODY599*'.
///
///     VARNAM = 'BODY599*'
///     INDENT = ' '
///     START  =  1
///
///     CALL GNPOOL ( VARNAM, START, ROOM, N, CVALS, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///
///        WRITE (*,*) 'There are no matching variables ' //
///    .               'in the kernel pool.'
///     ELSE
///
///        WRITE (*,*) 'Kernel pool variables:'
///        WRITE (*,*)
///
///        DO I = 1, N
///           WRITE (*,*) INDENT, CVALS(I)
///        END DO
///
///        DO WHILE ( N .EQ. ROOM )
///
///           START = START + N
///           CALL GNPOOL ( VARNAM, START, ROOM, N, CVALS, FOUND )
///
///           DO I = 1, N
///              WRITE (*,*) INDENT, CVALS(I)
///           END DO
///
///        END DO
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 8.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added MAXLEN
///         description in $Parameters.
///
/// -    SPICELIB Version 8.1.0, 19-MAR-2009 (NJB)
///
///         ZZPINI call was updated for compatibility
///         with new watcher system implementation.
///
/// -    SPICELIB Version 8.0.0, 04-JUN-1999 (WLT)
///
///         Added the entry points PCPOOL, PDPOOL and PIPOOL to allow
///         direct insertion of data into the kernel pool without having
///         to read an external file.
///
///         Added the interface LMPOOL that allows SPICE
///         programs to load text kernels directly from memory
///         instead of requiring a text file.
///
///         Added the entry point SZPOOL to return kernel pool definition
///         parameters.
///
///         Added the entry point DVPOOL to allow the removal of a variable
///         from the kernel pool.
///
///         Added the entry point GNPOOL to allow users to determine
///         variables that are present in the kernel pool
/// ```
pub fn gnpool(
    ctx: &mut SpiceContext,
    name: &str,
    start: i32,
    room: i32,
    n: &mut i32,
    cvals: CharArrayMut,
    found: &mut bool,
) -> crate::Result<()> {
    GNPOOL(
        name.as_bytes(),
        start,
        room,
        n,
        cvals,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GNPOOL (Get names of kernel pool variables)
pub fn GNPOOL(
    NAME: &[u8],
    START: i32,
    ROOM: i32,
    N: &mut i32,
    CVALS: CharArrayMut,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CVALS = DummyCharArrayMut::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"GNPOOL", ctx)?;

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Perform the one obvious error check first.
    //
    if (ROOM < 1) {
        SETMSG(b"The amount of room specified as available for output in the output array was: #.  The amount of room must be positive. ", ctx);

        ERRINT(b"#", ROOM, ctx);
        SIGERR(b"SPICE(BADARRAYSIZE)", ctx)?;
        CHKOUT(b"GNPOOL", ctx)?;
        return Ok(());
    }

    //
    // So far we've encountered no matching names.
    //
    save.HITS = 0;
    *N = 0;
    save.BEGIN = intrinsics::MAX0(&[1, START]);

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXVAR;
        let m3__: i32 = 1;
        save.K = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // See if there is any variable associated with this hash value.
            //
            save.NNODE = save.NAMLST[save.K];

            while (save.NNODE > 0) {
                //
                // There is some name list associated with this node. See if
                // it the current one matches the supplied template.
                //
                if MATCHI(&save.PNAMES[save.NNODE], NAME, b"*", b"%", ctx) {
                    //
                    // We've got a match.  Record this fact and if we have
                    // reached (or passed) the starting point, put this name
                    // on the output list.
                    //
                    save.HITS = (save.HITS + 1);

                    if (save.HITS >= START) {
                        if (*N < ROOM) {
                            *N = (*N + 1);
                            fstr::assign(CVALS.get_mut(*N), save.PNAMES.get(save.NNODE));
                        }
                        //
                        // If we've filled up the buffer, we may as well
                        // quit now.
                        //
                        if (*N == ROOM) {
                            *FOUND = true;
                            CHKOUT(b"GNPOOL", ctx)?;
                            return Ok(());
                        }
                    }
                }

                //
                // Get the next name for this node.
                //
                save.NNODE = save.NMPOOL[[NEXT, save.NNODE]];
            }
            //
            // Advance to the next hash value.
            //
            save.K += m3__;
        }
    }

    *FOUND = (save.HITS > 0);

    CHKOUT(b"GNPOOL", ctx)?;
    Ok(())
}

/// Delete watch from kernel pool
///
/// Delete a name from the list of agents to notify whenever a member
/// of a list of kernel variables is updated.
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
///  AGENT      I   The name of an agent to be notified after updates.
/// ```
///
/// # Detailed Input
///
/// ```text
///  AGENT    is any agent name that has previously been associated
///           with a kernel pool watch via a call to SWPOOL. The
///           agent name will be deleted from the notification list
///           of every watched kernel variable.
///
///           Watched variables whose notification lists become
///           empty will be deleted.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  It's not an error to delete a non-existent agent---one
///      that is not present in the watcher system. A call to
///      delete a non-existent agent has no effect on the state
///      of the watcher system.
///
///  2)  If an attempt is made to delete an agent that
///      has an unchecked update, the error SPICE(UPDATEPENDING)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Kernel pool watches are a limited resource; the ability
///  to delete watches when they're no longer needed is essential
///  to allow programs that make heavy use of kernel pool watches
///  to run for extended periods.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have an application subroutine, MYTASK, that
///  needs to access a large data set in the kernel pool. If this
///  data could be kept in local storage and kernel pool queries
///  performed only when the data in the kernel pool has been
///  updated, the routine can perform much more efficiently.
///
///  If at some point the local stored data no longer need to be
///  watched---for example, if they're removed from the local
///  buffer to make room for other data---the watch set by the
///  agent 'MYTASK' on those data can be deleted via the call
///
///     CALL DWPOOL ( 'MYTASK' )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  It is recommended that watches be deleted only by
///      routines that established them.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 11-SEP-2013 (BVS)
///
///         Updated to increment POOL state counter. Updated description
///         of the exception 1).
///
/// -    SPICELIB Version 1.0.0, 19-MAR-2009 (NJB)
/// ```
pub fn dwpool(ctx: &mut SpiceContext, agent: &str) -> crate::Result<()> {
    DWPOOL(agent.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DWPOOL ( Delete watch from kernel pool )
pub fn DWPOOL(AGENT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DWPOOL", ctx)?;

    //
    // Initialize the pool if necessary.
    //
    if save.FIRST {
        ZZPINI(
            &mut save.FIRST,
            MAXVAR,
            MAXVAL,
            MAXLIN,
            &mut save.BEGDAT,
            &mut save.BEGTXT,
            save.NMPOOL.as_slice_mut(),
            save.DPPOOL.as_slice_mut(),
            save.CHPOOL.as_slice_mut(),
            save.NAMLST.as_slice_mut(),
            save.DATLST.as_slice_mut(),
            MAXAGT,
            MXNOTE,
            save.WTVARS.as_arg_mut(),
            save.WTPTRS.as_slice_mut(),
            save.WTPOOL.as_slice_mut(),
            save.WTAGNT.as_arg_mut(),
            save.AGENTS.as_arg_mut(),
            save.ACTIVE.as_arg_mut(),
            save.NOTIFY.as_arg_mut(),
            save.SUBCTR.as_slice_mut(),
            ctx,
        )?;
    }

    //
    // Make sure we're not silencing an agent who has something
    // to say.
    //
    if ELEMC(AGENT, save.AGENTS.as_arg(), ctx)? {
        SETMSG(b"Could not delete AGENT # from the watch symbol table because AGENT is associated with at least one updated kernel variable. ", ctx);
        ERRCH(b"#", AGENT, ctx);
        SIGERR(b"SPICE(UPDATEPENDING)", ctx)?;
        CHKOUT(b"DWPOOL", ctx)?;
        return Ok(());
    }

    //
    // Increment POOL state counter.
    //
    ZZCTRINC(save.SUBCTR.as_slice_mut(), ctx)?;

    //
    // AGENT is no longer on the list of agents associated with a
    // kernel variable update.
    //
    REMOVC(AGENT, save.AGENTS.as_arg_mut(), ctx)?;

    //
    // For each kernel variable in the watcher's list, remove
    // AGENT from its list of guys to be notified when a variable change
    // occurs. If AGENT is the only value associated with the variable,
    // delete the kernel variable's entry from the table.
    //
    // This outer loop is relatively tricky, since
    //
    //    1) The upper loop bound can change during loop execution.
    //
    //    2) The loop index I doesn't necessary increase on every
    //       loop pass.
    //
    // Infinite loops can lurk in code with the above attributes. We
    // need to know that the loop will always terminate. Presume that
    // no SPICE error occurs during the loop: then we observe
    // that on each loop pass, either I increases or the loop bound
    // CARDC(WTVARS) decreases, so the difference
    //
    //    CARDC(WTVARS) - I
    //
    // does in fact decrease on every loop iteration. When this
    // difference becomes -1, the loop will end.
    //
    // If a SPICE error occurs during the loop, the FAILED test
    // will terminate the loop.
    //
    // Since WTVARS may shrink due to deletion of watches, we
    // fetch the cardinality of WTVARS on each loop iteration.
    //
    save.I = 1;

    while ((save.I <= CARDC(save.WTVARS.as_arg(), ctx)?) && !FAILED(ctx)) {
        //
        // Search the list of agents associated with the Ith watched
        // variable for AGENT. We want the list count as well, so
        // we'll traverse the whole list (which likely is short).
        //
        // We don't use ZZGAPOOL here because we need to get the
        // watcher pool nodes associated with AGENT.
        //
        // If we find AGENT, we'll use AGNODE to designate
        // the node associated with AGENT.
        //
        save.NODE = save.WTPTRS[save.I];
        save.NNODES = 0;
        save.AGNODE = 0;

        while (save.NODE > 0) {
            save.NNODES = (save.NNODES + 1);
            //
            // Fetch the next agent for the Ith kernel variable.
            //
            if fstr::eq(save.WTAGNT.get(save.NODE), AGENT) {
                //
                // Save the current node.
                //
                save.AGNODE = save.NODE;
            }
            //
            // Find the next node in the list.
            //
            save.NODE = LNKNXT(save.NODE, save.WTPOOL.as_slice(), ctx)?;
        }

        if (save.AGNODE > 0) {
            //
            // The input agent is on the agent list for the Ith watched
            // kernel variable. Delete this agent from the list. Delete
            // the node corresponding to AGENT from the watch pool. First
            // set the corresponding agent name to blank.
            //
            fstr::assign(save.WTAGNT.get_mut(save.AGNODE), b" ");

            //
            // If we're about to delete the head node of the agent list,
            // we'll need to update WTPTRS(I) to point to the new head.
            // It's possible that this agent list is empty after deletion
            // of AGNODE; we'll handle that case after the LNKFSL call
            // below.
            //
            if (save.WTPTRS[save.I] == save.AGNODE) {
                save.WTPTRS[save.I] = LNKNXT(save.AGNODE, save.WTPOOL.as_slice(), ctx)?;
            }

            //
            // Now free AGNODE.
            //
            LNKFSL(save.AGNODE, save.AGNODE, save.WTPOOL.as_slice_mut(), ctx)?;

            if (save.NNODES == 1) {
                //
                // In fact AGENT is the *only* agent for the Ith variable.
                // Deleting AGENT means that nobody's watching this
                // variable any more, so delete the variable from the
                // watched variable set.

                save.NW = CARDC(save.WTVARS.as_arg(), ctx)?;

                fstr::assign(&mut save.VARNAM, save.WTVARS.get(save.I));

                REMOVC(&save.VARNAM, save.WTVARS.as_arg_mut(), ctx)?;

                //
                // Remove the associated pointer from the pointer array.
                //
                REMLAI(1, save.I, save.WTPTRS.as_slice_mut(), &mut save.NW, ctx)?;

                //
                // Since we deleted the current variable table entry and
                // compressed the set WTVARS and the array WTPTRS, I now
                // points to the next variable in the table. Decrement I
                // here to compensate for the increment operation at the
                // bottom of the loop.
                //
                save.I = (save.I - 1);
            }
            //
            // We've now deleted AGENT from the AGENT list for WTVARS(I).
            // If the deletion left no agents watching WTVARS(I), we
            // deleted WTVARS(I) and its associated pointer WTPTRS(I).
            //
        }
        //
        // We've processed the Ith kernel variable in the watcher table.
        //
        // If we deleted the Ith WTVARS entry, we decremented I
        // at that time, so the increment operation here always is
        // applicable.
        //
        save.I = (save.I + 1);

        //
        // At this point in the loop, either I has increased or
        // CARDC(WTVARS) has decreased; hence we've made progress
        // toward loop termination.
        //
    }

    CHKOUT(b"DWPOOL", ctx)?;
    Ok(())
}

/// Private: view kernel pool watch system
///
/// SPICE Private routine intended solely for the support of SPICE
/// routines. Users should not call this routine directly due to the
/// volatile nature of this routine.
///
/// Return copies of certain POOL variables that are used in the
/// implementation of the watcher mechanism. These variables
/// comprise a data structure that maps kernel variable names to sets
/// of agents watching those kernel variables.
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
///  UWVARS     O   Watched kernel variable set.
///  UWPTRS     O   Pointers from variables into the watch pool.
///  UWPOOL     O   Watch pool used for managing agent names.
///  UWAGNT     O   Array of agent names.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UWVARS   is a set into which the local watcher system
///           set WTVARS has been copied.
///
///  UWPTRS   is an array into which the local watcher system
///           array WTPTRS has been copied.
///
///  UWPOOL   is a doubly linked list pool into which the local
///           watcher system doubly linked list pool WTPOOL has
///           been copied.
///
///  UWAGNT   is an array into which the local watcher system
///           array WTAGNT has been copied.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the output array UWVARS is too small to hold the
///      set WTVARS, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If any output array other than UWVARS is to small
///      to hold the corresponding watch system component,
///      memory corruption will occur.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is not part of the SPICELIB API. This routine
///  may be removed in a later version of the SPICE Toolkit, or
///  its interface may change.
///
///  SPICE-based application code should not call this routine.
///
///  This is an "inspection hatch" routine used for SPICELIB
///  testing.
/// ```
///
/// # Examples
///
/// ```text
///  See the TSPICE test family F_DWPOOL.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This is a private routine. See $Particulars above.
///
///  2)  The caller must provide output arrays of adequate
///      size. See the declarations of the watch system
///      components in the umbrella routine POOL for size
///      requirements.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 27-MAR-2014 (BVS)
///
///         Set $Index_Entries to "None." to make this entry no appear in
///         permuted index.
///
/// -    SPICELIB Version 1.0.0, 19-MAR-2009 (NJB)
/// ```
pub fn zzvupool(
    ctx: &mut SpiceContext,
    uwvars: CharArrayMut,
    uwptrs: &mut [i32],
    uwpool: &mut [[i32; 2]],
    uwagnt: CharArrayMut,
) -> crate::Result<()> {
    ZZVUPOOL(
        uwvars,
        uwptrs,
        uwpool.as_flattened_mut(),
        uwagnt,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ZZVUPOOL ( Private: view kernel pool watch system )
pub fn ZZVUPOOL(
    UWVARS: CharArrayMut,
    UWPTRS: &mut [i32],
    UWPOOL: &mut [i32],
    UWAGNT: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut UWVARS = DummyCharArrayMut::new(UWVARS, None, LBCELL..);
    let mut UWPTRS = DummyArrayMut::new(UWPTRS, 1..);
    let mut UWPOOL = DummyArrayMut2D::new(UWPOOL, 1..=2, LBCELL..);
    let mut UWAGNT = DummyCharArrayMut::new(UWAGNT, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZVUPOOL", ctx)?;

    COPYC(save.WTVARS.as_arg(), UWVARS.as_arg_mut(), ctx)?;
    MOVEI(
        save.WTPTRS.as_slice(),
        CARDC(save.WTVARS.as_arg(), ctx)?,
        UWPTRS.as_slice_mut(),
    );

    //
    // UWPOOL is expected to have dimensions
    //
    //    ( 2,  LBPOOL : MXNOTE )
    //

    save.I = (2 * (6 + MXNOTE));

    MOVEI(save.WTPOOL.as_slice(), save.I, UWPOOL.as_slice_mut());

    MOVEC(save.WTAGNT.as_arg(), MXNOTE, UWAGNT.as_arg_mut());

    CHKOUT(b"ZZVUPOOL", ctx)?;
    Ok(())
}

/// Private: check/update user's POOL state counter
///
/// SPICE Private routine intended solely for the support of SPICE
/// routines. Users should not call this routine directly due to the
/// volatile nature of this routine.
///
/// Check and update the POOL state counter tracked by a caller
/// (user) routine.
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
///  USRCTR    I-O  POOL state counter tracked by the caller
///  UPDATE     O   Flag indicating if input counter was updated
///
///  CTRSIZ     P   Dimension of the counter array
/// ```
///
/// # Detailed Input
///
/// ```text
///  USRCTR   is the value of the POOL state counter tracked by
///           (saved in) the caller (user) routine.
/// ```
///
/// # Detailed Output
///
/// ```text
///  USRCTR   is the current POOL state counter.
///
///  UPDATE   is the logical flag indicating whether the input POOL
///           state counter was different from the current POOL
///           state counter and, therefore, had to be updated
///           (UPDATE = .TRUE.) or if it was the same (UPDATE =
///           .FALSE.).
/// ```
///
/// # Parameters
///
/// ```text
///  CTRSIZ   is the dimension of the counter array used by
///           various SPICE subsystems to uniquely identify
///           changes in their states. This parameter is
///           defined in the private include file 'zzctr.inc'.
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
///  This routine is not part of the SPICELIB API. This routine
///  may be removed in a later version of the SPICE Toolkit, or
///  its interface may change.
///
///  SPICE-based application code should not call this routine.
///
///  This routine allows other routines to be aware of POOL state
///  change due to addition or deletion variables or watchers. Such
///  awareness is needed to be able to locally save some POOL-based
///  data (e.g. body name/ID mappings or frame definitions) and only
///  update these locally saved values if the POOL has changed.
///
///  To make use of the POOL state counter to achieve this goal the
///  caller routines save the POOL state counter returned by the first
///  call to this routine and then check that saved value against the
///  current POOL state counter and update it by subsequent calls this
///  routine.
/// ```
///
/// # Examples
///
/// ```text
///  The routines that need to be aware of and act on the POOL state
///  change initialize a local POOL counter array using ZZCTRUIN, save
///  it, and check it against the current POOL state counter and
///  update it, if needed, using this entry point, as follows:
///
///     C
///     C     Include zzctr.inc to access CTRSIZ.
///     C
///           INCLUDE              'zzctr.inc'
///           ...
///
///     C
///     C     In local variable declarations declare and save
///     C     the local POOL state counter. Also declare the
///     C     update flag.
///     C
///           INTEGER               USRCTR ( CTRSIZ )
///           LOGICAL               UPDATE
///           ...
///           SAVE                  USRCTR
///           ...
///
///     C
///     C     In all places where initialization is done
///     C     initialize the local POOL state counter using
///     C     ZZCTRUIN to ensure an update on the first check.
///     C
///           IF ( FIRST ) THEN
///              ...
///              CALL ZZCTRUIN( USRCTR )
///              FIRST = .FALSE.
///           END IF
///           ...
///
///     C
///     C     In all places where there is a need to check for
///     C     the POOL state change call this entry to
///     C     check and update the local POOL state counter.
///     C
///           CALL ZZPCTRCK ( USRCTR, UPDATE )
///
///           IF ( UPDATE ) THEN
///
///     C
///     C        It the POOL state changed, do what needs to
///     C        be done to deal with saved values based
///     C        on POOL data.
///     C
///              ...
///
///           END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This is a private routine. See $Particulars above.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 27-MAR-2014 (BVS)
/// ```
pub fn zzpctrck(ctx: &mut SpiceContext, usrctr: &mut [i32; 2], update: &mut bool) {
    ZZPCTRCK(usrctr, update, ctx.raw_context());
}

//$Procedure ZZPCTRCK ( Private: check/update user's POOL state counter )
pub fn ZZPCTRCK(USRCTR: &mut [i32], UPDATE: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..=CTRSIZ);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return;
    }

    ZZCTRCHK(save.SUBCTR.as_slice(), USRCTR.as_slice_mut(), UPDATE, ctx);
}
