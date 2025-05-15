//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LINLEN: i32 = 80;
const COMMA: &[u8; 1] = &fstr::extend_const::<1>(b",");
const BLANK: &[u8; 1] = &fstr::extend_const::<1>(b" ");
const LPAREN: &[u8; 1] = &fstr::extend_const::<1>(b"(");
const RPAREN: &[u8; 1] = &fstr::extend_const::<1>(b")");

/// Read the next variable from a kernel file
///
/// Read the next variable from a SPICE ASCII kernel file into a
/// double precision symbol table.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [SYMBOLS](crate::required_reading::symbols)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TABSYM,
///  TABPTR,
///  TABVAL    I-O  Symbol table.
///  NAME       O   Name of the variable.
///  EOF        O   End of file indicator.
///  LINLEN     P   Maximum line length.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table. On
///           input, the table may or may not contain any variables.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABSYM,
///  TABPTR,
///  TABVAL   on output, contains the name and values of the next
///           variable in kernel file. Depending on the assignment
///           directive, the values in the file may replace or augment
///           any existing values.
///
///   NAME    is the name of the variable. NAME is blank if no variable
///           is read.
///
///   EOF     is .TRUE. when the end of the kernel file has been
///           reached, and is .FALSE. otherwise. The kernel file is
///           closed automatically when the end of the file is reached.
/// ```
///
/// # Parameters
///
/// ```text
///  LINLEN   is the maximum length of a line in the kernel file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs parsing a date from the kernel file, the
///      error SPICE(DATEEXPECTED) is signaled.
///
///  2)  If an error occurs parsing a numeric value from the kernel
///      file, the error SPICE(NUMBEREXPECTED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  RDKVAR reads from the file most recently opened by RDKNEW.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, RDKNEW and RDKVAR are used to read
///  the contents of two kernel files into a single symbol table.
///  First, the table is cleared.
///
///      CALL SCARDC ( 0, TABSYM )
///      CALL SCARDI ( 0, TABPTR )
///      CALL SCARDD ( 0, TABVAL )
///
///  Next, the files are opened and read individually.
///
///      DO I = 1, 2
///         CALL RDKNEW ( KERNEL(I), EOF )
///
///         DO WHILE ( .NOT. EOF )
///            CALL RDKVAR ( TABSYM, TABPTR, TABVAL, NAME, EOF )
///         END DO
///      END DO
///
///  Let the files KERNEL(1) and KERNEL(2) contain
///
///      ===========================================================
///
///      \begindata
///      DELTA_T_A       =   32.184
///      K               =    1.657D-3
///      ORBIT_ECC       =    1.671D-2
///      MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///
///      ===========================================================
///
///  and
///
///      ===========================================================
///      \begindata
///       K               =    0.0D0
///      ===========================================================
///
///  respectively. Then the contents of the symbol table are
///
///       DELTA_T_A  -->   32.184
///       K          -->    0.0D0
///       MEAN_ANOM  -->    6.239996D0
///                         1.99096871D-7
///       ORBIT_ECC  -->    1.671D-2
///
///  In particular, the value of K read from the second file replaces
///  the value read from the first file.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The input file must be opened and initialized by RDKNEW prior
///      to the first call to RDKVAR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 10-MAR-1992 (WLT)
///
///         Changed the length of the local character variable ERROR so
///         that it would always have a length greater than the lengths of
///         the character string values placed into it.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 10-MAR-1992 (WLT)
///
///         Changed the length of the local character variable ERROR so
///         that it would always have a length greater than the lengths of
///         the character string values placed into it.
///
///         The length of the character variable ERROR was changed from 30
///         to 80.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
/// -    Beta Version 2.0.0, 23-OCT-1989 (HAN)
///
///         Added a test to FAILED in the main DO-loop to prevent
///         infinite looping. If the error mode was set to 'RETURN'
///         and an error occurred, the same line could be processed
///         forever.
///
/// -    Beta Version 1.1.0, 13-JAN-1989 (IMU)
///
///         Variable name may now take up an entire line. The previous
///         maximum length (32 characters) was tied to the known length
///         used by POOL. That length is now parameterized. Rather than
///         have two parameters, which could get out of synch, RDKVAR
///         now assumes that a variable name can be as long as an input
///         line.
/// ```
pub fn rdkvar(
    ctx: &mut SpiceContext,
    tabsym: CharArrayMut,
    tabptr: &mut [i32],
    tabval: &mut [f64],
    name: &mut str,
    eof: &mut bool,
) -> crate::Result<()> {
    RDKVAR(
        tabsym,
        tabptr,
        tabval,
        fstr::StrBytes::new(name).as_mut(),
        eof,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDKVAR ( Read the next variable from a kernel file )
pub fn RDKVAR(
    TABSYM: CharArrayMut,
    TABPTR: &mut [i32],
    TABVAL: &mut [f64],
    NAME: &mut [u8],
    EOF: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut TABSYM = DummyCharArrayMut::new(TABSYM, None, LBCELL..);
    let mut TABPTR = DummyArrayMut::new(TABPTR, LBCELL..);
    let mut TABVAL = DummyArrayMut::new(TABVAL, LBCELL..);
    let mut LINE = [b' '; LINLEN as usize];
    let mut VARNAM = [b' '; LINLEN as usize];
    let mut DIRCTV = [b' '; 3];
    let mut STATUS = [b' '; 6];
    let mut CVALUE = [b' '; 30];
    let mut DVALUE: f64 = 0.0;
    let mut I: i32 = 0;
    let mut ERROR = [b' '; 80];

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
        CHKIN(b"RDKVAR", ctx)?;
    }

    //
    // No variable yet.
    //
    fstr::assign(NAME, b" ");

    //
    // No parsing error has occurred yet.
    //
    fstr::assign(&mut ERROR, b" ");

    //
    // Get the next data line. Unless something is terribly wrong,
    // this will begin a new variable definition. We have to read
    // the whole variable, unless we luck out and get an error, in
    // which case we can quit.
    //
    fstr::assign(&mut STATUS, b"BEGIN");

    while (fstr::ne(&STATUS, b"DONE") && !FAILED(ctx)) {
        RDKDAT(&mut LINE, EOF, ctx)?;

        if *EOF {
            CHKOUT(b"RDKVAR", ctx)?;
            return Ok(());
        }

        //
        // Replace commas with blanks. We make no distinctions between
        // the two.
        //
        REPLCH(&LINE.clone(), COMMA, BLANK, &mut LINE);

        //
        // The first word on the first line should be the name of a
        // variable. The second word should be a directive: = or +=.
        //
        if fstr::eq(&STATUS, b"BEGIN") {
            NEXTWD(&LINE.clone(), &mut VARNAM, &mut LINE);
            NEXTWD(&LINE.clone(), &mut DIRCTV, &mut LINE);

            //
            // If this is replacement (=) and not an addition (+=),
            // delete the values currently associated with the variable.
            // They will be replaced later.
            //
            if fstr::eq(&DIRCTV, b"=") {
                SYDELD(
                    &VARNAM,
                    TABSYM.as_arg_mut(),
                    TABPTR.as_slice_mut(),
                    TABVAL.as_slice_mut(),
                    ctx,
                )?;
            }

            //
            // If this is a vector, the next thing on the line will be a
            // left parenthesis. Otherwise, assume that this is a scalar.
            // If it's a vector, get the first value. If it's a scalar,
            // plant a bogus right parenthesis, to make the following loop
            // terminate after one iteration.
            //
            NEXTWD(&LINE.clone(), &mut CVALUE, &mut LINE);

            if fstr::eq(&CVALUE, LPAREN) {
                NEXTWD(&LINE.clone(), &mut CVALUE, &mut LINE);
            } else {
                fstr::assign(&mut LINE, RPAREN);
            }

        //
        // For subsequent lines, treat everything as a new value.
        //
        } else {
            NEXTWD(&LINE.clone(), &mut CVALUE, &mut LINE);
        }

        //
        // We have a value anyway. Store it in the table.
        //
        // Keep going until the other shoe (the right parenthesis)
        // drops, or until the end of the line is reached.
        //
        // Dates begin with @; anything else is presumed to be a number.
        //
        while (fstr::ne(&CVALUE, RPAREN) && fstr::ne(&CVALUE, BLANK)) {
            if fstr::eq(fstr::substr(&CVALUE, 1..=1), b"@") {
                TPARSE(fstr::substr(&CVALUE, 2..), &mut DVALUE, &mut ERROR, ctx)?;

                if fstr::ne(&ERROR, b" ") {
                    fstr::assign(
                        &mut ERROR,
                        &fstr::concat(b"Encountered : ", fstr::substr(&CVALUE, 2..)),
                    );
                    SETMSG(&ERROR, ctx);
                    SIGERR(b"SPICE(DATEEXPECTED)", ctx)?;
                    CHKOUT(b"RDKVAR", ctx)?;
                    return Ok(());
                }
            } else {
                NPARSD(&CVALUE, &mut DVALUE, &mut ERROR, &mut I, ctx);

                if fstr::ne(&ERROR, b" ") {
                    fstr::assign(&mut ERROR, &fstr::concat(b"Encountered : ", &CVALUE));
                    SETMSG(&ERROR, ctx);
                    SIGERR(b"SPICE(NUMBEREXPECTED)", ctx)?;
                    CHKOUT(b"RDKVAR", ctx)?;
                    return Ok(());
                }
            }

            SYENQD(
                &VARNAM,
                DVALUE,
                TABSYM.as_arg_mut(),
                TABPTR.as_slice_mut(),
                TABVAL.as_slice_mut(),
                ctx,
            )?;

            NEXTWD(&LINE.clone(), &mut CVALUE, &mut LINE);
        }

        if fstr::eq(&CVALUE, RPAREN) {
            fstr::assign(&mut STATUS, b"DONE");
        } else {
            fstr::assign(&mut STATUS, b"INVAR");
        }
    }

    //
    // Return the name of the variable, but only if everything went okay.
    //
    fstr::assign(NAME, &VARNAM);

    CHKOUT(b"RDKVAR", ctx)?;
    Ok(())
}
