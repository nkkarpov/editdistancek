#![deny(missing_docs)]

//! # Edit Distance
//! Find the Levenshtein edit distance between `s` and `t`.

use std::cmp::{max, min};

/// Returns edit distance between `s` and `t`.
pub fn edit_distance(s: &[u8], t: &[u8]) -> usize {
    return edit_distance_k(s, t, max(s.len(), t.len())).unwrap();
}

/// If edit distance `d` between `s` and `t` is at most `k`, then returns `Some(d)` otherwise returns `None`.
pub fn edit_distance_k(s: &[u8], t: &[u8], k: usize) -> Option<usize> {
    let (s, t, s_length, t_length) = if s.len() > t.len() {
        (t, s, t.len(), s.len())
    } else {
        (s, t, s.len(), t.len())
    };
    let diff = t_length - s_length;
    if diff > k {
        None
    } else {
        let shift = k + 1;
        let (mut a, mut b) = (vec![-1isize; 2 * k + 3], vec![-1isize; 2 * k + 3]);

        for h in 0..=k {
            let (a, b) = if (h & 1) == 0 {
                (&b, &mut a)
            } else {
                (&a, &mut b)
            };
            let (p, q) = (
                shift - min(1 + (k - diff) / 2, h),
                shift + min(1 + k / 2 + diff, h),
            );
            for i in p..=q {
                b[i] = {
                    let r = (max(max(a[i - 1], a[i] + 1), a[i + 1] + 1)) as usize;
                    if r >= s_length || r + i - shift >= t_length {
                        r
                    } else {
                        mismatch(&s[r..], &t[(r + i - shift)..]) + r
                    }
                } as isize;
                if i + s_length == t_length + shift && b[i] as usize >= s_length {
                    return Some(h);
                }
            }
        }
        None
    }
}

/// Returns the length of longest common prefix `s` and `t` (uses SIMD if it is possible).
#[inline(always)]
pub fn mismatch(s: &[u8], t: &[u8]) -> usize {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return mismatch_fast(s, t);
    }
    #[allow(unreachable_code)]
    {
        mismatch_naive(s, t)
    }
}

/// Returns the length of longest common prefix `s` and `t` (with SIMD optimizations).
#[inline(always)]
#[allow(dead_code)]
pub fn mismatch_fast(s: &[u8], t: &[u8]) -> usize {
    let l = s.len().min(t.len());
    let mut xs = &s[..l];
    let mut ys = &t[..l];
    let mut off = 0;

    unsafe {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        while xs.len() >= 16 {
            let x = _mm_loadu_si128(xs.as_ptr() as _);
            let y = _mm_loadu_si128(ys.as_ptr() as _);
            let r = _mm_cmpeq_epi8(x, y);
            let r = _mm_movemask_epi8(r);
            if r != 65535 {
                return off + r.trailing_ones() as usize;
            }
            xs = &xs[16..];
            ys = &ys[16..];
            off += 16;
        }
    }
    off + mismatch_naive(xs, ys)
}

fn mismatch_naive(s: &[u8], t: &[u8]) -> usize {
    s.iter().zip(t).take_while(|(x, y)| x == y).count()
}
