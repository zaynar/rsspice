//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LNSIZE: i32 = 80;

/// String from pool
///
/// Retrieve the Nth string from a kernel pool variable, where the
/// string may be continued across several components of the kernel
/// pool variable.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   Name of the kernel pool variable.
///  NTH        I   Index of the full string to retrieve.
///  CONTIN     I   Character sequence used to indicate continuation.
///  NTHSTR     O   A full string concatenated across continuations.
///  SIZE       O   The number of characters in the full string value.
///  FOUND      O   Flag indicating success or failure of request.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is the name of a kernel pool variable for which
///           the caller wants to retrieve a full (potentially
///           continued) string component.
///
///  NTH      is the number of the string to retrieve from the kernel
///           pool. The range of NTH is 1 to the number of full strings
///           that are present.
///
///  CONTIN   is a sequence of characters which (if they appear as the
///           last non-blank sequence of characters in a component of a
///           value of a kernel pool variable) act as a continuation
///           marker: the marker indicates that the string associated
///           with the component is continued into the next literal
///           component of the kernel pool variable.
///
///           If CONTIN is blank, all of the components of ITEM will be
///           retrieved as a single string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NTHSTR   is the NTH full string associated with the kernel pool
///           variable specified by ITEM.
///
///           Note that if NTHSTR is not sufficiently long to hold the
///           fully continued string, the value will be truncated. You
///           can determine if NTHSTR has been truncated by examining
///           the variable SIZE.
///
///  SIZE     is the index of last non-blank character of the continued
///           string as it is represented in the kernel pool. This is
///           the actual number of characters needed to hold the
///           requested string. If NTHSTR contains a truncated portion
///           of the full string, RTRIM(NTHSTR) will be less than SIZE.
///
///           If the value of NTHSTR should be a blank, then SIZE will
///           be set to 1.
///
///  FOUND    is a logical variable indicating success of the request
///           to retrieve the NTH string associated with ITEM. If an
///           Nth string exists, FOUND will be set to .TRUE.;
///           otherwise FOUND will be set to .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the variable specified by ITEM is not present in the
///      kernel pool or is present but is not character valued,
///      NTHSTR will be returned as a blank, SIZE will be
///      returned with the value 0 and FOUND will be set to .FALSE. In
///      particular if NTH is less than 1, NTHSTR will be returned as a
///      blank, SIZE will be zero and FOUND will be .FALSE.
///
///  2)  If the variable specified has a blank string associated
///      with its NTH full string, NTHSTR will be blank, SIZE
///      will be 1 and FOUND will be set to .TRUE.
///
///  3)  If NTHSTR is not long enough to hold all of the characters
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
///      from the point of view of STPOOL if CONTIN is set to the
///      blank character.
/// ```
///
/// # Particulars
///
/// ```text
///  The SPICE Kernel Pool provides a very convenient interface
///  for supplying both numeric and textual data to user application
///  programs. However, any particular component of a character
///  valued component of a kernel pool variable is limited to 80
///  or fewer characters in length.
///
///  This routine allows you to overcome this limitation by
///  "continuing" a character component of a kernel pool variable.
///  To do this you need to select a continuation sequence
///  of characters and then insert this sequence as the last non-blank
///  set of characters that make up the portion of the component
///  that should be continued.
///
///  For example, you may decide to use the sequence '//' to indicate
///  that a string should be continued to the next component of
///  a kernel pool variable. Then set up the
///  kernel pool variable as shown below
///
///     LONG_STRINGS = ( 'This is part of the first component //'
///                      'that needs more than one line when //'
///                      'inserting it into the kernel pool.'
///                      'This is the second string that is split //'
///                      'up as several components of a kernel pool //'
///                      'variable.' )
///
///  When loaded into the kernel pool, the variable LONG_STRINGS
///  will have six literal components:
///
///     COMPONENT (1) = 'This is part of the first component //'
///     COMPONENT (2) = 'that needs more than one line when //'
///     COMPONENT (3) = 'inserting it into the kernel pool.'
///     COMPONENT (4) = 'This is the second string that is split //'
///     COMPONENT (5) = 'up as several components of a kernel pool //'
///     COMPONENT (6) = 'variable.'
///
///  These are the components that would be retrieved by the call
///
///     CALL GCPOOL ( 'LONG_STRINGS', 1, 6, N, COMPONENT, FOUND )
///
///  However, using the routine STPOOL you can view the variable
///  LONG_STRINGS as having two long components.
///
///     STRGNA = 'This is part of the first component that '
///    . //      'needs more than one line when inserting '
///    . //      'it into the kernel pool. '
///
///     STRGNB = 'This is the second string that is split '
///    . //      'up as several components of a kernel pool '
///    . //      'variable. '
///
///
///  These string components would be retrieved by the following two
///  calls.
///
///     CALL STPOOL ( 'LONG_STRINGS', 1, '//', STRGNA, SIZE, FOUND )
///     CALL STPOOL ( 'LONG_STRINGS', 2, '//', STRGNB, SIZE, FOUND )
/// ```
///
/// # Examples
///
/// ```text
///  Example 1. Retrieving file names.
///
///  Suppose a you have used the kernel pool as a mechanism for
///  specifying SPK files to load at startup but that the full
///  names of the files are too long to be contained in a single
///  text line of a kernel pool assignment.
///
///  By selecting an appropriate continuation character ('*' for
///  example)  you can insert the full names of the SPK files
///  into the kernel pool and then retrieve them using this
///  routine.
///
///  First set up the kernel pool specification of the strings
///  as shown here:
///
///        SPK_FILES = ( 'this_is_the_full_path_specification_*'
///                      'of_a_file_with_a_long_name'
///                      'this_is_the_full_path_specification_*'
///                      'of_a_second_file_with_a_very_long_*'
///                      'name' )
///
///  Now to retrieve and load the SPK_FILES one at a time,
///  exercise the following loop.
///
///  INTEGER               FILSIZ
///  PARAMETER           ( FILSIZ = 255 )
///
///  CHARACTER*(FILSIZ)    FILE
///  INTEGER               I
///
///  I = 1
///
///  CALL STPOOL ( 'SPK_FILES', I, '*', FILE, SIZE, FOUND )
///
///  DO WHILE ( FOUND .AND. RTRIM(FILE) .EQ. SIZE )
///
///     CALL SPKLEF ( FILE, HANDLE )
///     I = I + 1
///     CALL STPOOL ( 'SPK_FILES', I, '*', FILE, SIZE, FOUND )
///
///  END DO
///
///  IF ( FOUND .AND. RTRIM(FILE) .NE. SIZE ) THEN
///
///     WRITE (*,*) 'The ', I, '''th file name was too long.'
///
///  END IF
///
///
///  Example 2. Retrieving all components as a string.
///
///
///  Occasionally, it may be useful to retrieve the entire
///  contents of a kernel pool variable as a single string. To
///  do this you can use the blank character as the
///  continuation character. For example if you place the
///  following assignment in a text kernel
///
///      COMMENT = (  'This is a long note '
///                   ' about the intended '
///                   ' use of this text kernel that '
///                   ' can be retrieved at run time.' )
///
///  you can retrieve COMMENT as single string via the call below.
///
///     CALL STPOOL ( 'COMMENT', 1, ' ', COMMNT, SIZE, FOUND )
///
///  The result will be that COMMNT will have the following value.
///
///     COMMNT = 'This is a long note about the intended use of '
///    . //      'this text kernel that can be retrieved at run '
///    . //      'time. '
///
///  Note that the leading blanks of each component of COMMENT are
///  significant, trailing blanks are not significant.
///
///  If COMMENT had been set as
///
///      COMMENT = (  'This is a long note '
///                   'about the intended '
///                   'use of this text kernel that '
///                   'can be retrieved at run time.' )
///
///  Then the call to STPOOL above would have resulted in several
///  words being run together as shown below.
///
///
///     COMMNT = 'This is a long noteabout the intendeduse of '
///    . //      'this text kernel thatcan be retrieved at run '
///    . //      'time. '
///
///
///  resulted in several words being run together as shown below.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Changed the output argument name "STRING" to "NTHSTR" for
///         consistency with other routines.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 11-JUL-1997 (WLT)
/// ```
pub fn stpool(
    ctx: &mut SpiceContext,
    item: &str,
    nth: i32,
    contin: &str,
    nthstr: &mut str,
    size: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    STPOOL(
        item.as_bytes(),
        nth,
        contin.as_bytes(),
        fstr::StrBytes::new(nthstr).as_mut(),
        size,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STPOOL ( String from pool )
pub fn STPOOL(
    ITEM: &[u8],
    NTH: i32,
    CONTIN: &[u8],
    NTHSTR: &mut [u8],
    SIZE: &mut i32,
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
    let mut STRNO: i32 = 0;
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

    if (NTH < 1) {
        *FOUND = false;
        fstr::assign(NTHSTR, b" ");
        *SIZE = 0;
        return Ok(());
    }

    CHKIN(b"STPOOL", ctx)?;

    ROOM = intrinsics::LEN(NTHSTR);
    CSIZE = RTRIM(CONTIN);
    PUTAT = 1;

    //
    // Retrieve components until we've gone past the first NTH-1
    // strings.
    //
    STRNO = 1;
    COMP = 1;
    *FOUND = false;

    while (STRNO < NTH) {
        GCPOOL(
            ITEM,
            COMP,
            1,
            &mut N,
            CharArrayMut::from_mut(&mut PART),
            &mut GOTIT,
            ctx,
        )?;

        GOTIT = (N > 0);

        if !GOTIT {
            fstr::assign(NTHSTR, b" ");
            *SIZE = 0;
            *FOUND = false;
            CHKOUT(b"STPOOL", ctx)?;
            return Ok(());
        }

        CLAST = RTRIM(&PART);
        CFIRST = ((CLAST - CSIZE) + 1);

        if (CFIRST < 0) {
            STRNO = (STRNO + 1);
        } else if fstr::ne(fstr::substr(&PART, CFIRST..=CLAST), CONTIN) {
            STRNO = (STRNO + 1);
        }

        COMP = (COMP + 1);
    }

    //
    // Once we've reached this point, COMP points to the component
    // of the kernel pool variable that is the beginning of the NTH
    // string.  Now just retrieve components until we run out or
    // one is not continued.
    //
    MORE = true;
    fstr::assign(NTHSTR, b" ");
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
                        fstr::substr_mut(NTHSTR, PUTAT..),
                        fstr::substr(&PART, 1..=CLAST),
                    );
                }

                PUTAT = (PUTAT + CLAST);
                MORE = false;
            } else if fstr::ne(fstr::substr(&PART, CFIRST..=CLAST), CONTIN) {
                if (PUTAT <= ROOM) {
                    fstr::assign(
                        fstr::substr_mut(NTHSTR, PUTAT..),
                        fstr::substr(&PART, 1..=CLAST),
                    );
                }
                PUTAT = (PUTAT + CLAST);
                MORE = false;
            } else if (CFIRST > 1) {
                if (PUTAT <= ROOM) {
                    fstr::assign(
                        fstr::substr_mut(NTHSTR, PUTAT..),
                        fstr::substr(&PART, 1..=(CFIRST - 1)),
                    );
                }
                PUTAT = ((PUTAT + CFIRST) - 1);
            }
        }

        COMP = (COMP + 1);
    }

    //
    // We are done.  Get the size of the full string and checkout.
    //
    *SIZE = (PUTAT - 1);

    CHKOUT(b"STPOOL", ctx)?;

    Ok(())
}
