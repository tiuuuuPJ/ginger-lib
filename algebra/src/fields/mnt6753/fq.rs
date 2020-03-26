use crate::{
    biginteger::BigInteger768 as BigInteger,
    fields::{Fp768, Fp768Parameters, FpParameters},
};

pub type Fq = Fp768<FqParameters>;

pub struct FqParameters;

impl Fp768Parameters for FqParameters {}
impl FpParameters for FqParameters {
    type BigInt = BigInteger;

    const MODULUS: BigInteger = BigInteger([
        0xD90776E240000001,
        0x4EA099170FA13A4F,
        0xD6C381BC3F005797,
        0xB9DFF97634993AA4,
        0x3EEBCA9429212636,
        0xB26C5C28C859A99B,
        0x99D124D9A15AF79D,
        0x07FDB925E8A0ED8D,
        0x5EB7E8F96C97D873,
        0xB7F997505B8FAFED,
        0x10229022EEE2CDAD,
        0x01C4C62D92C411,
    ]);

    const MODULUS_BITS: u32 = 753;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 15;

    const R: BigInteger = BigInteger([
        0xB99680147FFF6F42,
        0x4EB16817B589CEA8,
        0xA1EBD2D90C79E179,
        0x0F725CAEC549C0DA,
        0xAB0C4EE6D3E6DAD4,
        0x9FBCA908DE0CCB62,
        0x320C3BB713338498,
        0x598B4302D2F00A62,
        0x4074C9CBFD8CA621,
        0x0FA47EDB3865E88C,
        0x95455FB31FF9A195,
        0x7B479EC8E242,
    ]);

    const R2: BigInteger = BigInteger([
        0x3F9C69C7B7F4C8D1,
        0x70A50FA9EE48D127,
        0xCDBE6702009569CB,
        0x6BD8C6C6C49EDC38,
        0x7955876CC35EE94E,
        0xC7285529BE54A3F4,
        0xDED52121ECEC77CF,
        0x99BE80F2EE12EE8E,
        0xC8A0FF01493BDCEF,
        0xACC27988F3D9A316,
        0xD9E817A8FB44B3C9,
        0x5B58037E0E4,
    ]);

    const INV: u64 = 0xC90776E23FFFFFFF;

    const GENERATOR: BigInteger = BigInteger([
        0xEEE0A5D37FF6635E,
        0xFF458536CFA1CFF4,
        0x659AF978D8169AB0,
        0x1F1841C24780E3F1,
        0x602213036DCFEF3A,
        0xD1D5C8F39D72DB20,
        0xEB8B63C1C0FFEFAB,
        0xD2488E985F6CFA4E,
        0xCCE1C2A623F7A66A,
        0x2A060F4D5085B19A,
        0xA9111A596408842F,
        0x11CA8D50BF627,
    ]);

    const TWO_ADICITY: u32 = 30;

    const ROOT_OF_UNITY: BigInteger = BigInteger([
        0x307F66B297671883,
        0xD72A7F2B1E645F4E,
        0x67079DAA9A902283,
        0xF33F7620A86C668B,
        0x8878570D66464C12,
        0xA557AF5B524F522B,
        0x5FAFA3F6EF19319D,
        0x1EB9E04110A65629,
        0x3F96FEB3C639A0B0,
        0x4D4FE37DF3FFD732,
        0xADC831BD55BCF3E9,
        0x1B9F32A8BD6AB,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xEC83BB7120000000,
        0xA7504C8B87D09D27,
        0x6B61C0DE1F802BCB,
        0x5CEFFCBB1A4C9D52,
        0x9F75E54A1490931B,
        0xD9362E14642CD4CD,
        0xCCE8926CD0AD7BCE,
        0x83FEDC92F45076C6,
        0xAF5BF47CB64BEC39,
        0xDBFCCBA82DC7D7F6,
        0x88114811777166D6,
        0xE26316C96208,
    ]);

    const T: BigInteger = BigInteger([
        0x3E84E93F641DDB89,
        0xFC015E5D3A82645C,
        0xD264EA935B0E06F0,
        0xA48498DAE77FE5D8,
        0x2166A66CFBAF2A50,
        0x856BDE76C9B170A3,
        0xA283B63667449366,
        0xB25F61CC1FF6E497,
        0x6E3EBFB57ADFA3E5,
        0xBB8B36B6DFE65D41,
        0xB64B1044408A408B,
        0x71318,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x1F42749FB20EEDC4,
        0x7E00AF2E9D41322E,
        0x69327549AD870378,
        0x52424C6D73BFF2EC,
        0x90B353367DD79528,
        0x42B5EF3B64D8B851,
        0xD141DB1B33A249B3,
        0xD92FB0E60FFB724B,
        0xB71F5FDABD6FD1F2,
        0xDDC59B5B6FF32EA0,
        0x5B25882220452045,
        0x3898C,
    ]);
}
