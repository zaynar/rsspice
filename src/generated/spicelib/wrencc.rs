//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QUOTE: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"\'");
const ESCCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"@");
const CHRFPC: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b" ");
const CHRLPC: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"~");
const MAXENC: i32 = 64;
const HEXBAS: i32 = 16;

struct SaveVars {
    HEXDIG: ActualCharArray,
    INTESC: i32,
    INTFPC: i32,
    INTLPC: i32,
    INTQUO: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut HEXDIG = ActualCharArray::new(1, 0..=15);
        let mut INTESC: i32 = 0;
        let mut INTFPC: i32 = 0;
        let mut INTLPC: i32 = 0;
        let mut INTQUO: i32 = 0;
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"0"),
                Val::C(b"1"),
                Val::C(b"2"),
                Val::C(b"3"),
                Val::C(b"4"),
                Val::C(b"5"),
                Val::C(b"6"),
                Val::C(b"7"),
                Val::C(b"8"),
                Val::C(b"9"),
                Val::C(b"A"),
                Val::C(b"B"),
                Val::C(b"C"),
                Val::C(b"D"),
                Val::C(b"E"),
                Val::C(b"F"),
            ]
            .into_iter();
            HEXDIG
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;

        Self {
            HEXDIG,
            INTESC,
            INTFPC,
            INTLPC,
            INTQUO,
            FIRST,
        }
    }
}

fn ZZICHR(CARG: &[u8]) -> i32 {
    let CARG = &CARG[..1 as usize];
    (intrinsics::ICHAR(CARG)
        - (intrinsics::MAX0(&[-1, intrinsics::MIN0(&[0, intrinsics::ICHAR(CARG)])]) * 256))
}

