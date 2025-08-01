//! #  Windows Required Reading
//!
//!  Last revised on 2010 MAY 18 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  SPICE Toolkits define the SPICE data type "window" for use in the
//!    manipulation of continuous intervals on the real line. Corresponding
//!    window routines in SPICELIB are designed to simplify the use of this
//!    data type.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  By definition, a window is a double precision SPICE cell that contains
//!    zero or more intervals.
//!
//!  An interval is an ordered pair of numbers,
//!
//!  
//!
//! ```text
//!    [ a(i), b(i) ]
//! ```
//!
//!  such that
//!
//!  
//!
//! ```text
//!    a(i)  <  b(i)
//!          -
//! ```
//!
//!  The intervals within a window are both ordered and disjoint. That is,
//!    the beginning of each interval is greater than the end of the previous
//!    interval:
//!
//!  
//!
//! ```text
//!    b(i)  <  a(i+1)
//! ```
//!
//!  This restriction is enforced primarily because it allows efficient
//!    window operations.
//!
//!  The intervals stored in windows typically represent intervals of time
//!    (seconds, days, or centuries past a reference epoch). However, windows
//!    can represent any kinds of intervals.
//!
//!  
//!
//!
//!  
//! ###  Basic Concepts
//!
//!  "size" - The maximum number of elements storable in a SPICE cell.
//!    Cells are created with a constant size.
//!
//!  "cardinality - The number of elements stored in a cell. "cardinality"
//!    describes how much of "size" is used. "cardinality" satisfies the
//!    relationship:
//!
//!  
//!
//! ```text
//!       cardinality < size
//!                   -
//! ```
//!
//!  "measure" - the measure of an interval is the length of the interval,
//!    that is the difference of its endpoints:
//!
//!  
//!
//! ```text
//!    m(i) = b(i) - a(i)
//! ```
//!
//!  Note that the singleton interval
//!
//!  
//!
//! ```text
//!    [ a(i), a(i) ]
//! ```
//!
//!  has measure zero. The window
//!
//!  
//!
//! ```text
//!    [1,2], [4,9], [16,16]
//! ```
//!
//!  contains intervals of measure 1, 5, and 0 respectively.
//!
//!  The concept of measure extends readily to the gaps between adjacent
//!    intervals. In the example above, the window contains gaps of measure 2
//!    and 7. Intervals separated by gaps of measure zero or less are said to
//!    overlap. Overlapping intervals created by the window routines are merged
//!    as soon as they are created.
//!
//!  Finally, the measure of a window is the sum of the measures of its
//!    intervals. In the example above, the measure of the window is 6. Note
//!    that a floating point window containing only singletons has measure
//!    zero.
//!
//!  
//!
//!
//!  
//! ##  The window data type
//!
//!  SPICE windows are a form of the SPICE cell data type \[1], so any
//!    restriction that applies to the use of cells applies to the use of
//!    windows as well. The implementation of a SPICE window differs according
//!    to the Toolkit language.
//!
//!  The SPICELIB implementation of SPICE windows consists of Fortran double
//!    precision cells.
//!
//!  Example:
//!
//!  
//!
//! ```text
//!    C
//!    C     Define our variable types.
//!    C
//!          INTEGER           LBCELL
//!          PARAMETER        (LBCELL = -5 )
//!  
//!          INTEGER           MAXSIZ
//!          PARAMETER        (MAXSIZ = 8 )
//!  
//!    C
//!    C     Define a cell WIN to use as a window.
//!    C
//!    C     The window can hold eight (MAXSIZ = 8) double precision values,
//!    C     thus four intervals.
//!    C
//!          DOUBLE PRECISION  WIN(LBCELL:MAXSIZ)
//! ```
//!
//!  The cell size, 'MAXSIZ', must have an even value to use the cell as a
//!    window. Two values define a window interval, so a window of N intervals
//!    requires a cell of size 2*N.
//!
//!  The size and cardinality of a window must be initialized (using the cell
//!    routines [SSIZED](crate::raw::ssized) and [SCARDD](crate::raw::scardd)) before the window may be used by any of the
//!    SPICELIB window routines.
//!
//!  Any of the general cell routines in SPICELIB may be used with SPICE
//!    windows. For example, [COPYD](crate::raw::copyd) may be used to copy the contents of one
//!    window into another. The function [CARDD](crate::raw::cardd) may be used to determine the
//!    number of endpoints (that is, twice the number of intervals) in a
//!    window.
//!
//!  All errors are reported via standard SPICELIB error handling.
//!
//!  With the exception of the initialization routines, all window routines
//!    assume that input cells do contain valid windows---that is, ordered and
//!    distinct sets of endpoints. The windows subsystem may not signal errors
//!    resulting from attempts to operate on invalid windows.
//!
//!  
//!
//!
//!  
//! ##  References
//!
//!  
//!
//! *  \[1]
//!
//!
//!  CELLS Required Reading ([cells.req](crate::required_reading::cells)).
//!
//!     
//! #  Window Routines
//!
//!  The window routines in SPICELIB fall into the following categories.
//!
//!  
//!
//! * Initialization
//!
//!  * Unary
//!
//!  * Binary
//!
//!  * Complement
//!
//!  * Comparison
//!
//!     
//! ##  Initialization Routines
//!
//!  [WNVALD](crate::raw::wnvald) takes as input a double precision SPICE cell containing pairs of
//!    endpoints and validates it to form a window. The calling sequence is
//!    shown below.
//!
//!  
//!
//! ```text
//!    WNVALD ( SIZE, N, WINDOW )
//! ```
//!
//!  On input, WINDOW is a cell of size SIZE containing N endpoints. During
//!    validation, the intervals are ordered, and overlapping intervals are
//!    merged. On output, the cardinality of WINDOW is the number of endpoints
//!    remaining, and the window is ready for use with any of the window
//!    routines.
//!
//!  Because validation is done in place, there is no chance of overflow.
//!    However, other errors may be detected. For example, if the left endpoint
//!    of any interval is greater than the corresponding right endpoint, [WNVALD](crate::raw::wnvald)
//!    signals an error.
//!
//!  Validation is primarily useful for ordering and merging intervals added
//!    to a cell by [APPNDD](crate::raw::appndd), or directly assigned to the cell.
//!
//!  Building a large window is done most efficiently by assigning the window
//!    elements and then calling [WNVALD](crate::raw::wnvald). Building up the window by repeated
//!    insertion requires repeated ordering operations; [WNVALD](crate::raw::wnvald) does a single
//!    ordering operation.
//!
//!  
//!
//!
//!  
//! ##  Unary Routines
//!
//!  Each unary routine operates on a single window. Six unary operations are
//!    supported, each of which alters the contents of the input window. The
//!    calling sequences are shown below.
//!
//!  
//!
//! ```text
//!    WNCOND ( LEFT, RIGHT, WINDOW )        { Contract }
//!  
//!    WNEXPD ( LEFT, RIGHT, WINDOW )        { Expand }
//!  
//!    WNEXTD ( SIDE, WINDOW )               { Extract }
//!  
//!    WNFILD ( SMALL, WINDOW )              { Fill }
//!  
//!    WNFLTD ( SMALL, WINDOW )              { Filter }
//!  
//!    WNINSD ( LEFT, RIGHT, WINDOW )        { Insert }
//! ```
//!
//!  Each of the unary window routines works in place. That is, only one
//!    window, WINDOW, appears in each calling sequence, serving as both input
//!    and output. Windows whose original contents need to be preserved should
//!    be copied prior to calling any of the unary routines.
//!
//!  [WNINSD](crate::raw::wninsd) inserts the interval whose endpoints are LEFT and RIGHT into
//!    WINDOW. If the input interval overlaps any of the intervals in the
//!    window, the intervals are merged. Thus, the cardinality of WINDOW can
//!    actually decrease as the result of an insertion.
//!
//!  [WNEXPD](crate::raw::wnexpd) and [WNCOND](crate::raw::wncond) expand (lengthen) and contract (shorten) each of the
//!    intervals in WINDOW. The adjustments are not necessarily symmetric. That
//!    is, [WNEXPD](crate::raw::wnexpd) works by subtracting LEFT units from the left endpoint of
//!    each interval and adding RIGHT units to the right endpoint of each
//!    interval. [WNCOND](crate::raw::wncond) is the same as EXP with the signs of the arguments
//!    reversed, and is primarily provided for clarity in coding. (Expansion by
//!    negative increments is a messy concept.) Intervals are merged when
//!    expansion causes them to overlap. Intervals are dropped when they are
//!    contracted by amounts greater than their measures.
//!
//!  [WNFLTD](crate::raw::wnfltd) and [WNFILD](crate::raw::wnfild) remove small intervals and small gaps between adjacent
//!    intervals. Both routines take as input a minimum measure, SMALL. [WNFLTD](crate::raw::wnfltd)
//!    filters out (drops) intervals with measures less than or equal to SMALL,
//!    while [WNFILD](crate::raw::wnfild) merges adjacent intervals separated by gaps with measures
//!    less than or equal to SMALL.
//!
//!  Depending on the value of SIDE, [WNEXTD](crate::raw::wnextd) extracts the left or right
//!    endpoints of each interval in WINDOW. The resulting window contains only
//!    the singleton intervals
//!
//!  
//!
//! ```text
//!    [ a(1), a(1) ], ..., [ a(n), a(n) ]
//! ```
//!
//!  or
//!
//!  
//!
//! ```text
//!    [ b(1), b(1) ], ..., [ b(n), b(n) ]
//! ```
//!
//!     
//! ##  Binary Routines
//!
//!  Binary routines operate on two input windows to produce a third
//!    (distinct) output window. Three major binary operations are supported.
//!    The calling sequences are shown below.
//!
//!  
//!
//! ```text
//!    WNUNID ( A, B, C )           { Union }
//!  
//!    WNINTD ( A, B, C )           { Intersection }
//!  
//!    WNDIFD ( A, B, C )           { Difference }
//! ```
//!
//!  In contrast with the unary routines, none of the binary routines work in
//!    place. The output window, C, must be distinct from both of the input
//!    windows, A and B. We will have more to say about this later on.
//!
//!  [WNUNID](crate::raw::wnunid) places the union of A and B into C. The union of two windows
//!    contains every point that is contained in the first window, or in the
//!    second window, or in both windows.
//!
//!  [WNINTD](crate::raw::wnintd) places the intersection of A and B into C. The intersection of
//!    two windows contains every point that is contained in the first window
//!    AND in the second.
//!
//!  [WNDIFD](crate::raw::wndifd) places the difference of A and B into C. The difference of two
//!    windows contains every point that is contained in the first window, but
//!    NOT in the second.
//!
//!  In each case, if the output window, C, is not large enough to hold the
//!    result of the operation, as many intervals as will fit are inserted into
//!    the window, and a SPICE error is signaled. (You can control the effect
//!    of this error on your program; refer to Error Required Reading.)
//!
//!  In each of the binary routines, the output window must be distinct from
//!    both of the input windows. All three of the binary operations can, in
//!    principle, be performed in place, but not all can be performed
//!    efficiently. Consequently, for the sake of consistency, none of the
//!    routines work in place. For example, the following calls are invalid.
//!
//!  
//!
//! ```text
//!    WNINTD ( A, B, A )
//!  
//!    WNINTD ( A, B, B )
//!  
//! ```
//!
//!  In each of the examples above, whether or not the subroutine signals an
//!    error, the results will almost certainly be wrong. Nearly the same
//!    effect can be achieved, however, by placing the result into a temporary
//!    window, which can be immediately copied back into one of the input
//!    windows, as shown below.
//!
//!  
//!
//! ```text
//!    WNINTD ( A, B, TEMP )
//!  
//!    COPYD ( TEMP, A )
//! ```
//!
//!     
//! ##  Complement Routines
//!
//!  [WNCOMD](crate::raw::wncomd) determines the complement of a window with respect to an
//!    interval. The calling sequence is
//!
//!  
//!
//! ```text
//!    WNCOMD ( LEFT, RIGHT, A, C )         { Complement }
//! ```
//!
//!  As with the binary routines, the output window, C, must be distinct from
//!    the input window, A.
//!
//!  Mathematically, the complement of a window contains those points that
//!    are not contained in the window. That is, the complement of the set of
//!    closed intervals
//!
//!  
//!
//! ```text
//!    [ a(1), b(1) ], [ a(2), b(2) ], ..., [ a(n), b(n) ]
//! ```
//!
//!  is the set of open intervals
//!
//!  
//!
//! ```text
//!    ( -inf, a(1) ), ( b(1), a(2) ), ..., ( b(n), +inf )
//! ```
//!
//!  Not all computer languages offer a satisfactory way to represent
//!    infinity, so [WNCOMD](crate::raw::wncomd) must take the complement with respect to a finite
//!    interval.
//!
//!  Since the results of a window routine must be another window, [WNCOMD](crate::raw::wncomd)
//!    returns the closure of the set theoretical complement. In short, the
//!    double precision complement of the window
//!
//!  
//!
//! ```text
//!    [ a(1), b(1) ], [ a(2), b(2) ], ..., [ a(n), b(n) ]
//! ```
//!
//!  with respect to the interval from LEFT to RIGHT is the intersection of
//!    the windows
//!
//!  
//!
//! ```text
//!    ( -inf, a(1) ], [ b(1), a(2) ], ..., [ b(n), +inf )
//! ```
//!
//!  and \[ LEFT, RIGHT ].
//!
//!  Intervals of measure zero (singleton intervals) in the original window
//!    are replaced by gaps of measure zero, which are filled. Thus,
//!    complementing a window twice does not necessarily yield the original
//!    window.
//!
//!  
//!
//!
//!  
//! ##  Comparison Routines
//!
//!  Comparison routines allow the contents of windows to be compared against
//!    the contents of other windows. There are four comparison routines: three
//!    logical functions and one subroutine. The calling sequences are shown
//!    below.
//!
//!  
//!
//! ```text
//!    WNELMD ( POINT, WINDOW )                         { Element }
//!  
//!    WNINCD ( LEFT, RIGHT, WINDOW )                   { Inclusion }
//!  
//!    WNRELD ( A, OP, B )                              { Relation }
//!  
//!    WNSUMD ( WINDOW, MEAS, AVG, STDDEV, SHORT, LONG) { Summary }
//! ```
//!
//!  [WNELMD](crate::raw::wnelmd) returns true if the input point, POINT, is an element of the
//!    input window, WINDOW---that is, whenever the point lies within one of
//!    the intervals of the window.
//!
//!  Similarly, [WNINCD](crate::raw::wnincd) is true whenever the input interval, from LEFT to
//!    RIGHT, is included in the input window, WINDOW---that is, whenever the
//!    interval lies entirely within one of the intervals of the window.
//!
//!  [WNRELD](crate::raw::wnreld) is true whenever a specified relationship between the input
//!    windows, A and B, is satisfied. Each relationship corresponds to a
//!    comparison operator, OP. The complete set of operators recognized by
//!    [WNRELD](crate::raw::wnreld) is shown below.
//!
//!  
//!
//! ```text
//!    '='          is equal to (contains the same intervals as)
//!    '<>'         is not equal to
//!    '<='         is a subset of
//!    '<'          is a proper subset of
//!    '>='         is a superset of
//!    '>'          is a proper superset of
//! ```
//!
//!  For example, the expression
//!
//!  
//!
//! ```text
//!    WNRELD ( NEEDED, '<=', AVAIL )
//!  
//! ```
//!
//!  is true whenever the window NEEDED is a subset of the window AVAIL. One
//!    window is a subset of another window if each of the intervals in the
//!    first window is included in one of the intervals in the second window.
//!    In addition, the first window is a proper subset of the second if the
//!    second window contains at least one point not contained in the first
//!    window. The following pairs of expressions are equivalent.
//!
//!  
//!
//! ```text
//!    WNRELD ( A, '>', B )
//!    WNRELD ( B, '<', A )
//!  
//!    WNRELD ( A, '>=', B )
//!    WNRELD ( B, '<=', A )
//! ```
//!
//!  [WNSUMD](crate::raw::wnsumd) provides a summary of the input window, WINDOW. It computes the
//!    measure of the window, MEAS, and the average, AVG, and standard
//!    deviation, STDDEV, of the measures of the individual intervals in the
//!    window. It also returns the indices of the left endpoints of the
//!    shortest and longest intervals in the window. All of these quantities
//!    and indices are zero if the window contains no intervals.
//!
//!  The following describes the relation of SHORT and LONG to the window
//!    data:
//!
//!  The left endpoint of the shortest interval has value:
//!
//!  
//!
//! ```text
//!    WINDOW(SHORT)
//! ```
//!
//!  The right endpoint of the shortest interval has value:
//!
//!  
//!
//! ```text
//!    WINDOW(SHORT+1)
//! ```
//!
//!  The left endpoint of the longest interval has value:
//!
//!  
//!
//! ```text
//!    WINDOW(LONG)
//! ```
//!
//!  The right endpoint of the longest interval has value:
//!
//!  
//!
//! ```text
//!    WINDOW(LONG+1)
//! ```
//!
//!     
//! #  Summary
//!
//!  The following is a summary of the window routines in SPICELIB. (DP =
//!    double precision)
//!
//!  
//!
//! *  [WNCARD](crate::raw::wncard) - Cardinality of a DP window
//!
//!  *  [WNCOMD](crate::raw::wncomd) - Complement a DP window
//!
//!  *  [WNCOND](crate::raw::wncond) - Contract the intervals of a DP window
//!
//!  *  [WNDIFD](crate::raw::wndifd) - Difference two DP windows
//!
//!  *  [WNELMD](crate::raw::wnelmd) - Element of a DP window
//!
//!  *  [WNEXPD](crate::raw::wnexpd) - Expand the intervals of a DP window
//!
//!  *  [WNEXTD](crate::raw::wnextd) - Extract the endpoints from a DP window
//!
//!  *  [WNFETD](crate::raw::wnfetd) - Fetch an interval from a DP window
//!
//!  *  [WNFILD](crate::raw::wnfild) - Fill small gaps in a DP window
//!
//!  *  [WNFLTD](crate::raw::wnfltd) - Filter small intervals from a DP window
//!
//!  *  [WNINCD](crate::raw::wnincd) - Included in a DP window
//!
//!  *  [WNINSD](crate::raw::wninsd) - Insert an interval into a DP window
//!
//!  *  [WNINTD](crate::raw::wnintd) - Intersect two DP windows
//!
//!  *  [WNRELD](crate::raw::wnreld) - Compare two DP windows
//!
//!  *  [WNSUMD](crate::raw::wnsumd) - Summary of a DP window
//!
//!  *  [WNUNID](crate::raw::wnunid) - Union two DP windows
//!
//!  *  [WNVALD](crate::raw::wnvald) - Validate a DP window
//!
//!     
//! #  Appendix: Document Revision History
//!
//!  
//!
//!
//!  
//! ###  Febuary 6, 2009 (EDW)
//!
//!  Document edited to conform to current NAIF format for Required Reading
//!    Documents.
//!
//!  
//!
//!
//!     
