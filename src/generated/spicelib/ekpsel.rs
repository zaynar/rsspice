//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const CNAMSZ: i32 = 32;
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const EQARCH: i32 = 2;
const EQINIT: i32 = (EQARCH + 1);
const EQPARS: i32 = (EQINIT + 1);
const EQNRES: i32 = (EQPARS + 1);
const EQTRES: i32 = (EQNRES + 1);
const EQSCHK: i32 = (EQTRES + 1);
const EQNTAB: i32 = (EQSCHK + 1);
const EQNCNS: i32 = (EQNTAB + 1);
const EQMXML: i32 = -1;
const EQNCNJ: i32 = (EQNCNS + 1);
const EQNORD: i32 = (EQNCNJ + 1);
const EQNSEL: i32 = (EQNORD + 1);
const EQNSIZ: i32 = (EQNSEL + 1);
const EQNPTR: i32 = (EQNSIZ + 1);
const EQCSIZ: i32 = (EQNPTR + 1);
const EQCPTR: i32 = (EQCSIZ + 1);
const EQBSEL: i32 = (EQCPTR + 1);
const EQBCON: i32 = (EQBSEL + 1);
const EQBCNJ: i32 = (EQBCON + 1);
const EQBORD: i32 = (EQBCON + 1);
const EQVBAS: i32 = EQBORD;
const EQDTYP: i32 = 1;
const EQBLEX: i32 = (EQDTYP + 1);
const EQELEX: i32 = (EQBLEX + 1);
const EQBSTR: i32 = (EQELEX + 1);
const EQESTR: i32 = (EQBSTR + 1);
const EQVPTR: i32 = (EQELEX + 1);
const EQVDSZ: i32 = 6;
const EQBCOL: i32 = 1;
const EQCIDX: i32 = EQVDSZ;
const EQBTAB: i32 = 1;
const EQTORD: i32 = EQVDSZ;
const EQCTYP: i32 = 1;
const EQCOL: i32 = 1;
const EQVAL: i32 = 2;
const EQLTAB: i32 = (EQCTYP + 1);
const EQLCOL: i32 = (EQLTAB + EQVDSZ);
const EQOPCD: i32 = (EQLCOL + EQVDSZ);
const EQRTAB: i32 = (EQOPCD + 1);
const EQRCOL: i32 = (EQRTAB + EQVDSZ);
const EQBVAL: i32 = (EQOPCD + 1);
const EQCDSZ: i32 = (2 + (4 * EQVDSZ));
const EQOTAB: i32 = 1;
const EQOCOL: i32 = (EQOTAB + EQVDSZ);
const EQODIR: i32 = (EQOCOL + EQVDSZ);
const EQODSZ: i32 = (1 + (2 * EQVDSZ));
const EQASND: i32 = 0;
const EQDSND: i32 = 1;
const EQSTAB: i32 = 1;
const EQSCOL: i32 = (EQSTAB + EQVDSZ);
const EQSDSZ: i32 = (2 * EQVDSZ);
const EQIMIN: i32 =
    (((((EQVBAS + ((10 * EQVDSZ) * 2)) + (1000 * EQCDSZ)) + 1000) + (10 * EQODSZ)) + (50 * EQSDSZ));
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const NTYPES: i32 = 4;
const SHORT: i32 = 4;

struct SaveVars {
    CHRTYP: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CHRTYP = ActualCharArray::new(SHORT, 1..=NTYPES);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CHR"),
                Val::C(b"DP"),
                Val::C(b"INT"),
                Val::C(b"TIME"),
            ]
            .into_iter();
            CHRTYP
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { CHRTYP }
    }
}

