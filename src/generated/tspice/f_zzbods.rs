//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXL: i32 = 36;
const MAXP: i32 = 150;
const NPERM: i32 = 692;
const MAXE: i32 = 853;
const NROOM: i32 = 14983;

//$Procedure F_ZZBODS (Family of tests for ZZBODTRN)
pub fn F_ZZBODS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CODE: i32 = 0;
    let mut C2NCOD = ActualArray::<i32>::new(1..=NPERM);
    let mut N2CCOD = ActualArray::<i32>::new(1..=NPERM);
    let mut NAME = [b' '; MAXL as usize];
    let mut C2NNAM = ActualCharArray::new(MAXL, 1..=NPERM);
    let mut N2CNAM = ActualCharArray::new(MAXL, 1..=NPERM);
    let mut TSTSTR = [b' '; (MAXL + 64) as usize];
    let mut FOUND: bool = false;

    //
    // Local variables.
    //

    //
    // A script generates this file. Do not edit by hand.
    // Edit the creation script to modify the contents of
    // F_ZZBODS.FOR.
    //

    //
    // The data list for the NAME -> ID tests.
    //

    N2CCOD[1] = 0;
    fstr::assign(N2CNAM.get_mut(1), b"SOLAR_SYSTEM_BARYCENTER");

    N2CCOD[2] = 0;
    fstr::assign(N2CNAM.get_mut(2), b"SSB");

    N2CCOD[3] = 0;
    fstr::assign(N2CNAM.get_mut(3), b"SOLAR SYSTEM BARYCENTER");

    N2CCOD[4] = 1;
    fstr::assign(N2CNAM.get_mut(4), b"MERCURY_BARYCENTER");

    N2CCOD[5] = 1;
    fstr::assign(N2CNAM.get_mut(5), b"MERCURY BARYCENTER");

    N2CCOD[6] = 2;
    fstr::assign(N2CNAM.get_mut(6), b"VENUS_BARYCENTER");

    N2CCOD[7] = 2;
    fstr::assign(N2CNAM.get_mut(7), b"VENUS BARYCENTER");

    N2CCOD[8] = 3;
    fstr::assign(N2CNAM.get_mut(8), b"EARTH_BARYCENTER");

    N2CCOD[9] = 3;
    fstr::assign(N2CNAM.get_mut(9), b"EMB");

    N2CCOD[10] = 3;
    fstr::assign(N2CNAM.get_mut(10), b"EARTH MOON BARYCENTER");

    N2CCOD[11] = 3;
    fstr::assign(N2CNAM.get_mut(11), b"EARTH-MOON BARYCENTER");

    N2CCOD[12] = 3;
    fstr::assign(N2CNAM.get_mut(12), b"EARTH BARYCENTER");

    N2CCOD[13] = 4;
    fstr::assign(N2CNAM.get_mut(13), b"MARS_BARYCENTER");

    N2CCOD[14] = 4;
    fstr::assign(N2CNAM.get_mut(14), b"MARS BARYCENTER");

    N2CCOD[15] = 5;
    fstr::assign(N2CNAM.get_mut(15), b"JUPITER_BARYCENTER");

    N2CCOD[16] = 5;
    fstr::assign(N2CNAM.get_mut(16), b"JUPITER BARYCENTER");

    N2CCOD[17] = 6;
    fstr::assign(N2CNAM.get_mut(17), b"SATURN_BARYCENTER");

    N2CCOD[18] = 6;
    fstr::assign(N2CNAM.get_mut(18), b"SATURN BARYCENTER");

    N2CCOD[19] = 7;
    fstr::assign(N2CNAM.get_mut(19), b"URANUS_BARYCENTER");

    N2CCOD[20] = 7;
    fstr::assign(N2CNAM.get_mut(20), b"URANUS BARYCENTER");

    N2CCOD[21] = 8;
    fstr::assign(N2CNAM.get_mut(21), b"NEPTUNE_BARYCENTER");

    N2CCOD[22] = 8;
    fstr::assign(N2CNAM.get_mut(22), b"NEPTUNE BARYCENTER");

    N2CCOD[23] = 9;
    fstr::assign(N2CNAM.get_mut(23), b"PLUTO_BARYCENTER");

    N2CCOD[24] = 9;
    fstr::assign(N2CNAM.get_mut(24), b"PLUTO BARYCENTER");

    N2CCOD[25] = 10;
    fstr::assign(N2CNAM.get_mut(25), b"SUN");

    N2CCOD[26] = 199;
    fstr::assign(N2CNAM.get_mut(26), b"MERCURY");

    N2CCOD[27] = 299;
    fstr::assign(N2CNAM.get_mut(27), b"VENUS");

    N2CCOD[28] = 399;
    fstr::assign(N2CNAM.get_mut(28), b"EARTH");

    N2CCOD[29] = 301;
    fstr::assign(N2CNAM.get_mut(29), b"MOON");

    N2CCOD[30] = 499;
    fstr::assign(N2CNAM.get_mut(30), b"MARS");

    N2CCOD[31] = 401;
    fstr::assign(N2CNAM.get_mut(31), b"PHOBOS");

    N2CCOD[32] = 402;
    fstr::assign(N2CNAM.get_mut(32), b"DEIMOS");

    N2CCOD[33] = 599;
    fstr::assign(N2CNAM.get_mut(33), b"JUPITER");

    N2CCOD[34] = 501;
    fstr::assign(N2CNAM.get_mut(34), b"IO");

    N2CCOD[35] = 502;
    fstr::assign(N2CNAM.get_mut(35), b"EUROPA");

    N2CCOD[36] = 503;
    fstr::assign(N2CNAM.get_mut(36), b"GANYMEDE");

    N2CCOD[37] = 504;
    fstr::assign(N2CNAM.get_mut(37), b"CALLISTO");

    N2CCOD[38] = 505;
    fstr::assign(N2CNAM.get_mut(38), b"AMALTHEA");

    N2CCOD[39] = 506;
    fstr::assign(N2CNAM.get_mut(39), b"HIMALIA");

    N2CCOD[40] = 507;
    fstr::assign(N2CNAM.get_mut(40), b"ELARA");

    N2CCOD[41] = 508;
    fstr::assign(N2CNAM.get_mut(41), b"PASIPHAE");

    N2CCOD[42] = 509;
    fstr::assign(N2CNAM.get_mut(42), b"SINOPE");

    N2CCOD[43] = 510;
    fstr::assign(N2CNAM.get_mut(43), b"LYSITHEA");

    N2CCOD[44] = 511;
    fstr::assign(N2CNAM.get_mut(44), b"CARME");

    N2CCOD[45] = 512;
    fstr::assign(N2CNAM.get_mut(45), b"ANANKE");

    N2CCOD[46] = 513;
    fstr::assign(N2CNAM.get_mut(46), b"LEDA");

    N2CCOD[47] = 514;
    fstr::assign(N2CNAM.get_mut(47), b"THEBE");

    N2CCOD[48] = 515;
    fstr::assign(N2CNAM.get_mut(48), b"ADRASTEA");

    N2CCOD[49] = 516;
    fstr::assign(N2CNAM.get_mut(49), b"METIS");

    N2CCOD[50] = 517;
    fstr::assign(N2CNAM.get_mut(50), b"CALLIRRHOE");

    N2CCOD[51] = 518;
    fstr::assign(N2CNAM.get_mut(51), b"THEMISTO");

    N2CCOD[52] = 519;
    fstr::assign(N2CNAM.get_mut(52), b"MEGACLITE");

    N2CCOD[53] = 520;
    fstr::assign(N2CNAM.get_mut(53), b"TAYGETE");

    N2CCOD[54] = 521;
    fstr::assign(N2CNAM.get_mut(54), b"CHALDENE");

    N2CCOD[55] = 522;
    fstr::assign(N2CNAM.get_mut(55), b"HARPALYKE");

    N2CCOD[56] = 523;
    fstr::assign(N2CNAM.get_mut(56), b"KALYKE");

    N2CCOD[57] = 524;
    fstr::assign(N2CNAM.get_mut(57), b"IOCASTE");

    N2CCOD[58] = 525;
    fstr::assign(N2CNAM.get_mut(58), b"ERINOME");

    N2CCOD[59] = 526;
    fstr::assign(N2CNAM.get_mut(59), b"ISONOE");

    N2CCOD[60] = 527;
    fstr::assign(N2CNAM.get_mut(60), b"PRAXIDIKE");

    N2CCOD[61] = 528;
    fstr::assign(N2CNAM.get_mut(61), b"AUTONOE");

    N2CCOD[62] = 529;
    fstr::assign(N2CNAM.get_mut(62), b"THYONE");

    N2CCOD[63] = 530;
    fstr::assign(N2CNAM.get_mut(63), b"HERMIPPE");

    N2CCOD[64] = 531;
    fstr::assign(N2CNAM.get_mut(64), b"AITNE");

    N2CCOD[65] = 532;
    fstr::assign(N2CNAM.get_mut(65), b"EURYDOME");

    N2CCOD[66] = 533;
    fstr::assign(N2CNAM.get_mut(66), b"EUANTHE");

    N2CCOD[67] = 534;
    fstr::assign(N2CNAM.get_mut(67), b"EUPORIE");

    N2CCOD[68] = 535;
    fstr::assign(N2CNAM.get_mut(68), b"ORTHOSIE");

    N2CCOD[69] = 536;
    fstr::assign(N2CNAM.get_mut(69), b"SPONDE");

    N2CCOD[70] = 537;
    fstr::assign(N2CNAM.get_mut(70), b"KALE");

    N2CCOD[71] = 538;
    fstr::assign(N2CNAM.get_mut(71), b"PASITHEE");

    N2CCOD[72] = 539;
    fstr::assign(N2CNAM.get_mut(72), b"HEGEMONE");

    N2CCOD[73] = 540;
    fstr::assign(N2CNAM.get_mut(73), b"MNEME");

    N2CCOD[74] = 541;
    fstr::assign(N2CNAM.get_mut(74), b"AOEDE");

    N2CCOD[75] = 542;
    fstr::assign(N2CNAM.get_mut(75), b"THELXINOE");

    N2CCOD[76] = 543;
    fstr::assign(N2CNAM.get_mut(76), b"ARCHE");

    N2CCOD[77] = 544;
    fstr::assign(N2CNAM.get_mut(77), b"KALLICHORE");

    N2CCOD[78] = 545;
    fstr::assign(N2CNAM.get_mut(78), b"HELIKE");

    N2CCOD[79] = 546;
    fstr::assign(N2CNAM.get_mut(79), b"CARPO");

    N2CCOD[80] = 547;
    fstr::assign(N2CNAM.get_mut(80), b"EUKELADE");

    N2CCOD[81] = 548;
    fstr::assign(N2CNAM.get_mut(81), b"CYLLENE");

    N2CCOD[82] = 549;
    fstr::assign(N2CNAM.get_mut(82), b"KORE");

    N2CCOD[83] = 550;
    fstr::assign(N2CNAM.get_mut(83), b"HERSE");

    N2CCOD[84] = 553;
    fstr::assign(N2CNAM.get_mut(84), b"DIA");

    N2CCOD[85] = 699;
    fstr::assign(N2CNAM.get_mut(85), b"SATURN");

    N2CCOD[86] = 601;
    fstr::assign(N2CNAM.get_mut(86), b"MIMAS");

    N2CCOD[87] = 602;
    fstr::assign(N2CNAM.get_mut(87), b"ENCELADUS");

    N2CCOD[88] = 603;
    fstr::assign(N2CNAM.get_mut(88), b"TETHYS");

    N2CCOD[89] = 604;
    fstr::assign(N2CNAM.get_mut(89), b"DIONE");

    N2CCOD[90] = 605;
    fstr::assign(N2CNAM.get_mut(90), b"RHEA");

    N2CCOD[91] = 606;
    fstr::assign(N2CNAM.get_mut(91), b"TITAN");

    N2CCOD[92] = 607;
    fstr::assign(N2CNAM.get_mut(92), b"HYPERION");

    N2CCOD[93] = 608;
    fstr::assign(N2CNAM.get_mut(93), b"IAPETUS");

    N2CCOD[94] = 609;
    fstr::assign(N2CNAM.get_mut(94), b"PHOEBE");

    N2CCOD[95] = 610;
    fstr::assign(N2CNAM.get_mut(95), b"JANUS");

    N2CCOD[96] = 611;
    fstr::assign(N2CNAM.get_mut(96), b"EPIMETHEUS");

    N2CCOD[97] = 612;
    fstr::assign(N2CNAM.get_mut(97), b"HELENE");

    N2CCOD[98] = 613;
    fstr::assign(N2CNAM.get_mut(98), b"TELESTO");

    N2CCOD[99] = 614;
    fstr::assign(N2CNAM.get_mut(99), b"CALYPSO");

    N2CCOD[100] = 615;
    fstr::assign(N2CNAM.get_mut(100), b"ATLAS");

    N2CCOD[101] = 616;
    fstr::assign(N2CNAM.get_mut(101), b"PROMETHEUS");

    N2CCOD[102] = 617;
    fstr::assign(N2CNAM.get_mut(102), b"PANDORA");

    N2CCOD[103] = 618;
    fstr::assign(N2CNAM.get_mut(103), b"PAN");

    N2CCOD[104] = 619;
    fstr::assign(N2CNAM.get_mut(104), b"YMIR");

    N2CCOD[105] = 620;
    fstr::assign(N2CNAM.get_mut(105), b"PAALIAQ");

    N2CCOD[106] = 621;
    fstr::assign(N2CNAM.get_mut(106), b"TARVOS");

    N2CCOD[107] = 622;
    fstr::assign(N2CNAM.get_mut(107), b"IJIRAQ");

    N2CCOD[108] = 623;
    fstr::assign(N2CNAM.get_mut(108), b"SUTTUNGR");

    N2CCOD[109] = 624;
    fstr::assign(N2CNAM.get_mut(109), b"KIVIUQ");

    N2CCOD[110] = 625;
    fstr::assign(N2CNAM.get_mut(110), b"MUNDILFARI");

    N2CCOD[111] = 626;
    fstr::assign(N2CNAM.get_mut(111), b"ALBIORIX");

    N2CCOD[112] = 627;
    fstr::assign(N2CNAM.get_mut(112), b"SKATHI");

    N2CCOD[113] = 628;
    fstr::assign(N2CNAM.get_mut(113), b"ERRIAPUS");

    N2CCOD[114] = 629;
    fstr::assign(N2CNAM.get_mut(114), b"SIARNAQ");

    N2CCOD[115] = 630;
    fstr::assign(N2CNAM.get_mut(115), b"THRYMR");

    N2CCOD[116] = 631;
    fstr::assign(N2CNAM.get_mut(116), b"NARVI");

    N2CCOD[117] = 632;
    fstr::assign(N2CNAM.get_mut(117), b"METHONE");

    N2CCOD[118] = 633;
    fstr::assign(N2CNAM.get_mut(118), b"PALLENE");

    N2CCOD[119] = 634;
    fstr::assign(N2CNAM.get_mut(119), b"POLYDEUCES");

    N2CCOD[120] = 635;
    fstr::assign(N2CNAM.get_mut(120), b"DAPHNIS");

    N2CCOD[121] = 636;
    fstr::assign(N2CNAM.get_mut(121), b"AEGIR");

    N2CCOD[122] = 637;
    fstr::assign(N2CNAM.get_mut(122), b"BEBHIONN");

    N2CCOD[123] = 638;
    fstr::assign(N2CNAM.get_mut(123), b"BERGELMIR");

    N2CCOD[124] = 639;
    fstr::assign(N2CNAM.get_mut(124), b"BESTLA");

    N2CCOD[125] = 640;
    fstr::assign(N2CNAM.get_mut(125), b"FARBAUTI");

    N2CCOD[126] = 641;
    fstr::assign(N2CNAM.get_mut(126), b"FENRIR");

    N2CCOD[127] = 642;
    fstr::assign(N2CNAM.get_mut(127), b"FORNJOT");

    N2CCOD[128] = 643;
    fstr::assign(N2CNAM.get_mut(128), b"HATI");

    N2CCOD[129] = 644;
    fstr::assign(N2CNAM.get_mut(129), b"HYRROKKIN");

    N2CCOD[130] = 645;
    fstr::assign(N2CNAM.get_mut(130), b"KARI");

    N2CCOD[131] = 646;
    fstr::assign(N2CNAM.get_mut(131), b"LOGE");

    N2CCOD[132] = 647;
    fstr::assign(N2CNAM.get_mut(132), b"SKOLL");

    N2CCOD[133] = 648;
    fstr::assign(N2CNAM.get_mut(133), b"SURTUR");

    N2CCOD[134] = 649;
    fstr::assign(N2CNAM.get_mut(134), b"ANTHE");

    N2CCOD[135] = 650;
    fstr::assign(N2CNAM.get_mut(135), b"JARNSAXA");

    N2CCOD[136] = 651;
    fstr::assign(N2CNAM.get_mut(136), b"GREIP");

    N2CCOD[137] = 652;
    fstr::assign(N2CNAM.get_mut(137), b"TARQEQ");

    N2CCOD[138] = 653;
    fstr::assign(N2CNAM.get_mut(138), b"AEGAEON");

    N2CCOD[139] = 799;
    fstr::assign(N2CNAM.get_mut(139), b"URANUS");

    N2CCOD[140] = 701;
    fstr::assign(N2CNAM.get_mut(140), b"ARIEL");

    N2CCOD[141] = 702;
    fstr::assign(N2CNAM.get_mut(141), b"UMBRIEL");

    N2CCOD[142] = 703;
    fstr::assign(N2CNAM.get_mut(142), b"TITANIA");

    N2CCOD[143] = 704;
    fstr::assign(N2CNAM.get_mut(143), b"OBERON");

    N2CCOD[144] = 705;
    fstr::assign(N2CNAM.get_mut(144), b"MIRANDA");

    N2CCOD[145] = 706;
    fstr::assign(N2CNAM.get_mut(145), b"CORDELIA");

    N2CCOD[146] = 707;
    fstr::assign(N2CNAM.get_mut(146), b"OPHELIA");

    N2CCOD[147] = 708;
    fstr::assign(N2CNAM.get_mut(147), b"BIANCA");

    N2CCOD[148] = 709;
    fstr::assign(N2CNAM.get_mut(148), b"CRESSIDA");

    N2CCOD[149] = 710;
    fstr::assign(N2CNAM.get_mut(149), b"DESDEMONA");

    N2CCOD[150] = 711;
    fstr::assign(N2CNAM.get_mut(150), b"JULIET");

    N2CCOD[151] = 712;
    fstr::assign(N2CNAM.get_mut(151), b"PORTIA");

    N2CCOD[152] = 713;
    fstr::assign(N2CNAM.get_mut(152), b"ROSALIND");

    N2CCOD[153] = 714;
    fstr::assign(N2CNAM.get_mut(153), b"BELINDA");

    N2CCOD[154] = 715;
    fstr::assign(N2CNAM.get_mut(154), b"PUCK");

    N2CCOD[155] = 716;
    fstr::assign(N2CNAM.get_mut(155), b"CALIBAN");

    N2CCOD[156] = 717;
    fstr::assign(N2CNAM.get_mut(156), b"SYCORAX");

    N2CCOD[157] = 718;
    fstr::assign(N2CNAM.get_mut(157), b"PROSPERO");

    N2CCOD[158] = 719;
    fstr::assign(N2CNAM.get_mut(158), b"SETEBOS");

    N2CCOD[159] = 720;
    fstr::assign(N2CNAM.get_mut(159), b"STEPHANO");

    N2CCOD[160] = 721;
    fstr::assign(N2CNAM.get_mut(160), b"TRINCULO");

    N2CCOD[161] = 722;
    fstr::assign(N2CNAM.get_mut(161), b"FRANCISCO");

    N2CCOD[162] = 723;
    fstr::assign(N2CNAM.get_mut(162), b"MARGARET");

    N2CCOD[163] = 724;
    fstr::assign(N2CNAM.get_mut(163), b"FERDINAND");

    N2CCOD[164] = 725;
    fstr::assign(N2CNAM.get_mut(164), b"PERDITA");

    N2CCOD[165] = 726;
    fstr::assign(N2CNAM.get_mut(165), b"MAB");

    N2CCOD[166] = 727;
    fstr::assign(N2CNAM.get_mut(166), b"CUPID");

    N2CCOD[167] = 899;
    fstr::assign(N2CNAM.get_mut(167), b"NEPTUNE");

    N2CCOD[168] = 801;
    fstr::assign(N2CNAM.get_mut(168), b"TRITON");

    N2CCOD[169] = 802;
    fstr::assign(N2CNAM.get_mut(169), b"NEREID");

    N2CCOD[170] = 803;
    fstr::assign(N2CNAM.get_mut(170), b"NAIAD");

    N2CCOD[171] = 804;
    fstr::assign(N2CNAM.get_mut(171), b"THALASSA");

    N2CCOD[172] = 805;
    fstr::assign(N2CNAM.get_mut(172), b"DESPINA");

    N2CCOD[173] = 806;
    fstr::assign(N2CNAM.get_mut(173), b"GALATEA");

    N2CCOD[174] = 807;
    fstr::assign(N2CNAM.get_mut(174), b"LARISSA");

    N2CCOD[175] = 808;
    fstr::assign(N2CNAM.get_mut(175), b"PROTEUS");

    N2CCOD[176] = 809;
    fstr::assign(N2CNAM.get_mut(176), b"HALIMEDE");

    N2CCOD[177] = 810;
    fstr::assign(N2CNAM.get_mut(177), b"PSAMATHE");

    N2CCOD[178] = 811;
    fstr::assign(N2CNAM.get_mut(178), b"SAO");

    N2CCOD[179] = 812;
    fstr::assign(N2CNAM.get_mut(179), b"LAOMEDEIA");

    N2CCOD[180] = 813;
    fstr::assign(N2CNAM.get_mut(180), b"NESO");

    N2CCOD[181] = 999;
    fstr::assign(N2CNAM.get_mut(181), b"PLUTO");

    N2CCOD[182] = 901;
    fstr::assign(N2CNAM.get_mut(182), b"CHARON");

    N2CCOD[183] = 902;
    fstr::assign(N2CNAM.get_mut(183), b"NIX");

    N2CCOD[184] = 903;
    fstr::assign(N2CNAM.get_mut(184), b"HYDRA");

    N2CCOD[185] = 904;
    fstr::assign(N2CNAM.get_mut(185), b"KERBEROS");

    N2CCOD[186] = 905;
    fstr::assign(N2CNAM.get_mut(186), b"STYX");

    N2CCOD[187] = -1;
    fstr::assign(N2CNAM.get_mut(187), b"GEOTAIL");

    N2CCOD[188] = -3;
    fstr::assign(N2CNAM.get_mut(188), b"MOM");

    N2CCOD[189] = -3;
    fstr::assign(N2CNAM.get_mut(189), b"MARS ORBITER MISSION");

    N2CCOD[190] = -5;
    fstr::assign(N2CNAM.get_mut(190), b"AKATSUKI");

    N2CCOD[191] = -5;
    fstr::assign(N2CNAM.get_mut(191), b"VCO");

    N2CCOD[192] = -5;
    fstr::assign(N2CNAM.get_mut(192), b"PLC");

    N2CCOD[193] = -5;
    fstr::assign(N2CNAM.get_mut(193), b"PLANET-C");

    N2CCOD[194] = -6;
    fstr::assign(N2CNAM.get_mut(194), b"P6");

    N2CCOD[195] = -6;
    fstr::assign(N2CNAM.get_mut(195), b"PIONEER-6");

    N2CCOD[196] = -7;
    fstr::assign(N2CNAM.get_mut(196), b"P7");

    N2CCOD[197] = -7;
    fstr::assign(N2CNAM.get_mut(197), b"PIONEER-7");

    N2CCOD[198] = -8;
    fstr::assign(N2CNAM.get_mut(198), b"WIND");

    N2CCOD[199] = -12;
    fstr::assign(N2CNAM.get_mut(199), b"VENUS ORBITER");

    N2CCOD[200] = -12;
    fstr::assign(N2CNAM.get_mut(200), b"P12");

    N2CCOD[201] = -12;
    fstr::assign(N2CNAM.get_mut(201), b"PIONEER 12");

    N2CCOD[202] = -12;
    fstr::assign(N2CNAM.get_mut(202), b"LADEE");

    N2CCOD[203] = -13;
    fstr::assign(N2CNAM.get_mut(203), b"POLAR");

    N2CCOD[204] = -18;
    fstr::assign(N2CNAM.get_mut(204), b"MGN");

    N2CCOD[205] = -18;
    fstr::assign(N2CNAM.get_mut(205), b"MAGELLAN");

    N2CCOD[206] = -18;
    fstr::assign(N2CNAM.get_mut(206), b"LCROSS");

    N2CCOD[207] = -20;
    fstr::assign(N2CNAM.get_mut(207), b"P8");

    N2CCOD[208] = -20;
    fstr::assign(N2CNAM.get_mut(208), b"PIONEER-8");

    N2CCOD[209] = -21;
    fstr::assign(N2CNAM.get_mut(209), b"SOHO");

    N2CCOD[210] = -23;
    fstr::assign(N2CNAM.get_mut(210), b"P10");

    N2CCOD[211] = -23;
    fstr::assign(N2CNAM.get_mut(211), b"PIONEER-10");

    N2CCOD[212] = -24;
    fstr::assign(N2CNAM.get_mut(212), b"P11");

    N2CCOD[213] = -24;
    fstr::assign(N2CNAM.get_mut(213), b"PIONEER-11");

    N2CCOD[214] = -25;
    fstr::assign(N2CNAM.get_mut(214), b"LP");

    N2CCOD[215] = -25;
    fstr::assign(N2CNAM.get_mut(215), b"LUNAR PROSPECTOR");

    N2CCOD[216] = -27;
    fstr::assign(N2CNAM.get_mut(216), b"VK1");

    N2CCOD[217] = -27;
    fstr::assign(N2CNAM.get_mut(217), b"VIKING 1 ORBITER");

    N2CCOD[218] = -28;
    fstr::assign(N2CNAM.get_mut(218), b"JUPITER ICY MOONS EXPLORER");

    N2CCOD[219] = -28;
    fstr::assign(N2CNAM.get_mut(219), b"JUICE");

    N2CCOD[220] = -29;
    fstr::assign(N2CNAM.get_mut(220), b"STARDUST");

    N2CCOD[221] = -29;
    fstr::assign(N2CNAM.get_mut(221), b"SDU");

    N2CCOD[222] = -29;
    fstr::assign(N2CNAM.get_mut(222), b"NEXT");

    N2CCOD[223] = -30;
    fstr::assign(N2CNAM.get_mut(223), b"VK2");

    N2CCOD[224] = -30;
    fstr::assign(N2CNAM.get_mut(224), b"VIKING 2 ORBITER");

    N2CCOD[225] = -30;
    fstr::assign(N2CNAM.get_mut(225), b"DS-1");

    N2CCOD[226] = -31;
    fstr::assign(N2CNAM.get_mut(226), b"VG1");

    N2CCOD[227] = -31;
    fstr::assign(N2CNAM.get_mut(227), b"VOYAGER 1");

    N2CCOD[228] = -32;
    fstr::assign(N2CNAM.get_mut(228), b"VG2");

    N2CCOD[229] = -32;
    fstr::assign(N2CNAM.get_mut(229), b"VOYAGER 2");

    N2CCOD[230] = -33;
    fstr::assign(N2CNAM.get_mut(230), b"NEOS");

    N2CCOD[231] = -33;
    fstr::assign(N2CNAM.get_mut(231), b"NEO SURVEYOR");

    N2CCOD[232] = -37;
    fstr::assign(N2CNAM.get_mut(232), b"HYB2");

    N2CCOD[233] = -37;
    fstr::assign(N2CNAM.get_mut(233), b"HAYABUSA 2");

    N2CCOD[234] = -37;
    fstr::assign(N2CNAM.get_mut(234), b"HAYABUSA2");

    N2CCOD[235] = -39;
    fstr::assign(N2CNAM.get_mut(235), b"LUNAR POLAR HYDROGEN MAPPER");

    N2CCOD[236] = -39;
    fstr::assign(N2CNAM.get_mut(236), b"LUNAH-MAP");

    N2CCOD[237] = -40;
    fstr::assign(N2CNAM.get_mut(237), b"CLEMENTINE");

    N2CCOD[238] = -41;
    fstr::assign(N2CNAM.get_mut(238), b"MEX");

    N2CCOD[239] = -41;
    fstr::assign(N2CNAM.get_mut(239), b"MARS EXPRESS");

    N2CCOD[240] = -43;
    fstr::assign(N2CNAM.get_mut(240), b"IMAP");

    N2CCOD[241] = -44;
    fstr::assign(N2CNAM.get_mut(241), b"BEAGLE2");

    N2CCOD[242] = -44;
    fstr::assign(N2CNAM.get_mut(242), b"BEAGLE 2");

    N2CCOD[243] = -45;
    fstr::assign(N2CNAM.get_mut(243), b"JNSA");

    N2CCOD[244] = -45;
    fstr::assign(N2CNAM.get_mut(244), b"JANUS_A");

    N2CCOD[245] = -46;
    fstr::assign(N2CNAM.get_mut(245), b"MS-T5");

    N2CCOD[246] = -46;
    fstr::assign(N2CNAM.get_mut(246), b"SAKIGAKE");

    N2CCOD[247] = -47;
    fstr::assign(N2CNAM.get_mut(247), b"PLANET-A");

    N2CCOD[248] = -47;
    fstr::assign(N2CNAM.get_mut(248), b"SUISEI");

    N2CCOD[249] = -47;
    fstr::assign(N2CNAM.get_mut(249), b"GNS");

    N2CCOD[250] = -47;
    fstr::assign(N2CNAM.get_mut(250), b"GENESIS");

    N2CCOD[251] = -48;
    fstr::assign(N2CNAM.get_mut(251), b"HUBBLE SPACE TELESCOPE");

    N2CCOD[252] = -48;
    fstr::assign(N2CNAM.get_mut(252), b"HST");

    N2CCOD[253] = -49;
    fstr::assign(N2CNAM.get_mut(253), b"LUCY");

    N2CCOD[254] = -53;
    fstr::assign(N2CNAM.get_mut(254), b"MARS PATHFINDER");

    N2CCOD[255] = -53;
    fstr::assign(N2CNAM.get_mut(255), b"MPF");

    N2CCOD[256] = -53;
    fstr::assign(N2CNAM.get_mut(256), b"MARS ODYSSEY");

    N2CCOD[257] = -53;
    fstr::assign(N2CNAM.get_mut(257), b"MARS SURVEYOR 01 ORBITER");

    N2CCOD[258] = -55;
    fstr::assign(N2CNAM.get_mut(258), b"ULYSSES");

    N2CCOD[259] = -57;
    fstr::assign(N2CNAM.get_mut(259), b"LUNAR ICECUBE");

    N2CCOD[260] = -58;
    fstr::assign(N2CNAM.get_mut(260), b"VSOP");

    N2CCOD[261] = -58;
    fstr::assign(N2CNAM.get_mut(261), b"HALCA");

    N2CCOD[262] = -59;
    fstr::assign(N2CNAM.get_mut(262), b"RADIOASTRON");

    N2CCOD[263] = -61;
    fstr::assign(N2CNAM.get_mut(263), b"JUNO");

    N2CCOD[264] = -62;
    fstr::assign(N2CNAM.get_mut(264), b"EMM");

    N2CCOD[265] = -62;
    fstr::assign(N2CNAM.get_mut(265), b"EMIRATES MARS MISSION");

    N2CCOD[266] = -64;
    fstr::assign(N2CNAM.get_mut(266), b"ORX");

    N2CCOD[267] = -64;
    fstr::assign(N2CNAM.get_mut(267), b"OSIRIS-REX");

    N2CCOD[268] = -65;
    fstr::assign(N2CNAM.get_mut(268), b"MCOA");

    N2CCOD[269] = -65;
    fstr::assign(N2CNAM.get_mut(269), b"MARCO-A");

    N2CCOD[270] = -66;
    fstr::assign(N2CNAM.get_mut(270), b"VEGA 1");

    N2CCOD[271] = -66;
    fstr::assign(N2CNAM.get_mut(271), b"MCOB");

    N2CCOD[272] = -66;
    fstr::assign(N2CNAM.get_mut(272), b"MARCO-B");

    N2CCOD[273] = -67;
    fstr::assign(N2CNAM.get_mut(273), b"VEGA 2");

    N2CCOD[274] = -68;
    fstr::assign(N2CNAM.get_mut(274), b"MERCURY MAGNETOSPHERIC ORBITER");

    N2CCOD[275] = -68;
    fstr::assign(N2CNAM.get_mut(275), b"MMO");

    N2CCOD[276] = -68;
    fstr::assign(N2CNAM.get_mut(276), b"BEPICOLOMBO MMO");

    N2CCOD[277] = -70;
    fstr::assign(N2CNAM.get_mut(277), b"DEEP IMPACT IMPACTOR SPACECRAFT");

    N2CCOD[278] = -72;
    fstr::assign(N2CNAM.get_mut(278), b"JNSB");

    N2CCOD[279] = -72;
    fstr::assign(N2CNAM.get_mut(279), b"JANUS_B");

    N2CCOD[280] = -74;
    fstr::assign(N2CNAM.get_mut(280), b"MRO");

    N2CCOD[281] = -74;
    fstr::assign(N2CNAM.get_mut(281), b"MARS RECON ORBITER");

    N2CCOD[282] = -76;
    fstr::assign(N2CNAM.get_mut(282), b"CURIOSITY");

    N2CCOD[283] = -76;
    fstr::assign(N2CNAM.get_mut(283), b"MSL");

    N2CCOD[284] = -76;
    fstr::assign(N2CNAM.get_mut(284), b"MARS SCIENCE LABORATORY");

    N2CCOD[285] = -77;
    fstr::assign(N2CNAM.get_mut(285), b"GLL");

    N2CCOD[286] = -77;
    fstr::assign(N2CNAM.get_mut(286), b"GALILEO ORBITER");

    N2CCOD[287] = -78;
    fstr::assign(N2CNAM.get_mut(287), b"GIOTTO");

    N2CCOD[288] = -79;
    fstr::assign(N2CNAM.get_mut(288), b"SPITZER");

    N2CCOD[289] = -79;
    fstr::assign(N2CNAM.get_mut(289), b"SPACE INFRARED TELESCOPE FACILITY");

    N2CCOD[290] = -79;
    fstr::assign(N2CNAM.get_mut(290), b"SIRTF");

    N2CCOD[291] = -81;
    fstr::assign(N2CNAM.get_mut(291), b"CASSINI ITL");

    N2CCOD[292] = -82;
    fstr::assign(N2CNAM.get_mut(292), b"CAS");

    N2CCOD[293] = -82;
    fstr::assign(N2CNAM.get_mut(293), b"CASSINI");

    N2CCOD[294] = -84;
    fstr::assign(N2CNAM.get_mut(294), b"PHOENIX");

    N2CCOD[295] = -85;
    fstr::assign(N2CNAM.get_mut(295), b"LRO");

    N2CCOD[296] = -85;
    fstr::assign(N2CNAM.get_mut(296), b"LUNAR RECON ORBITER");

    N2CCOD[297] = -85;
    fstr::assign(N2CNAM.get_mut(297), b"LUNAR RECONNAISSANCE ORBITER");

    N2CCOD[298] = -86;
    fstr::assign(N2CNAM.get_mut(298), b"CH1");

    N2CCOD[299] = -86;
    fstr::assign(N2CNAM.get_mut(299), b"CHANDRAYAAN-1");

    N2CCOD[300] = -90;
    fstr::assign(N2CNAM.get_mut(300), b"CASSINI SIMULATION");

    N2CCOD[301] = -93;
    fstr::assign(N2CNAM.get_mut(301), b"NEAR EARTH ASTEROID RENDEZVOUS");

    N2CCOD[302] = -93;
    fstr::assign(N2CNAM.get_mut(302), b"NEAR");

    N2CCOD[303] = -94;
    fstr::assign(N2CNAM.get_mut(303), b"MO");

    N2CCOD[304] = -94;
    fstr::assign(N2CNAM.get_mut(304), b"MARS OBSERVER");

    N2CCOD[305] = -94;
    fstr::assign(N2CNAM.get_mut(305), b"MGS");

    N2CCOD[306] = -94;
    fstr::assign(N2CNAM.get_mut(306), b"MARS GLOBAL SURVEYOR");

    N2CCOD[307] = -95;
    fstr::assign(N2CNAM.get_mut(307), b"MGS SIMULATION");

    N2CCOD[308] = -96;
    fstr::assign(N2CNAM.get_mut(308), b"PARKER SOLAR PROBE");

    N2CCOD[309] = -96;
    fstr::assign(N2CNAM.get_mut(309), b"SPP");

    N2CCOD[310] = -96;
    fstr::assign(N2CNAM.get_mut(310), b"SOLAR PROBE PLUS");

    N2CCOD[311] = -97;
    fstr::assign(N2CNAM.get_mut(311), b"TOPEX/POSEIDON");

    N2CCOD[312] = -98;
    fstr::assign(N2CNAM.get_mut(312), b"NEW HORIZONS");

    N2CCOD[313] = -107;
    fstr::assign(N2CNAM.get_mut(313), b"TROPICAL RAINFALL MEASURING MISSION");

    N2CCOD[314] = -107;
    fstr::assign(N2CNAM.get_mut(314), b"TRMM");

    N2CCOD[315] = -112;
    fstr::assign(N2CNAM.get_mut(315), b"ICE");

    N2CCOD[316] = -116;
    fstr::assign(N2CNAM.get_mut(316), b"MARS POLAR LANDER");

    N2CCOD[317] = -116;
    fstr::assign(N2CNAM.get_mut(317), b"MPL");

    N2CCOD[318] = -117;
    fstr::assign(N2CNAM.get_mut(318), b"EDL DEMONSTRATOR MODULE");

    N2CCOD[319] = -117;
    fstr::assign(N2CNAM.get_mut(319), b"EDM");

    N2CCOD[320] = -117;
    fstr::assign(N2CNAM.get_mut(320), b"EXOMARS 2016 EDM");

    N2CCOD[321] = -119;
    fstr::assign(N2CNAM.get_mut(321), b"MARS_ORBITER_MISSION_2");

    N2CCOD[322] = -119;
    fstr::assign(N2CNAM.get_mut(322), b"MOM2");

    N2CCOD[323] = -121;
    fstr::assign(N2CNAM.get_mut(323), b"MERCURY PLANETARY ORBITER");

    N2CCOD[324] = -121;
    fstr::assign(N2CNAM.get_mut(324), b"MPO");

    N2CCOD[325] = -121;
    fstr::assign(N2CNAM.get_mut(325), b"BEPICOLOMBO MPO");

    N2CCOD[326] = -127;
    fstr::assign(N2CNAM.get_mut(326), b"MARS CLIMATE ORBITER");

    N2CCOD[327] = -127;
    fstr::assign(N2CNAM.get_mut(327), b"MCO");

    N2CCOD[328] = -130;
    fstr::assign(N2CNAM.get_mut(328), b"MUSES-C");

    N2CCOD[329] = -130;
    fstr::assign(N2CNAM.get_mut(329), b"HAYABUSA");

    N2CCOD[330] = -131;
    fstr::assign(N2CNAM.get_mut(330), b"SELENE");

    N2CCOD[331] = -131;
    fstr::assign(N2CNAM.get_mut(331), b"KAGUYA");

    N2CCOD[332] = -135;
    fstr::assign(N2CNAM.get_mut(332), b"DART");

    N2CCOD[333] = -135;
    fstr::assign(N2CNAM.get_mut(333), b"DOUBLE ASTEROID REDIRECTION TEST");

    N2CCOD[334] = -140;
    fstr::assign(N2CNAM.get_mut(334), b"EPOCH");

    N2CCOD[335] = -140;
    fstr::assign(N2CNAM.get_mut(335), b"DIXI");

    N2CCOD[336] = -140;
    fstr::assign(N2CNAM.get_mut(336), b"EPOXI");

    N2CCOD[337] = -140;
    fstr::assign(N2CNAM.get_mut(337), b"DEEP IMPACT FLYBY SPACECRAFT");

    N2CCOD[338] = -142;
    fstr::assign(N2CNAM.get_mut(338), b"TERRA");

    N2CCOD[339] = -142;
    fstr::assign(N2CNAM.get_mut(339), b"EOS-AM1");

    N2CCOD[340] = -143;
    fstr::assign(N2CNAM.get_mut(340), b"TRACE GAS ORBITER");

    N2CCOD[341] = -143;
    fstr::assign(N2CNAM.get_mut(341), b"TGO");

    N2CCOD[342] = -143;
    fstr::assign(N2CNAM.get_mut(342), b"EXOMARS 2016 TGO");

    N2CCOD[343] = -144;
    fstr::assign(N2CNAM.get_mut(343), b"SOLO");

    N2CCOD[344] = -144;
    fstr::assign(N2CNAM.get_mut(344), b"SOLAR ORBITER");

    N2CCOD[345] = -146;
    fstr::assign(N2CNAM.get_mut(345), b"LUNAR-A");

    N2CCOD[346] = -148;
    fstr::assign(N2CNAM.get_mut(346), b"DFLY");

    N2CCOD[347] = -148;
    fstr::assign(N2CNAM.get_mut(347), b"DRAGONFLY");

    N2CCOD[348] = -150;
    fstr::assign(N2CNAM.get_mut(348), b"CASSINI PROBE");

    N2CCOD[349] = -150;
    fstr::assign(N2CNAM.get_mut(349), b"HUYGENS PROBE");

    N2CCOD[350] = -150;
    fstr::assign(N2CNAM.get_mut(350), b"CASP");

    N2CCOD[351] = -151;
    fstr::assign(N2CNAM.get_mut(351), b"AXAF");

    N2CCOD[352] = -151;
    fstr::assign(N2CNAM.get_mut(352), b"CHANDRA");

    N2CCOD[353] = -152;
    fstr::assign(N2CNAM.get_mut(353), b"CH2O");

    N2CCOD[354] = -152;
    fstr::assign(N2CNAM.get_mut(354), b"CHANDRAYAAN-2 ORBITER");

    N2CCOD[355] = -153;
    fstr::assign(N2CNAM.get_mut(355), b"CH2L");

    N2CCOD[356] = -153;
    fstr::assign(N2CNAM.get_mut(356), b"CHANDRAYAAN-2 LANDER");

    N2CCOD[357] = -154;
    fstr::assign(N2CNAM.get_mut(357), b"AQUA");

    N2CCOD[358] = -155;
    fstr::assign(N2CNAM.get_mut(358), b"KPLO");

    N2CCOD[359] = -155;
    fstr::assign(N2CNAM.get_mut(359), b"KOREAN PATHFINDER LUNAR ORBITER");

    N2CCOD[360] = -156;
    fstr::assign(N2CNAM.get_mut(360), b"ADITYA");

    N2CCOD[361] = -156;
    fstr::assign(N2CNAM.get_mut(361), b"ADIT");

    N2CCOD[362] = -159;
    fstr::assign(N2CNAM.get_mut(362), b"EURC");

    N2CCOD[363] = -159;
    fstr::assign(N2CNAM.get_mut(363), b"EUROPA CLIPPER");

    N2CCOD[364] = -164;
    fstr::assign(N2CNAM.get_mut(364), b"LUNAR FLASHLIGHT");

    N2CCOD[365] = -165;
    fstr::assign(N2CNAM.get_mut(365), b"MAP");

    N2CCOD[366] = -166;
    fstr::assign(N2CNAM.get_mut(366), b"IMAGE");

    N2CCOD[367] = -168;
    fstr::assign(N2CNAM.get_mut(367), b"PERSEVERANCE");

    N2CCOD[368] = -168;
    fstr::assign(N2CNAM.get_mut(368), b"MARS 2020");

    N2CCOD[369] = -168;
    fstr::assign(N2CNAM.get_mut(369), b"MARS2020");

    N2CCOD[370] = -168;
    fstr::assign(N2CNAM.get_mut(370), b"M2020");

    N2CCOD[371] = -170;
    fstr::assign(N2CNAM.get_mut(371), b"JWST");

    N2CCOD[372] = -170;
    fstr::assign(N2CNAM.get_mut(372), b"JAMES WEBB SPACE TELESCOPE");

    N2CCOD[373] = -172;
    fstr::assign(N2CNAM.get_mut(373), b"EXM RSP SCC");

    N2CCOD[374] = -172;
    fstr::assign(N2CNAM.get_mut(374), b"EXM SPACECRAFT COMPOSITE");

    N2CCOD[375] = -172;
    fstr::assign(N2CNAM.get_mut(375), b"EXOMARS SCC");

    N2CCOD[376] = -173;
    fstr::assign(N2CNAM.get_mut(376), b"EXM RSP SP");

    N2CCOD[377] = -173;
    fstr::assign(N2CNAM.get_mut(377), b"EXM SURFACE PLATFORM");

    N2CCOD[378] = -173;
    fstr::assign(N2CNAM.get_mut(378), b"EXOMARS SP");

    N2CCOD[379] = -174;
    fstr::assign(N2CNAM.get_mut(379), b"EXM RSP RM");

    N2CCOD[380] = -174;
    fstr::assign(N2CNAM.get_mut(380), b"EXM ROVER");

    N2CCOD[381] = -174;
    fstr::assign(N2CNAM.get_mut(381), b"EXOMARS ROVER");

    N2CCOD[382] = -177;
    fstr::assign(N2CNAM.get_mut(382), b"GRAIL-A");

    N2CCOD[383] = -178;
    fstr::assign(N2CNAM.get_mut(383), b"PLANET-B");

    N2CCOD[384] = -178;
    fstr::assign(N2CNAM.get_mut(384), b"NOZOMI");

    N2CCOD[385] = -181;
    fstr::assign(N2CNAM.get_mut(385), b"GRAIL-B");

    N2CCOD[386] = -183;
    fstr::assign(N2CNAM.get_mut(386), b"CLUSTER 1");

    N2CCOD[387] = -185;
    fstr::assign(N2CNAM.get_mut(387), b"CLUSTER 2");

    N2CCOD[388] = -188;
    fstr::assign(N2CNAM.get_mut(388), b"MUSES-B");

    N2CCOD[389] = -189;
    fstr::assign(N2CNAM.get_mut(389), b"NSYT");

    N2CCOD[390] = -189;
    fstr::assign(N2CNAM.get_mut(390), b"INSIGHT");

    N2CCOD[391] = -190;
    fstr::assign(N2CNAM.get_mut(391), b"SIM");

    N2CCOD[392] = -194;
    fstr::assign(N2CNAM.get_mut(392), b"CLUSTER 3");

    N2CCOD[393] = -196;
    fstr::assign(N2CNAM.get_mut(393), b"CLUSTER 4");

    N2CCOD[394] = -197;
    fstr::assign(N2CNAM.get_mut(394), b"EXOMARS_LARA");

    N2CCOD[395] = -197;
    fstr::assign(N2CNAM.get_mut(395), b"LARA");

    N2CCOD[396] = -198;
    fstr::assign(N2CNAM.get_mut(396), b"INTEGRAL");

    N2CCOD[397] = -198;
    fstr::assign(N2CNAM.get_mut(397), b"NASA-ISRO SAR MISSION");

    N2CCOD[398] = -198;
    fstr::assign(N2CNAM.get_mut(398), b"NISAR");

    N2CCOD[399] = -200;
    fstr::assign(N2CNAM.get_mut(399), b"CONTOUR");

    N2CCOD[400] = -202;
    fstr::assign(N2CNAM.get_mut(400), b"MAVEN");

    N2CCOD[401] = -203;
    fstr::assign(N2CNAM.get_mut(401), b"DAWN");

    N2CCOD[402] = -205;
    fstr::assign(N2CNAM.get_mut(402), b"SOIL MOISTURE ACTIVE AND PASSIVE");

    N2CCOD[403] = -205;
    fstr::assign(N2CNAM.get_mut(403), b"SMAP");

    N2CCOD[404] = -210;
    fstr::assign(N2CNAM.get_mut(404), b"LICIA");

    N2CCOD[405] = -210;
    fstr::assign(N2CNAM.get_mut(405), b"LICIACUBE");

    N2CCOD[406] = -212;
    fstr::assign(N2CNAM.get_mut(406), b"STV51");

    N2CCOD[407] = -213;
    fstr::assign(N2CNAM.get_mut(407), b"STV52");

    N2CCOD[408] = -214;
    fstr::assign(N2CNAM.get_mut(408), b"STV53");

    N2CCOD[409] = -226;
    fstr::assign(N2CNAM.get_mut(409), b"ROSETTA");

    N2CCOD[410] = -227;
    fstr::assign(N2CNAM.get_mut(410), b"KEPLER");

    N2CCOD[411] = -228;
    fstr::assign(N2CNAM.get_mut(411), b"GLL PROBE");

    N2CCOD[412] = -228;
    fstr::assign(N2CNAM.get_mut(412), b"GALILEO PROBE");

    N2CCOD[413] = -234;
    fstr::assign(N2CNAM.get_mut(413), b"STEREO AHEAD");

    N2CCOD[414] = -235;
    fstr::assign(N2CNAM.get_mut(414), b"STEREO BEHIND");

    N2CCOD[415] = -236;
    fstr::assign(N2CNAM.get_mut(415), b"MESSENGER");

    N2CCOD[416] = -238;
    fstr::assign(N2CNAM.get_mut(416), b"SMART1");

    N2CCOD[417] = -238;
    fstr::assign(N2CNAM.get_mut(417), b"SM1");

    N2CCOD[418] = -238;
    fstr::assign(N2CNAM.get_mut(418), b"S1");

    N2CCOD[419] = -238;
    fstr::assign(N2CNAM.get_mut(419), b"SMART-1");

    N2CCOD[420] = -239;
    fstr::assign(N2CNAM.get_mut(420), b"MARTIAN MOONS EXPLORATION");

    N2CCOD[421] = -239;
    fstr::assign(N2CNAM.get_mut(421), b"MMX");

    N2CCOD[422] = -240;
    fstr::assign(N2CNAM.get_mut(422), b"SMART LANDER FOR INVESTIGATING MOON");

    N2CCOD[423] = -240;
    fstr::assign(N2CNAM.get_mut(423), b"SLIM");

    N2CCOD[424] = -242;
    fstr::assign(N2CNAM.get_mut(424), b"LUNAR TRAILBLAZER");

    N2CCOD[425] = -243;
    fstr::assign(N2CNAM.get_mut(425), b"VIPER");

    N2CCOD[426] = -248;
    fstr::assign(N2CNAM.get_mut(426), b"VEX");

    N2CCOD[427] = -248;
    fstr::assign(N2CNAM.get_mut(427), b"VENUS EXPRESS");

    N2CCOD[428] = -253;
    fstr::assign(N2CNAM.get_mut(428), b"OPPORTUNITY");

    N2CCOD[429] = -253;
    fstr::assign(N2CNAM.get_mut(429), b"MER-1");

    N2CCOD[430] = -254;
    fstr::assign(N2CNAM.get_mut(430), b"SPIRIT");

    N2CCOD[431] = -254;
    fstr::assign(N2CNAM.get_mut(431), b"MER-2");

    N2CCOD[432] = -255;
    fstr::assign(N2CNAM.get_mut(432), b"PSYC");

    N2CCOD[433] = -301;
    fstr::assign(N2CNAM.get_mut(433), b"HELIOS 1");

    N2CCOD[434] = -302;
    fstr::assign(N2CNAM.get_mut(434), b"HELIOS 2");

    N2CCOD[435] = -362;
    fstr::assign(N2CNAM.get_mut(435), b"RADIATION BELT STORM PROBE A");

    N2CCOD[436] = -362;
    fstr::assign(N2CNAM.get_mut(436), b"RBSP_A");

    N2CCOD[437] = -363;
    fstr::assign(N2CNAM.get_mut(437), b"RADIATION BELT STORM PROBE B");

    N2CCOD[438] = -363;
    fstr::assign(N2CNAM.get_mut(438), b"RBSP_B");

    N2CCOD[439] = -500;
    fstr::assign(N2CNAM.get_mut(439), b"RSAT");

    N2CCOD[440] = -500;
    fstr::assign(N2CNAM.get_mut(440), b"SELENE Relay Satellite");

    N2CCOD[441] = -500;
    fstr::assign(N2CNAM.get_mut(441), b"SELENE Rstar");

    N2CCOD[442] = -500;
    fstr::assign(N2CNAM.get_mut(442), b"Rstar");

    N2CCOD[443] = -502;
    fstr::assign(N2CNAM.get_mut(443), b"VSAT");

    N2CCOD[444] = -502;
    fstr::assign(N2CNAM.get_mut(444), b"SELENE VLBI Radio Satellite");

    N2CCOD[445] = -502;
    fstr::assign(N2CNAM.get_mut(445), b"SELENE VRAD Satellite");

    N2CCOD[446] = -502;
    fstr::assign(N2CNAM.get_mut(446), b"SELENE Vstar");

    N2CCOD[447] = -502;
    fstr::assign(N2CNAM.get_mut(447), b"Vstar");

    N2CCOD[448] = -550;
    fstr::assign(N2CNAM.get_mut(448), b"MARS-96");

    N2CCOD[449] = -550;
    fstr::assign(N2CNAM.get_mut(449), b"M96");

    N2CCOD[450] = -550;
    fstr::assign(N2CNAM.get_mut(450), b"MARS 96");

    N2CCOD[451] = -550;
    fstr::assign(N2CNAM.get_mut(451), b"MARS96");

    N2CCOD[452] = -652;
    fstr::assign(N2CNAM.get_mut(452), b"MERCURY TRANSFER MODULE");

    N2CCOD[453] = -652;
    fstr::assign(N2CNAM.get_mut(453), b"MTM");

    N2CCOD[454] = -652;
    fstr::assign(N2CNAM.get_mut(454), b"BEPICOLOMBO MTM");

    N2CCOD[455] = -750;
    fstr::assign(N2CNAM.get_mut(455), b"SPRINT-A");

    N2CCOD[456] = 50000001;
    fstr::assign(N2CNAM.get_mut(456), b"SHOEMAKER-LEVY 9-W");

    N2CCOD[457] = 50000002;
    fstr::assign(N2CNAM.get_mut(457), b"SHOEMAKER-LEVY 9-V");

    N2CCOD[458] = 50000003;
    fstr::assign(N2CNAM.get_mut(458), b"SHOEMAKER-LEVY 9-U");

    N2CCOD[459] = 50000004;
    fstr::assign(N2CNAM.get_mut(459), b"SHOEMAKER-LEVY 9-T");

    N2CCOD[460] = 50000005;
    fstr::assign(N2CNAM.get_mut(460), b"SHOEMAKER-LEVY 9-S");

    N2CCOD[461] = 50000006;
    fstr::assign(N2CNAM.get_mut(461), b"SHOEMAKER-LEVY 9-R");

    N2CCOD[462] = 50000007;
    fstr::assign(N2CNAM.get_mut(462), b"SHOEMAKER-LEVY 9-Q");

    N2CCOD[463] = 50000008;
    fstr::assign(N2CNAM.get_mut(463), b"SHOEMAKER-LEVY 9-P");

    N2CCOD[464] = 50000009;
    fstr::assign(N2CNAM.get_mut(464), b"SHOEMAKER-LEVY 9-N");

    N2CCOD[465] = 50000010;
    fstr::assign(N2CNAM.get_mut(465), b"SHOEMAKER-LEVY 9-M");

    N2CCOD[466] = 50000011;
    fstr::assign(N2CNAM.get_mut(466), b"SHOEMAKER-LEVY 9-L");

    N2CCOD[467] = 50000012;
    fstr::assign(N2CNAM.get_mut(467), b"SHOEMAKER-LEVY 9-K");

    N2CCOD[468] = 50000013;
    fstr::assign(N2CNAM.get_mut(468), b"SHOEMAKER-LEVY 9-J");

    N2CCOD[469] = 50000014;
    fstr::assign(N2CNAM.get_mut(469), b"SHOEMAKER-LEVY 9-H");

    N2CCOD[470] = 50000015;
    fstr::assign(N2CNAM.get_mut(470), b"SHOEMAKER-LEVY 9-G");

    N2CCOD[471] = 50000016;
    fstr::assign(N2CNAM.get_mut(471), b"SHOEMAKER-LEVY 9-F");

    N2CCOD[472] = 50000017;
    fstr::assign(N2CNAM.get_mut(472), b"SHOEMAKER-LEVY 9-E");

    N2CCOD[473] = 50000018;
    fstr::assign(N2CNAM.get_mut(473), b"SHOEMAKER-LEVY 9-D");

    N2CCOD[474] = 50000019;
    fstr::assign(N2CNAM.get_mut(474), b"SHOEMAKER-LEVY 9-C");

    N2CCOD[475] = 50000020;
    fstr::assign(N2CNAM.get_mut(475), b"SHOEMAKER-LEVY 9-B");

    N2CCOD[476] = 50000021;
    fstr::assign(N2CNAM.get_mut(476), b"SHOEMAKER-LEVY 9-A");

    N2CCOD[477] = 50000022;
    fstr::assign(N2CNAM.get_mut(477), b"SHOEMAKER-LEVY 9-Q1");

    N2CCOD[478] = 50000023;
    fstr::assign(N2CNAM.get_mut(478), b"SHOEMAKER-LEVY 9-P2");

    N2CCOD[479] = 1000001;
    fstr::assign(N2CNAM.get_mut(479), b"AREND");

    N2CCOD[480] = 1000002;
    fstr::assign(N2CNAM.get_mut(480), b"AREND-RIGAUX");

    N2CCOD[481] = 1000003;
    fstr::assign(N2CNAM.get_mut(481), b"ASHBROOK-JACKSON");

    N2CCOD[482] = 1000004;
    fstr::assign(N2CNAM.get_mut(482), b"BOETHIN");

    N2CCOD[483] = 1000005;
    fstr::assign(N2CNAM.get_mut(483), b"BORRELLY");

    N2CCOD[484] = 1000006;
    fstr::assign(N2CNAM.get_mut(484), b"BOWELL-SKIFF");

    N2CCOD[485] = 1000007;
    fstr::assign(N2CNAM.get_mut(485), b"BRADFIELD");

    N2CCOD[486] = 1000008;
    fstr::assign(N2CNAM.get_mut(486), b"BROOKS 2");

    N2CCOD[487] = 1000009;
    fstr::assign(N2CNAM.get_mut(487), b"BRORSEN-METCALF");

    N2CCOD[488] = 1000010;
    fstr::assign(N2CNAM.get_mut(488), b"BUS");

    N2CCOD[489] = 1000011;
    fstr::assign(N2CNAM.get_mut(489), b"CHERNYKH");

    N2CCOD[490] = 1000012;
    fstr::assign(N2CNAM.get_mut(490), b"67P/CHURYUMOV-GERASIMENKO (1969 R1)");

    N2CCOD[491] = 1000012;
    fstr::assign(N2CNAM.get_mut(491), b"CHURYUMOV-GERASIMENKO");

    N2CCOD[492] = 1000013;
    fstr::assign(N2CNAM.get_mut(492), b"CIFFREO");

    N2CCOD[493] = 1000014;
    fstr::assign(N2CNAM.get_mut(493), b"CLARK");

    N2CCOD[494] = 1000015;
    fstr::assign(N2CNAM.get_mut(494), b"COMAS SOLA");

    N2CCOD[495] = 1000016;
    fstr::assign(N2CNAM.get_mut(495), b"CROMMELIN");

    N2CCOD[496] = 1000017;
    fstr::assign(N2CNAM.get_mut(496), b"D\'ARREST");

    N2CCOD[497] = 1000018;
    fstr::assign(N2CNAM.get_mut(497), b"DANIEL");

    N2CCOD[498] = 1000019;
    fstr::assign(N2CNAM.get_mut(498), b"DE VICO-SWIFT");

    N2CCOD[499] = 1000020;
    fstr::assign(N2CNAM.get_mut(499), b"DENNING-FUJIKAWA");

    N2CCOD[500] = 1000021;
    fstr::assign(N2CNAM.get_mut(500), b"DU TOIT 1");

    N2CCOD[501] = 1000022;
    fstr::assign(N2CNAM.get_mut(501), b"DU TOIT-HARTLEY");

    N2CCOD[502] = 1000023;
    fstr::assign(N2CNAM.get_mut(502), b"DUTOIT-NEUJMIN-DELPORTE");

    N2CCOD[503] = 1000024;
    fstr::assign(N2CNAM.get_mut(503), b"DUBIAGO");

    N2CCOD[504] = 1000025;
    fstr::assign(N2CNAM.get_mut(504), b"ENCKE");

    N2CCOD[505] = 1000026;
    fstr::assign(N2CNAM.get_mut(505), b"FAYE");

    N2CCOD[506] = 1000027;
    fstr::assign(N2CNAM.get_mut(506), b"FINLAY");

    N2CCOD[507] = 1000028;
    fstr::assign(N2CNAM.get_mut(507), b"FORBES");

    N2CCOD[508] = 1000029;
    fstr::assign(N2CNAM.get_mut(508), b"GEHRELS 1");

    N2CCOD[509] = 1000030;
    fstr::assign(N2CNAM.get_mut(509), b"GEHRELS 2");

    N2CCOD[510] = 1000031;
    fstr::assign(N2CNAM.get_mut(510), b"GEHRELS 3");

    N2CCOD[511] = 1000032;
    fstr::assign(N2CNAM.get_mut(511), b"GIACOBINI-ZINNER");

    N2CCOD[512] = 1000033;
    fstr::assign(N2CNAM.get_mut(512), b"GICLAS");

    N2CCOD[513] = 1000034;
    fstr::assign(N2CNAM.get_mut(513), b"GRIGG-SKJELLERUP");

    N2CCOD[514] = 1000035;
    fstr::assign(N2CNAM.get_mut(514), b"GUNN");

    N2CCOD[515] = 1000036;
    fstr::assign(N2CNAM.get_mut(515), b"HALLEY");

    N2CCOD[516] = 1000037;
    fstr::assign(N2CNAM.get_mut(516), b"HANEDA-CAMPOS");

    N2CCOD[517] = 1000038;
    fstr::assign(N2CNAM.get_mut(517), b"HARRINGTON");

    N2CCOD[518] = 1000039;
    fstr::assign(N2CNAM.get_mut(518), b"HARRINGTON-ABELL");

    N2CCOD[519] = 1000040;
    fstr::assign(N2CNAM.get_mut(519), b"HARTLEY 1");

    N2CCOD[520] = 1000041;
    fstr::assign(N2CNAM.get_mut(520), b"HARTLEY 2");

    N2CCOD[521] = 1000042;
    fstr::assign(N2CNAM.get_mut(521), b"HARTLEY-IRAS");

    N2CCOD[522] = 1000043;
    fstr::assign(N2CNAM.get_mut(522), b"HERSCHEL-RIGOLLET");

    N2CCOD[523] = 1000044;
    fstr::assign(N2CNAM.get_mut(523), b"HOLMES");

    N2CCOD[524] = 1000045;
    fstr::assign(N2CNAM.get_mut(524), b"HONDA-MRKOS-PAJDUSAKOVA");

    N2CCOD[525] = 1000046;
    fstr::assign(N2CNAM.get_mut(525), b"HOWELL");

    N2CCOD[526] = 1000047;
    fstr::assign(N2CNAM.get_mut(526), b"IRAS");

    N2CCOD[527] = 1000048;
    fstr::assign(N2CNAM.get_mut(527), b"JACKSON-NEUJMIN");

    N2CCOD[528] = 1000049;
    fstr::assign(N2CNAM.get_mut(528), b"JOHNSON");

    N2CCOD[529] = 1000050;
    fstr::assign(N2CNAM.get_mut(529), b"KEARNS-KWEE");

    N2CCOD[530] = 1000051;
    fstr::assign(N2CNAM.get_mut(530), b"KLEMOLA");

    N2CCOD[531] = 1000052;
    fstr::assign(N2CNAM.get_mut(531), b"KOHOUTEK");

    N2CCOD[532] = 1000053;
    fstr::assign(N2CNAM.get_mut(532), b"KOJIMA");

    N2CCOD[533] = 1000054;
    fstr::assign(N2CNAM.get_mut(533), b"KOPFF");

    N2CCOD[534] = 1000055;
    fstr::assign(N2CNAM.get_mut(534), b"KOWAL 1");

    N2CCOD[535] = 1000056;
    fstr::assign(N2CNAM.get_mut(535), b"KOWAL 2");

    N2CCOD[536] = 1000057;
    fstr::assign(N2CNAM.get_mut(536), b"KOWAL-MRKOS");

    N2CCOD[537] = 1000058;
    fstr::assign(N2CNAM.get_mut(537), b"KOWAL-VAVROVA");

    N2CCOD[538] = 1000059;
    fstr::assign(N2CNAM.get_mut(538), b"LONGMORE");

    N2CCOD[539] = 1000060;
    fstr::assign(N2CNAM.get_mut(539), b"LOVAS 1");

    N2CCOD[540] = 1000061;
    fstr::assign(N2CNAM.get_mut(540), b"MACHHOLZ");

    N2CCOD[541] = 1000062;
    fstr::assign(N2CNAM.get_mut(541), b"MAURY");

    N2CCOD[542] = 1000063;
    fstr::assign(N2CNAM.get_mut(542), b"NEUJMIN 1");

    N2CCOD[543] = 1000064;
    fstr::assign(N2CNAM.get_mut(543), b"NEUJMIN 2");

    N2CCOD[544] = 1000065;
    fstr::assign(N2CNAM.get_mut(544), b"NEUJMIN 3");

    N2CCOD[545] = 1000066;
    fstr::assign(N2CNAM.get_mut(545), b"OLBERS");

    N2CCOD[546] = 1000067;
    fstr::assign(N2CNAM.get_mut(546), b"PETERS-HARTLEY");

    N2CCOD[547] = 1000068;
    fstr::assign(N2CNAM.get_mut(547), b"PONS-BROOKS");

    N2CCOD[548] = 1000069;
    fstr::assign(N2CNAM.get_mut(548), b"PONS-WINNECKE");

    N2CCOD[549] = 1000070;
    fstr::assign(N2CNAM.get_mut(549), b"REINMUTH 1");

    N2CCOD[550] = 1000071;
    fstr::assign(N2CNAM.get_mut(550), b"REINMUTH 2");

    N2CCOD[551] = 1000072;
    fstr::assign(N2CNAM.get_mut(551), b"RUSSELL 1");

    N2CCOD[552] = 1000073;
    fstr::assign(N2CNAM.get_mut(552), b"RUSSELL 2");

    N2CCOD[553] = 1000074;
    fstr::assign(N2CNAM.get_mut(553), b"RUSSELL 3");

    N2CCOD[554] = 1000075;
    fstr::assign(N2CNAM.get_mut(554), b"RUSSELL 4");

    N2CCOD[555] = 1000076;
    fstr::assign(N2CNAM.get_mut(555), b"SANGUIN");

    N2CCOD[556] = 1000077;
    fstr::assign(N2CNAM.get_mut(556), b"SCHAUMASSE");

    N2CCOD[557] = 1000078;
    fstr::assign(N2CNAM.get_mut(557), b"SCHUSTER");

    N2CCOD[558] = 1000079;
    fstr::assign(N2CNAM.get_mut(558), b"SCHWASSMANN-WACHMANN 1");

    N2CCOD[559] = 1000080;
    fstr::assign(N2CNAM.get_mut(559), b"SCHWASSMANN-WACHMANN 2");

    N2CCOD[560] = 1000081;
    fstr::assign(N2CNAM.get_mut(560), b"SCHWASSMANN-WACHMANN 3");

    N2CCOD[561] = 1000082;
    fstr::assign(N2CNAM.get_mut(561), b"SHAJN-SCHALDACH");

    N2CCOD[562] = 1000083;
    fstr::assign(N2CNAM.get_mut(562), b"SHOEMAKER 1");

    N2CCOD[563] = 1000084;
    fstr::assign(N2CNAM.get_mut(563), b"SHOEMAKER 2");

    N2CCOD[564] = 1000085;
    fstr::assign(N2CNAM.get_mut(564), b"SHOEMAKER 3");

    N2CCOD[565] = 1000086;
    fstr::assign(N2CNAM.get_mut(565), b"SINGER-BREWSTER");

    N2CCOD[566] = 1000087;
    fstr::assign(N2CNAM.get_mut(566), b"SLAUGHTER-BURNHAM");

    N2CCOD[567] = 1000088;
    fstr::assign(N2CNAM.get_mut(567), b"SMIRNOVA-CHERNYKH");

    N2CCOD[568] = 1000089;
    fstr::assign(N2CNAM.get_mut(568), b"STEPHAN-OTERMA");

    N2CCOD[569] = 1000090;
    fstr::assign(N2CNAM.get_mut(569), b"SWIFT-GEHRELS");

    N2CCOD[570] = 1000091;
    fstr::assign(N2CNAM.get_mut(570), b"TAKAMIZAWA");

    N2CCOD[571] = 1000092;
    fstr::assign(N2CNAM.get_mut(571), b"TAYLOR");

    N2CCOD[572] = 1000093;
    fstr::assign(N2CNAM.get_mut(572), b"TEMPEL_1");

    N2CCOD[573] = 1000093;
    fstr::assign(N2CNAM.get_mut(573), b"TEMPEL 1");

    N2CCOD[574] = 1000094;
    fstr::assign(N2CNAM.get_mut(574), b"TEMPEL 2");

    N2CCOD[575] = 1000095;
    fstr::assign(N2CNAM.get_mut(575), b"TEMPEL-TUTTLE");

    N2CCOD[576] = 1000096;
    fstr::assign(N2CNAM.get_mut(576), b"TRITTON");

    N2CCOD[577] = 1000097;
    fstr::assign(N2CNAM.get_mut(577), b"TSUCHINSHAN 1");

    N2CCOD[578] = 1000098;
    fstr::assign(N2CNAM.get_mut(578), b"TSUCHINSHAN 2");

    N2CCOD[579] = 1000099;
    fstr::assign(N2CNAM.get_mut(579), b"TUTTLE");

    N2CCOD[580] = 1000100;
    fstr::assign(N2CNAM.get_mut(580), b"TUTTLE-GIACOBINI-KRESAK");

    N2CCOD[581] = 1000101;
    fstr::assign(N2CNAM.get_mut(581), b"VAISALA 1");

    N2CCOD[582] = 1000102;
    fstr::assign(N2CNAM.get_mut(582), b"VAN BIESBROECK");

    N2CCOD[583] = 1000103;
    fstr::assign(N2CNAM.get_mut(583), b"VAN HOUTEN");

    N2CCOD[584] = 1000104;
    fstr::assign(N2CNAM.get_mut(584), b"WEST-KOHOUTEK-IKEMURA");

    N2CCOD[585] = 1000105;
    fstr::assign(N2CNAM.get_mut(585), b"WHIPPLE");

    N2CCOD[586] = 1000106;
    fstr::assign(N2CNAM.get_mut(586), b"WILD 1");

    N2CCOD[587] = 1000107;
    fstr::assign(N2CNAM.get_mut(587), b"WILD 2");

    N2CCOD[588] = 1000108;
    fstr::assign(N2CNAM.get_mut(588), b"WILD 3");

    N2CCOD[589] = 1000109;
    fstr::assign(N2CNAM.get_mut(589), b"WIRTANEN");

    N2CCOD[590] = 1000110;
    fstr::assign(N2CNAM.get_mut(590), b"WOLF");

    N2CCOD[591] = 1000111;
    fstr::assign(N2CNAM.get_mut(591), b"WOLF-HARRINGTON");

    N2CCOD[592] = 1000112;
    fstr::assign(N2CNAM.get_mut(592), b"LOVAS 2");

    N2CCOD[593] = 1000113;
    fstr::assign(N2CNAM.get_mut(593), b"URATA-NIIJIMA");

    N2CCOD[594] = 1000114;
    fstr::assign(N2CNAM.get_mut(594), b"WISEMAN-SKIFF");

    N2CCOD[595] = 1000115;
    fstr::assign(N2CNAM.get_mut(595), b"HELIN");

    N2CCOD[596] = 1000116;
    fstr::assign(N2CNAM.get_mut(596), b"MUELLER");

    N2CCOD[597] = 1000117;
    fstr::assign(N2CNAM.get_mut(597), b"SHOEMAKER-HOLT 1");

    N2CCOD[598] = 1000118;
    fstr::assign(N2CNAM.get_mut(598), b"HELIN-ROMAN-CROCKETT");

    N2CCOD[599] = 1000119;
    fstr::assign(N2CNAM.get_mut(599), b"HARTLEY 3");

    N2CCOD[600] = 1000120;
    fstr::assign(N2CNAM.get_mut(600), b"PARKER-HARTLEY");

    N2CCOD[601] = 1000121;
    fstr::assign(N2CNAM.get_mut(601), b"HELIN-ROMAN-ALU 1");

    N2CCOD[602] = 1000122;
    fstr::assign(N2CNAM.get_mut(602), b"WILD 4");

    N2CCOD[603] = 1000123;
    fstr::assign(N2CNAM.get_mut(603), b"MUELLER 2");

    N2CCOD[604] = 1000124;
    fstr::assign(N2CNAM.get_mut(604), b"MUELLER 3");

    N2CCOD[605] = 1000125;
    fstr::assign(N2CNAM.get_mut(605), b"SHOEMAKER-LEVY 1");

    N2CCOD[606] = 1000126;
    fstr::assign(N2CNAM.get_mut(606), b"SHOEMAKER-LEVY 2");

    N2CCOD[607] = 1000127;
    fstr::assign(N2CNAM.get_mut(607), b"HOLT-OLMSTEAD");

    N2CCOD[608] = 1000128;
    fstr::assign(N2CNAM.get_mut(608), b"METCALF-BREWINGTON");

    N2CCOD[609] = 1000129;
    fstr::assign(N2CNAM.get_mut(609), b"LEVY");

    N2CCOD[610] = 1000130;
    fstr::assign(N2CNAM.get_mut(610), b"SHOEMAKER-LEVY 9");

    N2CCOD[611] = 1000131;
    fstr::assign(N2CNAM.get_mut(611), b"HYAKUTAKE");

    N2CCOD[612] = 1000132;
    fstr::assign(N2CNAM.get_mut(612), b"HALE-BOPP");

    N2CCOD[613] = 1003228;
    fstr::assign(N2CNAM.get_mut(613), b"C/2013 A1");

    N2CCOD[614] = 1003228;
    fstr::assign(N2CNAM.get_mut(614), b"SIDING SPRING");

    N2CCOD[615] = 2000001;
    fstr::assign(N2CNAM.get_mut(615), b"CERES");

    N2CCOD[616] = 2000002;
    fstr::assign(N2CNAM.get_mut(616), b"PALLAS");

    N2CCOD[617] = 2000004;
    fstr::assign(N2CNAM.get_mut(617), b"VESTA");

    N2CCOD[618] = 2000016;
    fstr::assign(N2CNAM.get_mut(618), b"PSYCHE");

    N2CCOD[619] = 2000021;
    fstr::assign(N2CNAM.get_mut(619), b"LUTETIA");

    N2CCOD[620] = 2000052;
    fstr::assign(N2CNAM.get_mut(620), b"52_EUROPA");

    N2CCOD[621] = 2000052;
    fstr::assign(N2CNAM.get_mut(621), b"52 EUROPA");

    N2CCOD[622] = 2000216;
    fstr::assign(N2CNAM.get_mut(622), b"KLEOPATRA");

    N2CCOD[623] = 2000253;
    fstr::assign(N2CNAM.get_mut(623), b"MATHILDE");

    N2CCOD[624] = 2000433;
    fstr::assign(N2CNAM.get_mut(624), b"EROS");

    N2CCOD[625] = 2000511;
    fstr::assign(N2CNAM.get_mut(625), b"DAVIDA");

    N2CCOD[626] = 2002867;
    fstr::assign(N2CNAM.get_mut(626), b"STEINS");

    N2CCOD[627] = 2004015;
    fstr::assign(N2CNAM.get_mut(627), b"WILSON-HARRINGTON");

    N2CCOD[628] = 2004179;
    fstr::assign(N2CNAM.get_mut(628), b"TOUTATIS");

    N2CCOD[629] = 2009969;
    fstr::assign(N2CNAM.get_mut(629), b"1992KD");

    N2CCOD[630] = 2009969;
    fstr::assign(N2CNAM.get_mut(630), b"BRAILLE");

    N2CCOD[631] = 2025143;
    fstr::assign(N2CNAM.get_mut(631), b"ITOKAWA");

    N2CCOD[632] = 2101955;
    fstr::assign(N2CNAM.get_mut(632), b"BENNU");

    N2CCOD[633] = 2162173;
    fstr::assign(N2CNAM.get_mut(633), b"RYUGU");

    N2CCOD[634] = 2431010;
    fstr::assign(N2CNAM.get_mut(634), b"IDA");

    N2CCOD[635] = 2431011;
    fstr::assign(N2CNAM.get_mut(635), b"DACTYL");

    N2CCOD[636] = 2486958;
    fstr::assign(N2CNAM.get_mut(636), b"ARROKOTH");

    N2CCOD[637] = 9511010;
    fstr::assign(N2CNAM.get_mut(637), b"GASPRA");

    N2CCOD[638] = 20000617;
    fstr::assign(N2CNAM.get_mut(638), b"PATROCLUS_BARYCENTER");

    N2CCOD[639] = 20000617;
    fstr::assign(N2CNAM.get_mut(639), b"PATROCLUS BARYCENTER");

    N2CCOD[640] = 20003548;
    fstr::assign(N2CNAM.get_mut(640), b"EURYBATES_BARYCENTER");

    N2CCOD[641] = 20003548;
    fstr::assign(N2CNAM.get_mut(641), b"EURYBATES BARYCENTER");

    N2CCOD[642] = 20011351;
    fstr::assign(N2CNAM.get_mut(642), b"LEUCUS");

    N2CCOD[643] = 20015094;
    fstr::assign(N2CNAM.get_mut(643), b"POLYMELE");

    N2CCOD[644] = 20021900;
    fstr::assign(N2CNAM.get_mut(644), b"ORUS");

    N2CCOD[645] = 20052246;
    fstr::assign(N2CNAM.get_mut(645), b"DONALDJOHANSON");

    N2CCOD[646] = 20065803;
    fstr::assign(N2CNAM.get_mut(646), b"DIDYMOS_BARYCENTER");

    N2CCOD[647] = 20065803;
    fstr::assign(N2CNAM.get_mut(647), b"DIDYMOS BARYCENTER");

    N2CCOD[648] = 120000617;
    fstr::assign(N2CNAM.get_mut(648), b"MENOETIUS");

    N2CCOD[649] = 120003548;
    fstr::assign(N2CNAM.get_mut(649), b"QUETA");

    N2CCOD[650] = 120065803;
    fstr::assign(N2CNAM.get_mut(650), b"DIMORPHOS");

    N2CCOD[651] = 920000617;
    fstr::assign(N2CNAM.get_mut(651), b"PATROCLUS");

    N2CCOD[652] = 920003548;
    fstr::assign(N2CNAM.get_mut(652), b"EURYBATES");

    N2CCOD[653] = 920065803;
    fstr::assign(N2CNAM.get_mut(653), b"DIDYMOS");

    N2CCOD[654] = 398989;
    fstr::assign(N2CNAM.get_mut(654), b"NOTO");

    N2CCOD[655] = 398990;
    fstr::assign(N2CNAM.get_mut(655), b"NEW NORCIA");

    N2CCOD[656] = 399001;
    fstr::assign(N2CNAM.get_mut(656), b"GOLDSTONE");

    N2CCOD[657] = 399002;
    fstr::assign(N2CNAM.get_mut(657), b"CANBERRA");

    N2CCOD[658] = 399003;
    fstr::assign(N2CNAM.get_mut(658), b"MADRID");

    N2CCOD[659] = 399004;
    fstr::assign(N2CNAM.get_mut(659), b"USUDA");

    N2CCOD[660] = 399005;
    fstr::assign(N2CNAM.get_mut(660), b"DSS-05");

    N2CCOD[661] = 399005;
    fstr::assign(N2CNAM.get_mut(661), b"PARKES");

    N2CCOD[662] = 399012;
    fstr::assign(N2CNAM.get_mut(662), b"DSS-12");

    N2CCOD[663] = 399013;
    fstr::assign(N2CNAM.get_mut(663), b"DSS-13");

    N2CCOD[664] = 399014;
    fstr::assign(N2CNAM.get_mut(664), b"DSS-14");

    N2CCOD[665] = 399015;
    fstr::assign(N2CNAM.get_mut(665), b"DSS-15");

    N2CCOD[666] = 399016;
    fstr::assign(N2CNAM.get_mut(666), b"DSS-16");

    N2CCOD[667] = 399017;
    fstr::assign(N2CNAM.get_mut(667), b"DSS-17");

    N2CCOD[668] = 399023;
    fstr::assign(N2CNAM.get_mut(668), b"DSS-23");

    N2CCOD[669] = 399024;
    fstr::assign(N2CNAM.get_mut(669), b"DSS-24");

    N2CCOD[670] = 399025;
    fstr::assign(N2CNAM.get_mut(670), b"DSS-25");

    N2CCOD[671] = 399026;
    fstr::assign(N2CNAM.get_mut(671), b"DSS-26");

    N2CCOD[672] = 399027;
    fstr::assign(N2CNAM.get_mut(672), b"DSS-27");

    N2CCOD[673] = 399028;
    fstr::assign(N2CNAM.get_mut(673), b"DSS-28");

    N2CCOD[674] = 399033;
    fstr::assign(N2CNAM.get_mut(674), b"DSS-33");

    N2CCOD[675] = 399034;
    fstr::assign(N2CNAM.get_mut(675), b"DSS-34");

    N2CCOD[676] = 399035;
    fstr::assign(N2CNAM.get_mut(676), b"DSS-35");

    N2CCOD[677] = 399036;
    fstr::assign(N2CNAM.get_mut(677), b"DSS-36");

    N2CCOD[678] = 399042;
    fstr::assign(N2CNAM.get_mut(678), b"DSS-42");

    N2CCOD[679] = 399043;
    fstr::assign(N2CNAM.get_mut(679), b"DSS-43");

    N2CCOD[680] = 399045;
    fstr::assign(N2CNAM.get_mut(680), b"DSS-45");

    N2CCOD[681] = 399046;
    fstr::assign(N2CNAM.get_mut(681), b"DSS-46");

    N2CCOD[682] = 399049;
    fstr::assign(N2CNAM.get_mut(682), b"DSS-49");

    N2CCOD[683] = 399053;
    fstr::assign(N2CNAM.get_mut(683), b"DSS-53");

    N2CCOD[684] = 399054;
    fstr::assign(N2CNAM.get_mut(684), b"DSS-54");

    N2CCOD[685] = 399055;
    fstr::assign(N2CNAM.get_mut(685), b"DSS-55");

    N2CCOD[686] = 399056;
    fstr::assign(N2CNAM.get_mut(686), b"DSS-56");

    N2CCOD[687] = 399061;
    fstr::assign(N2CNAM.get_mut(687), b"DSS-61");

    N2CCOD[688] = 399063;
    fstr::assign(N2CNAM.get_mut(688), b"DSS-63");

    N2CCOD[689] = 399064;
    fstr::assign(N2CNAM.get_mut(689), b"DSS-64");

    N2CCOD[690] = 399065;
    fstr::assign(N2CNAM.get_mut(690), b"DSS-65");

    N2CCOD[691] = 399066;
    fstr::assign(N2CNAM.get_mut(691), b"DSS-66");

    N2CCOD[692] = 399069;
    fstr::assign(N2CNAM.get_mut(692), b"DSS-69");

    //
    // The data list for the ID -> NAME tests.
    // One-to-one and on-to.
    //

    C2NCOD[1] = -1;
    fstr::assign(C2NNAM.get_mut(1), b"GEOTAIL");

    C2NCOD[2] = -107;
    fstr::assign(C2NNAM.get_mut(2), b"TRMM");

    C2NCOD[3] = -112;
    fstr::assign(C2NNAM.get_mut(3), b"ICE");

    C2NCOD[4] = -116;
    fstr::assign(C2NNAM.get_mut(4), b"MPL");

    C2NCOD[5] = -117;
    fstr::assign(C2NNAM.get_mut(5), b"EXOMARS 2016 EDM");

    C2NCOD[6] = -119;
    fstr::assign(C2NNAM.get_mut(6), b"MOM2");

    C2NCOD[7] = -12;
    fstr::assign(C2NNAM.get_mut(7), b"LADEE");

    C2NCOD[8] = -121;
    fstr::assign(C2NNAM.get_mut(8), b"BEPICOLOMBO MPO");

    C2NCOD[9] = -127;
    fstr::assign(C2NNAM.get_mut(9), b"MCO");

    C2NCOD[10] = -13;
    fstr::assign(C2NNAM.get_mut(10), b"POLAR");

    C2NCOD[11] = -130;
    fstr::assign(C2NNAM.get_mut(11), b"HAYABUSA");

    C2NCOD[12] = -131;
    fstr::assign(C2NNAM.get_mut(12), b"KAGUYA");

    C2NCOD[13] = -135;
    fstr::assign(C2NNAM.get_mut(13), b"DOUBLE ASTEROID REDIRECTION TEST");

    C2NCOD[14] = -140;
    fstr::assign(C2NNAM.get_mut(14), b"DEEP IMPACT FLYBY SPACECRAFT");

    C2NCOD[15] = -142;
    fstr::assign(C2NNAM.get_mut(15), b"EOS-AM1");

    C2NCOD[16] = -143;
    fstr::assign(C2NNAM.get_mut(16), b"EXOMARS 2016 TGO");

    C2NCOD[17] = -144;
    fstr::assign(C2NNAM.get_mut(17), b"SOLAR ORBITER");

    C2NCOD[18] = -146;
    fstr::assign(C2NNAM.get_mut(18), b"LUNAR-A");

    C2NCOD[19] = -148;
    fstr::assign(C2NNAM.get_mut(19), b"DRAGONFLY");

    C2NCOD[20] = -150;
    fstr::assign(C2NNAM.get_mut(20), b"CASP");

    C2NCOD[21] = -151;
    fstr::assign(C2NNAM.get_mut(21), b"CHANDRA");

    C2NCOD[22] = -152;
    fstr::assign(C2NNAM.get_mut(22), b"CHANDRAYAAN-2 ORBITER");

    C2NCOD[23] = -153;
    fstr::assign(C2NNAM.get_mut(23), b"CHANDRAYAAN-2 LANDER");

    C2NCOD[24] = -154;
    fstr::assign(C2NNAM.get_mut(24), b"AQUA");

    C2NCOD[25] = -155;
    fstr::assign(C2NNAM.get_mut(25), b"KOREAN PATHFINDER LUNAR ORBITER");

    C2NCOD[26] = -156;
    fstr::assign(C2NNAM.get_mut(26), b"ADIT");

    C2NCOD[27] = -159;
    fstr::assign(C2NNAM.get_mut(27), b"EUROPA CLIPPER");

    C2NCOD[28] = -164;
    fstr::assign(C2NNAM.get_mut(28), b"LUNAR FLASHLIGHT");

    C2NCOD[29] = -165;
    fstr::assign(C2NNAM.get_mut(29), b"MAP");

    C2NCOD[30] = -166;
    fstr::assign(C2NNAM.get_mut(30), b"IMAGE");

    C2NCOD[31] = -168;
    fstr::assign(C2NNAM.get_mut(31), b"M2020");

    C2NCOD[32] = -170;
    fstr::assign(C2NNAM.get_mut(32), b"JAMES WEBB SPACE TELESCOPE");

    C2NCOD[33] = -172;
    fstr::assign(C2NNAM.get_mut(33), b"EXOMARS SCC");

    C2NCOD[34] = -173;
    fstr::assign(C2NNAM.get_mut(34), b"EXOMARS SP");

    C2NCOD[35] = -174;
    fstr::assign(C2NNAM.get_mut(35), b"EXOMARS ROVER");

    C2NCOD[36] = -177;
    fstr::assign(C2NNAM.get_mut(36), b"GRAIL-A");

    C2NCOD[37] = -178;
    fstr::assign(C2NNAM.get_mut(37), b"NOZOMI");

    C2NCOD[38] = -18;
    fstr::assign(C2NNAM.get_mut(38), b"LCROSS");

    C2NCOD[39] = -181;
    fstr::assign(C2NNAM.get_mut(39), b"GRAIL-B");

    C2NCOD[40] = -183;
    fstr::assign(C2NNAM.get_mut(40), b"CLUSTER 1");

    C2NCOD[41] = -185;
    fstr::assign(C2NNAM.get_mut(41), b"CLUSTER 2");

    C2NCOD[42] = -188;
    fstr::assign(C2NNAM.get_mut(42), b"MUSES-B");

    C2NCOD[43] = -189;
    fstr::assign(C2NNAM.get_mut(43), b"INSIGHT");

    C2NCOD[44] = -190;
    fstr::assign(C2NNAM.get_mut(44), b"SIM");

    C2NCOD[45] = -194;
    fstr::assign(C2NNAM.get_mut(45), b"CLUSTER 3");

    C2NCOD[46] = -196;
    fstr::assign(C2NNAM.get_mut(46), b"CLUSTER 4");

    C2NCOD[47] = -197;
    fstr::assign(C2NNAM.get_mut(47), b"LARA");

    C2NCOD[48] = -198;
    fstr::assign(C2NNAM.get_mut(48), b"NISAR");

    C2NCOD[49] = -20;
    fstr::assign(C2NNAM.get_mut(49), b"PIONEER-8");

    C2NCOD[50] = -200;
    fstr::assign(C2NNAM.get_mut(50), b"CONTOUR");

    C2NCOD[51] = -202;
    fstr::assign(C2NNAM.get_mut(51), b"MAVEN");

    C2NCOD[52] = -203;
    fstr::assign(C2NNAM.get_mut(52), b"DAWN");

    C2NCOD[53] = -205;
    fstr::assign(C2NNAM.get_mut(53), b"SMAP");

    C2NCOD[54] = -21;
    fstr::assign(C2NNAM.get_mut(54), b"SOHO");

    C2NCOD[55] = -210;
    fstr::assign(C2NNAM.get_mut(55), b"LICIACUBE");

    C2NCOD[56] = -212;
    fstr::assign(C2NNAM.get_mut(56), b"STV51");

    C2NCOD[57] = -213;
    fstr::assign(C2NNAM.get_mut(57), b"STV52");

    C2NCOD[58] = -214;
    fstr::assign(C2NNAM.get_mut(58), b"STV53");

    C2NCOD[59] = -226;
    fstr::assign(C2NNAM.get_mut(59), b"ROSETTA");

    C2NCOD[60] = -227;
    fstr::assign(C2NNAM.get_mut(60), b"KEPLER");

    C2NCOD[61] = -228;
    fstr::assign(C2NNAM.get_mut(61), b"GALILEO PROBE");

    C2NCOD[62] = -23;
    fstr::assign(C2NNAM.get_mut(62), b"PIONEER-10");

    C2NCOD[63] = -234;
    fstr::assign(C2NNAM.get_mut(63), b"STEREO AHEAD");

    C2NCOD[64] = -235;
    fstr::assign(C2NNAM.get_mut(64), b"STEREO BEHIND");

    C2NCOD[65] = -236;
    fstr::assign(C2NNAM.get_mut(65), b"MESSENGER");

    C2NCOD[66] = -238;
    fstr::assign(C2NNAM.get_mut(66), b"SMART-1");

    C2NCOD[67] = -239;
    fstr::assign(C2NNAM.get_mut(67), b"MMX");

    C2NCOD[68] = -24;
    fstr::assign(C2NNAM.get_mut(68), b"PIONEER-11");

    C2NCOD[69] = -240;
    fstr::assign(C2NNAM.get_mut(69), b"SLIM");

    C2NCOD[70] = -242;
    fstr::assign(C2NNAM.get_mut(70), b"LUNAR TRAILBLAZER");

    C2NCOD[71] = -243;
    fstr::assign(C2NNAM.get_mut(71), b"VIPER");

    C2NCOD[72] = -248;
    fstr::assign(C2NNAM.get_mut(72), b"VENUS EXPRESS");

    C2NCOD[73] = -25;
    fstr::assign(C2NNAM.get_mut(73), b"LUNAR PROSPECTOR");

    C2NCOD[74] = -253;
    fstr::assign(C2NNAM.get_mut(74), b"MER-1");

    C2NCOD[75] = -254;
    fstr::assign(C2NNAM.get_mut(75), b"MER-2");

    C2NCOD[76] = -255;
    fstr::assign(C2NNAM.get_mut(76), b"PSYC");

    C2NCOD[77] = -27;
    fstr::assign(C2NNAM.get_mut(77), b"VIKING 1 ORBITER");

    C2NCOD[78] = -28;
    fstr::assign(C2NNAM.get_mut(78), b"JUICE");

    C2NCOD[79] = -29;
    fstr::assign(C2NNAM.get_mut(79), b"NEXT");

    C2NCOD[80] = -3;
    fstr::assign(C2NNAM.get_mut(80), b"MARS ORBITER MISSION");

    C2NCOD[81] = -30;
    fstr::assign(C2NNAM.get_mut(81), b"DS-1");

    C2NCOD[82] = -301;
    fstr::assign(C2NNAM.get_mut(82), b"HELIOS 1");

    C2NCOD[83] = -302;
    fstr::assign(C2NNAM.get_mut(83), b"HELIOS 2");

    C2NCOD[84] = -31;
    fstr::assign(C2NNAM.get_mut(84), b"VOYAGER 1");

    C2NCOD[85] = -32;
    fstr::assign(C2NNAM.get_mut(85), b"VOYAGER 2");

    C2NCOD[86] = -33;
    fstr::assign(C2NNAM.get_mut(86), b"NEO SURVEYOR");

    C2NCOD[87] = -362;
    fstr::assign(C2NNAM.get_mut(87), b"RBSP_A");

    C2NCOD[88] = -363;
    fstr::assign(C2NNAM.get_mut(88), b"RBSP_B");

    C2NCOD[89] = -37;
    fstr::assign(C2NNAM.get_mut(89), b"HAYABUSA2");

    C2NCOD[90] = -39;
    fstr::assign(C2NNAM.get_mut(90), b"LUNAH-MAP");

    C2NCOD[91] = -40;
    fstr::assign(C2NNAM.get_mut(91), b"CLEMENTINE");

    C2NCOD[92] = -41;
    fstr::assign(C2NNAM.get_mut(92), b"MARS EXPRESS");

    C2NCOD[93] = -43;
    fstr::assign(C2NNAM.get_mut(93), b"IMAP");

    C2NCOD[94] = -44;
    fstr::assign(C2NNAM.get_mut(94), b"BEAGLE 2");

    C2NCOD[95] = -45;
    fstr::assign(C2NNAM.get_mut(95), b"JANUS_A");

    C2NCOD[96] = -46;
    fstr::assign(C2NNAM.get_mut(96), b"SAKIGAKE");

    C2NCOD[97] = -47;
    fstr::assign(C2NNAM.get_mut(97), b"GENESIS");

    C2NCOD[98] = -48;
    fstr::assign(C2NNAM.get_mut(98), b"HST");

    C2NCOD[99] = -49;
    fstr::assign(C2NNAM.get_mut(99), b"LUCY");

    C2NCOD[100] = -5;
    fstr::assign(C2NNAM.get_mut(100), b"PLANET-C");

    C2NCOD[101] = -500;
    fstr::assign(C2NNAM.get_mut(101), b"Rstar");

    C2NCOD[102] = -502;
    fstr::assign(C2NNAM.get_mut(102), b"Vstar");

    C2NCOD[103] = -53;
    fstr::assign(C2NNAM.get_mut(103), b"MARS SURVEYOR 01 ORBITER");

    C2NCOD[104] = -55;
    fstr::assign(C2NNAM.get_mut(104), b"ULYSSES");

    C2NCOD[105] = -550;
    fstr::assign(C2NNAM.get_mut(105), b"MARS96");

    C2NCOD[106] = -57;
    fstr::assign(C2NNAM.get_mut(106), b"LUNAR ICECUBE");

    C2NCOD[107] = -58;
    fstr::assign(C2NNAM.get_mut(107), b"HALCA");

    C2NCOD[108] = -59;
    fstr::assign(C2NNAM.get_mut(108), b"RADIOASTRON");

    C2NCOD[109] = -6;
    fstr::assign(C2NNAM.get_mut(109), b"PIONEER-6");

    C2NCOD[110] = -61;
    fstr::assign(C2NNAM.get_mut(110), b"JUNO");

    C2NCOD[111] = -62;
    fstr::assign(C2NNAM.get_mut(111), b"EMIRATES MARS MISSION");

    C2NCOD[112] = -64;
    fstr::assign(C2NNAM.get_mut(112), b"OSIRIS-REX");

    C2NCOD[113] = -65;
    fstr::assign(C2NNAM.get_mut(113), b"MARCO-A");

    C2NCOD[114] = -652;
    fstr::assign(C2NNAM.get_mut(114), b"BEPICOLOMBO MTM");

    C2NCOD[115] = -66;
    fstr::assign(C2NNAM.get_mut(115), b"MARCO-B");

    C2NCOD[116] = -67;
    fstr::assign(C2NNAM.get_mut(116), b"VEGA 2");

    C2NCOD[117] = -68;
    fstr::assign(C2NNAM.get_mut(117), b"BEPICOLOMBO MMO");

    C2NCOD[118] = -7;
    fstr::assign(C2NNAM.get_mut(118), b"PIONEER-7");

    C2NCOD[119] = -70;
    fstr::assign(C2NNAM.get_mut(119), b"DEEP IMPACT IMPACTOR SPACECRAFT");

    C2NCOD[120] = -72;
    fstr::assign(C2NNAM.get_mut(120), b"JANUS_B");

    C2NCOD[121] = -74;
    fstr::assign(C2NNAM.get_mut(121), b"MARS RECON ORBITER");

    C2NCOD[122] = -750;
    fstr::assign(C2NNAM.get_mut(122), b"SPRINT-A");

    C2NCOD[123] = -76;
    fstr::assign(C2NNAM.get_mut(123), b"MARS SCIENCE LABORATORY");

    C2NCOD[124] = -77;
    fstr::assign(C2NNAM.get_mut(124), b"GALILEO ORBITER");

    C2NCOD[125] = -78;
    fstr::assign(C2NNAM.get_mut(125), b"GIOTTO");

    C2NCOD[126] = -79;
    fstr::assign(C2NNAM.get_mut(126), b"SIRTF");

    C2NCOD[127] = -8;
    fstr::assign(C2NNAM.get_mut(127), b"WIND");

    C2NCOD[128] = -81;
    fstr::assign(C2NNAM.get_mut(128), b"CASSINI ITL");

    C2NCOD[129] = -82;
    fstr::assign(C2NNAM.get_mut(129), b"CASSINI");

    C2NCOD[130] = -84;
    fstr::assign(C2NNAM.get_mut(130), b"PHOENIX");

    C2NCOD[131] = -85;
    fstr::assign(C2NNAM.get_mut(131), b"LUNAR RECONNAISSANCE ORBITER");

    C2NCOD[132] = -86;
    fstr::assign(C2NNAM.get_mut(132), b"CHANDRAYAAN-1");

    C2NCOD[133] = -90;
    fstr::assign(C2NNAM.get_mut(133), b"CASSINI SIMULATION");

    C2NCOD[134] = -93;
    fstr::assign(C2NNAM.get_mut(134), b"NEAR");

    C2NCOD[135] = -94;
    fstr::assign(C2NNAM.get_mut(135), b"MARS GLOBAL SURVEYOR");

    C2NCOD[136] = -95;
    fstr::assign(C2NNAM.get_mut(136), b"MGS SIMULATION");

    C2NCOD[137] = -96;
    fstr::assign(C2NNAM.get_mut(137), b"SOLAR PROBE PLUS");

    C2NCOD[138] = -97;
    fstr::assign(C2NNAM.get_mut(138), b"TOPEX/POSEIDON");

    C2NCOD[139] = -98;
    fstr::assign(C2NNAM.get_mut(139), b"NEW HORIZONS");

    C2NCOD[140] = 0;
    fstr::assign(C2NNAM.get_mut(140), b"SOLAR SYSTEM BARYCENTER");

    C2NCOD[141] = 1;
    fstr::assign(C2NNAM.get_mut(141), b"MERCURY BARYCENTER");

    C2NCOD[142] = 10;
    fstr::assign(C2NNAM.get_mut(142), b"SUN");

    C2NCOD[143] = 1000001;
    fstr::assign(C2NNAM.get_mut(143), b"AREND");

    C2NCOD[144] = 1000002;
    fstr::assign(C2NNAM.get_mut(144), b"AREND-RIGAUX");

    C2NCOD[145] = 1000003;
    fstr::assign(C2NNAM.get_mut(145), b"ASHBROOK-JACKSON");

    C2NCOD[146] = 1000004;
    fstr::assign(C2NNAM.get_mut(146), b"BOETHIN");

    C2NCOD[147] = 1000005;
    fstr::assign(C2NNAM.get_mut(147), b"BORRELLY");

    C2NCOD[148] = 1000006;
    fstr::assign(C2NNAM.get_mut(148), b"BOWELL-SKIFF");

    C2NCOD[149] = 1000007;
    fstr::assign(C2NNAM.get_mut(149), b"BRADFIELD");

    C2NCOD[150] = 1000008;
    fstr::assign(C2NNAM.get_mut(150), b"BROOKS 2");

    C2NCOD[151] = 1000009;
    fstr::assign(C2NNAM.get_mut(151), b"BRORSEN-METCALF");

    C2NCOD[152] = 1000010;
    fstr::assign(C2NNAM.get_mut(152), b"BUS");

    C2NCOD[153] = 1000011;
    fstr::assign(C2NNAM.get_mut(153), b"CHERNYKH");

    C2NCOD[154] = 1000012;
    fstr::assign(C2NNAM.get_mut(154), b"CHURYUMOV-GERASIMENKO");

    C2NCOD[155] = 1000013;
    fstr::assign(C2NNAM.get_mut(155), b"CIFFREO");

    C2NCOD[156] = 1000014;
    fstr::assign(C2NNAM.get_mut(156), b"CLARK");

    C2NCOD[157] = 1000015;
    fstr::assign(C2NNAM.get_mut(157), b"COMAS SOLA");

    C2NCOD[158] = 1000016;
    fstr::assign(C2NNAM.get_mut(158), b"CROMMELIN");

    C2NCOD[159] = 1000017;
    fstr::assign(C2NNAM.get_mut(159), b"D\'ARREST");

    C2NCOD[160] = 1000018;
    fstr::assign(C2NNAM.get_mut(160), b"DANIEL");

    C2NCOD[161] = 1000019;
    fstr::assign(C2NNAM.get_mut(161), b"DE VICO-SWIFT");

    C2NCOD[162] = 1000020;
    fstr::assign(C2NNAM.get_mut(162), b"DENNING-FUJIKAWA");

    C2NCOD[163] = 1000021;
    fstr::assign(C2NNAM.get_mut(163), b"DU TOIT 1");

    C2NCOD[164] = 1000022;
    fstr::assign(C2NNAM.get_mut(164), b"DU TOIT-HARTLEY");

    C2NCOD[165] = 1000023;
    fstr::assign(C2NNAM.get_mut(165), b"DUTOIT-NEUJMIN-DELPORTE");

    C2NCOD[166] = 1000024;
    fstr::assign(C2NNAM.get_mut(166), b"DUBIAGO");

    C2NCOD[167] = 1000025;
    fstr::assign(C2NNAM.get_mut(167), b"ENCKE");

    C2NCOD[168] = 1000026;
    fstr::assign(C2NNAM.get_mut(168), b"FAYE");

    C2NCOD[169] = 1000027;
    fstr::assign(C2NNAM.get_mut(169), b"FINLAY");

    C2NCOD[170] = 1000028;
    fstr::assign(C2NNAM.get_mut(170), b"FORBES");

    C2NCOD[171] = 1000029;
    fstr::assign(C2NNAM.get_mut(171), b"GEHRELS 1");

    C2NCOD[172] = 1000030;
    fstr::assign(C2NNAM.get_mut(172), b"GEHRELS 2");

    C2NCOD[173] = 1000031;
    fstr::assign(C2NNAM.get_mut(173), b"GEHRELS 3");

    C2NCOD[174] = 1000032;
    fstr::assign(C2NNAM.get_mut(174), b"GIACOBINI-ZINNER");

    C2NCOD[175] = 1000033;
    fstr::assign(C2NNAM.get_mut(175), b"GICLAS");

    C2NCOD[176] = 1000034;
    fstr::assign(C2NNAM.get_mut(176), b"GRIGG-SKJELLERUP");

    C2NCOD[177] = 1000035;
    fstr::assign(C2NNAM.get_mut(177), b"GUNN");

    C2NCOD[178] = 1000036;
    fstr::assign(C2NNAM.get_mut(178), b"HALLEY");

    C2NCOD[179] = 1000037;
    fstr::assign(C2NNAM.get_mut(179), b"HANEDA-CAMPOS");

    C2NCOD[180] = 1000038;
    fstr::assign(C2NNAM.get_mut(180), b"HARRINGTON");

    C2NCOD[181] = 1000039;
    fstr::assign(C2NNAM.get_mut(181), b"HARRINGTON-ABELL");

    C2NCOD[182] = 1000040;
    fstr::assign(C2NNAM.get_mut(182), b"HARTLEY 1");

    C2NCOD[183] = 1000041;
    fstr::assign(C2NNAM.get_mut(183), b"HARTLEY 2");

    C2NCOD[184] = 1000042;
    fstr::assign(C2NNAM.get_mut(184), b"HARTLEY-IRAS");

    C2NCOD[185] = 1000043;
    fstr::assign(C2NNAM.get_mut(185), b"HERSCHEL-RIGOLLET");

    C2NCOD[186] = 1000044;
    fstr::assign(C2NNAM.get_mut(186), b"HOLMES");

    C2NCOD[187] = 1000045;
    fstr::assign(C2NNAM.get_mut(187), b"HONDA-MRKOS-PAJDUSAKOVA");

    C2NCOD[188] = 1000046;
    fstr::assign(C2NNAM.get_mut(188), b"HOWELL");

    C2NCOD[189] = 1000047;
    fstr::assign(C2NNAM.get_mut(189), b"IRAS");

    C2NCOD[190] = 1000048;
    fstr::assign(C2NNAM.get_mut(190), b"JACKSON-NEUJMIN");

    C2NCOD[191] = 1000049;
    fstr::assign(C2NNAM.get_mut(191), b"JOHNSON");

    C2NCOD[192] = 1000050;
    fstr::assign(C2NNAM.get_mut(192), b"KEARNS-KWEE");

    C2NCOD[193] = 1000051;
    fstr::assign(C2NNAM.get_mut(193), b"KLEMOLA");

    C2NCOD[194] = 1000052;
    fstr::assign(C2NNAM.get_mut(194), b"KOHOUTEK");

    C2NCOD[195] = 1000053;
    fstr::assign(C2NNAM.get_mut(195), b"KOJIMA");

    C2NCOD[196] = 1000054;
    fstr::assign(C2NNAM.get_mut(196), b"KOPFF");

    C2NCOD[197] = 1000055;
    fstr::assign(C2NNAM.get_mut(197), b"KOWAL 1");

    C2NCOD[198] = 1000056;
    fstr::assign(C2NNAM.get_mut(198), b"KOWAL 2");

    C2NCOD[199] = 1000057;
    fstr::assign(C2NNAM.get_mut(199), b"KOWAL-MRKOS");

    C2NCOD[200] = 1000058;
    fstr::assign(C2NNAM.get_mut(200), b"KOWAL-VAVROVA");

    C2NCOD[201] = 1000059;
    fstr::assign(C2NNAM.get_mut(201), b"LONGMORE");

    C2NCOD[202] = 1000060;
    fstr::assign(C2NNAM.get_mut(202), b"LOVAS 1");

    C2NCOD[203] = 1000061;
    fstr::assign(C2NNAM.get_mut(203), b"MACHHOLZ");

    C2NCOD[204] = 1000062;
    fstr::assign(C2NNAM.get_mut(204), b"MAURY");

    C2NCOD[205] = 1000063;
    fstr::assign(C2NNAM.get_mut(205), b"NEUJMIN 1");

    C2NCOD[206] = 1000064;
    fstr::assign(C2NNAM.get_mut(206), b"NEUJMIN 2");

    C2NCOD[207] = 1000065;
    fstr::assign(C2NNAM.get_mut(207), b"NEUJMIN 3");

    C2NCOD[208] = 1000066;
    fstr::assign(C2NNAM.get_mut(208), b"OLBERS");

    C2NCOD[209] = 1000067;
    fstr::assign(C2NNAM.get_mut(209), b"PETERS-HARTLEY");

    C2NCOD[210] = 1000068;
    fstr::assign(C2NNAM.get_mut(210), b"PONS-BROOKS");

    C2NCOD[211] = 1000069;
    fstr::assign(C2NNAM.get_mut(211), b"PONS-WINNECKE");

    C2NCOD[212] = 1000070;
    fstr::assign(C2NNAM.get_mut(212), b"REINMUTH 1");

    C2NCOD[213] = 1000071;
    fstr::assign(C2NNAM.get_mut(213), b"REINMUTH 2");

    C2NCOD[214] = 1000072;
    fstr::assign(C2NNAM.get_mut(214), b"RUSSELL 1");

    C2NCOD[215] = 1000073;
    fstr::assign(C2NNAM.get_mut(215), b"RUSSELL 2");

    C2NCOD[216] = 1000074;
    fstr::assign(C2NNAM.get_mut(216), b"RUSSELL 3");

    C2NCOD[217] = 1000075;
    fstr::assign(C2NNAM.get_mut(217), b"RUSSELL 4");

    C2NCOD[218] = 1000076;
    fstr::assign(C2NNAM.get_mut(218), b"SANGUIN");

    C2NCOD[219] = 1000077;
    fstr::assign(C2NNAM.get_mut(219), b"SCHAUMASSE");

    C2NCOD[220] = 1000078;
    fstr::assign(C2NNAM.get_mut(220), b"SCHUSTER");

    C2NCOD[221] = 1000079;
    fstr::assign(C2NNAM.get_mut(221), b"SCHWASSMANN-WACHMANN 1");

    C2NCOD[222] = 1000080;
    fstr::assign(C2NNAM.get_mut(222), b"SCHWASSMANN-WACHMANN 2");

    C2NCOD[223] = 1000081;
    fstr::assign(C2NNAM.get_mut(223), b"SCHWASSMANN-WACHMANN 3");

    C2NCOD[224] = 1000082;
    fstr::assign(C2NNAM.get_mut(224), b"SHAJN-SCHALDACH");

    C2NCOD[225] = 1000083;
    fstr::assign(C2NNAM.get_mut(225), b"SHOEMAKER 1");

    C2NCOD[226] = 1000084;
    fstr::assign(C2NNAM.get_mut(226), b"SHOEMAKER 2");

    C2NCOD[227] = 1000085;
    fstr::assign(C2NNAM.get_mut(227), b"SHOEMAKER 3");

    C2NCOD[228] = 1000086;
    fstr::assign(C2NNAM.get_mut(228), b"SINGER-BREWSTER");

    C2NCOD[229] = 1000087;
    fstr::assign(C2NNAM.get_mut(229), b"SLAUGHTER-BURNHAM");

    C2NCOD[230] = 1000088;
    fstr::assign(C2NNAM.get_mut(230), b"SMIRNOVA-CHERNYKH");

    C2NCOD[231] = 1000089;
    fstr::assign(C2NNAM.get_mut(231), b"STEPHAN-OTERMA");

    C2NCOD[232] = 1000090;
    fstr::assign(C2NNAM.get_mut(232), b"SWIFT-GEHRELS");

    C2NCOD[233] = 1000091;
    fstr::assign(C2NNAM.get_mut(233), b"TAKAMIZAWA");

    C2NCOD[234] = 1000092;
    fstr::assign(C2NNAM.get_mut(234), b"TAYLOR");

    C2NCOD[235] = 1000093;
    fstr::assign(C2NNAM.get_mut(235), b"TEMPEL 1");

    C2NCOD[236] = 1000094;
    fstr::assign(C2NNAM.get_mut(236), b"TEMPEL 2");

    C2NCOD[237] = 1000095;
    fstr::assign(C2NNAM.get_mut(237), b"TEMPEL-TUTTLE");

    C2NCOD[238] = 1000096;
    fstr::assign(C2NNAM.get_mut(238), b"TRITTON");

    C2NCOD[239] = 1000097;
    fstr::assign(C2NNAM.get_mut(239), b"TSUCHINSHAN 1");

    C2NCOD[240] = 1000098;
    fstr::assign(C2NNAM.get_mut(240), b"TSUCHINSHAN 2");

    C2NCOD[241] = 1000099;
    fstr::assign(C2NNAM.get_mut(241), b"TUTTLE");

    C2NCOD[242] = 1000100;
    fstr::assign(C2NNAM.get_mut(242), b"TUTTLE-GIACOBINI-KRESAK");

    C2NCOD[243] = 1000101;
    fstr::assign(C2NNAM.get_mut(243), b"VAISALA 1");

    C2NCOD[244] = 1000102;
    fstr::assign(C2NNAM.get_mut(244), b"VAN BIESBROECK");

    C2NCOD[245] = 1000103;
    fstr::assign(C2NNAM.get_mut(245), b"VAN HOUTEN");

    C2NCOD[246] = 1000104;
    fstr::assign(C2NNAM.get_mut(246), b"WEST-KOHOUTEK-IKEMURA");

    C2NCOD[247] = 1000105;
    fstr::assign(C2NNAM.get_mut(247), b"WHIPPLE");

    C2NCOD[248] = 1000106;
    fstr::assign(C2NNAM.get_mut(248), b"WILD 1");

    C2NCOD[249] = 1000107;
    fstr::assign(C2NNAM.get_mut(249), b"WILD 2");

    C2NCOD[250] = 1000108;
    fstr::assign(C2NNAM.get_mut(250), b"WILD 3");

    C2NCOD[251] = 1000109;
    fstr::assign(C2NNAM.get_mut(251), b"WIRTANEN");

    C2NCOD[252] = 1000110;
    fstr::assign(C2NNAM.get_mut(252), b"WOLF");

    C2NCOD[253] = 1000111;
    fstr::assign(C2NNAM.get_mut(253), b"WOLF-HARRINGTON");

    C2NCOD[254] = 1000112;
    fstr::assign(C2NNAM.get_mut(254), b"LOVAS 2");

    C2NCOD[255] = 1000113;
    fstr::assign(C2NNAM.get_mut(255), b"URATA-NIIJIMA");

    C2NCOD[256] = 1000114;
    fstr::assign(C2NNAM.get_mut(256), b"WISEMAN-SKIFF");

    C2NCOD[257] = 1000115;
    fstr::assign(C2NNAM.get_mut(257), b"HELIN");

    C2NCOD[258] = 1000116;
    fstr::assign(C2NNAM.get_mut(258), b"MUELLER");

    C2NCOD[259] = 1000117;
    fstr::assign(C2NNAM.get_mut(259), b"SHOEMAKER-HOLT 1");

    C2NCOD[260] = 1000118;
    fstr::assign(C2NNAM.get_mut(260), b"HELIN-ROMAN-CROCKETT");

    C2NCOD[261] = 1000119;
    fstr::assign(C2NNAM.get_mut(261), b"HARTLEY 3");

    C2NCOD[262] = 1000120;
    fstr::assign(C2NNAM.get_mut(262), b"PARKER-HARTLEY");

    C2NCOD[263] = 1000121;
    fstr::assign(C2NNAM.get_mut(263), b"HELIN-ROMAN-ALU 1");

    C2NCOD[264] = 1000122;
    fstr::assign(C2NNAM.get_mut(264), b"WILD 4");

    C2NCOD[265] = 1000123;
    fstr::assign(C2NNAM.get_mut(265), b"MUELLER 2");

    C2NCOD[266] = 1000124;
    fstr::assign(C2NNAM.get_mut(266), b"MUELLER 3");

    C2NCOD[267] = 1000125;
    fstr::assign(C2NNAM.get_mut(267), b"SHOEMAKER-LEVY 1");

    C2NCOD[268] = 1000126;
    fstr::assign(C2NNAM.get_mut(268), b"SHOEMAKER-LEVY 2");

    C2NCOD[269] = 1000127;
    fstr::assign(C2NNAM.get_mut(269), b"HOLT-OLMSTEAD");

    C2NCOD[270] = 1000128;
    fstr::assign(C2NNAM.get_mut(270), b"METCALF-BREWINGTON");

    C2NCOD[271] = 1000129;
    fstr::assign(C2NNAM.get_mut(271), b"LEVY");

    C2NCOD[272] = 1000130;
    fstr::assign(C2NNAM.get_mut(272), b"SHOEMAKER-LEVY 9");

    C2NCOD[273] = 1000131;
    fstr::assign(C2NNAM.get_mut(273), b"HYAKUTAKE");

    C2NCOD[274] = 1000132;
    fstr::assign(C2NNAM.get_mut(274), b"HALE-BOPP");

    C2NCOD[275] = 1003228;
    fstr::assign(C2NNAM.get_mut(275), b"SIDING SPRING");

    C2NCOD[276] = 120000617;
    fstr::assign(C2NNAM.get_mut(276), b"MENOETIUS");

    C2NCOD[277] = 120003548;
    fstr::assign(C2NNAM.get_mut(277), b"QUETA");

    C2NCOD[278] = 120065803;
    fstr::assign(C2NNAM.get_mut(278), b"DIMORPHOS");

    C2NCOD[279] = 199;
    fstr::assign(C2NNAM.get_mut(279), b"MERCURY");

    C2NCOD[280] = 2;
    fstr::assign(C2NNAM.get_mut(280), b"VENUS BARYCENTER");

    C2NCOD[281] = 2000001;
    fstr::assign(C2NNAM.get_mut(281), b"CERES");

    C2NCOD[282] = 2000002;
    fstr::assign(C2NNAM.get_mut(282), b"PALLAS");

    C2NCOD[283] = 2000004;
    fstr::assign(C2NNAM.get_mut(283), b"VESTA");

    C2NCOD[284] = 2000016;
    fstr::assign(C2NNAM.get_mut(284), b"PSYCHE");

    C2NCOD[285] = 2000021;
    fstr::assign(C2NNAM.get_mut(285), b"LUTETIA");

    C2NCOD[286] = 2000052;
    fstr::assign(C2NNAM.get_mut(286), b"52 EUROPA");

    C2NCOD[287] = 20000617;
    fstr::assign(C2NNAM.get_mut(287), b"PATROCLUS BARYCENTER");

    C2NCOD[288] = 2000216;
    fstr::assign(C2NNAM.get_mut(288), b"KLEOPATRA");

    C2NCOD[289] = 2000253;
    fstr::assign(C2NNAM.get_mut(289), b"MATHILDE");

    C2NCOD[290] = 20003548;
    fstr::assign(C2NNAM.get_mut(290), b"EURYBATES BARYCENTER");

    C2NCOD[291] = 2000433;
    fstr::assign(C2NNAM.get_mut(291), b"EROS");

    C2NCOD[292] = 2000511;
    fstr::assign(C2NNAM.get_mut(292), b"DAVIDA");

    C2NCOD[293] = 20011351;
    fstr::assign(C2NNAM.get_mut(293), b"LEUCUS");

    C2NCOD[294] = 20015094;
    fstr::assign(C2NNAM.get_mut(294), b"POLYMELE");

    C2NCOD[295] = 20021900;
    fstr::assign(C2NNAM.get_mut(295), b"ORUS");

    C2NCOD[296] = 2002867;
    fstr::assign(C2NNAM.get_mut(296), b"STEINS");

    C2NCOD[297] = 2004015;
    fstr::assign(C2NNAM.get_mut(297), b"WILSON-HARRINGTON");

    C2NCOD[298] = 2004179;
    fstr::assign(C2NNAM.get_mut(298), b"TOUTATIS");

    C2NCOD[299] = 20052246;
    fstr::assign(C2NNAM.get_mut(299), b"DONALDJOHANSON");

    C2NCOD[300] = 20065803;
    fstr::assign(C2NNAM.get_mut(300), b"DIDYMOS BARYCENTER");

    C2NCOD[301] = 2009969;
    fstr::assign(C2NNAM.get_mut(301), b"BRAILLE");

    C2NCOD[302] = 2025143;
    fstr::assign(C2NNAM.get_mut(302), b"ITOKAWA");

    C2NCOD[303] = 2101955;
    fstr::assign(C2NNAM.get_mut(303), b"BENNU");

    C2NCOD[304] = 2162173;
    fstr::assign(C2NNAM.get_mut(304), b"RYUGU");

    C2NCOD[305] = 2431010;
    fstr::assign(C2NNAM.get_mut(305), b"IDA");

    C2NCOD[306] = 2431011;
    fstr::assign(C2NNAM.get_mut(306), b"DACTYL");

    C2NCOD[307] = 2486958;
    fstr::assign(C2NNAM.get_mut(307), b"ARROKOTH");

    C2NCOD[308] = 299;
    fstr::assign(C2NNAM.get_mut(308), b"VENUS");

    C2NCOD[309] = 3;
    fstr::assign(C2NNAM.get_mut(309), b"EARTH BARYCENTER");

    C2NCOD[310] = 301;
    fstr::assign(C2NNAM.get_mut(310), b"MOON");

    C2NCOD[311] = 398989;
    fstr::assign(C2NNAM.get_mut(311), b"NOTO");

    C2NCOD[312] = 398990;
    fstr::assign(C2NNAM.get_mut(312), b"NEW NORCIA");

    C2NCOD[313] = 399;
    fstr::assign(C2NNAM.get_mut(313), b"EARTH");

    C2NCOD[314] = 399001;
    fstr::assign(C2NNAM.get_mut(314), b"GOLDSTONE");

    C2NCOD[315] = 399002;
    fstr::assign(C2NNAM.get_mut(315), b"CANBERRA");

    C2NCOD[316] = 399003;
    fstr::assign(C2NNAM.get_mut(316), b"MADRID");

    C2NCOD[317] = 399004;
    fstr::assign(C2NNAM.get_mut(317), b"USUDA");

    C2NCOD[318] = 399005;
    fstr::assign(C2NNAM.get_mut(318), b"PARKES");

    C2NCOD[319] = 399012;
    fstr::assign(C2NNAM.get_mut(319), b"DSS-12");

    C2NCOD[320] = 399013;
    fstr::assign(C2NNAM.get_mut(320), b"DSS-13");

    C2NCOD[321] = 399014;
    fstr::assign(C2NNAM.get_mut(321), b"DSS-14");

    C2NCOD[322] = 399015;
    fstr::assign(C2NNAM.get_mut(322), b"DSS-15");

    C2NCOD[323] = 399016;
    fstr::assign(C2NNAM.get_mut(323), b"DSS-16");

    C2NCOD[324] = 399017;
    fstr::assign(C2NNAM.get_mut(324), b"DSS-17");

    C2NCOD[325] = 399023;
    fstr::assign(C2NNAM.get_mut(325), b"DSS-23");

    C2NCOD[326] = 399024;
    fstr::assign(C2NNAM.get_mut(326), b"DSS-24");

    C2NCOD[327] = 399025;
    fstr::assign(C2NNAM.get_mut(327), b"DSS-25");

    C2NCOD[328] = 399026;
    fstr::assign(C2NNAM.get_mut(328), b"DSS-26");

    C2NCOD[329] = 399027;
    fstr::assign(C2NNAM.get_mut(329), b"DSS-27");

    C2NCOD[330] = 399028;
    fstr::assign(C2NNAM.get_mut(330), b"DSS-28");

    C2NCOD[331] = 399033;
    fstr::assign(C2NNAM.get_mut(331), b"DSS-33");

    C2NCOD[332] = 399034;
    fstr::assign(C2NNAM.get_mut(332), b"DSS-34");

    C2NCOD[333] = 399035;
    fstr::assign(C2NNAM.get_mut(333), b"DSS-35");

    C2NCOD[334] = 399036;
    fstr::assign(C2NNAM.get_mut(334), b"DSS-36");

    C2NCOD[335] = 399042;
    fstr::assign(C2NNAM.get_mut(335), b"DSS-42");

    C2NCOD[336] = 399043;
    fstr::assign(C2NNAM.get_mut(336), b"DSS-43");

    C2NCOD[337] = 399045;
    fstr::assign(C2NNAM.get_mut(337), b"DSS-45");

    C2NCOD[338] = 399046;
    fstr::assign(C2NNAM.get_mut(338), b"DSS-46");

    C2NCOD[339] = 399049;
    fstr::assign(C2NNAM.get_mut(339), b"DSS-49");

    C2NCOD[340] = 399053;
    fstr::assign(C2NNAM.get_mut(340), b"DSS-53");

    C2NCOD[341] = 399054;
    fstr::assign(C2NNAM.get_mut(341), b"DSS-54");

    C2NCOD[342] = 399055;
    fstr::assign(C2NNAM.get_mut(342), b"DSS-55");

    C2NCOD[343] = 399056;
    fstr::assign(C2NNAM.get_mut(343), b"DSS-56");

    C2NCOD[344] = 399061;
    fstr::assign(C2NNAM.get_mut(344), b"DSS-61");

    C2NCOD[345] = 399063;
    fstr::assign(C2NNAM.get_mut(345), b"DSS-63");

    C2NCOD[346] = 399064;
    fstr::assign(C2NNAM.get_mut(346), b"DSS-64");

    C2NCOD[347] = 399065;
    fstr::assign(C2NNAM.get_mut(347), b"DSS-65");

    C2NCOD[348] = 399066;
    fstr::assign(C2NNAM.get_mut(348), b"DSS-66");

    C2NCOD[349] = 399069;
    fstr::assign(C2NNAM.get_mut(349), b"DSS-69");

    C2NCOD[350] = 4;
    fstr::assign(C2NNAM.get_mut(350), b"MARS BARYCENTER");

    C2NCOD[351] = 401;
    fstr::assign(C2NNAM.get_mut(351), b"PHOBOS");

    C2NCOD[352] = 402;
    fstr::assign(C2NNAM.get_mut(352), b"DEIMOS");

    C2NCOD[353] = 499;
    fstr::assign(C2NNAM.get_mut(353), b"MARS");

    C2NCOD[354] = 5;
    fstr::assign(C2NNAM.get_mut(354), b"JUPITER BARYCENTER");

    C2NCOD[355] = 50000001;
    fstr::assign(C2NNAM.get_mut(355), b"SHOEMAKER-LEVY 9-W");

    C2NCOD[356] = 50000002;
    fstr::assign(C2NNAM.get_mut(356), b"SHOEMAKER-LEVY 9-V");

    C2NCOD[357] = 50000003;
    fstr::assign(C2NNAM.get_mut(357), b"SHOEMAKER-LEVY 9-U");

    C2NCOD[358] = 50000004;
    fstr::assign(C2NNAM.get_mut(358), b"SHOEMAKER-LEVY 9-T");

    C2NCOD[359] = 50000005;
    fstr::assign(C2NNAM.get_mut(359), b"SHOEMAKER-LEVY 9-S");

    C2NCOD[360] = 50000006;
    fstr::assign(C2NNAM.get_mut(360), b"SHOEMAKER-LEVY 9-R");

    C2NCOD[361] = 50000007;
    fstr::assign(C2NNAM.get_mut(361), b"SHOEMAKER-LEVY 9-Q");

    C2NCOD[362] = 50000008;
    fstr::assign(C2NNAM.get_mut(362), b"SHOEMAKER-LEVY 9-P");

    C2NCOD[363] = 50000009;
    fstr::assign(C2NNAM.get_mut(363), b"SHOEMAKER-LEVY 9-N");

    C2NCOD[364] = 50000010;
    fstr::assign(C2NNAM.get_mut(364), b"SHOEMAKER-LEVY 9-M");

    C2NCOD[365] = 50000011;
    fstr::assign(C2NNAM.get_mut(365), b"SHOEMAKER-LEVY 9-L");

    C2NCOD[366] = 50000012;
    fstr::assign(C2NNAM.get_mut(366), b"SHOEMAKER-LEVY 9-K");

    C2NCOD[367] = 50000013;
    fstr::assign(C2NNAM.get_mut(367), b"SHOEMAKER-LEVY 9-J");

    C2NCOD[368] = 50000014;
    fstr::assign(C2NNAM.get_mut(368), b"SHOEMAKER-LEVY 9-H");

    C2NCOD[369] = 50000015;
    fstr::assign(C2NNAM.get_mut(369), b"SHOEMAKER-LEVY 9-G");

    C2NCOD[370] = 50000016;
    fstr::assign(C2NNAM.get_mut(370), b"SHOEMAKER-LEVY 9-F");

    C2NCOD[371] = 50000017;
    fstr::assign(C2NNAM.get_mut(371), b"SHOEMAKER-LEVY 9-E");

    C2NCOD[372] = 50000018;
    fstr::assign(C2NNAM.get_mut(372), b"SHOEMAKER-LEVY 9-D");

    C2NCOD[373] = 50000019;
    fstr::assign(C2NNAM.get_mut(373), b"SHOEMAKER-LEVY 9-C");

    C2NCOD[374] = 50000020;
    fstr::assign(C2NNAM.get_mut(374), b"SHOEMAKER-LEVY 9-B");

    C2NCOD[375] = 50000021;
    fstr::assign(C2NNAM.get_mut(375), b"SHOEMAKER-LEVY 9-A");

    C2NCOD[376] = 50000022;
    fstr::assign(C2NNAM.get_mut(376), b"SHOEMAKER-LEVY 9-Q1");

    C2NCOD[377] = 50000023;
    fstr::assign(C2NNAM.get_mut(377), b"SHOEMAKER-LEVY 9-P2");

    C2NCOD[378] = 501;
    fstr::assign(C2NNAM.get_mut(378), b"IO");

    C2NCOD[379] = 502;
    fstr::assign(C2NNAM.get_mut(379), b"EUROPA");

    C2NCOD[380] = 503;
    fstr::assign(C2NNAM.get_mut(380), b"GANYMEDE");

    C2NCOD[381] = 504;
    fstr::assign(C2NNAM.get_mut(381), b"CALLISTO");

    C2NCOD[382] = 505;
    fstr::assign(C2NNAM.get_mut(382), b"AMALTHEA");

    C2NCOD[383] = 506;
    fstr::assign(C2NNAM.get_mut(383), b"HIMALIA");

    C2NCOD[384] = 507;
    fstr::assign(C2NNAM.get_mut(384), b"ELARA");

    C2NCOD[385] = 508;
    fstr::assign(C2NNAM.get_mut(385), b"PASIPHAE");

    C2NCOD[386] = 509;
    fstr::assign(C2NNAM.get_mut(386), b"SINOPE");

    C2NCOD[387] = 510;
    fstr::assign(C2NNAM.get_mut(387), b"LYSITHEA");

    C2NCOD[388] = 511;
    fstr::assign(C2NNAM.get_mut(388), b"CARME");

    C2NCOD[389] = 512;
    fstr::assign(C2NNAM.get_mut(389), b"ANANKE");

    C2NCOD[390] = 513;
    fstr::assign(C2NNAM.get_mut(390), b"LEDA");

    C2NCOD[391] = 514;
    fstr::assign(C2NNAM.get_mut(391), b"THEBE");

    C2NCOD[392] = 515;
    fstr::assign(C2NNAM.get_mut(392), b"ADRASTEA");

    C2NCOD[393] = 516;
    fstr::assign(C2NNAM.get_mut(393), b"METIS");

    C2NCOD[394] = 517;
    fstr::assign(C2NNAM.get_mut(394), b"CALLIRRHOE");

    C2NCOD[395] = 518;
    fstr::assign(C2NNAM.get_mut(395), b"THEMISTO");

    C2NCOD[396] = 519;
    fstr::assign(C2NNAM.get_mut(396), b"MEGACLITE");

    C2NCOD[397] = 520;
    fstr::assign(C2NNAM.get_mut(397), b"TAYGETE");

    C2NCOD[398] = 521;
    fstr::assign(C2NNAM.get_mut(398), b"CHALDENE");

    C2NCOD[399] = 522;
    fstr::assign(C2NNAM.get_mut(399), b"HARPALYKE");

    C2NCOD[400] = 523;
    fstr::assign(C2NNAM.get_mut(400), b"KALYKE");

    C2NCOD[401] = 524;
    fstr::assign(C2NNAM.get_mut(401), b"IOCASTE");

    C2NCOD[402] = 525;
    fstr::assign(C2NNAM.get_mut(402), b"ERINOME");

    C2NCOD[403] = 526;
    fstr::assign(C2NNAM.get_mut(403), b"ISONOE");

    C2NCOD[404] = 527;
    fstr::assign(C2NNAM.get_mut(404), b"PRAXIDIKE");

    C2NCOD[405] = 528;
    fstr::assign(C2NNAM.get_mut(405), b"AUTONOE");

    C2NCOD[406] = 529;
    fstr::assign(C2NNAM.get_mut(406), b"THYONE");

    C2NCOD[407] = 530;
    fstr::assign(C2NNAM.get_mut(407), b"HERMIPPE");

    C2NCOD[408] = 531;
    fstr::assign(C2NNAM.get_mut(408), b"AITNE");

    C2NCOD[409] = 532;
    fstr::assign(C2NNAM.get_mut(409), b"EURYDOME");

    C2NCOD[410] = 533;
    fstr::assign(C2NNAM.get_mut(410), b"EUANTHE");

    C2NCOD[411] = 534;
    fstr::assign(C2NNAM.get_mut(411), b"EUPORIE");

    C2NCOD[412] = 535;
    fstr::assign(C2NNAM.get_mut(412), b"ORTHOSIE");

    C2NCOD[413] = 536;
    fstr::assign(C2NNAM.get_mut(413), b"SPONDE");

    C2NCOD[414] = 537;
    fstr::assign(C2NNAM.get_mut(414), b"KALE");

    C2NCOD[415] = 538;
    fstr::assign(C2NNAM.get_mut(415), b"PASITHEE");

    C2NCOD[416] = 539;
    fstr::assign(C2NNAM.get_mut(416), b"HEGEMONE");

    C2NCOD[417] = 540;
    fstr::assign(C2NNAM.get_mut(417), b"MNEME");

    C2NCOD[418] = 541;
    fstr::assign(C2NNAM.get_mut(418), b"AOEDE");

    C2NCOD[419] = 542;
    fstr::assign(C2NNAM.get_mut(419), b"THELXINOE");

    C2NCOD[420] = 543;
    fstr::assign(C2NNAM.get_mut(420), b"ARCHE");

    C2NCOD[421] = 544;
    fstr::assign(C2NNAM.get_mut(421), b"KALLICHORE");

    C2NCOD[422] = 545;
    fstr::assign(C2NNAM.get_mut(422), b"HELIKE");

    C2NCOD[423] = 546;
    fstr::assign(C2NNAM.get_mut(423), b"CARPO");

    C2NCOD[424] = 547;
    fstr::assign(C2NNAM.get_mut(424), b"EUKELADE");

    C2NCOD[425] = 548;
    fstr::assign(C2NNAM.get_mut(425), b"CYLLENE");

    C2NCOD[426] = 549;
    fstr::assign(C2NNAM.get_mut(426), b"KORE");

    C2NCOD[427] = 550;
    fstr::assign(C2NNAM.get_mut(427), b"HERSE");

    C2NCOD[428] = 553;
    fstr::assign(C2NNAM.get_mut(428), b"DIA");

    C2NCOD[429] = 599;
    fstr::assign(C2NNAM.get_mut(429), b"JUPITER");

    C2NCOD[430] = 6;
    fstr::assign(C2NNAM.get_mut(430), b"SATURN BARYCENTER");

    C2NCOD[431] = 601;
    fstr::assign(C2NNAM.get_mut(431), b"MIMAS");

    C2NCOD[432] = 602;
    fstr::assign(C2NNAM.get_mut(432), b"ENCELADUS");

    C2NCOD[433] = 603;
    fstr::assign(C2NNAM.get_mut(433), b"TETHYS");

    C2NCOD[434] = 604;
    fstr::assign(C2NNAM.get_mut(434), b"DIONE");

    C2NCOD[435] = 605;
    fstr::assign(C2NNAM.get_mut(435), b"RHEA");

    C2NCOD[436] = 606;
    fstr::assign(C2NNAM.get_mut(436), b"TITAN");

    C2NCOD[437] = 607;
    fstr::assign(C2NNAM.get_mut(437), b"HYPERION");

    C2NCOD[438] = 608;
    fstr::assign(C2NNAM.get_mut(438), b"IAPETUS");

    C2NCOD[439] = 609;
    fstr::assign(C2NNAM.get_mut(439), b"PHOEBE");

    C2NCOD[440] = 610;
    fstr::assign(C2NNAM.get_mut(440), b"JANUS");

    C2NCOD[441] = 611;
    fstr::assign(C2NNAM.get_mut(441), b"EPIMETHEUS");

    C2NCOD[442] = 612;
    fstr::assign(C2NNAM.get_mut(442), b"HELENE");

    C2NCOD[443] = 613;
    fstr::assign(C2NNAM.get_mut(443), b"TELESTO");

    C2NCOD[444] = 614;
    fstr::assign(C2NNAM.get_mut(444), b"CALYPSO");

    C2NCOD[445] = 615;
    fstr::assign(C2NNAM.get_mut(445), b"ATLAS");

    C2NCOD[446] = 616;
    fstr::assign(C2NNAM.get_mut(446), b"PROMETHEUS");

    C2NCOD[447] = 617;
    fstr::assign(C2NNAM.get_mut(447), b"PANDORA");

    C2NCOD[448] = 618;
    fstr::assign(C2NNAM.get_mut(448), b"PAN");

    C2NCOD[449] = 619;
    fstr::assign(C2NNAM.get_mut(449), b"YMIR");

    C2NCOD[450] = 620;
    fstr::assign(C2NNAM.get_mut(450), b"PAALIAQ");

    C2NCOD[451] = 621;
    fstr::assign(C2NNAM.get_mut(451), b"TARVOS");

    C2NCOD[452] = 622;
    fstr::assign(C2NNAM.get_mut(452), b"IJIRAQ");

    C2NCOD[453] = 623;
    fstr::assign(C2NNAM.get_mut(453), b"SUTTUNGR");

    C2NCOD[454] = 624;
    fstr::assign(C2NNAM.get_mut(454), b"KIVIUQ");

    C2NCOD[455] = 625;
    fstr::assign(C2NNAM.get_mut(455), b"MUNDILFARI");

    C2NCOD[456] = 626;
    fstr::assign(C2NNAM.get_mut(456), b"ALBIORIX");

    C2NCOD[457] = 627;
    fstr::assign(C2NNAM.get_mut(457), b"SKATHI");

    C2NCOD[458] = 628;
    fstr::assign(C2NNAM.get_mut(458), b"ERRIAPUS");

    C2NCOD[459] = 629;
    fstr::assign(C2NNAM.get_mut(459), b"SIARNAQ");

    C2NCOD[460] = 630;
    fstr::assign(C2NNAM.get_mut(460), b"THRYMR");

    C2NCOD[461] = 631;
    fstr::assign(C2NNAM.get_mut(461), b"NARVI");

    C2NCOD[462] = 632;
    fstr::assign(C2NNAM.get_mut(462), b"METHONE");

    C2NCOD[463] = 633;
    fstr::assign(C2NNAM.get_mut(463), b"PALLENE");

    C2NCOD[464] = 634;
    fstr::assign(C2NNAM.get_mut(464), b"POLYDEUCES");

    C2NCOD[465] = 635;
    fstr::assign(C2NNAM.get_mut(465), b"DAPHNIS");

    C2NCOD[466] = 636;
    fstr::assign(C2NNAM.get_mut(466), b"AEGIR");

    C2NCOD[467] = 637;
    fstr::assign(C2NNAM.get_mut(467), b"BEBHIONN");

    C2NCOD[468] = 638;
    fstr::assign(C2NNAM.get_mut(468), b"BERGELMIR");

    C2NCOD[469] = 639;
    fstr::assign(C2NNAM.get_mut(469), b"BESTLA");

    C2NCOD[470] = 640;
    fstr::assign(C2NNAM.get_mut(470), b"FARBAUTI");

    C2NCOD[471] = 641;
    fstr::assign(C2NNAM.get_mut(471), b"FENRIR");

    C2NCOD[472] = 642;
    fstr::assign(C2NNAM.get_mut(472), b"FORNJOT");

    C2NCOD[473] = 643;
    fstr::assign(C2NNAM.get_mut(473), b"HATI");

    C2NCOD[474] = 644;
    fstr::assign(C2NNAM.get_mut(474), b"HYRROKKIN");

    C2NCOD[475] = 645;
    fstr::assign(C2NNAM.get_mut(475), b"KARI");

    C2NCOD[476] = 646;
    fstr::assign(C2NNAM.get_mut(476), b"LOGE");

    C2NCOD[477] = 647;
    fstr::assign(C2NNAM.get_mut(477), b"SKOLL");

    C2NCOD[478] = 648;
    fstr::assign(C2NNAM.get_mut(478), b"SURTUR");

    C2NCOD[479] = 649;
    fstr::assign(C2NNAM.get_mut(479), b"ANTHE");

    C2NCOD[480] = 650;
    fstr::assign(C2NNAM.get_mut(480), b"JARNSAXA");

    C2NCOD[481] = 651;
    fstr::assign(C2NNAM.get_mut(481), b"GREIP");

    C2NCOD[482] = 652;
    fstr::assign(C2NNAM.get_mut(482), b"TARQEQ");

    C2NCOD[483] = 653;
    fstr::assign(C2NNAM.get_mut(483), b"AEGAEON");

    C2NCOD[484] = 699;
    fstr::assign(C2NNAM.get_mut(484), b"SATURN");

    C2NCOD[485] = 7;
    fstr::assign(C2NNAM.get_mut(485), b"URANUS BARYCENTER");

    C2NCOD[486] = 701;
    fstr::assign(C2NNAM.get_mut(486), b"ARIEL");

    C2NCOD[487] = 702;
    fstr::assign(C2NNAM.get_mut(487), b"UMBRIEL");

    C2NCOD[488] = 703;
    fstr::assign(C2NNAM.get_mut(488), b"TITANIA");

    C2NCOD[489] = 704;
    fstr::assign(C2NNAM.get_mut(489), b"OBERON");

    C2NCOD[490] = 705;
    fstr::assign(C2NNAM.get_mut(490), b"MIRANDA");

    C2NCOD[491] = 706;
    fstr::assign(C2NNAM.get_mut(491), b"CORDELIA");

    C2NCOD[492] = 707;
    fstr::assign(C2NNAM.get_mut(492), b"OPHELIA");

    C2NCOD[493] = 708;
    fstr::assign(C2NNAM.get_mut(493), b"BIANCA");

    C2NCOD[494] = 709;
    fstr::assign(C2NNAM.get_mut(494), b"CRESSIDA");

    C2NCOD[495] = 710;
    fstr::assign(C2NNAM.get_mut(495), b"DESDEMONA");

    C2NCOD[496] = 711;
    fstr::assign(C2NNAM.get_mut(496), b"JULIET");

    C2NCOD[497] = 712;
    fstr::assign(C2NNAM.get_mut(497), b"PORTIA");

    C2NCOD[498] = 713;
    fstr::assign(C2NNAM.get_mut(498), b"ROSALIND");

    C2NCOD[499] = 714;
    fstr::assign(C2NNAM.get_mut(499), b"BELINDA");

    C2NCOD[500] = 715;
    fstr::assign(C2NNAM.get_mut(500), b"PUCK");

    C2NCOD[501] = 716;
    fstr::assign(C2NNAM.get_mut(501), b"CALIBAN");

    C2NCOD[502] = 717;
    fstr::assign(C2NNAM.get_mut(502), b"SYCORAX");

    C2NCOD[503] = 718;
    fstr::assign(C2NNAM.get_mut(503), b"PROSPERO");

    C2NCOD[504] = 719;
    fstr::assign(C2NNAM.get_mut(504), b"SETEBOS");

    C2NCOD[505] = 720;
    fstr::assign(C2NNAM.get_mut(505), b"STEPHANO");

    C2NCOD[506] = 721;
    fstr::assign(C2NNAM.get_mut(506), b"TRINCULO");

    C2NCOD[507] = 722;
    fstr::assign(C2NNAM.get_mut(507), b"FRANCISCO");

    C2NCOD[508] = 723;
    fstr::assign(C2NNAM.get_mut(508), b"MARGARET");

    C2NCOD[509] = 724;
    fstr::assign(C2NNAM.get_mut(509), b"FERDINAND");

    C2NCOD[510] = 725;
    fstr::assign(C2NNAM.get_mut(510), b"PERDITA");

    C2NCOD[511] = 726;
    fstr::assign(C2NNAM.get_mut(511), b"MAB");

    C2NCOD[512] = 727;
    fstr::assign(C2NNAM.get_mut(512), b"CUPID");

    C2NCOD[513] = 799;
    fstr::assign(C2NNAM.get_mut(513), b"URANUS");

    C2NCOD[514] = 8;
    fstr::assign(C2NNAM.get_mut(514), b"NEPTUNE BARYCENTER");

    C2NCOD[515] = 801;
    fstr::assign(C2NNAM.get_mut(515), b"TRITON");

    C2NCOD[516] = 802;
    fstr::assign(C2NNAM.get_mut(516), b"NEREID");

    C2NCOD[517] = 803;
    fstr::assign(C2NNAM.get_mut(517), b"NAIAD");

    C2NCOD[518] = 804;
    fstr::assign(C2NNAM.get_mut(518), b"THALASSA");

    C2NCOD[519] = 805;
    fstr::assign(C2NNAM.get_mut(519), b"DESPINA");

    C2NCOD[520] = 806;
    fstr::assign(C2NNAM.get_mut(520), b"GALATEA");

    C2NCOD[521] = 807;
    fstr::assign(C2NNAM.get_mut(521), b"LARISSA");

    C2NCOD[522] = 808;
    fstr::assign(C2NNAM.get_mut(522), b"PROTEUS");

    C2NCOD[523] = 809;
    fstr::assign(C2NNAM.get_mut(523), b"HALIMEDE");

    C2NCOD[524] = 810;
    fstr::assign(C2NNAM.get_mut(524), b"PSAMATHE");

    C2NCOD[525] = 811;
    fstr::assign(C2NNAM.get_mut(525), b"SAO");

    C2NCOD[526] = 812;
    fstr::assign(C2NNAM.get_mut(526), b"LAOMEDEIA");

    C2NCOD[527] = 813;
    fstr::assign(C2NNAM.get_mut(527), b"NESO");

    C2NCOD[528] = 899;
    fstr::assign(C2NNAM.get_mut(528), b"NEPTUNE");

    C2NCOD[529] = 9;
    fstr::assign(C2NNAM.get_mut(529), b"PLUTO BARYCENTER");

    C2NCOD[530] = 901;
    fstr::assign(C2NNAM.get_mut(530), b"CHARON");

    C2NCOD[531] = 902;
    fstr::assign(C2NNAM.get_mut(531), b"NIX");

    C2NCOD[532] = 903;
    fstr::assign(C2NNAM.get_mut(532), b"HYDRA");

    C2NCOD[533] = 904;
    fstr::assign(C2NNAM.get_mut(533), b"KERBEROS");

    C2NCOD[534] = 905;
    fstr::assign(C2NNAM.get_mut(534), b"STYX");

    C2NCOD[535] = 920000617;
    fstr::assign(C2NNAM.get_mut(535), b"PATROCLUS");

    C2NCOD[536] = 920003548;
    fstr::assign(C2NNAM.get_mut(536), b"EURYBATES");

    C2NCOD[537] = 920065803;
    fstr::assign(C2NNAM.get_mut(537), b"DIDYMOS");

    C2NCOD[538] = 9511010;
    fstr::assign(C2NNAM.get_mut(538), b"GASPRA");

    C2NCOD[539] = 999;
    fstr::assign(C2NNAM.get_mut(539), b"PLUTO");

    testutil::TOPEN(b"F_ZZBODS", ctx)?;

    //
    // Test the BODN2C call against the known body list.
    //
    for I in 1..=NPERM {
        fstr::assign(
            &mut TSTSTR,
            &fstr::concat(b"Check known name/ID code pairs, BODN2C: ", N2CNAM.get(I)),
        );
        testutil::TCASE(&TSTSTR, ctx)?;

        spicelib::BODN2C(&N2CNAM[I], &mut CODE, &mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSI(b"CODE", CODE, b"=", N2CCOD[I], 0, OK, ctx)?;
    }

    //
    // Test the BODC2N call against the known body list.
    // This body list removed duplicate ID -> NAME assignments,
    // preserving the expected mapping precedence for IDs
    // mapped to multiple names.
    //
    for I in 1..=539 {
        fstr::assign(&mut TSTSTR, b"Check known name/ID code pairs, BODC2N: #");
        spicelib::REPMI(&TSTSTR.clone(), b"#", C2NCOD[I], &mut TSTSTR, ctx);
        testutil::TCASE(&TSTSTR, ctx)?;

        spicelib::BODC2N(C2NCOD[I], &mut NAME, &mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"NAME", &NAME, b"=", &C2NNAM[I], OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
