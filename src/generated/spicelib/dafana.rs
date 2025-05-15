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
const TBSIZE: i32 = 20;
const ARYCNT: i32 = 3;
const BWDPTR: i32 = 2;
const CRLEN: i32 = 1000;
const DPRSIZ: i32 = 128;
const FWDPTR: i32 = 1;
const IFNLEN: i32 = 60;
const LBCELL: i32 = -5;
const MAXNDC: i32 = 124;
const MAXNIC: i32 = 250;
const MAXSUM: i32 = 125;
const NIL: i32 = -1;

struct SaveVars {
    STFH: StackArray<i32, 20>,
    STIFNM: ActualCharArray,
    STADDG: StackArray<bool, 20>,
    STFRST: StackArray<i32, 20>,
    STLAST: StackArray<i32, 20>,
    STFREE: StackArray<i32, 20>,
    STBEGN: StackArray<i32, 20>,
    STLSUM: ActualArray2D<f64>,
    STNAME: ActualCharArray,
    STPOOL: StackArray<i32, 20>,
    STHEAD: i32,
    STFPTR: i32,
    NAMREC: Vec<u8>,
    DAFNAM: Vec<u8>,
    IFNAME: Vec<u8>,
    DC: StackArray<f64, 124>,
    SUMREC: StackArray<f64, 128>,
    BWARD: i32,
    CLOC: i32,
    DLOC: i32,
    FREE: i32,
    FWARD: i32,
    IC: StackArray<i32, 250>,
    NAMSIZ: i32,
    ND: i32,
    NI: i32,
    NARRAY: i32,
    NEXT: i32,
    NEXTP: i32,
    OPNSET: ActualArray<i32>,
    P: i32,
    PREV: i32,
    SUMSIZ: i32,
    WORD: i32,
    FIRST: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STFH = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STIFNM = ActualCharArray::new(IFNLEN, 1..=TBSIZE);
        let mut STADDG = StackArray::<bool, 20>::new(1..=TBSIZE);
        let mut STFRST = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STLAST = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STFREE = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STBEGN = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STLSUM = ActualArray2D::<f64>::new(1..=MAXSUM, 1..=TBSIZE);
        let mut STNAME = ActualCharArray::new(CRLEN, 1..=TBSIZE);
        let mut STPOOL = StackArray::<i32, 20>::new(1..=TBSIZE);
        let mut STHEAD: i32 = 0;
        let mut STFPTR: i32 = 0;
        let mut NAMREC = vec![b' '; CRLEN as usize];
        let mut DAFNAM = vec![b' '; FILEN as usize];
        let mut IFNAME = vec![b' '; IFNLEN as usize];
        let mut DC = StackArray::<f64, 124>::new(1..=MAXNDC);
        let mut SUMREC = StackArray::<f64, 128>::new(1..=DPRSIZ);
        let mut BWARD: i32 = 0;
        let mut CLOC: i32 = 0;
        let mut DLOC: i32 = 0;
        let mut FREE: i32 = 0;
        let mut FWARD: i32 = 0;
        let mut IC = StackArray::<i32, 250>::new(1..=MAXNIC);
        let mut NAMSIZ: i32 = 0;
        let mut ND: i32 = 0;
        let mut NI: i32 = 0;
        let mut NARRAY: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut NEXTP: i32 = 0;
        let mut OPNSET = ActualArray::<i32>::new(LBCELL..=FTSIZE);
        let mut P: i32 = 0;
        let mut PREV: i32 = 0;
        let mut SUMSIZ: i32 = 0;
        let mut WORD: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;

        FIRST = true;
        STHEAD = NIL;
        STFPTR = NIL;

        Self {
            STFH,
            STIFNM,
            STADDG,
            STFRST,
            STLAST,
            STFREE,
            STBEGN,
            STLSUM,
            STNAME,
            STPOOL,
            STHEAD,
            STFPTR,
            NAMREC,
            DAFNAM,
            IFNAME,
            DC,
            SUMREC,
            BWARD,
            CLOC,
            DLOC,
            FREE,
            FWARD,
            IC,
            NAMSIZ,
            ND,
            NI,
            NARRAY,
            NEXT,
            NEXTP,
            OPNSET,
            P,
            PREV,
            SUMSIZ,
            WORD,
            FIRST,
            FOUND,
        }
    }
}

