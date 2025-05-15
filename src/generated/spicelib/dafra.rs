//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NAMLEN: i32 = 1000;
const SUMLEN: i32 = 128;

/// DAF, Re-order arrays
///
/// Reorder the arrays in a DAF according to a given order vector.
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
///  HANDLE     I   Handle of DAF.
///  IORDER     I   Order vector.
///  N          I   Dimension of IORDER.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF that has been opened for
///           write access. Use DAFOPW, for example, to open
///           an existing file and get its handle.
///
///  IORDER   is the order vector to be used to re-order the
///           arrays stored in the DAF specified by HANDLE.
///
///           An integer order vector is an array of length
///           N whose elements are the integers 1 through N.
///
///           The first element of IORDER is the index of the
///           first array in the re-ordered file, and so on.
///
///  N        is the number of elements in the order vector.
///           This may be less than the number of arrays in
///           the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If IORDER is not an order vector (that is, if it does
///      not contain every integer between 1 and N), the error
///      SPICE(DISORDER) is signaled.
///
///  2)  If N is greater than the number of arrays in the file,
///      the error SPICE(DISARRAY) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFRA does not actually move the elements of the double
///  precision arrays; it works by rearranging the contents
///  of the summary and name records in the file. The result
///  is that the search routines (BFS, FNA, BBS, FPA) will
///  return the arrays in the indicated order.
///
///  After re-ordering, array IORDER(1) of the input file is the
///  first array of the output file, array IORDER(2) of the input
///  file is the second array of the output file, and so on.
///
///  The order vector used by DAFRA is typically created for
///  a related array by one of the ORDER routines, as shown in
///  the example below.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment sorts the arrays in a DAF by name.
///
///     C
///     C     Collect the names of the arrays in the file.
///     C
///           CALL DAFOPW ( FILE, HANDLE )
///
///           N = 0
///           CALL DAFBFS ( HANDLE )
///           CALL DAFFNA ( FOUND  )
///
///           DO WHILE ( FOUND )
///              N = N + 1
///              CALL DAFGN  ( NAMES(I) )
///              CALL DAFFNA ( FOUND    )
///           END DO
///
///     C
///     C     Sort the names.
///     C
///           CALL ORDERC ( NAMES, N, IORDER )
///
///     C
///     C     Re-order the arrays.
///     C
///           CALL DARFA  ( HANDLE, IORDER, N )
///           CALL DAFCLS ( HANDLE            )
///
///  Afterward, a forward search like the one shown below
///
///     CALL DAFBFS ( HANDLE )
///     CALL DAFFNA ( FOUND  )
///
///     DO WHILE ( FOUND )
///        CALL DAFGN ( NAME )
///        WRITE (*,*) NAME
///
///        CALL DAFFNA ( FOUND )
///     END DO
///
///  produces an ordered list of the names in the sorted file.
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
/// -    SPICELIB Version 1.1.0, 27-AUG-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 28-MAR-1991 (IMU)
/// ```
pub fn dafra(ctx: &mut SpiceContext, handle: i32, iorder: &mut [i32], n: i32) -> crate::Result<()> {
    DAFRA(handle, iorder, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRA ( DAF, Re-order arrays )
pub fn DAFRA(HANDLE: i32, IORDER: &mut [i32], N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IORDER = DummyArrayMut::new(IORDER, 1..);
    let mut HOLDN = [b' '; NAMLEN as usize];
    let mut TEMPN = [b' '; NAMLEN as usize];
    let mut HOLDS = StackArray::<f64, 128>::new(1..=SUMLEN);
    let mut TEMPS = StackArray::<f64, 128>::new(1..=SUMLEN);
    let mut HOLD: i32 = 0;
    let mut INDEX: i32 = 0;
    let mut START: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut FOUND: bool = false;

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
        CHKIN(b"DAFRA", ctx)?;
    }

    //
    // If the order vector has fewer than two elements, don't bother.
    //
    if (N < 2) {
        CHKOUT(b"DAFRA", ctx)?;
        return Ok(());
    }

    //
    // If IORDER is not an order vector, complain.
    //
    if !ISORDV(IORDER.as_slice_mut(), N) {
        SETMSG(b"Sorry, IORDER is not an order vector.", ctx);
        SIGERR(b"SPICE(DISORDER)", ctx)?;
        CHKOUT(b"DAFRA", ctx)?;
        return Ok(());
    }

    //
    // If the number of arrays to be moved exceeds the number of
    // arrays in the file, complain.
    //
    TOTAL = 0;

    DAFBFS(HANDLE, ctx)?;
    DAFFNA(&mut FOUND, ctx)?;

    while (FOUND && !FAILED(ctx)) {
        TOTAL = (TOTAL + 1);
        DAFFNA(&mut FOUND, ctx)?;
    }

    if FAILED(ctx) {
        CHKOUT(b"DAFRA", ctx)?;
        return Ok(());
    } else if (TOTAL < N) {
        SETMSG(b"N (#) exceeds number of arrays (#).", ctx);
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", TOTAL, ctx);
        SIGERR(b"SPICE(DISARRAY)", ctx)?;
        CHKOUT(b"DAFRA", ctx)?;
        return Ok(());
    }

    //
    // Not surprisingly, this routine is patterned closely after the
    // (original) REORDx routines in SPICELIB. The only differences
    // are that
    //
    //    1) This routine is not error free---it checks to make
    //       sure that IORDER is in fact an order vector, and that
    //       every element in IORDER refers to an existing array.
    //
    //    2) Instead of moving elements of an array in and out of
    //       a temporary location, it moves summaries and names.
    //       This means that two sets of temporary storage locations
    //       are needed: one to hold the summary and name of the
    //       guy who began the current cycle; and one to hold the guy
    //       being moved from location HOLD to location INDEX.
    //
    START = 1;

    while ((START < N) && !FAILED(ctx)) {
        //
        // Start the cycle. One guy (pair of summary and name record)
        // has to sit out (in HOLDS and HOLDN) until the end of the cycle
        // is reached.
        //
        INDEX = START;
        HOLD = IORDER[INDEX];

        DAFBFS(HANDLE, ctx)?;

        for I in 1..=INDEX {
            DAFFNA(&mut FOUND, ctx)?;
        }

        DAFGS(HOLDS.as_slice_mut(), ctx)?;
        DAFGN(&mut HOLDN, ctx)?;

        //
        // Move guys from HOLD to INDEX; then update HOLD (to point
        // to the next guy to be moved) and INDEX (to point at the
        // space just vacated).
        //
        // Keep going until HOLD points to the first guy moved during
        // the current cycle. This ends the cycle.
        //
        while (HOLD != START) {
            //
            // Get the guy in position HOLD.
            //
            DAFBFS(HANDLE, ctx)?;

            for I in 1..=HOLD {
                DAFFNA(&mut FOUND, ctx)?;
            }

            DAFGS(TEMPS.as_slice_mut(), ctx)?;
            DAFGN(&mut TEMPN, ctx)?;

            //
            // Move him to position INDEX. (Note that DAFWS is used to
            // update the summary instead of DAFRS, because the addresses
            // are actually being changed.)
            //
            DAFBFS(HANDLE, ctx)?;

            for I in 1..=INDEX {
                DAFFNA(&mut FOUND, ctx)?;
            }

            DAFWS(TEMPS.as_slice(), ctx)?;
            DAFRN(&TEMPN, ctx)?;

            //
            // Update HOLD and INDEX.
            //
            INDEX = HOLD;
            HOLD = IORDER[HOLD];
            IORDER[INDEX] = -IORDER[INDEX];
        }

        //
        // The last element in the cycle is restored from TEMP.
        //
        DAFBFS(HANDLE, ctx)?;

        for I in 1..=INDEX {
            DAFFNA(&mut FOUND, ctx)?;
        }

        DAFWS(HOLDS.as_slice(), ctx)?;
        DAFRN(&HOLDN, ctx)?;

        IORDER[HOLD] = -IORDER[HOLD];

        //
        // Begin the next cycle at the next element in the order
        // vector with a positive sign. (That is, the next one
        // that hasn't been moved.)
        //
        while ((IORDER[START] < 0) && (START < N)) {
            START = (START + 1);
        }
    }

    //
    // Restore the original signs of the elements of the order
    // vector, for the next go around.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        INDEX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            IORDER[INDEX] = i32::abs(IORDER[INDEX]);
            INDEX += m3__;
        }
    }

    CHKOUT(b"DAFRA", ctx)?;
    Ok(())
}
