//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BSIZE: i32 = 128;

struct SaveVars {
    BUFFER: StackArray<f64, 128>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BUFFER = StackArray::<f64, 128>::new(1..=BSIZE);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), BSIZE as usize))
                .chain([]);

            BUFFER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { BUFFER }
    }
}

/// DAF, write data to address
///
/// Write or rewrite the double precision data bounded by two
/// addresses within a DAF.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DAF.
///  BEGIN,
///  END        I   Initial, final address within file.
///  DATA       I   Data to be stored between BEGIN and END.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF.
///
///  BEGIN,
///  END      are the initial and final addresses of a contiguous
///           set of double precision numbers within a DAF.
///           Presumably, these make up all or part of a
///           particular array.
///
///  DATA     are the double precision data to be stored between
///           the specified addresses within the specified file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If BEGIN is zero or negative, the error SPICE(DAFNEGADDR)
///      is signaled.
///
///  2)  If the BEGIN > END, the error SPICE(DAFBEGGTEND)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The principal reason that DAFs are so easy to use is that
///  the data in each DAF are considered to be one long contiguous
///  set of double precision numbers. You can store data anywhere
///  within a DAF without knowing (or caring) about the physical
///  records in which they are stored.
///
///  Of course, if you are merely adding arrays to a DAF,
///  you should not use DAFWDA directly, but should use DAFANA
///  (add new array) and its entry points, since these update
///  the appropriate bookkeeping records automatically.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of DAFWDA
///  to update an imaginary array. The array begins with a directory
///  containing 11 epochs. Each pair of epochs bounds an
///  interval, and each interval is covered by a set of eight
///  osculating elements.
///
///  By accident, the elements were written with the wrong value for
///  the GM of the central body (the last element in each set). Each
///  set must be retrieved, updated,and rewritten.
///
///     CALL DAFUS ( SUM, ND, NI, DC, IC )
///     BEGIN = IC(5)
///
///     DO I = 1, 10
///        OFFSET = BEGIN + 11 + (I - 1) * 8
///
///        CALL DAFRDA ( HANDLE, OFFSET+1, OFFSET+8, ELEMENTS )
///        ELEMENTS(8) = NEW_GM
///
///        CALL DAFWDA ( HANDLE, OFFSET+1, OFFSET+8, ELEMENTS )
///     END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 27-OCT-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Local variable BUFFER is now initialized and saved.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafwda(
    ctx: &mut SpiceContext,
    handle: i32,
    begin: i32,
    end: i32,
    data: &[f64],
) -> crate::Result<()> {
    DAFWDA(handle, begin, end, data, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFWDA ( DAF, write data to address )
pub fn DAFWDA(
    HANDLE: i32,
    BEGIN: i32,
    END: i32,
    DATA: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATA = DummyArray::new(DATA, 1..);
    let mut BEGR: i32 = 0;
    let mut BEGW: i32 = 0;
    let mut ENDR: i32 = 0;
    let mut ENDW: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut N: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DAFWDA", ctx)?;

    //
    // Bad addresses?
    //
    if (BEGIN <= 0) {
        SETMSG(b"Negative beginning address: #", ctx);
        ERRINT(b"#", BEGIN, ctx);
        SIGERR(b"SPICE(DAFNEGADDR)", ctx)?;
        CHKOUT(b"DAFWDA", ctx)?;
        return Ok(());
    } else if (BEGIN > END) {
        SETMSG(
            b"Beginning address (#) greater than ending address (#)",
            ctx,
        );
        ERRINT(b"#", BEGIN, ctx);
        ERRINT(b"#", END, ctx);
        SIGERR(b"SPICE(DAFBEGGTEND)", ctx)?;
        CHKOUT(b"DAFWDA", ctx)?;
        return Ok(());
    }

    //
    // Convert raw addresses to record/word representations.
    //
    DAFARW(BEGIN, &mut BEGR, &mut BEGW, ctx)?;
    DAFARW(END, &mut ENDR, &mut ENDW, ctx)?;

    //
    // The first and last records may have to be read, updated, and
    // rewritten. Any records in between may be written directly.
    //
    NEXT = 1;

    for RECNO in BEGR..=ENDR {
        if ((RECNO == BEGR) || (RECNO == ENDR)) {
            DAFRDR(
                HANDLE,
                RECNO,
                1,
                BSIZE,
                save.BUFFER.as_slice_mut(),
                &mut FOUND,
                ctx,
            )?;

            if !FOUND {
                CLEARD(BSIZE, save.BUFFER.as_slice_mut());
            }
        }

        if (BEGR == ENDR) {
            FIRST = BEGW;
            N = ((ENDW - BEGW) + 1);
        } else if (RECNO == BEGR) {
            FIRST = BEGW;
            N = ((BSIZE - BEGW) + 1);
        } else if (RECNO == ENDR) {
            FIRST = 1;
            N = ENDW;
        } else {
            FIRST = 1;
            N = BSIZE;
        }

        MOVED(DATA.subarray(NEXT), N, save.BUFFER.subarray_mut(FIRST));
        NEXT = (NEXT + N);

        DAFWDR(HANDLE, RECNO, save.BUFFER.as_slice(), ctx)?;
    }

    CHKOUT(b"DAFWDA", ctx)?;
    Ok(())
}
