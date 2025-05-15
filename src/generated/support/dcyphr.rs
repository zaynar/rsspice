//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const ROOM: i32 = 2;
const MAXMSG: i32 = 90;
const MAXLEN: i32 = 800;

struct SaveVars {
    FIRST: bool,
    NEXT: bool,
    HP: bool,
    SUN: bool,
    SGI: bool,
    VAX: bool,
    PC: bool,
    ALPHA: bool,
    LBND: i32,
    UBND: i32,
    N: i32,
    ATTR: ActualCharArray,
    MESSGE: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;
        let mut NEXT: bool = false;
        let mut HP: bool = false;
        let mut SUN: bool = false;
        let mut SGI: bool = false;
        let mut VAX: bool = false;
        let mut PC: bool = false;
        let mut ALPHA: bool = false;
        let mut LBND: i32 = 0;
        let mut UBND: i32 = 0;
        let mut N: i32 = 0;
        let mut ATTR = ActualCharArray::new(WDSIZE, 1..=ROOM);
        let mut MESSGE = ActualCharArray::new(MAXLEN, 1..=MAXMSG);

        FIRST = true;

        Self {
            FIRST,
            NEXT,
            HP,
            SUN,
            SGI,
            VAX,
            PC,
            ALPHA,
            LBND,
            UBND,
            N,
            ATTR,
            MESSGE,
        }
    }
}