/// DAF, add new array
///
/// Add a new array to an existing DAF.
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
///  HANDLE     I   DAFBNA, DAFCAD
///  SUM        I   DAFBNA
///  NAME       I   DAFBNA
///  DATA       I   DAFADA
///  N          I   DAFADA
///  TBSIZE     P   DAFANA
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF opened for write access
///           by a previous call to DAFOPW or DAFOPN.
///
///  SUM      is the summary for the array being added.
///
///  NAME     is the name of the array being added.
///
///  DATA     contains all or part of the data in the array.
///
///  N        is the number of elements in DATA.
/// ```
///
/// # Parameters
///
/// ```text
///  TBSIZE   is the size of the file table maintained internally
///           by DAFANA,  TBSIZE is the maximum number of DAFs
///           that can be in use simultaneously by this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DAFANA is called directly, the error SPICE(BOGUSENTRY)
///      is signaled.
///
///  2)  See entry points DAFBNA, DAFADA, DAFENA, and DAFCAD
///      for exceptions specific to those entry points.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE, above.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFANA serves as an umbrella, allowing data to be shared by its
///  entry points:
///
///     DAFBNA         Begin new array.
///     DAFADA         Add data to array.
///     DAFCAD         Continue adding data.
///     DAFENA         End new array.
///
///  The main function of these entry points is to simplify the
///  addition of new arrays to existing DAFs.
///
///  An application can add data to a single DAF, or to multiple DAFs,
///  simultaneously. In the case of writing to a single DAF, the
///  creation of a new array requires four steps:
///
///     1) Open a DAF for write access, using either DAFOPW
///        (if the file already exists) or DAFOPN (if it does not).
///
///           CALL DAFOPW ( FNAME, HANDLE )
///
///     2) Begin the new DAF by calling DAFBNA,
///
///           CALL DAFBNA ( HANDLE, SUM, NAME )
///
///     3) Add data to the array by calling DAFADA as many times
///        as necessary,
///
///           CALL GET_DATA ( DATA, N, FOUND )
///
///           DO WHILE ( FOUND )
///              CALL DAFADA   ( DATA, N        )
///              CALL GET_DATA ( DATA, N, FOUND )
///           END DO
///
///     4) End the array by calling DAFENA,
///
///           CALL DAFENA
///
///  Note that the data can be added in chunks of any size, so long
///  as the chunks are ordered correctly.
///
///  In applications that add data to multiple DAFs simultaneously, it
///  is necessary to specify which DAF to add data to. The DAFANA
///  entry points that allow specification of a DAF via a file handle
///  argument are DAFBNA (DAF, begin new array) and DAFCAD (DAF,
///  continue adding data).  As in the single-DAF case, arrays are
///  started by calls to DAFBNA, and data is added to arrays by calls
///  to DAFADA. The last DAF designated by the input file handle
///  supplied to DAFBNA or DAFCAD is the `current DAF'. If a
///  DAF contains an array started by a call to DAFBNA but not yet
///  completed by a call to DAFENA, we call this array the `current
///  array' for that DAF. Each call to DAFADA will add data to the
///  current array in the current DAF. A call to DAFENA will make the
///  current array in the current DAF a permanent addition to that DAF.
///
///  The notion of `current DAF' as discussed here applies only to
///  DAFs acted upon by entry points of DAFANA. In DAFFA, there is a
///  DAF that is treated as the `current DAF' for searching; there is
///  no connection between the DAFs regarded as current by DAFANA and
///  DAFFA.
///
///  In the following example, we write data obtained from the routine
///  GET_DATA into two separate DAFs. The first N/2 elements of the
///  array DATA will be written to the first DAF; the rest of the
///  array will be written to the second DAF.
///
///
///     1) Open the DAFs for write access, using either DAFOPW
///        (if the files already exist) or DAFOPN (if they do not).
///
///           CALL DAFOPW ( FNAME1, HANDL1 )
///           CALL DAFOPW ( FNAME2, HANDL2 )
///
///     2) Begin the new DAFs by calling DAFBNA,
///
///           CALL DAFBNA ( HANDL1, SUM1, NAME1 )
///           CALL DAFBNA ( HANDL2, SUM2, NAME2 )
///
///     3) Add data to the arrays by calling DAFCAD and DAFADA as many
///        times as necessary, selecting the file to add data to by
///        calling DAFCAD:
///
///           CALL GET_DATA ( DATA, N, FOUND )
///
///           DO WHILE ( FOUND )
///
///              CALL DAFCAD   ( HANDL1                          )
///              CALL DAFADA   ( DATA,               N/2         )
///
///              CALL DAFCAD   ( HANDL2                          )
///              CALL DAFADA   ( DATA( N/2 + 1 ),    N - N/2     )
///
///              CALL GET_DATA ( DATA, N, FOUND )
///
///           END DO
///
///     4) End each array by calling DAFENA, selecting the file
///        in which to end the array by calling DAFCAD:
///
///           CALL DAFCAD ( HANDL1 )
///           CALL DAFENA
///
///           CALL DAFCAD ( HANDL2 )
///           CALL DAFENA
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
///  1) The following example illustrates one possible way to copy
///     an array from one DAF to another, N words at a time.
///
///     Use the CK kernel below as the original DAF file.
///
///        vo2_swu_ck2.bc
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFANA_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           INTEGER               MAXNSZ
///           PARAMETER           ( MAXNSZ = 1000 )
///
///           INTEGER               MAXND
///           PARAMETER           ( MAXND  = 124  )
///
///           INTEGER               MAXNI
///           PARAMETER           ( MAXNI  = 250  )
///
///           INTEGER               NWORDS
///           PARAMETER           ( NWORDS = 100  )
///
///           INTEGER               MAXSUM
///           PARAMETER           ( MAXSUM = 125  )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(MAXNSZ)    NAME
///
///           DOUBLE PRECISION      DATA   ( NWORDS )
///           DOUBLE PRECISION      DC     ( MAXND  )
///           DOUBLE PRECISION      SUM    ( MAXSUM )
///
///           INTEGER               BIDX
///           INTEGER               CHUNK
///           INTEGER               EIDX
///           INTEGER               IC     ( MAXNI  )
///           INTEGER               ND
///           INTEGER               NI
///           INTEGER               ORIGIN
///           INTEGER               TARGET
///
///           LOGICAL               FOUND
///
///     C
///     C     Open the origin DAF file for reading.
///     C
///           CALL DAFOPR ( 'vo2_swu_ck2.bc', ORIGIN )
///
///     C
///     C     Start forward search in origin DAF.
///     C
///           CALL DAFBFS ( ORIGIN )
///
///     C
///     C     Find the first array in origin DAF.
///     C
///           CALL DAFFNA ( FOUND  )
///
///     C
///     C     Get the summary and name of the current array in the
///     C     ORIGIN DAF file
///     C
///           CALL DAFGS  ( SUM  )
///           CALL DAFGN  ( NAME )
///
///     C
///     C     Unpack the summary.
///     C
///           CALL DAFHSF ( ORIGIN, ND, NI )
///           CALL DAFUS  ( SUM,    ND, NI, DC, IC )
///
///     C
///     C     Open the target DAF file for writing. Use 'CK' as
///     C     data type, and reserve no records for comments.
///     C
///           CALL DAFONW ( 'dafana_ex1.bc', 'CK', ND, NI,
///          .              'CK file created for example 1 DAFANA', 0,
///          .              TARGET )
///
///     C
///     C     Begin a new array in the target DAF file, using the
///     C     origin SUM and NAME.
///     C
///           CALL DAFBNA ( TARGET, SUM, NAME )
///
///     C
///     C     Copy the complete array for the first segment of the
///     C     origin DAF file.
///     C
///           BIDX = IC(NI-1)
///           EIDX = IC(NI  )
///
///           DO WHILE ( BIDX .LE. EIDX )
///
///              CHUNK = MIN ( BIDX + NWORDS - 1, EIDX )
///
///              CALL DAFGDA ( ORIGIN, BIDX, CHUNK, DATA )
///              CALL DAFADA ( DATA,   NWORDS )
///
///              BIDX = BIDX + NWORDS
///
///           END DO
///
///     C
///     C     End the new array in the target DAF.
///     C
///           CALL DAFENA
///
///     C
///     C     Close the DAF files.
///     C
///           CALL DAFCLS ( ORIGIN )
///           CALL DAFCLS ( TARGET )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new CK file exists in the
///     output directory.
///
///
///  2)  A simple example demonstrating simultaneous addition
///      of data to multiple DAFs.
///
///      Assume we have data from a text file containing three
///      columns of numbers. We will write the data from each
///      column out to a separate DAF.
///
///      To confirm that the DAFs created by this program contain the
///      correct contents, we will read the data from each DAF and
///      combine it to create a matrix. This matrix should contain
///      the same data as the file we assumed to be the source for
///      our dataset.
///
///      The format of the output text should be as follows:
///
///         .-                   -.
///         |  n11    n12    n13  |
///         |  n21    n22    n23  |
///         |   .      .      .   |
///         |   .      .      .   |
///         |   .      .      .   |
///         `-                   -'
///
///      where the symbol nij indicates the jth number on the ith line
///      of the source data file.
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFANA_EX2
///           IMPLICIT NONE
///
///     C
///     C     Assume we have read columns of d.p. numbers
///     C     from a text file. Write the data from each
///     C     column into a separate DAF.  Read these DAFs
///     C     and create a matrix containing the same data
///     C     as assumed input text file.
///     C
///     C     Since we do not need to retain any descriptive
///     C     information about the DAFs inside of the files
///     C     themselves, we'll use a summary format having
///     C     two integer components (the minimum--these are
///     C     reserved for use by the DAF routines) and zero
///     C     double precision components.
///     C
///     C     The internal file names and array names will
///     C     simply indicate the data sources.
///     C
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FNMLEN
///           PARAMETER           ( FNMLEN = 20 )
///
///           INTEGER               LINLEN
///           PARAMETER           ( LINLEN = 80 )
///
///           INTEGER               MAXLNS
///           PARAMETER           ( MAXLNS =  9 )
///
///           INTEGER               MAXCOL
///           PARAMETER           ( MAXCOL =  3 )
///
///           INTEGER               ND
///           PARAMETER           ( ND     =  0 )
///
///           INTEGER               NDAF
///           PARAMETER           ( NDAF   =  3 )
///
///           INTEGER               NI
///           PARAMETER           ( NI     =  2 )
///
///           INTEGER               NUMLEN
///           PARAMETER           ( NUMLEN = 30 )
///
///           INTEGER               SIG
///           PARAMETER           ( SIG    = 10 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FNMLEN)    DAFNAM ( NDAF   )
///           CHARACTER*(FNMLEN)    INFILE
///           CHARACTER*(LINLEN)    LINE
///           CHARACTER*(NUMLEN)    NUMCH
///           CHARACTER*(LINLEN)    PRSERR
///           CHARACTER*(FNMLEN)    RESULT
///
///           DOUBLE PRECISION      DC     ( 1      )
///           DOUBLE PRECISION      NUMBER ( MAXLNS, MAXCOL )
///           DOUBLE PRECISION      NUMDP
///           DOUBLE PRECISION      SUMMRY ( 1      )
///
///           INTEGER               FA
///           INTEGER               HAN    ( NDAF   )
///           INTEGER               I
///           INTEGER               IA
///           INTEGER               IC     ( NI     )
///           INTEGER               J
///           INTEGER               LENGTH
///           INTEGER               NCOLS
///           INTEGER               PTR
///
///           LOGICAL               EOF
///           LOGICAL               FOUND
///
///     C
///     C     Initial values
///     C
///           DATA                  DAFNAM   /  'COLUMN1.DAF',
///          .                                  'COLUMN2.DAF',
///          .                                  'COLUMN3.DAF'  /
///
///           DATA                  NUMBER  /
///          .               11.D0, 21.D0, 31.D0, 41.D0, 51.D0,
///          .               61.D0, 71.D0, 81.D0, 91.D0,
///          .               12.D0, 22.D0, 32.D0, 42.D0, 52.D0,
///          .               62.D0, 72.D0, 82.D0, 92.D0,
///          .               13.D0, 23.D0, 33.D0, 43.D0, 52.D0,
///          .               63.D0, 73.D0, 83.D0, 93.D0        /
///
///     C
///     C     Create the new DAFs, and start a new array in each
///     C     one.  Just use the file name for the internal file
///     C     name and array name, for each DAF.  No assignments
///     C     are required for the array summaries.
///     C
///           DO I = 1, NDAF
///
///              CALL DAFOPN ( DAFNAM(I), ND, NI,
///          .                 DAFNAM(I), 0,  HAN(I) )
///
///              CALL DAFBNA ( HAN(I), SUMMRY, DAFNAM(I) )
///
///           END DO
///
///     C
///     C     At this point, we assume that we have read the
///     C     file line by line. Add the numbers from each column
///     C     to the corresponding DAF.
///     C
///           DO I = 1, MAXLNS
///
///     C
///     C        Add the number from the ith column to the array
///     C        in the ith DAF.  We'll use DAFCAD to select
///     C        the correct DAF to add data to.
///     C
///              DO J = 1, NDAF
///                 CALL DAFCAD ( HAN(J)         )
///                 CALL DAFADA ( NUMBER(I,J), 1 )
///              END DO
///
///           END DO
///
///     C
///     C     Finish ("end") the arrays.  Again, we'll use DAFCAD
///     C     to select the DAFs in which the arrays are to be
///     C     finished.  After finishing each array, close the DAF
///     C     containing it.
///     C
///           DO I = 1, NDAF
///              CALL DAFCAD ( HAN(I) )
///              CALL DAFENA
///              CALL DAFCLS ( HAN(I) )
///           END DO
///
///     C
///     C     Now for the verification step.  We'll try to
///     C     print a matrix containing the same data as
///     C     the original input file.  The format of the numbers,
///     C     the delimiters separating the numbers, spacing, and
///     C     non-printing characters may differ.
///     C
///     C     Open the DAFs for reading.
///     C
///           DO I = 1, NDAF
///              CALL DAFOPR ( DAFNAM(I), HAN(I) )
///           END DO
///
///     C
///     C     Obtain the start and end addresses of the
///     C     data in each DAF.  To do this, we'll need to
///     C     obtain and unpack the array summaries.
///     C
///     C     If all went well, the addresses should be the
///     C     same for each DAF.  We'll assume that the initial
///     C     and final addresses in the first DAF are correct
///     C     for all three.
///     C
///           CALL DAFBFS ( HAN(1) )
///           CALL DAFFNA ( FOUND  )
///           CALL DAFGS  ( SUMMRY )
///           CALL DAFUS  ( SUMMRY, ND, NI, DC, IC )
///
///           IA      =  IC( NI-1 )
///           FA      =  IC( NI   )
///           LENGTH  =  FA - IA + 1
///
///     C
///     C     Now read numbers from the DAFs and build up
///     C     lines of text.  Print these lines out.
///     C
///           DO I = 0,  LENGTH - 1
///
///              LINE = ' '
///
///              DO J = 1, NDAF
///
///                 CALL DAFRDA ( HAN(J), IA+I, IA+I, NUMDP )
///
///     C
///     C           Convert the double precision number to a
///     C           character string, and append it to the current
///     C           line.
///     C
///                 CALL DPSTR  ( NUMDP,  SIG,        NUMCH )
///                 CALL SUFFIX ( NUMCH,  3,          LINE  )
///
///              END DO
///
///              WRITE(*,*) LINE
///
///           END DO
///
///     C
///     C     Close the DAFs.
///     C
///           DO I = 1, NDAF
///              CALL DAFCLS( HAN(I) )
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///          1.100000000E+01    1.200000000E+01    1.300000000E+01
///          2.100000000E+01    2.200000000E+01    2.300000000E+01
///          3.100000000E+01    3.200000000E+01    3.300000000E+01
///          4.100000000E+01    4.200000000E+01    4.300000000E+01
///          5.100000000E+01    5.200000000E+01    5.200000000E+01
///          6.100000000E+01    6.200000000E+01    6.300000000E+01
///          7.100000000E+01    7.200000000E+01    7.300000000E+01
///          8.100000000E+01    8.200000000E+01    8.300000000E+01
///          9.100000000E+01    9.200000000E+01    9.300000000E+01
///
///
///     Note that after run completion, three new DAF files exist in
///     the output directory.
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header of DAFANA and all entry points to comply with
///         NAIF standard. Added complete code examples to DAFANA $Examples
///         section based on the existing fragments.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated the entry points of DAFANA to enable its
///         internal state table size, TBSIZE, to be smaller
///         than the file table maintained by DAFAH: FTSIZE.
///
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Updated to remove potential compiler warnings from the
///         truncation of double precision numbers to integers.
///
///         Also changed was a numeric constant from 1.D0 to the
///         equivalent, but more aesthetically pleasing 1.0D0.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Updated to support simultaneous writes to multiple DAFs.
///         The $Examples section of this routine now illustrates
///         usage of the routine DAFCAD.
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
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         This umbrella and its entry points were updated to
///         work properly with the changes in the DAF system as
///         a result of its utilization of the new handle manager.
///
///         Since DAFAH now tracks FTSIZE files as defined in
///         the include file 'zzddhman.inc', it was decided that
///         in the interest of releasing the toolkit this module
///         would undergo simple changes.  As such most previous
///         references to FTSIZE in this umbrella have been replaced
///         with TBSIZE where appropriate.  DAFBNA now signals an
///         error if there is not enough room to add a new DAF's
///         dossier to the state table.  Also, after attempting to
///         clean up all files listed in the state table that are
///         not currently open, DAFBNA attempts to locate the
///         first dossier with STADDG set to FALSE.  This is then
///         freed to make room for the new DAF.  If DAFBNA fails
///         to locate such a dossier in the state table, it
///         signals the error SPICE(STFULL).
///
///         The parameter FILEN was removed, as it is defined
///         on an environmental basis in the include file
///         'zzddhman.inc'.
///
///
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Updated to remove potential compiler warnings from the
///         truncation of double precision numbers to integers. Two
///         assignments to NARRAY were updated, being changed from:
///
///            NARRAY = SUMREC(ARYCNT)
///
///         to
///
///            NARRAY = IDINT ( SUMREC(ARYCNT) )
///
///         Also changed was a numeric constant from 1.D0 to the
///         equivalent, but more aesthetically pleasing 1.0D0.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Updated to support simultaneous writes to multiple DAFs.
///
///         In previous versions of DAFANA, data could be added to only
///         one DAF array at a time.  In fact, DAFAH allowed only one
///         DAF to be open for writing at any time.  Therefore, there was
///         no question about which DAF was being operated on by either of
///         the DAFANA entry points that don't accept file handles as
///         input arguments:  DAFADA and DAFENA.  In the current version
///         of DAFANA, the entry points that don't accept file handles as
///         inputs operate on the `current DAF'.  The current DAF is the
///         last one in which a new array was started by DAFBNA, or in
///         which addition of data to an array was continued by the new
///         entry point DAFCAD.  DAFCAD was added to allow users to set
///         the current DAF, so that additions of data to arrays in
///         multiple DAFs can be interleaved.
///
///         Note that the notion of `current DAF' as discussed here applies
///         only to DAFs acted upon by entry points of DAFANA. In DAFFA,
///         there is a DAF that is treated as the `current DAF' for
///         searching; there is no connection between the DAFs regarded
///         as current by DAFANA and DAFFA.
///
///         The two principal changes to DAFANA are the addition of the
///         new entry point DAFCAD, and the addition of a data structure
///         called the `state table'. The state table is a collection of
///         parallel arrays that maintain information about the state
///         of each data addition that is currently in progress. The
///         state table arrays are indexed by a singly linked list pool;
///         this mechanism allows addition and deletion of information
///         about data additions without requiring movement of data
///         already in the state table.
///
///         The linked list pool contains an `active' list and a `free'
///         list. Nodes in the active list are used to index elements of
///         the state table where information about additions in progress
///         is stored. The head node of the active list is of particular
///         significance: the state information pointed to by this node
///         is that of the current DAF. Nodes in the free list index
///         elements of the state table that are available for use.
///
///         When an array is started in a DAF that is not already `known'
///         to DAFANA, information about the DAF is added to the state
///         table. If there are no free elements in the state table,
///         the routine starting the array (DAFBNA) will perform garbage
///         collection: the routine will test the handles of each file
///         about which information in stored in the state table to see
///         whether that file is still open. Nodes containing information
///         about DAFs that are no longer open will be moved to the free
///         list.
///
///         Whenever a DAF becomes the current DAF, the linked list
///         that indexes the state table is adjusted so that the node
///         pointing to information about the current DAF is at the head
///         of the active list. This way, a slight efficiency is gained
///         when repeated data additions are made to the same DAF, since
///         the linear search through the state table for information on
///         that DAF will be shortened.
///
///         Since the algorithms for maintenance of linked lists are well
///         known, they are not documented here. However, see the
///         internals of the SPICELIB routine SPKBSR for a nice diagram
///         describing a similar data structure.
///
///         The state table contains two arrays that are quite large:
///         there are buffers that contain the name and array summary for
///         each array under construction. A parallel situation exists
///         in DAFFA, where there are buffers that contain the last
///         character record and summary record read from each DAF. The
///         total storage required for these arrays (in DAFANA and DAFFA
///         together) is 4000 * TBSIZE bytes. For this reason, it may be
///         a good idea to reduce the value of TBSIZE in SPICELIB versions
///         for machines where memory is scarce.
///
///         On a completely different topic: the local declarations in
///         DAFANA have been alphabetized and separated by type, except
///         for those relating to the state table. Several hard-coded
///         constants have been replaced by parameters.
/// ```
pub fn dafana(
    ctx: &mut SpiceContext,
    handle: i32,
    sum: &[f64],
    name: &str,
    data: &[f64],
    n: i32,
) -> crate::Result<()> {
    DAFANA(handle, sum, name.as_bytes(), data, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFANA ( DAF, add new array )
pub fn DAFANA(
    HANDLE: i32,
    SUM: &[f64],
    NAME: &[u8],
    DATA: &[f64],
    N: i32,
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
    // State variables.
    //
    // These variables define the state of each DAF to which data
    // is currently being added.  For each DAF that we're writing to, we
    // maintain a copy of:
    //
    //    STFH           File handle.
    //
    //    STIFNM         Internal file name.
    //
    //    STADDG         (`State table: adding') Flag indicating
    //                   whether addition of data to an array is in
    //                   progress.
    //
    //    STFRST         Record number of initial summary record.
    //
    //    STLAST         Record number of final summary record.
    //
    //    STBEGN         Beginning address of new array.
    //
    //    STFREE         Address of next free word.
    //
    //    STLSUM         Local copy of the array summary for the current
    //                   array.
    //
    //    STNAME         Local copy of the array name for the current
    //                   array.
    //
    //
    // These variables are maintained in a table of parallel arrays;
    // the size of the table is TBSIZE.
    //
    //

    //
    // The table of state variables is indexed by a singly linked list
    // of pointers.  This mechanism avoids the work of moving
    // the state variable data about as information about DAFs is
    // added to or deleted from the table.
    //
    // The structure containing the linked list pointers is called a
    // `pool.'  The pool contains a list of `active' nodes and a list
    // of free nodes.  The head nodes of the active and free lists are
    // maintained as the variables STHEAD (`state table head') and
    // STFPTR (`state table free pointer'), respectively.  Every node in
    // the pool is on exactly one of these lists.
    //

    //
    // The pool starts out with all of the nodes on the free list.
    // DAFBNA initializes the pool.  As new DAFs are written to,
    // DAFBNA adds information about them to the state table.  Every
    // time a DAF array is started by DAFBNA, or selected for
    // continuation by DAFCAD, the routine in question `moves' the
    // DAF's state information to the head of the active list, if the
    // state information is not already there.  This re-organization is
    // accomplished by deleting the node for the DAF from its current
    // position in the active list and inserting the node at the head of
    // the list.  Thus, the change is made merely by setting pointers,
    // not by moving chunks of data in the state table.
    //
    // It may happen that there is no room left in the state table
    // to accommodate information about a new DAF.  In this case,
    // garbage collection must be performed:  DAFBNA frees all nodes in
    // the table that index DAFs that are not currently open.
    //
    // Note that the routine DAFADA does not modify the state table; it
    // merely adds data to the DAF that is at the head of the active
    // list.
    //

    //
    // Other local variables
    //

    //
    // Save everything between calls
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
        CHKIN(b"DAFANA", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"DAFANA", ctx)?;
    }

    Ok(())
}

/// DAF, begin new array
///
/// Begin a new array in a DAF.
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
///  HANDLE     I   Handle of DAF.
///  SUM        I   Summary of new array.
///  NAME       I   Name of new array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF opened for write access
///           by a previous call to DAFOPW or DAFOPN.
///
///  SUM      is the summary of a new array to be added to the
///           specified file. The addresses (the final two integer
///           components) need not be filled in.
///
///  NAME     is the name of the new array.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is not that of a DAF that is open for
///      writing, an error is signaled by a routine in the call tree of
///      this routine. These files are implicitly of the native binary
///      file format.
///
///  2)  If the input array name is too long to fit in the number
///      of characters allowed by the summary format of the DAF
///      designated by HANDLE, the excess characters are truncated.
///      No error is signaled.
///
///  3)  If there is not enough room in the state table to add
///      the DAF associated with HANDLE, the error SPICE(STFULL)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE, above.
/// ```
///
/// # Particulars
///
/// ```text
///  Only one array can be added to a DAF at any one time, so
///  calling DAFBNA cancels any addition to the file specified
///  by HANDLE that may be in progress. No warning is issued.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in DAFANA.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated DAFBNA to support changes made to the DAF
///         system that utilize the new handle manager. See
///         the $Revisions section of DAFANA for a detailed
///         discussion of the changes.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Modified to support simultaneous writes to multiple DAFs.
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
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Modified to support simultaneous writes to multiple DAFs.
///         DAFBNA now adds information about DAFs to the state table,
///         deletes information about closed DAFs from the state table,
///         and initializes the state pool.
/// ```
pub fn dafbna(ctx: &mut SpiceContext, handle: i32, sum: &[f64], name: &str) -> crate::Result<()> {
    DAFBNA(handle, sum, name.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFBNA ( DAF, begin new array )
pub fn DAFBNA(HANDLE: i32, SUM: &[f64], NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SUM = DummyArray::new(SUM, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFBNA", ctx)?;
    }

    //
    // Check out the file handle before going any further.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFBNA", ctx)?;
        return Ok(());
    }

    //
    // Initialize the state table pool, if this hasn't been done yet.
    // Also initialize the cell used to obtain the set of handles of
    // open DAFs.
    //

    if save.FIRST {
        SSIZEI(FTSIZE, save.OPNSET.as_slice_mut(), ctx)?;

        for I in 1..=(TBSIZE - 1) {
            save.STPOOL[I] = (I + 1);
        }

        save.STPOOL[TBSIZE] = NIL;
        save.STFPTR = 1;
        save.STHEAD = NIL;
        save.FIRST = false;
    }

    //
    // We know that the beginning of the array will be the first
    // free address in the file. We also need the summary format.
    // Get both items from the file record.
    //
    // We won't use the information we're obtaining now until
    // after we've placed the state information for the current
    // DAF at the head of the active list, but we want to make sure
    // that we can actually read the file record first.  So, we
    // do the read now and avoid modifying the active list if the
    // read fails.
    //
    DAFRFR(
        HANDLE,
        &mut save.ND,
        &mut save.NI,
        &mut save.IFNAME,
        &mut save.FWARD,
        &mut save.BWARD,
        &mut save.FREE,
        ctx,
    )?;

    //
    // If we couldn't read the file record, bail out now.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFBNA", ctx)?;
        return Ok(());
    }

    //
    // See whether we already have an entry for this DAF in the
    // state table.  Find the previous node if possible.
    //
    save.P = save.STHEAD;
    save.PREV = NIL;
    save.FOUND = false;

    while ((save.P != NIL) && !save.FOUND) {
        if (save.STFH[save.P] == HANDLE) {
            save.FOUND = true;
        } else {
            save.PREV = save.P;
            save.P = save.STPOOL[save.P];
        }
    }

    //
    // At this point, either FOUND is false, or P points to a
    // state table entry describing the DAF indicated by HANDLE.
    // In the latter case, PREV is the predecessor of P.
    //
    //
    if save.FOUND {
        //
        // We already have a dossier on this DAF.  We already have
        // the information on the summary format, but we must re-set
        // the rest of our state information.
        //
        // Rather than doing the update here, we do it outside of this
        // IF block.  That way, the update gets done in just one place.
        // This just makes life easier:  if the collection of state
        // variables is changed, there are fewer places to forget to
        // make the required code changes.
        //
        // Move the node for this DAF to the head of the active list,
        // if it is not already there:
        //
        //    - Make the predecessor of P point to the successor of P.
        //
        //    - Make P point to the head of the active list.
        //
        //    - Make P the active list head node.
        //
        //
        if (save.P != save.STHEAD) {
            //
            // P is in the active list, but is not at the head.  So,
            // the predecessor of P is not NIL.
            //
            save.STPOOL[save.PREV] = save.STPOOL[save.P];
            save.STPOOL[save.P] = save.STHEAD;
            save.STHEAD = save.P;
        }
    } else {
        //
        // We don't yet have any information on this DAF.  Make a new
        // state table entry for the DAF.  We may need to make room for
        // the new information by freeing space allocated to DAFs that
        // are no longer open.
        //
        if (save.STFPTR == NIL) {
            //
            // Oops, we're out of space.  Time for garbage collection.
            // Test each file handle to see whether it designates a DAF
            // that is still open.  DAFHOF will tell us which handles
            // point to open DAFs.
            //
            DAFHOF(save.OPNSET.as_slice_mut(), ctx)?;

            save.P = save.STHEAD;
            save.PREV = NIL;
            //
            // For every DAF file represented in the state table, we'll
            // delete the corresponding state information if the DAF is
            // now closed.  We traverse the active list, examining each
            // file handle as we go.
            //

            while (save.P != NIL) {
                if ELEMI(save.STFH[save.P], save.OPNSET.as_slice(), ctx)? {
                    //
                    // The file is open. Have a look at the next node.
                    //
                    save.PREV = save.P;
                    save.P = save.STPOOL[save.P];
                } else {
                    //
                    // This file handle is not on the list, so free the
                    // node pointing to the information about the DAF it
                    // designated:
                    //
                    //    - Save the successor of P.
                    //
                    //    - Link the predecessor of node P to the successor
                    //      of P, if the predecessor is not NIL.
                    //
                    //    - If it happens that P is the head node of the
                    //      active list, set the head equal to the
                    //      successor of P.
                    //
                    //    - Link P into the free list.
                    //
                    //    - Set P equal to its saved successor.
                    //
                    //    - (PREV remains unchanged.)
                    //
                    //
                    save.NEXTP = save.STPOOL[save.P];

                    if (save.P == save.STHEAD) {
                        //
                        // Re-assign STHEAD so that we don't lose the head
                        // of the active list.  P has no predecessor in this
                        // case, so there's no need to set the forward pointer
                        // of node PREV.
                        //
                        save.STHEAD = save.NEXTP;
                    } else {
                        //
                        // Since P is not the head node of the active list,
                        // PREV is not NIL, so we'll need to set the forward
                        // pointer of node PREV.
                        //
                        save.STPOOL[save.PREV] = save.NEXTP;
                    }

                    save.STPOOL[save.P] = save.STFPTR;
                    save.STFPTR = save.P;
                    save.P = save.NEXTP;
                }
            }

            //
            // At this point, we've freed all nodes from the active
            // list that were used to index information about DAFs that
            // are no longer open.  Now see if we still need to make
            // room.  If so, locate the first dossier with STADDG(P)
            // set to FALSE.  We know then that this file is not
            // currently involved in an array addition.
            //
            if (save.STFPTR == NIL) {
                save.FOUND = false;
                save.P = save.STHEAD;
                save.PREV = NIL;

                while ((save.P != NIL) && !save.FOUND) {
                    //
                    // If STADDG(P) is TRUE, then we must continue
                    // searching.
                    //
                    if save.STADDG[save.P] {
                        save.PREV = save.P;
                        save.P = save.STPOOL[save.P];
                    } else {
                        save.FOUND = true;
                        //
                        // No array is presently being added to the DAF
                        // associated with this dossier, so free the
                        // node pointing to the information about the DAF it
                        // designated:
                        //
                        // - Save the successor of P.
                        //
                        // - Link the predecessor of node P to the successor
                        //   of P, if the predecessor is not NIL.
                        //
                        // - If it happens that P is the head node of the
                        //   active list, set the head equal to the
                        //   successor of P.
                        //
                        // - Link P into the free list.
                        //
                        // - Set P equal to its saved successor.
                        //
                        // - (PREV remains unchanged.)
                        //
                        //
                        save.NEXTP = save.STPOOL[save.P];

                        if (save.P == save.STHEAD) {
                            //
                            // Re-assign STHEAD so that we don't lose the head
                            // of the active list.  P has no predecessor in
                            // this case, so there's no need to set the
                            // forward pointer of node PREV.
                            //
                            save.STHEAD = save.NEXTP;
                        } else {
                            //
                            // Since P is not the head node of the active list,
                            // PREV is not NIL, so we'll need to set the
                            // forward pointer of node PREV.
                            //
                            save.STPOOL[save.PREV] = save.NEXTP;
                        }

                        save.STPOOL[save.P] = save.STFPTR;
                        save.STFPTR = save.P;
                        save.P = save.NEXTP;
                    }
                }
            }

            //
            // Now, check to see if there is now room to add the dossier
            // for the new DAF to the state table.  If not signal an error.
            //
            if (save.STFPTR == NIL) {
                SETMSG(b"Attempt to initiate create a new array in DAF \'#\' has failed. DAFANA\'s state table has room to manage writing to # new arrays simultaneously, but there is no room left in the table for this DAF.", ctx);
                ERRHAN(b"#", HANDLE, ctx)?;
                ERRINT(b"#", TBSIZE, ctx);
                SIGERR(b"SPICE(STFULL)", ctx)?;
                CHKOUT(b"DAFBNA", ctx)?;
                return Ok(());
            }
        }

        //
        // If we reach here, then we have room in the state table for
        // the new DAF.  The first free node is indicated by SFTPTR.
        // Allocate this node and use it to index the state information
        // for the new DAF.
        //
        save.P = save.STFPTR;

        //
        // Update the free list pointer, link P to the previous head
        // of the active list, and make P the head of the active list.
        //
        save.STFPTR = save.STPOOL[save.P];
        save.STPOOL[save.P] = save.STHEAD;
        save.STHEAD = save.P;
    }

    //
    // At this point, P is the head node of the active list, and P is
    // the index in the state table of the information for the current
    // DAF.
    //

    //
    // Set the state information for the current array.
    //
    save.STFH[save.P] = HANDLE;
    fstr::assign(save.STIFNM.get_mut(save.P), &save.IFNAME);
    save.STADDG[save.P] = true;
    save.STFRST[save.P] = save.FWARD;
    save.STLAST[save.P] = save.BWARD;
    save.STBEGN[save.P] = save.FREE;
    save.STFREE[save.P] = save.FREE;

    //
    // Find out how big the array summary is supposed to be.
    //
    DAFHSF(save.STFH[save.P], &mut save.ND, &mut save.NI, ctx)?;

    save.SUMSIZ = (save.ND + ((save.NI + 1) / 2));

    //
    // Set the local copies of the array's summary and name.
    //
    MOVED(
        SUM.as_slice(),
        save.SUMSIZ,
        save.STLSUM.subarray_mut([1, save.P]),
    );

    fstr::assign(save.STNAME.get_mut(save.P), NAME);

    CHKOUT(b"DAFBNA", ctx)?;
    Ok(())
}

/// DAF, add data to array
///
/// Add one or more double precision words of data to the newest
/// array in the current DAF.
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
///  DATA       I   Elements of the new array.
///  N          I   Number of elements in DATA.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DATA     is an arbitrary number of double precision words to
///           be added to the data in the array being created.
///
///  N        is the number of double precision words in DATA.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there are no DAFs to which data is currently being added,
///      the error SPICE(DAFNOWRITE) is signaled.
///
///  2)  If a new array has not been started in the current DAF (by a
///      call to DAFBNA), the error SPICE(DAFNEWCONFLICT) is signaled.
///
///  3)  If N is less than one, no data are added to the file.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFADA adds data to the last array begun by DAFBNA or selected
///  by DAFCAD.
///
///  Data can be added to a DAF in chunks of any size, so long
///  as the chunks are added in the proper order.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in DAFANA.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated entry points to support changes made to the DAF
///         system that utilize the new handle manager. See
///         the $Revisions section of DAFANA for a detailed
///         discussion of the changes.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Updated to work with new DAF routines that allow writing
///         to multiple DAFs simultaneously. Functionality for
///         applications that write to one DAF at a time is unchanged.
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
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Updated to work with new DAF routines that allow writing
///         to multiple DAFs simultaneously. Functionality for
///         applications that write to one DAF at a time is unchanged.
/// ```
pub fn dafada(ctx: &mut SpiceContext, data: &[f64], n: i32) -> crate::Result<()> {
    DAFADA(data, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFADA ( DAF, add data to array )
pub fn DAFADA(DATA: &[f64], N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyArray::new(DATA, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFADA", ctx)?;
    }

    //
    // This routine operates on the DAF at the head of the active list.
    //
    save.P = save.STHEAD;

    //
    // We must make sure that the requested addition can be performed.
    // We don't validate the file handle here because this is one place
    // where we are concerned about speed.  The low-level writer routine
    // DAFWDR will handle the check.
    //
    if (save.P == NIL) {
        SETMSG(b"No DAF is currently being written.", ctx);
        SIGERR(b"SPICE(DAFNOWRITE)", ctx)?;
        CHKOUT(b"DAFADA", ctx)?;
        return Ok(());

    //
    // An array cannot be extended unless begun first.
    //
    } else if !save.STADDG[save.P] {
        //
        // Validate the current handle, then get the name of the DAF.
        //
        DAFSIH(save.STFH[save.P], b"WRITE", ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFADA", ctx)?;
            return Ok(());
        }

        DAFHFN(save.STFH[save.P], &mut save.DAFNAM, ctx)?;
        SETMSG(
            b"An attempt was made to add data to an array that has not yet been begun, in file #.",
            ctx,
        );
        ERRCH(b"#", &save.DAFNAM, ctx);
        SIGERR(b"SPICE(DAFNEWCONFLICT)", ctx)?;
        CHKOUT(b"DAFADA", ctx)?;
        return Ok(());

    //
    // Start adding data at the first free address, then update that
    // address to get ready for the next addition.
    //
    } else if (N >= 1) {
        DAFWDA(
            save.STFH[save.P],
            save.STFREE[save.P],
            ((save.STFREE[save.P] + N) - 1),
            DATA.as_slice(),
            ctx,
        )?;
        save.STFREE[save.P] = (save.STFREE[save.P] + N);
    }

    CHKOUT(b"DAFADA", ctx)?;
    Ok(())
}

/// DAF, end new array
///
/// End the addition of data to the newest array in the current DAF.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Exceptions
///
/// ```text
///  1)  If there are no DAFs to which data is currently being added,
///      the error SPICE(DAFNOWRITE) is signaled, or the error will
///      be detected by routines called by this routine.
///
///  2)  If a new array has not been started in the current DAF (by a
///      call to DAFBNA), the error SPICE(DAFNEWCONFLICT) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFENA makes the current array a permanent addition to the
///  current DAF.
///
///  The pointers within the file are not changed until an array
///  is ended successfully. If an error occurs or if the current
///  DAF is closed before DAFENA is called, the last array will
///  not be visible to the DAF reader routines.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in DAFANA.
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries from $Revisions section.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated entry points to support changes made to the DAF
///         system that utilize the new handle manager. See
///         the $Revisions section of DAFANA for a detailed
///         discussion of the changes.
///
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Updated to remove potential compiler warnings from the
///         truncation of double precision numbers to integers.
///
///         Also changed was a numeric constant from 1.D0 to the
///         equivalent, but more aesthetically pleasing 1.0D0.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB) (WLT)
///
///         Updated to work with new DAF routines that allow writing
///         to multiple DAFs simultaneously. Functionality for
///         applications that write to one DAF at a time is unchanged.
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
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Updated to remove potential compiler warnings from the
///         truncation of double precision numbers to integers. Two
///         assignments to NARRAY were updated, being changed from:
///
///            NARRAY = SUMREC(ARYCNT)
///
///         to
///
///            NARRAY = IDINT ( SUMREC(ARYCNT) )
///
///         Also changed was a numeric constant from 1.D0 to the
///         equivalent, but more aesthetically pleasing 1.0D0.
/// ```
pub fn dafena(ctx: &mut SpiceContext) -> crate::Result<()> {
    DAFENA(ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFENA ( DAF, end new array )
pub fn DAFENA(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFENA", ctx)?;
    }

    //
    // This routine operates on the DAF at the head of the active list.
    //
    save.P = save.STHEAD;

    if (save.P == NIL) {
        SETMSG(b"No DAF is currently being written.", ctx);
        SIGERR(b"SPICE(DAFNOWRITE)", ctx)?;
        CHKOUT(b"DAFENA", ctx)?;
        return Ok(());

    //
    // A new array cannot be ended unless begun first.
    //
    } else if !save.STADDG[save.P] {
        //
        // Validate the current handle, then get the name of the DAF.
        //
        DAFSIH(save.STFH[save.P], b"WRITE", ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFENA", ctx)?;
            return Ok(());
        }

        DAFHFN(save.STFH[save.P], &mut save.DAFNAM, ctx)?;
        SETMSG(
            b"An attempt was made to end an array that has not yet been begun, in file #.",
            ctx,
        );
        ERRCH(b"#", &save.DAFNAM, ctx);
        SIGERR(b"SPICE(DAFNEWCONFLICT)", ctx)?;
        CHKOUT(b"DAFENA", ctx)?;
        return Ok(());
    }

    //
    // No more data. The array ends just before the next free
    // address. The summary should be complete except for the
    // initial and final addresses of the data, of which we
    // have been keeping track.
    //
    DAFHSF(save.STFH[save.P], &mut save.ND, &mut save.NI, ctx)?;

    DAFUS(
        save.STLSUM.subarray([1, save.P]),
        save.ND,
        save.NI,
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
    );

    save.IC[(save.NI - 1)] = save.STBEGN[save.P];
    save.IC[save.NI] = (save.STFREE[save.P] - 1);

    DAFPS(
        save.ND,
        save.NI,
        save.DC.as_slice(),
        save.IC.as_slice(),
        save.STLSUM.subarray_mut([1, save.P]),
    );

    //
    // The summary should be stored in the final summary record (the
    // one at the end of the file). Get that entire record, and the
    // corresponding name record.
    //
    DAFRDR(
        save.STFH[save.P],
        save.STLAST[save.P],
        1,
        DPRSIZ,
        save.SUMREC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    DAFRCR(
        save.STFH[save.P],
        (save.STLAST[save.P] + 1),
        &mut save.NAMREC,
        ctx,
    )?;
    save.NARRAY = (save.SUMREC[ARYCNT] as i32);

    //
    // The number of arrays determines where the summary and name
    // are stored within the summary record. Adding this array increases
    // the number of arrays by one.
    //
    save.SUMSIZ = (save.ND + ((save.NI + 1) / 2));
    save.DLOC = ((ARYCNT + 1) + (save.NARRAY * save.SUMSIZ));

    MOVED(
        save.STLSUM.subarray([1, save.P]),
        save.SUMSIZ,
        save.SUMREC.subarray_mut(save.DLOC),
    );

    save.NAMSIZ = (8 * save.SUMSIZ);
    save.CLOC = (1 + (save.NARRAY * save.NAMSIZ));

    fstr::assign(
        fstr::substr_mut(
            &mut save.NAMREC,
            save.CLOC..=((save.CLOC + save.NAMSIZ) - 1),
        ),
        save.STNAME.get(save.P),
    );

    save.SUMREC[ARYCNT] = (save.SUMREC[ARYCNT] + 1.0);
    save.NARRAY = (save.SUMREC[ARYCNT] as i32);

    //
    // Usually, adding an array does not fill the final summary
    // record, and it can simply be replaced.
    //
    if (save.NARRAY < (MAXSUM / save.SUMSIZ)) {
        DAFWDR(
            save.STFH[save.P],
            save.STLAST[save.P],
            save.SUMREC.as_slice(),
            ctx,
        )?;
        DAFWCR(
            save.STFH[save.P],
            (save.STLAST[save.P] + 1),
            &save.NAMREC,
            ctx,
        )?;

    //
    // When the record becomes full, a new one must be written.
    // However, this fact should be transparent to the user.
    //
    } else {
        //
        // The new summary record will be stored in the next free record
        // in the file. This summary record should point to it.
        //
        // To find out which record the next free address is in, we use
        // DAFARW (`address to record and word').
        //
        DAFARW(
            (save.STFREE[save.P] - 1),
            &mut save.NEXT,
            &mut save.WORD,
            ctx,
        )?;
        save.NEXT = (save.NEXT + 1);
        save.SUMREC[FWDPTR] = (save.NEXT as f64);

        DAFWDR(
            save.STFH[save.P],
            save.STLAST[save.P],
            save.SUMREC.as_slice(),
            ctx,
        )?;
        DAFWCR(
            save.STFH[save.P],
            (save.STLAST[save.P] + 1),
            &save.NAMREC,
            ctx,
        )?;

        //
        // The new summary record should point backwards to the one just
        // written, and should point forwards to nothing. Of course,
        // it contains no summaries, and no names.
        //
        CLEARD(DPRSIZ, save.SUMREC.as_slice_mut());
        save.SUMREC[FWDPTR] = 0.0;
        save.SUMREC[BWDPTR] = (save.STLAST[save.P] as f64);
        save.SUMREC[ARYCNT] = 0.0;
        fstr::assign(&mut save.NAMREC, b" ");

        DAFWDR(save.STFH[save.P], save.NEXT, save.SUMREC.as_slice(), ctx)?;
        DAFWCR(save.STFH[save.P], (save.NEXT + 1), &save.NAMREC, ctx)?;

        //
        // If a new summary record  was added, the first free address
        // lies just beyond the end of the matching character record.
        //
        // We use DAFRWA (`record and word to address') to calculate
        // the next free address.
        //
        save.STLAST[save.P] = save.NEXT;
        DAFRWA((save.STLAST[save.P] + 2), 1, &mut save.STFREE[save.P], ctx)?;
    }

    //
    // The new value STFREE(P) must be rewritten in the file record each
    // time a new array is added. If a new record was added, the new
    // value of STLAST(P) will be rewritten as well.
    //
    DAFWFR(
        save.STFH[save.P],
        save.ND,
        save.NI,
        &save.STIFNM[save.P],
        save.STFRST[save.P],
        save.STLAST[save.P],
        save.STFREE[save.P],
        ctx,
    )?;

    //
    // Ready for another array.
    //
    save.STADDG[save.P] = false;

    CHKOUT(b"DAFENA", ctx)?;
    Ok(())
}

/// DAF, continue adding data
///
/// Select a DAF that already has a new array in progress as the
/// one to continue adding data to.
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
///  HANDLE     I   Handle of DAF to continue adding data to.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF that is open for write
///           access and in which a new array has been
///           started by a call to DAFBNA.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is not that of a DAF that is open for
///      writing, an error is signaled by a routine in the call tree of
///      this routine.
///
///  2)  If no array is currently being added to in the file indicated
///      by HANDLE, the error SPICE(NOARRAYSTARTED) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFCAD supports simultaneous addition of data to arrays in
///  multiple DAFs. In applications that use this capability,
///  DAFCAD should be called prior to each call to DAFADA or DAFENA
///  to specify which DAF is to be acted upon.
///
///  Here is a code fragment that adds a new array to each of N
///  existing DAFs, simultaneously. The data to be added to each
///  is broken up into M chunks; one chunk is written to each DAF
///  at a time. The data is contained in the array CHUNK, dimensioned
///
///      DOUBLE PRECISION      CHUNK ( MAXDAT, M, N )
///
///  The actual amount of data in the Jth chunk for the Ith file is
///  given by
///
///      AMOUNT (J,I)
///
///
///
///      DO I = 1, N
///         CALL DAFOPW ( HANDLE(I) )
///         CALL DAFBNA ( HANDLE(I) )
///      END DO
///
///      DO J = 1, M
///
///         DO I = 1, N
///            CALL DAFCAD  ( HANDLE(I)                  )
///            CALL DAFADA  ( CHUNK(1,J,I),  AMOUNT(J,I) )
///         END DO
///
///      END DO
///
///      DO I = 1, N
///         CALL DAFCAD  ( HANDLE(I) )
///         CALL DAFENA
///      END DO
///
///
///  Note that if we write all of the data for each array to just one
///  DAF at a time, we don't need to use DAFCAD:
///
///     DO I = 1, N
///
///        CALL DAFOPW ( HANDLE(I) )
///        CALL DAFBNA ( HANDLE(I) )
///
///        DO J = 1, M
///           CALL DAFADA ( CHUNK(1,J,I),  AMOUNT(J,I) )
///        END DO
///
///        CALL DAFENA
///
///     END DO
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in DAFANA.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Edited entry #2
///         in $Exceptions section: No write in progress is detected by
///         this routine.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
///
///         Updated entry points to support changes made to the DAF
///         system that utilize the new handle manager. See
///         the $Revisions section of DAFANA for a detailed
///         discussion of the changes.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 04-SEP-1991 (NJB) (WLT)
/// ```
pub fn dafcad(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DAFCAD(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFCAD ( DAF, continue adding data )
pub fn DAFCAD(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFCAD", ctx)?;
    }

    //
    // Check out the file handle before going any further.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFCAD", ctx)?;
        return Ok(());
    }

    //
    // See whether we already have an entry for this DAF in the
    // state table.  Find the previous node if possible.
    //
    save.P = save.STHEAD;
    save.PREV = NIL;
    save.FOUND = false;

    while ((save.P != NIL) && !save.FOUND) {
        if (save.STFH[save.P] == HANDLE) {
            save.FOUND = true;
        } else {
            save.PREV = save.P;
            save.P = save.STPOOL[save.P];
        }
    }

    //
    // Either FOUND is false, or P is the index in the state table of
    // the DAF specified by HANDLE, and PREV is the predecessor of P.
    //

    //
    // You can't continue writing to a DAF that you're not
    // already writing to.
    //
    if !save.FOUND {
        DAFHFN(HANDLE, &mut save.DAFNAM, ctx)?;
        SETMSG(b"No write in progress to #. (Handle was #.) ", ctx);
        ERRCH(b"#", &save.DAFNAM, ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(NOARRAYSTARTED)", ctx)?;
        CHKOUT(b"DAFCAD", ctx)?;
        return Ok(());
    } else if !save.STADDG[save.P] {
        DAFHFN(HANDLE, &mut save.DAFNAM, ctx)?;
        SETMSG(b"No write in progress to #. (Handle was #.) ", ctx);
        ERRCH(b"#", &save.DAFNAM, ctx);
        ERRINT(b"#", HANDLE, ctx);
        SIGERR(b"SPICE(NOARRAYSTARTED)", ctx)?;
        CHKOUT(b"DAFCAD", ctx)?;
        return Ok(());
    }

    //
    // Move the node for this DAF to the head of the active list,
    // if it is not already there:
    //
    //    - Make the predecessor of P point to the successor of P.
    //
    //    - Make P point to the head of the active list.
    //
    //    - Make P the active list head node.
    //
    //

    if (save.P != save.STHEAD) {
        //
        // P is in the active list, but is not at the head.  So,
        // the predecessor of P is not NIL.
        //
        save.STPOOL[save.PREV] = save.STPOOL[save.P];
        save.STPOOL[save.P] = save.STHEAD;
        save.STHEAD = save.P;
    }

    CHKOUT(b"DAFCAD", ctx)?;
    Ok(())
}
