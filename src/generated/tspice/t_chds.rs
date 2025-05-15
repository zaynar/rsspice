//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NDC: i32 = 2;
const MAXIC: i32 = 6;

//$ Disclaimer
//
//     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//
//     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//
//     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//
pub fn T_CHDS(
    NAME: &[u8],
    ARRAY: &[f64],
    COMP: &[u8],
    EXP: &[f64],
    SIZE: i32,
    TOL: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let EXP = DummyArray::new(EXP, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=NDC);
    let mut XDC = StackArray::<f64, 2>::new(1..=NDC);
    let mut NIC: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=MAXIC);
    let mut XIC = StackArray::<i32, 6>::new(1..=MAXIC);

    //
    // Check segment descriptors for BSR test families.
    //
    //    10-NOV-2001 (NJB)
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    NIC = ((SIZE - 2) * 2);

    spicelib::DAFUS(
        ARRAY.as_slice(),
        NDC,
        NIC,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );
    spicelib::DAFUS(
        EXP.as_slice(),
        NDC,
        NIC,
        XDC.as_slice_mut(),
        XIC.as_slice_mut(),
    );
    //
    // Check the d.p. components.
    //
    testutil::CHCKAD(NAME, DC.as_slice(), COMP, XDC.as_slice(), NDC, TOL, OK, ctx)?;

    if !*OK {
        return Ok(());
    }

    //
    // Check the integer components.
    //
    testutil::CHCKAI(NAME, IC.as_slice(), COMP, XIC.as_slice(), NIC, OK, ctx)?;

    Ok(())
}
