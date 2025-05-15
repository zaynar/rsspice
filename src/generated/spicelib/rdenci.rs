//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ERRLEN: i32 = 80;
const MAXENC: i32 = 64;
const WRKSIZ: i32 = 64;

/// Read encoded integers from text file
///
/// Read N encoded integers from a text file, decoding them into
/// their equivalent integers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT      I    Fortran unit number of input text file.
///  N         I    Number of integers to read and decode.
///  DATA      O    List of decoded integers.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for a previously opened text
///           file. All reading will begin at the CURRENT POSITION
///           in the text file.
///
///  N        is the number of encoded integers to be read from the
///           text file attached to UNIT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     is the list of decoded integers read from the text file
///           attached to UNIT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If N, the number of data items, is not positive, the error
///      SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If an error occurs while reading from the text file attached
///      to UNIT, the error SPICE(FILEREADFAILED) is signaled.
///
///  3)  If an error occurs while decoding a number, the error
///      SPICE(DECODINGERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of UNIT in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will read N encoded integers beginning at the
///  current position in a previously opened text file. The current
///  position in a file is defined to be the text line immediately
///  following the last text line that was written or read. The
///  integers will be decoded and placed into a list of integers
///  which will be passed back to the caller. The encoded integers
///  are represented as quoted character strings so that a Fortran
///  list directed read may be used to read the encoded values,
///  rather than a formatted read with the format specifier
///  FMT = '(A)'.
///
///  This routine is one of a pair of routines which are used to
///  encode and decode integers:
///
///        WRENCI -- Encode and write integers to a file.
///        RDENCI -- Read and decode integers from a file.
///
///  The encoding/decoding of integers is performed to provide a
///  portable means for transferring data values.
///
///  Currently the encoded integers are represented as signed
///  hexadecimal numbers See INT2HX.FOR and HX2INT.FOR for details.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose we have the following input file which contains the values
///  1 - 100 encoded, and that the input file has already been opened
///  for reading. The arrow, '-->', indicates the current position in
///  the file.
///
///     -->'1' '2' '3' '4' '5' '6' '7' '8' '9' 'A' 'B' 'C' 'D' 'E'
///        'F' '10' '11' '12' '13' '14' '15' '16' '17' '18' '19'
///        '1A' '1B' '1C' '1D' '1E' '1F' '20' '21' '22' '23' '24'
///        '25' '26' '27' '28' '29' '2A' '2B' '2C' '2D' '2E' '2F'
///        '30' '31' '32' '33' '34' '35' '36' '37' '38' '39' '3A'
///        '3B' '3C' '3D' '3E' '3F' '40'
///        '41' '42' '43' '44' '45' '46' '47' '48' '49' '4A' '4B'
///        '4C' '4D' '4E' '4F' '50' '51' '52' '53' '54' '55' '56'
///        '57' '58' '59' '5A' '5B' '5C' '5D' '5E' '5F' '60' '61'
///        '62' '63' '64'
///
///  Then, the following code fragment would read and decode these
///  100 values.
///
///        N = 100
///        CALL RDENCI( UNIT, N, DATA )
///
///   Upon returning, the array data would contain the values 1 - 100.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument DATA in $Brief_I/O table.
///
/// -    SPICELIB Version 1.0.0, 19-OCT-1992 (KRG)
/// ```
pub fn rdenci(ctx: &mut SpiceContext, unit: i32, n: i32, data: &mut [i32]) -> crate::Result<()> {
    RDENCI(unit, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDENCI  ( Read encoded integers from text file )
pub fn RDENCI(UNIT: i32, N: i32, DATA: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut ERRMSG = [b' '; ERRLEN as usize];
    let mut WORK = ActualCharArray::new(MAXENC, 1..=WRKSIZ);
    let mut IOSTAT: i32 = 0;
    let mut ITMBEG: i32 = 0;
    let mut NITMS: i32 = 0;
    let mut ERROR: bool = false;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
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
        CHKIN(b"RDENCI", ctx)?;
    }
    //
    // Check to see if the number of data items is less than or equal
    // to zero. If it is, signal an error.
    //
    if (N < 1) {
        SETMSG(
            b"The number of data items to be read was not positive: #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"RDENCI", ctx)?;
        return Ok(());
    }
    //
    // Initialize the beginning location to place the decoded data
    // items.
    //
    ITMBEG = 1;
    //
    // We read in the encoded numbers in blocks of size WRKSIZ, and if
    // there was not a read error we will attempt to decode the numbers.
    // We signal an error if either:
    //
    //            (1) there is a read error
    //            (2) there is an error decoding a number.
    //
    // NOTE: EOF is interpreted as a read error because we know a priori
    //       exactly how many data items we need to read: N.
    //
    // Begin decoding the encoded data items read from the input file
    // in blocks of size NITMS. Each time the number of data items
    // NITMS is reached, decode the encoded numbers into the data array.
    //
    while (ITMBEG <= N) {
        //
        // The number of items is either the size of the workspace, or
        // the number of data items which remain to be processed, which
        // should always be less than or equal to the size of the
        // workspace.
        //
        NITMS = intrinsics::MIN0(&[WRKSIZ, ((N - ITMBEG) + 1)]);
        //
        // Read in a block of data items to be decoded.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::ListDirectedReader::new(ctx.io_unit(UNIT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                for I in intrinsics::range(1, NITMS, 1) {
                    reader.read_str(&mut WORK[I])?;
                }
                reader.finish()?;
                Ok(())
            })?;
        }
        //
        // Check to see if we got a read error: IOSTAT .NE. 0. If we did,
        // then signal an error. EOF is considered to be a read error,
        // since we know exactly how many data items we expect to read.
        //
        if (IOSTAT != 0) {
            SETMSG(b"Error reading from logical unit #, IOSTAT = #.", ctx);
            ERRINT(b"#", UNIT, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"RDENCI", ctx)?;
            return Ok(());
        }
        //
        // Begin to decode the data items into the data array. Signal an
        // error if we cannot decode a data item.
        //
        for I in 1..=NITMS {
            HX2INT(
                &WORK[I],
                &mut DATA[((ITMBEG + I) - 1)],
                &mut ERROR,
                &mut ERRMSG,
                ctx,
            );
            if ERROR {
                SETMSG(
                    b"Decoding error occurred while attempting to decode item #: #. #",
                    ctx,
                );
                ERRINT(b"#", I, ctx);
                ERRCH(b"#", &WORK[I], ctx);
                ERRCH(b"#", &ERRMSG, ctx);
                SIGERR(b"SPICE(DECODINGERROR)", ctx)?;
                CHKOUT(b"RDENCI", ctx)?;
                return Ok(());
            }
        }
        //
        // Position the data item pointer at the next location to begin
        // placing the decoded items in the array DATA, and continue
        // processing the until done.
        //
        ITMBEG = (ITMBEG + NITMS);
    }

    CHKOUT(b"RDENCI", ctx)?;
    Ok(())
}
