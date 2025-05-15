//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure T_PCKEQ ( Text PCK equation implementation )
pub fn T_PCKEQ(
    BODY: i32,
    ET: f64,
    RA: f64,
    DEC: f64,
    PM: f64,
    ANGSET: &[i32],
    RADSET: &[i32],
    RADII: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }
    //
    // This routine should never be called.
    //
    spicelib::CHKIN(b"T_PCKEQ", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_PCKEQ", ctx)?;
    Ok(())
}

//$Procedure T_PCKAST ( Text PCK Euler angle body set )
pub fn T_PCKAST(ANGSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ANGSET = DummyArrayMut::new(ANGSET, LBCELL..);

    spicelib::CHKIN(b"T_PCKAST", ctx)?;

    spicelib::SCARDI(0, ANGSET.as_slice_mut(), ctx)?;

    //
    // Body -10 is used for testing features
    // not exercised by the normal cases.
    //
    spicelib::INSRTI(-10, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(10, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(199, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(299, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(399, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(499, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(599, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(699, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(799, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(899, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(999, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(301, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(401, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(402, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(501, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(502, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(503, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(504, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(505, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(514, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(515, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(516, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(601, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(602, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(603, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(604, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(605, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(606, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(608, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(609, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(610, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(611, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(612, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(613, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(614, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(615, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(616, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(617, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(618, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(701, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(702, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(703, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(704, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(705, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(706, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(707, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(708, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(709, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(710, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(711, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(712, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(713, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(714, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(715, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(801, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(803, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(804, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(805, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(806, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(807, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(808, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(901, ANGSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(1000005, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000012, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000093, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000001, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000002, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000004, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000021, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000052, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000433, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000511, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2002867, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2025143, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2431010, ANGSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(9511010, ANGSET.as_slice_mut(), ctx)?;

    //
    // 607 is Hyperion.
    //
    //
    // 802 is Nereid.
    //
    spicelib::CHKOUT(b"T_PCKAST", ctx)?;
    Ok(())
}

//$Procedure T_PCKRST ( Text PCK radii body set )
pub fn T_PCKRST(RADSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut RADSET = DummyArrayMut::new(RADSET, LBCELL..);

    spicelib::CHKIN(b"T_PCKRST", ctx)?;

    spicelib::SCARDI(0, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(10, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(199, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(299, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(399, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(499, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(599, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(699, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(799, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(899, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(999, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(301, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(401, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(402, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(501, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(502, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(503, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(504, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(505, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(506, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(507, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(508, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(509, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(510, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(511, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(512, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(513, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(514, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(515, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(516, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(601, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(602, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(603, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(604, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(605, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(606, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(607, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(608, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(609, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(610, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(611, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(612, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(613, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(614, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(615, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(616, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(617, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(618, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(632, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(633, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(634, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(635, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(649, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(653, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(701, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(702, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(703, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(704, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(705, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(706, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(707, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(708, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(709, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(710, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(711, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(712, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(713, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(714, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(715, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(801, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(802, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(803, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(804, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(805, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(806, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(807, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(808, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(901, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(1000005, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000012, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000036, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000041, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000093, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(1000107, RADSET.as_slice_mut(), ctx)?;

    spicelib::INSRTI(2000001, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000004, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000016, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000021, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000052, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000253, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000433, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2000511, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2002867, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2004179, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2025143, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(2431010, RADSET.as_slice_mut(), ctx)?;
    spicelib::INSRTI(9511010, RADSET.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"T_PCKRST", ctx)?;
    Ok(())
}

//$Procedure T_PCKANG ( Text PCK Euler angle equation implementation )
pub fn T_PCKANG(
    BODY: i32,
    ET: f64,
    RA: &mut f64,
    DEC: &mut f64,
    PM: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut A0: f64 = 0.0;
    let mut D0: f64 = 0.0;
    let mut D: f64 = 0.0;
    let mut E1: f64 = 0.0;
    let mut E2: f64 = 0.0;
    let mut E3: f64 = 0.0;
    let mut E4: f64 = 0.0;
    let mut E5: f64 = 0.0;
    let mut E6: f64 = 0.0;
    let mut E7: f64 = 0.0;
    let mut E8: f64 = 0.0;
    let mut E9: f64 = 0.0;
    let mut E10: f64 = 0.0;
    let mut E11: f64 = 0.0;
    let mut E12: f64 = 0.0;
    let mut E13: f64 = 0.0;
    let mut J1: f64 = 0.0;
    let mut J2: f64 = 0.0;
    let mut J3: f64 = 0.0;
    let mut J4: f64 = 0.0;
    let mut J5: f64 = 0.0;
    let mut J6: f64 = 0.0;
    let mut J7: f64 = 0.0;
    let mut J8: f64 = 0.0;
    let mut JA: f64 = 0.0;
    let mut JB: f64 = 0.0;
    let mut JC: f64 = 0.0;
    let mut JD: f64 = 0.0;
    let mut JE: f64 = 0.0;
    let mut M1: f64 = 0.0;
    let mut M2: f64 = 0.0;
    let mut M3: f64 = 0.0;
    let mut M4: f64 = 0.0;
    let mut M5: f64 = 0.0;
    let mut M6: f64 = 0.0;
    let mut M7: f64 = 0.0;
    let mut M8: f64 = 0.0;
    let mut M9: f64 = 0.0;
    let mut M10: f64 = 0.0;
    let mut M11: f64 = 0.0;
    let mut M12: f64 = 0.0;
    let mut M13: f64 = 0.0;
    let mut M14: f64 = 0.0;
    let mut M15: f64 = 0.0;
    let mut M16: f64 = 0.0;
    let mut M17: f64 = 0.0;
    let mut M18: f64 = 0.0;
    let mut M19: f64 = 0.0;
    let mut M20: f64 = 0.0;
    let mut M21: f64 = 0.0;
    let mut M22: f64 = 0.0;
    let mut M23: f64 = 0.0;
    let mut M24: f64 = 0.0;
    let mut M25: f64 = 0.0;
    let mut M26: f64 = 0.0;
    let mut M27: f64 = 0.0;
    let mut M28: f64 = 0.0;
    let mut M29: f64 = 0.0;
    let mut M30: f64 = 0.0;
    let mut M31: f64 = 0.0;
    let mut MC1: f64 = 0.0;
    let mut MC2: f64 = 0.0;
    let mut MC3: f64 = 0.0;
    let mut MC4: f64 = 0.0;
    let mut MC5: f64 = 0.0;
    let mut N: f64 = 0.0;
    let mut N1: f64 = 0.0;
    let mut N2: f64 = 0.0;
    let mut N3: f64 = 0.0;
    let mut N4: f64 = 0.0;
    let mut N5: f64 = 0.0;
    let mut N6: f64 = 0.0;
    let mut N7: f64 = 0.0;
    let mut S1: f64 = 0.0;
    let mut S2: f64 = 0.0;
    let mut S3: f64 = 0.0;
    let mut S4: f64 = 0.0;
    let mut S5: f64 = 0.0;
    let mut S6: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut U1: f64 = 0.0;
    let mut U2: f64 = 0.0;
    let mut U3: f64 = 0.0;
    let mut U4: f64 = 0.0;
    let mut U5: f64 = 0.0;
    let mut U6: f64 = 0.0;
    let mut U7: f64 = 0.0;
    let mut U8: f64 = 0.0;
    let mut U9: f64 = 0.0;
    let mut U10: f64 = 0.0;
    let mut U11: f64 = 0.0;
    let mut U12: f64 = 0.0;
    let mut U13: f64 = 0.0;
    let mut U14: f64 = 0.0;
    let mut U15: f64 = 0.0;
    let mut U16: f64 = 0.0;
    let mut W: f64 = 0.0;

    spicelib::CHKIN(b"T_PCKANG", ctx)?;

    D = (ET / spicelib::SPD());
    T = (D / (365.25 * 100.0));

    if (BODY == 10) {
        A0 = 286.13;
        D0 = 63.87;
        W = (84.176 + (14.1844 * D));
    } else if (BODY == 199) {
        MC1 = ((174.7910857 + (4.092335 * D)) * spicelib::RPD(ctx));
        MC2 = ((349.5821714 + (8.18467 * D)) * spicelib::RPD(ctx));
        MC3 = ((164.3732571 + (12.277005 * D)) * spicelib::RPD(ctx));
        MC4 = ((339.1643429 + (16.36934 * D)) * spicelib::RPD(ctx));
        MC5 = ((153.9554286 + (20.461675 * D)) * spicelib::RPD(ctx));

        A0 = (281.0103 - (0.0328 * T));

        D0 = (61.4155 - (0.0049 * T));

        W = ((((((329.5988 + (6.1385108 * D)) + (0.01067257 * f64::sin(MC1)))
            - (0.00112309 * f64::sin(MC2)))
            - (0.0001104 * f64::sin(MC3)))
            - (0.00002539 * f64::sin(MC4)))
            - (0.00000571 * f64::sin(MC5)));
    } else if (BODY == 299) {
        A0 = 272.76;
        D0 = 67.16;
        W = (160.2 - (1.4813688 * D));
    } else if (BODY == 399) {
        A0 = (0.0 - (0.641 * T));
        D0 = (90.0 - (0.557 * T));
        W = (190.147 + (360.9856235 * D));
    } else if (BODY == 499) {
        //
        // Phase angles for Mars orientation only.
        //
        M16 = ((198.991226 + (19139.4819985 * T)) * spicelib::RPD(ctx));
        M17 = ((226.292679 + (38280.8511281 * T)) * spicelib::RPD(ctx));
        M18 = ((249.663391 + (57420.7251593 * T)) * spicelib::RPD(ctx));
        M19 = ((266.18351 + (76560.636795 * T)) * spicelib::RPD(ctx));
        M20 = ((79.398797 + (0.5042615 * T)) * spicelib::RPD(ctx));

        M21 = ((122.433576 + (19139.9407476 * T)) * spicelib::RPD(ctx));
        M22 = ((43.058401 + (38280.8753272 * T)) * spicelib::RPD(ctx));
        M23 = ((57.663379 + (57420.7517205 * T)) * spicelib::RPD(ctx));
        M24 = ((79.476401 + (76560.6495004 * T)) * spicelib::RPD(ctx));
        M25 = ((166.325722 + (0.5042615 * T)) * spicelib::RPD(ctx));

        M26 = ((129.071773 + (19140.0328244 * T)) * spicelib::RPD(ctx));
        M27 = ((36.352167 + (38281.0473591 * T)) * spicelib::RPD(ctx));
        M28 = ((56.668646 + (57420.929536 * T)) * spicelib::RPD(ctx));
        M29 = ((67.364003 + (76560.2552215 * T)) * spicelib::RPD(ctx));
        M30 = ((104.79268 + (95700.4387578 * T)) * spicelib::RPD(ctx));
        M31 = ((95.391654 + (0.5042615 * T)) * spicelib::RPD(ctx));

        A0 = ((((((317.269202 - (0.10927547 * T)) + (0.000068 * f64::sin(M16)))
            + (0.000238 * f64::sin(M17)))
            + (0.000052 * f64::sin(M18)))
            + (0.000009 * f64::sin(M19)))
            + (0.419057 * f64::sin(M20)));

        D0 = ((((((54.432516 - (0.05827105 * T)) + (0.000051 * f64::cos(M21)))
            + (0.000141 * f64::cos(M22)))
            + (0.000031 * f64::cos(M23)))
            + (0.000005 * f64::cos(M24)))
            + (1.591274 * f64::cos(M25)));

        W = (((((((176.049863 + (350.891982443297 * D)) + (0.000145 * f64::sin(M26)))
            + (0.000157 * f64::sin(M27)))
            + (0.00004 * f64::sin(M28)))
            + (0.000001 * f64::sin(M29)))
            + (0.000001 * f64::sin(M30)))
            + (0.584542 * f64::sin(M31)));
    } else if (BODY == 599) {
        JA = ((99.360714 + (4850.4046 * T)) * spicelib::RPD(ctx));
        JB = ((175.895369 + (1191.9605 * T)) * spicelib::RPD(ctx));
        JC = ((300.323162 + (262.5475 * T)) * spicelib::RPD(ctx));
        JD = ((114.012305 + (6070.2476 * T)) * spicelib::RPD(ctx));
        JE = ((49.511251 + (64.3 * T)) * spicelib::RPD(ctx));

        A0 = ((((((268.056595 - (0.006499 * T)) + (0.000117 * f64::sin(JA)))
            + (0.000938 * f64::sin(JB)))
            + (0.001432 * f64::sin(JC)))
            + (0.00003 * f64::sin(JD)))
            + (0.00215 * f64::sin(JE)));

        D0 = ((((((64.495303 + (0.002413 * T)) + (0.00005 * f64::cos(JA)))
            + (0.000404 * f64::cos(JB)))
            + (0.000617 * f64::cos(JC)))
            - (0.000013 * f64::cos(JD)))
            + (0.000926 * f64::cos(JE)));

        W = (284.95 + (870.536 * D));
    } else if (BODY == 699) {
        A0 = (40.589 - (0.036 * T));
        D0 = (83.537 - (0.004 * T));
        W = (38.9 + (810.7939024 * D));
    } else if (BODY == 799) {
        A0 = 257.311;
        D0 = -15.175;
        W = (203.81 - (501.1600928 * D));
    } else if (BODY == 899) {
        N = ((357.85 + (52.316 * T)) * spicelib::RPD(ctx));

        A0 = (299.36 + (0.7 * f64::sin(N)));

        D0 = (43.46 - (0.51 * f64::cos(N)));

        W = ((285.946 + (541.1397757 * D)) - (0.48 * f64::sin(N)));
    } else if (BODY == 999) {
        A0 = 132.993;
        D0 = -6.163;
        W = (302.695 + (56.3625225 * D));
    } else if (BODY == 301) {
        E1 = ((125.045 - (0.0529921 * D)) * spicelib::RPD(ctx));
        E2 = ((250.089 - (0.1059842 * D)) * spicelib::RPD(ctx));
        E3 = ((260.008 + (13.0120009 * D)) * spicelib::RPD(ctx));
        E4 = ((176.625 + (13.3407154 * D)) * spicelib::RPD(ctx));
        E5 = ((357.529 + (0.9856003 * D)) * spicelib::RPD(ctx));
        E6 = ((311.589 + (26.4057084 * D)) * spicelib::RPD(ctx));
        E7 = ((134.963 + (13.064993 * D)) * spicelib::RPD(ctx));
        E8 = ((276.617 + (0.3287146 * D)) * spicelib::RPD(ctx));
        E9 = ((34.226 + (1.7484877 * D)) * spicelib::RPD(ctx));
        E10 = ((15.134 - (0.1589763 * D)) * spicelib::RPD(ctx));
        E11 = ((119.743 + (0.0036096 * D)) * spicelib::RPD(ctx));
        E12 = ((239.961 + (0.1643573 * D)) * spicelib::RPD(ctx));
        E13 = ((25.053 + (12.9590088 * D)) * spicelib::RPD(ctx));

        A0 = ((((((((269.9949 + (0.0031 * T)) - (3.8787 * f64::sin(E1)))
            - (0.1204 * f64::sin(E2)))
            + (0.07 * f64::sin(E3)))
            - (0.0172 * f64::sin(E4)))
            + (0.0072 * f64::sin(E6)))
            - (0.0052 * f64::sin(E10)))
            + (0.0043 * f64::sin(E13)));

        D0 = (((((((((66.5392 + (0.013 * T)) + (1.5419 * f64::cos(E1)))
            + (0.0239 * f64::cos(E2)))
            - (0.0278 * f64::cos(E3)))
            + (0.0068 * f64::cos(E4)))
            - (0.0029 * f64::cos(E6)))
            + (0.0009 * f64::cos(E7)))
            + (0.0008 * f64::cos(E10)))
            - (0.0009 * f64::cos(E13)));

        W = (((((((((((((((38.3213 + (13.17635815 * D)) - ((0.0000000000014 * D) * D))
            + (3.561 * f64::sin(E1)))
            + (0.1208 * f64::sin(E2)))
            - (0.0642 * f64::sin(E3)))
            + (0.0158 * f64::sin(E4)))
            + (0.0252 * f64::sin(E5)))
            - (0.0066 * f64::sin(E6)))
            - (0.0047 * f64::sin(E7)))
            - (0.0046 * f64::sin(E8)))
            + (0.0028 * f64::sin(E9)))
            + (0.0052 * f64::sin(E10)))
            + (0.004 * f64::sin(E11)))
            + (0.0019 * f64::sin(E12)))
            - (0.0044 * f64::sin(E13)));
    } else if ((BODY / 100) == 4) {
        //
        // For each of M5, M6, and M7, the quadratic term is omitted from
        // the coded equation but is shown in comments. The kernel pool
        // software cannot handle the quadratic term; trying both of
        // these expressions allows the user to observe the effect of
        // ignoring that term.
        //
        M1 = ((190.72646643 + (15917.10818695 * T)) * spicelib::RPD(ctx));
        M2 = ((21.4689247 + (31834.27934054 * T)) * spicelib::RPD(ctx));
        M3 = ((332.86082793 + (19139.89694742 * T)) * spicelib::RPD(ctx));
        M4 = ((394.93256437 + (38280.79631835 * T)) * spicelib::RPD(ctx));
        M5 = ((189.6327156 + (41215158.1842005 * T)) * spicelib::RPD(ctx));
        //
        // M5 quadratic term: 12.71192322 * T**2
        //
        M6 = ((19.26538605 + (82430316.3686428 * T)) * spicelib::RPD(ctx));
        //
        // M6 quadratic term: 25.42412173 * T**2
        //
        M7 = ((208.89882434 + (123645474.5446679 * T)) * spicelib::RPD(ctx));
        //
        // M7 quadratic term: 38.13293168 * T**2
        //

        M8 = ((121.46893664 + (660.22803474 * T)) * spicelib::RPD(ctx));
        M9 = ((231.05028581 + (660.9912354 * T)) * spicelib::RPD(ctx));
        M10 = ((251.37314025 + (1320.50145245 * T)) * spicelib::RPD(ctx));
        M11 = ((217.98635955 + (38279.9612555 * T)) * spicelib::RPD(ctx));
        M12 = ((196.19729402 + (19139.83628608 * T)) * spicelib::RPD(ctx));
        M13 = ((10.80071482 + (10414879.22849759 * T)) * spicelib::RPD(ctx));
        M14 = ((345.99306351 + (4801583.39793913 * T)) * spicelib::RPD(ctx));
        M15 = ((303.28024985 + (10415546.400505 * T)) * spicelib::RPD(ctx));

        //
        // See the formulas for Mars for constants defining
        // additional phase angles numbered 16-31.
        //

        if (BODY == 401) {
            A0 = (((((317.67071657 - (0.10844326 * T)) - (1.78428399 * f64::sin(M1)))
                + (0.02212824 * f64::sin(M2)))
                - (0.01028251 * f64::sin(M3)))
                - (0.00475595 * f64::sin(M4)));

            D0 = (((((52.88627266 - (0.06134706 * T)) - (1.07516537 * f64::cos(M1)))
                + (0.00668626 * f64::cos(M2)))
                - (0.0064874 * f64::cos(M3)))
                + (0.00281576 * f64::cos(M4)));

            W = (((((((((35.1877444 + (1128.84475928 * D)) + ((12.72192797 * T) * T))
                + (1.42421769 * f64::sin(M1)))
                - (0.02273783 * f64::sin(M2)))
                + (0.00410711 * f64::sin(M3)))
                + (0.00631964 * f64::sin(M4)))
                + (1.73203319 * f64::sin(M5)))
                + (0.01635407 * f64::sin(M6)))
                + (0.00021426 * f64::sin(M7)));
        } else if (BODY == 402) {
            A0 = ((((((316.65705808 - (0.10518014 * T)) + (3.09217726 * f64::sin(M8)))
                + (0.22980637 * f64::sin(M9)))
                + (0.06418655 * f64::sin(M10)))
                + (0.02533537 * f64::sin(M11)))
                + (0.00778695 * f64::sin(M12)));

            D0 = ((((((53.50992033 - (0.05979094 * T)) + (1.83936004 * f64::cos(M8)))
                + (0.1432532 * f64::cos(M9)))
                + (0.01911409 * f64::cos(M10)))
                - (0.0148259 * f64::cos(M11)))
                + (0.0019243 * f64::cos(M12)));

            W = (((((((((79.39932954 + (285.16188899 * D)) - (2.73954829 * f64::sin(M8)))
                - (0.39968606 * f64::sin(M9)))
                - (0.06563259 * f64::sin(M10)))
                - (0.0291294 * f64::sin(M11)))
                + (0.0169916 * f64::sin(M12)))
                + (0.03080596 * f64::sin(M13)))
                + (0.01248044 * f64::sin(M14)))
                - (0.00437509 * f64::sin(M15)));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if ((BODY / 100) == 5) {
        J1 = ((73.32 + (91472.9 * T)) * spicelib::RPD(ctx));
        J2 = ((24.62 + (45137.2 * T)) * spicelib::RPD(ctx));
        J3 = ((283.9 + (4850.7 * T)) * spicelib::RPD(ctx));
        J4 = ((355.8 + (1191.3 * T)) * spicelib::RPD(ctx));
        J5 = ((119.9 + (262.1 * T)) * spicelib::RPD(ctx));
        J6 = ((229.8 + (64.3 * T)) * spicelib::RPD(ctx));
        J7 = ((352.25 + (2382.6 * T)) * spicelib::RPD(ctx));
        J8 = ((113.35 + (6070.0 * T)) * spicelib::RPD(ctx));

        if (BODY == 501) {
            A0 = (((268.05 - (0.009 * T)) + (0.094 * f64::sin(J3))) + (0.024 * f64::sin(J4)));

            D0 = (((64.5 + (0.003 * T)) + (0.04 * f64::cos(J3))) + (0.011 * f64::cos(J4)));

            W = (((200.39 + (203.4889538 * D)) - (0.085 * f64::sin(J3))) - (0.022 * f64::sin(J4)));
        } else if (BODY == 502) {
            A0 = (((((268.08 - (0.009 * T)) + (1.086 * f64::sin(J4))) + (0.06 * f64::sin(J5)))
                + (0.015 * f64::sin(J6)))
                + (0.009 * f64::sin(J7)));

            D0 = (((((64.51 + (0.003 * T)) + (0.468 * f64::cos(J4))) + (0.026 * f64::cos(J5)))
                + (0.007 * f64::cos(J6)))
                + (0.002 * f64::cos(J7)));

            W = (((((36.022 + (101.3747235 * D)) - (0.98 * f64::sin(J4)))
                - (0.054 * f64::sin(J5)))
                - (0.014 * f64::sin(J6)))
                - (0.008 * f64::sin(J7)));
        } else if (BODY == 503) {
            A0 = ((((268.2 - (0.009 * T)) - (0.037 * f64::sin(J4))) + (0.431 * f64::sin(J5)))
                + (0.091 * f64::sin(J6)));

            D0 = ((((64.57 + (0.003 * T)) - (0.016 * f64::cos(J4))) + (0.186 * f64::cos(J5)))
                + (0.039 * f64::cos(J6)));

            W = ((((44.064 + (50.3176081 * D)) + (0.033 * f64::sin(J4))) - (0.389 * f64::sin(J5)))
                - (0.082 * f64::sin(J6)));
        } else if (BODY == 504) {
            A0 = ((((268.72 - (0.009 * T)) - (0.068 * f64::sin(J5))) + (0.59 * f64::sin(J6)))
                + (0.01 * f64::sin(J8)));

            D0 = ((((64.83 + (0.003 * T)) - (0.029 * f64::cos(J5))) + (0.254 * f64::cos(J6)))
                - (0.004 * f64::cos(J8)));

            W = ((((259.51 + (21.5710715 * D)) + (0.061 * f64::sin(J5))) - (0.533 * f64::sin(J6)))
                - (0.009 * f64::sin(J8)));
        } else if (BODY == 505) {
            A0 = (((268.05 - (0.009 * T)) - (0.84 * f64::sin(J1)))
                + (0.01 * f64::sin(((2 as f64) * J1))));

            D0 = ((64.49 + (0.003 * T)) - (0.36 * f64::cos(J1)));

            W = (((231.67 + (722.631456 * D)) + (0.76 * f64::sin(J1)))
                - (0.01 * f64::sin(((2 as f64) * J1))));
        } else if (BODY == 514) {
            A0 = (((268.05 - (0.009 * T)) - (2.11 * f64::sin(J2)))
                + (0.04 * f64::sin(((2 as f64) * J2))));

            D0 = (((64.49 + (0.003 * T)) - (0.91 * f64::cos(J2)))
                + (0.01 * f64::cos(((2 as f64) * J2))));

            W = (((8.56 + (533.70041 * D)) + (1.91 * f64::sin(J2)))
                - (0.04 * f64::sin(((2 as f64) * J2))));
        } else if (BODY == 515) {
            A0 = (268.05 - (0.009 * T));
            D0 = (64.49 + (0.003 * T));
            W = (33.29 + (1206.9986602 * D));
        } else if (BODY == 516) {
            A0 = (268.05 - (0.009 * T));
            D0 = (64.49 + (0.003 * T));
            W = (346.09 + (1221.2547301 * D));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if ((BODY / 100) == 6) {
        S1 = ((353.32 + (75706.7 * T)) * spicelib::RPD(ctx));
        S2 = ((28.72 + (75706.7 * T)) * spicelib::RPD(ctx));
        S3 = ((177.4 - (36505.5 * T)) * spicelib::RPD(ctx));
        S4 = ((300.0 - (7225.9 * T)) * spicelib::RPD(ctx));
        S5 = ((316.45 + (506.2 * T)) * spicelib::RPD(ctx));
        S6 = ((345.2 - (1016.3 * T)) * spicelib::RPD(ctx));

        if (BODY == 601) {
            A0 = ((40.66 - (0.036 * T)) + (13.56 * f64::sin(S3)));

            D0 = ((83.52 - (0.004 * T)) - (1.53 * f64::cos(S3)));

            W = (((333.46 + (381.994555 * D)) - (13.48 * f64::sin(S3))) - (44.85 * f64::sin(S5)));
        } else if (BODY == 602) {
            A0 = (40.66 - (0.036 * T));
            D0 = (83.52 - (0.004 * T));
            W = (6.32 + (262.7318996 * D));
        } else if (BODY == 603) {
            A0 = ((40.66 - (0.036 * T)) + (9.66 * f64::sin(S4)));

            D0 = ((83.52 - (0.004 * T)) - (1.09 * f64::cos(S4)));

            W = (((8.95 + (190.6979085 * D)) - (9.6 * f64::sin(S4))) + (2.23 * f64::sin(S5)));
        } else if (BODY == 604) {
            A0 = (40.66 - (0.036 * T));

            D0 = (83.52 - (0.004 * T));

            W = (357.6 + (131.5349316 * D));
        } else if (BODY == 605) {
            A0 = ((40.38 - (0.036 * T)) + (3.1 * f64::sin(S6)));

            D0 = ((83.55 - (0.004 * T)) - (0.35 * f64::cos(S6)));

            W = ((235.16 + (79.6900478 * D)) - (3.08 * f64::sin(S6)));
        } else if (BODY == 606) {
            A0 = 39.4827;

            D0 = 83.4279;

            W = (186.5855 + (22.5769768 * D));
        } else if (BODY == 608) {
            A0 = (318.16 - (3.949 * T));
            D0 = (75.03 - (1.143 * T));
            W = (355.2 + (4.5379572 * D));
        } else if (BODY == 609) {
            A0 = 356.9;
            D0 = 77.8;
            W = (178.58 + (931.639 * D));
        } else if (BODY == 610) {
            A0 = (((40.58 - (0.036 * T)) - (1.623 * f64::sin(S2)))
                + (0.023 * f64::sin(((2 as f64) * S2))));

            D0 = (((83.52 - (0.004 * T)) - (0.183 * f64::cos(S2)))
                + (0.001 * f64::cos(((2 as f64) * S2))));

            W = (((58.83 + (518.2359876 * D)) + (1.613 * f64::sin(S2)))
                - (0.023 * f64::sin(((2 as f64) * S2))));
        } else if (BODY == 611) {
            A0 = (((40.58 - (0.036 * T)) - (3.153 * f64::sin(S1)))
                + (0.086 * f64::sin(((2 as f64) * S1))));

            D0 = (((83.52 - (0.004 * T)) - (0.356 * f64::cos(S1)))
                + (0.005 * f64::cos(((2 as f64) * S1))));

            W = (((293.87 + (518.4907239 * D)) + (3.133 * f64::sin(S1)))
                - (0.086 * f64::sin(((2 as f64) * S1))));
        } else if (BODY == 612) {
            A0 = (40.85 - (0.036 * T));
            D0 = (83.34 - (0.004 * T));
            W = (245.12 + (131.6174056 * D));
        } else if (BODY == 613) {
            A0 = (50.51 - (0.036 * T));
            D0 = (84.06 - (0.004 * T));
            W = (56.88 + (190.6979332 * D));
        } else if (BODY == 614) {
            A0 = (36.41 - (0.036 * T));

            D0 = (85.04 - (0.004 * T));

            W = (153.51 + (190.6742373 * D));
        } else if (BODY == 615) {
            A0 = (40.58 - (0.036 * T));
            D0 = (83.53 - (0.004 * T));
            W = (137.88 + (598.306 * D));
        } else if (BODY == 616) {
            A0 = (40.58 - (0.036 * T));
            D0 = (83.53 - (0.004 * T));
            W = (296.14 + (587.289 * D));
        } else if (BODY == 617) {
            A0 = (40.58 - (0.036 * T));
            D0 = (83.53 - (0.004 * T));
            W = (162.92 + (572.7891 * D));
        } else if (BODY == 618) {
            A0 = (40.6 - (0.036 * T));
            D0 = (83.5 - (0.004 * T));
            W = (48.8 + (626.044 * D));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if ((BODY / 100) == 7) {
        U1 = ((115.75 + (54991.87 * T)) * spicelib::RPD(ctx));
        U2 = ((141.69 + (41887.66 * T)) * spicelib::RPD(ctx));
        U3 = ((135.03 + (29927.35 * T)) * spicelib::RPD(ctx));
        U4 = ((61.77 + (25733.59 * T)) * spicelib::RPD(ctx));
        U5 = ((249.32 + (24471.46 * T)) * spicelib::RPD(ctx));
        U6 = ((43.86 + (22278.41 * T)) * spicelib::RPD(ctx));
        U7 = ((77.66 + (20289.42 * T)) * spicelib::RPD(ctx));
        U8 = ((157.36 + (16652.76 * T)) * spicelib::RPD(ctx));
        U9 = ((101.81 + (12872.63 * T)) * spicelib::RPD(ctx));
        U10 = ((138.64 + (8061.81 * T)) * spicelib::RPD(ctx));
        U11 = ((102.23 - (2024.22 * T)) * spicelib::RPD(ctx));
        U12 = ((316.41 + (2863.96 * T)) * spicelib::RPD(ctx));
        U13 = ((304.01 - (51.94 * T)) * spicelib::RPD(ctx));
        U14 = ((308.71 - (93.17 * T)) * spicelib::RPD(ctx));
        U15 = ((340.82 - (75.32 * T)) * spicelib::RPD(ctx));
        U16 = ((259.14 - (504.81 * T)) * spicelib::RPD(ctx));

        if (BODY == 701) {
            A0 = (257.43 + (0.29 * f64::sin(U13)));

            D0 = (-15.1 + (0.28 * f64::cos(U13)));

            W = (((156.22 - (142.8356681 * D)) + (0.05 * f64::sin(U12))) + (0.08 * f64::sin(U13)));
        } else if (BODY == 702) {
            A0 = (257.43 + (0.21 * f64::sin(U14)));

            D0 = (-15.1 + (0.2 * f64::cos(U14)));

            W = (((108.05 - (86.8688923 * D)) - (0.09 * f64::sin(U12))) + (0.06 * f64::sin(U14)));
        } else if (BODY == 703) {
            A0 = (257.43 + (0.29 * f64::sin(U15)));

            D0 = (-15.1 + (0.28 * f64::cos(U15)));

            W = ((77.74 - (41.3514316 * D)) + (0.08 * f64::sin(U15)));
        } else if (BODY == 704) {
            A0 = (257.43 + (0.16 * f64::sin(U16)));

            D0 = (-15.1 + (0.16 * f64::cos(U16)));

            W = ((6.77 - (26.7394932 * D)) + (0.04 * f64::sin(U16)));
        } else if (BODY == 705) {
            A0 = ((257.43 + (4.41 * f64::sin(U11))) - (0.04 * f64::sin(((2 as f64) * U11))));

            D0 = ((-15.08 + (4.25 * f64::cos(U11))) - (0.02 * f64::cos(((2 as f64) * U11))));

            W = (((((30.7 - (254.6906892 * D)) - (1.27 * f64::sin(U12)))
                + (0.15 * f64::sin(((2 as f64) * U12))))
                + (1.15 * f64::sin(U11)))
                - (0.09 * f64::sin(((2 as f64) * U11))));
        } else if (BODY == 706) {
            A0 = (257.31 - (0.15 * f64::sin(U1)));

            D0 = (-15.18 + (0.14 * f64::cos(U1)));

            W = ((127.69 - (1074.520573 * D)) - (0.04 * f64::sin(U1)));
        } else if (BODY == 707) {
            A0 = (257.31 - (0.09 * f64::sin(U2)));

            D0 = (-15.18 + (0.09 * f64::cos(U2)));

            W = ((130.35 - (956.406815 * D)) - (0.03 * f64::sin(U2)));
        } else if (BODY == 708) {
            A0 = (257.31 - (0.16 * f64::sin(U3)));

            D0 = (-15.18 + (0.16 * f64::cos(U3)));

            W = ((105.46 - (828.391476 * D)) - (0.04 * f64::sin(U3)));
        } else if (BODY == 709) {
            A0 = (257.31 - (0.04 * f64::sin(U4)));

            D0 = (-15.18 + (0.04 * f64::cos(U4)));

            W = ((59.16 - (776.581632 * D)) - (0.01 * f64::sin(U4)));
        } else if (BODY == 710) {
            A0 = (257.31 - (0.17 * f64::sin(U5)));

            D0 = (-15.18 + (0.16 * f64::cos(U5)));

            W = ((95.08 - (760.053169 * D)) - (0.04 * f64::sin(U5)));
        } else if (BODY == 711) {
            A0 = (257.31 - (0.06 * f64::sin(U6)));

            D0 = (-15.18 + (0.06 * f64::cos(U6)));

            W = ((302.56 - (730.125366 * D)) - (0.02 * f64::sin(U6)));
        } else if (BODY == 712) {
            A0 = (257.31 - (0.09 * f64::sin(U7)));

            D0 = (-15.18 + (0.09 * f64::cos(U7)));

            W = ((25.03 - (701.486587 * D)) - (0.02 * f64::sin(U7)));
        } else if (BODY == 713) {
            A0 = (257.31 - (0.29 * f64::sin(U8)));

            D0 = (-15.18 + (0.28 * f64::cos(U8)));

            W = ((314.9 - (644.631126 * D)) - (0.08 * f64::sin(U8)));
        } else if (BODY == 714) {
            A0 = (257.31 - (0.03 * f64::sin(U9)));

            D0 = (-15.18 + (0.03 * f64::cos(U9)));

            W = ((297.46 - (577.362817 * D)) - (0.01 * f64::sin(U9)));
        } else if (BODY == 715) {
            A0 = (257.31 - (0.33 * f64::sin(U10)));

            D0 = (-15.18 + (0.31 * f64::cos(U10)));

            W = ((91.24 - (472.545069 * D)) - (0.09 * f64::sin(U10)));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if ((BODY / 100) == 8) {
        N = ((357.85 + (52.316 * T)) * spicelib::RPD(ctx));
        N1 = ((323.92 + (62606.6 * T)) * spicelib::RPD(ctx));
        N2 = ((220.51 + (55064.2 * T)) * spicelib::RPD(ctx));
        N3 = ((354.27 + (46564.5 * T)) * spicelib::RPD(ctx));
        N4 = ((75.31 + (26109.4 * T)) * spicelib::RPD(ctx));
        N5 = ((35.36 + (14325.4 * T)) * spicelib::RPD(ctx));
        N6 = ((142.61 + (2824.6 * T)) * spicelib::RPD(ctx));
        N7 = ((177.85 + (52.316 * T)) * spicelib::RPD(ctx));

        if (BODY == 801) {
            A0 = (((((((((299.36 - (32.35 * f64::sin(N7)))
                - (6.28 * f64::sin(((2 as f64) * N7))))
                - (2.08 * f64::sin(((3 as f64) * N7))))
                - (0.74 * f64::sin(((4 as f64) * N7))))
                - (0.28 * f64::sin(((5 as f64) * N7))))
                - (0.11 * f64::sin(((6 as f64) * N7))))
                - (0.07 * f64::sin(((7 as f64) * N7))))
                - (0.02 * f64::sin(((8 as f64) * N7))))
                - (0.01 * f64::sin(((9 as f64) * N7))));

            D0 = (((((((41.17 + (22.55 * f64::cos(N7))) + (2.1 * f64::cos(((2 as f64) * N7))))
                + (0.55 * f64::cos(((3 as f64) * N7))))
                + (0.16 * f64::cos(((4 as f64) * N7))))
                + (0.05 * f64::cos(((5 as f64) * N7))))
                + (0.02 * f64::cos(((6 as f64) * N7))))
                + (0.01 * f64::cos(((7 as f64) * N7))));

            W = ((((((((((296.53 - (61.2572637 * D)) + (22.25 * f64::sin(N7)))
                + (6.73 * f64::sin(((2 as f64) * N7))))
                + (2.05 * f64::sin(((3 as f64) * N7))))
                + (0.74 * f64::sin(((4 as f64) * N7))))
                + (0.28 * f64::sin(((5 as f64) * N7))))
                + (0.11 * f64::sin(((6 as f64) * N7))))
                + (0.05 * f64::sin(((7 as f64) * N7))))
                + (0.02 * f64::sin(((8 as f64) * N7))))
                + (0.01 * f64::sin(((9 as f64) * N7))));
        } else if (BODY == 803) {
            A0 = (((299.36 + (0.7 * f64::sin(N))) - (6.49 * f64::sin(N1)))
                + (0.25 * f64::sin(((2 as f64) * N1))));

            D0 = (((43.36 - (0.51 * f64::cos(N))) - (4.75 * f64::cos(N1)))
                + (0.09 * f64::cos(((2 as f64) * N1))));

            W = ((((254.06 + (1222.8441209 * D)) - (0.48 * f64::sin(N))) + (4.4 * f64::sin(N1)))
                - (0.27 * f64::sin(((2 as f64) * N1))));
        } else if (BODY == 804) {
            A0 = ((299.36 + (0.7 * f64::sin(N))) - (0.28 * f64::sin(N2)));

            D0 = ((43.45 - (0.51 * f64::cos(N))) - (0.21 * f64::cos(N2)));

            W = (((102.06 + (1155.7555612 * D)) - (0.48 * f64::sin(N))) + (0.19 * f64::sin(N2)));
        } else if (BODY == 805) {
            A0 = ((299.36 + (0.7 * f64::sin(N))) - (0.09 * f64::sin(N3)));

            D0 = ((43.45 - (0.51 * f64::cos(N))) - (0.07 * f64::cos(N3)));

            W = (((306.51 + (1075.7341562 * D)) - (0.49 * f64::sin(N))) + (0.06 * f64::sin(N3)));
        } else if (BODY == 806) {
            A0 = ((299.36 + (0.7 * f64::sin(N))) - (0.07 * f64::sin(N4)));

            D0 = ((43.43 - (0.51 * f64::cos(N))) - (0.05 * f64::cos(N4)));

            W = (((258.09 + (839.6597686 * D)) - (0.48 * f64::sin(N))) + (0.05 * f64::sin(N4)));
        } else if (BODY == 807) {
            A0 = ((299.36 + (0.7 * f64::sin(N))) - (0.27 * f64::sin(N5)));

            D0 = ((43.41 - (0.51 * f64::cos(N))) - (0.2 * f64::cos(N5)));

            W = (((179.41 + (649.053447 * D)) - (0.48 * f64::sin(N))) + (0.19 * f64::sin(N5)));
        } else if (BODY == 808) {
            A0 = ((299.27 + (0.7 * f64::sin(N))) - (0.05 * f64::sin(N6)));

            D0 = ((42.91 - (0.51 * f64::cos(N))) - (0.04 * f64::cos(N6)));

            W = (((93.38 + (320.7654228 * D)) - (0.48 * f64::sin(N))) + (0.04 * f64::sin(N6)));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if ((BODY / 100) == 9) {
        if (BODY == 901) {
            A0 = 132.993;

            D0 = -6.163;

            W = (122.695 + (56.3625225 * D));
        } else {
            spicelib::SETMSG(b"Body code # was not recognized.", ctx);
            spicelib::ERRINT(b"#", BODY, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
            spicelib::CHKOUT(b"T_PCKANG", ctx)?;
            return Ok(());
        }
    } else if (BODY == 2000001) {
        //
        // Ceres.
        //
        A0 = 291.418;

        D0 = 66.764;

        W = (170.65 + (952.1532 * D));
    } else if (BODY == 2000002) {
        //
        // Pallas.
        //
        A0 = 33.0;

        D0 = -3.0;

        W = (38.0 + (1105.8036 * D));
    } else if (BODY == 2000004) {
        //
        // Vesta.
        //
        A0 = 309.031;

        D0 = 42.235;

        W = (285.39 + (1617.3329428 * D));
    } else if (BODY == 2000021) {
        //
        // Lutetia.
        //
        A0 = 52.0;

        D0 = 12.0;

        W = (94.0 + (1057.7515 * D));
    } else if (BODY == 2000052) {
        //
        // Europa.
        //
        A0 = 257.0;

        D0 = 12.0;

        W = (55.0 + (1534.6472187 * D));
    } else if (BODY == 2431010) {
        //
        // Ida.
        //
        A0 = 168.76;

        D0 = -87.12;

        W = (274.05 + (1864.628007 * D));
    } else if (BODY == 2000433) {
        //
        // Eros.
        //
        A0 = 11.35;

        D0 = 17.22;

        W = (326.07 + (1639.38864745 * D));
    } else if (BODY == 2000511) {
        //
        // Davida.
        //
        A0 = 297.0;

        D0 = 5.0;

        W = (268.1 + (1684.4193549 * D));
    } else if (BODY == 9511010) {
        //
        // Gaspra.
        //
        A0 = 9.47;

        D0 = 26.7;

        W = (83.67 + (1226.911485 * D));
    } else if (BODY == 2002867) {
        //
        // Steins.
        //
        A0 = 91.0;

        D0 = -62.0;

        W = (321.76 + (1428.09917 * D));
    } else if (BODY == 2025143) {
        //
        // Itokawa.
        //
        A0 = 90.53;

        D0 = -66.3;

        W = (0.0 + (712.143 * D));
    } else if (BODY == 1000093) {
        //
        // Tempel 1.
        //
        //
        //    2005-07-04 05:45:38.4 TDB (DI impact):
        //
        //       W = 109.7  W1 = 211.849/d
        //
        //    2011-02-15 04:40:18.6 TDB (Stardust NExT c/a):
        //
        A0 = 255.0;

        D0 = 64.5;

        W = (69.2 + (212.807 * D));
    } else if (BODY == 1000005) {
        //
        // 19/P Borrelly.
        //
        A0 = 218.5;

        D0 = -12.5;

        W = (0.0 + (324.3 * D));
    } else if (BODY == 1000012) {
        //
        // 67P/Churyumov-Gerasimenko
        //
        //    2014-03-23 : 2014-09-03
        //
        A0 = 69.54;

        D0 = 64.11;

        W = (114.69 + (696.543884683 * D));
    } else {
        spicelib::SETMSG(b"Body code # was not recognized.", ctx);
        spicelib::ERRINT(b"#", BODY, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
        spicelib::CHKOUT(b"T_PCKANG", ctx)?;
        return Ok(());
    }

    *RA = (spicelib::RPD(ctx) * A0);
    *DEC = (spicelib::RPD(ctx) * D0);
    *PM = (spicelib::RPD(ctx) * W);

    spicelib::CHKOUT(b"T_PCKANG", ctx)?;
    Ok(())
}

//$Procedure T_PCKRAD ( Text PCK body radii )
pub fn T_PCKRAD(BODY: i32, RADII: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut RADII = DummyArrayMut::new(RADII, 1..=3);

    spicelib::CHKIN(b"T_PCKRAD", ctx)?;

    if (BODY == 10) {
        spicelib::VPACK(695700.0, 695700.0, 695700.0, RADII.as_slice_mut());
    } else if (BODY == 199) {
        spicelib::VPACK(2440.53, 2440.53, 2438.26, RADII.as_slice_mut());
    } else if (BODY == 299) {
        spicelib::VPACK(6051.8, 6051.8, 6051.8, RADII.as_slice_mut());
    } else if (BODY == 399) {
        spicelib::VPACK(6378.1366, 6378.1366, 6356.7519, RADII.as_slice_mut());
    } else if (BODY == 499) {
        spicelib::VPACK(3396.19, 3396.19, 3376.2, RADII.as_slice_mut());
    } else if (BODY == 599) {
        spicelib::VPACK(71492.0, 71492.0, 66854.0, RADII.as_slice_mut());
    } else if (BODY == 699) {
        spicelib::VPACK(60268.0, 60268.0, 54364.0, RADII.as_slice_mut());
    } else if (BODY == 799) {
        spicelib::VPACK(25559.0, 25559.0, 24973.0, RADII.as_slice_mut());
    } else if (BODY == 899) {
        spicelib::VPACK(24764.0, 24764.0, 24341.0, RADII.as_slice_mut());
    } else if (BODY == 999) {
        spicelib::VPACK(1188.3, 1188.3, 1188.3, RADII.as_slice_mut());
    } else if (BODY == 301) {
        spicelib::VPACK(1737.4, 1737.4, 1737.4, RADII.as_slice_mut());
    } else if (BODY == 401) {
        spicelib::VPACK(13.0, 11.4, 9.1, RADII.as_slice_mut());
    } else if (BODY == 402) {
        spicelib::VPACK(7.8, 6.0, 5.1, RADII.as_slice_mut());
    } else if (BODY == 501) {
        spicelib::VPACK(1829.4, 1819.4, 1815.7, RADII.as_slice_mut());
    } else if (BODY == 502) {
        spicelib::VPACK(1562.6, 1560.3, 1559.5, RADII.as_slice_mut());
    } else if (BODY == 503) {
        spicelib::VPACK(2631.2, 2631.2, 2631.2, RADII.as_slice_mut());
    } else if (BODY == 504) {
        spicelib::VPACK(2410.3, 2410.3, 2410.3, RADII.as_slice_mut());
    } else if (BODY == 505) {
        spicelib::VPACK(125.0, 73.0, 64.0, RADII.as_slice_mut());
    } else if (BODY == 506) {
        spicelib::VPACK(85.0, 85.0, 85.0, RADII.as_slice_mut());
    } else if (BODY == 507) {
        spicelib::VPACK(40.0, 40.0, 40.0, RADII.as_slice_mut());
    } else if (BODY == 508) {
        spicelib::VPACK(18.0, 18.0, 18.0, RADII.as_slice_mut());
    } else if (BODY == 509) {
        spicelib::VPACK(14.0, 14.0, 14.0, RADII.as_slice_mut());
    } else if (BODY == 510) {
        spicelib::VPACK(12.0, 12.0, 12.0, RADII.as_slice_mut());
    } else if (BODY == 511) {
        spicelib::VPACK(15.0, 15.0, 15.0, RADII.as_slice_mut());
    } else if (BODY == 512) {
        spicelib::VPACK(10.0, 10.0, 10.0, RADII.as_slice_mut());
    } else if (BODY == 513) {
        spicelib::VPACK(5.0, 5.0, 5.0, RADII.as_slice_mut());
    } else if (BODY == 514) {
        spicelib::VPACK(58.0, 49.0, 42.0, RADII.as_slice_mut());
    } else if (BODY == 515) {
        spicelib::VPACK(10.0, 8.0, 7.0, RADII.as_slice_mut());
    } else if (BODY == 516) {
        spicelib::VPACK(30.0, 20.0, 17.0, RADII.as_slice_mut());
    } else if (BODY == 601) {
        spicelib::VPACK(207.8, 196.7, 190.6, RADII.as_slice_mut());
    } else if (BODY == 602) {
        spicelib::VPACK(256.6, 251.4, 248.3, RADII.as_slice_mut());
    } else if (BODY == 603) {
        spicelib::VPACK(538.4, 528.3, 526.3, RADII.as_slice_mut());
    } else if (BODY == 604) {
        spicelib::VPACK(563.4, 561.3, 559.6, RADII.as_slice_mut());
    } else if (BODY == 605) {
        spicelib::VPACK(765.0, 763.1, 762.4, RADII.as_slice_mut());
    } else if (BODY == 606) {
        spicelib::VPACK(2575.15, 2574.78, 2574.47, RADII.as_slice_mut());
    } else if (BODY == 607) {
        spicelib::VPACK(180.1, 133.0, 102.7, RADII.as_slice_mut());
    } else if (BODY == 608) {
        spicelib::VPACK(745.7, 745.7, 712.1, RADII.as_slice_mut());
    } else if (BODY == 609) {
        spicelib::VPACK(109.4, 108.5, 101.8, RADII.as_slice_mut());
    } else if (BODY == 610) {
        spicelib::VPACK(101.7, 93.0, 76.3, RADII.as_slice_mut());
    } else if (BODY == 611) {
        spicelib::VPACK(64.9, 57.3, 53.0, RADII.as_slice_mut());
    } else if (BODY == 612) {
        spicelib::VPACK(22.5, 19.6, 13.3, RADII.as_slice_mut());
    } else if (BODY == 613) {
        spicelib::VPACK(16.3, 11.8, 9.8, RADII.as_slice_mut());
    } else if (BODY == 614) {
        spicelib::VPACK(15.3, 9.3, 6.3, RADII.as_slice_mut());
    } else if (BODY == 615) {
        spicelib::VPACK(20.5, 17.8, 9.4, RADII.as_slice_mut());
    } else if (BODY == 616) {
        spicelib::VPACK(68.2, 41.6, 28.2, RADII.as_slice_mut());
    } else if (BODY == 617) {
        spicelib::VPACK(52.2, 40.8, 31.5, RADII.as_slice_mut());
    } else if (BODY == 618) {
        spicelib::VPACK(17.2, 15.4, 10.4, RADII.as_slice_mut());
    } else if (BODY == 632) {
        spicelib::VPACK(1.94, 1.29, 1.21, RADII.as_slice_mut());
    } else if (BODY == 633) {
        spicelib::VPACK(2.88, 2.08, 1.8, RADII.as_slice_mut());
    } else if (BODY == 634) {
        spicelib::VPACK(1.5, 1.2, 1.0, RADII.as_slice_mut());
    } else if (BODY == 635) {
        spicelib::VPACK(4.6, 4.5, 2.8, RADII.as_slice_mut());
    } else if (BODY == 649) {
        spicelib::VPACK(0.5, 0.5, 0.5, RADII.as_slice_mut());
    } else if (BODY == 653) {
        spicelib::VPACK(0.7, 0.25, 0.2, RADII.as_slice_mut());
    } else if (BODY == 701) {
        spicelib::VPACK(581.1, 577.9, 577.7, RADII.as_slice_mut());
    } else if (BODY == 702) {
        spicelib::VPACK(584.7, 584.7, 584.7, RADII.as_slice_mut());
    } else if (BODY == 703) {
        spicelib::VPACK(788.9, 788.9, 788.9, RADII.as_slice_mut());
    } else if (BODY == 704) {
        spicelib::VPACK(761.4, 761.4, 761.4, RADII.as_slice_mut());
    } else if (BODY == 705) {
        spicelib::VPACK(240.4, 234.2, 232.9, RADII.as_slice_mut());
    } else if (BODY == 706) {
        spicelib::VPACK(13.0, 13.0, 13.0, RADII.as_slice_mut());
    } else if (BODY == 707) {
        spicelib::VPACK(15.0, 15.0, 15.0, RADII.as_slice_mut());
    } else if (BODY == 708) {
        spicelib::VPACK(21.0, 21.0, 21.0, RADII.as_slice_mut());
    } else if (BODY == 709) {
        spicelib::VPACK(31.0, 31.0, 31.0, RADII.as_slice_mut());
    } else if (BODY == 710) {
        spicelib::VPACK(27.0, 27.0, 27.0, RADII.as_slice_mut());
    } else if (BODY == 711) {
        spicelib::VPACK(42.0, 42.0, 42.0, RADII.as_slice_mut());
    } else if (BODY == 712) {
        spicelib::VPACK(54.0, 54.0, 54.0, RADII.as_slice_mut());
    } else if (BODY == 713) {
        spicelib::VPACK(27.0, 27.0, 27.0, RADII.as_slice_mut());
    } else if (BODY == 714) {
        spicelib::VPACK(33.0, 33.0, 33.0, RADII.as_slice_mut());
    } else if (BODY == 715) {
        spicelib::VPACK(77.0, 77.0, 77.0, RADII.as_slice_mut());
    } else if (BODY == 801) {
        spicelib::VPACK(1352.6, 1352.6, 1352.6, RADII.as_slice_mut());
    } else if (BODY == 802) {
        spicelib::VPACK(170.0, 170.0, 170.0, RADII.as_slice_mut());
    } else if (BODY == 803) {
        spicelib::VPACK(29.0, 29.0, 29.0, RADII.as_slice_mut());
    } else if (BODY == 804) {
        spicelib::VPACK(40.0, 40.0, 40.0, RADII.as_slice_mut());
    } else if (BODY == 805) {
        spicelib::VPACK(74.0, 74.0, 74.0, RADII.as_slice_mut());
    } else if (BODY == 806) {
        spicelib::VPACK(79.0, 79.0, 79.0, RADII.as_slice_mut());
    } else if (BODY == 807) {
        spicelib::VPACK(96.0, 96.0, 96.0, RADII.as_slice_mut());
    } else if (BODY == 808) {
        spicelib::VPACK(218.0, 208.0, 201.0, RADII.as_slice_mut());
    } else if (BODY == 901) {
        spicelib::VPACK(606.0, 606.0, 606.0, RADII.as_slice_mut());
    } else if (BODY == 1000005) {
        //
        // Borrelly. (Effective radius)
        //
        spicelib::VPACK(4.22, 4.22, 4.22, RADII.as_slice_mut());
    } else if (BODY == 1000012) {
        //
        // Churyumov-Gerasimenko.
        //
        spicelib::VPACK(2.4, 1.55, 1.2, RADII.as_slice_mut());
    } else if (BODY == 1000036) {
        //
        // Halley.
        //
        spicelib::VPACK(8.0, 4.0, 4.0, RADII.as_slice_mut());
    } else if (BODY == 1000093) {
        //
        // Tempel 1.  (Effective radius)
        //
        spicelib::VPACK(3.0, 3.0, 3.0, RADII.as_slice_mut());
    } else if (BODY == 1000041) {
        //
        // Hartley 2.
        //
        //  CALL VPACK ( 0.34D0, 1.16D0, 0.34, RADII )

        spicelib::VPACK(0.58, 0.58, 0.58, RADII.as_slice_mut());
    } else if (BODY == 1000107) {
        //
        // Wild 2.
        //
        spicelib::VPACK(2.7, 1.9, 1.5, RADII.as_slice_mut());
    } else if (BODY == 2000001) {
        //
        // Ceres.
        //
        spicelib::VPACK(487.3, 487.3, 446.0, RADII.as_slice_mut());
    } else if (BODY == 2000004) {
        //
        // Vesta.
        //
        spicelib::VPACK(289.0, 280.0, 229.0, RADII.as_slice_mut());
    } else if (BODY == 2000021) {
        //
        // Lutetia.
        //
        spicelib::VPACK(62.0, 50.5, 46.5, RADII.as_slice_mut());
    } else if (BODY == 2000016) {
        //
        // Psyche.
        //
        spicelib::VPACK(139.5, 116.0, 94.5, RADII.as_slice_mut());
    } else if (BODY == 2000052) {
        //
        // Europa.
        //
        spicelib::VPACK(189.5, 165.0, 124.5, RADII.as_slice_mut());
    } else if (BODY == 2000253) {
        //
        // Mathilde.
        //
        spicelib::VPACK(33.0, 24.0, 23.0, RADII.as_slice_mut());
    } else if (BODY == 2000433) {
        //
        // Eros.
        //
        spicelib::VPACK(17.0, 5.5, 5.5, RADII.as_slice_mut());
    } else if (BODY == 2000511) {
        //
        // Davida.
        //
        spicelib::VPACK(180.0, 147.0, 127.0, RADII.as_slice_mut());
    } else if (BODY == 2002867) {
        //
        // Steins.
        //
        spicelib::VPACK(3.24, 2.73, 2.04, RADII.as_slice_mut());
    } else if (BODY == 2004179) {
        //
        // Toutatis.
        //
        spicelib::VPACK(2.13, 1.015, 0.85, RADII.as_slice_mut());
    } else if (BODY == 2025143) {
        //
        // Itokawa.
        //
        spicelib::VPACK(0.268, 0.147, 0.104, RADII.as_slice_mut());
    } else if (BODY == 2431010) {
        //
        // Ida.
        //
        spicelib::VPACK(26.8, 12.0, 7.6, RADII.as_slice_mut());
    } else if (BODY == 9511010) {
        //
        // Gaspra.
        //
        spicelib::VPACK(9.1, 5.2, 4.4, RADII.as_slice_mut());
    } else {
        spicelib::SETMSG(b"Body code # was not recognized.", ctx);
        spicelib::ERRINT(b"#", BODY, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDBODYCODE)", ctx)?;
        spicelib::CHKOUT(b"T_PCKRAD", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_PCKRAD", ctx)?;
    Ok(())
}