/// Write characters to text file encoded
///
/// Encode and write characters to a text file.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT      I    Fortran unit number of output text file.
///  N         I    Number of characters to encode and write.
///  DATA      I    List of characters to encode and write.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the Fortran unit number for a previously opened text
///           file. All writing will begin at the CURRENT POSITION
///           in the text file.
///
///  N        is the number of data items, characters, to be encoded
///           and written to the text file attached to UNIT.
///
///  DATA     is the list of characters to be encoded and written to
///           the text file attached to UNIT.
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
///      to unit UNIT, the error SPICE(FILEWRITEFAILED) is signaled.
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
///  This routine will encode and write the first N contiguous
///  characters contained in the data buffer array DATA. The
///  encoded characters will be written to a previously opened
///  text file attached to logical unit UNIT beginning at the
///  current position in the file. The current position in a
///  file is defined to be the text line immediately following
///  the last text line that was written or read.
///
///  The first N contiguous characters in the data buffer array
///  DATA are defined to be those N characters encountered while
///  moving from the lowest array indices to highest array indices,
///  i.e., those characters encountered while moving from ``left''
///  to ``right'' and ``top'' to ``bottom'' in the character array
///  DATA, beginning at the first character position, DATA(1)(1:1).
///  Logically all of the array elements in the data buffer DATA
///  containing characters to be encoded can be thought of as being
///  concatenated together into one long character string.
///
///  On any single call to this routine, the encoded characters
///  will be contiguous when written, and all but possibly the
///  final character string written to the file will contain
///  MAXENC characters. The last, if it does not contain MAXENC
///  characters, will be padded with blanks so that it has a
///  length of MAXENC characters. The encoded character strings
///  are meant to be read and processed in blocks of MAXENC
///  characters.
///
///  This routine is one of a pair of routines which are used to
///  encode and decode ASCII characters:
///
///        WRENCC -- Encode and write ASCII characters to a file.
///        RDENCC -- Read and decode ASCII characters from a file.
///
///  The encoding/decoding of characters is performed to provide
///  a portable means for transferring character data values.
///
///  The encoded characters are written to the output text file as
///  quoted character strings so that a Fortran list directed read
///  may be used to read the character strings, rather than a Fortran
///  formatted read with format specifier FMT = '(A)'.
///
///  This routine is for use with the ASCII character set and
///  extensions to it. The supported characters must have decimal
///  values in the range from 0 to 255.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples demonstrate the use of this routine. In
///  each of the examples, the variable UNIT is the Fortran logical
///  unit of a previously opened text file, and the variable N is
///  an integer which will represent the number of characters to be
///  encoded.
///
///  The first example demonstrates a typical correct usage of this
///  routine. The second example demonstrates what would probably
///  be the most common incorrect usage of this routine. The first
///  two examples are attempting to encode the sentence 'This is the
///  data.', which has a length of N = 17 characters. The third
///  example presents ``before'' and ``after'' pictures of the complete
///  ASCII character set.
///
///  Example 1
///  ---------
///
///     This example demonstrates a typical usage of this routine.
///
///     Let the character data buffer have the following declaration
///     in the calling program:
///
///        CHARACTER*(4)         DATA(5)
///
///     We make the following variable assignments:
///
///        DATA(1) = 'This'
///        DATA(2) = ' is '
///        DATA(3) = 'the '
///        DATA(4) = 'data'
///        DATA(5) = '.'
///        N = 17
///
///     The subroutine call
///
///        CALL WRENCC( UNIT, N, DATA )
///
///     will produce a record in the text file attached to the
///     logical unit UNIT which is identical to the following
///     except for the length of the character string written.
///
///        'This is the data.                             '
///
///
///  Example 2
///  ---------
///
///     This example is meant to demonstrate what would probably be
///     a common misuse of this routine.
///
///     Let the character data buffer have the following declaration
///     in the calling program:
///
///        CHARACTER*(10)         DATA(2)
///
///     We make the following variable assignments:
///
///        DATA(1) = 'This is'
///        DATA(2) = ' the data.'
///        N = 17
///
///     The subroutine call
///
///        CALL WRENCC( UNIT, N, DATA )
///
///     will produce a record in the text file attached to the
///     logical unit UNIT which is identical to the following
///     except for the length of the character string written.
///
///        'This is    the da                             '
///
///     This is probably not what was intended. The problem is that
///     all of the characters which were to be encoded did not appear
///     contiguously in the data buffer DATA. The first element of the
///     character string array DATA has three ``extra'' blanks
///     following the 's' in the word 'is'. To correctly encode the
///     data, the following assignments should be made:
///
///        DATA(1) = 'This is th'
///        DATA(2) = 'e data.'
///
///  Example 3
///  ---------
///
///     This example presents the results of applying WRENCC to
///     the complete ASCII character set and an extension with
///     characters having decimal values form 128 to 255.
///
///     Let the character data buffer have the following declaration
///     in the calling program:
///
///        CHARACTER*(1)          DATA(0:255)
///
///     Then, letting
///
///        DATA(I) = CHAR( I ), I = 0, 255
///        N = 256
///
///     the subroutine call
///
///        CALL WRENCC( UNIT, N, DATA )
///
///     would produce
///
///  '@00@01@02@03@04@05@06@07@08@09@0A@0B@0C@0D@0E@0F@10@11@12@13@14@'
///  '15@16@17@18@19@1A@1B@1C@1D@1E@1F !"#$%&@27()*+,-./0123456789:;<='
///  '>?@40ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{'
///  '|}~@7F@80@81@82@83@84@85@86@87@88@89@8A@8B@8C@8D@8E@8F@90@91@92@'
///  '93@94@95@96@97@98@99@9A@9B@9C@9D@9E@9F@A0@A1@A2@A3@A4@A5@A6@A7@A'
///  '8@A9@AA@AB@AC@AD@AE@AF@B0@B1@B2@B3@B4@B5@B6@B7@B8@B9@BA@BB@BC@BD'
///  '@BE@BF@C0@C1@C2@C3@C4@C5@C6@C7@C8@C9@CA@CB@CC@CD@CE@CF@D0@D1@D2@'
///  'D3@D4@D5@D6@D7@D8@D9@DA@DB@DC@DD@DE@DF@E0@E1@E2@E3@E4@E5@E6@E7@E'
///  '8@E9@EA@EB@EC@ED@EE@EF@F0@F1@F2@F3@F4@F5@F6@F7@F8@F9@FA@FB@FC@FD'
///  '@FE@FF                                                          '
///
///  Example 4
///  ---------
///
///     This example demonstrates the use of WRENCC and RDENCC for
///     writing and subsequent reading of character data using data
///     buffers that are ``shaped'' differently, i.e., that have
///     different dimensions.
///
///     Let the input and output character data buffers have the
///     following declarations:
///
///        CHARACTER*(25)  OUTBUF(3)
///        CHARACTER*(10)  INPBUF(7)
///
///     Further, let the output buffer contain the following data:
///
///        OUTBUF(1) = 'Today is the first day of'
///        OUTBUF(2) = ' the rest of my life, so '
///        OUTBUF(3) = 'I will enjoy it.'
///
///     There are exactly N = 66 significant characters in the output
///     buffer. The code fragment
///
///        N = 66
///        CALL WRENCC ( UNIT, N, OUTBUF )
///        REWIND ( UNIT )
///        CALL RDENCC ( UNIT, N, INPBUF )
///
///     has the effect of placing the original data into the
///     differently ``shaped'' input buffer with the following
///     results:
///
///        INPBUF(1) = 'Today is t'
///        INPBUF(2) = 'he first d'
///        INPBUF(3) = 'ay of the '
///        INPBUF(4) = 'rest of my'
///        INPBUF(5) = ' life, so '
///        INPBUF(6) = 'I will enj'
///        INPBUF(7) = 'oy it.    '
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.30.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 1.29.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.28.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 1.27.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 1.26.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 1.25.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 1.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 1.23.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 1.22.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 1.21.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 1.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 1.19.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 1.18.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 1.17.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 1.16.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 1.15.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 1.14.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 1.13.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 1.12.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 1.11.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 1.10.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 1.9.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 1.8.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 1.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 1.6.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 1.5.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 1.4.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 1.3.1, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 1.3.0, 05-DEC-2001 (FST)
///
///         Replaced ICHAR with the statement function ZZICHR
///         to fix a problem on some PC-LINUX environments.
///
/// -    SPICELIB Version 1.2.0, 09-SEP-1993 (KRG)
///
///         The list directed write was changed to a formatted write using
///         the specifier FMT='(A)'. This was done in order to prevent a
///         space from appearing as the first character on each line of the
///         file for certain computer platforms.
///
/// -    SPICELIB Version 1.1.0, 08-MAR-1993 (KRG)
///
///         The variables INTESC, INTFPC, INTLPC, INTQUO were not saved
///         when they should have been. This eventually caused some
///         problems, so it was fixed. They are now saved.
///
/// -    SPICELIB Version 1.0.0, 20-OCT-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.3.0, 05-DEC-2001 (FST)
///
///         Previous versions of this routine required the range
///         of ICHAR to be [0,255].  This is not the case on some
///         environments, so references to ICHAR were replaced
///         with a ZZICHR statement function that returns values
///         in this range for all supported environments.
/// ```
pub fn wrencc(ctx: &mut SpiceContext, unit: i32, n: i32, data: CharArray) -> crate::Result<()> {
    WRENCC(unit, n, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRENCC ( Write characters to text file encoded )
pub fn WRENCC(UNIT: i32, N: i32, DATA: CharArray, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyCharArray::new(DATA, None, 1..);
    let mut CH = [b' '; 1 as usize];
    let mut ENCCHR = [b' '; MAXENC as usize];
    let mut LFTOVR = [b' '; 2 as usize];
    let mut DTALEN: i32 = 0;
    let mut DTALIN: i32 = 0;
    let mut DTAPOS: i32 = 0;
    let mut ENCPOS: i32 = 0;
    let mut HIBITS: i32 = 0;
    let mut INTCH: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut LOBITS: i32 = 0;
    let mut NCHARS: i32 = 0;
    let mut NCHOUT: i32 = 0;
    let mut ROOM: i32 = 0;

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
    // Statement Functions
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //
    // Define the hexadecimal digits
    //

    //
    // Statement Function Definitions
    //
    // This function controls the conversion of characters to integers.
    // Some versions of the g77 implement ICHAR with a signed integer.
    // This function computes the value of ICHAR that this code requires
    // on any version of g77 for x86 Linux.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"WRENCC", ctx)?;
    }

    if save.FIRST {
        //
        // Initialize the integer values for the special characters
        //
        save.FIRST = false;
        save.INTESC = ZZICHR(ESCCHR);
        save.INTQUO = ZZICHR(QUOTE);
        save.INTFPC = ZZICHR(CHRFPC);
        save.INTLPC = ZZICHR(CHRLPC);
    }
    //
    // Get the length of a data ``line'' in the data buffer DATA.
    //
    DTALEN = intrinsics::LEN(&DATA[1]);
    //
    // Make sure that the encoding character string is empty when we
    // start.
    //
    fstr::assign(&mut ENCCHR, b" ");
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
        CHKOUT(b"WRENCC", ctx)?;
        return Ok(());
    }
    //
    // We need to begin scanning through the characters and placing them
    // into a temporary buffer that is an appropriate length for output
    // to the text file (see the parameter MAXENC above).
    //
    // Initialize all of the counters and pointers used to move through
    // the various character data buffers and count the number of
    // characters processed.
    //
    // Initialize the data line and data line position.
    //
    DTALIN = 1;
    DTAPOS = 1;
    //
    // Initialize the encoded character buffer position.
    //
    ENCPOS = 1;
    //
    // Set the number of characters encoded to zero, and set the number
    // of characters output to zero. The number of output characters may
    // be larger than the number of characters because characters that
    // are escaped are more than one character in length.
    //
    NCHARS = 0;
    NCHOUT = 0;

    while (NCHARS < N) {
        //
        // At this point, we know the following:
        //
        //    (1) 1 <= ENCPOS <= MAXENC
        //    (2) 1 <= DTAPOS <= DTALEN
        //    (3) 1 <= DTALIN
        //    (4) 0 <= NCHARS <= N
        //    (5) 0 <= NCHOUT
        //
        fstr::assign(&mut CH, fstr::substr(DATA.get(DTALIN), DTAPOS..=DTAPOS));
        INTCH = ZZICHR(&CH);
        //
        // If the character is a special character, then encode it and
        // place it in the encoded character buffer. Otherwise the
        // character is a printing character, so just put it in the
        // encoded character buffer.
        //
        if ((((INTCH < save.INTFPC) || (INTCH > save.INTLPC)) || (INTCH == save.INTESC))
            || (INTCH == save.INTQUO))
        {
            //
            // The character is a nonprinting character, the escape
            // character, or a single quote, and so we need to encode
            // it using the escape character ESCCHR followed by two
            // hexadecimal digits which represent the position of the
            // character in the ASCII character sequence.
            //
            HIBITS = (INTCH / HEXBAS);
            LOBITS = (INTCH - (HIBITS * HEXBAS));
            fstr::assign(fstr::substr_mut(&mut ENCCHR, ENCPOS..=ENCPOS), ESCCHR);
            //
            // We need to see if there is enough room in the encoded
            // character buffer to place all of the hexadecimal digits
            // in the encoding. If not, we need to put what we can in the
            // encoded character buffer and temporarily store the rest,
            // which will be placed in the encoded character buffer after
            // the filled buffer is written to the file.
            //
            ROOM = (MAXENC - ENCPOS);

            if (ROOM >= 2) {
                fstr::assign(
                    fstr::substr_mut(&mut ENCCHR, (ENCPOS + 1)..=(ENCPOS + 1)),
                    save.HEXDIG.get(HIBITS),
                );
                fstr::assign(
                    fstr::substr_mut(&mut ENCCHR, (ENCPOS + 2)..=(ENCPOS + 2)),
                    save.HEXDIG.get(LOBITS),
                );
            } else if (ROOM == 1) {
                fstr::assign(
                    fstr::substr_mut(&mut ENCCHR, (ENCPOS + 1)..=(ENCPOS + 1)),
                    save.HEXDIG.get(HIBITS),
                );
                fstr::assign(
                    fstr::substr_mut(&mut LFTOVR, 1..=1),
                    save.HEXDIG.get(LOBITS),
                );
                fstr::assign(fstr::substr_mut(&mut LFTOVR, 2..=2), b" ");
            } else {
                fstr::assign(
                    fstr::substr_mut(&mut LFTOVR, 1..=1),
                    save.HEXDIG.get(HIBITS),
                );
                fstr::assign(
                    fstr::substr_mut(&mut LFTOVR, 2..=2),
                    save.HEXDIG.get(LOBITS),
                );
            }
            //
            // Increment the character buffer pointers, including the
            // pointer for the encoded character (possibly over
            // incrementing, but that's OK).
            //
            NCHARS = (NCHARS + 1);
            DTAPOS = (DTAPOS + 1);
            ENCPOS = (ENCPOS + 3);
            NCHOUT = (NCHOUT + 3);
        } else {
            //
            // The character is a printing character, and we encode it
            // as itself and increment the character buffer pointers
            // appropriately.
            //
            fstr::assign(fstr::substr_mut(&mut ENCCHR, ENCPOS..=ENCPOS), &CH);
            NCHARS = (NCHARS + 1);
            DTAPOS = (DTAPOS + 1);
            ENCPOS = (ENCPOS + 1);
            NCHOUT = (NCHOUT + 1);
        }
        //
        // If we have filled the encoded character buffer, we need to
        // write it out to the file and prepare it for reuse.
        //
        if (ENCPOS > MAXENC) {
            //
            // Write out the encoded character buffer placing single
            // quotes around it so that it may be read using a Fortran
            // list directed read statement rather than the format
            // specifier FMT = '(A)'.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &ENCCHR), QUOTE))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(b"Error writing to logical unit #, IOSTAT = #.", ctx);
                ERRINT(b"#", UNIT, ctx);
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"WRENCC", ctx)?;
                return Ok(());
            }
            //
            // Get ready to fill up the encoded character buffer again,
            // taking care to place any leftover characters in the buffer
            // first.
            //
            NCHOUT = (NCHOUT - MAXENC);

            if (NCHOUT > 0) {
                fstr::assign(
                    fstr::substr_mut(&mut ENCCHR, 1..=2),
                    fstr::substr(&LFTOVR, 1..=2),
                );
            }

            ENCPOS = (1 + NCHOUT);
            fstr::assign(fstr::substr_mut(&mut ENCCHR, ENCPOS..), b" ");
            fstr::assign(&mut LFTOVR, b" ");
        }
        //
        // If we have reached the end of the current data ``line'' in the
        // data buffer DATA, we need to increment the data line pointer
        // and reset the data position pointer.
        //
        if (DTAPOS > DTALEN) {
            DTALIN = (DTALIN + 1);
            DTAPOS = 1;
        }
    }
    //
    // If the number of output characters remaining is greater than
    // zero, we need to flush the encoded character buffer before
    // exiting, because we have a partially filled encoded character
    // buffer. Otherwise, we're done.
    //
    // This last encoded string that is written will be padded with
    // blanks out to MAXENC character positions, so there is no
    // ``garbage'' written at the end of the data.
    //
    if (NCHOUT > 0) {
        //
        // Write out the encoded character buffer placing single
        // quotes around it so that it may be read using a Fortran
        // list directed read statement rather than the format
        // specifier FMT = '(A)'.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &ENCCHR), QUOTE))?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            SETMSG(b"Error writing to logical unit #, IOSTAT = #.", ctx);
            ERRINT(b"#", UNIT, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"WRENCC", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"WRENCC", ctx)?;
    Ok(())
}
