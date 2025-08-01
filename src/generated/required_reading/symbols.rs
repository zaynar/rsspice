//! #  Symbol Tables
//!
//!  Last revised on 2008 JAN 17 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  Symbol tables are SPICE data structures used to associate variable names
//!    with sets of values.
//!
//!  
//!
//!
//!  
//! ##  Revisions
//!
//!  June 15, 1992
//!
//!  
//!
//! *  The document differs from the previous version of December 2, 1989 in that
//! a number of typographical errors were corrected and a paragraph in the
//! section on Symbol Table Initialization was modified to improve its clarity.
//!
//!     
//! ##  What are Symbol Tables?
//!
//!  Symbol tables are storage structures used to associate multiple values
//!    with one variable name called a symbol. This storage structure consists
//!    of three arrays. The first array, called the name table, contains the
//!    names of the symbols. The second array, called the pointer table,
//!    contains the pointers which associate the symbol name with its values.
//!    The third array, called the value table, contains values associated with
//!    the symbol names in the name table.
//!
//!  As implemented in SPICELIB, there are three types of symbol tables:
//!    character, double precision, and integer. While the symbol names are
//!    always character strings, the type of the symbol table is determined by
//!    the data type of the values. For example, an integer symbol table has
//!    integer values associated with its symbol names.
//!
//!  
//!
//!
//!  
//! ##  Illustration of Symbol Table Representation
//!
//!  This is an illustration of how the contents of a symbol table are
//!    represented in the SPICELIB documentation. This symbol table is a double
//!    precision symbol table.
//!
//!  
//!
//! ```text
//!    symbol name          associated values
//!    --------------       -----------------
//!    BODY4_GM        -->   4.282628654899D4
//!    BODY4_POLE_DEC  -->   5.2886D1
//!                         -6.1D-2
//!                          0.0D0
//!    BODY4_POLE_RA   -->   3.17681D2
//!                         -1.08D-1
//!                          0.0D0
//! ```
//!
//!  This symbol table contains three symbols. One of the symbols, BODY4 GM,
//!    points to a single value. The other two symbols each point to three
//!    values.
//!
//!  
//!
//!
//!  
//! ##  Why use Symbol Tables?
//!
//!  The primary use of symbol tables is to implement associative arrays:
//!    that is, arrays which are indexed by character strings rather than by
//!    indices. For example, you may wish to store the masses for several
//!    planets and satellites, without knowing ahead of time which ones you
//!    will be using. You might use a double precision symbol table for this,
//!    storing the mass of Jupiter as element 'JUPITER', the mass of Europa as
//!    element 'EUROPA', and so on.
//!
//!  The fact that more than one value may be stored under a single name
//!    (symbol) allows you to store polynomials, vectors, matrices, or any set
//!    of associated values under one name. Examples from the SPICELIB kernel
//!    pool include: three axes for one body under the name 'BODYxxx AXES';
//!    polynomials for the right ascension and declination of the pole of a
//!    body (three terms each) under the name 'BODYxxx POLE RA' and 'BODYxxx
//!    POLE DEC'. Other examples might be to list the names of active
//!    satellites under the name of the parent planet; to list one or more
//!    vidicom matrices under the name 'K-MATRIX'; and to store scalar, vector,
//!    and matrix values in a simulated desk calculator.
//!
//!  In short, any time you need to store something and look it up later, you
//!    can use symbol tables. The advantages come into play mostly when the
//!    things to be stored are not known until run-time, or when a program is
//!    undergoing development and the things to be stored are subject to rapid
//!    change.
//!
//!  
//!
//!
//!  
//! ##  Symbol Subroutine Naming Conventions
//!
//!  The names of the symbol table subroutines in SPICELIB are assigned as
//!    follows. Each name is of the form SYfffx.
//!
//!  
//!
//! *  SY
//!
//!
//!  indicates that the subroutine belongs to the symbol table family of
//! subroutines.
//!
//!  *  fff
//!
//!
//!  is a three letter mnemonic code indicating the function of the
//! subroutine.
//!
//!  *  x
//!
//!
//!  indicates the data type of the values associated with the symbols in
//! the name table. The data types are: C for character, D for double
//! precision, and I for integer.
//!
//!  In the descriptive text that follows, the generic routines are referred
//!    to by their mnemonic codes, and specific routines are referred to by
//!    their full names. For example, the notation DEL refers to the generic
//!    routine, DEL, for deleting a symbol. The notation [SYDELC](crate::raw::sydelc) refers to the
//!    particular routine that deletes a symbol from a character symbol table.
//!
//!  
//!
//!
//!  
//! ##  Symbol Table Initialization
//!
//!  Symbol tables are implemented using cells, another SPICELIB data
//!    structure. Hence, the restrictions that apply to cells also apply to
//!    symbol tables. The size and cardinality of the components of a symbol
//!    table must be initialized before the symbol table can be used properly.
//!    The cell routines SSIZEx should be used for this initialization. Consult
//!    the Required Reading for the family of cell routines ([cells.req](crate::required_reading::cells)) if you
//!    are not familiar with their use.
//!
//!  Before using the symbol tables, you must initialize the name table,
//!    pointer table, and value table. This initialization sets the size and
//!    cardinality of the component tables.
//!
//!  The size of the name table must equal the size of the pointer table. In
//!    other words, both must contain the same number of elements. Also, the
//!    size of the value table should be large enough to accommodate the
//!    maximum number of values anticipated. If the size of any of the
//!    component tables of a symbol table is too small, it is treated as an
//!    error by the symbol table routines.
//!
//!  The cardinality of the component tables should be set to zero before
//!    using a symbol table.
//!
//!  The following piece of code demonstrates the easiest way to initialize a
//!    symbol table. Using the cell routines SSIZEx to create a symbol table
//!    containing up to thirty symbols and up to one hundred-fifty values, the
//!    initialization looks like this:
//!
//!  Initialize the name table:
//!
//!  
//!
//! ```text
//!    CALL SSIZEC ( 30,   TABSYM )
//! ```
//!
//!  Initialize the pointer table:
//!
//!  
//!
//! ```text
//!    CALL SSIZEI ( 30,   TABPTR )
//! ```
//!
//!  Initialize the value table:
//!
//!  
//!
//! ```text
//!    CALL SSIZEC ( 150,  TABVAL )
//! ```
//!
//!  The name table always contains character values and is initialized with [SSIZEC](crate::raw::ssizec). Likewise, the pointer table always contains integer values and
//!    is thus initialized with [SSIZEI](crate::raw::ssizei). The initialization of the value table
//!    is different for each of the types of symbol tables. In the example
//!    above the routine [SSIZEC](crate::raw::ssizec) was used to initialize the value table for a
//!    character symbol table. A double precision value table should be
//!    initialized using [SSIZED](crate::raw::ssized), and an integer values table should be
//!    initialized using [SSIZEI](crate::raw::ssizei).
//!
//!  
//!
//!
//!  
//! ##  A Comprehensive Example
//!
//!  The following examples illustrate how each symbol table routine is used.
//!    The first five examples illustrate how to create a symbol, delete a
//!    symbol, duplicate a symbol, rename a symbol, and fetch the name of a
//!    symbol. The next four examples demonstrate how to add a value, delete a
//!    value, obtain the values, and reorder the values associated with an
//!    existing symbol. The final example illustrates how to determine the
//!    number of values associated with a symbol.
//!
//!  
//!
//!
//!  
//! ###  Creating a symbol
//!
//!  Suppose that you want to create a symbol table of famous scientists and
//!    their fields of study. First, you must create a symbol and give it one
//!    or more associated values. SET and PUT create symbols. If you want to
//!    create a symbol with one value, use the SET routine. Otherwise, use the
//!    PUT routine. The routine used depends on the initial number of values
//!    associated with the symbol. The call below demonstrates how to create
//!    the symbol 'EINSTEIN' with the associated value 'BROWNIAN MOTION'.
//!    Because this symbol has one value, the SET routine should be used.
//!
//!  The call,
//!
//!  
//!
//! ```text
//!    CALL SYSETC
//!    ( 'EINSTEIN', 'BROWNIAN MOTION', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  creates the symbol table:
//!
//!  
//!
//! ```text
//!    EINSTEIN   -->  BROWNIAN MOTION
//! ```
//!
//!  To create a symbol giving it more than one value, use the PUT routine.
//!
//!  If the VALUES array contains the elements,
//!
//!  
//!
//! ```text
//!    ELECTRIC CHARGE
//!    PHOTOELECTRIC EFFECT
//! ```
//!
//!  N is 2 (the number of elements in the VALUES array), and the symbol you
//!    want to create is named 'MILLIKAN', the call,
//!
//!  
//!
//! ```text
//!    CALL SYPUTC
//!    ( 'MILLIKAN', VALUES, N, TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  creates a new symbol in the symbol table. The symbol table now looks
//!    like this:
//!
//!  
//!
//! ```text
//!    EINSTEIN   -->  BROWNIAN MOTION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//! ```
//!
//!  Imagine now that the symbol table has several symbols.
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!    HAHN       -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    PLANCK     -->  ELEMENTARY QUANTA
//! ```
//!
//!     
//! ###  Deleting a symbol
//!
//!  The routine DEL deletes a symbol from the symbol table.
//!
//!  The call,
//!
//!  
//!
//! ```text
//!    CALL SYDELC ( 'PLANCK', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  deletes the scientist PLANCK from the table. The symbol table now looks
//!    like this:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!    HAHN       -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//! ```
//!
//!     
//! ###  Duplicating a symbol
//!
//!  Perhaps after doing some research, you find that the scientist STRASSMAN
//!    also worked on NUCLEAR FISSION. You'd like to add him to the symbol
//!    table. Well, you can do this in two ways. You could create a symbol
//!    'STRASSMAN', or you could duplicate the symbol 'HAHN' and give it the
//!    name 'STRASSMAN' since their associated values are the same. The routine
//!    DUP duplicates a symbol.
//!
//!  Using the DUP routine,
//!
//!  
//!
//! ```text
//!    CALL SYDUPC ( 'HAHN', 'STRASSMAN', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  changes the symbol table contents to:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!    HAHN       -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!  The same results could have been achieved using the SET routine to
//!    create a symbol with one associated value, or the PUT routine if the
//!    symbol you wanted to create had more than one associated value. The call
//!    for creating the symbol 'STRASSMAN' with the value 'NUCLEAR FISSION'
//!    would look like this:
//!
//!  
//!
//! ```text
//!    CALL SYSETC
//!    ( 'STRASSMAN', 'NUCLEAR FISSION', TABSYM, TABPRT, TABVAL )
//! ```
//!
//!     
//! ###  Renaming a symbol
//!
//!  The routine REN exists for renaming a symbol.
//!
//!  Using the REN routine,
//!
//!  
//!
//! ```text
//!    CALL SYRENC ( 'HAHN', 'FERMI', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  the symbol 'HAHN' is renamed to 'FERMI'.
//!
//!  The symbol table now looks like this:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!     
//! ###  Obtaining the name of a symbol
//!
//!  The routine FET allows you to obtain the name of a particular symbol in
//!    the symbol table. Perhaps you want to know the names of the first four
//!    symbols in the symbol table. (Note that the FET routine does not modify
//!    the contents of the symbol table.)
//!
//!  The following code will 'fetch' and write to the screen the names of the
//!    first four symbols in the symbol table.
//!
//!  
//!
//! ```text
//!    DO I = 1, 4
//!       CALL SYFETC ( I, TABSYM, TABPTR, TABVAL, NAME, FOUND )
//!       WRITE (6,*) NAME
//!    END DO
//! ```
//!
//!     
//! ###  Adding a new value to a symbol
//!
//!  Suppose that you want to add a value to a symbol. This can be done by
//!    either 'pushing' or 'enqueueing' a value onto the symbol. Pushing a
//!    value onto a symbol means that the value becomes the first value
//!    associated with the symbol. Enqueueing the value onto the symbol means
//!    that the value becomes the last value associated with the symbol. The
//!    routines PSH or ENQ are used to add a value to the values already
//!    associated with a symbol.
//!
//!  If the call is,
//!
//!  
//!
//! ```text
//!    CALL SYPSHC
//!    ( 'EINSTEIN', 'GENERAL RELATIVITY', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  the contents of the symbol table are now:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  GENERAL RELATIVITY
//!                    BROWNIAN MOTION
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!  Let the next call be:
//!
//!  
//!
//! ```text
//!    CALL SYENQC
//!    ( 'EINSTEIN', 'PHOTOELECTRIC EFFECT', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  The contents of the symbol table are modified to be:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  TRANSISTOR EFFECT
//!                    SUPERCONDUCTIVITY
//!    EINSTEIN   -->  GENERAL RELATIVITY
//!                    BROWNIAN MOTION
//!                    PHOTOELECTRIC EFFECT
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!     
//! ###  Deleting a value from a symbol
//!
//!  The only value that can be deleted is the first value associated with a
//!    symbol. The first value associated with the symbol is deleted using the
//!    POP routine. The call below demonstrates how to 'pop' a value associated
//!    with the symbol 'BARDEEN'.
//!
//!  The call,
//!
//!  
//!
//! ```text
//!    CALL SYPOPC ( 'BARDEEN', TABSYM, TABPTR, TABVAL, VALUE, FOUND )
//! ```
//!
//!  results in the symbol table:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  SUPERCONDUCTIVITY
//!    EINSTEIN   -->  GENERAL RELATIVITY
//!                    BROWNIAN MOTION
//!                    PHOTOELECTRIC EFFECT
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!  If there are no remaining values associated with the symbol after VALUE
//!    has been popped, the symbol is removed from the symbol table.
//!
//!  
//!
//!
//!  
//! ###  Obtaining values associated with a symbol
//!
//!  Some symbol table routines exist to obtain values associated with a
//!    particular symbol. All of the values can be obtained, a subset of the
//!    values, or just a particular value associated with a symbol. The
//!    routines to do this are GET, SEL, and NTH respectively. These routines
//!    do not modify the symbol tables. To obtain all the values associated
//!    with the symbol 'EINSTEIN' use the GET routine.
//!
//!  Calling the GET routine,
//!
//!  
//!
//! ```text
//!    CALL SYGETC
//!    ( 'EINSTEIN', TABSYM, TABPTR, TABVAL, N, VALUES, FOUND )
//! ```
//!
//!  returns the following information about the symbol:
//!
//!  
//!
//! *  N
//!
//!
//!  the number of values returned
//!
//!  *  VALUES
//!
//!
//!  an array containing the symbol's values
//!
//!  *  FOUND
//!
//!
//!  indicates whether or not the symbol was found in the symbol table
//!
//!  The following information is returned for the symbol 'EINSTEIN':
//!
//!  
//!
//! ```text
//!    N        3
//!  
//!    VALUES   GENERAL RELATIVITY
//!             BROWNIAN MOTION
//!             PHOTOELECTRIC EFFECT
//!  
//!    FOUND    TRUE
//! ```
//!
//!     
//! ###  Reordering the values associated with a symbol
//!
//!  Two routines exist for reordering the values associated with a symbol.
//!    The routine ORD will order the values in increasing order from the first
//!    value to the last. Character values are ordered according to the ASCII
//!    collating sequence. The second routine, TRN, transposes two values
//!    associated with a symbol.
//!
//!  Calling the ORD routine to order the values associated with the symbol
//!    'EINSTEIN',
//!
//!  
//!
//! ```text
//!    CALL SYORDC ( 'EINSTEIN', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  the contents of the symbol table are modified to be:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!                    GENERAL RELATIVITY
//!                    PHOTOELECTRIC EFFECT
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  ELECTRIC CHARGE
//!                    PHOTOELECTRIC EFFECT
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!  In order to transpose the first and second value associated with the
//!    symbol 'MILLIKAN', use the TRN routine.
//!
//!  The call,
//!
//!  
//!
//! ```text
//!    CALL SYTRNC ( 'MILLIKAN', 1, 2, TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  Changes the symbol table to look like this:
//!
//!  
//!
//! ```text
//!    BARDEEN    -->  SUPERCONDUCTIVITY
//!    EINSTEIN   -->  BROWNIAN MOTION
//!                    GENERAL RELATIVITY
//!                    PHOTOELECTRIC EFFECT
//!    FERMI      -->  NUCLEAR FISSION
//!    MILLIKAN   -->  PHOTOELECTRIC EFFECT
//!                    ELECTRIC CHARGE
//!    STRASSMAN  -->  NUCLEAR FISSION
//! ```
//!
//!     
//! ###  Determining the dimension of a symbol
//!
//!  The integer function DIM exists for determining how many values are
//!    associated with a symbol. (Note that the DIM function does not modify
//!    the symbol table.)
//!
//!  The code,
//!
//!  
//!
//! ```text
//!    NUMSUB = SYDIMC ( 'EINSTEIN', TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  returns the value of 3 for NUMSUB.
//!
//!  
//!
//!
//!  
//! ##  Three Letter Mnemonics used in Subroutine Names
//!
//!  The following is a list of the three letter mnemonics and their related
//!    functions.
//!
//!  
//!
//! *  DEL
//!
//!
//!  delete a symbol
//!
//!  *  DIM
//!
//!
//!  return the dimension of a symbol (function)
//!
//!  *  DUP
//!
//!
//!  duplicate a symbol
//!
//!  *  ENQ
//!
//!
//!  enque a value onto an existing symbol
//!
//!  *  FET
//!
//!
//!  fetch the name of the Nth symbol
//!
//!  *  GET
//!
//!
//!  return all of the values associated with a symbol
//!
//!  *  NTH
//!
//!
//!  return the Nth value associated with a symbol
//!
//!  *  ORD
//!
//!
//!  order the values associated with a symbol
//!
//!  *  POP
//!
//!
//!  pop a value associated with a symbol
//!
//!  *  PSH
//!
//!
//!  push a value onto a symbol
//!
//!  *  PUT
//!
//!
//!  create a symbol with several associated values
//!
//!  *  REN
//!
//!
//!  rename an existing symbol
//!
//!  *  SET
//!
//!
//!  create a symbol with one associated value
//!
//!  *  SEL
//!
//!
//!  select a subset of values associated with a symbol
//!
//!  *  TRN
//!
//!
//!  transpose two values associated with a symbol
//!
//!     
//! ##  Calling Sequences
//!
//!  The following is a list of the calling sequences of the generic symbol
//!    table routines in SPICELIB.
//!
//!  Subroutines:
//!
//!  
//!
//! ```text
//!    SYDELx  ( NAME, TABSYM, TABPTR, TABVAL )
//!  
//!    SYDUPx  ( NAME, COPY, TABSYM, TABPTR, TABVAL )
//!  
//!    SYENQx  ( NAME, VALUE, TABSYM, TABPTR, TABVAL )
//!  
//!    SYFETx  ( NTH, TABSYM, TABPTR, TABVAL, NAME, FOUND )
//!  
//!    SYGETx  ( NAME, TABSYM, TABPTR, TABVAL, N, VALUES, FOUND )
//!  
//!    SYNTHx  ( NAME, NTH, TABSYM, TABPTR, TABVAL, VALUE, FOUND )
//!  
//!    SYORDx  ( NAME, TABSYM, TABPTR, TABVAL )
//!  
//!    SYPOPx  ( NAME, TABSYM, TABPTR, TABVAL, VALUE, FOUND )
//!  
//!    SYPSHx  ( NAME, VALUE, TABSYM, TABPTR, TABVAL )
//!  
//!    SYPUTx  ( NAME, VALUES, N, TABSYM, TABPTR, TABVAL )
//!  
//!    SYRENx  ( OLD, NEW, TABSYM, TABPTR, TABVAL )
//!  
//!    SYSELx  ( NAME, BEGIN, END, TABSYM, TABPTR, TABVAL, VALUES, FOUND )
//!  
//!    SYSETx  ( NAME, VALUE, TABSYM, TABPTR, TABVAL )
//!  
//!    SYTRNx  ( NAME, I, J, TABSYM, TABPTR, TABVAL )
//! ```
//!
//!  Functions:
//!
//!  
//!
//! ```text
//!    SYDIMx  ( NAME, TABSYM, TABPTR, TABVAL )
//! ```
//!
//!      
