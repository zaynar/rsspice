//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
const CNAMSZ: i32 = 32;
const LBCELL: i32 = -5;
const MAXCOL: i32 = 100;

/// EK, find data
///
/// Find E-kernel data that satisfy a set of constraints.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  QUERY      I   Query specifying data to be found.
///  NMROWS     O   Number of matching rows.
///  ERROR      O   Flag indicating whether query parsed correctly.
///  ERRMSG     O   Parse error description.
/// ```
///
/// # Detailed Input
///
/// ```text
///  QUERY    is a character string that specifies a set of EK
///           data to select from those present in currently
///           loaded EK files. The selected data will be
///           retrievable via the EK fetch routines EKGC, EKGD,
///           and EKGI.
///
///           The query consists of four clauses, the third and
///           fourth of which are optional. The general form
///           of a query is
///
///              SELECT <column list>
///              FROM <table list>
///              [WHERE <constraint list>]
///              [ORDER BY <ORDER BY column list>]
///
///           where brackets indicate optional items. The
///           elements of the query shown above are called,
///           respectively, the `SELECT clause', the
///           `FROM clause', the `WHERE clause', and the
///           `ORDER BY clause'. The result of a query may be
///           thought of as a new table, whose columns are those
///           specified in the SELECT clause, whose rows are
///           those satisfying the constraints of the WHERE
///           clause, and whose rows are ordered according to
///           the ORDER BY clause.
///
///           The SELECT clause specifies a list of columns
///           from which data are to be selected. In a simple
///           (non-join) query, these columns must belong to
///           the single table specified in the FROM clause.
///
///           The form of a SELECT clause is
///
///              SELECT <column name> [ ,<column name>...]
///
///           In queries having multiple tables in the FROM
///           clause, column names are ambiguous if they occur
///           in more than one table in the FROM clause. Such
///           column names must be qualified with table
///           identifiers. These identifiers may be the names of
///           the tables to which the columns belong, or table
///           `aliases', names (usually short ones) associated
///           with tables in the FROM clause. Table aliases have
///           duration limited to the execution of the query to
///           which they belong.
///
///           The form of a qualified column name is
///
///              <table name>.<column name>
///
///           or
///
///              <table alias>.<column name>
///
///
///           The FROM clause specifies the tables from which
///           data are to be selected. In simple queries, only
///           one table is listed. In this case the form of
///           the FROM clause is
///
///              FROM <table name>
///
///           In queries involving multiple tables, the form of
///           the FROM clause becomes
///
///              FROM <table name> [<table alias>]
///                   [ , <table name> [<table alias>] ... ]
///
///           The aliases associated with the table names must
///           be distinct and must not be the actual names of
///           loaded EK tables.
///
///           Queries involving multiple tables are called
///           `joins'.
///
///           The meaning of a FROM clause containing multiple
///           tables is that the output is to be a subset of
///           the rows of the Cartesian product of the listed
///           tables. Normally, WHERE clause constraints are
///           supplied to reduce the selected rows to a set of
///           interest.
///
///           The most common example of a join is a query with
///           two tables listed in the FROM clause, and a WHERE
///           clause constraint enforcing equality of members
///           of a column in the first table with members of
///           column in the second table. Such a query is
///           called an `equi-join'. A join in which columns
///           of different tables are related by an inequality
///           is called a `non-equi-join'. Any type of join
///           other than an equi-join may be very slow to
///           evaluate, due to the large number of elements that
///           may be contained in the Cartesian
///           product of the listed tables.
///
///           The WHERE clause lists constraints that must
///           be met by each row satisfying the query. The
///           constraints are specified as a logical combination
///           of relational expressions. The form of the
///           constraint list is
///
///              WHERE <constraint expression>
///
///           where each <constraint expression> consists of one
///           or more simple relational expressions of the form
///
///              <column name> <operator> <RHS symbol>
///
///           where
///
///              <RHS symbol>
///
///           is a column name, a literal value, or the special
///           symbol
///
///              NULL
///
///           and
///
///              <operator>
///
///           is any of
///
///              EQ, GE, GT, LE, LIKE, LT, NE, NOT LIKE, <, <=,
///              =, >, >=, !=, <>
///
///           For comparison with null values, the special
///           syntaxes
///
///              <column name> IS NULL
///              <column name> IS NOT NULL
///
///           are allowed, in addition to the standard
///           comparison syntaxes using the equality or
///           inequality operators.
///
///           The LIKE operator allows comparison of a string
///           value against a template. The template syntax
///           is that allowed by the SPICELIB routine MATCHI.
///           Templates may include literal characters, the
///           wild string marker '*', and the wild character
///           marker '%'. Case is significant in templates.
///
///           Templates are bracketed by quote characters, just
///           as are literal strings.
///
///           The query language also supports the BETWEEN and
///           NOT BETWEEN constructs
///
///              <column> BETWEEN <symbol 1> AND <symbol 2>
///
///              <column> NOT BETWEEN <symbol 1> AND <symbol 2>
///
///           The tokens
///
///              <symbol 1>
///              <symbol 2>
///
///           may be literal values or column names.
///
///           The BETWEEN operator considers values that match
///           the bounds to satisfy the condition: the BETWEEN
///           operator tests for inclusion in the closed interval
///           defined by the bounds.
///
///           In the WHERE clause, simple relational expressions
///           may be combined using the logical operators AND,
///           OR, and NOT, as in the Fortran programming
///           language. Parentheses may be used to enforce a
///           desired order of evaluation of logical expressions.
///
///           The expression syntax is NOT symmetric: literal
///           values must not appear on the left hand side of the
///           operators that apply to them.
///
///           The columns named in a constraint clause must
///           belong to the tables listed in the FROM clause.
///           If the query is a join, qualifying table names or
///           aliases are required wherever their omission would
///           result in ambiguity.
///
///           Data types of the columns or constants used on the
///           right-hand-sides of operators must match the data
///           types of the corresponding columns on the
///           left-hand-sides, except that comparison of integer
///           and double precision quantities is permitted.
///
///           Literal strings used in constraints are always
///           bracketed by quotes. Either single  quotes (')
///           or double quotes (") may be used, but the same
///           quote character must be used to start and end any
///           literal string. Within character string values,
///           quote characters must be doubled in order to be
///           recognized. Case is significant in character
///           except in comparisons using the LIKE and NOT LIKE
///           operators, which ignore case: the expression
///
///              ANIMAL LIKE "*A*"
///
///           would be considered true when ANIMAL takes the
///           value
///
///              "cat"
///
///           Time values are considered to be strings and
///           require bracketing quotes. Currently, the
///           only time values allowed are UTC times in ISO
///           format, UTC times represented in forms accepted by
///           the SPICELIB routine TPARSE, and SCLK strings in
///           NAIF format.
///
///           The ORDER BY clause indicates which columns to
///           use to order the output generated by the query.
///           The columns in the ORDER BY clause define a
///           dictionary ordering, with the first listed column
///           acting as a primary key, the second column acting
///           as a secondary key, and so on.
///
///           For each ORDER BY column, the keywords ASC or DESC
///           may be supplied to indicate whether the items in
///           that column are to be listed in ascending or
///           descending order. Ascending order is the default.
///           The direction in which data items increase is
///           referred to as the `order sense'.
///
///           The ORDER BY clause, if present, must appear
///           last in the query.
///
///           The form of the ORDER BY clause is
///
///              ORDER BY <column name> [<order sense>]
///                       [ ,<column name> [<order sense>]...]
///
///           Rows satisfying the query constraints will be
///           returned so that the entries of the first column
///           specified in the ORDER BY clause will be appear in
///           the order specified by the order sense keyword,
///           which is assumed to be ASC if absent. When entries
///           in the first through Nth ORDER BY column are equal,
///           the entries in the (N+1)st ORDER BY column
///           determine the order of the rows, and so on.
///
///           As in the WHERE clause, column names must be
///           qualified by table names or table aliases where
///           they would otherwise be ambiguous.
///
///           The query language is word-oriented, and some
///           indicate whether the words are reserved. Reserved
///           words must be separated from other words by white
///           space. It is not necessary to use white space
///           to separate words and punctuation characters.
///           The list of reserved words is
///
///              AND
///              BETWEEN
///              BY
///              COLUMN
///              EQ
///              FROM
///              GE
///              GT
///              IS
///              LE
///              LT
///              LIKE
///              NE
///              NOT
///              NULL
///              OR
///              ORDER
///              SELECT
///              WHERE
///
///           The left and right parenthesis characters are also
///           reserved; they may not be used in queries outside
///           of quoted strings.
///
///           Case is not significant in queries, except within
///           literal strings.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NMROWS   is the number of rows that match the query
///           criteria. NMROWS is defined if and only if
///           ERROR is returned .FALSE.
///
///  ERROR    is a logical flag indicating whether the query
///           failed to parse correctly.
///
///  ERRMSG   is a character string that describes EKFIND's
///           diagnosis of a parse error, should one occur.
///           Otherwise, ERRMSG will be returned blank.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include files.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  Most of the exceptions that can occur on a call to
///      EKFIND are caused by errors in the input query. EKFIND
///      attempts to diagnose these via the output error flag and
///      error message, instead of signaling errors. The following
///      classes of errors are detected:
///
///         Scanning errors---these result from badly formed query
///         in which EKFIND could not identify all of the tokens.
///         When these errors occur, EKFIND may be too confused to
///         give a helpful diagnostic message.
///
///         Parsing errors---these result from a badly formed
///         query that EKFIND was able to separate into tokens
///         but that EKFIND determined to be syntactically invalid:
///
///         Name resolution errors---these result from referencing
///         invalid or ambiguous column or table names in a query.
///
///         Time resolution errors---these result from use of time
///         strings that cannot be parsed.
///
///         Semantic errors---these result from a syntactically
///         valid query that violates a limit or a restriction on
///         values used in a query.
///
///
///  Some problems with queries are not trapped by EKFIND but
///  instead cause errors to be signaled. These are listed below.
///
///  2)  If no E-kernels are loaded at the time this routine is called,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If a leapseconds kernel is is not loaded before this routine
///      is called, UTC time values may not be used in queries. If they
///      are, an error is signaled by a routine in the call tree of
///      this routine.
///
///  4)  If an SCLK kernel for the appropriate spacecraft clock has not
///      been loaded before this routine is called, SCLK values for
///      that clock may not be used in queries. If they are, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine operates almost entirely by side effects: it
///  prepares the EK fetch routines to return event data that
///  satisfy the input query. See the EK Required Reading for
///  examples of use of this routine in conjunction with the EK
///  fetch routines.
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
///  1) Perform a query on an EK file that contains a database with
///     the different commands of the Deep Impact spacecraft
///     subsystem, and a table with the subsystem id, the parameter
///     name, and the description of that parameters, ordered by
///     subsystem name. Print the number of records of that table.
///
///     Use the EK kernel below to load the Deep Impact spacecraft
///     subsystem commands dictionary.
///
///        dif_cmdict_128_20050620.bdb
///
///
///     Example code begins here.
///
///
///           PROGRAM EKFIND_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Maximum length of an input query,
///     C     MAXQRY, and the maximum length of literal string
///     C     values, MAXSTR, from eklimit.inc.
///     C
///           INCLUDE 'ekqlimit.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME =
///          .                         'dif_cmdict_128_20050620.bdb' )
///
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 1840 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(MAXQRY)    QUERY
///
///           INTEGER               NMROWS
///
///           LOGICAL               ERROR
///
///     C
///     C     Open an EK file.
///     C
///           CALL FURNSH ( EKNAME )
///
///     C
///     C     The EK file contains the table 'DIF_COMMANDS',
///     C     and that 'DIF_COMMANDS' contains columns named:
///     C
///     C       SUBSYSTEM, COMMAND, PARAMETER_NAME, DESCRIPTION
///     C
///     C     Define a set of constraints to perform a query on all
///     C     loaded EK files (the SELECT clause).
///     C
///           QUERY = 'Select SUBSYSTEM, COMMAND, PARAMETER_NAME, '
///          .   //   'DESCRIPTION from DIF_COMMANDS '
///          .   //   'order by SUBSYSTEM'
///
///     C
///     C     Query the EK system for data rows matching the
///     C     SELECT constraints.
///     C
///           CALL EKFIND ( QUERY, NMROWS, ERROR, ERRMSG )
///
///     C
///     C     Check whether an error occurred while processing the
///     C     SELECT clause. If so, output the error message.
///     C
///           IF ( ERROR ) THEN
///
///              WRITE(*,*) 'SELECT clause error: ', ERRMSG
///
///           ELSE
///
///     C
///     C        If no error, NMROWS contains the number of rows
///     C        matching the constraints specified in the query
///     C        string.
///     C
///              WRITE(*,*) 'Number of matching rows: ', NMROWS
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Number of matching rows:         5798
///
///
///  2) Examples of strings containing syntactically valid queries:
///
///         SELECT COL1 FROM TAB1
///
///         select col1 from tab1 where col1 gt 5
///
///         SELECT COL2 FROM TAB1 WHERE COL2 > 5.7D0 ORDER BY COL2
///
///         SELECT COL2 FROM TAB1 WHERE COL1 != 5
///
///         SELECT COL2 FROM TAB1 WHERE COL1 GE COL2
///
///         SELECT COL1, COL2, COL3 FROM TAB1 ORDER BY COL1
///
///         SELECT COL3 FROM TAB1 WHERE COL5 EQ "ABC"
///
///         SELECT COL3 FROM TAB1 WHERE COL5 = 'ABC'
///
///         SELECT COL3 FROM TAB1 WHERE COL5 LIKE 'A*'
///
///         SELECT COL3 FROM TAB1 WHERE COL5 LIKE 'A%%'
///
///         SELECT COL4 FROM TAB1 WHERE COL4 = '1995 JAN 1 12:38:09.7'
///
///         SELECT COL4 FROM TAB1 WHERE COL4 = "1995 JAN 1 12:38:09.7"
///
///         SELECT COL4 FROM TAB1 WHERE
///         COL4 NE 'GLL SCLK 02724646:67:7:2'
///
///         SELECT COL1 FROM TAB1 WHERE COL1 != NULL
///
///         SELECT COL1 FROM TAB1 WHERE COL1 IS NULL
///
///         SELECT COL1 FROM TAB1 WHERE COL1 IS NOT NULL
///
///         SELECT COL1, COL2, COL3 FROM TAB1
///         WHERE (COL1 BETWEEN 4 AND 6) AND (COL3 NOT LIKE "A%%")
///         ORDER BY COL1, COL3
///
///         SELECT COL4 FROM TAB1
///         WHERE COL4 BETWEEN "1995 JAN 1 12:38" AND
///         "October 23, 1995"
///
///         SELECT COL1, COL2 FROM TAB1 WHERE
///         NOT (    ( ( COL1 <  COL2 ) AND ( COL1 > 5   ) )  OR
///                  ( ( COL1 >= COL2 ) AND ( COL2 <= 10 ) )      )
///
///
///         SELECT T1.COL1, T1.COL2, T2.COL2, T2.COL3
///         FROM TABLE1 T1, TABLE2 T2
///         WHERE T1.COL1 = T2.COL1
///         AND T1.COL2 > 5
///         ORDER BY T1.COL1, T2.COL2
///
///
///  3) Examples of syntactically invalid queries:
///
///         SELECT TIME WHERE TIME
///         LT 1991 JAN 1                      {FROM clause is absent}
///
///         select time from table1 where
///         time lt 1991 jan 1                 {time string is not
///                                             quoted}
///
///         select time from table1
///         where time .lt. '1991 jan 1'       {operator should be lt}
///
///         select cmd from table1
///         where "cmd,6tmchg" != cmd          {value is on left side
///                                             of operator}
///
///         select event_type from table1
///         where event_type eq ""             {quoted string is empty
///                                             ---use " " to indicate
///                                             a blank string}
///
///         select event_type from table1
///         where event_type = "COMMENT"
///         order TIME                         {ORDER BY phrase is
///                                             lacking BY keyword}
///
///         select COL1 from table where
///         where COL1 eq MOC_EVENT            {literal string on
///                                             right-hand-side of
///                                             operator is not quoted}
///
///
///
///      In the following examples, we'll assume that the program
///      calling EKFIND has loaded an EK containing two segments
///      having columns having the following names and attributes:
///
///
///       TABLE1:
///       ==========
///
///         Column name        Data type         Size       Indexed?
///         -----------        ---------         ----       --------
///         EVENT_TYPE         CHARACTER*32      1          YES
///         EVENT_PARAMETERS   CHARACTER*(*)     1          NO
///         COMMENT            CHARACTER*80      VARIABLE   NO
///
///
///       TABLE2:
///       ==========
///
///         Column name        Data type         Size       Indexed?
///         -----------        ---------         ----       --------
///         EVENT_TYPE         CHARACTER*32      1          YES
///         EVENT_PARAMETERS   CHARACTER*80      1          NO
///         COMMENT            CHARACTER*80      VARIABLE   NO
///         COMMAND            CHARACTER*80      1          YES
///
///
///      Then the following queries are semantically invalid:
///
///         SELECT EVENT_PARAMETERS
///         FROM TABLE1
///         WHERE EVENT_DURATION = 7.0         {No column called
///                                             EVENT_DURATION
///                                             is present in a loaded
///                                             EK}
///
///         SELECT COMMENT FROM TABLE2
///         WHERE COMMENT EQ "N/A"             {The COMMENT column does
///                                             not have size 1 and
///                                             therefore cannot be
///                                             referenced in a query}
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A leapseconds kernel must be loaded before this routine may
///      be called, if UTC time values are used in input queries.
///
///  2)  An appropriate SCLK kernel must be loaded before this routine
///      may be called, if SCLK values are used in input queries.
///
///  3)  Data found in response to a query become unavailable
///      when a fast load is initiated via EKIFLD. Any desired
///      fetches of the data must be performed before a fast
///      load or any other operation that modifies the EK scratch
///      area is initiated.
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
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.4, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.0.3, 19-DEC-2001 (NJB)
///
///         $Restrictions section was updated.
///
/// -    SPICELIB Version 1.0.2, 14-JAN-1997 (NJB)
///
///         Syntax descriptions for comparisons using null values have been
///         added. The $Examples section was augmented with sample queries
///         demonstrating use of the IS NULL and IS NOT NULL comparison
///         operators.
///
/// -    SPICELIB Version 1.0.1, 16-AUG-1996 (NJB)
///
///         $Exceptions section of header was updated to indicate that
///         calling this routine while no E-kernels are loaded will cause
///         an error to be signaled. Previous version line was changed
///         from "Beta" to "SPICELIB," and the previous version was
///         corrected to 1.0.0.
///
/// -    SPICELIB Version 1.0.0, 24-OCT-1995 (NJB)
/// ```
pub fn ekfind(
    ctx: &mut SpiceContext,
    query: &str,
    nmrows: &mut i32,
    error: &mut bool,
    errmsg: &mut str,
) -> crate::Result<()> {
    EKFIND(
        query.as_bytes(),
        nmrows,
        error,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKFIND ( EK, find data )
pub fn EKFIND(
    QUERY: &[u8],
    NMROWS: &mut i32,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CHRBUF = [b' '; MAXCLN as usize];
    let mut EQRYC = [b' '; MAXCLN as usize];
    let mut EQRYD = StackArray::<f64, 100>::new(1..=MAXQNM);
    let mut NUMVLS = StackArray::<f64, 100>::new(1..=MAXQNM);
    let mut CHBEGS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut CHENDS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut EQRYI = ActualArray::<i32>::new(LBCELL..=EQIMIN);
    let mut ERRPTR: i32 = 0;
    let mut LXBEGS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut LXENDS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut NTOKEN: i32 = 0;
    let mut TOKENS = ActualArray::<i32>::new(1..=MAXTOK);
    let mut VALUES = ActualArray::<i32>::new(1..=MAXTOK);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Storage limits:
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKFIND", ctx)?;
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
    // Find the tokens in the input query.
    //
    ZZEKSCAN(
        QUERY,
        MAXTOK,
        MAXQNM,
        &mut NTOKEN,
        TOKENS.as_slice_mut(),
        LXBEGS.as_slice_mut(),
        LXENDS.as_slice_mut(),
        VALUES.as_slice_mut(),
        NUMVLS.as_slice_mut(),
        &mut CHRBUF,
        CHBEGS.as_slice_mut(),
        CHENDS.as_slice_mut(),
        ERROR,
        ERRMSG,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKFIND", ctx)?;
        return Ok(());
    }

    //
    // Now parse the query.
    //
    ZZEKPARS(
        QUERY,
        NTOKEN,
        LXBEGS.as_slice(),
        LXENDS.as_slice(),
        TOKENS.as_slice(),
        VALUES.as_slice(),
        NUMVLS.as_slice(),
        &CHRBUF,
        CHBEGS.as_slice(),
        CHENDS.as_slice(),
        EQRYI.as_slice_mut(),
        &mut EQRYC,
        EQRYD.as_slice_mut(),
        ERROR,
        ERRMSG,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKFIND", ctx)?;
        return Ok(());
    }

    //
    // Resolve names.
    //
    ZZEKNRES(
        QUERY,
        EQRYI.as_slice_mut(),
        &EQRYC,
        ERROR,
        ERRMSG,
        &mut ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKFIND", ctx)?;
        return Ok(());
    }

    //
    // Resolve time values, if necessary.
    //
    ZZEKTRES(
        QUERY,
        EQRYI.as_slice_mut(),
        &EQRYC,
        EQRYD.as_slice_mut(),
        ERROR,
        ERRMSG,
        &mut ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKFIND", ctx)?;
        return Ok(());
    }

    //
    // Perform semantic checks.
    //
    ZZEKSEMC(
        QUERY,
        EQRYI.as_slice_mut(),
        &EQRYC,
        ERROR,
        ERRMSG,
        &mut ERRPTR,
        ctx,
    )?;

    if *ERROR {
        CHKOUT(b"EKFIND", ctx)?;
        return Ok(());
    }

    //
    // If we arrived here, the encoded query is ready for execution.
    // Find the data satisfying the constraints.
    //
    EKSRCH(
        EQRYI.as_slice(),
        &EQRYC,
        EQRYD.as_slice(),
        NMROWS,
        ERROR,
        ERRMSG,
        ctx,
    )?;

    CHKOUT(b"EKFIND", ctx)?;
    Ok(())
}
