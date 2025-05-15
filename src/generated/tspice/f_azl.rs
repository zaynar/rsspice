//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LOOSE: f64 = 0.0000001;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;
const LNSIZE: i32 = 80;
const NCASE: i32 = 28;
const NFLAGS: i32 = 4;
const NVECT: i32 = 7;
const NSCALE: i32 = 5;
const NRAND: i32 = 100;

struct SaveVars {
    TITLE: Vec<u8>,
    AZ: f64,
    AZ2: f64,
    DELTAZ: f64,
    DXDAZ: StackArray<f64, 28>,
    DXDEL: StackArray<f64, 28>,
    DXDR: StackArray<f64, 28>,
    DYDAZ: StackArray<f64, 28>,
    DYDEL: StackArray<f64, 28>,
    DYDR: StackArray<f64, 28>,
    DZDAZ: StackArray<f64, 28>,
    DZDEL: StackArray<f64, 28>,
    DZDR: StackArray<f64, 28>,
    EL2: f64,
    E3: StackArray<f64, 3>,
    EL: f64,
    IDMAT: StackArray2D<f64, 9>,
    INJACO: StackArray2D<f64, 9>,
    JAC2: StackArray2D<f64, 9>,
    JACOBI: StackArray2D<f64, 9>,
    MPROD: StackArray2D<f64, 9>,
    R: f64,
    R2: f64,
    REC: StackArray<f64, 3>,
    REC2: StackArray<f64, 3>,
    RECTAN: StackArray2D<f64, 21>,
    SCALES: StackArray<f64, 5>,
    XAZ: StackArray<f64, 28>,
    XEL: StackArray<f64, 28>,
    XEL2: f64,
    XR: StackArray<f64, 28>,
    XRANGE: f64,
    XREC: StackArray<f64, 3>,
    SEED: i32,
    AZCCW: StackArray<bool, 4>,
    ELPLSZ: StackArray<bool, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut AZ: f64 = 0.0;
        let mut AZ2: f64 = 0.0;
        let mut DELTAZ: f64 = 0.0;
        let mut DXDAZ = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DXDEL = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DXDR = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DYDAZ = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DYDEL = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DYDR = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DZDAZ = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DZDEL = StackArray::<f64, 28>::new(1..=NCASE);
        let mut DZDR = StackArray::<f64, 28>::new(1..=NCASE);
        let mut EL2: f64 = 0.0;
        let mut E3 = StackArray::<f64, 3>::new(1..=3);
        let mut EL: f64 = 0.0;
        let mut IDMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut INJACO = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut JAC2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut JACOBI = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut MPROD = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut R: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut REC = StackArray::<f64, 3>::new(1..=3);
        let mut REC2 = StackArray::<f64, 3>::new(1..=3);
        let mut RECTAN = StackArray2D::<f64, 21>::new(1..=3, 1..=NVECT);
        let mut SCALES = StackArray::<f64, 5>::new(1..=NSCALE);
        let mut XAZ = StackArray::<f64, 28>::new(1..=NCASE);
        let mut XEL = StackArray::<f64, 28>::new(1..=NCASE);
        let mut XEL2: f64 = 0.0;
        let mut XR = StackArray::<f64, 28>::new(1..=NCASE);
        let mut XRANGE: f64 = 0.0;
        let mut XREC = StackArray::<f64, 3>::new(1..=3);
        let mut SEED: i32 = 0;
        let mut AZCCW = StackArray::<bool, 4>::new(1..=NFLAGS);
        let mut ELPLSZ = StackArray::<bool, 4>::new(1..=NFLAGS);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            E3.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, NVECT, 1) {
                for J in intrinsics::range(1, 3, 1) {
                    RECTAN[[J, I]] = clist.next().unwrap().into_f64();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(true),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
            ]
            .into_iter();
            for I in intrinsics::range(1, 4, 1) {
                AZCCW[I] = clist.next().unwrap().into_bool();
                ELPLSZ[I] = clist.next().unwrap().into_bool();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(180.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(270.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(90.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 19, 1) {
                XR[I] = clist.next().unwrap().into_f64();
                XAZ[I] = clist.next().unwrap().into_f64();
                XEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-90.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(90.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(20, NCASE, 1) {
                XR[I] = clist.next().unwrap().into_f64();
                XAZ[I] = clist.next().unwrap().into_f64();
                XEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 19, 1) {
                DXDR[I] = clist.next().unwrap().into_f64();
                DXDAZ[I] = clist.next().unwrap().into_f64();
                DXDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(20, NCASE, 1) {
                DXDR[I] = clist.next().unwrap().into_f64();
                DXDAZ[I] = clist.next().unwrap().into_f64();
                DXDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 19, 1) {
                DYDR[I] = clist.next().unwrap().into_f64();
                DYDAZ[I] = clist.next().unwrap().into_f64();
                DYDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(20, NCASE, 1) {
                DYDR[I] = clist.next().unwrap().into_f64();
                DYDAZ[I] = clist.next().unwrap().into_f64();
                DYDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 19, 1) {
                DZDR[I] = clist.next().unwrap().into_f64();
                DZDAZ[I] = clist.next().unwrap().into_f64();
                DZDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(20, NCASE, 1) {
                DZDR[I] = clist.next().unwrap().into_f64();
                DZDAZ[I] = clist.next().unwrap().into_f64();
                DZDEL[I] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
    Val::D(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001),
    Val::D(0.5),
    Val::D(1.0),
    Val::D(2.0),
    Val::D(1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0),
  ].into_iter();
            SCALES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            AZ,
            AZ2,
            DELTAZ,
            DXDAZ,
            DXDEL,
            DXDR,
            DYDAZ,
            DYDEL,
            DYDR,
            DZDAZ,
            DZDEL,
            DZDR,
            EL2,
            E3,
            EL,
            IDMAT,
            INJACO,
            JAC2,
            JACOBI,
            MPROD,
            R,
            R2,
            REC,
            REC2,
            RECTAN,
            SCALES,
            XAZ,
            XEL,
            XEL2,
            XR,
            XRANGE,
            XREC,
            SEED,
            AZCCW,
            ELPLSZ,
        }
    }
}

//$Procedure F_AZL ( Azimuth/elevation coordinate tests )
pub fn F_AZL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // The equations for the transformation between rectangular and
    // azimuth/elevation coordinates, taking into account the possible
    // ways of measuring azimuth and elevation are as follows:
    //
    //    X = R * DCOS( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    Y = R * DSIN( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    Z = R *                       DSIN( ELDIR * EL )
    //
    // or equivalently, since cosine is an even function,
    //
    //    X = R * DCOS( AZ )          * DCOS( EL )
    //    Y = R * DSIN( AZSNSE * AZ ) * DCOS( EL )
    //    Z = R * DSIN( ELDIR  * EL )
    //
    // The following data block contains test values for
    //
    //    DX/DR  =               DCOS( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    DX/DAZ = -R * AZSNSE * DSIN( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    DX/DEL = -R * ELDIR  * DCOS( AZSNSE * AZ ) * DSIN( ELDIR * EL )
    //

    //
    // The following data block contains test values for
    //
    //    DY/DR  =               DSIN( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    DY/DAZ =  R * AZSNSE * DCOS( AZSNSE * AZ ) * DCOS( ELDIR * EL )
    //    DY/DEL = -R * ELDIR  * DSIN( AZSNSE * AZ ) * DSIN( ELDIR * EL )
    //
    // or equivalently, since cosine is an even function,
    //
    //    DY/DR  =  DSIN( AZSNSE * AZ ) * DCOS( EL )
    //    DY/DAZ =  R * AZSNSE * DCOS( AZ ) * DCOS( EL )
    //    DY/DEL = -R * ELDIR  * DSIN( AZSNSE * AZ ) * DSIN( ELDIR * EL )

    //
    // The following data block contains test values for
    //
    //    DZ/DR  =              DSIN( ELDIR * EL )
    //    DZ/DAZ =              0.000D0
    //    DZ/DEL = R * ELDIR  * DCOS( ELDIR * EL )
    //

    //
    // Scales used for range values:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_AZL", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: RECAZL
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Loop through the canned cases. These cover the nominal
    // use of the routine.
    //
    for I in 1..=NVECT {
        for J in 1..=NFLAGS {
            for K in 1..=NSCALE {
                //
                // --- Case: -------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"Canned case # (RECAZL); AZCCW = #; ELPLSZ = #, SCALE = #.",
                );
                spicelib::REPMI(
                    &save.TITLE.to_vec(),
                    b"#",
                    ((NFLAGS * (I - 1)) + J),
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.AZCCW[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.ELPLSZ[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SCALES[K],
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::VSCL(
                    save.SCALES[K],
                    save.RECTAN.subarray([1, I]),
                    save.REC.as_slice_mut(),
                );

                spicelib::RECAZL(
                    save.REC.as_slice(),
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    &mut save.R,
                    &mut save.AZ,
                    &mut save.EL,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.AZ = (spicelib::DPR(ctx) * save.AZ);
                save.EL = (spicelib::DPR(ctx) * save.EL);

                save.XRANGE = (save.SCALES[K] * save.XR[((NFLAGS * (I - 1)) + J)]);

                if (save.XRANGE == 0.0) {
                    testutil::CHCKSD(b"RANGE", save.R, b"~", save.XRANGE, VTIGHT, OK, ctx)?;
                } else {
                    testutil::CHCKSD(b"RANGE", save.R, b"~/", save.XRANGE, VTIGHT, OK, ctx)?;
                }

                testutil::CHCKSD(
                    b"AZ",
                    save.AZ,
                    b"~",
                    save.XAZ[((NFLAGS * (I - 1)) + J)],
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(
                    b"EL",
                    save.EL,
                    b"~",
                    save.XEL[((NFLAGS * (I - 1)) + J)],
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: -------------------------------------------------------
                //

                //
                // Update the title from the previous case.
                //
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"Canned",
                    b"Perturbed AZ",
                    &mut save.TITLE,
                );

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::VSCL(
                    save.SCALES[K],
                    save.RECTAN.subarray([1, I]),
                    save.REC.as_slice_mut(),
                );

                //
                // Perturb the input vector as long as we don't have a
                // degenerate case.
                //
                if ((save.REC[1] != 0.0) || (save.REC[2] != 0.0)) {
                    //
                    // Rotate the input vector by +30 degrees in the
                    // positive AZ direction.
                    //
                    save.DELTAZ = (spicelib::RPD(ctx) * 30 as f64);
                } else {
                    save.DELTAZ = 0.0;
                }

                if save.AZCCW[J] {
                    spicelib::VROTV(
                        save.REC.as_slice(),
                        save.E3.as_slice(),
                        save.DELTAZ,
                        save.REC2.as_slice_mut(),
                    );
                } else {
                    spicelib::VROTV(
                        save.REC.as_slice(),
                        save.E3.as_slice(),
                        -save.DELTAZ,
                        save.REC2.as_slice_mut(),
                    );
                }

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::RECAZL(
                    save.REC2.as_slice(),
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    &mut save.R,
                    &mut save.AZ,
                    &mut save.EL,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Subtract the delta from the returned azimuth for
                // comparison against the canned expected result.
                //
                save.AZ = (save.AZ - save.DELTAZ);

                //
                // Convert AZ and EL to degrees for comparison with expected
                // values.
                //
                save.AZ = (spicelib::DPR(ctx) * save.AZ);

                save.EL = (spicelib::DPR(ctx) * save.EL);

                //
                // Move AZ into the range [ XAZ - 360, XAZ + 360 ]
                // if necessary. We presume AZ and XAZ differ by a
                // very small amount.
                //
                if (save.AZ < (save.XAZ[((NFLAGS * (I - 1)) + J)] - 90.0)) {
                    save.AZ = (save.AZ + 360.0);
                }

                if (save.AZ > (save.XAZ[((NFLAGS * (I - 1)) + J)] + 90.0)) {
                    save.AZ = (save.AZ - 360.0);
                }

                save.XRANGE = (save.SCALES[K] * save.XR[((NFLAGS * (I - 1)) + J)]);

                if (save.XRANGE == 0.0) {
                    testutil::CHCKSD(b"RANGE", save.R, b"~", save.XRANGE, VTIGHT, OK, ctx)?;
                } else {
                    testutil::CHCKSD(b"RANGE", save.R, b"~/", save.XRANGE, VTIGHT, OK, ctx)?;
                }
                //
                // Note slightly relaxed tolerance for this case.
                //
                testutil::CHCKSD(
                    b"AZ",
                    save.AZ,
                    b"~",
                    save.XAZ[((NFLAGS * (I - 1)) + J)],
                    TIGHT,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(
                    b"EL",
                    save.EL,
                    b"~",
                    save.XEL[((NFLAGS * (I - 1)) + J)],
                    VTIGHT,
                    OK,
                    ctx,
                )?;

                //
                // --- Case: -------------------------------------------------------
                //

                //
                // Update the title from the previous case.
                //
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"Perturbed AZ",
                    b"Perturbed AZ and EL",
                    &mut save.TITLE,
                );

                testutil::TCASE(&save.TITLE, ctx)?;

                spicelib::VSCL(
                    save.SCALES[K],
                    save.RECTAN.subarray([1, I]),
                    save.REC.as_slice_mut(),
                );

                //
                // Perturb AZ of the input vector as long as we don't have
                // a degenerate case.
                //
                if ((save.REC[1] != 0.0) || (save.REC[2] != 0.0)) {
                    //
                    // Rotate the input vector by +30 degrees in the
                    // positive AZ direction.
                    //
                    save.DELTAZ = (spicelib::RPD(ctx) * 30 as f64);
                } else {
                    save.DELTAZ = 0.0;
                }

                if save.AZCCW[J] {
                    spicelib::VROTV(
                        save.REC.as_slice(),
                        save.E3.as_slice(),
                        save.DELTAZ,
                        save.REC2.as_slice_mut(),
                    );
                } else {
                    spicelib::VROTV(
                        save.REC.as_slice(),
                        save.E3.as_slice(),
                        -save.DELTAZ,
                        save.REC2.as_slice_mut(),
                    );
                }

                //
                // Set EL of the input vector if the original EL is
                // 0 degrees but the input is not the zero vector.
                //
                if ((save.RECTAN[[3, I]] == 0.0) && !spicelib::VZERO(save.RECTAN.subarray([1, I])))
                {
                    //
                    // Set the Z component of REC2 to be consistent with
                    // an elevation magnitude of 60 degrees.
                    //
                    if save.ELPLSZ[J] {
                        save.REC2[3] = ((f64::sqrt(3.0) / 2 as f64) * save.SCALES[K]);
                    } else {
                        save.REC2[3] = -((f64::sqrt(3.0) / 2 as f64) * save.SCALES[K]);
                    }
                    //
                    // Scale the X and Y components of REC2 to be consistent
                    // with the elevation.
                    //
                    save.REC2[1] = (save.REC2[1] / 2 as f64);
                    save.REC2[2] = (save.REC2[2] / 2 as f64);
                }

                spicelib::RECAZL(
                    save.REC2.as_slice(),
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    &mut save.R,
                    &mut save.AZ,
                    &mut save.EL,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Subtract the delta from the returned azimuth for
                // comparison against the canned expected result.
                //
                save.AZ = (save.AZ - save.DELTAZ);

                //
                // Convert AZ and EL to degrees for comparison with expected
                // values.
                //
                save.AZ = (spicelib::DPR(ctx) * save.AZ);
                save.EL = (spicelib::DPR(ctx) * save.EL);

                //
                // Move AZ into the range [ XAZ - 360, XAZ + 360 ]
                // if necessary. We presume AZ and XAZ differ by a
                // very small amount.
                //
                if (save.AZ < (save.XAZ[((NFLAGS * (I - 1)) + J)] - 90.0)) {
                    save.AZ = (save.AZ + 360.0);
                }

                if (save.AZ > (save.XAZ[((NFLAGS * (I - 1)) + J)] + 90.0)) {
                    save.AZ = (save.AZ - 360.0);
                }

                save.XRANGE = (save.SCALES[K] * save.XR[((NFLAGS * (I - 1)) + J)]);

                if (save.XRANGE == 0.0) {
                    testutil::CHCKSD(b"RANGE", save.R, b"~", save.XRANGE, VTIGHT, OK, ctx)?;
                } else {
                    testutil::CHCKSD(b"RANGE", save.R, b"~/", save.XRANGE, VTIGHT, OK, ctx)?;
                }
                //
                // Note slightly relaxed tolerance for this case.
                //
                testutil::CHCKSD(
                    b"AZ",
                    save.AZ,
                    b"~",
                    save.XAZ[((NFLAGS * (I - 1)) + J)],
                    TIGHT,
                    OK,
                    ctx,
                )?;

                if ((save.RECTAN[[3, I]] == 0.0) && !spicelib::VZERO(save.RECTAN.subarray([1, I])))
                {
                    save.XEL2 = 60.0;

                    testutil::CHCKSD(b"EL", save.EL, b"~", save.XEL2, VTIGHT, OK, ctx)?;
                } else {
                    testutil::CHCKSD(
                        b"EL",
                        save.EL,
                        b"~",
                        save.XEL[((NFLAGS * (I - 1)) + J)],
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    //
    // *****************************************************************
    //
    // Normal cases: AZLREC
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Loop through the canned cases. These cover the nominal
    // use of the routine.
    //
    for I in 1..=NVECT {
        for J in 1..=NFLAGS {
            for K in 1..=NSCALE {
                //
                // --- Case: -------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"Canned case # (AZLREC); AZCCW = #; ELPLSZ = #, SCALE = #.",
                );
                spicelib::REPMI(
                    &save.TITLE.to_vec(),
                    b"#",
                    ((NFLAGS * (I - 1)) + J),
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.AZCCW[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.ELPLSZ[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SCALES[K],
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                save.R = (save.SCALES[K] * save.XR[((NFLAGS * (I - 1)) + J)]);

                spicelib::AZLREC(
                    save.R,
                    (spicelib::RPD(ctx) * save.XAZ[((NFLAGS * (I - 1)) + J)]),
                    (spicelib::RPD(ctx) * save.XEL[((NFLAGS * (I - 1)) + J)]),
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    save.REC.as_slice_mut(),
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Test the vector relative error if the test point is not
                // at the origin.
                //
                spicelib::VSCL(
                    save.SCALES[K],
                    save.RECTAN.subarray([1, I]),
                    save.XREC.as_slice_mut(),
                );

                if !spicelib::VZERO(save.RECTAN.subarray([1, I])) {
                    testutil::CHCKAD(
                        b"REC",
                        save.REC.as_slice(),
                        b"~~/",
                        save.XREC.as_slice(),
                        3,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // Check component-wise differences.
                    //
                    testutil::CHCKAD(
                        b"REC",
                        save.REC.as_slice(),
                        b"~",
                        save.XREC.as_slice(),
                        3,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Confirm that AZLREC and RECAZL are inverses. Loop through
    // random cases. These rely on RECAZL.
    //
    save.SEED = -100001;

    for I in 1..=NRAND {
        //
        // Pick R so that its logarithm is uniformly distributed.
        //
        save.R = f64::exp(testutil::T_RANDD(f64::ln(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001), f64::ln(1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0), &mut save.SEED, ctx)?);

        save.AZ = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut save.SEED, ctx)?;

        save.EL = testutil::T_RANDD(
            -spicelib::HALFPI(ctx),
            spicelib::HALFPI(ctx),
            &mut save.SEED,
            ctx,
        )?;

        for J in 1..=NFLAGS {
            //
            // --- Case: -------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Random case # (AZLREC and RECAZL); AZCCW = #; ELPLSZ = #. rand = #.",
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPML(
                &save.TITLE.to_vec(),
                b"#",
                save.AZCCW[J],
                b"C",
                &mut save.TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPML(
                &save.TITLE.to_vec(),
                b"#",
                save.ELPLSZ[J],
                b"C",
                &mut save.TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::AZLREC(
                save.R,
                save.AZ,
                save.EL,
                save.AZCCW[J],
                save.ELPLSZ[J],
                save.REC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Use RECAZL, which we presume to be valid, to invert the
            // conversion performed by AZLREC.
            //
            spicelib::RECAZL(
                save.REC.as_slice(),
                save.AZCCW[J],
                save.ELPLSZ[J],
                &mut save.R2,
                &mut save.AZ2,
                &mut save.EL2,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Use slightly loose tolerance for random inputs.
            //
            testutil::CHCKSD(b"R2", save.R2, b"~/", save.R, TIGHT, OK, ctx)?;
            testutil::CHCKSD(b"AZ2", save.AZ2, b"~/", save.AZ, TIGHT, OK, ctx)?;
            testutil::CHCKSD(b"EL2", save.EL2, b"~/", save.EL, TIGHT, OK, ctx)?;
        }
    }

    //
    // *****************************************************************
    //
    // Normal cases: DRDAZL
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Loop through the canned cases. These cover the nominal
    // use of the routine.
    //
    for I in 1..=NVECT {
        for J in 1..=NFLAGS {
            for K in 1..=NSCALE {
                //
                // --- Case: -------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"Canned case # (DRDAZL); AZCCW = #; ELPLSZ = #, SCALE = #.",
                );
                spicelib::REPMI(
                    &save.TITLE.to_vec(),
                    b"#",
                    ((NFLAGS * (I - 1)) + J),
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.AZCCW[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.ELPLSZ[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SCALES[K],
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Scale the input range. This will cause scaling of the
                // of the partial derivatives with respect to AZ and EL
                // of the rectangular coordinates.
                //
                save.R = (save.SCALES[K] * save.XR[((NFLAGS * (I - 1)) + J)]);

                spicelib::DRDAZL(
                    save.R,
                    (spicelib::RPD(ctx) * save.XAZ[((NFLAGS * (I - 1)) + J)]),
                    (spicelib::RPD(ctx) * save.XEL[((NFLAGS * (I - 1)) + J)]),
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    save.JACOBI.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.JAC2[[1, 1]] = save.DXDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[1, 2]] = (save.DXDAZ[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);
                save.JAC2[[1, 3]] = (save.DXDEL[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);

                save.JAC2[[2, 1]] = save.DYDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[2, 2]] = (save.DYDAZ[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);
                save.JAC2[[2, 3]] = (save.DYDEL[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);

                save.JAC2[[3, 1]] = save.DZDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[3, 2]] = (save.DZDAZ[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);
                save.JAC2[[3, 3]] = (save.DZDEL[((NFLAGS * (I - 1)) + J)] * save.SCALES[K]);
                //
                // Scale the expected matrix to account for the scale of
                // the input vector.
                //
                if (save.SCALES[K] == 1.0) {
                    testutil::CHCKAD(
                        b"JACOBI",
                        save.JACOBI.as_slice(),
                        b"~",
                        save.JAC2.as_slice(),
                        9,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                } else {
                    testutil::CHCKAD(
                        b"JACOBI",
                        save.JACOBI.as_slice(),
                        b"~~/",
                        save.JAC2.as_slice(),
                        9,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    //
    // *****************************************************************
    //
    // Normal cases: DAZLDR
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Loop through the canned cases. These cover the nominal
    // use of the routine.
    //
    // For testing DAZLDR, use only the first 4 vectors, as the
    // Jacobian matrix is undefined for points on the Z axis.
    //
    for I in 1..=4 {
        for J in 1..=NFLAGS {
            for K in 1..=NSCALE {
                //
                // --- Case: -------------------------------------------------------
                //
                fstr::assign(
                    &mut save.TITLE,
                    b"Canned case # (DAZLDR); AZCCW = #; ELPLSZ = #, SCALE = #.",
                );

                spicelib::REPMI(
                    &save.TITLE.to_vec(),
                    b"#",
                    ((NFLAGS * (I - 1)) + J),
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.AZCCW[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPML(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.ELPLSZ[J],
                    b"C",
                    &mut save.TITLE,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMD(
                    &save.TITLE.to_vec(),
                    b"#",
                    save.SCALES[K],
                    6,
                    &mut save.TITLE,
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::TCASE(&save.TITLE, ctx)?;

                //
                // Scale the input vector. This will cause inverse
                // scaling of the partial derivatives of AZ and EL
                // with respect to the rectangular coordinates.
                //
                spicelib::VSCL(
                    save.SCALES[K],
                    save.RECTAN.subarray([1, I]),
                    save.REC.as_slice_mut(),
                );

                spicelib::DAZLDR(
                    save.REC[1],
                    save.REC[2],
                    save.REC[3],
                    save.AZCCW[J],
                    save.ELPLSZ[J],
                    save.JACOBI.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.JAC2[[1, 1]] = save.DXDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[1, 2]] = save.DXDAZ[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[1, 3]] = save.DXDEL[((NFLAGS * (I - 1)) + J)];

                save.JAC2[[2, 1]] = save.DYDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[2, 2]] = save.DYDAZ[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[2, 3]] = save.DYDEL[((NFLAGS * (I - 1)) + J)];

                save.JAC2[[3, 1]] = save.DZDR[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[3, 2]] = save.DZDAZ[((NFLAGS * (I - 1)) + J)];
                save.JAC2[[3, 3]] = save.DZDEL[((NFLAGS * (I - 1)) + J)];

                spicelib::INVORT(save.JAC2.as_slice(), save.INJACO.as_slice_mut(), ctx)?;

                for M in 1..=3 {
                    //
                    // Row 2 of INJACO is the gradient of AZ.
                    // Row 3 of INJACO is the gradient of EL.
                    //
                    save.INJACO[[2, M]] = (save.INJACO[[2, M]] / save.SCALES[K]);
                    save.INJACO[[3, M]] = (save.INJACO[[3, M]] / save.SCALES[K]);
                }

                if (save.SCALES[K] == 1.0) {
                    testutil::CHCKAD(
                        b"JACOBI",
                        save.JACOBI.as_slice(),
                        b"~",
                        save.INJACO.as_slice(),
                        9,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                } else {
                    testutil::CHCKAD(
                        b"JACOBI",
                        save.JACOBI.as_slice(),
                        b"~~/",
                        save.INJACO.as_slice(),
                        9,
                        VTIGHT,
                        OK,
                        ctx,
                    )?;
                }
            }
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    // Confirm that DAZLDR and DRDAZL are inverses. Loop through
    // random cases.
    //
    save.SEED = -100001;

    spicelib::IDENT(save.IDMAT.as_slice_mut());

    for I in 1..=NRAND {
        //
        // Pick R so that its logarithm is uniformly distributed.
        // Limit the scale of R to avoid cases where extreme roundoff
        // may occur.
        //
        save.R = f64::exp(testutil::T_RANDD(
            f64::ln(0.000001),
            f64::ln(1000000000.0),
            &mut save.SEED,
            ctx,
        )?);

        save.AZ = testutil::T_RANDD(0.0, spicelib::TWOPI(ctx), &mut save.SEED, ctx)?;

        //
        // Bound the elevation away from the poles, where there is
        // great loss of precision.
        //
        save.EL = testutil::T_RANDD(
            (-spicelib::HALFPI(ctx) + 0.001),
            (spicelib::HALFPI(ctx) - 0.001),
            &mut save.SEED,
            ctx,
        )?;

        for J in 1..=NFLAGS {
            //
            // --- Case: -------------------------------------------------------
            //
            fstr::assign(
                &mut save.TITLE,
                b"Random case # (DAZLDR and DRDAZL); AZCCW = #; ELPLSZ = #.",
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPML(
                &save.TITLE.to_vec(),
                b"#",
                save.AZCCW[J],
                b"C",
                &mut save.TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPML(
                &save.TITLE.to_vec(),
                b"#",
                save.ELPLSZ[J],
                b"C",
                &mut save.TITLE,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::DRDAZL(
                save.R,
                save.AZ,
                save.EL,
                save.AZCCW[J],
                save.ELPLSZ[J],
                save.JACOBI.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::AZLREC(
                save.R,
                save.AZ,
                save.EL,
                save.AZCCW[J],
                save.ELPLSZ[J],
                save.REC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DAZLDR(
                save.REC[1],
                save.REC[2],
                save.REC[3],
                save.AZCCW[J],
                save.ELPLSZ[J],
                save.JAC2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MXM(
                save.JAC2.as_slice(),
                save.JACOBI.as_slice(),
                save.MPROD.as_slice_mut(),
            );

            //
            // Use very loose tolerance---approximately single
            // precision---for the product of Jacobians produced from
            // random Range, AZ, EL coordinates. It appears this is the
            // best we can do, at least with the current Jacobian matrix
            // code.
            //
            testutil::CHCKAD(
                b"MPROD",
                save.MPROD.as_slice(),
                b"~~/",
                save.IDMAT.as_slice(),
                9,
                LOOSE,
                OK,
                ctx,
            )?;
        }
    }

    //
    // *****************************************************************
    //
    // Non-error exceptional cases: AZLREC
    //
    // *****************************************************************
    //

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLREC: negative input range", ctx)?;

    save.AZ = (spicelib::RPD(ctx) * 30 as f64);
    save.EL = (spicelib::RPD(ctx) * 60 as f64);
    save.R = -1.0;

    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::AZLREC(
        -save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC2.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSCLIP(-1.0, save.REC2.as_slice_mut());

    testutil::CHCKAD(
        b"REC",
        save.REC.as_slice(),
        b"~",
        save.REC2.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLREC: AZ out of range", ctx)?;

    save.AZ = (spicelib::RPD(ctx) * (30.0 + 720.0));
    save.EL = (spicelib::RPD(ctx) * 60 as f64);
    save.R = 1.0;

    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.AZ = (spicelib::RPD(ctx) * 30 as f64);
    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC2.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(+720) REC",
        save.REC.as_slice(),
        b"~",
        save.REC2.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    save.AZ = (spicelib::RPD(ctx) * (30.0 - 720.0));

    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"(-720) REC",
        save.REC.as_slice(),
        b"~",
        save.REC2.as_slice(),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"AZLREC: EL out of range", ctx)?;
    //
    // Just make sure no error is signaled. The results are not
    // specified.
    //
    save.AZ = (spicelib::RPD(ctx) * 30 as f64);
    save.EL = ((spicelib::RPD(ctx) * 90 as f64) + 0.00000000000001);
    save.R = 1.0;

    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.EL = -save.EL;

    spicelib::AZLREC(
        save.R,
        save.AZ,
        save.EL,
        false,
        true,
        save.REC.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: DRDAZL
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Error case: DRDAZL negative input range", ctx)?;

    spicelib::DRDAZL(-1.0, 0.0, 0.0, true, true, save.JACOBI.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: DAZLDR
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Test error handling for singularity. Use two cases, i.e.
    // vector #5 (0,0,1) and vector #7 (0,0,0), with different AZCCW
    // and ELPLSZ values, to verify independence with respect to the
    // flags and Z component of the vector.
    //
    testutil::TCASE(b"Error case: Jacobian undefined for point on Z-axis", ctx)?;

    spicelib::DAZLDR(
        save.RECTAN[[1, 5]],
        save.RECTAN[[2, 5]],
        save.RECTAN[[3, 5]],
        true,
        true,
        save.JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTONZAXIS)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Error case: Jacobian undefined for point on origin.", ctx)?;

    spicelib::DAZLDR(
        save.RECTAN[[1, 7]],
        save.RECTAN[[2, 7]],
        save.RECTAN[[3, 7]],
        false,
        true,
        save.JACOBI.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(POINTONZAXIS)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
