//! #  Cells
//!
//!  Last revised on 2017 MAR 13 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  Cells are SPICE data structures that are vectors of type double
//!    precision, integer, or character type carrying with them their own
//!    dimension and knowledge of how many components have been used.
//!
//!  
//!
//!
//!  
//! ##  Introduction
//!
//!  A "cell array"' is an array dimensioned from LBCELL to CMAX, where
//!    LBCELL is the standard lower bound of a cell (currently -5) and CMAX is
//!    the maximum number of elements that the cell is allowed to contain at
//!    any one time---that is, the maximum cardinality of the cell.
//!
//!  A "cell"' is a cell array in which elements LBCELL through 0 contain
//!    information about whatever is stored in the rest of the array. CMAX is
//!    stored in this part of the cell, as is CCUR, the current cardinality of
//!    the cell. For character cells, these values are encoded into character
//!    strings. The SPICELIB cell's compact representation allows the user to
//!    declare, pass, and otherwise manipulate cells without having to keep
//!    track of separate pointers and dimensions for each cell array. Thus, a
//!    routine to merge the contents of two arrays into a third, when coded
//!    using cells, looks like this
//!
//!  
//!
//! ```text
//!    CALL MERGE ( OLD, NEW, TOTAL )
//! ```
//!
//!  instead of like this
//!
//!  
//!
//! ```text
//!    CALL MERGE ( OLD, N_OLD, NEW, N_NEW, MAX_TOTAL, TOTAL, N_TOTAL )
//! ```
//!
//!  This is especially convenient for arrays that need to be passed as
//!    arguments through several levels of subprograms. Frequently, such arrays
//!    are placed in common blocks to avoid the proliferation of pointers and
//!    dimensions in calling sequences. This also remedies one of the serious
//!    flaws in the implementation of Fortran arrays---the inability of a
//!    subprogram to determine the size of an argument array into which it is
//!    to place values. Since the size of a cell is always available,
//!    subroutines that manipulate cells can always check for overflow
//!    conditions.
//!
//!  
//!
//!
//!  
//! ##  Naming Conventions
//!
//!  The type of a cell is the same as the type of the cell array. In
//!    general, cell routines come in groups of three, the last letter of which
//!    indicates the cell type. We will refer to the generic routines by
//!    substituting an "x"' for the last letter. Thus, CARDx may be any of the
//!    following: [CARDC](crate::raw::cardc), [CARDD](crate::raw::cardd), or [CARDI](crate::raw::cardi). In specific contexts, we will use the
//!    specific names of routines. Before a cell can be used, it must be
//!    initialized. During initialization, the size is set to the maximum
//!    cardinality of the cell, the current cardinality is set to zero, and the
//!    remaining control information becomes undefined. A cell need be
//!    initialized only once, after which it may be used freely. (This assumes,
//!    of course, that routines which manipulate the cells remember to reset
//!    the cardinality when appropriate.)
//!
//!  The subroutine SSIZEx is used to initialize a cell, as shown below.
//!
//!  
//!
//! ```text
//!    INTEGER          LBCELL
//!    PARAMETER      ( LBCELL = -5 )
//!  
//!    DOUBLE PRECISION X    ( LBCELL:100 )
//!    DOUBLE PRECISION TEMP ( LBCELL:100 )
//!     .
//!     .
//!  
//!    CALL SSIZED ( 100, X    )
//!    CALL SSIZED ( 100, TEMP )
//! ```
//!
//!  We strongly recommend that you use a parameter, LBCELL, to declare the
//!    lower bounds of cells, as shown above.
//!
//!  
//!
//!
//!  
//! ##  Using Cells
//!
//!  SPICELIB cells may be populated using the APPNDx routines. These
//!    routines "append" a datum to a cell: they insert a specified data item
//!    into the lowest-indexed free slot in the data area of the cell array.
//!    The APPNDx routines automatically update the cell's cardinality to
//!    reflect the addition of the new datum.
//!
//!  To append three double precision numbers onto an empty double precision
//!    cell, we could use the code fragment
//!
//!  
//!
//! ```text
//!    INTEGER            LBCELL
//!    PARAMETER        ( LBCELL = -5 )
//!  
//!    DOUBLE PRECISION   X ( LBCELL : 100 )
//!  
//!    CALL SSIZED ( 100, X )
//!  
//!    CALL APPNDD ( 0.D0, X )
//!    CALL APPNDD ( 0.D0, X )
//!    CALL APPNDD ( 1.D0, X )
//! ```
//!
//!  Another subroutine SCARDx is used to adjust the cardinality of a cell.
//!    This is necessary when directly inserting items into or removing items
//!    from a cell, as shown below.
//!
//!  
//!
//! ```text
//!    CALL SSIZED ( 100, X )
//!  
//!    X(1) = 0.D0
//!    X(2) = 0.D0
//!    X(3) = 1.D0
//!  
//!    CALL SCARDD ( 3, X )
//! ```
//!
//!  SSIZEx and SCARDx should always be used in lieu of altering the contents
//!    of elements LBCELL through 0 directly.
//!
//!  The subroutine COPYx copies the elements of one cell to another cell.
//!    This can be useful for modifying temporary or working cells, or for
//!    saving copies of cells which are about to be changed. For example,
//!
//!  
//!
//! ```text
//!    INTEGER            LBCELL
//!    PARAMETER        ( LBCELL = -5 )
//!  
//!    DOUBLE PRECISION   X    ( LBCELL : 100 )
//!    DOUBLE PRECISION   TEMP ( LBCELL : 100 )
//!  
//!    CALL SSIZED ( 100, X    )
//!    CALL SSIZED ( 100, TEMP )
//!  
//!    CALL COPYD ( X, TEMP )
//! ```
//!
//!  copies the contents of X into TEMP. In this case, the cells are the same
//!    size (each can hold up to 100 elements), so the operation will always
//!    succeed. In general, if the output cell is not large enough to hold the
//!    contents of the input cell, as many elements as will fit are inserted
//!    into the output cell, and the SPICELIB error handling mechanism reports
//!    the number of excess elements.
//!
//!  An extra check is performed by [COPYC](crate::raw::copyc), which copies character cells. In
//!    order to avoid truncation problems, [COPYC](crate::raw::copyc) verifies that the operation
//!    can be performed without losing any of the non-blank characters in the
//!    original cell. The loss of one or more non-blank characters is reported
//!    through the SPICELIB error handling mechanism.
//!
//!  The integer function CARDx returns the cardinality of a cell. This may
//!    be used to determine whether a cell is empty or not. (The cardinality of
//!    an empty cell is zero.) It may also be used to assist in accessing the
//!    elements of a cell individually, as in the following example.
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
//!    cell. This is useful primarily for predicting situations in which
//!    overflow can occur, as in the following example.
//!
//!  
//!
//! ```text
//!    IF ( CARDC ( WINNERS ) .LE. SIZEC ( NOBEL ) ) THEN
//!       CALL COPYC ( WINNERS, NOBEL )
//!    ELSE
//!      .
//!      .
//!      .
//!    END IF
//! ```
//!
//!     
//! ##  Character Cells
//!
//!  As we mentioned earlier, the size and cardinality of a cell are stored
//!    in the control area of the cell (elements LBCELL through 0). For numeric
//!    data types, this is accomplished by simple assignment. However, in the
//!    case of cell arrays of type character, the values for the size and
//!    cardinality must be encoded into character strings.
//!
//!  This is done by storing the numbers in base CHBASE, where CHBASE is the
//!    number of distinct characters in the character set supported by the host
//!    machine and compiler. (In ASCII environments, CHBASE is always at least
//!    128, and may be as high as 256.) The numbers are encoded and decoded by
//!    subroutines [ENCHAR](crate::raw::enchar) and [DECHAR](crate::raw::dechar) respectively. The value of parameter
//!    MINLEN (declared in [ENCHAR](crate::raw::enchar)) constrains the minimum length of the
//!    elements in a cell array. The nominal value for MINLEN is 5. Given this
//!    value,
//!
//!  
//!
//! ```text
//!    INTEGER          LBCELL
//!    PARAMETER      ( LBCELL = -5 )
//!  
//!    CHARACTER*5      NAMES    ( LBCELL:1000 )
//! ```
//!
//!  is a legal cell declaration, while
//!
//!  
//!
//! ```text
//!    INTEGER          LBCELL
//!    PARAMETER      ( LBCELL = -5 )
//!  
//!    CHARACTER*4      NAMES    ( LBCELL:1000 )
//! ```
//!
//!  is not.
//!
//!  
//!
//!
//!  
//! ##  Cell-based data types
//!
//!  SPICELIB contains several extended data types based on cells. For
//!    example, one family of routines uses cells to implement algebraic sets
//!    of all types (character, integer, double precision). Another uses double
//!    precision cells to manipulate collections of closed intervals of the
//!    real numbers, called windows.
//!
//!  All of these data types are supported by routines designed to manipulate
//!    them. However, because they are based on cells, all of these data types
//!    can be manipulated by the general cell routines as well. Thus, COPYx can
//!    be used to copy sets and windows, just as it can be used to copy vanilla
//!    cells.
//!
//!  
//!
//!
//!  
//! #  Summary
//!
//!  The following table summarizes the SPICELIB cell routines.
//!
//!  
//!
//! *  SSIZEx ( SIZE, CELL )
//!
//!
//!  Initialize (set the size of) a cell.
//!
//!  *  SCARDx ( CARD, CELL )
//!
//!
//!  Set the cardinality of a cell.
//!
//!  *  SIZEx ( CELL )
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
//!  *  APPNDx ( ITEM, CELL )
//!
//!
//!  Append an item to a cell.
//!
//!     
//! #  Appendix A --- Revision History
//!
//!  
//!
//!
//!  
//! ###  2017 MAR 13 by N. J. Bachman
//!
//!  Moved revision history to this appendix.
//!
//!  Updated description of the element reference macros.
//!
//!  Updated references to CSPICE cell "append" routines to name individual
//!    routines rather than to refer to them using the notation
//!
//!  
//!
//! ```text
//!    appndx_c
//! ```
//!
//!  Fixed typos.
//!
//!  
//!
//!
//!  
//! ###  2002 SEP 04 by N. J. Bachman
//!
//!  Made minor changes to formatting and wording.
//!
//!  
//!
//!
//!     
