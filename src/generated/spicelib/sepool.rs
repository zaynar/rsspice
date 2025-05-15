//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LNSIZE: i32 = 80;

/// String from pool
///
/// Retrieve the string starting at the FIDX element of the kernel
/// pool variable, where the string may be continued across several
/// components of the kernel pool variable.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   name of the kernel pool variable
///  FIDX       I   index of the first component of the string
///  CONTIN     I   character sequence used to indicate continuation
///  STRING     O   a full string concatenated across continuations
///  SIZE       O   the number of character in the full string value
///  LIDX       O   index of the last component of the string
///  FOUND      O   flag indicating success or failure of request
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is the name of a kernel pool variable for which
///           the caller wants to retrieve a full (potentially
///           continued) string.
///
///  FIDX     is the index of the first component (the start) of
///           the string in ITEM.
///
///  CONTIN   is a sequence of characters which (if they appear as
///           the last non-blank sequence of characters in a
///           component of a value of a kernel pool variable)
///           indicate that the string associated with the
///           component is continued into the next literal
///           component of the kernel pool variable.
///
///           If CONTIN is blank, all of the components of ITEM
///           will be retrieved as a single string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the full string starting at the FIDX element of the
///           kernel pool variable specified by ITEM.
///
///           Note that if STRING is not sufficiently long to hold
///           the fully continued string, the value will be
///           truncated. You can determine if STRING has been
///           truncated by examining the variable SIZE.
///
///  SIZE     is the index of last non-blank character of
///           continued string as it is represented in the
///           kernel pool. This is the actual number of characters
///           needed to hold the requested string. If STRING
///           contains a truncated portion of the full string,
///           RTRIM(STRING) will be less than SIZE.
///
///           If the value of STRING should be a blank, then
///           SIZE will be set to 1.
///
///  LIDX     is the index of the last component (the end) of
///           the retrieved string in ITEM.
///
///  FOUND    is a logical variable indicating success of the
///           request to retrieve the string.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the variable specified by ITEM is not present in the
///      kernel pool or is present but is not character valued, STRING
///      will be returned as a blank, SIZE will be returned with the
///      value 0 and FOUND will be set to .FALSE. In particular if NTH
///      is less than 1, STRING will be returned as a blank, SIZE will
///      be zero and FOUND will be .FALSE.
///
///  2)  If the variable specified has a blank string associated
///      with its full string starting at FIDX, STRING will be blank,
///      SIZE will be 1 and FOUND will be set to .TRUE.
///
///  3)  If STRING is not long enough to hold all of the characters
///      associated with the NTH string, it will be truncated on the
///      right.
///
///  4)  If the continuation character is a blank, every component
///      of the variable specified by ITEM will be inserted into
///      the output string.
///
///  5)  If the continuation character is blank, then a blank component
///      of a variable is treated as a component with no letters.
///      For example:
///
///         STRINGS = ( 'This is a variable'
///                     'with a blank'
///                     ' '
///                     'component.' )
///
///      Is equivalent to
///
///
///         STRINGS = ( 'This is a variable'
///                     'with a blank'
///                     'component.' )
///
///      from the point of view of SEPOOL if CONTIN is set to the
///      blank character.
/// ```
///
/// # Particulars
///
/// ```text
///   The SPICE Kernel Pool provides a very convenient interface
///   for supplying both numeric and textual data to user application
///   programs. However, any particular component of a character
///   valued component of a kernel pool variable is limited to 80
///   or fewer characters in length.
///
///   This routine allows you to overcome this limitation by
///   "continuing" a character component of a kernel pool variable.
///   To do this you need to select a continuation sequence
///   of characters and then insert this sequence as the last non-blank
///   set of characters that make up the portion of the component
///   that should be continued.
///
///   For example, you may decide to use the sequence '//' to indicate
///   that a string should be continued to the next component of
///   a kernel pool variable. Then set up the
///   kernel pool variable as shown below
///
///   LONG_STRINGS = ( 'This is part of the first component //'
///                    'that needs more than one line when //'
///                    'inserting it into the kernel pool.'
///                    'This is the second string that is split //'
///                    'up as several components of a kernel pool //'
///                    'variable.' )
///
///   When loaded into the kernel pool, the variable LONG_STRINGS
///   will have six literal components:
///
///      COMPONENT (1) = 'This is part of the first component //'
///      COMPONENT (2) = 'that needs more than one line when //'
///      COMPONENT (3) = 'inserting it into the kernel pool.'
///      COMPONENT (4) = 'This is the second string that is split //'
///      COMPONENT (5) = 'up as several components of a kernel pool //'
///      COMPONENT (6) = 'variable.'
///
///   These are the components that would be retrieved by the call
///
///      CALL GCPOOL ( 'LONG_STRINGS', 1, 6, N, COMPONENT, FOUND )
///
///   However, using the routine SEPOOL you can view the variable
///   LONG_STRINGS as having two long components.
///
///      STRING (1) = 'This is part of the first component that '
///  .   //           'needs more than one line when inserting '
///  .   //           'it into the kernel pool. '
///
///      STRING (2) = 'This is the second string that is split '
///  .   //           'up as several components of a kernel pool '
///  .   //           'variable. '
///
///
///   These string components would be retrieved by the following two
///   calls.
///
///      FIDX = 1
///      CALL SEPOOL ( 'LONG_STRINGS', FIDX, '//',
///     .                              STRING(1), SIZE, LIDX, FOUND )
///      FIDX = LIDX+1
///      CALL SEPOOL ( 'LONG_STRINGS', FIDX, '//',
///     .                              STRING(2), SIZE, LIDX, FOUND )
/// ```
///
/// # Examples
///
/// ```text
///  Example 1. Retrieving file names.
///
///   Suppose a you have used the kernel pool as a mechanism for
///   specifying SPK files to load at startup but that the full
///   names of the files are too long to be contained in a single
///   text line of a kernel pool assignment.
///
///   By selecting an appropriate continuation character ('*' for
///   example)  you can insert the full names of the SPK files
///   into the kernel pool and then retrieve them using this
///   routine.
///
///   First set up the kernel pool specification of the strings
///   as shown here:
///
///         SPK_FILES = ( 'this_is_the_full_path_specification_*'
///                       'of_a_file_with_a_long_name'
///                       'this_is_the_full_path_specification_*'
///                       'of_a_second_file_with_a_very_long_*'
///                       'name' )
///
///   Now to retrieve and load the SPK_FILES one at a time,
///   exercise the following loop.
///
///   INTEGER               FILSIZ
///   PARAMETER           ( FILSIZ = 255 )
///
///   CHARACTER*(FILSIZ)    FILE
///   INTEGER               I
///   INTEGER               LIDX
///
///   I = 1
///
///   CALL SEPOOL ( 'SPK_FILES', I, '*', FILE, SIZE, LIDX, FOUND )
///
///   DO WHILE ( FOUND .AND. RTRIM(FILE) .EQ. SIZE )
///
///      CALL SPKLEF ( FILE, HANDLE )
///      I = LIDX + 1
///      CALL SEPOOL ( 'SPK_FILES', I, '*', FILE, SIZE, LIDX, FOUND )
///   END DO
///
///   IF ( FOUND .AND. RTRIM(FILE) .NE. SIZE ) THEN
///      WRITE (*,*) 'The ', I, '''th file name was too long.'
///   END IF
///
///
///   Example 2. Retrieving all components as a string.
///
///
///   Occasionally, it may be useful to retrieve the entire
///   contents of a kernel pool variable as a single string. To
///   do this you can use the blank character as the
///   continuation character. For example if you place the
///   following assignment in a text kernel
///
///       COMMENT = (  'This is a long note '
///                    ' about the intended '
///                    ' use of this text kernel that '
///                    ' can be retrieved at run time.' )
///
///   you can retrieve COMMENT as single string via the call below.
///
///      CALL SEPOOL ( 'COMMENT', 1, ' ', COMMNT, SIZE, LIDX, FOUND )
///
///   The result will be that COMMNT will have the following value.
///
///      COMMNT = 'This is a long note about the intended use of '
///  .   //       'this text kernel that can be retrieved at run '
///  .   //       'time. '
///
///   Note that the leading blanks of each component of COMMENT are
///   significant, trailing blanks are not significant.
///
///   If COMMENT had been set as
///
///       COMMENT = (  'This is a long note '
///                    'about the intended '
///                    'use of this text kernel that '
///                    'can be retrieved at run time.' )
///
///   Then the call to SEPOOL above would have resulted in several
///   words being run together as shown below.
///
///
///      COMMNT = 'This is a long noteabout the intendeduse of '
///  .   //       'this text kernel thatcan be retrieved at run '
///  .   //       'time. '
///
///
///   resulted in several words being run together as shown below.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 12-APR-2012 (WLT) (BVS)
/// ```
pub fn sepool(
    ctx: &mut SpiceContext,
    item: &str,
    fidx: i32,
    contin: &str,
    string: &mut str,
    size: &mut i32,
    lidx: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SEPOOL(
        item.as_bytes(),
        fidx,
        contin.as_bytes(),
        fstr::StrBytes::new(string).as_mut(),
        size,
        lidx,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SEPOOL ( String from pool )
pub fn SEPOOL(
    ITEM: &[u8],
    FIDX: i32,
    CONTIN: &[u8],
    STRING: &mut [u8],
    SIZE: &mut i32,
    LIDX: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PART = [b' '; LNSIZE as usize];
    let mut CFIRST: i32 = 0;
    let mut CLAST: i32 = 0;
    let mut COMP: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut N: i32 = 0;
    let mut PUTAT: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut GOTIT: bool = false;
    let mut MORE: bool = false;

    // SPICELIB Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Return empty output if the input index is bad.
    //
    if (FIDX < 1) {
        *FOUND = false;
        fstr::assign(STRING, b" ");
        *SIZE = 0;
        *LIDX = 0;
        return Ok(());
    }

    //
    // Check in.
    //
    CHKIN(b"SEPOOL", ctx)?;

    //
    // Check if the first component exists. Return empty output if not.
    //
    GCPOOL(
        ITEM,
        FIDX,
        1,
        &mut N,
        CharArrayMut::from_mut(&mut PART),
        &mut GOTIT,
        ctx,
    )?;

    GOTIT = (GOTIT && (N > 0));

    if !GOTIT {
        *FOUND = false;
        fstr::assign(STRING, b" ");
        *SIZE = 0;
        *LIDX = 0;
        CHKOUT(b"SEPOOL", ctx)?;
        return Ok(());
    }

    //
    // Fetch the string using Bill's algorithm from STPOOL 'as is'.
    //
    ROOM = intrinsics::LEN(STRING);
    CSIZE = RTRIM(CONTIN);
    PUTAT = 1;

    COMP = FIDX;
    MORE = true;
    fstr::assign(STRING, b" ");
    N = 0;

    while MORE {
        GCPOOL(
            ITEM,
            COMP,
            1,
            &mut N,
            CharArrayMut::from_mut(&mut PART),
            &mut MORE,
            ctx,
        )?;

        MORE = (MORE && (N > 0));

        if MORE {
            *FOUND = true;

            CLAST = RTRIM(&PART);
            CFIRST = ((CLAST - CSIZE) + 1);

            if (CFIRST < 0) {
                if (PUTAT <= ROOM) {
                    fstr::assign(
                        fstr::substr_mut(STRING, PUTAT..),
                        fstr::substr(&PART, 1..=CLAST),
                    );
                }

                PUTAT = (PUTAT + CLAST);
                MORE = false;
            } else if fstr::ne(fstr::substr(&PART, CFIRST..=CLAST), CONTIN) {
                if (PUTAT <= ROOM) {
                    fstr::assign(
                        fstr::substr_mut(STRING, PUTAT..),
                        fstr::substr(&PART, 1..=CLAST),
                    );
                }
                PUTAT = (PUTAT + CLAST);
                MORE = false;
            } else if (CFIRST > 1) {
                if (PUTAT <= ROOM) {
                    fstr::assign(
                        fstr::substr_mut(STRING, PUTAT..),
                        fstr::substr(&PART, 1..=(CFIRST - 1)),
                    );
                }
                PUTAT = ((PUTAT + CFIRST) - 1);
            }
        }

        COMP = (COMP + 1);
    }

    //
    // We are done. Get the size of the full string and the index of its
    // last component and checkout.
    //
    *SIZE = (PUTAT - 1);
    *LIDX = (COMP - 1);

    CHKOUT(b"SEPOOL", ctx)?;

    Ok(())
}
