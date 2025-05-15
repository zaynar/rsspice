//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const MAXTIK: i32 = 205;
const WDSIZE: i32 = 32;

fn TICKS(ET: f64, ZEROPT: f64) -> f64 {
    ((ET - ZEROPT) * 10000.0)
}

fn SECNDS(TK: f64, ZEROPT: f64) -> f64 {
    (ZEROPT + (TK / 10000.0))
}

//$Procedure ZZTSTCK3 (Create at test CK of type 3 and SCLK file)
pub fn ZZTSTCK3(CK: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut REF = [b' '; WDSIZE as usize];
    let mut SEGID = [b' '; WDSIZE as usize];
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut BEGTIM: f64 = 0.0;
    let mut ENDTIM: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut TK: f64 = 0.0;
    let mut ZEROPT: f64 = 0.0;
    let mut MAXET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut STARTS = StackArray::<f64, 1>::new(1..=1);
    let mut TICK = StackArray::<f64, 205>::new(1..=MAXTIK);
    let mut QUATRN = ActualArray2D::<f64>::new(1..=4, 1..=MAXTIK);
    let mut ANGVEL = ActualArray2D::<f64>::new(1..=3, 1..=MAXTIK);
    let mut INST: i32 = 0;
    let mut J: i32 = 0;
    let mut NINTS: i32 = 0;
    let mut NTICKS: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut AVFLG: bool = false;

    //
    // Inline functions.
    //
    //
    // Local Variables.
    //

    //
    // Definitions of inline functions.
    //

    //
    // The first order of business is to wipe out any existing
    // files with the same name.
    //
    KILFIL(CK, ctx)?;

    //
    // Next create the C-kernel. Recall the relationship between
    // ET and encoded SCLK ticks.  There are 10000 ticks/second.
    // The zero point of the clock is 1 Jan 1980 TDB.
    //
    spicelib::TPARSE(b"1 Jan 1980", &mut ZEROPT, &mut ERROR, ctx)?;

    MAXET = (ZEROPT + 1000000000.0);
    ET = 0.0;
    J = 0;

    while (ET > ZEROPT) {
        J = (J + 1);
        TICK[J] = TICKS(ET, ZEROPT);

        J = (J + 1);
        TICK[J] = (TICK[(J - 1)] - 1.0);

        ET = (ET - 10000000.0);
    }

    J = (J + 1);
    TICK[J] = 0.0;

    ET = 10000000.0;

    while (ET < MAXET) {
        J = (J + 1);
        TICK[J] = TICKS(ET, ZEROPT);

        J = (J + 1);
        TICK[J] = (TICK[(J - 1)] - 1.0);

        ET = (ET + 10000000.0);
    }

    J = (J + 1);
    TICK[J] = TICKS(MAXET, ZEROPT);

    NTICKS = J;

    //
    // Sort the ticks.
    //
    spicelib::SHELLD(NTICKS, TICK.as_slice_mut());

    BEGTIM = TICK[1];
    ENDTIM = TICK[NTICKS];

    INST = -9999;
    fstr::assign(&mut REF, b"GALACTIC");
    AVFLG = true;
    fstr::assign(&mut SEGID, b"Test Segment for object -9999");

    for I in 1..=NTICKS {
        TK = TICK[I];
        ET = SECNDS(TK, ZEROPT);
        TSTATD(ET, ROT.as_slice_mut(), ANGVEL.subarray_mut([1, I]), ctx);
        spicelib::M2Q(ROT.as_slice(), QUATRN.subarray_mut([1, I]), ctx)?;
    }

    NINTS = 1;
    STARTS[1] = 0.0;

    spicelib::SPCOPN(CK, b"Test C-kernel", &mut HANDLE, ctx)?;
    spicelib::CKW03(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLG,
        &SEGID,
        NTICKS,
        TICK.as_slice(),
        QUATRN.as_slice(),
        ANGVEL.as_slice(),
        NINTS,
        STARTS.as_slice(),
        ctx,
    )?;
    //
    // Now create a second segment by simply taking that attitude
    // 10 million seconds later than those for body -9999
    //
    BEGTIM = TICK[1];
    ENDTIM = TICK[NTICKS];

    INST = -10000;
    fstr::assign(&mut REF, b"FK4");
    AVFLG = true;
    fstr::assign(&mut SEGID, b"Object -10000");

    for I in 1..=NTICKS {
        TK = TICK[I];
        ET = SECNDS(TK, ZEROPT);
        TSTATD(ET, ROT.as_slice_mut(), ANGVEL.subarray_mut([1, I]), ctx);
        spicelib::M2Q(ROT.as_slice(), QUATRN.subarray_mut([1, I]), ctx)?;
    }

    NINTS = 1;
    STARTS[1] = 0.0;

    spicelib::CKW03(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLG,
        &SEGID,
        NTICKS,
        TICK.as_slice(),
        QUATRN.as_slice(),
        ANGVEL.as_slice(),
        NINTS,
        STARTS.as_slice(),
        ctx,
    )?;

    //
    // Finally for the third segment take take the same attitudes
    // but 100 million seconds later than those for object -9999
    //
    BEGTIM = TICK[1];
    ENDTIM = TICK[NTICKS];

    INST = -10001;
    fstr::assign(&mut REF, b"J2000");
    AVFLG = true;
    fstr::assign(&mut SEGID, b"Test Segment for object -10001");

    for I in 1..=NTICKS {
        TK = TICK[I];
        ET = SECNDS(TK, ZEROPT);
        TSTATD(ET, ROT.as_slice_mut(), ANGVEL.subarray_mut([1, I]), ctx);
        spicelib::M2Q(ROT.as_slice(), QUATRN.subarray_mut([1, I]), ctx)?;
    }

    NINTS = 1;
    STARTS[1] = 0.0;

    spicelib::CKW03(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLG,
        &SEGID,
        NTICKS,
        TICK.as_slice(),
        QUATRN.as_slice(),
        ANGVEL.as_slice(),
        NINTS,
        STARTS.as_slice(),
        ctx,
    )?;

    //
    // Finis. Safely close the DAF.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    Ok(())
}
