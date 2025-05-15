//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const BODID: i32 = 1;
const CENID: i32 = (BODID + 1);
const SPKFRM: i32 = (CENID + 1);
const SPKTYP: i32 = (SPKFRM + 1);
const START: i32 = (SPKTYP + 1);
const FINISH: i32 = (START + 1);
const CKRATE: i32 = SPKTYP;
const CKTYPE: i32 = SPKFRM;
const ND: i32 = 2;
const NI: i32 = 6;
const SUMSIZ: i32 = (ND + ((NI + 1) / 2));

//$Procedure      ZZCKSPK ( SPK or CK )
pub fn ZZCKSPK(HANDLE: i32, CKSPK: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CHCKTM: f64 = 0.0;
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut LASTDP: f64 = 0.0;
    let mut FRSTTM: f64 = 0.0;
    let mut SUM = StackArray::<f64, 5>::new(1..=SUMSIZ);
    let mut TIMES = StackArray::<f64, 2>::new(1..=2);
    let mut ANGVEL: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut FROM: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut LAST: i32 = 0;
    let mut NCK2: i32 = 0;
    let mut NSPK: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut THISND: i32 = 0;
    let mut THISNI: i32 = 0;
    let mut TO: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut CK2OK: bool = false;
    let mut FOUND: bool = false;
    let mut SPKOK: bool = false;

    //
    // Local parameters
    //
    //
    // The following parameters point to the various slots in the
    // integer portion of the DAF descriptor where the values are
    // located.
    //

    //
    // These parameters give the number of integer and double precision
    // components of the descriptor for SPK and CK files.
    //

    //
    // The size of a summary.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZCKSPK", ctx)?;
    //
    // Make sure the values of ND and NI associated with this file
    // have the correct values.
    //
    DAFHSF(HANDLE, &mut THISND, &mut THISNI, ctx)?;

    if ((THISND != ND) || (THISNI != NI)) {
        fstr::assign(CKSPK, b"?");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    //
    // We've got the correct values for ND and NI, examine the descriptor
    // for the first array.
    //
    DAFBFS(HANDLE, ctx)?;
    DAFFNA(&mut FOUND, ctx)?;

    if FAILED(ctx) {
        fstr::assign(CKSPK, b"?");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    //
    // If we don't find any segments, we don't have a clue about
    // the file type.
    //
    if !FOUND {
        fstr::assign(CKSPK, b"?");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }
    //
    // Unpack the summary record.
    //
    DAFGS(SUM.as_slice_mut(), ctx)?;
    DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());

    //
    // Look at the slot where the angular velocity flag would
    // be located if this is a CK file.
    //
    ANGVEL = IC[CKRATE];
    TYPE = IC[CKTYPE];

    //
    // Test 1. The value of ANGVEL may do the trick
    // right at the start.
    //
    if (ANGVEL == 0) {
        fstr::assign(CKSPK, b"CK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    if (ANGVEL > 1) {
        fstr::assign(CKSPK, b"SPK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    //
    // Test 2. If this is an SPK file, it has a type 01 segment.
    // See if this is something orbiting the solar system
    // barycenter.
    //
    if (IC[CENID] == 0) {
        fstr::assign(CKSPK, b"SPK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    //
    // Test 3. This is the super test.  Compute the size of the
    // segment and fetch the last d.p. from the segment.
    //

    FIRST = IC[START];
    LAST = IC[FINISH];
    SIZE = ((LAST - FIRST) + 1);
    //
    // Check the size of the array to see if it has any chance
    // of being an SPK and if it does get the number of MDA records.
    //
    ZZSIZEOK((SIZE - 1), 72, 100, 0, &mut SPKOK, &mut NSPK, ctx)?;

    if !SPKOK {
        fstr::assign(CKSPK, b"CK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    DAFGDA(HANDLE, LAST, LAST, std::slice::from_mut(&mut LASTDP), ctx)?;
    //
    // See if the last number in the file is the allowed number of
    // MDA records.  If not, this must be a CK segment.
    //
    if (LASTDP != (NSPK as f64)) {
        fstr::assign(CKSPK, b"CK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }
    //
    // If we are still here, the last d.p. in the segment matches the
    // expected number of MDA records.  If the potential CK type is
    // not 2, we must have an SPK file.
    //
    if (TYPE != 2) {
        fstr::assign(CKSPK, b"SPK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }
    //
    // We are getting down to the nitty gritty here. See if the
    // size is compatible with a type 02 C-kernel.
    //
    ZZSIZEOK(SIZE, 10, 100, 1, &mut CK2OK, &mut NCK2, ctx)?;

    if !CK2OK {
        fstr::assign(CKSPK, b"SPK");
        CHKOUT(b"ZZCKSPK", ctx)?;
        return Ok(());
    }

    //
    // So much for being nice. We need to examine the structure of the
    // actual data in the segment.  There are two cases to consider:
    // when there is 1 or fewer type 02 CK directory records and when
    // there is more than 1.  Note that to get to this point there must
    // be at least 1 directory value if this is a CK type 02 segment.
    // (To see this check the sizes when ZZSIZEOK returns TRUE for
    // both type 1 SPK and type 02 CK. The only such sizes in which
    // there the number CK type 02 directory values is one or fewer
    // are SIZE = 1081, 1441, and 1801 which correspond to (NSPK,NCK2) =
    // (15,108), (20,144), (25, 180).  In all of these cases there is
    // exactly 1 ck type 02 directory value.)
    //
    if (NCK2 < 201) {
        //
        // Recall that MDA record contains its stop time as the first
        // entry of the record.  These epochs show up duplicated in the
        // epochs portion of the segment.
        //
        // If this is a type 01 SPK segment, there are no directory
        // records and the first epoch shows up in the slot NSPK before
        // the last slot of the segment.  If it is a type 02 CK segment
        // the last stop tick shows up in this slot.  We need to look
        // at this value to see what's up.
        //
        DAFGDA(
            HANDLE,
            (LAST - NSPK),
            (LAST - NSPK),
            std::slice::from_mut(&mut FRSTTM),
            ctx,
        )?;

        //
        // Now (under the assumption that we have an SPK segment) look
        // up the epoch from the last MDA record--- the NSPK'th
        // record.  This epoch must be greater than the first epoch
        // in the array of epochs.
        FROM = (FIRST + ((NSPK - 1) * 71));
        TO = FROM;
        DAFGDA(HANDLE, FROM, TO, std::slice::from_mut(&mut CHCKTM), ctx)?;

        //
        // If this is a type 02 segment.  The value we just picked out
        // will come from the array of stop ticks.  The array of stop
        // ticks is non-decreasing so:
        //
        if (CHCKTM > FRSTTM) {
            fstr::assign(CKSPK, b"SPK");
        } else {
            fstr::assign(CKSPK, b"CK");
        }
    } else {
        //
        // In this case there are at least 2 directory records if we
        // have a CK.  We read the last potential tick value and the
        // first potential directory value..  Note that the last potential
        // stop tick must be greater than the first potential directory
        // record.
        //
        FROM = (LAST - ((NCK2 - 1) / 100));
        TO = (FROM + 1);

        DAFGDA(HANDLE, FROM, TO, TIMES.as_slice_mut(), ctx)?;
        //
        // If we happen to have a TYPE 01 SPK segment we've just
        // read two consecutive values from the epochs sub-array of the
        // segment. Here's a sketch of why this is so:
        //
        //   The number of directory records for a CK type 02 segment is
        //   (NCK2-1)/100  which is the same as SIZE/1001.
        //
        //   The number of directory records for an SPK type 01 segment is
        //   (NSPK-1)/100  which is the same as SIZE/7201.
        //
        //   The number of stop ticks for type 02 CK is NCK2 ~ SIZE/10
        //
        //   The number of epochs for a type 01 SPK is  NSPK ~ SIZE/72
        //
        // so NSPK directories < NCK2 directories < NCK2 directories + 1
        // < NSPK + NSPK directories < NCK2.  Consequently, the
        // two values just read are either the last stop tick and the
        // first CK directory value or two consecutive epochs.
        // In the first case TIMES(1) > TIMES(2), in the later case
        // we have TIMES(1) < TIMES(2)
        //
        if (TIMES[1] > TIMES[2]) {
            fstr::assign(CKSPK, b"CK");
        } else {
            fstr::assign(CKSPK, b"SPK");
        }
    }

    CHKOUT(b"ZZCKSPK", ctx)?;
    Ok(())
}
