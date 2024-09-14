//! Fill a buffer with a repeated byte pattern.

/// Fill `dst` with `src`.
///
///  - If `src.len()` is smaller than `dst.len()`, `src` will be repeated.
///  - If `src.len()` is greater than `dst.len()`, `src` will be truncated.
///
/// # Examples
///
/// ```
/// use memset_pattern::memset_pattern;
///
/// let src = [0xaa, 0xbb, 0xcc];
/// let mut dst = [0; 7];
///
/// memset_pattern(&mut dst, &src);
///
/// assert_eq!(dst, [0xaa, 0xbb, 0xcc, 0xaa, 0xbb, 0xcc, 0xaa]);
/// ```
#[allow(clippy::match_overlapping_arm)]
pub fn memset_pattern(mut dst: &mut [u8], src: &[u8]) {
    match src.len() {
        0 => panic!(),
        1 => dst.fill(src[0]),
        2 => memset_pattern_power_of_2::<2>(dst, src.try_into().unwrap()),
        4 => memset_pattern_power_of_2::<4>(dst, src.try_into().unwrap()),
        8 => memset_pattern_power_of_2::<8>(dst, src.try_into().unwrap()),
        16 => memset_pattern_power_of_2::<16>(dst, src.try_into().unwrap()),
        32 => memset_pattern_power_of_2::<32>(dst, src.try_into().unwrap()),
        64 => memset_pattern_power_of_2::<64>(dst, src.try_into().unwrap()),
        128 => memset_pattern_power_of_2::<128>(dst, src.try_into().unwrap()),
        ..=16 => memset_pattern_block::<16>(dst, src),
        ..=32 => memset_pattern_block::<32>(dst, src),
        _ => {
            while dst.len() >= src.len() {
                dst[..src.len()].copy_from_slice(src);
                dst = &mut dst[src.len()..];
            }

            let remaining = dst.len();
            dst.copy_from_slice(&src[..remaining]);
        }
    }
}

fn memset_pattern_power_of_2<const N: usize>(mut dst: &mut [u8], src: &[u8; N]) {
    while dst.len() >= N {
        dst[..N].copy_from_slice(src);
        dst = &mut dst[N..];
    }

    let remaining = dst.len();
    dst.copy_from_slice(&src[..remaining]);
}

fn memset_pattern_block<const N: usize>(mut dst: &mut [u8], src: &[u8]) {
    let mut new_src = [0; N];
    let mut new_src_len = 0;

    while new_src_len <= new_src.len() - src.len() {
        new_src[new_src_len..new_src_len + src.len()].copy_from_slice(src);
        new_src_len += src.len();
    }

    while dst.len() >= N {
        dst[..N].copy_from_slice(&new_src);
        dst = &mut dst[new_src_len..];
    }

    let remaining = dst.len();
    dst.copy_from_slice(&new_src[..remaining]);
}
