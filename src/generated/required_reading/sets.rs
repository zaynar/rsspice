//! #  Sets
//!
//!  Last revised on 2010 MAY 18 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  Sets are SPICE data structures that are a special case of SPICE cells --
//!    vectors of type double precision, integer, or character -- carrying with
//!    them their own dimension and knowledge of how many components have been
//!    used.
//!
//!  
//!
//!
//!  
//! ###  Revisions
//!
//!  September 04, 2002
//!
//!  
//!
//! *  Minor changes of formatting and wording were made. References to the
//! non-existent EMPTYx routines were deleted.
//!
//!  September 27, 1991
//!
//!  
//!
//! *  A typo in the previous version of March, 1990 has been corrected.
//!
//!     
//! ##  Introduction
//!
//!  The set data type is a subclass of the more basic SPICELIB cell data
//!    type. In order to understand and use sets, you must first understand how
//!    to use cells.
//!
//!  A "set" is a character, integer, or double precision cell in which the
//!    following restrictions are observed:
//!
//!  
//!
//! *  1. The elements of a set are distinct: sets never contain duplicate elements.
//! Character sets are case sensitive. For example, a set may contain all of
//! the following strings:
//!
//!  ```text
//!             'AB', 'Ab', 'aB', 'ab'.
//! ```
//!  *  2. The elements of a set are always stored contiguously in elements 1 through
//! N, where N is the cardinality of the set.
//!
//!  *  3. The elements are sorted in increasing order. Set routines come in groups of
//! three, one for character sets, one for double precision sets, and one for
//! integer sets. The name of each routine ends in C, D, or I, according to the
//! type of set upon which it operates.
//!
//!  Thus, [INSRTC](crate::raw::insrtc) inserts an element into a character set, [INSRTD](crate::raw::insrtd) inserts an
//!    element into a double precision set, and [INSRTI](crate::raw::insrti) inserts an element into
//!    an integer set. We will refer to the generic routines by substituting an
//!    x for the last letter. Thus, the routine ELEMx may refer to [ELEMC](crate::raw::elemc), [ELEMD](crate::raw::elemd), or [ELEMI](crate::raw::elemi). In specific contexts, we will use the specific names of
//!    routines.
//!
//!  
//!
//!
//!  
//! ##  Initialization
//!
//!  Like other cells, sets must be initialized before they can be used.
//!    Because it is frequently useful to pre-load the elements of a set, you
//!    can initialize a set from a non-empty array. This process, called
//!    validation, is done by the set routines [VALIDC](crate::raw::validc), [VALIDD](crate::raw::validd) and [VALIDI](crate::raw::validi). In
//!    the following example,
//!
//!  
//!
//! ```text
//!     INTEGER      LBCELL
//!     PARAMETER  ( LBCELL = -5 )
//!  
//!     INTEGER      BODIES  ( LBCELL:100 )
//!  
//!     DATA       ( BODIES(I), I = 1, 8 )   /  3, 301,
//!    .                                        3, 399,
//!    .                                        5, 501,
//!    .                                        6, 601   /
//!  
//!     CALL VALIDI ( 100, 8, BODIES )
//! ```
//!
//!  the integer set BODIES is validated. The size of BODIES is set to 100.
//!    The eight elements of the array (stored in elements 1 through 8) are
//!    ordered internally; duplicate elements (in this case, the number 3,
//!    which appears twice) are removed; and the cardinality of the set becomes
//!    the number of distinct elements, seven. The set is now ready for use
//!    with the remaining set routines.
//!
//!  The original contents of elements LBCELL through 0 are destroyed during
//!    validation.
//!
//!  Validation of an array is useful primarily for creating sets from arrays
//!    initialized in DATA statements or filled via input operations. Because
//!    the array is ordered during validation, the array may contain duplicate
//!    elements, and may be unsorted (or, more precisely, sorted according to
//!    some other, possibly more meaningful, criteria).
//!
//!  
//!
//!
//!  
//! ##  Cell routines
//!
//!  A set is by definition a special kind of cell. Thus any of the general
//!    cell routines may be used with sets. Sets may be copied using COPYx, and
//!    the cardinality of a set may be determined by using CARDx. The APPNDx
//!    routines may be used to add elements to a SPICELIB set, provided the set
//!    is validated prior to use.
//!
//!  An example of using the SPICELIB cardinality functions to define a loop
//!    bound:
//!
//!  
//!
//! ```text
//!    WRITE (6,*) 'Winners of the Nobel Prize for Physics:'
//!  
//!    DO I = 1, CARDC ( NOBEL )
//!       WRITE (6,*) NOBEL(I)
//!    END DO
//! ```
//!
//!  The integer function SIZEx returns the size (maximum cardinality) of a
//!    set. This is useful primarily for predicting situations in which
//!    overflow can occur.
//!
//!  
//!
//!
//!  
//! ##  Unary Routines
//!
//!  Unary routines operate on a single set. Two unary operations are
//!    supported, both of which may alter the contents of the input set.
//!
//!  
//!
//! *  1. The insertion of an element into a set.
//!
//!  *  2. The removal of an element from a set.
//!
//!  In the following example, the element
//!
//!  
//!
//! ```text
//!    'PLUTO'
//! ```
//!
//!  is removed from the character set PLANETS and inserted into the
//!    character set ASTEROIDS.
//!
//!  
//!
//! ```text
//!    CALL REMOVC ( 'PLUTO', PLANETS   )
//!    CALL INSRTC ( 'PLUTO', ASTEROIDS )
//! ```
//!
//!  If
//!
//!  
//!
//! ```text
//!    'PLUTO'
//! ```
//!
//!  is not an element of the set PLANETS, then the contents of PLANETS are
//!    not changed. Similarly, if
//!
//!  
//!
//! ```text
//!    'PLUTO'
//! ```
//!
//!  is already an element of ASTEROIDS, the contents of ASTEROIDS remain
//!    unchanged.
//!
//!  If a set is not large enough to accommodate the insertion of an element,
//!    the SPICELIB error handling mechanism reports the excess.
//!
//!  
//!
//!
//!  
//! ##  Binary Routines
//!
//!  Binary routines operate on two input sets to produce a third (distinct)
//!    output set. The four major algebraic binary set operations are
//!    supported: UNION, INTERSECTION, DIFFERENCE, and SYMMETRIC DIFFERENCE.
//!
//!  The UNION of two sets contains every element which is in the first set,
//!    or in the second set, or in both sets.
//!
//!  
//!
//! ```text
//!    {a,b}        U       {c,d}       =    {a,b,c,d}
//!    {a,b,c}      U       {b,c,d}     =    {a,b,c,d}
//!    {a,b,c,d}    U       {}          =    {a,b,c,d}
//!    {}           U       {a,b,c,d}   =    {a,b,c,d}
//!    {}           U       {}          =    {}
//! ```
//!
//!  The INTERSECTION of two sets contains every element which is in both the
//!    first set AND in the second set.
//!
//!  
//!
//! ```text
//!    {a,b}        *       {c,d}       =    {}
//!    {a,b,c}      *       {b,c,d}     =    {b,c}
//!    {a,b,c,d}    *       {}          =    {}
//!    {}           *       {a,b,c,d}   =    {}
//!    {}           *       {}          =    {}
//! ```
//!
//!  The DIFFERENCE of two sets contains every element which is in the first
//!    set, but NOT in the second.
//!
//!  
//!
//! ```text
//!    {a,b}        -       {c,d}       =    {a,b}
//!    {a,b,c}      -       {b,c,d}     =    {a}
//!    {a,b,c,d}    -       {}          =    {a,b,c,d}
//!    {}           -       {a,b,c,d}   =    {}
//!    {}           -       {}          =    {}
//! ```
//!
//!  The SYMMETRIC DIFFERENCE of two sets contains every element which is in
//!    the first set OR in the second set, but NOT in both sets.
//!
//!  
//!
//! ```text
//!    {a,b}        ^       {c,d}       =    {a,b,c,d}
//!    {a,b,c}      ^       {b,c,d}     =    {a,d}
//!    {a,b,c,d}    ^       {}          =    {a,b,c,d}
//!    {}           ^       {a,b,c,d}   =    {a,b,c,d}
//!    {}           ^       {}          =    {}
//! ```
//!
//!  Each of the routines takes two input sets and returns an output set.
//!
//!  The following calls
//!
//!  
//!
//! ```text
//!    CALL UNIONC ( PLANETS, ASTEROIDS, RESULT )
//!    CALL INTERC ( PLANETS, ASTEROIDS, RESULT )
//!    CALL DIFFC  ( PLANETS, ASTEROIDS, RESULT )
//!    CALL SDIFFC ( PLANETS, ASTEROIDS, RESULT )
//! ```
//!
//!  respectively place the union, intersection, difference, and symmetric
//!    difference of the character sets PLANETS and ASTEROIDS into the
//!    character set RESULT.
//!
//!  In each case, if the output set RESULT is not large enough to hold the
//!    result of the operation, as many elements as will fit are inserted into
//!    the set, and the SPICELIB error handling mechanism reports the excess.
//!
//!  In each of the binary routines, the output set must be distinct from
//!    both of the input sets. (All four of the binary operations can be
//!    performed in place, but not efficiently. Consequently, for the sake of
//!    consistency, none of the routines work in place.) For example, the
//!    following calls are invalid.
//!
//!  
//!
//! ```text
//!    CALL UNIONI ( CURRENT, NEW,     CURRENT )
//!    CALL INTERI ( NEW,     CURRENT, CURRENT )
//! ```
//!
//!  In each of the examples above, the subroutine may or may not return an
//!    error. However, the results will almost certainly be wrong.
//!
//!  
//!
//!
//!  
//! ##  Comparison Routines
//!
//!  The comparison routines implement the following tests.
//!
//!  
//!
//! *  1. Is a given item a member of a set?
//!
//!  *  2. Does a given relationship exist between two sets?
//!
//!  In the first case, the LOGICAL functions [ELEMC](crate::raw::elemc), [ELEMD](crate::raw::elemd), and [ELEMI](crate::raw::elemi) are
//!    true whenever the specified item is an element of the specified set, and
//!    are false otherwise. Let the character sets PLANETS and ASTEROIDS
//!    contain the following elements.
//!
//!  
//!
//! ```text
//!    PLANETS            ASTEROIDS
//!    --------           ----------
//!    'Earth'            'Apollo'
//!    'Mars'             'Ceres'
//!    'Pluto'
//!    'Venus'
//! ```
//!
//!  Then all of the following expressions are true.
//!
//!  
//!
//! ```text
//!    ELEMC ( 'Earth',  PLANETS   )
//!    ELEMC ( 'Pluto',  PLANETS   )
//!    ELEMC ( 'Ceres',  ASTEROIDS )
//! ```
//!
//!  And all of the following expressions are false.
//!
//!  
//!
//! ```text
//!    ELEMC ( 'Saturn', PLANETS   )
//!    ELEMC ( 'Pluto',  ASTEROIDS )
//!    ELEMC ( 'CERES',  ASTEROIDS )
//! ```
//!
//!  The LOGICAL functions [SETC](crate::raw::setc), [SETD](crate::raw::setd), and [SETI](crate::raw::seti) are true whenever the
//!    specified relationship between two sets exists, and are false otherwise.
//!
//!  In the following example, [SETI](crate::raw::seti) is used to repeat an operation for as
//!    long as the integer set FINISHED remains a proper subset of the integer
//!    set PLANNED.
//!
//!  
//!
//! ```text
//!    DO WHILE ( SETI ( FINISHED, '<', PLANNED ) )
//!     .
//!     .
//!    END DO
//! ```
//!
//!  The full list of valid operators is given below.
//!
//!  
//!
//! ```text
//!    Operator     is read
//!    --------     ---------------------------------------------
//!    '='          "is equal to (contains the same elements as)"
//!    '<>'         "is not equal to"
//!    '<='         "is a subset of"
//!    '<'          "is a proper subset of"
//!    '>='         "is a superset of"
//!    '>'          "is a proper superset of"
//! ```
//!
//!  Let the integer sets A, B, and C contain the following elements. Let E
//!    be an empty integer set.
//!
//!  
//!
//! ```text
//!    A        B        C
//!    ---      ---      ---
//!    1        1        1
//!    2        3        3
//!    3
//!    4
//! ```
//!
//!  Then all of the following expressions are true.
//!
//!  
//!
//! ```text
//!    SETI ( B, '=',  C )      "B is equal to C"
//!    SETI ( A, '<>', C )      "A is not equal to C"
//!    SETI ( A, '>',  B )      "A is a proper superset of B"
//!    SETI ( B, '<=', C )      "B is a subset of C"
//!    SETI ( C, '<=', B )      "C is a subset of B"
//!    SETI ( A, '<=', A )      "A is a subset of A"
//!    SETI ( E, '<=', B )      "E is a subset of B"
//!    SETI ( E, '<',  B )      "E is a proper subset of B"
//!    SETI ( E, '<=', E )      "E is a subset of E"
//! ```
//!
//!  And all of the following are false.
//!
//!  
//!
//! ```text
//!    SETI ( B, '<>',  C )      "B is not equal to C"
//!    SETI ( A, '=',   C )      "A is equal to C"
//!    SETI ( A, '<',   B )      "A is a proper subset of B"
//!    SETI ( B, '<',   C )      "B is a proper subset of C"
//!    SETI ( B, '>=',  A )      "B is a superset of A"
//!    SETI ( A, '>',   A )      "A is a proper superset of A"
//!    SETI ( E, '>=',  A )      "E is a superset of A"
//!    SETI ( E, '<',   E )      "E is a proper subset of E"
//! ```
//!
//!     
//! ##  Summary
//!
//!  The following table summarizes the set routines in the SPICELIB library.
//!
//!  
//!
//!
//!  
//! ###  Initialization
//!
//!  
//!
//! *  VALIDx ( SIZE, N, SET )
//!
//!
//!  Validate a set from an array.
//!
//!     
//! ###  Utilities
//!
//!  
//!
//! *  SIZEx ( CELL )
//!
//!
//!  Return the size of a cell.
//!
//!  *  CARDx ( CELL )
//!
//!
//!  Return the cardinality of a cell.
//!
//!  *  COPYx ( ORIG, COPY )
//!
//!
//!  Copy the contents of a cell.
//!
//!     
//! ###  Unary
//!
//!  
//!
//! *  INSRTx ( ITEM, SET )
//!
//!
//!  Insert an item into a set.
//!
//!  *  REMOVx ( ITEM, SET )
//!
//!
//!  Remove an item from a set.
//!
//!     
//! ###  Binary
//!
//!  
//!
//! *  UNIONx ( A, B, C )
//!
//!
//!  Take the union of two sets.
//!
//!  *  INTERx ( A, B, C )
//!
//!
//!  Take the intersection of two sets.
//!
//!  *  DIFFx ( A, B, C )
//!
//!
//!  Take the difference of two sets.
//!
//!  *  SDIFFx ( A, B, C )
//!
//!
//!  Take the symmetric difference of two sets.
//!
//!     
//! ###  Comparison
//!
//!  
//!
//! *  ELEMx ( ITEM, SET)
//!
//!
//!  Is an item in a set?
//!
//!  *  SETx ( A, REL, B )
//!
//!
//!  What is the relationship between two sets? Set relationships are listed
//! below.
//!
//!     
//! ###  Set Relationships
//!
//!  
//!
//! ```text
//!    =      is equal to (contains the same elements as)
//!  
//!    <>     is not equal to
//!  
//!    <=     is a subset of
//!  
//!    <      is a proper subset of
//!  
//!    >=     is a superset of
//!  
//!    >      is a proper superset of
//! ```
//!
//!      
