//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;
const DIRSIZ: i32 = 100;
const BUFSIZ: i32 = (DIRSIZ + 1);

struct SaveVars {
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;

        FIRST = true;

        Self { FIRST }
    }
}

/// Generic Segments: Fetch ref. value and index
///
/// Find the reference value associated with the value X and its
/// index in a generic segment. The segment is identified by a DAF
/// file handle and segment descriptor.
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
///  HANDLE     I   The handle of a DAF open for reading.
///  DESCR      I   The descriptor for a DAF generic segment.
///  X          I   The key value used to find a reference and index.
///  VALUE      O   The reference value associated with X.
///  INDX       O   The index of VALUE within the reference values.
///  FOUND      O   A flag indicating whether values for X were found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF open for reading
///
///  DESCR    is the descriptor of the generic segment that we are
///           going to search for a reference value to associate with
///           X.
///
///  X        is a value for which the associated reference value
///           and reference index is requested.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUE    is the reference value associated with the input value
///           X.
///
///  INDX     is the index of VALUE within the set of reference
///           values for the generic segment. This value may be used
///           to obtain a particular packet of data from the generic
///           segment.
///
///  FOUND    is a logical flag indicating whether a reference value
///           associated with X was found. If a reference value was
///           found, FOUND will have a value of .TRUE.; otherwise it
///           will have a value of .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the reference directory structure is unrecognized, the
///      error SPICE(UNKNOWNREFDIR) is signaled. The most likely cause
///      of this error is that an upgrade to your version of the SPICE
///      toolkit is needed.
///
///  2)  If a value computed for the index of an implicitly indexed
///      generic segment is too large to be represented as an integer,
///      the error SPICE(INDEXTOOLARGE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE above.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to easily find the index and value
///  of the reference item that should be associated with a
///  value X. Given this information you can then easily retrieve
///  the packet that should be associated with X.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have a generic segment that contains the
///  following items.
///
///      1)  Packets that model the motion of a body as a function
///          of time over some interval of time.
///
///      2)  Reference values that are the epochs corresponding
///          to the beginning of the intervals for the packets.
///
///  To retrieve the correct packet to use to compute the position
///  and velocity of the body at a particular epoch,  ET, you could
///  use the following code. (Note this block of code assumes that
///  you aren't going to run into any exceptional cases such as ET
///  falling outside the range of times for which the packets can
///  provide ephemeris data.)
///
///     Find out the index of the time that should be associated
///     with the ET we've been given
///
///     CALL SGFRVI ( HANDLE, DESCR, ET,  ETFND, INDX, FOUND )
///
///     Fetch the INDX'th ephemeris packet from the segment.
///
///     CALL SGFPKT ( HANDLE, DESCR, INDX, EPHEM )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The segment described by DESCR MUST be a generic segment,
///      otherwise the results of this routine are not predictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///
/// -    SPICELIB Version 1.1.0, 08-MAY-1996 (WLT)
///
///         A bug was found in the EXPCLS index case when the
///         trying to retrieve the last value in a generic segment.
///         This bug was discovered by the HP compiler complaining
///         that an index used was not initialized.
///
///         The offending line was
///
///                  MYVALU = BUFFER(I)
///
///         The corrected line is:
///
///                  MYVALU = BUFFER(BFINDX)
///
/// -    SPICELIB Version 1.0.0, 28-MAR-1994 (KRG) (WLT)
/// ```
pub fn sgfrvi(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    x: f64,
    value: &mut f64,
    indx: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    SGFRVI(handle, descr, x, value, indx, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGFRVI ( Generic Segments: Fetch ref. value and index )
pub fn SGFRVI(
    HANDLE: i32,
    DESCR: &[f64],
    X: f64,
    VALUE: &mut f64,
    INDX: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..);
    let mut BUFFER = StackArray::<f64, 101>::new(1..=BUFSIZ);
    let mut DPIMAX: f64 = 0.0;
    let mut DPTEMP: f64 = 0.0;
    let mut ENDREF: f64 = 0.0;
    let mut MYVALU: f64 = 0.0;
    let mut BEGIN: i32 = 0;
    let mut BFINDX: i32 = 0;
    let mut FULLRD: i32 = 0;
    let mut END: i32 = 0;
    let mut I: i32 = 0;
    let mut MYINDX: i32 = 0;
    let mut NFETCH: i32 = 0;
    let mut MYNPKT: i32 = 0;
    let mut MYNRDR: i32 = 0;
    let mut MYNREF: i32 = 0;
    let mut MYRDRB: i32 = 0;
    let mut MYRDRT: i32 = 0;
    let mut MYREFB: i32 = 0;
    let mut RDRIDX: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut DONE: bool = false;
    let mut ISDIRV: bool = false;
    let mut MYFND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //
    // Include the mnemonic values for the generic segment declarations.
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //
    //
    // Initial Values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGFRVI", ctx)?;
    //
    // Set the value for the maximum index as a double precision number,
    // but only do it the first time into the subroutine.
    //
    if save.FIRST {
        save.FIRST = false;
        DPIMAX = (INTMAX() as f64);
    }

    //
    // Collect the necessary meta data values common to all cases.
    //
    SGMETA(HANDLE, DESCR.as_slice(), NPKT, &mut MYNPKT, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), NREF, &mut MYNREF, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), RDRTYP, &mut MYRDRT, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), REFBAS, &mut MYREFB, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGFRVI", ctx)?;
        return Ok(());
    }
    //
    // Check to be sure that we know how to deal with the type of index
    // in the segment. The index type should be between the minimum
    // allowed index type, MNIDXT, and the maximum allowed index type,
    // MXIDXT, as specified in the file 'sgparam.inc'.
    //
    if ((MYRDRT < MNIDXT) || (MYRDRT > MXIDXT)) {
        SETMSG(b"The generic DAF segment you attempted to read has an unsupported reference directory structure. The integer code given for this structure is #, and allowed codes are within the range # to #. The likely cause of this anomaly is your version of SPICELIB needs updating. Contact your system administrator or NAIF for a toolkit update.", ctx);
        ERRINT(b"#", MYRDRT, ctx);
        ERRINT(b"#", MNIDXT, ctx);
        ERRINT(b"#", MXIDXT, ctx);
        SIGERR(b"SPICE(UNKNOWNREFDIR)", ctx)?;
        CHKOUT(b"SGFRVI", ctx)?;
        return Ok(());
    }
    //
    // We don't have an index yet and we initialize things to zero.
    //
    MYFND = false;
    MYINDX = 0;
    MYVALU = 0.0;
    //
    // We pass the idiot checks, so lets proceed. We have a IF block for
    // each allowed reference directory type code.
    //
    //    For implicitly indexed data packets, the interval
    //
    //       [ BUFFER(1), BUFFER(1) + (N - 1) * BUFFER(2) )
    //
    //    is divided into subintervals as follows:
    //
    //       (-infinity, r1), [r_1,r_2) [r_2, r_3), ..., [r_i, r_(i+1)),
    //        ..., [r_N, +infinity),
    //
    //    where N = the number of packets in the segment, MYNPKT, and
    //    r_i = BUFFER(1) + (i-1) * BUFFER(2).
    //
    //    If X is in [r_i, r_(i+1)), i = 1, N-1, then we found a value
    //    and the index returned will be i with the reference value
    //    returned will be r_i.
    //
    //    If X is in [r_N, +infinity), then we found a value and the
    //    index returned will be N and the reference value returned will
    //    be r_N.
    //
    //    If X is in (-infinity, r1), we have two possibilities:
    //
    //       1) If the index type is implicit closest, we found a value,
    //          the index returned will be 1 and the reference value
    //          returned will be r_1.
    //
    //       2) If the index type is implicit less than or equal, we do
    //          not find a value.
    //
    //    For explicitly indexed packets we simply search the reference
    //    directory for an appropriate reference value.
    //
    if ((MYRDRT != IMPLE) && (MYRDRT != IMPCLS)) {
        //
        // In addition to the meta data items we already have, we also
        // need these.
        //
        SGMETA(HANDLE, DESCR.as_slice(), NRDR, &mut MYNRDR, ctx)?;
        SGMETA(HANDLE, DESCR.as_slice(), RDRBAS, &mut MYRDRB, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFRVI", ctx)?;
            return Ok(());
        }
        //
        // We need to scan the reference directory (if there is one) to
        // determine the appropriate block of reference values to read
        // from the generic segment. Then we compute the number of
        // reference values to fetch and examine. Finally, based on the
        // index type we figure out whether we have found a reference
        // value or not. It will take a little while to get there, so
        // let's get going.
        //
        // We have not started yet, so we're not done and we cannot have a
        // reference directory value yet.
        //
        DONE = false;
        ISDIRV = false;
        //
        // We have not read any full buffers of reference directory values
        // yet, all of the reference directory values remain to be read,
        // and we have no index for a reference directory value.
        //
        FULLRD = 0;
        REMAIN = MYNRDR;
        RDRIDX = 0;
        //
        // Search the reference directory values to select the appropriate
        // block of reference values to read.
        //
        while (!DONE && (REMAIN > 0)) {
            //
            // Read a buffer of reference directory items.
            //
            NFETCH = intrinsics::MIN0(&[DIRSIZ, REMAIN]);
            BEGIN = ((MYRDRB + (FULLRD * DIRSIZ)) + 1);
            END = ((BEGIN + NFETCH) - 1);

            DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGFRVI", ctx)?;
                return Ok(());
            }
            //
            // See if X is in the current buffer.
            //
            RDRIDX = LSTLED(X, NFETCH, BUFFER.as_slice());

            if (RDRIDX == 0) {
                //
                // If not, then X < BUFFER(1) and we're done. This indicates
                // that the desired reference value is before, or in, the
                // previous block of reference values.
                //
                DONE = true;
            } else if (RDRIDX == NFETCH) {
                //
                // If we get the last value of the buffer, then either we
                // are done, X = BUFFER(NFETCH), or X > BUFFER(NFETCH).
                //
                if (X == BUFFER[NFETCH]) {
                    //
                    // If X = BUFFER(NFETCH) we are done, we have a directory
                    // value, and it might be a value we want to return.
                    //
                    DONE = true;
                    ISDIRV = true;
                } else {
                    //
                    // Otherwise, we might have more stuff to read, so update
                    // the remainder and the current number of full buffer
                    // reads and try the loop again.
                    //
                    REMAIN = (REMAIN - NFETCH);

                    if (REMAIN > 0) {
                        //
                        // We don't want to increment FULLRD for a partial
                        // buffer read. The arithmetic for the index
                        // calculations below will use RDRIDX to deal with
                        // this.
                        //
                        FULLRD = (FULLRD + 1);
                    }
                }
            } else {
                //
                // BUFFER(1) <= X < BUFFER(NFETCH), i.e., we have something
                // in the buffer. Check to see if X = BUFFER(RDRIDX). If so,
                // we are done, we have a directory value, and it might be a
                // value we want to return. Otherwise, we are just done.
                //
                DONE = true;

                if (X == BUFFER[RDRIDX]) {
                    ISDIRV = true;
                }
            }
        }

        RDRIDX = ((FULLRD * DIRSIZ) + RDRIDX);
        //
        // There are three cases that we need to consider when X is not a
        // reference directory value:
        //
        //    Case 1: 0 < RDRIDX < MYNRDR (most common first)
        //    Case 2: RDRIDX = 0
        //    Case 3: RDRIDX = MYNRDR
        //
        if !ISDIRV {
            if ((RDRIDX > 0) && (RDRIDX < MYNRDR)) {
                //
                // If we were able to bracket X before reaching the end of
                // the reference directory, then we KNOW that we have a
                // candidate for a reference value in the reference data.
                // All we need to do is read the reference data and find it
                // in the buffer. We also read the reference directory
                // values that bracket the desired reference value into
                // BUFFER, so that they are there if we need them.
                //
                NFETCH = intrinsics::MIN0(&[BUFSIZ, ((MYNREF - (RDRIDX * DIRSIZ)) + 1)]);

                BEGIN = (MYREFB + (DIRSIZ * RDRIDX));
                END = ((BEGIN + NFETCH) - 1);

                DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }

                BFINDX = LSTLED(X, NFETCH, BUFFER.as_slice());
                MYINDX = (((DIRSIZ * RDRIDX) + BFINDX) - 1);
            } else if (RDRIDX == 0) {
                //
                // The reference value may be one of the reference values
                // less than the first reference directory item. So we
                // compute the beginning and ending addresses for the data,
                // read it in, and try to find a reference value.
                //
                NFETCH = intrinsics::MIN0(&[BUFSIZ, MYNREF]);

                BEGIN = (MYREFB + 1);
                END = ((BEGIN + NFETCH) - 1);

                DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }

                BFINDX = LSTLED(X, NFETCH, BUFFER.as_slice());
                MYINDX = BFINDX;
            } else if (RDRIDX == MYNRDR) {
                //
                // If we were not able to bracket X before reaching the end
                // of the reference directory, then we might have a
                // candidate for a reference value in the reference data
                // after the last reference directory value. All we need to
                // do is read the reference data and look.
                //
                // NOTE: NFETCH can never be zero or negative, so we can
                // glibly use it. The reason for this is the NFETCH can only
                // be zero if the desired reference value is a reference
                // directory value, and we already know that the reference
                // value we want is not a reference directory value, because
                // we are here. For similar reasons, NFETCH can never be
                // negative.
                //
                BEGIN = (MYREFB + (DIRSIZ * RDRIDX));
                END = (MYREFB + MYNREF);
                NFETCH = ((END - BEGIN) + 1);

                DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }

                BFINDX = LSTLED(X, NFETCH, BUFFER.as_slice());
                MYINDX = (((DIRSIZ * RDRIDX) + BFINDX) - 1);
            }
        } else {
            //
            // We have a reference directory value, whose index is easy to
            // compute.
            //
            MYINDX = (DIRSIZ * RDRIDX);
        }
        //
        // Now, if we have a candidate for a reference value, lets make
        // sure, based on the type of index we have.
        //
        if (MYRDRT == EXPLT) {
            //
            // We have a reference value only if X > some reference
            // value.
            //
            if !ISDIRV {
                //
                // If the value is not a reference directory value, then
                // we have two cases:
                //
                //    Case 1: 0 < MYINDX <= MYNREF
                //    Case 2: MYINDX = 0
                //
                if ((MYINDX > 0) && (MYINDX <= MYNREF)) {
                    //
                    // We found a reference value. The reference value we
                    // want is either the value indicated by MYINDX or
                    // the reference value immediately preceding MYINDX,
                    // if there is such a value. To deal with this we
                    // split the test up into two cases.
                    //
                    if (MYINDX > 1) {
                        //
                        // If X > BUFFER(BFINDX) then we are done, so set the
                        // value. If not, then we want the reference value
                        // that is immediately before the current one.
                        //
                        if (X > BUFFER[BFINDX]) {
                            MYFND = true;
                            MYVALU = BUFFER[BFINDX];
                        } else {
                            MYFND = true;
                            MYVALU = BUFFER[(BFINDX - 1)];
                            MYINDX = (MYINDX - 1);
                        }
                    } else {
                        //
                        // Remember, MYINDX is 1 here. If we are greater
                        // than the first reference value in the segment,
                        // we are done. Otherwise there is no reference
                        // value to be associated with X.
                        //
                        if (X > BUFFER[MYINDX]) {
                            MYFND = true;
                            MYVALU = BUFFER[MYINDX];
                        } else {
                            //
                            // We did not find a reference value. X was
                            // equal to the first reference value of the
                            // generic segment.
                            //
                            MYFND = false;
                        }
                    }
                } else if (MYINDX == 0) {
                    //
                    // We did not find a reference value. X was < the
                    // first reference value for the generic segment.
                    //
                    MYFND = false;
                }
            } else {
                //
                // We have a reference directory value, and we are done.
                // Either the reference directory value is the one we
                // want or the reference value immediately preceding it
                // is the one we want.
                //
                MYFND = true;

                MYINDX = (MYINDX - 1);

                BEGIN = (MYREFB + MYINDX);
                END = BEGIN;

                DAFGDA(HANDLE, BEGIN, END, std::slice::from_mut(&mut MYVALU), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }
            }
        } else if (MYRDRT == EXPLE) {
            //
            // We have a reference value only if X >= some reference
            // value. At this point, either we have the value and index
            // we want or X is before the first reference value of the
            // generic segment. We consider two cases, the first when X
            // is not a reference directory value, and the second when
            // it is.
            //
            if !ISDIRV {
                //
                // If X is not a directory value, then MYINDX is either
                // equal to zero, implying that X is before the first
                // reference value in the generic segment, or MYINDX > 0,
                // implying that we have found a reference value.
                //
                if ((MYINDX > 0) && (MYINDX <= MYNREF)) {
                    MYFND = true;
                    MYVALU = BUFFER[BFINDX];
                } else if (MYINDX == 0) {
                    //
                    // We did not find a reference value. X was < the
                    // first reference value for the generic segment.
                    //
                    MYFND = false;
                }
            } else {
                //
                // We have a reference directory value, and it is the one
                // we want.
                //
                MYFND = true;
                MYVALU = X;
            }
        } else if (MYRDRT == EXPCLS) {
            //
            // We have a reference value for every value of X. If X <
            // the first reference value of the generic segment, the
            // closest value is the first reference value. If X > the
            // last reference value of the generic segment, the closest
            // value is the last reference value. For X between the
            // first and last reference values we simple take the
            // closest reference value to X, resolving a tie by
            // accepting the larger reference value.
            //
            if !ISDIRV {
                //
                // If X is not a directory value, then MYINDX is either
                // equal to zero, implying that X is before the first
                // reference value in the generic segment,
                // 0 < MYINDX < MYNPKT, implying X is between the first
                // and last reference values in the generic segment, or
                // MYINDX = MYNPKT implying that X is greater than or
                // equal to the last reference value.
                //
                if ((MYINDX > 0) && (MYINDX < MYNREF)) {
                    I = BFINDX;
                    //
                    // Find the closest value to X, choosing the larger in
                    // the event of a tie.
                    //
                    if ((BUFFER[(I + 1)] - X) <= (X - BUFFER[I])) {
                        I = (I + 1);
                        MYINDX = (MYINDX + 1);
                    }

                    MYFND = true;
                    MYVALU = BUFFER[I];
                } else if (MYINDX == 0) {
                    //
                    // X is before the first reference value for the
                    // generic segment, so the closest reference value is
                    // the first one.
                    //
                    MYFND = true;
                    MYINDX = 1;
                    MYVALU = BUFFER[1];
                } else if (MYINDX == MYNREF) {
                    //
                    // X is at of after the last reference value for the
                    // generic segment, so the closest reference value is
                    // the last reference value, which will be in BUFFER.
                    //

                    MYFND = true;
                    MYVALU = BUFFER[BFINDX];
                }
            } else {
                //
                // We have a reference directory value, and it is the one
                // we want.
                //
                MYFND = true;
                MYVALU = X;
            }
        }
    } else if (MYRDRT == IMPLE) {
        //
        // Get the begin and end addresses from which to read the
        // reference values and get the reference values.
        //
        BEGIN = (MYREFB + 1);
        END = (MYREFB + 2);

        DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFRVI", ctx)?;
            return Ok(());
        }

        ENDREF = (BUFFER[1] + (((MYNPKT - 1) as f64) * BUFFER[2]));
        //
        // Compute the index if we can.
        //
        if (X < BUFFER[1]) {
            //
            // If X is less than BUFFER(1), we do not have a reference
            // value.
            //
            MYFND = false;
        } else if (X > ENDREF) {
            //
            // If X is greater than ENDREF, then we have a reference
            // value, ENDREF.
            //
            MYFND = true;
            MYINDX = MYNPKT;
            MYVALU = ENDREF;
        } else {
            //
            // r_1 < X < r_N, i.e., we found a value. Compute the index
            // and the reference value.
            //
            if (MYNPKT > 1) {
                MYFND = true;
                //
                // Compute the index.
                //
                DPTEMP = (1.0 + ((X - BUFFER[1]) / BUFFER[2]));
                //
                // Test to see if we can safely convert the index to an
                // integer.
                //
                if (DPTEMP > DPIMAX) {
                    SETMSG(b"The computed index is too large to be represented as an integer. The most likely problem is that an incorrect value was stored for the step size. The value found for the step was: #", ctx);
                    ERRDP(b"#", BUFFER[2], ctx);
                    SIGERR(b"SPICE(INDEXTOOLARGE)", ctx)?;
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }

                MYINDX = (DPTEMP as i32);
                MYINDX = intrinsics::MIN0(&[MYINDX, MYNPKT]);
            } else {
                //
                // There is only one packet.
                //
                MYINDX = 1;
            }
            //
            // Compute the reference value.
            //
            MYVALU = (BUFFER[1] + (((MYINDX - 1) as f64) * BUFFER[2]));
        }
    } else if (MYRDRT == IMPCLS) {
        //
        // Get the begin and end addresses from which to read the
        // reference values and get the reference values.
        //
        BEGIN = (MYREFB + 1);
        END = (MYREFB + 2);

        DAFGDA(HANDLE, BEGIN, END, BUFFER.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFRVI", ctx)?;
            return Ok(());
        }

        ENDREF = (BUFFER[1] + (((MYNPKT - 1) as f64) * BUFFER[2]));
        //
        // Compute the index if we can.
        //
        if (X < BUFFER[1]) {
            //
            // If X < BUFFER(1), then we found a value, the index
            // returned will be 1 and the reference value returned will
            // be BUFFER(1).
            //
            MYFND = true;
            MYINDX = 1;
            MYVALU = BUFFER[1];
        } else if (X > ENDREF) {
            //
            // If X > ENDREF, then we found a value, the index returned
            // will be MYNPKT and the reference value returned will be
            // ENDREF.
            //
            MYFND = true;
            MYINDX = MYNPKT;
            MYVALU = ENDREF;
        } else {
            //
            // r_1 < X < r_N, i.e., we found a value. Compute the index
            // and the reference value. If X is closer to r_I, the index
            // returned will be I with a reference value of r_I. If X is
            // closer to r_(I+1), the index returned will be I+1 with a
            // reference value of r_(I+1).
            //
            if (MYNPKT > 1) {
                MYFND = true;
                //
                // Compute the index.
                //
                DPTEMP = (1.5 + ((X - BUFFER[1]) / BUFFER[2]));

                if (DPTEMP > (DPIMAX + 0.5)) {
                    SETMSG(b"The computed index is too large to be represented as an integer. The most likely problem is that an incorrect value was stored for the step size. The value found for the step was: #", ctx);
                    ERRDP(b"#", BUFFER[2], ctx);
                    SIGERR(b"SPICE(INDEXTOOLARGE)", ctx)?;
                    CHKOUT(b"SGFRVI", ctx)?;
                    return Ok(());
                }

                MYINDX = (DPTEMP as i32);
            } else {
                //
                // There is only one packet.
                //
                MYINDX = 1;
            }
            //
            // Compute the reference value.
            //
            MYVALU = (BUFFER[1] + (((MYINDX - 1) as f64) * BUFFER[2]));
        }
    }

    //
    // At this point, we have either found a value or not. If so, then we
    // need to set the index, value, and found flag for output.
    // Otherwise, we simply set the found flag.
    //
    if MYFND {
        *INDX = MYINDX;
        *VALUE = MYVALU;
    }

    *FOUND = MYFND;

    CHKOUT(b"SGFRVI", ctx)?;
    Ok(())
}
