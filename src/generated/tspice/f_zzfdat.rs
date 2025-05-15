//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const NAMLEN: i32 = 32;

//$Procedure F_ZZFDAT ( ZZFDAT Test Family )
pub fn F_ZZFDAT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ATNAME = ActualCharArray::new(NAMLEN, 1..=(NINERT + NNINRT));
    let mut ATCODE = StackArray::<i32, 145>::new(1..=(NINERT + NNINRT));
    let mut ATCENT = StackArray::<i32, 145>::new(1..=(NINERT + NNINRT));
    let mut ATCLSS = StackArray::<i32, 145>::new(1..=(NINERT + NNINRT));
    let mut ATTYPE = StackArray::<i32, 145>::new(1..=(NINERT + NNINRT));
    let mut FRNAME = [b' '; NAMLEN as usize];
    let mut FRCODE: i32 = 0;
    let mut FRCENT: i32 = 0;
    let mut FRTYPE: i32 = 0;
    let mut FRCLSS: i32 = 0;
    let mut BCODE: i32 = 0;
    let mut NTOCHK: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // SPICELIB functions
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZFDAT", ctx)?;

    //
    // Set expected values.
    //
    fstr::assign(ATNAME.get_mut(1), b"IAU_MERCURY_BARYCENTER");
    ATCODE[1] = 10001;
    ATCENT[1] = 1;
    ATCLSS[1] = 1;
    ATTYPE[1] = PCK;

    fstr::assign(ATNAME.get_mut(2), b"IAU_VENUS_BARYCENTER");
    ATCODE[2] = 10002;
    ATCENT[2] = 2;
    ATCLSS[2] = 2;
    ATTYPE[2] = PCK;

    fstr::assign(ATNAME.get_mut(3), b"IAU_EARTH_BARYCENTER");
    ATCODE[3] = 10003;
    ATCENT[3] = 3;
    ATCLSS[3] = 3;
    ATTYPE[3] = PCK;

    fstr::assign(ATNAME.get_mut(4), b"IAU_MARS_BARYCENTER");
    ATCODE[4] = 10004;
    ATCENT[4] = 4;
    ATCLSS[4] = 4;
    ATTYPE[4] = PCK;

    fstr::assign(ATNAME.get_mut(5), b"IAU_JUPITER_BARYCENTER");
    ATCODE[5] = 10005;
    ATCENT[5] = 5;
    ATCLSS[5] = 5;
    ATTYPE[5] = PCK;

    fstr::assign(ATNAME.get_mut(6), b"IAU_SATURN_BARYCENTER");
    ATCODE[6] = 10006;
    ATCENT[6] = 6;
    ATCLSS[6] = 6;
    ATTYPE[6] = PCK;

    fstr::assign(ATNAME.get_mut(7), b"IAU_URANUS_BARYCENTER");
    ATCODE[7] = 10007;
    ATCENT[7] = 7;
    ATCLSS[7] = 7;
    ATTYPE[7] = PCK;

    fstr::assign(ATNAME.get_mut(8), b"IAU_NEPTUNE_BARYCENTER");
    ATCODE[8] = 10008;
    ATCENT[8] = 8;
    ATCLSS[8] = 8;
    ATTYPE[8] = PCK;

    fstr::assign(ATNAME.get_mut(9), b"IAU_PLUTO_BARYCENTER");
    ATCODE[9] = 10009;
    ATCENT[9] = 9;
    ATCLSS[9] = 9;
    ATTYPE[9] = PCK;

    fstr::assign(ATNAME.get_mut(10), b"IAU_SUN");
    ATCODE[10] = 10010;
    ATCENT[10] = 10;
    ATCLSS[10] = 10;
    ATTYPE[10] = PCK;

    fstr::assign(ATNAME.get_mut(11), b"IAU_MERCURY");
    ATCODE[11] = 10011;
    ATCENT[11] = 199;
    ATCLSS[11] = 199;
    ATTYPE[11] = PCK;

    fstr::assign(ATNAME.get_mut(12), b"IAU_VENUS");
    ATCODE[12] = 10012;
    ATCENT[12] = 299;
    ATCLSS[12] = 299;
    ATTYPE[12] = PCK;

    fstr::assign(ATNAME.get_mut(13), b"IAU_EARTH");
    ATCODE[13] = 10013;
    ATCENT[13] = 399;
    ATCLSS[13] = 399;
    ATTYPE[13] = PCK;

    fstr::assign(ATNAME.get_mut(14), b"IAU_MARS");
    ATCODE[14] = 10014;
    ATCENT[14] = 499;
    ATCLSS[14] = 499;
    ATTYPE[14] = PCK;

    fstr::assign(ATNAME.get_mut(15), b"IAU_JUPITER");
    ATCODE[15] = 10015;
    ATCENT[15] = 599;
    ATCLSS[15] = 599;
    ATTYPE[15] = PCK;

    fstr::assign(ATNAME.get_mut(16), b"IAU_SATURN");
    ATCODE[16] = 10016;
    ATCENT[16] = 699;
    ATCLSS[16] = 699;
    ATTYPE[16] = PCK;

    fstr::assign(ATNAME.get_mut(17), b"IAU_URANUS");
    ATCODE[17] = 10017;
    ATCENT[17] = 799;
    ATCLSS[17] = 799;
    ATTYPE[17] = PCK;

    fstr::assign(ATNAME.get_mut(18), b"IAU_NEPTUNE");
    ATCODE[18] = 10018;
    ATCENT[18] = 899;
    ATCLSS[18] = 899;
    ATTYPE[18] = PCK;

    fstr::assign(ATNAME.get_mut(19), b"IAU_PLUTO");
    ATCODE[19] = 10019;
    ATCENT[19] = 999;
    ATCLSS[19] = 999;
    ATTYPE[19] = PCK;

    fstr::assign(ATNAME.get_mut(20), b"IAU_MOON");
    ATCODE[20] = 10020;
    ATCENT[20] = 301;
    ATCLSS[20] = 301;
    ATTYPE[20] = PCK;

    fstr::assign(ATNAME.get_mut(21), b"IAU_PHOBOS");
    ATCODE[21] = 10021;
    ATCENT[21] = 401;
    ATCLSS[21] = 401;
    ATTYPE[21] = PCK;

    fstr::assign(ATNAME.get_mut(22), b"IAU_DEIMOS");
    ATCODE[22] = 10022;
    ATCENT[22] = 402;
    ATCLSS[22] = 402;
    ATTYPE[22] = PCK;

    fstr::assign(ATNAME.get_mut(23), b"IAU_IO");
    ATCODE[23] = 10023;
    ATCENT[23] = 501;
    ATCLSS[23] = 501;
    ATTYPE[23] = PCK;

    fstr::assign(ATNAME.get_mut(24), b"IAU_EUROPA");
    ATCODE[24] = 10024;
    ATCENT[24] = 502;
    ATCLSS[24] = 502;
    ATTYPE[24] = PCK;

    fstr::assign(ATNAME.get_mut(25), b"IAU_GANYMEDE");
    ATCODE[25] = 10025;
    ATCENT[25] = 503;
    ATCLSS[25] = 503;
    ATTYPE[25] = PCK;

    fstr::assign(ATNAME.get_mut(26), b"IAU_CALLISTO");
    ATCODE[26] = 10026;
    ATCENT[26] = 504;
    ATCLSS[26] = 504;
    ATTYPE[26] = PCK;

    fstr::assign(ATNAME.get_mut(27), b"IAU_AMALTHEA");
    ATCODE[27] = 10027;
    ATCENT[27] = 505;
    ATCLSS[27] = 505;
    ATTYPE[27] = PCK;

    fstr::assign(ATNAME.get_mut(28), b"IAU_HIMALIA");
    ATCODE[28] = 10028;
    ATCENT[28] = 506;
    ATCLSS[28] = 506;
    ATTYPE[28] = PCK;

    fstr::assign(ATNAME.get_mut(29), b"IAU_ELARA");
    ATCODE[29] = 10029;
    ATCENT[29] = 507;
    ATCLSS[29] = 507;
    ATTYPE[29] = PCK;

    fstr::assign(ATNAME.get_mut(30), b"IAU_PASIPHAE");
    ATCODE[30] = 10030;
    ATCENT[30] = 508;
    ATCLSS[30] = 508;
    ATTYPE[30] = PCK;

    fstr::assign(ATNAME.get_mut(31), b"IAU_SINOPE");
    ATCODE[31] = 10031;
    ATCENT[31] = 509;
    ATCLSS[31] = 509;
    ATTYPE[31] = PCK;

    fstr::assign(ATNAME.get_mut(32), b"IAU_LYSITHEA");
    ATCODE[32] = 10032;
    ATCENT[32] = 510;
    ATCLSS[32] = 510;
    ATTYPE[32] = PCK;

    fstr::assign(ATNAME.get_mut(33), b"IAU_CARME");
    ATCODE[33] = 10033;
    ATCENT[33] = 511;
    ATCLSS[33] = 511;
    ATTYPE[33] = PCK;

    fstr::assign(ATNAME.get_mut(34), b"IAU_ANANKE");
    ATCODE[34] = 10034;
    ATCENT[34] = 512;
    ATCLSS[34] = 512;
    ATTYPE[34] = PCK;

    fstr::assign(ATNAME.get_mut(35), b"IAU_LEDA");
    ATCODE[35] = 10035;
    ATCENT[35] = 513;
    ATCLSS[35] = 513;
    ATTYPE[35] = PCK;

    fstr::assign(ATNAME.get_mut(36), b"IAU_THEBE");
    ATCODE[36] = 10036;
    ATCENT[36] = 514;
    ATCLSS[36] = 514;
    ATTYPE[36] = PCK;

    fstr::assign(ATNAME.get_mut(37), b"IAU_ADRASTEA");
    ATCODE[37] = 10037;
    ATCENT[37] = 515;
    ATCLSS[37] = 515;
    ATTYPE[37] = PCK;

    fstr::assign(ATNAME.get_mut(38), b"IAU_METIS");
    ATCODE[38] = 10038;
    ATCENT[38] = 516;
    ATCLSS[38] = 516;
    ATTYPE[38] = PCK;

    fstr::assign(ATNAME.get_mut(39), b"IAU_MIMAS");
    ATCODE[39] = 10039;
    ATCENT[39] = 601;
    ATCLSS[39] = 601;
    ATTYPE[39] = PCK;

    fstr::assign(ATNAME.get_mut(40), b"IAU_ENCELADUS");
    ATCODE[40] = 10040;
    ATCENT[40] = 602;
    ATCLSS[40] = 602;
    ATTYPE[40] = PCK;

    fstr::assign(ATNAME.get_mut(41), b"IAU_TETHYS");
    ATCODE[41] = 10041;
    ATCENT[41] = 603;
    ATCLSS[41] = 603;
    ATTYPE[41] = PCK;

    fstr::assign(ATNAME.get_mut(42), b"IAU_DIONE");
    ATCODE[42] = 10042;
    ATCENT[42] = 604;
    ATCLSS[42] = 604;
    ATTYPE[42] = PCK;

    fstr::assign(ATNAME.get_mut(43), b"IAU_RHEA");
    ATCODE[43] = 10043;
    ATCENT[43] = 605;
    ATCLSS[43] = 605;
    ATTYPE[43] = PCK;

    fstr::assign(ATNAME.get_mut(44), b"IAU_TITAN");
    ATCODE[44] = 10044;
    ATCENT[44] = 606;
    ATCLSS[44] = 606;
    ATTYPE[44] = PCK;

    fstr::assign(ATNAME.get_mut(45), b"IAU_HYPERION");
    ATCODE[45] = 10045;
    ATCENT[45] = 607;
    ATCLSS[45] = 607;
    ATTYPE[45] = PCK;

    fstr::assign(ATNAME.get_mut(46), b"IAU_IAPETUS");
    ATCODE[46] = 10046;
    ATCENT[46] = 608;
    ATCLSS[46] = 608;
    ATTYPE[46] = PCK;

    fstr::assign(ATNAME.get_mut(47), b"IAU_PHOEBE");
    ATCODE[47] = 10047;
    ATCENT[47] = 609;
    ATCLSS[47] = 609;
    ATTYPE[47] = PCK;

    fstr::assign(ATNAME.get_mut(48), b"IAU_JANUS");
    ATCODE[48] = 10048;
    ATCENT[48] = 610;
    ATCLSS[48] = 610;
    ATTYPE[48] = PCK;

    fstr::assign(ATNAME.get_mut(49), b"IAU_EPIMETHEUS");
    ATCODE[49] = 10049;
    ATCENT[49] = 611;
    ATCLSS[49] = 611;
    ATTYPE[49] = PCK;

    fstr::assign(ATNAME.get_mut(50), b"IAU_HELENE");
    ATCODE[50] = 10050;
    ATCENT[50] = 612;
    ATCLSS[50] = 612;
    ATTYPE[50] = PCK;

    fstr::assign(ATNAME.get_mut(51), b"IAU_TELESTO");
    ATCODE[51] = 10051;
    ATCENT[51] = 613;
    ATCLSS[51] = 613;
    ATTYPE[51] = PCK;

    fstr::assign(ATNAME.get_mut(52), b"IAU_CALYPSO");
    ATCODE[52] = 10052;
    ATCENT[52] = 614;
    ATCLSS[52] = 614;
    ATTYPE[52] = PCK;

    fstr::assign(ATNAME.get_mut(53), b"IAU_ATLAS");
    ATCODE[53] = 10053;
    ATCENT[53] = 615;
    ATCLSS[53] = 615;
    ATTYPE[53] = PCK;

    fstr::assign(ATNAME.get_mut(54), b"IAU_PROMETHEUS");
    ATCODE[54] = 10054;
    ATCENT[54] = 616;
    ATCLSS[54] = 616;
    ATTYPE[54] = PCK;

    fstr::assign(ATNAME.get_mut(55), b"IAU_PANDORA");
    ATCODE[55] = 10055;
    ATCENT[55] = 617;
    ATCLSS[55] = 617;
    ATTYPE[55] = PCK;

    fstr::assign(ATNAME.get_mut(56), b"IAU_ARIEL");
    ATCODE[56] = 10056;
    ATCENT[56] = 701;
    ATCLSS[56] = 701;
    ATTYPE[56] = PCK;

    fstr::assign(ATNAME.get_mut(57), b"IAU_UMBRIEL");
    ATCODE[57] = 10057;
    ATCENT[57] = 702;
    ATCLSS[57] = 702;
    ATTYPE[57] = PCK;

    fstr::assign(ATNAME.get_mut(58), b"IAU_TITANIA");
    ATCODE[58] = 10058;
    ATCENT[58] = 703;
    ATCLSS[58] = 703;
    ATTYPE[58] = PCK;

    fstr::assign(ATNAME.get_mut(59), b"IAU_OBERON");
    ATCODE[59] = 10059;
    ATCENT[59] = 704;
    ATCLSS[59] = 704;
    ATTYPE[59] = PCK;

    fstr::assign(ATNAME.get_mut(60), b"IAU_MIRANDA");
    ATCODE[60] = 10060;
    ATCENT[60] = 705;
    ATCLSS[60] = 705;
    ATTYPE[60] = PCK;

    fstr::assign(ATNAME.get_mut(61), b"IAU_CORDELIA");
    ATCODE[61] = 10061;
    ATCENT[61] = 706;
    ATCLSS[61] = 706;
    ATTYPE[61] = PCK;

    fstr::assign(ATNAME.get_mut(62), b"IAU_OPHELIA");
    ATCODE[62] = 10062;
    ATCENT[62] = 707;
    ATCLSS[62] = 707;
    ATTYPE[62] = PCK;

    fstr::assign(ATNAME.get_mut(63), b"IAU_BIANCA");
    ATCODE[63] = 10063;
    ATCENT[63] = 708;
    ATCLSS[63] = 708;
    ATTYPE[63] = PCK;

    fstr::assign(ATNAME.get_mut(64), b"IAU_CRESSIDA");
    ATCODE[64] = 10064;
    ATCENT[64] = 709;
    ATCLSS[64] = 709;
    ATTYPE[64] = PCK;

    fstr::assign(ATNAME.get_mut(65), b"IAU_DESDEMONA");
    ATCODE[65] = 10065;
    ATCENT[65] = 710;
    ATCLSS[65] = 710;
    ATTYPE[65] = PCK;

    fstr::assign(ATNAME.get_mut(66), b"IAU_JULIET");
    ATCODE[66] = 10066;
    ATCENT[66] = 711;
    ATCLSS[66] = 711;
    ATTYPE[66] = PCK;

    fstr::assign(ATNAME.get_mut(67), b"IAU_PORTIA");
    ATCODE[67] = 10067;
    ATCENT[67] = 712;
    ATCLSS[67] = 712;
    ATTYPE[67] = PCK;

    fstr::assign(ATNAME.get_mut(68), b"IAU_ROSALIND");
    ATCODE[68] = 10068;
    ATCENT[68] = 713;
    ATCLSS[68] = 713;
    ATTYPE[68] = PCK;

    fstr::assign(ATNAME.get_mut(69), b"IAU_BELINDA");
    ATCODE[69] = 10069;
    ATCENT[69] = 714;
    ATCLSS[69] = 714;
    ATTYPE[69] = PCK;

    fstr::assign(ATNAME.get_mut(70), b"IAU_PUCK");
    ATCODE[70] = 10070;
    ATCENT[70] = 715;
    ATCLSS[70] = 715;
    ATTYPE[70] = PCK;

    fstr::assign(ATNAME.get_mut(71), b"IAU_TRITON");
    ATCODE[71] = 10071;
    ATCENT[71] = 801;
    ATCLSS[71] = 801;
    ATTYPE[71] = PCK;

    fstr::assign(ATNAME.get_mut(72), b"IAU_NEREID");
    ATCODE[72] = 10072;
    ATCENT[72] = 802;
    ATCLSS[72] = 802;
    ATTYPE[72] = PCK;

    fstr::assign(ATNAME.get_mut(73), b"IAU_NAIAD");
    ATCODE[73] = 10073;
    ATCENT[73] = 803;
    ATCLSS[73] = 803;
    ATTYPE[73] = PCK;

    fstr::assign(ATNAME.get_mut(74), b"IAU_THALASSA");
    ATCODE[74] = 10074;
    ATCENT[74] = 804;
    ATCLSS[74] = 804;
    ATTYPE[74] = PCK;

    fstr::assign(ATNAME.get_mut(75), b"IAU_DESPINA");
    ATCODE[75] = 10075;
    ATCENT[75] = 805;
    ATCLSS[75] = 805;
    ATTYPE[75] = PCK;

    fstr::assign(ATNAME.get_mut(76), b"IAU_GALATEA");
    ATCODE[76] = 10076;
    ATCENT[76] = 806;
    ATCLSS[76] = 806;
    ATTYPE[76] = PCK;

    fstr::assign(ATNAME.get_mut(77), b"IAU_LARISSA");
    ATCODE[77] = 10077;
    ATCENT[77] = 807;
    ATCLSS[77] = 807;
    ATTYPE[77] = PCK;

    fstr::assign(ATNAME.get_mut(78), b"IAU_PROTEUS");
    ATCODE[78] = 10078;
    ATCENT[78] = 808;
    ATCLSS[78] = 808;
    ATTYPE[78] = PCK;

    fstr::assign(ATNAME.get_mut(79), b"IAU_CHARON");
    ATCODE[79] = 10079;
    ATCENT[79] = 901;
    ATCLSS[79] = 901;
    ATTYPE[79] = PCK;

    fstr::assign(ATNAME.get_mut(80), b"ITRF93");
    ATCODE[80] = 13000;
    ATCENT[80] = 399;
    ATCLSS[80] = 3000;
    ATTYPE[80] = PCK;

    fstr::assign(ATNAME.get_mut(81), b"EARTH_FIXED");
    ATCODE[81] = 10081;
    ATCENT[81] = 399;
    ATCLSS[81] = 10081;
    ATTYPE[81] = TK;

    fstr::assign(ATNAME.get_mut(82), b"IAU_PAN");
    ATCODE[82] = 10082;
    ATCENT[82] = 618;
    ATCLSS[82] = 618;
    ATTYPE[82] = PCK;

    fstr::assign(ATNAME.get_mut(83), b"IAU_GASPRA");
    ATCODE[83] = 10083;
    ATCENT[83] = 9511010;
    ATCLSS[83] = 9511010;
    ATTYPE[83] = PCK;

    fstr::assign(ATNAME.get_mut(84), b"IAU_IDA");
    ATCODE[84] = 10084;
    ATCENT[84] = 2431010;
    ATCLSS[84] = 2431010;
    ATTYPE[84] = PCK;

    fstr::assign(ATNAME.get_mut(85), b"IAU_EROS");
    ATCODE[85] = 10085;
    ATCENT[85] = 2000433;
    ATCLSS[85] = 2000433;
    ATTYPE[85] = PCK;

    fstr::assign(ATNAME.get_mut(86), b"IAU_CALLIRRHOE");
    ATCODE[86] = 10086;
    ATCENT[86] = 517;
    ATCLSS[86] = 517;
    ATTYPE[86] = PCK;

    fstr::assign(ATNAME.get_mut(87), b"IAU_THEMISTO");
    ATCODE[87] = 10087;
    ATCENT[87] = 518;
    ATCLSS[87] = 518;
    ATTYPE[87] = PCK;

    fstr::assign(ATNAME.get_mut(88), b"IAU_MEGACLITE");
    ATCODE[88] = 10088;
    ATCENT[88] = 519;
    ATCLSS[88] = 519;
    ATTYPE[88] = PCK;

    fstr::assign(ATNAME.get_mut(89), b"IAU_TAYGETE");
    ATCODE[89] = 10089;
    ATCENT[89] = 520;
    ATCLSS[89] = 520;
    ATTYPE[89] = PCK;

    fstr::assign(ATNAME.get_mut(90), b"IAU_CHALDENE");
    ATCODE[90] = 10090;
    ATCENT[90] = 521;
    ATCLSS[90] = 521;
    ATTYPE[90] = PCK;

    fstr::assign(ATNAME.get_mut(91), b"IAU_HARPALYKE");
    ATCODE[91] = 10091;
    ATCENT[91] = 522;
    ATCLSS[91] = 522;
    ATTYPE[91] = PCK;

    fstr::assign(ATNAME.get_mut(92), b"IAU_KALYKE");
    ATCODE[92] = 10092;
    ATCENT[92] = 523;
    ATCLSS[92] = 523;
    ATTYPE[92] = PCK;

    fstr::assign(ATNAME.get_mut(93), b"IAU_IOCASTE");
    ATCODE[93] = 10093;
    ATCENT[93] = 524;
    ATCLSS[93] = 524;
    ATTYPE[93] = PCK;

    fstr::assign(ATNAME.get_mut(94), b"IAU_ERINOME");
    ATCODE[94] = 10094;
    ATCENT[94] = 525;
    ATCLSS[94] = 525;
    ATTYPE[94] = PCK;

    fstr::assign(ATNAME.get_mut(95), b"IAU_ISONOE");
    ATCODE[95] = 10095;
    ATCENT[95] = 526;
    ATCLSS[95] = 526;
    ATTYPE[95] = PCK;

    fstr::assign(ATNAME.get_mut(96), b"IAU_PRAXIDIKE");
    ATCODE[96] = 10096;
    ATCENT[96] = 527;
    ATCLSS[96] = 527;
    ATTYPE[96] = PCK;

    fstr::assign(ATNAME.get_mut(97), b"IAU_BORRELLY");
    ATCODE[97] = 10097;
    ATCENT[97] = 1000005;
    ATCLSS[97] = 1000005;
    ATTYPE[97] = PCK;

    fstr::assign(ATNAME.get_mut(98), b"IAU_TEMPEL_1");
    ATCODE[98] = 10098;
    ATCENT[98] = 1000093;
    ATCLSS[98] = 1000093;
    ATTYPE[98] = PCK;

    fstr::assign(ATNAME.get_mut(99), b"IAU_VESTA");
    ATCODE[99] = 10099;
    ATCENT[99] = 2000004;
    ATCLSS[99] = 2000004;
    ATTYPE[99] = PCK;

    fstr::assign(ATNAME.get_mut(100), b"IAU_ITOKAWA");
    ATCODE[100] = 10100;
    ATCENT[100] = 2025143;
    ATCLSS[100] = 2025143;
    ATTYPE[100] = PCK;

    fstr::assign(ATNAME.get_mut(101), b"IAU_CERES");
    ATCODE[101] = 10101;
    ATCENT[101] = 2000001;
    ATCLSS[101] = 2000001;
    ATTYPE[101] = PCK;

    fstr::assign(ATNAME.get_mut(102), b"IAU_PALLAS");
    ATCODE[102] = 10102;
    ATCENT[102] = 2000002;
    ATCLSS[102] = 2000002;
    ATTYPE[102] = PCK;

    fstr::assign(ATNAME.get_mut(103), b"IAU_LUTETIA");
    ATCODE[103] = 10103;
    ATCENT[103] = 2000021;
    ATCLSS[103] = 2000021;
    ATTYPE[103] = PCK;

    fstr::assign(ATNAME.get_mut(104), b"IAU_DAVIDA");
    ATCODE[104] = 10104;
    ATCENT[104] = 2000511;
    ATCLSS[104] = 2000511;
    ATTYPE[104] = PCK;

    fstr::assign(ATNAME.get_mut(105), b"IAU_STEINS");
    ATCODE[105] = 10105;
    ATCENT[105] = 2002867;
    ATCLSS[105] = 2002867;
    ATTYPE[105] = PCK;

    fstr::assign(ATNAME.get_mut(106), b"IAU_BENNU");
    ATCODE[106] = 10106;
    ATCENT[106] = 2101955;
    ATCLSS[106] = 2101955;
    ATTYPE[106] = PCK;

    fstr::assign(ATNAME.get_mut(107), b"IAU_52_EUROPA");
    ATCODE[107] = 10107;
    ATCENT[107] = 2000052;
    ATCLSS[107] = 2000052;
    ATTYPE[107] = PCK;

    fstr::assign(ATNAME.get_mut(108), b"IAU_NIX");
    ATCODE[108] = 10108;
    ATCENT[108] = 902;
    ATCLSS[108] = 902;
    ATTYPE[108] = PCK;

    fstr::assign(ATNAME.get_mut(109), b"IAU_HYDRA");
    ATCODE[109] = 10109;
    ATCENT[109] = 903;
    ATCLSS[109] = 903;
    ATTYPE[109] = PCK;

    fstr::assign(ATNAME.get_mut(110), b"IAU_RYUGU");
    ATCODE[110] = 10110;
    ATCENT[110] = 2162173;
    ATCLSS[110] = 2162173;
    ATTYPE[110] = PCK;

    fstr::assign(ATNAME.get_mut(111), b"IAU_ARROKOTH");
    ATCODE[111] = 10111;
    ATCENT[111] = 2486958;
    ATCLSS[111] = 2486958;
    ATTYPE[111] = PCK;

    fstr::assign(ATNAME.get_mut(112), b"IAU_DIDYMOS_BARYCENTER");
    ATCODE[112] = 10112;
    ATCENT[112] = 20065803;
    ATCLSS[112] = 20065803;
    ATTYPE[112] = PCK;

    fstr::assign(ATNAME.get_mut(113), b"IAU_DIDYMOS");
    ATCODE[113] = 10113;
    ATCENT[113] = 920065803;
    ATCLSS[113] = 920065803;
    ATTYPE[113] = PCK;

    fstr::assign(ATNAME.get_mut(114), b"IAU_DIMORPHOS");
    ATCODE[114] = 10114;
    ATCENT[114] = 120065803;
    ATCLSS[114] = 120065803;
    ATTYPE[114] = PCK;

    fstr::assign(ATNAME.get_mut(115), b"IAU_DONALDJOHANSON");
    ATCODE[115] = 10115;
    ATCENT[115] = 20052246;
    ATCLSS[115] = 20052246;
    ATTYPE[115] = PCK;

    fstr::assign(ATNAME.get_mut(116), b"IAU_EURYBATES");
    ATCODE[116] = 10116;
    ATCENT[116] = 920003548;
    ATCLSS[116] = 920003548;
    ATTYPE[116] = PCK;

    fstr::assign(ATNAME.get_mut(117), b"IAU_EURYBATES_BARYCENTER");
    ATCODE[117] = 10117;
    ATCENT[117] = 20003548;
    ATCLSS[117] = 20003548;
    ATTYPE[117] = PCK;

    fstr::assign(ATNAME.get_mut(118), b"IAU_QUETA");
    ATCODE[118] = 10118;
    ATCENT[118] = 120003548;
    ATCLSS[118] = 120003548;
    ATTYPE[118] = PCK;

    fstr::assign(ATNAME.get_mut(119), b"IAU_POLYMELE");
    ATCODE[119] = 10119;
    ATCENT[119] = 20015094;
    ATCLSS[119] = 20015094;
    ATTYPE[119] = PCK;

    fstr::assign(ATNAME.get_mut(120), b"IAU_LEUCUS");
    ATCODE[120] = 10120;
    ATCENT[120] = 20011351;
    ATCLSS[120] = 20011351;
    ATTYPE[120] = PCK;

    fstr::assign(ATNAME.get_mut(121), b"IAU_ORUS");
    ATCODE[121] = 10121;
    ATCENT[121] = 20021900;
    ATCLSS[121] = 20021900;
    ATTYPE[121] = PCK;

    fstr::assign(ATNAME.get_mut(122), b"IAU_PATROCLUS_BARYCENTER");
    ATCODE[122] = 10122;
    ATCENT[122] = 20000617;
    ATCLSS[122] = 20000617;
    ATTYPE[122] = PCK;

    fstr::assign(ATNAME.get_mut(123), b"IAU_PATROCLUS");
    ATCODE[123] = 10123;
    ATCENT[123] = 920000617;
    ATCLSS[123] = 920000617;
    ATTYPE[123] = PCK;

    fstr::assign(ATNAME.get_mut(124), b"IAU_MENOETIUS");
    ATCODE[124] = 10124;
    ATCENT[124] = 120000617;
    ATCLSS[124] = 120000617;
    ATTYPE[124] = PCK;

    NTOCHK = 124;

    //
    // Check frames one by one.
    //
    for I in 1..=NTOCHK {
        //
        // Set test case name.
        //
        testutil::TCASE(&fstr::concat(b"ZZFDAT Entries for ", ATNAME.get(I)), ctx)?;

        //
        // Check NAME to CODE and CODE to NAME mappings.
        //
        spicelib::NAMFRM(&ATNAME[I], &mut FRCODE, ctx)?;
        spicelib::FRMNAM(ATCODE[I], &mut FRNAME, ctx)?;

        //
        // Check results.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"FRCODE", FRCODE, b"=", ATCODE[I], 0, OK, ctx)?;
        testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", &ATNAME[I], OK, ctx)?;

        //
        // Now check frame attributes as reported by FRINFO.
        //
        spicelib::FRINFO(
            ATCODE[I],
            &mut FRCENT,
            &mut FRTYPE,
            &mut FRCLSS,
            &mut FOUND,
            ctx,
        )?;

        //
        // Check results.
        //
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CLSSID", FRCLSS, b"=", ATCLSS[I], 0, OK, ctx)?;
        testutil::CHCKSI(b"CLASS", FRTYPE, b"=", ATTYPE[I], 0, OK, ctx)?;
        testutil::CHCKSI(b"CENT", FRCENT, b"=", ATCENT[I], 0, OK, ctx)?;

        //
        // If it's an 'IAU_' style PCK frame, check that ID returned
        // for the body from the frame name is the same as the frame
        // center ID.
        //
        if ((ATTYPE[I] == PCK) && fstr::eq(fstr::substr(ATNAME.get(I), 1..=4), b"IAU_")) {
            spicelib::BODN2C(
                fstr::substr(&ATNAME[I], 5..=spicelib::RTRIM(&ATNAME[I])),
                &mut BCODE,
                &mut FOUND,
                ctx,
            )?;

            //
            // Check results.
            //
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
            testutil::CHCKSI(b"BCODE", BCODE, b"=", ATCENT[I], 0, OK, ctx)?;
        }
    }
    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