/// EK, parse SELECT clause
///
/// Parse the SELECT clause of an EK query, returning full particulars
/// concerning each selected item.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  QUERY      I   EK query.
///  N          O   Number of items in SELECT clause of QUERY.
///  XBEGS      O   Begin positions of expressions in SELECT clause.
///  XENDS      O   End positions of expressions in SELECT clause.
///  XTYPES     O   Data types of expressions.
///  XCLASS     O   Classes of expressions.
///  TABS       O   Names of tables qualifying SELECT columns.
///  COLS       O   Names of columns in SELECT clause of QUERY.
///  ERROR      O   Error flag.
///  ERRMSG     O   Parse error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  QUERY    is a character string containing an EK query.
///           EK queries have the general form
///
///              SELECT <select expr>, <select expr>, ...
///              FROM <table spec>, <table spec>, ...
///              [WHERE <constraint list>]
///              [ORDER BY <order-by column list>]
///
///           Here the symbol <select expr> indicates any
///           expression representing an entity that can be
///           selected. Commonly, the selected items are
///           columns, with or without qualifying table names,
///           having the form
///
///              <column name>
///              <table name>.<column name>
///              <table alias>.<column name>
///
///           but more general expressions may also be selected.
///           Examples are functions, such as
///
///              COUNT(*)
///              COUNT( <table name>.<column name> )
///              MAX  ( <table name>.<column name> )
///
///           or expressions involving constants, such as
///
///              2 * <column name>
/// ```
///
/// # Detailed Output
///
/// ```text
///  N        is the number of items specified in the
///           SELECT clause of the input query.
///
///  XBEGS,
///  XENDS    are, respectively, arrays of begin and end
///           positions of expressions designating items in the
///           SELECT clause of the input query. The Ith
///           expression is located in the substring
///
///              QUERY ( XBEGS(I) : XENDS(I) )
///
///  XTYPES   is an array of short strings indicating the data
///           types of the expressions in the SELECT clause.
///           Values and meanings of XTYPES are:
///
///              'CHR'        Character type
///              'DP'         Double precision type
///              'INT'        Integer type
///              'TIME'       Time type
///
///           The Ith element of XTYPES refers to the Ith
///           selected item.
///
///           The data type of an expression indicates which
///           fetch routine to use to obtain values of the
///           selected expression. The mapping of data types
///           to fetch routines is shown below:
///
///              'CHR'        EKGC
///              'DP'         EKGD
///              'INT'        EKGI
///              'TIME'       EKGD
///
///           Note that time values are stored as d.p. numbers.
///
///  XCLASS   is an array of short strings giving the classes
///           of the expressions occurring in the SELECT clause
///           of the input query. Values and meanings of
///           XCLASS are:
///
///              'COL'        Selected item was a column. The
///                           column may qualified.
///
///              'FUNC'       Selected item was a simple
///                           function invocation of the form
///
///                              F ( <column> )
///
///                           or else was
///
///                              COUNT(*)
///
///              'EXPR'       Selected item was a more general
///                           expression than those shown above.
///
///           The Ith element of XCLASS refers to the Ith
///           selected item.
///
///           When a selected item is a column, the values of
///           the arguments TABS and COLS (discussed below) are
///           defined.
///
///  TABS     is an array of names of tables corresponding to
///           the columns in the SELECT clause. The Ith element
///           of TABS corresponds to the table containing the
///           Ith SELECT column. Table names returned in TABS
///           are the actual names of tables in loaded EK, not
///           aliases supplied in the input query. Table names
///           are supplied even if the corresponding column was
///           unqualified in the input query, as long as the
///           column name was unambiguous.
///
///           The contents of TABS(I) are defined if and only if
///           the returned value of XCLASS(I) is 'COL'.
///
///  COLS     is an array containing the columns of the SELECT
///           clause. The contents of COLS(I) are defined if and
///           only if the returned value of XCLASS(I) is 'COL'.
///
///  ERROR    is a logical flag indicating whether the input
///           QUERY parsed correctly. The other outputs of this
///           routine, except for ERRMSG, are undefined if a
///           parse error occurred. ERROR is returned .TRUE. if
///           a parse error occurred, .FALSE. otherwise.
///
///  ERRMSG   is a character string describing the cause of a
///           parse error, if such an error occurred. Otherwise,
///           ERRMSG is returned blank.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  Parse failures do not cause this routine to signal errors;
///      instead, the ERROR and ERRMSG outputs indicate invalid
///      QUERY.
///
///  2)  Queries cannot be parsed correctly unless at least one EK
///      is loaded.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows callers of the EK fetch routines to determine
///  at run time the attributes of the columns from which data is to be
///  fetched.
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
///  1) Query the EK system and fetch data matching that query.
///
///     The program shown here does not rely on advance
///     knowledge of the input query or the contents of any loaded EK
///     files.
///
///     To simplify the example, we assume that all data are scalar.
///     This assumption relieves us of the need to test the size of
///     column entries before fetching them. In the event that a
///     column contains variable-size array entries, the entry point
///     EKNELT may be called to obtain the size of column entries to
///     be fetched. See EKNELT for an example.
///
///
///     Use the EK kernel below to load the information from the
///     original Supplementary Engineering Data Record (SEDR) data
///     set generated by the Viking Project.
///
///        vo_sedr.bdb
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM EKPSEL_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include EK Query Limit Parameters
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               RTRIM
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME = 'vo_sedr.bdb' )
///
///           CHARACTER*(*)         LSKNAM
///           PARAMETER           ( LSKNAM = 'naif0012.tls' )
///
///           INTEGER               DESCSZ
///           PARAMETER           ( DESCSZ = 31   )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///           INTEGER               ITEMSZ
///           PARAMETER           ( ITEMSZ = DESCSZ + 4 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 27   )
///
///           INTEGER               TYPLEN
///           PARAMETER           ( TYPLEN = 4    )
///
///           INTEGER               XCLSLN
///           PARAMETER           ( XCLSLN = 4    )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(MAXSTR)    CDATA
///           CHARACTER*(MAXCLN)    COLS   ( MAXSEL )
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(ITEMSZ)    ITEM
///           CHARACTER*(DESCSZ)    OUTSTR
///           CHARACTER*(MAXQRY)    QUERY
///           CHARACTER*(TIMLEN)    UTCSTR
///           CHARACTER*(MAXCLN)    TABS   ( MAXTAB )
///           CHARACTER*(XCLSLN)    XCLASS ( MAXSEL )
///           CHARACTER*(TYPLEN)    XTYPES ( MAXSEL )
///
///           DOUBLE PRECISION      DDATA
///           DOUBLE PRECISION      TDATA
///
///           INTEGER               B
///           INTEGER               COLNO
///           INTEGER               E
///           INTEGER               HANDLE
///           INTEGER               IDATA
///           INTEGER               N
///           INTEGER               NMROWS
///           INTEGER               ROW
///           INTEGER               XBEGS  ( MAXSEL )
///           INTEGER               XENDS  ( MAXSEL )
///
///           LOGICAL               ERROR
///           LOGICAL               FOUND
///           LOGICAL               NULL
///
///     C
///     C     Load leapseconds file for time conversion.
///     C
///           CALL FURNSH ( LSKNAM )
///
///     C
///     C     Load EK.
///     C
///           CALL EKLEF  ( EKNAME, HANDLE )
///
///     C
///     C     Setup the query.  Parse the SELECT clause using
///     C     EKPSEL.
///     C
///           QUERY = 'Select IMAGE_NUMBER, IMAGE_ID, '
///          .   //          'PLATFORM_CLOCK, IMAGE_TIME '
///          .   //   'from VIKING_SEDR_DATA '
///          .   //   'where IMAGE_NUMBER < 25850000 '
///          .   //   'order by IMAGE_NUMBER'
///
///           CALL EKPSEL ( QUERY,  N,    XBEGS, XENDS, XTYPES,
///          .              XCLASS, TABS, COLS,  ERROR, ERRMSG )
///
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) ERRMSG
///
///           ELSE
///
///     C
///     C        Submit query to the EK query system.
///     C
///              CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///              IF ( ERROR ) THEN
///
///                 WRITE(*,*) ERRMSG
///
///              ELSE
///
///     C
///     C           Fetch the rows that matched the query.
///     C
///                 DO ROW = 1, NMROWS
///
///     C
///     C              Fetch data from the Ith row.
///     C
///                    WRITE (*,*) ' '
///                    WRITE (*,*) 'ROW = ', ROW
///
///                    DO COLNO = 1, N
///
///     C
///     C                 Fetch the data from the Jth selected
///     C                 column.
///     C
///                       IF ( XCLASS(COLNO) .EQ. 'COL' ) THEN
///
///                          OUTSTR  =  COLS(COLNO)
///                          CALL PREFIX ( '.',         0, OUTSTR )
///                          CALL PREFIX ( TABS(COLNO), 0, OUTSTR )
///                          ITEM = '  ' // OUTSTR // ':'
///
///                       ELSE
///
///                          B  =  XBEGS(COLNO)
///                          E  =  XENDS(COLNO)
///                          ITEM = '  ITEM = ' // QUERY(B:E)
///
///                       END IF
///
///                       IF ( XTYPES(COLNO) .EQ. 'CHR' ) THEN
///
///                          CALL EKGC ( COLNO,  ROW,  1,
///          .                           CDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, CDATA(:RTRIM(CDATA))
///                          END IF
///
///
///                       ELSE IF ( XTYPES(COLNO) .EQ. 'DP' ) THEN
///
///                          CALL EKGD ( COLNO,  ROW,  1,
///          .                           DDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, DDATA
///                          END IF
///
///
///                       ELSE IF ( XTYPES(COLNO) .EQ. 'INT' ) THEN
///
///                          CALL EKGI ( COLNO,  ROW,  1,
///          .                           IDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             WRITE(*,*) ITEM, IDATA
///                          END IF
///
///
///                       ELSE
///     C
///     C                    The item is a time value.  Convert it
///     C                    to UTC for output.
///     C
///                          CALL EKGD ( COLNO,  ROW,  1,
///          .                           TDATA, NULL, FOUND )
///
///                          IF ( NULL ) THEN
///                             WRITE(*,*) ITEM, '<Null>'
///                          ELSE
///                             CALL ET2UTC ( TDATA, 'C', 3, UTCSTR )
///                             WRITE(*,*) ITEM, UTCSTR
///                          END IF
///
///                       END IF
///
///     C
///     C              We're done with the column having index COLNO.
///     C
///                    END DO
///
///     C
///     C           We're done with the row having index ROW.
///     C
///                 END DO
///
///     C
///     C        We either processed the query or had an error.
///     C
///              END IF
///
///     C
///     C     We either parsed the SELECT clause or had an error.
///     C
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      ROW =            1
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25837050
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C09
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 16:50:55.925
///
///      ROW =            2
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25837051
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C10
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 16:51:00.269
///
///      ROW =            3
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25840344
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C11
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 20:56:53.051
///
///      ROW =            4
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25840345
///        VIKING_SEDR_DATA.IMAGE_ID      : 168C12
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 16 20:56:57.395
///
///      ROW =            5
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25843638
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C01
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.88000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 01:02:50.177
///
///      ROW =            6
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25843639
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C02
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.27000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 01:02:54.521
///
///      ROW =            7
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25846934
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C03
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 05:08:56.263
///
///      ROW =            8
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25846935
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C04
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    119.52000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 05:09:00.607
///
///      ROW =            9
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848026
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C05
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:28.424
///
///      ROW =           10
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848030
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C09
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:46.174
///
///      ROW =           11
///        VIKING_SEDR_DATA.IMAGE_NUMBER  :     25848032
///        VIKING_SEDR_DATA.IMAGE_ID      : 169C11
///        VIKING_SEDR_DATA.PLATFORM_CLOCK:    120.14000000000000
///        VIKING_SEDR_DATA.IMAGE_TIME    : 1976 JUN 17 06:30:55.168
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Currently, column names are the only supported expressions.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragment.
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB)
/// ```
pub fn ekpsel(
    ctx: &mut SpiceContext,
    query: &str,
    n: &mut i32,
    xbegs: &mut [i32],
    xends: &mut [i32],
    xtypes: CharArrayMut,
    xclass: CharArrayMut,
    tabs: CharArrayMut,
    cols: CharArrayMut,
    error: &mut bool,
    errmsg: &mut str,
) -> crate::Result<()> {
    EKPSEL(
        query.as_bytes(),
        n,
        xbegs,
        xends,
        xtypes,
        xclass,
        tabs,
        cols,
        error,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKPSEL ( EK, parse SELECT clause )
pub fn EKPSEL(
    QUERY: &[u8],
    N: &mut i32,
    XBEGS: &mut [i32],
    XENDS: &mut [i32],
    XTYPES: CharArrayMut,
    XCLASS: CharArrayMut,
    TABS: CharArrayMut,
    COLS: CharArrayMut,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XBEGS = DummyArrayMut::new(XBEGS, 1..);
    let mut XENDS = DummyArrayMut::new(XENDS, 1..);
    let mut XTYPES = DummyCharArrayMut::new(XTYPES, None, 1..);
    let mut XCLASS = DummyCharArrayMut::new(XCLASS, None, 1..);
    let mut TABS = DummyCharArrayMut::new(TABS, None, 1..);
    let mut COLS = DummyCharArrayMut::new(COLS, None, 1..);
    let mut AKA = [b' '; TNAMSZ as usize];
    let mut COLUMN = [b' '; CNAMSZ as usize];
    let mut EQRYC = [b' '; MAXCLN as usize];
    let mut QTAB = [b' '; TNAMSZ as usize];
    let mut EQRYD = StackArray::<f64, 100>::new(1..=MAXQNM);
    let mut ATTDSC = StackArray::<i32, 6>::new(1..=ADSCSZ);
    let mut COLIDX: i32 = 0;
    let mut EQRYI = ActualArray::<i32>::new(LBCELL..=EQIMIN);
    let mut ERRPTR: i32 = 0;
    let mut TABIDX: i32 = 0;

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
    // Saved values
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
        CHKIN(b"EKPSEL", ctx)?;
    }

    //
    // Initialize the encoded query each time, for safety.
    //
    ZZEKQINI(
        EQIMIN,
        MAXQNM,
        EQRYI.as_slice_mut(),
        &mut EQRYC,
        EQRYD.as_slice_mut(),
        ctx,
    )?;

    //
    // Encode the input query.
    //
    ZZEKENCD(
        QUERY,
        EQRYI.as_slice_mut(),
        &mut EQRYC,
        EQRYD.as_slice_mut(),
        ERROR,
        ERRMSG,
        &mut ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKPSEL", ctx)?;
        return Ok(());
    }

    //
    // Look up the number of SELECT columns.  For each column, look up
    // the parent table, the alias, and the column's name.
    //
    ZZEKREQI(EQRYI.as_slice(), b"NUM_SELECT_COLS", N, ctx)?;

    for I in 1..=*N {
        ZZEKQSEL(
            EQRYI.as_slice(),
            &EQRYC,
            I,
            &mut XBEGS[I],
            &mut XENDS[I],
            &mut QTAB,
            &mut TABIDX,
            &mut COLS[I],
            &mut COLIDX,
            ctx,
        )?;

        //
        // Make the table index to the table name.
        //
        ZZEKQTAB(
            EQRYI.as_slice(),
            &EQRYC,
            TABIDX,
            &mut TABS[I],
            &mut AKA,
            ctx,
        )?;

        //
        // Currently, every expression is a column.
        //
        fstr::assign(XCLASS.get_mut(I), b"COL");

        //
        // Look up the data type of the column.
        //
        EKCII(&TABS[I], COLIDX, &mut COLUMN, ATTDSC.as_slice_mut(), ctx)?;

        fstr::assign(XTYPES.get_mut(I), save.CHRTYP.get(ATTDSC[ATTTYP]));
    }

    CHKOUT(b"EKPSEL", ctx)?;
    Ok(())
}
