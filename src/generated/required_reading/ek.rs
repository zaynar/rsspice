//! #  E-Kernel Required Reading
//!
//!  Last revised on 2016 MAR 23 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The SPICE events subsystem and Events Kernel (EK) are used to implement
//!    the sequence component of the Events Kernel (EK/ESQ), and may be used in
//!    any other application where a modest SQL-like database is called for.
//!
//!  
//!
//!
//!  
//! ##  References
//!
//!  NAIF document numbers are shown preceding document titles.
//!
//!  
//!
//! *  1. \[218] KERNEL Required Reading ([kernel.req](crate::required_reading::kernel))
//!
//!  *  2. \[222] SCLK Required Reading ([sclk.req](crate::required_reading::sclk))
//!
//!  *  3. \[225] TIME Required Reading ([time.req](crate::required_reading::time))
//!
//!  *  4. \[278] COMMNT User's Guide ([commnt.ug](crate::raw::commnt.ug))
//!
//!  *  5. \[284] INSPEKT User's Guide ([inspekt.ug](crate::raw::inspekt.ug))
//!
//!  *  6. \[286] DAS Required Reading ([das.req](crate::required_reading::das))
//!
//!  *  7. \[333] Converting and Porting SPICE Data Files ([convert.ug](crate::raw::convert.ug))
//!
//!  *  8. \[A] (No NAIF document number available) SPICE Tutorial chapter: "Events
//! Kernel Notebook Component ENB"
//!
//!     
//! ##  Introduction
//!
//!  The E-kernel ("EK") is the logical component of the SPICE system that
//!    deals with "event" data. In the context of space science missions,
//!    event data may be interpreted as the collection of information
//!    concerning planned or unplanned mission activities or occurrences that
//!    can assist in extracting the full value of the science data returned
//!    from those missions. Event data may also include data used to assist
//!    mission operations. The definition of event data given here is
//!    deliberately broad; selection of appropriate event data must be carried
//!    out on an application-by-application basis. Examples of event data
//!    include, but are not limited to, statements of scientific objectives,
//!    planned or actual sequences of commands executed onboard spacecraft, and
//!    notebook or log entries of scientists or mission operations personnel.
//!
//!  The SPICE E-kernel (EK) subsystem is intended to support convenient
//!    recording, electronic transfer, archival, examination, and manipulation
//!    of event data by human users and software. Because the form, content,
//!    and quantity of event data may vary widely from one mission or
//!    application to next, the EK subsystem emphasizes flexibility in
//!    accommodating event data and imposes few restrictions on the types of
//!    data that can be included within the subsystem.
//!
//!  The EK subsystem includes two separate software mechanisms for storing
//!    and handling event data. One of these is a simple, stand-alone
//!    relational database system. This system includes event data files, SPICE
//!    software that manipulates those files, and documentation. The data files
//!    used by this system are called "sequence E-kernels," "sequence EK
//!    files" or "sequence EKs"; often the qualifier "sequence" is
//!    omitted. SPICE EK software enables sequence EKs to be examined,
//!    interactively or through an application programming interface (API), by
//!    means of a simple, SQL-like query language. The sequence EK file format
//!    and associated software are discussed in detail below.
//!
//!  The second mechanism is an e-mail and web-based software system that
//!    allows users to archive and share text notes and e-mail messages, the
//!    latter of which may optionally include MIME attachments. This is known
//!    as the "experimenter's notebook" or "ENB" system. The ENB system is
//!    documented in reference \[A].
//!
//!  While the EK subsystem allows users to package an almost limitless
//!    variety of event data, the subsystem is designed to support in
//!    particular three of the categories of data listed above:
//!
//!  
//!
//! * Statements of scientific objectives
//!
//!  * Sequences of spacecraft commands
//!
//!  * Notebook or log entries
//!
//!  The EK subsystem is partitioned into three "components," each one
//!    designed to accommodate one of the types of data listed above. The names
//!    of these components are, respectively,
//!
//!  
//!
//! * The Science Plan
//!
//!  * The Sequence Component
//!
//!  * The Experimenter's Notebook ("ENB")
//!
//!     
//! ##  EK subsystem components
//!
//!  Below, we present a high-level description of the components of the EK
//!    subsystem and suggested applications of these components.
//!
//!  
//!
//!
//!  
//! ###  EK science plan component
//!
//!  The purpose of the Science Plan component of the SPICE EK subsystem is
//!    to record high-level descriptions of planned mission activities,
//!    particularly, but not necessarily limited to, those pertaining to
//!    science experiments. Science plan entries should enable the reader to
//!    understand the objectives of mission activities, and where appropriate,
//!    should give a high-level description of how they are carried out. A
//!    description of a mosaic of optical photographs might belong in the
//!    science plan; the commands used to execute the mosaic probably would be
//!    best relegated to the EK sequence component, which is described below.
//!
//!  Depending on mission requirements, either the ENB or Sequence EK system
//!    may be suitable for storing Science Plan data.
//!
//!  
//!
//!
//!  
//! ###  EK sequence component
//!
//!  The sequence component of the SPICE EK subsystem is intended to deal
//!    with event data that fit the relational database paradigm: specifically,
//!    data that can be meaningfully and advantageously represented as a series
//!    of tables, each one of which is characterized by a collection of rows
//!    and columns. This type of organization is likely appropriate when:
//!
//!  
//!
//! * The structure of the data is regular
//!
//!  * The volume of the data is large
//!
//!  * The capability of searching the data rapidly, preferably in random-access
//! fashion, for items of interest is required
//!
//!  * The capability of specifying relational search constraints is required
//!
//!  * The data must be accessible by software as well as by human readers
//!
//!  This last criterion may apply even when the primary means of access to
//!    the data is visual inspection, if the data volume is large enough:
//!    software may be required to enable users to select manageable subsets of
//!    the data to browse on-line or in the form of printed listings.
//!
//!  Data stored in the sequence component of the EK subsystem might
//!    represent sequences of time-tagged "events." Sequences of commands
//!    sent to a spacecraft are an example of such event data. Terse notes
//!    indicating occurrences of geometric events such as equator crossings or
//!    times of closest approach of a spacecraft relative to a target are
//!    another example of suitable data to include in this component.
//!
//!  When event data consist of or include descriptions of state changes of
//!    systems of interest, a sequence EK containing these data could be used
//!    to find the states of the corresponding systems at a given time.
//!
//!  The data comprising an event may correspond to a row in a table, and
//!    attributes of the event could be represented by entries in different
//!    columns within the row. A trivial, fictitious example of this sort of
//!    logical organization is shown in the table below:
//!
//!  
//!
//! ```text
//!              (column 1)   (column 2)           (column 3)
//!  
//!                 TIME       MNEMONIC              EVENT
//!  
//!             +------------------------------------------------------+
//!    (row 1)  | 3987:64:2 | CMD,PWRON  |  Turn camera power on       |
//!             +------------------------------------------------------+
//!    (row 2)  | 3989:01:0 | CMD,FILCLR |  Select CLEAR filter        |
//!             +------------------------------------------------------+
//!    (row 3)  | 4000:01:5 | CMD,SHUTR  |  Shutter photo              |
//!             +------------------------------------------------------+
//!    (row 4)  | 4000:01:5 | COMMENT    |  OPNAV photo #1 complete    |
//!             +------------------------------------------------------+
//!        .                               .
//!        .                               .
//!        .                               .
//!  
//! ```
//!
//!  With regard to such a table, we might wish to construct queries such as:
//!
//!  
//!
//! ```text
//!    "Find the filter selection commands that occurred between
//!    spacecraft clock times 5000:23:0 and 5001:00:0"
//!  
//!    "Find the events containing the word ``camera'' and display them
//!    ordered by mnemonic"
//!  
//!    "Find the last event description starting with the string ``Turn''
//!    prior to the UTC time 1-JAN-1997 12:14:02"
//!  
//!    "Find the times of all the ``Shutter photo'' events"
//! ```
//!
//!  We might want to display the rows satisfying these queries on our
//!    terminals, dump them to a file, or use them to drive a program. All of
//!    these functions are supported by the sequence EK subsystem. Note that
//!    the queries shown above are English paraphrases of the equivalent
//!    expressions in the EK query language.
//!
//!  The functional capabilities described above are provided by files and
//!    software capable of accessing those files. The EK API contains
//!    "writer" software that enables users to create sequence EK files that
//!    contain data organized in a tabular fashion. The data then can be
//!    accessed using "reader" subroutines from the EK API, or interactively
//!    using the EK browsing program INSPEKT.
//!
//!  Sequence EK files are binary files and therefore cannot be read directly
//!    using text editing programs. However, the program INSPEKT can dump any
//!    selected portion of any sequence EK as a text file, using user-specified
//!    formats, so in a sense sequence EK files are more flexible than flat
//!    text files as a repository for event information. By using a
//!    database-style internal data representation rather than a
//!    format-oriented one, they avoid the constraints on their contents that
//!    would be imposed by adoption of fixed file formats.
//!
//!  Sequence EK files may be ported between computer systems having
//!    different internal data formats; the SPICE Toolkit utilities TOBIN and
//!    TOXFR support this function.
//!
//!  Sequence EK files may also have labels and free-form text inserted into
//!    them to assist in clear and complete identification of the files; the
//!    SPICE Toolkit utility COMMNT may be used for this purpose.
//!
//!  A detailed discussion of the functional characteristics of the sequence
//!    component is given below in the chapter titled "Sequence EK Concepts."
//!
//!  
//!
//!
//!  
//! ##  EK experimenter's notebook component
//!
//!  The experimenter's notebook component of the EK subsystem is primarily
//!    intended to be a mechanism for recording after-the-fact observations,
//!    particularly of anomalies or other unplanned events. Notes of general
//!    interest to scientists may also be appropriate.
//!
//!  More generally, the experimenter's notebook component may include any EK
//!    data that don't fit into the other two components. For example, if the
//!    available, human-readable command sequence data are extremely small in
//!    volume, it may be more practical to include them in the experimenter's
//!    notebook than to insert them into a binary sequence EK.
//!
//!  
//!
//!
//!  
//! #  Sequence EK Concepts
//!
//!  
//!
//!
//!  
//! ##  Relational database functionality
//!
//!  The sequence EK subsystem is a simple, stand-alone relational database
//!    system, with a few modifications to better adapt the system to
//!    EK-specific applications.
//!
//!  The sequence EK subsystem provides an application programming interface
//!    (API) for creating, modifying, reading, summarizing, and annotating
//!    sequence EK files. In particular, the API supports reading using a
//!    query-and-fetch mechanism: an application passes a request for data
//!    called a "query" to the EK subsystem, then retrieves the data using a
//!    suite of API routines. Queries are expressed in a simple language that
//!    closely resembles the standard relational database query language SQL.
//!
//!  The sequence EK query capability is also provided using the SPICE
//!    interactive browsing utility INSPEKT. However, INSPEKT does not support
//!    any sequence EK writing functionality.
//!
//!  The functionality of sequence EK software is almost completely
//!    independent of its intended application as a system for handling event
//!    data. One could think of the software system not as an "event kernel"
//!    but simply as a "database kernel," and in fact the term "database
//!    kernel" and the acronym "DBK" have been used in some SPICE
//!    documentation. However, since the "EK" prefix has already been widely
//!    used in naming routines belonging to the EK API, we'll stick with the
//!    name "EK" in our discussion.
//!
//!  
//!
//!
//!  
//! ##  The sequence EK data model
//!
//!  Below we discuss the logical organization of data in the sequence EK
//!    subsystem.
//!
//!  
//!
//!
//!  
//! ###  Tables
//!
//!  A sequence EK database is logically a set of tables. Each table is made
//!    up of rows and columns. Tables are "rectangular": there is the same
//!    number of rows in each column. The intersection of a row and column is
//!    called a "column entry." All columns within a table have the same
//!    number of entries---one per row.
//!
//!  The sequence EK data model diverges slightly from the relational model
//!    in that columns are allowed to have arrays as entries. We call such
//!    columns "array-valued" or "vector-valued." When a column entry is an
//!    array, we call the components of the array "column entry elements" or
//!    "elements" if the context is clear.
//!
//!  
//!
//!
//!  
//! ###  Column attributes
//!
//!  Each column has a number of attributes that characterize the data it
//!    contains. These attributes apply uniformly to the entries within a
//!    column:
//!
//!  
//!
//! * Data type
//!
//!  *  Column entries may have INTEGER, CHARACTER, DOUBLE PRECISION, or TIME data
//! type.
//!
//!  *  The TIME data type is used to represent epochs. As such, the TIME type
//! plays the same role as the DATE datatype in the SQL language.
//!
//!  * Dimension
//!
//!  *  Column entries may have fixed or variable dimension. When a column has
//! fixed-dimension entries of size 1, the column is said to be
//! "scalar-valued." Otherwise, the column is said to be "array valued,"
//! "vector valued," or, if the dimension is variable, "variable-sized."
//! There is no limit imposed by the EK subsystem on the dimension of a
//! column's entries.
//!
//!  *  Array-valued columns may not be referenced in query constraints.
//!
//!  * String length (applies only to character columns)
//!
//!  *  Array-valued columns having entries of CHARACTER type have a fixed, maximum
//! string length associated with them. Scalar-valued CHARACTER columns may
//! have variable-length strings as entries. For all CHARACTER columns, the
//! maximum allowed string length is 1024 characters.
//!
//!  * Whether null values are allowed
//!
//!  *  At the time an EK segment is created, each column declaration indicates
//! whether the column may contain null values. In columns that allow null
//! entries, entries may be designated as null when segments are written.
//!
//!     
//! ##  The EK query language
//!
//!  
//!
//!
//!  
//! ###  Query syntax
//!
//!  An EK query is a character string that specifies a set of EK data to
//!    select from those present in currently loaded EK files. A query
//!    specifies columns and tables of interest, and optionally specifies
//!    constraints that entries in the tables' rows must match. We refer to
//!    rows that satisfy the constraints as "matching rows."
//!
//!  The selected data will be retrievable using the EK fetch routines [EKGC](crate::raw::ekgc),
//!    [EKGD](crate::raw::ekgd), and [EKGI](crate::raw::ekgi).
//!
//!  The query consists of four clauses, the third and fourth of which are
//!    optional. The general form of a query is
//!
//!  
//!
//! ```text
//!    SELECT <column list>
//!    FROM <table list>
//!    [WHERE <constraint list>]
//!    [ORDER BY <ORDER BY column list>]
//! ```
//!
//!  where brackets indicate optional items. The elements of the query shown
//!    above are called, respectively, the "SELECT clause," the "FROM
//!    clause," the "WHERE clause," and the "ORDER BY clause." The result
//!    of a query may be thought of as a new table, whose columns are those
//!    specified in the SELECT clause and whose rows are those satisfying the
//!    constraints of the WHERE clause, ordered according to the ORDER BY
//!    clause.
//!
//!  
//!
//!
//!  
//! ###  The SELECT clause
//!
//!  The SELECT clause specifies a list of columns from which to select data.
//!    In a simple (non-join) query, these columns must belong to the single
//!    table specified in the FROM clause.
//!
//!  The form of a SELECT clause is
//!
//!  
//!
//! ```text
//!    SELECT <column name> [ , <column name>...]
//! ```
//!
//!  In queries having multiple tables in the FROM clause (see below), column
//!    names are ambiguous if they occur in more than one table in the FROM
//!    clause. Such column names must be qualified with table identifiers.
//!    These identifiers may be the names of the tables to which the columns
//!    belong, or table "aliases," names (usually short ones) associated with
//!    tables in the FROM clause. Table aliases have duration limited to the
//!    execution of the query to which they belong.
//!
//!  The form of a qualified column name is
//!
//!  
//!
//! ```text
//!    <table name>.<column name>
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!    <table alias>.<column name>
//! ```
//!
//!  Columns named in the SELECT clause must be present in some loaded EK for
//!    the query to be semantically valid.
//!
//!  
//!
//!
//!  
//! ###  The FROM clause
//!
//!  The FROM clause specifies the tables from which to select data. In
//!    simple queries, only one table is listed. In this case the form of the
//!    FROM clause is
//!
//!  
//!
//! ```text
//!    FROM <table name>
//! ```
//!
//!  In queries involving multiple tables, the form of the FROM clause
//!    becomes
//!
//!  
//!
//! ```text
//!    FROM <table name> [<table alias>]
//!         [ , <table name> [<table alias>] ... ]
//! ```
//!
//!  The aliases associated with the table names must be distinct and must
//!    not be the actual names of loaded EK tables.
//!
//!  Queries involving multiple tables are called "joins."
//!
//!  The meaning of a FROM clause containing multiple tables is that the
//!    output is to be a subset of the rows of the Cartesian product of the
//!    listed tables. Normally, WHERE clause constraints are supplied to reduce
//!    the selected rows to a set of interest.
//!
//!  The most common example of a join is a query with two tables listed in
//!    the FROM clause, and a WHERE clause constraint enforcing equality of
//!    members of a column in the first table with members of column in the
//!    second table. Such a query is called an "equi-join." A join in which
//!    columns of different tables are related by an inequality is called a
//!    "non-equi-join." Any type of join other than an equi-join may be very
//!    slow to evaluate, due to the large number of elements that may be
//!    contained in the Cartesian product of the listed tables.
//!
//!  
//!
//!
//!  
//! ###  The WHERE clause
//!
//!  The WHERE clause lists constraints that must be met by each row
//!    satisfying the query. The constraints are specified as a logical
//!    combination of relational expressions. The form of the constraint list
//!    is
//!
//!  
//!
//! ```text
//!    WHERE <constraint expression>
//! ```
//!
//!  where each \<constraint expression> consists of one or more simple
//!    relational expressions of the form
//!
//!  
//!
//! ```text
//!    <column name> <operator> <RHS symbol>
//! ```
//!
//!  Here
//!
//!  
//!
//! ```text
//!    <RHS symbol>
//! ```
//!
//!  is a column name, a literal value, or the special symbol
//!
//!  
//!
//! ```text
//!    NULL
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!    <operator>
//! ```
//!
//!  is any of
//!
//!  
//!
//! ```text
//!    EQ, GE, GT, LE, LIKE, LT, NE, NOT LIKE, <, <=, =, >, >=, !=, <>
//! ```
//!
//!  For comparison with null values, the special expressions
//!
//!  
//!
//! ```text
//!    <column name> IS NULL
//!    <column name> IS NOT NULL
//! ```
//!
//!  are allowed.
//!
//!  The LIKE operator allows comparison of a string value against a
//!    template. The template syntax is that allowed by the CSPICE routine
//!    [MATCHI](crate::raw::matchi). Templates may include literal characters, the wild string marker
//!    '*', and the wild character marker '%'. Case is significant in
//!    templates.
//!
//!  Templates are bracketed by quote characters, just as are literal
//!    strings.
//!
//!  The query language also supports the BETWEEN and NOT BETWEEN constructs
//!
//!  
//!
//! ```text
//!    <column> BETWEEN <symbol 1> AND <symbol 2>
//!  
//!    <column> NOT BETWEEN <symbol 1> AND <symbol 2>
//! ```
//!
//!  The tokens
//!
//!  
//!
//! ```text
//!    <symbol 1>
//!    <symbol 2>
//! ```
//!
//!  may be literal values or column names.
//!
//!  The BETWEEN operator considers values that match the bounds to satisfy
//!    the condition: the BETWEEN operator tests for inclusion in the closed
//!    interval defined by the bounds.
//!
//!  The order of the bounds doesn't matter: the bounds are considered to
//!    define the interval from the smaller bound to the larger.
//!
//!  In the WHERE clause, simple relational expressions may be combined using
//!    the logical operators AND, OR, and NOT, as in the Fortran programming
//!    language. Parentheses may be used to enforce a desired order of
//!    evaluation of logical expressions.
//!
//!  The expression syntax is NOT symmetric: literal values must not appear
//!    on the left hand side of the operators that apply to them.
//!
//!  Data types of the columns or constants used on the right-hand-sides of
//!    operators must match the data types of the corresponding columns on the
//!    left-hand-sides, except that comparison of integer and double precision
//!    quantities is permitted.
//!
//!  The columns named in a WHERE clause must belong to the tables listed in
//!    the FROM clause. If the query is a join, qualifying table names or
//!    aliases are required wherever their omission would result in ambiguity.
//!
//!  Columns referenced in a WHERE clause must be scalar-valued.
//!
//!  
//!
//!
//!  
//! ###  The ORDER BY Clause
//!
//!  The "ORDER BY" clause indicates which columns to use to order the
//!    output generated by the query. The columns in the order-by clause define
//!    a dictionary ordering, with the first listed column acting as a primary
//!    key, the second column acting as a secondary key, and so on.
//!
//!  For each ORDER BY column, the keywords ASC or DESC may be supplied to
//!    indicate whether the items in that column are to be listed in ascending
//!    or descending order. Ascending order is the default. The direction in
//!    which data items increase is referred to as the "order sense."
//!
//!  The ORDER BY clause, if present, must appear last in the query.
//!
//!  The form of the ORDER BY clause is
//!
//!  
//!
//! ```text
//!    ORDER BY <column name> [<order sense>]
//!             [ ,<column name> [<order sense>]...]
//! ```
//!
//!  Rows satisfying the query constraints will be returned so that the
//!    entries of the first column specified in the ORDER BY clause will appear
//!    in the order specified by the order sense keyword, which is assumed to
//!    be ASC if absent. When entries in the first through Nth ORDER BY column
//!    are equal, the entries in the (N+1)st ORDER BY column determine the
//!    order of the rows, and so on.
//!
//!  As in the WHERE clause, ORDER BY column names must be qualified by table
//!    names or table aliases where they would otherwise be ambiguous.
//!
//!  In order for a column to be eligible to be referenced in an ORDER BY
//!    clause, the column must scalar valued.
//!
//!  
//!
//!
//!  
//! ###  Case sensitivity
//!
//!  Case is not significant in queries, except within literal strings. For
//!    these case sensitivity depends on the relational operators applied to
//!    the strings. All comparison operators other than the LIKE operator are
//!    case sensitive: for example, the strings
//!
//!  
//!
//! ```text
//!    "And"
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!    "and"
//! ```
//!
//!  are not considered to be equal. On the other hand, the expression
//!
//!  
//!
//! ```text
//!    ANIMAL LIKE "*A*"
//! ```
//!
//!  would be considered true when ANIMAL takes the value
//!
//!  
//!
//! ```text
//!    "cat"
//! ```
//!
//!  Case is not significant in time values.
//!
//!  
//!
//!
//!  
//! ###  White space
//!
//!  The blank is the only character considered to be a white space character
//!    in the EK query syntax. In particular, tabs are not treated as white
//!    space characters.
//!
//!  Within string constants, leading or embedded white space is significant.
//!    Elsewhere, any string of one or more consecutive blanks is interpreted
//!    as a single blank.
//!
//!  White space is required to separate alphanumeric tokens, such as
//!
//!  
//!
//! ```text
//!    SELECT
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!     LT
//! ```
//!
//!  White space may be omitted between special characters and alphanumeric
//!    tokens, such as
//!
//!  
//!
//! ```text
//!    )
//! ```
//!
//!  and
//!
//!  
//!
//! ```text
//!    WHERE
//! ```
//!
//!     
//! ###  Numeric values
//!
//!  Any numeric format accepted by the SPICELIB routine [PRSDP](crate::raw::prsdp) may be used to
//!    represent numeric values in relational expressions.
//!
//!  The equality operator EQ indicates a test for exact equality. Care must
//!    be taken in testing double precision column entries for equality with a
//!    specified value; round-off errors may cause such tests to fail
//!    unexpectedly.
//!
//!  
//!
//!
//!  
//! ###  String values
//!
//!  Literal strings used in the right hand side of relational expressions,
//!    such as the string
//!
//!  
//!
//! ```text
//!    SSI_EVENT
//! ```
//!
//!  in the query
//!
//!  
//!
//! ```text
//!    * where event_type eq "SSI_EVENT"
//! ```
//!
//!  are always bracketed by quotation marks. Either single or double quotes
//!    may be used, as long as the string is started and terminated with the
//!    same character. Within character string values, quote characters must be
//!    doubled in order to be recognized.
//!
//!  
//!
//!
//!  
//! ###  Time values
//!
//!  Time values are considered to be strings and require bracketing quotes.
//!    Either single or double quotes are allowed, as long as the quote
//!    characters match. The allowed time values are strings accepted by the
//!    SPICELIB routine [STR2ET](crate::raw::str2et), and SCLK strings in SPICE format.
//!
//!  When SCLK strings are used, they must be prefixed by a substring
//!    indicating the name of the clock, followed by the token SCLK. For
//!    example:
//!
//!  
//!
//! ```text
//!    MGS SCLK 2400001.125
//! ```
//!
//!  Time values specified in queries are always converted to barycentric
//!    dynamical time (TDB) before comparisons with column entries are
//!    performed. Therefore, programs using the EK subsystem should load a
//!    leapseconds kernel and any appropriate SCLK kernels before attempting to
//!    issue queries involving time values to the EK subsystem. See \[222] and
//!    \[225] for further information on time conversions.
//!
//!  As with double precision values, time values cannot generally be
//!    reliably tested for exact equality with column entries. It's usually
//!    better to test whether a time column entry is in a desired range than to
//!    test whether it's equal to a specific value.
//!
//!  
//!
//!
//!  
//! ###  Null values
//!
//!  The symbol
//!
//!  
//!
//! ```text
//!    NULL
//! ```
//!
//!  may be used on the right-hand-side of relational expression in which the
//!    column named on the left-hand of the expression allows null values, when
//!    the relational operators are either of
//!
//!  
//!
//! ```text
//!    IS NULL
//!    IS NOT NULL
//! ```
//!
//!  The case of the letters in the symbol "NULL" is not significant. The
//!    symbol is written without quotes.
//!
//!  
//!
//!
//!  
//! ###  Reserved Words
//!
//!  The query language contains the following reserved words:
//!
//!  
//!
//! ```text
//!    ALL
//!    AND
//!    ASC
//!    AVG
//!    BETWEEN
//!    BY
//!    COUNT
//!    DESC
//!    DISTINCT
//!    EQ
//!    FROM
//!    GE
//!    GROUP
//!    GT
//!    HAVING
//!    IS
//!    LE
//!    LIKE
//!    LT
//!    MAX
//!    MIN
//!    NE
//!    NOT
//!    NULL
//!    OR
//!    ORDER
//!    SELECT
//!    SUM
//!    WHERE
//! ```
//!
//!  Some of the above are not currently used but are reserved for upward
//!    compatibility.
//!
//!  Reserved words must be separated from other words in queries by white
//!    space.
//!
//!  
//!
//!
//!  
//! ###  Query grammar
//!
//!  A BNF representation of the sequence EK query grammar is as shown:
//!
//!  
//!
//! ```text
//!    <QUERY>                 =>    <SELECT clause> <FROM clause>
//!                                  <WHERE clause> <ORDER BY clause>
//!  
//!    <SELECT clause>         =>    SELECT <select list>
//!  
//!    <select list>           =>    <column entry>
//!                                | <select list>, <column entry>
//!  
//!    <column entry>          =>    <table name>.<column name>
//!                                | <column name>
//!  
//!    <FROM clause>           =>    FROM <table name list>
//!  
//!    <table name list>       =>    <table entry>
//!                                | <table name list>, <table entry>
//!  
//!    <table entry>           =>    <table name>
//!                                | <table name> <table alias>
//!  
//!    <WHERE clause>          =>    WHERE <relational expression>
//!                                | <NIL>
//!  
//!  
//!    <relational expression>  =>   <simple expression>
//!  
//!                                | <NULL value expression>
//!  
//!                                | NOT <relational expression>
//!  
//!                                |   ( <relational expression> )
//!  
//!                                |     <relational expression>
//!                                  AND <relational expression>
//!  
//!                                |     <relational expression>
//!                                  OR  <relational expression>
//!  
//!  
//!    <simple expression>      =>   <LHS> <operator> <RHS>
//!  
//!                                | <LHS> BETWEEN     <RHS> AND <RHS>
//!  
//!                                | <LHS> NOT BETWEEN <RHS> AND <RHS>
//!  
//!  
//!    <NULL value expression>  =>   <LHS> <Null operator> NULL
//!  
//!  
//!    <LHS>                    =>   <name>
//!  
//!  
//!    <RHS>                    =>   <name>
//!                                | <value>
//!  
//!  
//!    <name>                   =>   <identifier> . <identifier>
//!                                | <identifier>
//!  
//!  
//!    <operator>               =>   EQ
//!                                | GE
//!                                | GT
//!                                | LE
//!                                | LT
//!                                | NE
//!                                | LIKE
//!                                | NOT LIKE
//!                                | =
//!                                | >=
//!                                | >
//!                                | <=
//!                                | <
//!                                | !=
//!                                | <>
//!  
//!  
//!    <NULL operator>         =>    IS
//!                                | IS NOT
//!                                | EQ
//!                                | NE
//!                                | =
//!                                | !=
//!                                | <>
//!  
//!  
//!    <value>                 =>    <character value>
//!                                | <d.p. value>
//!                                | <integer value>
//!  
//!  
//!    <ORDER BY clause>       =>    ORDER BY <order-by list>
//!                                | <NIL>
//!  
//!    <order-by list>         =>    <order-by column entry>
//!                                | <order-by list>,
//!                                  <order-by column entry>
//!  
//!    <order-by column entry> =>    <column entry> <order>
//!                                | <column entry>
//!  
//!    <order>                 =>    ASC
//!                                | DESC
//! ```
//!
//!     
//! ###  Examples of syntactically valid queries
//!
//!  Below is a collection of examples of strings containing syntactically
//!    valid queries.
//!
//!  The column names referenced in the queries are used as examples and are
//!    not meant to suggest that columns having those names will be present in
//!    any particular EKs.
//!
//!  
//!
//! ```text
//!    SELECT COL1 FROM TAB1
//!  
//!    select col1 from tab1 where col1 gt 5
//!  
//!    SELECT COL2 FROM TAB1 WHERE COL2 > 5.7 ORDER BY COL2
//!  
//!    SELECT COL2 FROM TAB1 WHERE COL1 != 5
//!  
//!    SELECT COL2 FROM TAB1 WHERE COL1 GE COL2
//!  
//!    SELECT COL1, COL2, COL3 FROM TAB1 ORDER BY COL1
//!  
//!    SELECT COL3 FROM TAB1 WHERE COL5 EQ "ABC"
//!  
//!    SELECT COL3 FROM TAB1 WHERE COL5 = "ABC"
//!  
//!    SELECT COL3 FROM TAB1 WHERE COL5 LIKE 'A*'
//!  
//!    SELECT COL3 FROM TAB1 WHERE COL5 LIKE 'A%%'
//!  
//!    SELECT COL4 FROM TAB1 WHERE COL4 = '1995 JAN 1 12:38:09.7'
//!  
//!    SELECT COL4 FROM TAB1 WHERE COL4 = "1995 JAN 1 12:38:09.7"
//!  
//!    SELECT COL4 FROM TAB1 WHERE
//!    COL4 NE 'GLL SCLK 02724646:67:7:2'
//!  
//!    SELECT COL1 FROM TAB1 WHERE COL1 != NULL
//!  
//!    SELECT COL1 FROM TAB1 WHERE COL1 IS NULL
//!  
//!    SELECT COL1 FROM TAB1 WHERE COL1 IS NOT NULL
//!  
//!    SELECT COL1, COL2, COL3 FROM TAB1
//!    WHERE (COL1 BETWEEN 4 AND 6) AND (COL3 NOT LIKE "A%%")
//!    ORDER BY COL1, COL3
//!  
//!    SELECT COL4 FROM TAB1
//!    WHERE COL4 BETWEEN "1995 JAN 1 12:38" AND
//!    "October 23, 1995"
//!  
//!    SELECT COL1, COL2 FROM TAB1 WHERE
//!    NOT (    ( ( COL1 <  COL2 ) AND ( COL1 > 5   ) )  OR
//!             ( ( COL1 >= COL2 ) AND ( COL2 <= 10 ) )      )
//!  
//!  
//!    SELECT T1.COL1, T1.COL2, T2.COL2, T2.COL3
//!    FROM TABLE1 T1, TABLE2 T2
//!    WHERE T1.COL1 = T2.COL1
//!    AND T1.COL2 > 5
//!    ORDER BY T1.COL1, T2.COL2
//! ```
//!
//!     
//! ###  Examples of syntactically invalid queries
//!
//!  
//!
//! ```text
//!    SELECT TIME WHERE TIME
//!    LT 1991 JAN 1                      {FROM clause is absent}
//!  
//!    select time from table1 where
//!    time lt 1991 jan 1                 {time string is not
//!                                        quoted}
//!  
//!    select time from table1
//!    where time .lt. '1991 jan 1'       {operator should be lt}
//!  
//!    select cmd from table1
//!    where "cmd,6tmchg" != cmd          {value is on left side
//!                                        of operator}
//!  
//!    select event_type from table1
//!    where event_type eq ""             {quoted string is empty
//!                                        ---use " " to indicate
//!                                        a blank string}
//!  
//!    select event_type from table1
//!    where event_type = "COMMENT"
//!    order TIME                         {ORDER BY phrase is
//!                                        lacking BY keyword}
//!  
//!    select COL1 from table
//!    where COL1 eq MOC_EVENT            {literal string on
//!                                        right-hand-side of
//!                                        operator is not quoted}
//! ```
//!
//!     
//! ###  Examples of semantically invalid queries
//!
//!  In the following examples, we'll assume that an application program has
//!    loaded a sequence EK containing two segments containing columns having
//!    the following names and attributes:
//!
//!  
//!
//! ```text
//!    TABLE1:
//!    ==========
//!  
//!      Column name        Data type         Size       Indexed?
//!      -----------        ---------         ----       --------
//!      EVENT_TYPE         CHARACTER*32      1          YES
//!      EVENT_PARAMETERS   CHARACTER*(*)     1          NO
//!      COMMENT            CHARACTER*80      VARIABLE   NO
//!  
//!  
//!    TABLE2:
//!    ==========
//!  
//!      Column name        Data type         Size       Indexed?
//!      -----------        ---------         ----       --------
//!      EVENT_TYPE         CHARACTER*32      1          YES
//!      EVENT_PARAMETERS   CHARACTER*80      1          NO
//!      COMMENT            CHARACTER*80      VARIABLE   NO
//!      COMMAND            CHARACTER*80      1          YES
//! ```
//!
//!  Then the following queries are semantically invalid:
//!
//!  
//!
//! ```text
//!    SELECT EVENT_PARAMETERS
//!    FROM TABLE1
//!    WHERE EVENT_DURATION = 7.0         {No column called
//!                                        EVENT_DURATION
//!                                        is present in a loaded
//!                                        EK}
//!  
//!    SELECT COMMENT FROM TABLE2
//!    WHERE COMMENT EQ "N/A"             {The COMMENT column does
//!                                        not have size 1 and
//!                                        therefore cannot be
//!                                        referenced in a query}
//!  
//! ```
//!
//!     
//! ##  Sequence EK Files
//!
//!  Sequence EKs are binary files conforming to the SPICE DAS architecture,
//!    which is described in the DAS Required Reading document, [das.req](crate::required_reading::das). The
//!    SPICE file identification word occupying the first eight bytes of a
//!    properly created binary sequence EK file is "DAS/EK ". For more
//!    information on SPICE identification words refer to the Kernel Required
//!    Reading document, [kernel.req](crate::required_reading::kernel). Most users will not need to understand the
//!    details of the structure of these files.
//!
//!  
//!
//!
//!  
//! ###  Segments
//!
//!  Sequence EK files contain data organized in a series of logical tables
//!    called "segments." This organization is logical, not physical---the
//!    physical layout of the data in the file is transparent to applications
//!    that access sequence EKs using the EK API.
//!
//!  Each segment contains data belonging to one EK table. A sequence EK file
//!    may contain multiple segments for one or more distinct tables. Segments
//!    for a table may be distributed across multiple EK files.
//!
//!  Spreading data for a table across multiple segments has no affect on
//!    query interpretation. However, performance degradation may result if a
//!    sequence EK file contains a very large number of segments.
//!
//!  
//!
//!
//!  
//! ###  The comment area
//!
//!  Because sequence EK files are based on the DAS architecture, they
//!    inherit the DAS "comment area" feature; this allows sequence EKs to
//!    store free-form text comments. Comments may be labels or other
//!    descriptive text that fully identifies the file and indicates its
//!    intended purpose.
//!
//!  The contents of the comment area must be printable text. The comment
//!    area is line-oriented; text inserted into the comment area can be
//!    retrieved with the original line breaks preserved. It is recommended
//!    that text to be inserted into the comment area have no lines exceeding
//!    80 characters in length.
//!
//!  See the section "Sequence EK tools" for information on the SPICE
//!    Toolkit utilities that access the comment area.
//!
//!  
//!
//!
//!  
//! ##  Sequence EK tools
//!
//!  The SPICE Toolkit includes programs that may be used to create,
//!    summarize, or browse EK files. These are summarized below.
//!
//!  
//!
//!
//!  
//! ###  INSPEKT
//!
//!  INSPEKT is an interactive program for browsing binary sequence EK files.
//!    INSPEKT presents a user interface similar to that of an interactive
//!    database program: a user can "inspect" one or more binary EK files by
//!    issuing queries in a SQL-like language; in response to each query,
//!    INSPEKT displays event data that satisfy the query. INSPEKT provides
//!    users with many options for formatting the output produced in response
//!    to queries.
//!
//!  INSPEKT has an extensive, hyper-text style on-line help facility, and
//!    also has a detailed user's guide available as a paper document \[284].
//!
//!  
//!
//!
//!  
//! ###  COMMNT
//!
//!  COMMNT is an interactive, menu-driven program that allows users to
//!    manipulate the comment area of binary EK, SPK, CK, and PCK files. The
//!    supported operations are:
//!
//!  
//!
//! * Add or append comments to the comment area.
//!
//!  * Delete comments from the comment area.
//!
//!  * Extract comments from the comment area to a text file.
//!
//!  * Read the comment area.
//!
//!  COMMNT's user's guide, [commnt.ug](crate::raw::commnt.ug), is NAIF Document \[278].
//!
//!  
//!
//!
//!  
//! ###  TOXFR and TOBIN
//!
//!  Since sequence EKs are instances of DAS files, the DAS file transfer
//!    mechanisms apply. The SPICE utilities TOXFR and TOBIN may be used to
//!    convert binary sequence EKs to ASCII transfer format and back to binary
//!    format. See the CONVERT user's guide, [convert.ug](crate::raw::convert.ug), \[333] for details.
//!
//!  
//!
//!
//!  
//! ###  SPACIT
//!
//!  The SPICE utility SPACIT may also be used for converting sequence EKs
//!    between binary and transfer formats. SPACIT provides a rudimentary EK
//!    segment summary capability; however, INSPEKT is typically required to
//!    interactively extract useful information from a sequence EK.
//!
//!  
//!
//!
//!  
//! #  Reading sequence EKs
//!
//!  
//!
//!
//!  
//! ##  Loading and unloading sequence EKs
//!
//!  In order for a program to query one or more sequence EK files, the files
//!    must first be made available to the EK subsystem. This process is called
//!    "loading" the files. Loading EK files is normally accomplished by
//!    calling the generic SPICELIB kernel loader [FURNSH](crate::raw::furnsh):
//!
//!  
//!
//! ```text
//!    CALL FURNSH ( <fname> )               {Load SPICE kernel}
//! ```
//!
//!  A limited number of EK files may be loaded at any one time. The current
//!    maximum limit is 20 files.
//!
//!  The inverse routine corresponding to [FURNSH](crate::raw::furnsh) is [UNLOAD](crate::raw::unload). UNLOAD removes a
//!    loaded kernel from the SPICE system: the file is closed, and data
//!    structures referring to the file are updated to reflect the absence of
//!    the file.
//!
//!  See \[218] for further information on [FURNSH](crate::raw::furnsh) and [UNLOAD](crate::raw::unload).
//!
//!  Before queries may be processed, any supplementary kernels required for
//!    time conversion should be loaded. To enable use of UTC times in queries,
//!    a leapseconds kernel is required. To enable use of SCLK values in
//!    queries, an SCLK kernel for the appropriate spacecraft clock must be
//!    loaded.
//!
//!  All of the EK files loaded at any one time must have consistent table
//!    attributes: any two tables having the same name must have the same
//!    attributes, even if the tables belong to different files.
//!
//!  Unlike the SPK subsystem, the EK subsystem supports no prioritization
//!    scheme for loaded kernels: no kernel supersedes another. Rather, all
//!    rows of all loaded EKs are considered during query processing.
//!
//!  
//!
//!
//!  
//! ##  Query-and-fetch interface
//!
//!  The sequence EK subsystem's "query and fetch" capability is the
//!    principal high-level access mechanism for reading EK data. There are two
//!    steps to retrieving data using this mechanism:
//!
//!  
//!
//! *  1. Specify the data of interest by issuing a query.
//!
//!  *  2. Fetch the data that satisfy the query.
//!
//!  Data comprising the query results may be fetched in random order.
//!
//!  
//!
//!
//!  
//! ###  Issuing queries
//!
//!  To issue a query to the sequence EK subsystem, use [EKFIND](crate::raw::ekfind):
//!
//!  
//!
//! ```text
//!    CALL EKFIND ( <query>, NMROWS, ERROR, ERRMSG )    {Find rows
//!                                                       that satisfy
//!                                                       query        }
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  QUERY
//!
//!
//!  is a string containing an EK query. See the section "The EK query
//! language" above for a complete description of the language.
//!
//!  *  NMROWS
//!
//!
//!  is the number of "matching rows": rows satisfying the query
//! constraints, if any. NMROWS has a valid value only if the query
//! successfully executed.
//!
//!  *  ERROR
//!
//!
//!  is a logical flag indicating whether the query executed successfully.
//! If the input query is incorrect, ERROR will return true.
//!
//!  *  ERRMSG
//!
//!
//!  is a string containing an error diagnosis. ERRMSG has a valid value
//! only if when ERROR is true.
//!
//!  Query errors fall into a few categories:
//!
//!  
//!
//! * Scanning errors---these result from badly formed query in which [EKFIND](crate::raw::ekfind)
//! could not identify all of the tokens. When these errors occur, [EKFIND](crate::raw::ekfind) may
//! be too confused to give a helpful diagnostic message.
//!
//!  * Parsing errors---these result from a badly formed query that [EKFIND](crate::raw::ekfind) was
//! able to separate into tokens but that [EKFIND](crate::raw::ekfind) determined to be syntactically
//! invalid.
//!
//!  * Name resolution errors---these result from referencing invalid or ambiguous
//! column or table names in a query.
//!
//!  * Time resolution errors---these result when a failure occurs during the
//! parse of a time string.
//!
//!  * Miscellaneous semantic errors---these result from a syntactically valid
//! query that violates a limit or a restriction on values used in a query.
//!
//!  Some problems that may be encountered by [EKFIND](crate::raw::ekfind) are not due to invalid
//!    queries, but are genuine error conditions:
//!
//!  
//!
//! * No E-kernels are loaded at the time [EKFIND](crate::raw::ekfind) is called.
//!
//!  * A time value is used in a query before a leapseconds kernel is loaded.
//!
//!  * A SCLK value is used in a query before an SCLK kernel for the appropriate
//! spacecraft clock has been loaded.
//!
//!  These problems cause a SPICE error to signal. The outputs of [EKFIND](crate::raw::ekfind) are
//!    undefined in this case.
//!
//!  
//!
//!
//!  
//! ###  Fetching data from matching rows
//!
//!  The EK fetch routines [EKGC](crate::raw::ekgc), [EKGD](crate::raw::ekgd), and [EKGI](crate::raw::ekgi) return data of the indicated
//!    data type from rows satisfying an EK query.
//!
//!  The EK fetch routines return one column entry element at a time, so it
//!    is not necessary to know in advance the size of the column entry.
//!
//!  To fetch data from a character column, use
//!
//!  
//!
//! ```text
//!    CALL EKGC ( <selidx>, <row>, <elment>, CDATA,        {Get character
//!    .             NULL,    FOUND                  )       data}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  SELIDX
//!
//!
//!  is the index of the column of interest in the SELECT clause of the
//! query. Column indices range from 1 : NCOLS where NCOLS is the number of
//! columns referenced in the SELECT clause of the query.
//!
//!  *  ROW
//!
//!
//!  is the index of the row from which to fetch. Row indices range from 1 :
//! NMROWS where NMROWS is the number of matching rows returned by [EKFIND](crate::raw::ekfind).
//!
//!  *  ELMENT
//!
//!
//!  is the index of the column entry element to fetch. Element indices
//! range from 1 : NELTS where NELTS is the number of elements in the
//! column entry. It is not an error to specify an out-of-range value of
//! ELMENT: this will simply cause FOUND to indicate no element was found.
//!
//!  *  CDATA
//!
//!
//!  is a string containing the specified column entry element.
//!
//!  *  NULL
//!
//!
//!  is a flag indicating whether the entry is null.
//!
//!  *  FOUND
//!
//!
//!  is a flag indicating whether the specified column entry element was
//! found.
//!
//!  To fetch double precision or TIME column entry elements, use [EKGD](crate::raw::ekgd):
//!
//!  
//!
//! ```text
//!     CALL EKGD ( <selidx>, <row>, <elment>,
//!    .            DDATA,    NULL,  FOUND    )            {Get d.p. data}
//!  
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKGC](crate::raw::ekgc), except that DDATA represents a double precision number.
//!
//!  To fetch integer column entry elements, use [EKGI](crate::raw::ekgi):
//!
//!  
//!
//! ```text
//!    CALL EKGI ( <selidx>, <row>, <elment>,
//!                IDATA,    NULL,  FOUND )            {Get integer data}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKGC](crate::raw::ekgc), except that IDATA represents an integer.
//!
//!  
//!
//!
//!  
//! ###  Query support utilities
//!
//!  While an application can fetch array-valued column entries by calling
//!    the fetch routine of the appropriate data type in a loop, continuing
//!    until FOUND is false, it is more elegant to find the size of the entry
//!    in advance. The routine [EKNELT](crate::raw::eknelt) provides this service:
//!
//!  
//!
//! ```text
//!    EKNELT ( <selidx>, <row>, NELT )           {Get number of elements}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  SELIDX
//!
//!
//!  is the index of the column of interest in the SELECT clause of the
//! query. Column indices range from 1 : NCOLS where NCOLS is the number of
//! columns referenced in the SELECT clause of the query.
//!
//!  *  ROW
//!
//!
//!  is the index of the row from which to fetch. Row indices range from 1 :
//! NMROWS where NMROWS is the number of matching rows returned by [EKFIND](crate::raw::ekfind).
//!
//!  *  NELTS
//!
//!
//!  is the number of elements in the column entry.
//!
//!  The EK fetch routines [EKGC](crate::raw::ekgc), [EKGD](crate::raw::ekgd), [EKGI](crate::raw::ekgi), together with the utility [EKNELT](crate::raw::eknelt), suffice for applications in which the SELECT clause of the query
//!    is known in advance. For such applications, the data types of the SELECT
//!    columns are known in advance, so it is clear which fetch routine to call
//!    to retrieve any column entry.
//!
//!  Some more complex EK applications may require the ability to fetch
//!    results from an arbitrary query. In order to do this, an application
//!    must be able to determine at run time the names and data types of the
//!    SELECT columns. If an application needs to unambiguously identify the
//!    columns, the names of the tables to which the columns belong are needed
//!    as well.
//!
//!  Applications need not analyze a query to determine the fully qualified
//!    names and attributes of the SELECT columns---the EK subsystem provides
//!    the routine [EKPSEL](crate::raw::ekpsel) to do this job.
//!
//!  Note: in the discussion below, there are references to substrings in the
//!    SELECT clause as "expressions." Currently, the only supported
//!    expressions in the SELECT clause are column names. However, [EKPSEL](crate::raw::ekpsel) has
//!    been designed to support possible query language enhancements, such as
//!    specification of general expressions in the SELECT clause.
//!
//!  Calls to [EKPSEL](crate::raw::ekpsel) are made as shown:
//!
//!  
//!
//! ```text
//!     CALL EKPSEL ( <query>,  N,  XBEGS,  XENDS,  XTYPES,
//!    .              XCLASS, TABS, COLS,   ERROR,  ERRMSG ) { Parse
//!                                                            SELECT
//!                                                            clause }
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  QUERY
//!
//!
//!  is the query to be analyzed.
//!
//!  *  N
//!
//!
//!  is the number of expressions (columns) in the SELECT clause of the
//! query.
//!
//!  *  XBEGS
//!
//!
//!  is an array of starting indices of expressions in the SELECT clause.
//!
//!  *  XENDS
//!
//!
//!  is an array of ending indices of expressions in the SELECT clause. The
//! ith expression occupies query elements QUERY( XBEGS(I) ) through QUERY(
//! XENDS (I) ) .
//!
//!  *  XTYPES
//!
//!
//!  is an array of data types of the expressions in the SELECT clause of
//! the query.
//!
//!  *  XCLASS
//!
//!
//!  is an array of classes of the expressions in the SELECT clause of the
//! query. The ith element of XCLASS indicates whether the ith SELECT
//! expression is a column name, a function, or a more general expression.
//!
//!  *  TABS
//!
//!
//!  is an array containing table names corresponding to the SELECT columns.
//! Actual table names are returned, even if the column names are qualified
//! by aliases in the query. Elements of TABS corresponding to SELECT
//! expressions that are not column names are undefined.
//!
//!  *  COLS
//!
//!
//!  is an array containing unqualified versions of the column names
//! appearing in the SELECT clause. The ith element of COLS is defined only
//! if the ith expression in the SELECT clause is a column name.
//!
//!  *  ERROR
//!
//!
//!  is a flag indicating whether a parsing error occurred while analyzing
//! the query. If an error occurred, the other outputs, with the exception
//! of the error message, are undefined.
//!
//!  *  ERRMSG
//!
//!
//!  is a string containing an error diagnosis, if a parsing error occurred.
//! Otherwise, ERRMSG is returned blank.
//!
//!     
//! ##  Record-oriented reader interface
//!
//!  The EK record-oriented reader routines provide applications with a
//!    low-level interface for examining EK data. Using these routines, an
//!    application has direct access to specified EK column entries.
//!
//!  
//!
//!
//!  
//! ###  Opening files for record-oriented reading
//!
//!  The record-oriented readers may be used to read EKs open for either read
//!    or write access. If the EK is to be opened for read access, the routine
//!    [EKOPR](crate::raw::ekopr) is convenient:
//!
//!  
//!
//! ```text
//!    CALL EKOPR ( <fname>, HANDLE )             {EK, open for read}
//! ```
//!
//!  If the EK to be read is to be queried, then the EK should be loaded
//!    using [FURNSH](crate::raw::furnsh).
//!
//!  
//!
//! ```text
//!    CALL FURNSH ( <fname> )               {Load SPICE kernel}
//! ```
//!
//!  The file's handle may be obtained using a call to the SPICELIB routine [KINFO](crate::raw::kinfo).
//!
//!  
//!
//!
//!  
//! ###  Column entry readers
//!
//!  The record-oriented readers return a specified column entry from a
//!    specified EK. To read a character column entry, call [EKRCEC](crate::raw::ekrcec):
//!
//!  
//!
//! ```text
//!     CALL EKRCEC ( <handle>, <segno>, <recno>, <column>, {read character
//!    .              NVALS,    CVALS,   ISNULL         )    column entry}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the file handle of the sequence EK to be read.
//!
//!  *  SEGNO
//!
//!
//!  is the ordinal position in the EK of the segment containing the desired
//! column entry.
//!
//!  *  SEGNO
//!
//!
//!  is the ordinal position in the segment of the record containing the
//! desired column entry.
//!
//!  *  COLUMN
//!
//!
//!  is the name of the column containing the desired entry.
//!
//!  *  NVALS
//!
//!
//!  is the number of array elements in the column entry.
//!
//!  *  CVALS
//!
//!
//!  is the column entry itself. Note that CVALS must have sufficient size
//! to accommodate the entire column entry. CVALS is defined only if the
//! column entry is non-null.
//!
//!  *  ISNULL
//!
//!
//!  is a flag indicating whether the column entry is null.
//!
//!  To read a double precision or TIME column entry, call [EKRCED](crate::raw::ekrced):
//!
//!  
//!
//! ```text
//!     CALL EKRCED ( <handle>, <segno>,  <recno>, <column>,  {read d.p.
//!    .              NVALS,    DVALS,    ISNULL            )  column entry}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKRCED](crate::raw::ekrced), except that DVALS represents a double precision array.
//!
//!  To read an integer column entry, call [EKRCEI](crate::raw::ekrcei):
//!
//!  
//!
//! ```text
//!     CALL EKRCEI ( <handle>, <segno>,  <recno>, <column>, {read integer
//!    .              NVALS,    IVALS,    ISNULL           )  column entry}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKRCEC](crate::raw::ekrcec), except that IVALS represents an integer array.
//!
//!  
//!
//!
//!  
//! ##  Informational routines
//!
//!  
//!
//!
//!  
//! ###  Summarizing EK files
//!
//!  To summarize an EK file, the file must be open for read or write access,
//!    and the file handle must be available. If the file is open for write
//!    access, the file must be structurally valid: for example, a fast write
//!    operation on the file must not be partially executed.
//!
//!  The number of segments in an EK is found by calling [EKNSEG](crate::raw::eknseg):
//!
//!  
//!
//! ```text
//!    N = EKNSEG ( <handle> )                 {Return number of segments}
//! ```
//!
//!  The summary of the segment at ordinal position SEGNO is returned by [EKSSUM](crate::raw::ekssum):
//!
//!  
//!
//! ```text
//!     CALL EKSSUM ( <handle>, <segno>, TABNAM, NROWS,   {Summarize
//!    .              NCOLS,  CNAMES,    DTYPES, SIZES,    segment}
//!    .              STRLNS, INDEXD,    NULLOK      )
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the file handle of the sequence EK containing the segment of
//! interest.
//!
//!  *  SEGNO
//!
//!
//!  is the ordinal position in the EK of the segment to be summarized.
//!
//!  *  TABNAM
//!
//!
//!  is the name of the table to which the segment belongs.
//!
//!  *  NROWS
//!
//!
//!  is the number of rows in the segment.
//!
//!  *  NCOLS
//!
//!
//!  is the number of columns in the segment.
//!
//!  *  CNAMES
//!
//!
//!  is an array containing the names of the columns in the segment.
//!
//!  *  DTYPES
//!
//!
//!  is an array, parallel to CNAMES, containing the data types of the
//! columns in the segment.
//!
//!  *  SIZES
//!
//!
//!  is an array, parallel to CNAMES, containing the column entry array
//! sizes of the columns in the segment.
//!
//!  *  STRLNS
//!
//!
//!  is an array, parallel to CNAMES, containing the string lengths of the
//! character columns in the segment. The elements of STRLNS corresponding
//! to non-character columns are undefined.
//!
//!  *  INDEXD
//!
//!
//!  is an array, parallel to CNAMES, containing flags indicating whether
//! the corresponding columns are indexed.
//!
//!  *  NULLOK
//!
//!
//!  is an array, parallel to CNAMES, containing flags indicating whether
//! the corresponding columns allow null values.
//!
//!     
//! ###  Summarizing loaded tables
//!
//!  A "loaded EK table" is a virtual table consisting of the union of all
//!    rows of data from segments having a given table name and schema, where
//!    the segments belong to loaded EKs. These tables exist in the EK query
//!    subsystem when EKs are loaded using [FURNSH](crate::raw::furnsh).
//!
//!  The number of loaded tables may be found by calling [EKNTAB](crate::raw::ekntab):
//!
//!  
//!
//! ```text
//!    CALL EKNTAB ( N )             {Return number of loaded tables}
//! ```
//!
//!  The name of the nth loaded table may be found by calling [EKTNAM](crate::raw::ektnam):
//!
//!  
//!
//! ```text
//!    CALL EKTNAM ( <n>, TABLE )                 {Return table name}
//! ```
//!
//!  The number of columns in a specified, loaded table may be found by
//!    calling [EKCCNT](crate::raw::ekccnt):
//!
//!  
//!
//! ```text
//!    CALL EKCCNT ( <table>, CCOUNT )            {Return column count}
//! ```
//!
//!  The name and attributes of the column having a specified ordinal
//!    position within a specified, loaded table may be found by calling [EKCII](crate::raw::ekcii):
//!
//!  
//!
//! ```text
//!    CALL EKCII ( <table>, <cindex>, COLUMN, ATTDSC )
//!  
//!                                                 {Return attributes
//!                                                  of column specified
//!                                                  by index}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  TABLE
//!
//!
//!  is the name of the table containing the column of interest.
//!
//!  *  CINDEX
//!
//!
//!  is the ordinal position of the column of interest within its table.
//!
//!  *  COLUMN
//!
//!
//!  is the returned column name.
//!
//!  *  ATTDSC
//!
//!
//!  is the returned column attribute descriptor. See the include file
//! ekattdsc.inc for details.
//!
//!     
//! #  Writing sequence EKs
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  This chapter describes how to create new sequence EKs and update
//!    existing ones.
//!
//!  The basic sequence of operations by which a new sequence EK is created
//!    is:
//!
//!  
//!
//! *  1. Open a new sequence EK. This step prepares the sequence EK for the addition
//! of data.
//!
//!  *  2. Add one or more segments to the sequence EK. A segment may be created using
//! the record-oriented writers or the fast writers.
//!
//!  *  3. Close the sequence EK. This step writes bookkeeping information to the
//! file, flushes to the file any remaining buffered data that have not been
//! physically written out, and closes the file.
//!
//!  Existing segments in a sequence EK may be updated: new records may be
//!    added, records may be deleted, and individual column entries in existing
//!    segments may be updated.
//!
//!  An existing, closed sequence EK may be opened for write access, at which
//!    point all operations valid for a new sequence EK may be performed on the
//!    file.
//!
//!  The comment area of a sequence EK may be written to when the file is
//!    open for write access.
//!
//!  
//!
//!
//!  
//! ##  Opening a sequence EK for writing
//!
//!  
//!
//!
//!  
//! ###  Beginning a new sequence EK
//!
//!  A new sequence EK is opened and prepared for writing using a call to
//!    [EKOPN](crate::raw::ekopn):
//!
//!  
//!
//! ```text
//!    CALL EKOPN ( <fname>, <ifname>, <ncomch>, HANDLE )  {Open new EK}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  FNAME
//!
//!
//!  is the name of the new sequence EK.
//!
//!  *  IFNAME
//!
//!
//!  is the internal file name of the new sequence EK. This name may be up
//! to 60 characters in length; it must contain only printing characters
//! and blanks. IFNAME may be left blank.
//!
//!  *  NCOMCH
//!
//!
//!  is the number of comment characters to reserve in the comment area.
//! Zero is an acceptable value for NCOMCH. However, reserving sufficient
//! space for comments speeds up addition of comments after the file is
//! written, since data records will not need to be shifted to make room.
//! See the COMMNT User's Guide, [commnt.ug](crate::raw::commnt.ug), for further information.
//!
//!  *  HANDLE
//!
//!
//!  is the returned file handle. HANDLE will be used to identify the file
//! to other sequence EK routines.
//!
//!     
//! ###  Opening an existing sequence EK for writing
//!
//!  Use [EKOPW](crate::raw::ekopw) to open an existing sequence EK for writing:
//!
//!  
//!
//! ```text
//!    CALL EKOPW ( <fname>, HANDLE )           {Open EK for writing}
//! ```
//!
//!  The arguments of [EKOPW](crate::raw::ekopw) have the same meanings as the corresponding
//!    arguments of [EKOPN](crate::raw::ekopn).
//!
//!  
//!
//!
//!  
//! ##  Choosing a writing method
//!
//!  Before a new segment is started, the writing method must be selected:
//!    the choices are "record-oriented" or "fast."
//!
//!  Record-oriented writing allows records to be added to a segment one at a
//!    time; this approach simplifies creating records from a streaming data
//!    source. Records may be added to a segment in arbitrary order. Also, it
//!    is possible to build multiple segments simultaneously using the
//!    record-oriented writers.
//!
//!  The significant limitation of the record-oriented approach is that it is
//!    slow, particularly if the segment being written contains indexed
//!    columns. When execution speed is critical, it may be advisable to use
//!    the "fast writers." These routines can create a segment as much as 100
//!    times faster than their record-oriented counterparts. However, the fast
//!    writers require all of the segment's data to be staged before the
//!    segment is written.
//!
//!  Below, we discuss aspects of segment creation common to both writing
//!    approaches. See the sections "Using the record-oriented sequence EK
//!    writers" and "Using the fast writers" below for specifics on how to
//!    implement either approach.
//!
//!  
//!
//!
//!  
//! ###  Specifying segment attributes
//!
//!  When starting a new segment using either the record-oriented or fast
//!    writers, it is necessary to specify the name of the table the segment
//!    belongs to and the names and attributes of the columns in the table.
//!    When using the fast writers, it is also necessary to specify the number
//!    of rows in the segment.
//!
//!  
//!
//!
//!  
//! ###  Table and column names
//!
//!  Table and column names must start with a letter and contain only
//!    characters from the set
//!
//!  
//!
//! ```text
//!    {A-Z, a-z, 0-9, $,  _}
//! ```
//!
//!  Case is not significant.
//!
//!  Table names must not exceed TNAMSZ (see include file ektnamsz.inc)
//!    characters in length. Column names must not exceed CNAMSZ (see include
//!    file ekcnamsz.inc) characters in length.
//!
//!  
//!
//!
//!  
//! ###  Column declarations
//!
//!  Column attributes are specified in arrays of strings called "column
//!    declarations." There is one declaration per column. The syntax for
//!    column declarations is independent of the writing method.
//!
//!  Column declarations are strings that contain "keyword=value"
//!    assignments that define the attributes of the columns to which they
//!    apply. The column attributes defined by a column declaration are:
//!
//!  
//!
//! ```text
//!    DATATYPE
//!    SIZE
//!    <is the column indexed?>
//!    <does the column allow null values?>
//! ```
//!
//!  When a segment is started using [EKBSEG](crate::raw::ekbseg) or [EKIFLD](crate::raw::ekifld), an array of column
//!    declarations must be supplied as an input. The form of a column
//!    declaration string is a list of "keyword=value" assignments, delimited
//!    by commas, as shown:
//!
//!  
//!
//! ```text
//!    'DATATYPE  = <type>,'       //
//!    'SIZE      = <size>,'       //
//!    'INDEXED   = <boolean>,'    //
//!    'NULLS_OK  = <boolean>'
//! ```
//!
//!  For example, an indexed, scalar, integer column that does not allow null
//!    values would have the declaration
//!
//!  
//!
//! ```text
//!    'DATATYPE  = INTEGER, '    //
//!    'SIZE      = 1, '          //
//!    'INDEXED   = TRUE, '       //
//!    'NULLS_OK  = FALSE'
//! ```
//!
//!  Commas are required to separate the assignments within declarations;
//!    white space is optional; case is not significant.
//!
//!  The order in which the attribute keywords are listed in the declaration
//!    is not significant.
//!
//!  Data type specifications are required for each column.
//!
//!  Each column entry is effectively an array, each element of which has the
//!    declared data type. The SIZE keyword indicates how many elements are in
//!    each entry of the column. Note that only scalar-valued columns (those
//!    for which SIZE = 1) may be referenced in query constraints. A size
//!    assignment has the syntax
//!
//!  
//!
//! ```text
//!    SIZE = <integer>
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!    SIZE = VARIABLE
//! ```
//!
//!  The size value defaults to 1 if omitted.
//!
//!  The DATATYPE keyword defines the data type of column entries. The
//!    DATATYPE assignment syntax has any of the forms
//!
//!  
//!
//! ```text
//!    DATATYPE = CHARACTER*(<length>)
//!    DATATYPE = CHARACTER*(*)
//!    DATATYPE = DOUBLE PRECISION
//!    DATATYPE = INTEGER
//!    DATATYPE = TIME
//! ```
//!
//!  As the datatype declaration syntax suggests, character strings may have
//!    fixed or variable length. For example, a fixed-length string of 80
//!    characters is indicated by the declaration
//!
//!  
//!
//! ```text
//!    DATATYPE = CHARACTER*(80)
//! ```
//!
//!  while a variable-length string is indicated by an asterisk:
//!
//!  
//!
//! ```text
//!    DATATYPE = CHARACTER*(*)
//! ```
//!
//!  Variable-length strings have a practical length limit of 1024
//!    characters: the sequence EK writers allow one to write a scalar string
//!    of any length, but the sequence EK query routines will truncate a string
//!    whose length exceeds this limit.
//!
//!  Variable-length strings are allowed only in scalar character columns.
//!
//!  Optionally, scalar-valued columns may be indexed. Indexing can greatly
//!    speed up the processing of some queries, because indexing allows data to
//!    be found by a binary, rather than linear, search.
//!
//!  Each index increases the size of the sequence EK file by an amount
//!    greater than or equal to the space occupied by two integers times the
//!    number of rows in the affected table, so for potentially large sequence
//!    EK files, the issue of whether or not to index a column deserves some
//!    consideration.
//!
//!  To create an index for a column, use the assignment
//!
//!  
//!
//! ```text
//!    INDEXED = TRUE
//! ```
//!
//!  By default, columns are not indexed.
//!
//!  Optionally, any column can allow null values; this is indicated by the
//!    assignment
//!
//!  
//!
//! ```text
//!    NULLS_OK = TRUE
//! ```
//!
//!  in the column declaration. By default, null values are not allowed in
//!    column entries.
//!
//!  
//!
//!
//!  
//! ###  Consistency of schemas
//!
//!  All segments belonging to a given table, that is having the same table
//!    name, must have identical schemas: the same set of column names, with
//!    each pair of identically-named columns having identical declarations.
//!
//!  The sequence EK writer routines don't diagnose segment schema
//!    inconsistencies (to do so would be cumbersome at best, since
//!    inconsistencies could occur in separate files). However, loading into
//!    the sequence EK query system segments with identical table names but
//!    inconsistent column declarations will result in an error diagnosis.
//!
//!  
//!
//!
//!  
//! ##  Using the record-oriented sequence EK writers
//!
//!  
//!
//!
//!  
//! ###  Beginning a segment
//!
//!  The first step in beginning a new segment using the record-oriented
//!    writers is to define the new segment's schema. This is done by calling
//!    [EKBSEG](crate::raw::ekbseg):
//!
//!  
//!
//! ```text
//!     CALL EKBSEG ( <handle>, <tabnam>, <ncols>, <cnames>, {Begin
//!    .              <decls>,  segno                      )  segment}
//! ```
//!
//!  The inputs to [EKBSEG](crate::raw::ekbseg) are described below:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the file handle returned by the routine used to open the sequence
//! EK.
//!
//!  *  TABNAM
//!
//!
//!  is the name of the table to which the new segment belongs.
//!
//!  *  NCOLS
//!
//!
//!  is the number of columns in the table.
//!
//!  *  CNAMES
//!
//!
//!  is an array containing the names of the columns in the table.
//!
//!  *  DECLS
//!
//!
//!  is an array containing the declarations for each column. See the
//! section "Column declarations" above for details.
//!
//!  The sole output from [EKBSEG](crate::raw::ekbseg) is:
//!
//!  
//!
//! *  SEGNO
//!
//!
//!  This is the number of the new segment: the ordinal position of the
//! segment in the file. Segment numbers start at 1 in SPICELIB.
//!
//!     
//! ###  Adding records to a segment
//!
//!  The record-oriented sequence EK write routines support building sequence
//!    EK segments one record at a time.
//!
//!  A new segment is prepared for record-oriented writing using a call to
//!    [EKBSEG](crate::raw::ekbseg) (see "Starting a new segment" above). Next, records are added
//!    to the segment. Records may be appended or may be inserted into the
//!    segment.
//!
//!  To append a new, empty record to a segment, use [EKAPPR](crate::raw::ekappr):
//!
//!  
//!
//! ```text
//!    CALL EKAPPR ( <handle>, <segno>, RECNO )        {Append record}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the EK's file handle.
//!
//!  *  SEGNO
//!
//!
//!  is the number of the segment to write to. SEGNO must be a segment
//! number obtained from [EKBSEG](crate::raw::ekbseg).
//!
//!  *  RECNO
//!
//!
//!  is the number of the new record; RECNO is an output argument.
//!
//!  To insert a new, empty record into a segment, use [EKINSR](crate::raw::ekinsr):
//!
//!  
//!
//! ```text
//!    CALL EKINSR ( <handle>, <segno>, <recno> )       {Insert record}
//! ```
//!
//!  The arguments are the same as those of [EKAPPR](crate::raw::ekappr), except that here RECNO is
//!    an input. RECNO is the desired ordinal position of the new record: RECNO
//!    must be in the range
//!
//!  
//!
//! ```text
//!    1 : NREC+1
//! ```
//!
//!  where NREC is the number of records already in the segment.
//!
//!  Each new record starts out empty. The column entries in the record are
//!    filled in one-by-one using calls to the "add column entry" routines
//!    [EKACEC](crate::raw::ekacec), [EKACED](crate::raw::ekaced), and [EKACEI](crate::raw::ekacei). The column entries of a record may be
//!    written in any order.
//!
//!  Character column entries are written by [EKACEC](crate::raw::ekacec):
//!
//!  
//!
//! ```text
//!     CALL EKACEC ( <handle>, <segno>, <recno>, <column>,   {Add character
//!    .              <nvals>,  <cvals>, <isnull>          )   column entry}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the DAS file handle obtained from [EKOPN](crate::raw::ekopn).
//!
//!  *  SEGNO
//!
//!
//!  is the number of the segment to write to. SEGNO must be a segment
//! number obtained from [EKBSEG](crate::raw::ekbseg).
//!
//!  *  RECNO
//!
//!
//!  is the number of the new record; RECNO is an output argument.
//!
//!  *  COLUMN
//!
//!
//!  is the name of the column to which data is to be written.
//!
//!  *  NVALS
//!
//!
//!  is the number of elements in the column entry. For scalar entries,
//! NVALS is 1. For null-valued entries, NVALS is ignored.
//!
//!  *  CVALS
//!
//!
//!  is the string or array of strings comprising the column entry. CVALS is
//! ignored if the entry being added is null.
//!
//!  *  ISNULL
//!
//!
//!  is a logical flag indicating whether the entry being added is null. If
//! the column has fixed-length, variable-size entries, and the entry being
//! added is null, the size of the entry is considered to be 1.
//!
//!  Double precision column entries are written by [EKACED](crate::raw::ekaced):
//!
//!  
//!
//! ```text
//!     CALL EKACED ( <handle>, <segno>, <recno>, <column>,   {Add d.p.
//!    .              <nvals>,  <dvals>, <isnull>          )   column entry}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKACEC](crate::raw::ekacec), except that DVALS represents a double precision array.
//!
//!  Values of type TIME are also added using [EKACED](crate::raw::ekaced). When a column contains
//!    TIME values (as indicated by its declared data type), the values are
//!    stored as ephemeris seconds past J2000 TDB. When starting with UTC or
//!    SCLK time values, the SPICELIB conversion routines [STR2ET](crate::raw::str2et) or [SCS2E](crate::raw::scs2e) may
//!    be used to obtain equivalent double precision TDB values. See the
//!    [TIME.REQ](crate::raw::time.req) or [SCLK.REQ](crate::raw::sclk.req) Required Reading for details.
//!
//!  Integer column entries are written by [EKACEI](crate::raw::ekacei):
//!
//!  
//!
//! ```text
//!    CALL EKACEI ( <handle>, <segno>, <recno>, <column>,   {Add integer
//!    .             <nvals>,  <ivals>, <isnull>          )   column entry}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKACEC](crate::raw::ekacec), except that IVALS represents an integer array.
//!
//!  A record must have all of its column entries written in order to be
//!    valid: column entries do not have default values.
//!
//!  No action is required to "finish" a segment created by the
//!    record-oriented writers, although [EKCLS](crate::raw::ekcls) must be called to close the file
//!    when all segments have been written.
//!
//!  
//!
//!
//!  
//! ##  Using the fast writers
//!
//!  The sequence EK "fast write" capability (referred to as "fast load"
//!    in older documentation) allows construction of a sequence EK segment
//!    much more quickly than is possible with the record-oriented writers, at
//!    the expense of some flexibility.
//!
//!  The fast write approach involves creating one new segment at a time.
//!    Segments are constructed one column at a time: each column is added to a
//!    segment in one shot.
//!
//!  In order to add a segment to a sequence EK, the sequence EK must be open
//!    for write access. New sequence EK files are opened by calling [EKOPN](crate::raw::ekopn);
//!    existing sequence EKs are opened for writing by calling [EKOPW](crate::raw::ekopw).
//!
//!  The sequence of operations required to create a segment using the fast
//!    write routines is:
//!
//!  
//!
//! *  1. Initiate a fast write operation by calling [EKIFLD](crate::raw::ekifld). This call defines the
//! segment's schema and size. The number of rows in the segment must be known
//! at the time this call is made.
//!
//!  *  2. Add data columns to the segment by calling [EKACLC](crate::raw::ekaclc), [EKACLD](crate::raw::ekacld), or [EKACLI](crate::raw::ekacli).
//!
//!  *  Because each column is created by a single subroutine call, all data
//! constituting the column, along with null flags and entry size information,
//! must be buffered in arrays declared by the user's application.
//!
//!  *  3. Complete the segment by calling [EKFFLD](crate::raw::ekffld). This call writes out bookkeeping
//! information required to make the segment readable.
//!
//!  When all segments in a sequence EK are complete, the sequence EK must be
//!    closed by calling [EKCLS](crate::raw::ekcls).
//!
//!  
//!
//!
//!  
//! ###  Initiating a fast write
//!
//!  The first step in creating a new segment using the fast writers is to
//!    define the new segment's attributes, specifically its schema and the
//!    number of rows in the segment. This is done by calling [EKIFLD](crate::raw::ekifld):
//!
//!  
//!
//! ```text
//!     CALL EKIFLD ( <handle>, <tabnam>, <ncols>, <nrows>,  {Initiate
//!    .              <cnames>, <decls>,   SEGNO,   RCPTRS )  fast write}
//! ```
//!
//!  The inputs to [EKIFLD](crate::raw::ekifld) are described below.
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the file handle returned by the routine used to open the sequence
//! EK.
//!
//!  *  TABNAM
//!
//!
//!  is the name of the table to which the new segment belongs.
//!
//!  *  NCOLS
//!
//!
//!  is the number of columns in the table.
//!
//!  *  NROWS
//!
//!
//!  is the number of rows in the new segment.
//!
//!  *  CNAMES
//!
//!
//!  is an array containing the names of the columns in the table.
//!
//!  *  DECLS
//!
//!
//!  is an array containing the declarations for each column. The
//! declaration syntax is identical to that used by the record-oriented
//! routine [EKBSEG](crate::raw::ekbseg). See the section "Column declarations" above for
//! details.
//!
//!  The outputs from [EKIFLD](crate::raw::ekifld) are:
//!
//!  
//!
//! *  SEGNO
//!
//!
//!  This is the number of the new segment: the ordinal position of the
//! segment in the file. Segment numbers start at 1 in SPICELIB.
//!
//!  *  RCPTRS
//!
//!
//!  This is an integer work space array the caller must not modify. RCPTRS
//! must have dimension at least equal to the number of rows in the
//! segment. RCPTRS is passed as an input to the other fast writer
//! routines.
//!
//!     
//! ###  Adding columns to the segment
//!
//!  There are three fast writer routines used to add columns to a segment;
//!    the routine to use depends on the data type of the column.
//!
//!  To add a character column to a segment, call [EKACLC](crate::raw::ekaclc):
//!
//!  
//!
//! ```text
//!     CALL EKACLC( <handle>, <segno>,  <column>, <cvals>,  {Add
//!    .             <entszs>, <nlflgs>, <rcptrs>, wkindx  )  character
//!                                                           column}
//! ```
//!
//!  The inputs to [EKACLC](crate::raw::ekaclc) are described below.
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the file handle returned by the routine used to open the sequence
//! EK.
//!
//!  *  SEGNO
//!
//!
//!  is the segment number obtained from [EKIFLD](crate::raw::ekifld).
//!
//!  *  COLUMN
//!
//!
//!  is the name of the column to be added.
//!
//!  *  CVALS
//!
//!
//!  is an array containing the entire set of column entries for the
//! specified column. The entries are listed in row-order: the column entry
//! for the first row of the segment is first, followed by the column entry
//! for the second row, and so on. If the column is array-valued, the
//! successive elements of each column entry occupy successive strings in
//! the CVALS array.
//!
//!  *  Regarding null values: for columns having fixed-size entries, a null
//! entry must be allocated the same amount of space occupied by a non-null
//! entry in the array CVALS. For columns having variable-size entries,
//! null entries do not require any space in the CVALS array, but in any
//! case must have their allocated space described correctly by the
//! corresponding element of the ENTSZS array.
//!
//!  *  ENTSZS
//!
//!
//!  is an array of integers, each specifying the number of array elements
//! in the corresponding column entry. The values in the array ENTSZS are
//! used only for columns having variable-size entries, but in all cases,
//! ENTSZS must have dimension at least equal to the number of rows in the
//! segment. For null entries in variable-dimension columns, the
//! corresponding element of ENTSZS should be set to zero.
//!
//!  *  NLFLGS
//!
//!
//!  is an array of logical flags, each specifying whether the corresponding
//! column entry is null. For columns that don't allow null values, the
//! contents of this array are ignored. In all cases, NLFLGS must have
//! dimension at least equal to the number of rows in the segment.
//!
//!  *  RCPTRS
//!
//!
//!  This is an integer work space array the caller must not modify. RCPTRS
//! is an output argument of [EKIFLD](crate::raw::ekifld). RCPTRS must have dimension at least
//! equal to the number of rows in the segment.
//!
//!  *  WKINDX
//!
//!
//!  is an integer workspace array used to build an index for the column.
//!
//!  To add a double precision column to a segment, call [EKACLD](crate::raw::ekacld):
//!
//!  
//!
//! ```text
//!     CALL EKALCD ( <handle>, <segno>,  <column>, <dvals>,  {Add d.p.
//!    .              <entszs>, <nlflgs>, <rcptrs>, wkindx  )  column}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKACLC](crate::raw::ekaclc), except that DVALS represents a double precision array.
//!
//!  Values of type TIME are also added using [EKACLD](crate::raw::ekacld). When a column contains
//!    TIME values (as indicated by its declared data type), the values are
//!    stored as ephemeris seconds past J2000 TDB. When starting with UTC or
//!    SCLK time values, the SPICELIB conversion routines [STR2ET](crate::raw::str2et) or [SCS2E](crate::raw::scs2e) may
//!    be used to obtain equivalent double precision TDB values. See the
//!    [TIME.REQ](crate::raw::time.req) or [SCLK.REQ](crate::raw::sclk.req) Required Reading for details.
//!
//!  To add an integer column to a segment, call [EKACLI](crate::raw::ekacli):
//!
//!  
//!
//! ```text
//!    CALL EKALCI ( <handle>, <segno>,  <recno>, <ivals>,    {Add integer
//!    .             <entszs>, <nlflgs>, <rcptrs>, wkindx  )   column}
//! ```
//!
//!  The arguments have the same meanings as the corresponding arguments of [EKACLC](crate::raw::ekaclc), except that IVALS represents an integer array.
//!
//!  
//!
//!
//!  
//! ###  Completing a fast write
//!
//!  Once all columns have been added to the segment, the fast write
//!    operation on the segment is completed using a call to [EKFFLD](crate::raw::ekffld):
//!
//!  
//!
//! ```text
//!    CALL EKFFLD ( <handle>, <segno>, <rcptrs> ) {Finish fast write}
//! ```
//!
//!  The meanings of the arguments of [EKFFLD](crate::raw::ekffld) are identical to those of the
//!    same names belonging to [EKIFLD](crate::raw::ekifld).
//!
//!  Calling [EKFFLD](crate::raw::ekffld) is an essential step; the segment will not be
//!    structurally valid until this call has been made.
//!
//!  Once the fast write operation has been completed, the segment may be
//!    modified using the record-oriented writers.
//!
//!  
//!
//!
//!  
//! ###  Restrictions
//!
//!  Creating a segment using the fast writer routines should be regarded as
//!    an "indivisible" process: other EK operations should not be performed
//!    when a fast write is in progress.
//!
//!  Record-oriented append, insert, and delete operations are not supported
//!    for a segment in the process of being constructed by the fast writers.
//!    Updating or reading column entries in the middle of a fast write is also
//!    not supported.
//!
//!  Fast write operations may not be interleaved with query-and-fetch
//!    operations: an application may not start a fast write, issue a query,
//!    then continue the fast write, or vice versa.
//!
//!  Only one segment can be created at a time using the fast writers.
//!
//!  One cannot extend an existing segment using the fast write routines.
//!    However, a segment created using the fast writers, once completed using
//!    a call to [EKFFLD](crate::raw::ekffld), may be modified using the record-oriented write,
//!    update, or delete routines.
//!
//!  
//!
//!
//!  
//! ##  Updating an existing sequence EK
//!
//!  Except when a fast write is in progress, any segment of a sequence EK
//!    open for write access may be updated. Updates may consist of adding
//!    records, deleting records, or updating individual column entries.
//!
//!  Adding records is done using the record-oriented writers, which are
//!    described above. Column entries may be updated using the routines
//!    [EKUCEC](crate::raw::ekucec), [EKUCED](crate::raw::ekuced), and [EKUCEI](crate::raw::ekucei), which operate on, respectively, character,
//!    double precision (or time), and integer column entries. The argument
//!    lists of these routines are identical to the record-oriented column
//!    entry addition routines of the corresponding data types.
//!
//!  When updating a variable-size column entry, it is permissible to replace
//!    the original entry with one having a different size. Variable-length
//!    strings also can be replaced with strings of different lengths.
//!
//!  For columns that allow null values, null entries can be updated with
//!    non-null values and vice versa.
//!
//!  Records are deleted using a call to [EKDELR](crate::raw::ekdelr):
//!
//!  
//!
//! ```text
//!    CALL EKDELR ( <handle>, <segno>, <recno> ) {Delete record}
//! ```
//!
//!  With the arguments:
//!
//!  
//!
//! *  HANDLE
//!
//!
//!  is the DAS file handle obtained from [EKOPN](crate::raw::ekopn) or [EKOPW](crate::raw::ekopw).
//!
//!  *  SEGNO
//!
//!
//!  is the number of the segment containing the record of interest. SEGNO
//! must be a segment number obtained from [EKBSEG](crate::raw::ekbseg) or [EKIFLD](crate::raw::ekifld).
//!
//!  *  RECNO
//!
//!
//!  is the number of the record to delete.
//!
//!  Deleting all records from a segment simply leaves an empty segment in
//!    the parent sequence EK; segments cannot be deleted.
//!
//!  
//!
//!
//!  
//! ##  Closing a sequence EK
//!
//!  When all segments in a sequence EK are complete, the sequence EK must be
//!    closed by calling [EKCLS](crate::raw::ekcls). This step is necessary to create a structurally
//!    correct sequence EK, since some written data may be buffered but not yet
//!    written to the physical sequence EK after the last segment has been
//!    completed. Also, [EKCLS](crate::raw::ekcls) re-organizes (using the DAS subsystem) the
//!    physical records in the sequence EK to enhance read performance when the
//!    file is reopened.
//!
//!  The record-oriented read routines may be used to read data from a
//!    sequence EK before it has been closed. However, a sequence EK open for
//!    write access may not by loaded by [FURNSH](crate::raw::furnsh) and hence is not accessible by
//!    the sequence EK query and fetch routines.
//!
//!  
//!
//!
//!  
//! #  Appendix A --- Summary of E-kernel Routines
//!
//!  
//!
//!
//!  
//! ##  Summary of mnemonics
//!
//!  SPICELIB contains a family of subroutines that are designed specifically
//!    for use with binary EK files. The name of each routine begins with the
//!    letters "EK," followed by a two- or three-character mnemonic. For
//!    example, the routine that issues a query and finds the matching rows is
//!    named [EKFIND](crate::raw::ekfind), pronounced "E-K-FIND."
//!
//!  Many of the routines listed are entry points of another routine. If a
//!    routine is an entry point, the parent routine's name will be listed
//!    inside brackets preceding the mnemonic translation.
//!
//!  The following is a complete list of mnemonics and translations, in
//!    alphabetical order.
//!
//!  
//!
//! ```text
//!    EKACEC             ( EK, add character data to column           )
//!    EKACED             ( EK, add d.p. data to column                )
//!    EKACEI             ( EK, add integer data to column             )
//!    EKACLC             ( EK, add character column to segment        )
//!    EKACLD             ( EK, add d.p. column to segment             )
//!    EKACLI             ( EK, add integer column to segment          )
//!    EKAPPR             ( EK, append record onto segment             )
//!    EKBSEG             ( EK, start new segment                      )
//!    EKCCNT    [EKQMGR] ( EK, column count                           )
//!    EKCII     [EKQMGR] ( EK, column info by index                   )
//!    EKCLS              ( EK, close file                             )
//!    EKDELR             ( EK, delete record from segment             )
//!    EKFFLD             ( EK, finish fast write                      )
//!    EKFIND             ( EK, find data                              )
//!    EKGC      [EKQMGR] ( EK, get event data, character              )
//!    EKGD      [EKQMGR] ( EK, get event data, double precision       )
//!    EKGI      [EKQMGR] ( EK, get event data, integer                )
//!    EKIFLD             ( EK, initialize segment for fast write      )
//!    EKINSR             ( EK, insert record into segment             )
//!    EKLEF     [EKQMGR] ( EK, load event file                        )
//!    EKNELT    [EKQMGR] ( EK, get number of elements in column entry )
//!    EKNSEG             ( EK, number of segments in file             )
//!    EKNTAB    [EKQMGR] ( EK, return number of loaded tables         )
//!    EKOPN              ( EK, open new file                          )
//!    EKOPR              ( EK, open file for reading                  )
//!    EKOPS              ( EK, open scratch file                      )
//!    EKOPW              ( EK, open file for writing                  )
//!    EKPSEL             ( EK, parse SELECT clause                    )
//!    EKRCEC             ( EK, read column entry element, character   )
//!    EKRCED             ( EK, read column entry element, d.p.        )
//!    EKRCEI             ( EK, read column entry element, integer     )
//!    EKSRCH    [EKQMGR] ( EK, search for events                      )
//!    EKSSUM             ( EK, return segment summary                 )
//!    EKTNAM    [EKQMGR] ( EK, return name of loaded table            )
//!    EKUCEC             ( EK, update character column entry          )
//!    EKUCED             ( EK, update d.p. column entry               )
//!    EKUCEI             ( EK, update integer column entry            )
//!    EKUEF     [EKQMGR] ( EK, unload event file                      )
//! ```
//!
//!     
//! ##  Summary of Calling Sequences
//!
//!  The calling sequences for the EK subroutines are summarized below. The
//!    subroutines are grouped by purpose.
//!
//!  Load files for query access, unload files:
//!
//!  
//!
//! ```text
//!    FURNSH ( FNAME  )
//!    UNLOAD ( FNAME  )
//! ```
//!
//!  Open files for record-oriented reading or writing, close files:
//!
//!  
//!
//! ```text
//!    DAFLLC ( HANDLE )
//!    EKCLS  ( HANDLE )
//!    EKOPN  ( FNAME, IFNAME, NCOMCH, HANDLE )
//!    EKOPR  ( FNAME, HANDLE )
//!    EKOPS  ( HANDLE )
//!    EKOPW  ( FNAME, HANDLE )
//! ```
//!
//!  Obtain summaries of sequence EK segments:
//!
//!  
//!
//! ```text
//!    EKNSEG ( HANDLE )
//!    EKSSUM ( HANDLE, SEGNO, TABNAM, NROWS,  NCOLS, CNAMES,
//!             DTYPES, SIZES, STRLNS, INDEXD, NULLOK         )
//! ```
//!
//!  Obtain summaries of loaded tables:
//!
//!  
//!
//! ```text
//!    EKCCNT ( TABLE, CCOUNT )
//!    ECKII  ( TABLE, CINDEX, COLUMN, ATTDSC )
//!    EKNTAB ( N )
//!    EKTNAM ( N, TABLE  )
//! ```
//!
//!  Query and fetch:
//!
//!  
//!
//! ```text
//!    EKFIND   ( QUERY,  NMROWS, ERROR,  ERRMSG )
//!    EKGC     ( SELIDX, ROW,    ELMENT, CDATA,   NULL,   FOUND )
//!    EKGD     ( SELIDX, ROW,    ELMENT, DDATA,   NULL,   FOUND )
//!    EKGI     ( SELIDX, ROW,    ELMENT, IDATA,   NULL,   FOUND )
//!    EKNELT   ( SELIDX, ROW )
//!    EKPSEL   ( QUERY,  N,      XBEGS,  XENDS,  XTYPES,
//!               XCLASS, TABS,   COLS,   ERROR,  ERRMSG         )
//! ```
//!
//!  Record-oriented read:
//!
//!  
//!
//! ```text
//!    EKRCEC ( HANDLE, SEGNO, RECNO, COLUMN, NVALS, CVALS, ISNULL )
//!    EKRCED ( HANDLE, SEGNO, RECNO, COLUMN, NVALS, DVALS, ISNULL )
//!    EKRCEI ( HANDLE, SEGNO, RECNO, COLUMN, NVALS, IVALS, ISNULL )
//! ```
//!
//!  Fast write:
//!
//!  
//!
//! ```text
//!    EKIFLD ( HANDLE, TABNAM, NCOLS,  NROWS, CNAMES, DECLS, SEGNO,
//!             RCPTRS                                              )
//!    EKACLC ( HANDLE, SEGNO,  COLUMN, CVALS, ENTSZS, NLFLGS,
//!             RCPTRS, WKINDX                                 )
//!    EKACLD ( HANDLE, SEGNO,  COLUMN, DVALS, ENTSZS, NLFLGS,
//!             RCPTRS, WKINDX                                 )
//!    EKACLI ( HANDLE, SEGNO,  COLUMN, IVALS, ENTSZS, NLFLGS,
//!             RCPTRS, WKINDX                                 )
//!    EKFFLD ( HANDLE, SEGNO,  RCPTRS )
//! ```
//!
//!  Begin segment for record-oriented write:
//!
//!  
//!
//! ```text
//!    EKBSEG ( HANDLE, TABNAM, NCOLS, CNAMES, DECLS, SEGNO )
//! ```
//!
//!  Insert, append, or delete records:
//!
//!  
//!
//! ```text
//!    EKAPPR ( HANDLE, SEGNO,  RECNO )
//!    EKDELR ( HANDLE, SEGNO,  RECNO )
//!    EKINSR ( HANDLE, SEGNO,  RECNO )
//! ```
//!
//!  Record-oriented write and update:
//!
//!  
//!
//! ```text
//!    EKACEC ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, CVALS, ISNULL )
//!    EKACED ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, DVALS, ISNULL )
//!    EKACEI ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, IVALS, ISNULL )
//!    EKUCEC ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, CVALS, ISNULL )
//!    EKUCED ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, DVALS, ISNULL )
//!    EKUCEI ( HANDLE, SEGNO,  RECNO, COLUMN, NVALS, IVALS, ISNULL )
//! ```
//!
//!     
//! ##  Revisions
//!
//!  
//!
//!
//!  
//! ###  March 23, 2016 NJB (JPL)
//!
//!  Corrected documentation of some calling sequences.
//!
//!  
//!
//!
//!  
//! ###  February 24, 2010 EDW (JPL)
//!
//!  Documentation expanded to include descriptions of Icy functions.
//!
//!  
//!
//!
//!  
//! ###  April 1, 2009
//!
//!  Added a note about the SPICE file identification word for EK files.
//!
//!  
//!
//!
//!  
//! ###  Feb. 06, 2002
//!
//!  Several typos were corrected.
//!
//!  
//!
//!
//!  
//! ###  Jan. 15, 2002
//!
//!  Initial release.
//!
//!  
//!
//!
//!     
