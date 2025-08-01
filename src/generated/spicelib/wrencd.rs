//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QUOTE: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"\'");
const MAXENC: i32 = 64;
const WRKSIZ: i32 = 64;

/// Write encoded d.p. numbers to text file
///
/// Encode and write d.p. numbers to a text file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT      I    Fortran unit number of output text file.
///  N         I    Number of d.p. numbers to encode and write.
///  DATA      I    List of d.p. numbers to encode and write.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for a previously opened text
///           file. All writing will begin at the CURRENT POSITION
///           in the text file.
///
///  N        is the number of double precision numbers to be encoded
///           and written to the text file attached to UNIT.
///
///  DATA     is the list of double precision numbers to be encoded and
///           written to the text file attached to UNIT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the $Particulars section for a description of the effect of
///  this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If N, the number of data items, is not positive, the error
///      SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If an error occurs while writing to the text file attached
///      to UNIT, the error SPICE(FILEWRITEFAILED) is signaled.
///
///  3)  If the Fortran logical unit UNIT is not defined, the results
///      of this routine are unpredictable.
/// ```
///
/// # Files
///
/// ```text
///  See the description of UNIT in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will accept a list of one or more double precision
///  numbers which it will encode into equivalent text strings and
///  write to the current position in a text file. The current
///  position in a file is defined to be the text line immediately
///  following the last text line that was written or read. The
///  encoded d.p. numbers are written to the output text file as
///  quoted character strings so that a Fortran list directed read may
///  be used to read the encoded values, rather than a formatted read
///  with the format specifier FMT = '(A)'.
///
///  This routine is one of a pair of routines which are used to
///  encode and decode d.p. numbers:
///
///        WRENCD -- Encode and write d.p. numbers to a file.
///        RDENCD -- Read and decode d.p. numbers from a file.
///
///  The encoding/decoding of d.p.numbers is performed to provide a
///  portable means for transferring data values.
///
///  Currently the text string produced will be in a base 16
///  ``scientific notation.'' This format retains the full precision
///  available for d.p. numbers on any given computer architecture.
///  See DP2HX.FOR and HX2DP.FOR for details.
/// ```
///
/// # Examples
///
/// ```text
///  Please note that the output format in the examples is not
///  intended to be exactly identical with the output format of this
///  routine in actual use. The output format used in the examples is
///  intended to aid in the understanding of how this routine works.
///  It is NOT intended to be a specification of the output format for
///  this routine.
///
///  Let
///
///     UNIT     be the Fortran logical unit of a previously opened
///              text file.
///
///     N        = 100
///
///     DATA(I)  = DBLE(I), I = 1,N
///
///  Then, the subroutine call
///
///        CALL WRENCD( UNIT, N, DATA )
///
///  will write the first 100 integers as encoded d.p. numbers to the
///  output text file attached to UNIT, beginning at the current
///  position in the output file, which is marked by an arrow, '-->'.
///  The resulting output will look something like the following:
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
///     -->
///
///  At this point, the arrow marks the position of the file pointer
///  immediately after the call to WRENCD.
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
/// -    SPICELIB Version 1.3.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 09-SEP-1993 (KRG)
///
///         The list directed write was changed to a formatted write using
///         the specifier FMT='(A)'. This was done in order to prevent a
///         space from appearing as the first character on each line of the
///         file for certain computer platforms.
///
/// -    SPICELIB Version 1.1.0, 21-JUN-1993 (KRG)
///
///         This routine was modified to avoid the creation of long output
///         lines on some of the supported systems, such as the NeXT with
///         Absoft Fortran 3.2.
///
///         A disclaimer was added to the $Examples section concerning
///         the output format used. The disclaimer simply states that the
///         output format used in the example is not necessarily the
///         output format actually used by the routine.
///
/// -    SPICELIB Version 1.0.0, 20-OCT-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 09-SEP-1993 (KRG)
///
///         The list directed write was changed to a formatted write using
///         the  specifier FMT='(A)'. This was done in order to prevent a
///         space from appearing as the first character on each line of the
///         file for certain computer platforms.
///
/// -    SPICELIB Version 1.1.0, 21-JUN-1993 (KRG)
///
///         This routine was modified to avoid the creation of long output
///         lines on some of the supported systems, such as the NeXT with
///         Absoft Fortran 3.2.
///
///         On some of the supported computers this routine would produce
///         very long (greater than 1000 characters) output lines due to
///         the implicit DO loop used in the WRITE statement:
///
///             WRITE (UNIT,IOSTAT=IOSTAT,FMT=*)
///            .   ( QUOTE//WORK(I)(1:LENGTH(I))//QUOTE//' ', I=1,NITMS )
///
///         This problem was fixed by removing the implicit DO loop from
///         the WRITE statement and placing an equivalent DO loop around
///         the WRITE statement:
///
///             DO I = 1, NITMS
///                WRITE (UNIT,IOSTAT=IOSTAT,FMT=*)
///            .       QUOTE//WORK(I)(1:LENGTH(I))//QUOTE
///             END DO
///
///         The net effect of this will be that only a single datum will
///         be written on each line of output.
///
///         A disclaimer was added to the $Examples section concerning
///         the output format used. The disclaimer simply states that the
///         output format used in the example is not necessarily the
///         output format actually used by the routine.
///
/// -    SPICELIB Version 1.0.0, 20-OCT-1992 (KRG)
/// ```
pub fn wrencd(ctx: &mut SpiceContext, unit: i32, n: i32, data: &[f64]) -> crate::Result<()> {
    WRENCD(unit, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRENCD ( Write encoded d.p. numbers to text file )
pub fn WRENCD(UNIT: i32, N: i32, DATA: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let DATA = DummyArray::new(DATA, 1..);
    let mut WORK = ActualCharArray::new(MAXENC, 1..=WRKSIZ);
    let mut IOSTAT: i32 = 0;
    let mut ITMBEG: i32 = 0;
    let mut NITMS: i32 = 0;
    let mut LENGTH = StackArray::<i32, 64>::new(1..=WRKSIZ);

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
        CHKIN(b"WRENCD", ctx)?;
    }
    //
    // Check to see if the number of data items is less than or equal
    // to zero. If it is, signal an error.
    //
    if (N < 1) {
        SETMSG(
            b"The number of data items to be written was not positive: #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"WRENCD", ctx)?;
        return Ok(());
    }
    //
    // Initialize the beginning location for the data items to be
    // encoded.
    //
    ITMBEG = 1;
    //
    // Begin encoding the input data items in blocks of size NITMS.
    // Each time the number of data items NITMS is reached, write
    // out the encoded items in the workspace.
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
        // Encode each of the numbers into an equivalent character string.
        //
        for I in 1..=NITMS {
            DP2HX(DATA[((ITMBEG + I) - 1)], &mut WORK[I], &mut LENGTH[I], ctx);
        }
        //
        // Write out the current workspace, placing single quotes around
        // each of the character strings so that they may be read using
        // Fortran list directed read statements rather than the format
        // specifier FMT = '(A)'.
        //
        for I in 1..=NITMS {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&fstr::concat(
                        &fstr::concat(QUOTE, fstr::substr(WORK.get(I), 1..=LENGTH[I])),
                        QUOTE,
                    ))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }
            //
            // Check to see if we got a write error, IOSTAT .NE. 0.
            //
            if (IOSTAT != 0) {
                SETMSG(b"Error writing to logical unit #, IOSTAT = #.", ctx);
                ERRINT(b"#", UNIT, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"WRENCD", ctx)?;
                return Ok(());
            }
        }
        //
        // Position the data item pointer at the next location to begin
        // encoding the items in the array DATA, and continue processing
        // the data items until done.
        //
        ITMBEG = (ITMBEG + NITMS);
    }

    CHKOUT(b"WRENCD", ctx)?;
    Ok(())
}
