use proptest::prelude::*;
use pakx::util::{pack_scalar, unpack_scalar, Endian};

fn mask(bits: u32, x: i128) -> i128 {
    if bits == 128 { x } else {
        let m = (1i128 << bits) - 1;
        x & m
    }
}

proptest! {
    #[test]
    fn roundtrip_unsigned(width in prop_oneof![Just(8u32), Just(16), Just(32), Just(64), Just(128)],
                          be in any::<bool>(),
                          x in any::<i128>()) {
        let endian = if be { Endian::Big } else { Endian::Little };
        let b = pack_scalar(x, width, endian, /*signed*/ false, /*strict*/ false).unwrap();
        let y = unpack_scalar(&b, width, endian, /*signed*/ false);
        prop_assert_eq!(y, mask(width, x));
    }

    #[test]
    fn roundtrip_signed_in_range(width in prop_oneof![Just(8u32), Just(16), Just(32), Just(64)],
                                 be in any::<bool>(),
                                 x in any::<i32>()) {
        let bits = width;
        let min = -((1i128) << (bits - 1));
        let max = ((1i128) << (bits - 1)) - 1;
        let xi = (x as i128).clamp(min, max);

        let endian = if be { Endian::Big } else { Endian::Little };
        let b = pack_scalar(xi, width, endian, /*signed*/ true, /*strict*/ true).unwrap();
        let y = unpack_scalar(&b, width, endian, /*signed*/ true);
        prop_assert_eq!(y, xi);
    }
}
