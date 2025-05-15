//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Locate lines in a text file
///
/// Locate a group of lines in a text file delimited by markers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Logical unit connected to text file.
///  BMARK      I   Begin marker.
///  EMARK      I   End marker.
///  LINE      I-O  Workspace.
///  BLINE      O   Beginning line.
///  ELINE      O   Ending line.
///  FOUND      O   Markers found?
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is a logical unit that has been connected to a
///           text file by the calling program. Use the routine
///           TXTOPR to open the file for read access and get its
///           logical unit. The file pointer may be pointing to
///           any line in the file due to previous read statements,
///           for example, or due to previous calls to LOCLN.
///
///  BMARK,
///  EMARK    are markers that delimit some group of lines in
///           the part of the file following the current position
///           of the file pointer. The group begins with the
///           first line equivalent to BMARK and ends with the
///           next line equivalent to EMARK, ignoring leading
///           and trailing blanks.
///
///           If BMARK is blank, the group of lines begins with
///           the first line following the current position of the
///           file pointer; if EMARK is blank, the group of lines
///           ends with the last line in the file.
///
///   LINE       on input, is an arbitrary character string whose
///           contents are ignored. LINE is used to read lines
///           from the file connected to UNIT; its function
///           is to determine the maximum length of the lines
///           that can be read from the file. Lines longer
///           than the declared length of LINE are truncated
///           as they are read.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     on output, is undefined.
///
///  BLINE,
///  ELINE    are the line numbers of the first and last lines
///           in the group delimited by BMARK and EMARK.
///
///           By convention, the first line read by the routine
///           is line 1; the second line is line 2; and so on.
///           If BMARK is blank, BLINE will be 1.
///
///  FOUND    is .TRUE. if a group of lines delimited by BMARK and
///           EMARK is found, and is .FALSE. otherwise. ELINE is
///           the last line read by LOCLN, so if FOUND is .TRUE.,
///           the file pointer will be positioned on the line
///           after ELINE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FOUND is .FALSE., the values of BLINE and ELINE are not
///      changed.
///
///  2)  If an error occurs while reading from the input file,
///      the error SPICE(FILEREADFAILED) is signaled.
///
///  3)  Lines in the file that are longer than the declared length of
///      LINE are truncated as they are read. If the truncation of
///      line containing a marker causes truncation of that marker,
///      it will not match the input value for that marker, so
///      FOUND will be .FALSE.
/// ```
///
/// # Files
///
/// ```text
///  See argument UNIT.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine locates delimited groups of lines in a text file.
///  This allows files to be partitioned into sub-files; it also
///  allows related inputs to be grouped together in a relatively
///  free-format way.
/// ```
///
/// # Examples
///
/// ```text
///  1) Let FILE.TXT be a text file that contains the following lines.
///     (The lines are numbered for reference, but these numbers do
///     not appear in the file).
///
///        1    BEGIN POEM
///        2       Oh snail,
///        3       Climb Mount Fuji,
///        4       But slowly, slowly!
///        5    END POEM
///        6
///        7    BEGIN PROSE
///        8       Lady, one of us has this book open
///        9       to the wrong page.
///        10   END PROSE
///        11
///        12   BEGIN POEM
///        13      John Keats, John Keats,
///        14      John,
///        15      Put your scarf on.
///        16   END POEM
///        17
///        18   BEGIN QUOTE
///        19      That's not writing. That's typing.
///        20
///        21               (Truman Capote on Jack Kerouac)
///        22   END QUOTE
///        23
///        24   BEGIN POEM
///        25      Twice five syllables
///        26      Plus seven isn't much, but
///        27      That's haiku for you.
///        28   BEGIN POEM
///        29
///        30   BEGIN EQUATION
///        31            2
///        32      e = mc
///        33   END EQUATION
///
///  Then the code fragment
///
///        CALL TXTOPR ( 'FILE.TXT', UNIT )
///
///        BMARK = 'BEGIN POEM'
///        EMARK = 'END POEM'
///
///        CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///
///        DO WHILE ( FOUND )
///           WRITE (*,*) 'Found poem between lines ', B, ' and ', E
///
///           CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///        END DO
///
///  produces the following report:
///
///        Found poem between lines   1 and   5
///        Found poem between lines   7 and  11
///        Found poem between lines   8 and  12
///
///  Note that line numbers are returned relative to the position
///  of the file pointer when LOCLN is called. The following code
///  fragment generates the numbers relative to the start of the
///  file.
///
///        REWIND ( UNIT )
///
///        OFFSET = 0
///        CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///
///        DO WHILE ( FOUND )
///           WRITE (*,*) 'Found poem between lines ',
///       .                OFFSET + B,
///       .                ' and ',
///       .                OFFSET + E
///
///           OFFSET = OFFSET + E
///           CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///        END DO
///
///        CLOSE ( UNIT )
///
///  The following report is produced:
///
///        Found poem between lines   1 and   5
///        Found poem between lines  12 and  16
///        Found poem between lines  24 and  28
///
///
///  2) Given the same file, the code fragment
///
///        CALL TXTOPR ( 'FILE.TXT', UNIT )
///
///        CALL LOCLN ( UNIT,
///       .             'begin poem',
///       .             'end poem',
///       .             LINE,
///       .             B,
///       .             E,
///       .             FOUND )
///
///        CLOSE ( UNIT )
///
///  finds nothing because case is significant: FOUND is false,
///  and B and E are unchanged.
///
///  3) This code fragment
///
///        CALL TXTOPR ( 'FILE.TXT', UNIT )
///
///        CALL LOCLN ( UNIT,
///       .             ' ',
///       .             'BEGIN PROSE',
///       .             LINE,
///       .             B,
///       .             E,
///       .             FOUND )
///
///        CLOSE ( UNIT )
///
///  when executed on the same file returns B = 1 and E = 7.
///  In effect, a blank begin marker "matches" the first line
///  that is read.
///
///  Similarly, a blank end marker "matches" the last line of
///  the file, the code fragment
///
///        CALL TXTOPR ( 'FILE.TXT', UNIT )
///
///        CALL LOCLN ( UNIT,
///       .             'BEGIN QUOTE',
///       .             ' ',
///       .             LINE,
///       .             B,
///       .             E,
///       .             FOUND )
///
///        CLOSE ( UNIT )
///
///  when executed on the same file returns B = 18 and E = 33.
///  If both markers are blank, LOCLN basically counts the lines
///  in the file.
///
///  4) The code fragment
///
///        CALL TXTOPR ( 'FILE.TXT', UNIT )
///
///        MARK = 'BEGIN POEM'
///
///        CALL LOCLN ( UNIT, MARK, MARK, LINE, FIRST, SECOND, FOUND )
///
///        CLOSE ( UNIT )
///
///  returns FIRST = 1 and SECOND = 12 -- the first two lines that
///  are equivalent to MARK.
///
///  5) Nesting is not supported. That is, if UNIT is connected to
///  a file containing the following lines (ignoring line numbers),
///
///        1   Begin Object
///        2     Begin Object
///        3       Begin Object
///        4         Just kidding!
///        5       End Object
///        6     End Object
///        7   End Object
///
///        REWIND ( UNIT )
///
///        CALL LOCLN ( UNIT,
///       .             'Begin Object'
///       .             'End Object',
///       .             LINE,
///       .             B,
///       .             E,
///       .             FOUND )
///
///  returns B = 1 and E = 5, not E = 7.
///
///  6) Let UNIT be connected to a text file containing the
///  following lines, again ignoring line numbers which are
///  listed for easy reference.
///
///        1    The first case tests the capability of ...
///        2
///        3    NEW CASE
///        4       TARGET = JUPITER
///        5       EPOCH  = 21 JUN 1992 13:04
///        6    END CASE
///        7
///        8    The next case uses a different target and a slightly
///        9    longer exposure time...
///        10
///        11   NEW CASE
///        12      TARGET   = IO
///        13      EPOCH    = 21 JUN 1992 13:04
///        14      EXPOSURE = 2.44 SECONDS
///        15   END CASE
///        16
///        17   The next case changes targets in order to...
///        18
///        19   NEW CASE
///        20      TARGET   = EUROPA
///        21      EPOCH    = 21 JUN 1992 13:04
///        22      EXPOSURE = 2.44 SECONDS
///        23   END CASE
///
///  Then the code fragment
///
///        REWIND ( UNIT )
///
///        BMARK = 'NEW CASE'
///        EMARK = 'END CASE'
///
///        CASES  = 0
///        OFFSET = 0
///        CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///
///        DO WHILE ( FOUND )
///           CASES      = CASES  + 1
///           BEG(CASES) = OFFSET + B
///           END(CASES) = OFFSET + E
///
///           OFFSET = OFFSET + E
///           CALL LOCLN ( UNIT, BMARK, EMARK, LINE, B, E, FOUND )
///        END DO
///
///  saves the locations of the various input cases (skipping past
///  the intervening commentary) in the arrays BEG and END. After
///  running the code, CASES, BEG, and END have the following values:
///
///        CASES = 3
///        BEG   = 3,  11,  19
///        END   = 6,  15,  23
///
///  The following code fragment retrieves the i'th case.
///
///        REWIND ( UNIT )
///
///        DO J = 1, BEG(I) - 1
///           READ (UNIT,FMT='(A)') LINE
///        END DO
///
///        DO J = BEG(I), END(I)
///           READ (UNIT,FMT='(A)') LINE
///            .
///            .  Process the line
///            .
///        END DO
///
///  While this isn't an incredibly efficient way to process
///  large files, it can be effective for smaller files.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
pub fn locln(
    ctx: &mut SpiceContext,
    unit: i32,
    bmark: &str,
    emark: &str,
    line: &mut str,
    bline: &mut i32,
    eline: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    LOCLN(
        unit,
        bmark.as_bytes(),
        emark.as_bytes(),
        fstr::StrBytes::new(line).as_mut(),
        bline,
        eline,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LOCLN ( Locate lines in a text file )
pub fn LOCLN(
    UNIT: i32,
    BMARK: &[u8],
    EMARK: &[u8],
    LINE: &mut [u8],
    BLINE: &mut i32,
    ELINE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BLTEMP: i32 = 0;
    let mut ELTEMP: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut BFOUND: bool = false;
    let mut EFOUND: bool = false;
    let mut EOF: bool = false;

    //
    // SPICELIB functions
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
        CHKIN(b"LOCLN", ctx)?;
    }

    //
    // We'll use temporary variables BLTEMP and ELTEMP for BLINE and
    // ELINE until we know that both markers have been found.  We'll
    // use BFOUND to indicate whether or not BMARK was found, and
    // EFOUND to indicate whether or not EMARK was found.  EOF
    // indicates end of file.
    //
    BLTEMP = 0;
    BFOUND = false;
    EFOUND = false;
    EOF = false;

    //
    // Read through the file, line by line, searching for the first
    // occurrence of BMARK and counting lines as we go.  Once we
    // find BMARK, we'll start searching for EMARK.  After each read
    // we'll check for I/O errors.
    //

    while (!BFOUND && !EOF) {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }
        //
        // An end-of-file condition is indicated by a negative value
        // for IOSTAT. Any other non-zero value indicates some other
        // error.
        //
        if (IOSTAT > 0) {
            SETMSG(b"While searching for BMARK = #, an attempt to read the file named FILENAME failed.  The value of IOSTAT is #.", ctx);
            ERRCH(b"#", BMARK, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            ERRFNM(b"FILENAME", UNIT, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"LOCLN", ctx)?;
            return Ok(());
        } else if (IOSTAT < 0) {
            EOF = true;
        } else {
            //
            // The read was successful, so count the line then
            // check for a match.
            //
            BLTEMP = (BLTEMP + 1);

            LJUST(&LINE.to_vec(), LINE);

            //
            // By convention, if BMARK is blank, it matches the
            // first line that we read.  If it is not blank, we
            // compare it to the line just read, ignoring leading
            // and trailing blanks.
            //
            if fstr::eq(BMARK, b" ") {
                BFOUND = true;
            } else {
                if fstr::eq(fstr::substr(BMARK, LTRIM(BMARK)..), LINE) {
                    BFOUND = true;
                }
            }
        }
    }

    //
    // Start the search for EMARK starting from where we left off.
    //
    ELTEMP = BLTEMP;

    while (!EFOUND && !EOF) {
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::FormattedReader::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(LINE)?;
                reader.finish()?;
                Ok(())
            })?;
        }
        //
        // An end-of-file condition is indicated by a negative value
        // for IOSTAT. Any other non-zero value indicates some other
        // error.
        //
        if (IOSTAT > 0) {
            SETMSG(b"While searching for EMARK = #, an attempt to read the file named FILENAME failed.  The value of IOSTAT is #.", ctx);
            ERRCH(b"#", EMARK, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            ERRFNM(b"FILENAME", UNIT, ctx)?;
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"LOCLN", ctx)?;
            return Ok(());
        } else if (IOSTAT < 0) {
            EOF = true;
            //
            // By convention, if EMARK is blank, it matches the
            // last line in the file.
            //
            if fstr::eq(EMARK, b" ") {
                EFOUND = true;
            }
        } else {
            //
            // The read was successful, so count the line and check for
            // a match.
            //
            ELTEMP = (ELTEMP + 1);

            LJUST(&LINE.to_vec(), LINE);

            if fstr::ne(EMARK, b" ") {
                if fstr::eq(fstr::substr(EMARK, LTRIM(EMARK)..), LINE) {
                    EFOUND = true;
                }
            }
        }
    }

    //
    // Assign the line numbers to BLINE and ELINE only if both markers
    // were found.
    //
    *FOUND = (BFOUND && EFOUND);

    if *FOUND {
        *BLINE = BLTEMP;
        *ELINE = ELTEMP;
    }

    CHKOUT(b"LOCLN", ctx)?;
    Ok(())
}
