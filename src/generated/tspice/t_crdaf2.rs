//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSCSIZ: i32 = 5;
const TYPLEN: i32 = 3;

//$Procedure  T_CRDAF2 ( Create Supporting DAF Files )
pub fn T_CRDAF2(
    TYPE: &[u8],
    NAME: &[u8],
    IFNAME: &[u8],
    NSEG: i32,
    IDS: &[i32],
    TBEGS: &[f64],
    TENDS: &[f64],
    SEGIDS: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let IDS = DummyArray::new(IDS, 1..);
    let TBEGS = DummyArray::new(TBEGS, 1..);
    let TENDS = DummyArray::new(TENDS, 1..);
    let SEGIDS = DummyCharArray::new(SEGIDS, None, 1..);
    let mut LOCTYP = [b' '; TYPLEN as usize];
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut HANDLE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    spicelib::CHKIN(b"T_CRDAF2", ctx)?;

    spicelib::LJUST(TYPE, &mut LOCTYP);
    spicelib::UCASE(&LOCTYP.clone(), &mut LOCTYP, ctx);

    if fstr::eq(&LOCTYP, b"SPK") {
        spicelib::SPKOPN(NAME, IFNAME, 0, &mut HANDLE, ctx)?;
    } else if fstr::eq(&LOCTYP, b"CK") {
        spicelib::CKOPN(NAME, IFNAME, 0, &mut HANDLE, ctx)?;
    } else {
        spicelib::PCKOPN(NAME, IFNAME, 0, &mut HANDLE, ctx)?;
    }

    for I in 1..=NSEG {
        T_CRDESC(
            &LOCTYP,
            I,
            IDS[I],
            TBEGS[I],
            TENDS[I],
            DESCR.as_slice_mut(),
            ctx,
        )?;

        spicelib::DAFBNA(HANDLE, DESCR.as_slice(), &SEGIDS[I], ctx)?;
        spicelib::DAFENA(ctx)?;
    }

    spicelib::DAFCLS(HANDLE, ctx)?;

    spicelib::CHKOUT(b"T_CRDAF2", ctx)?;
    Ok(())
}