//$Procedure      DCYPHR ( Decypher the meaning of an IOSTAT code)
pub fn DCYPHR(IOSTAT: i32, FOUND: &mut bool, DIAGNS: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        PLTFRM(2, &mut save.N, save.ATTR.as_arg_mut(), ctx);

        save.NEXT = (save.FIRST && fstr::eq(save.ATTR.get(1), b"NEXT "));
        save.HP = (save.FIRST && fstr::eq(save.ATTR.get(1), b"HP   "));
        save.SUN = (save.FIRST && fstr::eq(save.ATTR.get(1), b"SUN  "));
        save.SGI = (save.FIRST && fstr::eq(save.ATTR.get(1), b"SGI  "));
        save.VAX = (save.FIRST && fstr::eq(save.ATTR.get(1), b"VAX  "));
        save.PC = (save.FIRST && fstr::eq(save.ATTR.get(1), b"PC   "));
        save.ALPHA = (save.FIRST && fstr::eq(save.ATTR.get(1), b"ALPHA"));
        save.FIRST = false;
    }

    if save.NEXT {
        save.LBND = 9999;
        save.UBND = 10032;

        fstr::assign(save.MESSGE.get_mut(1), b"The file is not open for reading.");
        fstr::assign(save.MESSGE.get_mut(2), b"The file is not open for writing.");
        fstr::assign(save.MESSGE.get_mut(3), b"The file was not found.");
        fstr::assign(
            save.MESSGE.get_mut(4),
            b"The record length specified was negative or 0.",
        );
        fstr::assign(save.MESSGE.get_mut(5), b"I/O buffer allocation failed.");

        fstr::assign(save.MESSGE.get_mut(6), b"The iolist specifier was bad.");
        fstr::assign(save.MESSGE.get_mut(7), b"The format string is in error.");
        fstr::assign(save.MESSGE.get_mut(8), b"The repeat count is illegal.");
        fstr::assign(
            save.MESSGE.get_mut(9),
            b"The hollerith count exceeds remaining format string.",
        );
        fstr::assign(
            save.MESSGE.get_mut(10),
            b"The format string is missing an opening \"(\".",
        );
        fstr::assign(
            save.MESSGE.get_mut(11),
            b"The format string has unmatched parentheses.",
        );
        fstr::assign(
            save.MESSGE.get_mut(12),
            b"The format string has unmatched quotes.",
        );
        fstr::assign(
            save.MESSGE.get_mut(13),
            b"A format descriptor is non-repeatable.",
        );
        fstr::assign(
            save.MESSGE.get_mut(14),
            b"The program attempted to read past end of the file.",
        );
        fstr::assign(save.MESSGE.get_mut(15), b"The file specification was bad.");
        fstr::assign(
            save.MESSGE.get_mut(16),
            b"The format group table overflowed.",
        );
        fstr::assign(
            save.MESSGE.get_mut(17),
            b"An illegal character was present in numeric input.",
        );
        fstr::assign(
            save.MESSGE.get_mut(18),
            b"No record was specified while using direct access I/O.",
        );
        fstr::assign(
            save.MESSGE.get_mut(19),
            b"The maximum record number was exceeded.",
        );
        fstr::assign(
            save.MESSGE.get_mut(20),
            b"An illegal file type was supplied for use with namelist directed I/O",
        );
        fstr::assign(
            save.MESSGE.get_mut(21),
            b"An illegal input for namelist directed I/O was encountered.",
        );
        fstr::assign(
            save.MESSGE.get_mut(22),
            b"A variable is not present in the current namelist.",
        );
        fstr::assign(
            save.MESSGE.get_mut(23),
            b"A variable type or size does not match edit descriptor.",
        );
        fstr::assign(
            save.MESSGE.get_mut(24),
            b"An llegal direct access record number was used.",
        );

        fstr::assign(
            save.MESSGE.get_mut(25),
            b"An internal file was used illegally.",
        );
        fstr::assign(
            save.MESSGE.get_mut(26),
            b"The OPEN specifiere \"RECL=\" is only valid for direct access files",
        );
        fstr::assign(
            save.MESSGE.get_mut(27),
            b"The Open specifiere \"BLOCK=\" is only valid for unformatted sequential files.",
        );
        fstr::assign(
            save.MESSGE.get_mut(28),
            b"The program was unable to truncate the file after rewind, backspace,or endfile.",
        );
        fstr::assign(
            save.MESSGE.get_mut(29),
            b"It\'s illegal to use formatted I/O on an entire structure.",
        );
        fstr::assign(
            save.MESSGE.get_mut(30),
            b"An illegal (negative) unit was specified.",
        );
        fstr::assign(
            save.MESSGE.get_mut(31),
            b"The specifications in a RE-OPEN do not match aprevious OPEN.",
        );
        fstr::assign(
            save.MESSGE.get_mut(32),
            b"An implicit OPEN can not be used for direct access files.",
        );
        fstr::assign(
            save.MESSGE.get_mut(33),
            b"The file already exists. It cannot be opened as a new file.",
        );
    } else if save.SUN {
        save.LBND = 99;
        save.UBND = 126;

        fstr::assign(save.MESSGE.get_mut(1), b"The format string is in error.");
        fstr::assign(save.MESSGE.get_mut(2), b"The unit number is illegal.");
        fstr::assign(
            save.MESSGE.get_mut(3),
            b"The logical unit was opened for unformatted I/O, not formatted.",
        );
        fstr::assign(
            save.MESSGE.get_mut(4),
            b"The logical unit was opened for formatted I/O, not unformatted.",
        );
        fstr::assign(save.MESSGE.get_mut(5), b"The logical unit was opened for sequential access, or the logical record length was specified as zero.");
        fstr::assign(
            save.MESSGE.get_mut(6),
            b"The logical unit was opened for direct I/O, not sequential.",
        );
        fstr::assign(
            save.MESSGE.get_mut(7),
            b"The program was unable to backspace the file.",
        );
        fstr::assign(
            save.MESSGE.get_mut(8),
            b"The format specified a left tab beyond the beginning of an internal input record.",
        );
        fstr::assign(save.MESSGE.get_mut(9), b"The system cannot return status information about the file. Perhaps the directory is unreadable.");
        fstr::assign(save.MESSGE.get_mut(10), b"Repeat counts in list-directed I/O must be followed by an asterisk with no blank spaces.");

        fstr::assign(
            save.MESSGE.get_mut(11),
            b"The program attempted to read past the end of a record.",
        );

        fstr::assign(save.MESSGE.get_mut(12), b"The program was unable to truncate an external sequential file on close, backspace, or rewind.");
        fstr::assign(
            save.MESSGE.get_mut(13),
            b"The list input is incomprehensible.",
        );
        fstr::assign(save.MESSGE.get_mut(14), b"The library dynamically creates buffers for internal use. The program is too big, and thus ran out of free space.");
        fstr::assign(save.MESSGE.get_mut(15), b"The logical unit was not open.");
        fstr::assign(save.MESSGE.get_mut(16), b"An unexpected character was encountered. Some format conversions cannot tolerate nonnumeric data.");
        fstr::assign(
            save.MESSGE.get_mut(17),
            b"Logical data must be true or false.",
        );
        fstr::assign(
            save.MESSGE.get_mut(18),
            b"The program tried to open an existing file with \"STATUS = NEW\".",
        );
        fstr::assign(
            save.MESSGE.get_mut(19),
            b"The program tried to open a nonexistent file with \"STATUS=OLD\".",
        );
        fstr::assign(
            save.MESSGE.get_mut(20),
            b"The program caused an unknown system error. Contact your system administrator!",
        );
        fstr::assign(save.MESSGE.get_mut(21), b"Direct access of a file requires seek ability. Sequential unformatted I/O and tabbing left also require seek ability.");
        fstr::assign(
            save.MESSGE.get_mut(22),
            b"An illegal argument was specified in the statement.",
        );
        fstr::assign(
            save.MESSGE.get_mut(23),
            b"The repeat count for list-directed input must be a positive integer.",
        );
        fstr::assign(
            save.MESSGE.get_mut(24),
            b"An illegal operation was attempted on the device associated with the unit.",
        );
        fstr::assign(
            save.MESSGE.get_mut(25),
            b"The program tried to open too many files. The limit is 64.",
        );
        fstr::assign(save.MESSGE.get_mut(26), b"The logical unit was not open.");
        fstr::assign(
            save.MESSGE.get_mut(27),
            b"A namelist read encountered an invalid data item.",
        );
    } else if save.HP {
        save.LBND = 899;
        save.UBND = 989;

        fstr::assign(save.MESSGE.get_mut(1), b"Error in format. Format specification does not start with a left parenthesis or end with a right parenthesis, or contains unrecognizable code or string; format specification is too long for library internal buffer. Change the format specification to proper syntax; split the format specifications into several statements. ");

        fstr::assign(save.MESSGE.get_mut(2), b"I/O with illegal unit number attempted. Negative unit number was used in an I/O statement. Use integers greater than or equal to 0 for an I/O number. ");

        fstr::assign(save.MESSGE.get_mut(3), b"Formatted I/O attempted on unformatted file. Formatted I/O was attempted on a file opened for unformatted I/O. Open the file for formatted I/O; do unformatted I/O on this file. ");

        fstr::assign(save.MESSGE.get_mut(4), b"Unformatted I/O attempted on formatted file. Unformatted I/O was attempted on a file opened for formatted I/O. Open the file for unformatted I/O; do formatted I/O on this file. ");

        fstr::assign(save.MESSGE.get_mut(5), b"Direct I/O attempted on sequential file. Direct operation attempted on sequential file; direct operation attempted on opened file connected to a terminal. Use sequential operations on this file; open file for direct access; do not do direct I/O on a file connected to a terminal. ");

        fstr::assign(save.MESSGE.get_mut(6), b"Error in list- or name-directed read of logical data. Found repeat value, but no asterisk; first character after optional decimal point was not \"T\" or \"F\". Change input data to correspond to syntax expected by list-directed input of logicals; use input statement that corresponds to syntax of input data. ");

        fstr::assign(save.MESSGE.get_mut(7), b"Illegal sequential I/O to tty attempted1. Executed a BACKSPACE, REWIND, formatted READ, or formatted WRITE, on this sequential file or device. Use a file or device that is considered blocked in HP-UX. ");

        fstr::assign(save.MESSGE.get_mut(8), b"List- or name-directed read of character data attempted. Found repeat value, but no asterisk; character not delimited by quotation marks. Change input data to correspond to syntax expected by list-directed input of characters; use input statement that corresponds to syntax of input data. ");

        fstr::assign(save.MESSGE.get_mut(9), b"Open of file with bad path-name attempted. Tried to open a file that the system would not allow for one of the following reasons: 1.  A component of the path prefix is not a directory. 2.  The named file does not exist. 3.  Search permission is denied for a component of the path prefix. Correct the path-name to invoke the file intended; check that the file is not corrupt; be sure that search permissions are set properly. ");

        fstr::assign(save.MESSGE.get_mut(10), b"Sequential I/O attempted on direct file. Attempted a BACKSPACE, REWIND, or ENDFILE on a direct file. Open the file for sequential access; do not use BACKSPACE, REWIND, or ENDFILE. ");

        fstr::assign(save.MESSGE.get_mut(11), b"Access past end of record attempted. Tried to do I/O on record of a file past beginning or end of record. Perform I/O operation within bounds of the record; increase record length. ");

        fstr::assign(save.MESSGE.get_mut(12), b"Recursive I/O attempted1. An I/O specifier or item in an I/O list attempted to do I/O (that is, calls to functions that do I/O). Remove calls to functions that do I/O from the specifier/list item; remove I/O statements from the function called by the specifier/list item. ");

        fstr::assign(save.MESSGE.get_mut(13), b"Error in list- or name-directed read of complex data. While reading complex data, one of the following problems has occurred: 1.  No left parenthesis or no repeat value. 2.  Found repeat value, but no asterisk. 3.  No comma after real part. 4.  No closing right parenthesis. Change input data to correspond to syntax expected by list-directed input of complex numbers; use input statement corresponding to syntax of input data. ");

        fstr::assign(save.MESSGE.get_mut(14), b"Out of free space. Library cannot store file name (from OPEN statement) or characters read (from list-directed read). Use shorter file name or read fewer characters; use fewer file names or read fewer character strings. ");

        fstr::assign(save.MESSGE.get_mut(15), b"Access of unconnected unit attempted. Unit specified in I/O statement has not previously been connected to anything. Connect unit (that is, OPEN it) before attempting I/O on it; perform I/O on another, already connected, unit. ");

        fstr::assign(save.MESSGE.get_mut(16), b"Read unexpected character. While reading an integer, read a character that was not a digit, \"+\", \"-\", comma, end-of-line or blank; while reading a real number, read a character that was not a digit, \"+\", \"-\", comma, end-of-line, blank, \"d\", \"D\", \"e\", \"E\", or period. Remove from input data any characters that are illegal in integers or real numbers. ");

        fstr::assign(save.MESSGE.get_mut(17), b"Error in read of logical data. A blank was read when logical data was expected. Change input data to correspond to syntax expected when reading logical data; use input statement corresponding to syntax of input data. ");

        fstr::assign(save.MESSGE.get_mut(18), b"Open with named scratch file attempted. Executed OPEN statement with STATUS=\'SCRATCH\', but also named the file (FILE= filename). Either open file with STATUS=\'SCRATCH\', or name the file in an OPEN statement, but not both. ");

        fstr::assign(save.MESSGE.get_mut(19), b"Open of existing file with STATUS=\'NEW\' attempted. Executed OPEN statement with STATUS=\'NEW\', but file already exists. Use OPEN without STATUS specifier, or with STATUS=\'OLD\', or STATUS=\'UNKNOWN\'. ");

        fstr::assign(save.MESSGE.get_mut(20), b"The value of IOSTAT was 919.  No explanation is provided in the HP documentation for this value of IOSTAT. . ");

        fstr::assign(save.MESSGE.get_mut(21), b"Open of file connected to different unit attempted. Executed OPEN statement with file name that is already associated with a UNIT specifier. Use an OPEN statement with a file name that is not connected to a unit name; open the connected file to the same unit name. ");

        fstr::assign(save.MESSGE.get_mut(22), b"Unformatted open with BLANK specifier attempted. OPEN statement specified FORM=\'UNFORMATTED\' and BLANK= xx. Use either FORM=\'FORMATTED\' or BLANK= xx, but not both, when opening files. ");

        fstr::assign(save.MESSGE.get_mut(23), b"I/O on illegal record attempted. Attempted to read a record of a formatted or unformatted direct file that is beyond the current end-of-file. Read records that are within the bounds of the file. ");

        fstr::assign(save.MESSGE.get_mut(24), b"Open with illegal FORM specifier attempted. FORM specifier did not begin with \"F\", \"f\", \"U\", or \"u\". Use either \'FORMATTED\' or \'UNFORMATTED\' for the FORM specifier in an OPEN statement. ");

        fstr::assign(save.MESSGE.get_mut(25), b"Close of scratch file with STATUS=\'KEEP\' attempted. The file specified in the CLOSE statement was previously opened with \'SCRATCH\' specified in the STATUS specifier. Open the file with a STATUS other than \'SCRATCH\'; do not specify STATUS=\'KEEP\' in the CLOSE statement for this scratch file. ");

        fstr::assign(save.MESSGE.get_mut(26), b"Open with illegal STATUS specifier attempted. STATUS specifier did not begin with \"O\", \"o\", \"N\", \"n\", \"S\", \"s\", \"U\", or \"u\". Use \'OLD\', \'NEW\', \'SCRATCH\', or \'UNKNOWN\' for the STATUS specifier in OPEN statement. ");

        fstr::assign(save.MESSGE.get_mut(27), b"Close with illegal STATUS specifier attempted. STATUS specifier did not begin with \"K\", \"k\", \"D\", or \"d\". statement. ");

        fstr::assign(save.MESSGE.get_mut(28), b"Open with illegal ACCESS specifier attempted. ACCESS specifier did not begin with \"S\", \"s\", \"D\", or \"d\". Use \'SEQUENTIAL\' or \'DIRECT\' for the ACCESS specifier in an OPEN statement. ");

        fstr::assign(save.MESSGE.get_mut(29), b"Open of sequential file with RECL specifier attempted. OPEN statement had both ACCESS=\'SEQUENTIAL\' and RECL= xx specified. Omit RECL specifier; specify ACCESS=\'DIRECT\'. ");

        fstr::assign(save.MESSGE.get_mut(30), b"Open of direct file with no RECL specifier attempted. OPEN statement has ACCESS=\'DIRECT\', but no RECL specifier. Add RECL specifier; specify ACCESS=\'SEQUENTIAL\'. or Open of direct file with no RECL or RECL=0 attempted1 OPEN statement has ACCESS=\'DIRECT\', but no RECL specifier. Add RECL specifier; specify ACCESS=\'SEQUENTIAL\'. ");

        fstr::assign(save.MESSGE.get_mut(31), b"Open with RECL less than 1 attempted. RECL specifier in OPEN statement was less than or equal to zero. Use a positive number for RECL specifier in OPEN statement. or Open with RECL less than zero attempted. RECL specifier in OPEN statement was less than or equal to zero. Use a positive number for RECL specifier in OPEN statement. ");

        fstr::assign(save.MESSGE.get_mut(32), b"Open with illegal BLANK specifier attempted. BLANK specifier did not begin with \"N\", \"n\", \"Z\", or \"z\". Use \'NULL\' or \'ZERO\' for BLANK specifier in OPEN statement. ");

        fstr::assign(save.MESSGE.get_mut(33), b"Too many units open at once. The program attempted to have greater than 60 files open at once. Close a presently open file before opening another. ");

        fstr::assign(save.MESSGE.get_mut(34), b"End of file encountered. Attempted to read beyond the end of a sequential file. Read records that are within bounds of the file. ");

        fstr::assign(save.MESSGE.get_mut(35), b"The value of IOSTAT was 934.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(
            save.MESSGE.get_mut(36),
            b"Internal library error. A rare software error has occurred. Report the error. ",
        );

        fstr::assign(save.MESSGE.get_mut(37), b"The value of IOSTAT was 936.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(38), b"Access of record <=0 attempted. Access of direct file specifier REC= negative number or 0. Use an integer greater than 0 in the REC= specifier. ");

        fstr::assign(save.MESSGE.get_mut(39), b"List I/O of unknown type attempted. An internal error has occurred. Report the error. ");

        fstr::assign(save.MESSGE.get_mut(40), b"Open of inaccessible file attempted. When opening a file with STATUS=\'OLD\', component of the path is not a directory, the named file does not exist, or the path points outside a process or allocated address space. Use legal pathname; insure existence of file; or open with STATUS=\'NEW\'. ");

        fstr::assign(save.MESSGE.get_mut(41), b"Open attempted. Too many files open; file permissions do not allow access. Close some files before opening more; change read/write access of file to allow open. ");

        fstr::assign(save.MESSGE.get_mut(42), b"Error in sequential unformatted read. Attempt to prepare file for sequential unformatted read failed. Use existing, non-corrupt file and be sure the system is not corrupt. ");

        fstr::assign(save.MESSGE.get_mut(43), b"Error in list- or name-directed read. System detected error while trying to do list read. Be sure system and file are not corrupt. ");

        fstr::assign(save.MESSGE.get_mut(44), b"Error in direct formatted read. System encountered problem while reading a character from specified external file. Be sure file and system are not corrupt. ");

        fstr::assign(save.MESSGE.get_mut(45), b"Error in direct unformatted I/O. System found error while concluding direct unformatted I/O call. Be sure file and system are not corrupt. ");

        fstr::assign(save.MESSGE.get_mut(46), b"Error in formatted I/O. System found error while reading or writing formatted data; usually means more characters were requested than exist in a record. Be sure format matches data.  Be sure file and system are not corrupt. ");

        fstr::assign(save.MESSGE.get_mut(47), b"Error in list I/O. List I/O was attempted on an unformatted file. Do list I/O on formatted file. ");

        fstr::assign(save.MESSGE.get_mut(48), b"Edit descriptor not compatible with type of item. Use an edit descriptor that is compatible with the data item; use a data item that is compatible with the edit descriptor. ");

        fstr::assign(save.MESSGE.get_mut(49), b"Write to write-protected file attempted. Change write protection bit to allow write; do not write to this file. ");

        fstr::assign(save.MESSGE.get_mut(50), b"Read from read-protected file attempted. Change read protection bit to allow read; do not read from this file. ");

        fstr::assign(save.MESSGE.get_mut(51), b"Value out of range. An index to an array or substring reference was outside of the declared limits. Check all indexes to arrays and substrings. ");

        fstr::assign(save.MESSGE.get_mut(52), b"Label out of bounds in assigned GOTO. The value of the variable did not correspond to any of the labels in the list in an assigned GOTO statement. Check for a possible logic error in the program or an incorrect list in the assigned GOTO statement. ");

        fstr::assign(save.MESSGE.get_mut(53), b"Zero increment value in DO loop. A DO loop with a zero increment has produced an infinite loop. Check for a logic error in the program. ");

        fstr::assign(save.MESSGE.get_mut(54), b"No repeatable edit descriptor in format statement. A repeat count was given for an edit descriptor that does not allow repetition. Add at least one repeatable edit descriptor to the format statement. ");

        fstr::assign(save.MESSGE.get_mut(55), b"Illegal use of empty format attempted. An empty format specification, (), was used with the list items specified. Remove the items from I/O list; fill in the format specifications with the appropriate format descriptors. ");

        fstr::assign(save.MESSGE.get_mut(56), b"Open with no FILE= and STATUS \'OLD\' or \'NEW\' attempted. Status \'NEW\' or \'OLD\' was attempted and FILE= was not specified. Change the STATUS specifier to \'SCRATCH\' or \'UNKNOWN\'; add the file specifier. ");

        fstr::assign(save.MESSGE.get_mut(57), b"The value of IOSTAT was 956.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(58), b"Format descriptor incompatible with numeric item in I/O list. A numeric item in the I/O list was matched with a nonnumeric format descriptor. Match format descriptors to I/O list. or File could not be truncated. Physical length of file could not be forced to match the logical length. ");

        fstr::assign(save.MESSGE.get_mut(59), b"Format descriptor incompatible with character item in I/O list. A character item in the I/O list was matched with a format descriptor other than \"A\" or \"R\". Match format descriptors to I/O list. or Unexpected character in NAMELIST read. An illegal character was found in NAMELIST-directed input. Be sure input data conforms to the syntax rules for NAMELIST-directed input. ");

        fstr::assign(save.MESSGE.get_mut(60), b"Format descriptor incompatible with logical item in I/O list. A logical item in the I/O list was matched with a format descriptor other than \"L\". Match format descriptors to I/O list. or Illegal subscript/substring in NAMELIST read. An invalid subscript or substring specifier was found in NAMELIST-directed input. Possible causes:  bad syntax, subscript/substring component out-of-bounds, wrong number of subscripts, substring on non-CHARACTER variable. Check input data for syntax errors.  Be sure subscript/substring specifiers are correct for data type. ");

        fstr::assign(save.MESSGE.get_mut(61), b"Format error: Missing starting left parenthesis. Format did not begin with a left parenthesis. Begin format with a left parenthesis. or Too many values in NAMELIST read. Too many input values were found during a NAMELIST-directed READ. This message will be generated by attempts to fill variables beyond their memory limits. Remove excess values from input data. ");

        fstr::assign(save.MESSGE.get_mut(62), b"Variable not in NAMELIST group. A variable name was encountered in the input stream which was not declared as part of the current NAMELIST group. Check input data with NAMELIST group declaration for differences. Format error: Invalid format descriptor. Format descriptor did not begin with a character that can start a legal format descriptor. Specify correct format descriptor. ");

        fstr::assign(save.MESSGE.get_mut(63), b"Unexpected character found following a number in the format string. Format error:  Character in the set IFEDGMNK@OLAR(PHX expected and not found. Specify correct format descriptor to follow number. or NAMELIST I/O attempted on unformatted file1 An illegal NAMELIST I/O operation was attempted on an unformatted file. OPEN file with FORM=\'FORMATTED\'. ");

        fstr::assign(save.MESSGE.get_mut(64), b"Format error: Trying to scale unscalable format specifier. The specifier being scaled is not \"F\", \"E\", \"D\", \"M\", \"N\", or \"G\". Scale only specifiers for floating-point I/O. or COUNT exceeds buffer length in ENCODE/DECODE1 The count of characters to be transferred exceeds the internal buffer length. Either transfer fewer characters or use a larger buffer. ");

        fstr::assign(save.MESSGE.get_mut(65), b"Format error: Parentheses too deeply nested. Too many left parentheses for the format processor to stack. Nest parentheses less deeply. ");

        fstr::assign(save.MESSGE.get_mut(66), b"Format error: Invalid tab specifier. A specifier beginning with \"T\" is not a correct tab specifier. Correct the specifier beginning with \"T\". ");

        fstr::assign(save.MESSGE.get_mut(67), b"Format error: Invalid blank specifier. A specifier beginning with \"B\" did not have \"N\" or \"Z\" as the next character. Correct the specifier beginning with \"B\". ");

        fstr::assign(save.MESSGE.get_mut(68), b"Format error: Specifier expected but end of format found. The end of the format was reached when another specifier was expected. Check the end of the format for a condition that would lead the processor to look for another specifier (possibly a missing right parenthesis). ");

        fstr::assign(save.MESSGE.get_mut(69), b"Format error: Missing separator. Other specifier found when /, :, or ) expected. Insert separator where needed. ");

        fstr::assign(save.MESSGE.get_mut(70), b"Format error: Digit expected. Number not found following format descriptor requiring a field width. Specify field width where required. ");

        fstr::assign(save.MESSGE.get_mut(71), b"Format error: Period expected in floating point format descriptor. No period was found to specify the number of decimal places in an \"F\", \"G\", \"E\", or \"D\" format descriptor. Specify the number of decimal places for the field. ");

        fstr::assign(save.MESSGE.get_mut(72), b"Format error: Unbalanced parentheses. More right parentheses than left parentheses were found. Correct format so parentheses balance. ");

        fstr::assign(save.MESSGE.get_mut(73), b"Format error: Invalid string in format. String extends past the end of the format or is too long for buffer. Check for unbalanced quotation mark or for \"H\" format count too large; or break up long string. ");

        fstr::assign(save.MESSGE.get_mut(74), b"Record length different in subsequent OPEN. Record length specified in redundant OPEN conflicted with the value as opened. Only BLANK= specifier may be changed by a redundant OPEN. ");

        fstr::assign(save.MESSGE.get_mut(75), b"Record accessed past end of internal file record (variable). An attempt was made to transfer more characters than internal file length. Match READ or WRITE with internal file size. ");

        fstr::assign(save.MESSGE.get_mut(76), b"Illegal new file number requested in fset function. The file number requested to be set was not a legal file system file number. Check that the OPEN succeeded and the file number is correct. ");

        fstr::assign(save.MESSGE.get_mut(77), b"Unexpected character in \"NAMELIST\" read. An illegal character was found in NAMELIST-directed input. Be sure input data conforms to the syntax rules for \"NAMELIST\"-directed input; remove illegal character from data. ");

        fstr::assign(save.MESSGE.get_mut(78), b"Illegal subscript or substring in \"NAMELIST\" read. An invalid subscript or substring specifier was found in NAMELIST-directed input.  Possible causes:  bad syntax, subscript/substring component out-of-bounds, wrong number of subscripts, substring on non-CHARACTER variable. Check input data for syntax errors.  Be sure subscript/substring specifiers are correct for data type; specify only array elements within the bounds of the array being read. ");

        fstr::assign(save.MESSGE.get_mut(79), b"Too many values in \"NAMELIST\" read. Too many input values were found during a NAMELIST-directed READ. This message will be generated by attempts to fill variables beyond their memory limits. Supply only as many values as the length of the array. ");

        fstr::assign(save.MESSGE.get_mut(80), b"Variable not in \"NAMELIST\" group. A variable name was encountered in the input stream which was not declared as part of the current NAMELIST group. Read only the variables in this NAMELIST. ");

        fstr::assign(save.MESSGE.get_mut(81), b"\"NAMELIST\" I/O attempted on unformatted file. An illegal NAMELIST I/O operation was attempted on an unformatted (binary) file. OPEN file with FORM=\'FORMATTED\'; use NAMELIST I/O only on formatted files. ");

        fstr::assign(save.MESSGE.get_mut(82), b"Value out of range in numeric read. Value read for the numeric item is too big or too small. Read only the values that fit in the range of the numeric type being read. ");

        fstr::assign(save.MESSGE.get_mut(83), b"The value of IOSTAT was 982.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(84), b"The value of IOSTAT was 983.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(85), b"The value of IOSTAT was 984.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(86), b"The value of IOSTAT was 985.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(87), b"The value of IOSTAT was 986.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(88), b"The value of IOSTAT was 987.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(89), b"The value of IOSTAT was 988.  No explanation is provided in the HP documentation for this value of IOSTAT. ");

        fstr::assign(save.MESSGE.get_mut(90), b"`Illegal FORTRAN NLS call: FORTRAN source code must be compiled with -Y. The FORTRAN source file was not compiled with the -Y option and NLS features were used. The problem is critical enough that program execution cannot continue. ");
    } else if save.SGI {
        save.LBND = 99;
        save.UBND = 169;

        fstr::assign(save.MESSGE.get_mut(1), b"error in format ");
        fstr::assign(save.MESSGE.get_mut(2), b"out of space for unit table ");
        fstr::assign(save.MESSGE.get_mut(3), b"formatted i/o not allowed ");
        fstr::assign(save.MESSGE.get_mut(4), b"unformatted i/o not allowed ");
        fstr::assign(save.MESSGE.get_mut(5), b"direct i/o not allowed ");
        fstr::assign(save.MESSGE.get_mut(6), b"sequential i/o not allowed ");
        fstr::assign(save.MESSGE.get_mut(7), b"can\'t backspace file ");
        fstr::assign(save.MESSGE.get_mut(8), b"null file name ");
        fstr::assign(save.MESSGE.get_mut(9), b"can\'t stat file ");
        fstr::assign(save.MESSGE.get_mut(10), b"unit not connected ");
        fstr::assign(save.MESSGE.get_mut(11), b"off end of record ");
        fstr::assign(save.MESSGE.get_mut(12), b"truncation failed in end file ");
        fstr::assign(save.MESSGE.get_mut(13), b"incomprehensible list input ");
        fstr::assign(save.MESSGE.get_mut(14), b"out of free space ");
        fstr::assign(save.MESSGE.get_mut(15), b"unit not connected ");
        fstr::assign(save.MESSGE.get_mut(16), b"read unexpected character ");
        fstr::assign(save.MESSGE.get_mut(17), b"blank logical input field ");
        fstr::assign(save.MESSGE.get_mut(18), b"bad variable type ");
        fstr::assign(save.MESSGE.get_mut(19), b"bad namelist name ");
        fstr::assign(save.MESSGE.get_mut(20), b"variable not in namelist ");
        fstr::assign(save.MESSGE.get_mut(21), b"no end record ");
        fstr::assign(save.MESSGE.get_mut(22), b"namelist subscript out of range ");
        fstr::assign(save.MESSGE.get_mut(23), b"negative repeat count ");
        fstr::assign(save.MESSGE.get_mut(24), b"illegal operation for unit ");
        fstr::assign(save.MESSGE.get_mut(25), b"off beginning of record ");
        fstr::assign(save.MESSGE.get_mut(26), b"no * after repeat count ");
        fstr::assign(save.MESSGE.get_mut(27), b"\'new\' file exists ");
        fstr::assign(save.MESSGE.get_mut(28), b"can\'t find \'old\' file ");
        fstr::assign(save.MESSGE.get_mut(29), b"unknown system error ");
        fstr::assign(save.MESSGE.get_mut(30), b"requires seek ability ");
        fstr::assign(save.MESSGE.get_mut(31), b"illegal argument ");
        fstr::assign(save.MESSGE.get_mut(32), b"duplicate key value on write ");
        fstr::assign(save.MESSGE.get_mut(33), b"indexed file not open ");
        fstr::assign(save.MESSGE.get_mut(34), b"bad isam argument ");
        fstr::assign(save.MESSGE.get_mut(35), b"bad key description ");
        fstr::assign(save.MESSGE.get_mut(36), b"too many open indexed files ");
        fstr::assign(save.MESSGE.get_mut(37), b"corrupted isam file ");
        fstr::assign(
            save.MESSGE.get_mut(38),
            b"isam file not opened for exclusive access ",
        );
        fstr::assign(save.MESSGE.get_mut(39), b"record locked ");
        fstr::assign(save.MESSGE.get_mut(40), b"key already exists ");
        fstr::assign(save.MESSGE.get_mut(41), b"cannot delete primary key ");
        fstr::assign(
            save.MESSGE.get_mut(42),
            b"beginning or end of file reached ",
        );
        fstr::assign(save.MESSGE.get_mut(43), b"cannot find requested record ");
        fstr::assign(save.MESSGE.get_mut(44), b"current record not defined ");
        fstr::assign(save.MESSGE.get_mut(45), b"isam file is exclusively locked ");
        fstr::assign(save.MESSGE.get_mut(46), b"filename too long ");
        fstr::assign(save.MESSGE.get_mut(47), b"cannot create lock file ");
        fstr::assign(save.MESSGE.get_mut(48), b"record too long ");
        fstr::assign(
            save.MESSGE.get_mut(49),
            b"key structure does not match file structure ",
        );
        fstr::assign(
            save.MESSGE.get_mut(50),
            b"direct access on an indexed file not allowed ",
        );
        fstr::assign(
            save.MESSGE.get_mut(51),
            b"keyed access on a sequential file not allowed ",
        );
        fstr::assign(
            save.MESSGE.get_mut(52),
            b"keyed access on a relative file not allowed ",
        );
        fstr::assign(
            save.MESSGE.get_mut(53),
            b"append access on an indexed file not allowed ",
        );
        fstr::assign(save.MESSGE.get_mut(54), b"must specify record length ");
        fstr::assign(
            save.MESSGE.get_mut(55),
            b"key field value type does not match key type ",
        );
        fstr::assign(
            save.MESSGE.get_mut(56),
            b"character key field value length too long ",
        );
        fstr::assign(
            save.MESSGE.get_mut(57),
            b"fixed record on sequential file not allowed ",
        );
        fstr::assign(
            save.MESSGE.get_mut(58),
            b"variable records allowed only on unformatted sequential file ",
        );
        fstr::assign(
            save.MESSGE.get_mut(59),
            b"stream records allowed only on formatted sequential file ",
        );
        fstr::assign(
            save.MESSGE.get_mut(60),
            b"maximum number of records in direct access file exceeded ",
        );
        fstr::assign(
            save.MESSGE.get_mut(61),
            b"attempt to write to a readonly file ",
        );
        fstr::assign(save.MESSGE.get_mut(62), b"must specify key descriptions ");
        fstr::assign(
            save.MESSGE.get_mut(63),
            b"carriage control not allowed for unformatted units ",
        );
        fstr::assign(save.MESSGE.get_mut(64), b"indexed files only ");
        fstr::assign(save.MESSGE.get_mut(65), b"cannot use on indexed file ");
        fstr::assign(
            save.MESSGE.get_mut(66),
            b"cannot use on indexed or append file ",
        );
        fstr::assign(save.MESSGE.get_mut(67), b"error in closing file ");
        fstr::assign(
            save.MESSGE.get_mut(68),
            b"invalid code in format specification ",
        );
        fstr::assign(
            save.MESSGE.get_mut(69),
            b"invalid record number in direct access file ",
        );
        fstr::assign(
            save.MESSGE.get_mut(70),
            b"cannot have endfile record on non-sequential file ",
        );
    } else if save.VAX {
        save.LBND = 0;
        save.UBND = 68;

        fstr::assign(save.MESSGE.get_mut(1), b"Not a Fortran-specific error. ");
        fstr::assign(
            save.MESSGE.get_mut(2),
            b"No diagnostics are available other than the value of IOSTAT is 2 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(3),
            b"No diagnostics are available other than the value of IOSTAT is 3 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(4),
            b"No diagnostics are available other than the value of IOSTAT is 4 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(5),
            b"No diagnostics are available other than the value of IOSTAT is 5 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(6),
            b"No diagnostics are available other than the value of IOSTAT is 6 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(7),
            b"No diagnostics are available other than the value of IOSTAT is 7 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(8),
            b"No diagnostics are available other than the value of IOSTAT is 8 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(9),
            b"No diagnostics are available other than the value of IOSTAT is 9 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(10),
            b"No diagnostics are available other than the value of IOSTAT is 10 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(11),
            b"No diagnostics are available other than the value of IOSTAT is 11 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(12),
            b"No diagnostics are available other than the value of IOSTAT is 12 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(13),
            b"No diagnostics are available other than the value of IOSTAT is 13 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(14),
            b"No diagnostics are available other than the value of IOSTAT is 14 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(15),
            b"No diagnostics are available other than the value of IOSTAT is 15 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(16),
            b"No diagnostics are available other than the value of IOSTAT is 16 ",
        );
        fstr::assign(save.MESSGE.get_mut(17), b"Syntax error in NAMELIST input. ");
        fstr::assign(
            save.MESSGE.get_mut(18),
            b"Too many values for NAMELIST variable. ",
        );
        fstr::assign(
            save.MESSGE.get_mut(19),
            b"Invalid reference to variable in NAMELIST input. ",
        );
        fstr::assign(save.MESSGE.get_mut(20), b"REWIND error. ");
        fstr::assign(save.MESSGE.get_mut(21), b"Duplicate file specifications. ");
        fstr::assign(save.MESSGE.get_mut(22), b"Input record too long. ");
        fstr::assign(save.MESSGE.get_mut(23), b"BACKSPACE error ");
        fstr::assign(save.MESSGE.get_mut(24), b"End-of-file during read. ");
        fstr::assign(save.MESSGE.get_mut(25), b"Record number outside range. ");
        fstr::assign(save.MESSGE.get_mut(26), b"OPEN or DEFINE FILE required. ");
        fstr::assign(
            save.MESSGE.get_mut(27),
            b"Too many records in IO statement. ",
        );
        fstr::assign(save.MESSGE.get_mut(28), b"CLOSE error. ");
        fstr::assign(save.MESSGE.get_mut(29), b"File not found. ");
        fstr::assign(save.MESSGE.get_mut(30), b"Open failure. ");
        fstr::assign(save.MESSGE.get_mut(31), b"Mixed file access modes. ");
        fstr::assign(save.MESSGE.get_mut(32), b"Invalid logical unit number. ");
        fstr::assign(save.MESSGE.get_mut(33), b"ENDFILE error. ");
        fstr::assign(save.MESSGE.get_mut(34), b"Unit already open. ");
        fstr::assign(save.MESSGE.get_mut(35), b"Segmented record format error. ");
        fstr::assign(
            save.MESSGE.get_mut(36),
            b"Attempt to access non-existent record. ",
        );
        fstr::assign(save.MESSGE.get_mut(37), b"Inconsistent record length. ");
        fstr::assign(save.MESSGE.get_mut(38), b"Error during write. ");
        fstr::assign(save.MESSGE.get_mut(39), b"Error during read. ");
        fstr::assign(save.MESSGE.get_mut(40), b"Recursive IO operation. ");
        fstr::assign(save.MESSGE.get_mut(41), b"Insufficient virtual memory. ");
        fstr::assign(save.MESSGE.get_mut(42), b"No such device. ");
        fstr::assign(save.MESSGE.get_mut(43), b"File name specification error. ");
        fstr::assign(save.MESSGE.get_mut(44), b"Inconsistent record type. ");
        fstr::assign(
            save.MESSGE.get_mut(45),
            b"Keyword value error in OPEN statement. ",
        );
        fstr::assign(
            save.MESSGE.get_mut(46),
            b"Inconsistent OPENCLOSE parameters. ",
        );
        fstr::assign(save.MESSGE.get_mut(47), b"Write to READONLY file. ");
        fstr::assign(
            save.MESSGE.get_mut(48),
            b"Invalid argument to Fortran Run-Time Library. ",
        );
        fstr::assign(save.MESSGE.get_mut(49), b"Invalid key specification. ");
        fstr::assign(
            save.MESSGE.get_mut(50),
            b"Inconsistent key change or duplicate key. ",
        );
        fstr::assign(save.MESSGE.get_mut(51), b"Inconsistent file organization. ");
        fstr::assign(save.MESSGE.get_mut(52), b"Specified record locked. ");
        fstr::assign(save.MESSGE.get_mut(53), b"No current record. ");
        fstr::assign(save.MESSGE.get_mut(54), b"REWRITE error. ");
        fstr::assign(save.MESSGE.get_mut(55), b"DELETE error. ");
        fstr::assign(save.MESSGE.get_mut(56), b"UNLOCK error. ");
        fstr::assign(save.MESSGE.get_mut(57), b"FIND error. ");
        fstr::assign(
            save.MESSGE.get_mut(58),
            b"No diagnostics are available other than the value of IOSTAT is 58 ",
        );
        fstr::assign(save.MESSGE.get_mut(59), b"List-directed IO syntax error. ");
        fstr::assign(save.MESSGE.get_mut(60), b"Infinite format loop. ");
        fstr::assign(save.MESSGE.get_mut(61), b"Formatvariable-type mismatch. ");
        fstr::assign(save.MESSGE.get_mut(62), b"Syntax error in format. ");
        fstr::assign(save.MESSGE.get_mut(63), b"Output conversion error. ");
        fstr::assign(save.MESSGE.get_mut(64), b"Input conversion error. ");
        fstr::assign(
            save.MESSGE.get_mut(65),
            b"No diagnostics are available other than the value of IOSTAT is 65 ",
        );
        fstr::assign(
            save.MESSGE.get_mut(66),
            b"Output statement overflows record. ",
        );
        fstr::assign(
            save.MESSGE.get_mut(67),
            b"Input statement requires too much data. ",
        );
        fstr::assign(
            save.MESSGE.get_mut(68),
            b"Variable format expression value error. ",
        );
    } else if save.PC {
        save.LBND = 2;
        save.UBND = 1;
    } else {
        save.LBND = 2;
        save.UBND = 1;
    }

    if ((IOSTAT > save.LBND) && (IOSTAT <= save.UBND)) {
        fstr::assign(DIAGNS, save.MESSGE.get((IOSTAT - save.LBND)));
        *FOUND = true;
    } else {
        fstr::assign(DIAGNS, b"The value of IOSTAT was #.  The meaning of this value is not available via the SPICE system. Please consult your FORTRAN manual for the meaning of this code.");

        spicelib::REPMI(&DIAGNS.to_vec(), b"#", IOSTAT, DIAGNS, ctx);
        *FOUND = false;
    }
}
