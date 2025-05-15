//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ERRLEN: i32 = 80;
const MAXENC: i32 = 64;
const WRKSIZ: i32 = 64;

/// Read encoded d.p. numbers from file
///
/// Read N encoded d.p. numbers from a text file, decoding them
/// into their equivalent d.p. numbers.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT      I    Fortran unit number of input text file.
///  N         I    Number of d.p. numbers to read and decode.
///  DATA      O    List of decoded d.p. numbers.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for a previously opened text
///           file. All reading will begin at the CURRENT POSITION
///           in the text file.
///
///  N        is the number of encoded double precision numbers, to be
///           read from the text file attached to UNIT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     is the list of decoded double precision numbers read from
///           the text file attached to UNIT.
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
///  This routine will read N encoded double precision numbers from
///  the current position in a previously opened text file. The
///  current position in a file is defined to be the text line
///  immediately following the last text line that was written or
///  read. The numbers will be decoded and placed into a list of
///  double precision numbers which will be passed back to the caller.
///  The encoded double precision numbers are represented as quoted
///  character strings so that a Fortran list directed read may be
///  used to read the encoded values, rather than a formatted read
///  with the format specifier FMT = '(A)'.
///
///  This routine is one of a pair of routines which are used to
///  encode and decode d.p. numbers:
///
///        WRENCD -- Encode and write d.p. numbers to a file.
///        RDENCD -- Read and decode d.p. numbers from a file.
///
///  The encoding/decoding of d.p. numbers is performed to provide a
///  portable means for transferring data values.
///
///  Currently the encoded d.p. numbers are represented in a base
///  16 ``scientific notation.'' See DP2HX.FOR and HX2DP.FOR for
///  details.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose we have the following input file which contains the
///  values 1.0D0 - 100.0D0 in encoded format, and that the input
///  file has already been opened for reading. The arrow, '-->',
///  indicates the current position in the file.
///
///     -->'1^1' '2^1' '3^1' '4^1' '5^1' '6^1' '7^1' '8^1' '9^1'
///        'A^1' 'B^1' 'C^1' 'D^1' 'E^1' 'F^1' '1^2' '11^2' '12^2'
///        '13^2' '14^2' '15^2' '16^2' '17^2' '18^2' '19^2' '1A^2'
///        '1B^2' '1C^2' '1D^2' '1E^2' '1F^2' '2^2' '21^2' '22^2'
///        '23^2' '24^2' '25^2' '26^2' '27^2' '28^2' '29^2' '2A^2'
///        '2B^2' '2C^2' '2D^2' '2E^2' '2F^2' '3^2' '31^2' '32^2'
///        '33^2' '34^2' '35^2' '36^2' '37^2' '38^2' '39^2' '3A^2'
///        '3B^2' '3C^2' '3D^2' '3E^2' '3F^2' '4^2'
///        '41^2' '42^2' '43^2' '44^2' '45^2' '46^2' '47^2' '48^2'
///        '49^2' '4A^2' '4B^2' '4C^2' '4D^2' '4E^2' '4F^2' '5^2'
///        '51^2' '52^2' '53^2' '54^2' '55^2' '56^2' '57^2' '58^2'
///        '59^2' '5A^2' '5B^2' '5C^2' '5D^2' '5E^2' '5F^2' '6^2'
///        '61^2' '62^2' '63^2' '64^2'
///
///  Then the following code fragment would read and decode these
///  100 values.
///
///        N = 100
///        CALL RDENCD( UNIT, N, DATA )
///
///   Upon returning, the array data would contain the values
///   1.0D0 - 100.0D0.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument DATA in $Brief_I/O table.
///
/// -    SPICELIB Version 1.1.0, 19-MAR-1999 (FST)
///
///         Modified the long error message for SPICE(FILEREADFAILED)
///         to indicate the possibility of an incomplete text transfer
///         file as the cause.
///
/// -    SPICELIB Version 1.0.0, 20-OCT-1992 (KRG)
/// ```
pub fn rdencd(ctx: &mut SpiceContext, unit: i32, n: i32, data: &mut [f64]) -> crate::Result<()> {
    RDENCD(unit, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDENCD  ( Read encoded d.p. numbers from file )
pub fn RDENCD(UNIT: i32, N: i32, DATA: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut ERRMSG = [b' '; ERRLEN as usize];
    let mut WORK = ActualCharArray::new(MAXENC, 1..=WRKSIZ);
    let mut ITMBEG: i32 = 0;
    let mut NITMS: i32 = 0;
    let mut IOSTAT: i32 = 0;
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
        CHKIN(b"RDENCD", ctx)?;
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
        CHKOUT(b"RDENCD", ctx)?;
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
    //            (2) there is an error decoding the number.
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
            SETMSG(b"Error reading from logical unit #, IOSTAT = #. One possible cause is an incomplete text transfer file.", ctx);
            ERRINT(b"#", UNIT, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            CHKOUT(b"RDENCD", ctx)?;
            return Ok(());
        }
        //
        // Begin to decode the data items into the data array. Signal an
        // error if we cannot decode a data item.
        //
        for I in 1..=NITMS {
            HX2DP(
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
                CHKOUT(b"RDENCD", ctx)?;
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

    CHKOUT(b"RDENCD", ctx)?;
    Ok(())
}
