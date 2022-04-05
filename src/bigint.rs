use crate::{ModularCoreOps, ModularPow, ModularUnaryOps, ModularSymbols};
use core::convert::TryInto;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};

// Forward modular operations to ref by ref
macro_rules! impl_mod_ops_by_ref {
    ($T:ty) => {
        // core ops
        impl ModularCoreOps<$T, &$T> for &$T {
            type Output = $T;
            #[inline]
            fn addm(self, rhs: $T, m: &$T) -> $T {
                self.addm(&rhs, &m)
            }
            #[inline]
            fn subm(self, rhs: $T, m: &$T) -> $T {
                self.subm(&rhs, &m)
            }
            #[inline]
            fn mulm(self, rhs: $T, m: &$T) -> $T {
                self.mulm(&rhs, &m)
            }
        }
        impl ModularCoreOps<&$T, &$T> for $T {
            type Output = $T;
            #[inline]
            fn addm(self, rhs: &$T, m: &$T) -> $T {
                (&self).addm(rhs, &m)
            }
            #[inline]
            fn subm(self, rhs: &$T, m: &$T) -> $T {
                (&self).subm(rhs, &m)
            }
            #[inline]
            fn mulm(self, rhs: &$T, m: &$T) -> $T {
                (&self).mulm(rhs, &m)
            }
        }
        impl ModularCoreOps<$T, &$T> for $T {
            type Output = $T;
            #[inline]
            fn addm(self, rhs: $T, m: &$T) -> $T {
                (&self).addm(&rhs, &m)
            }
            #[inline]
            fn subm(self, rhs: $T, m: &$T) -> $T {
                (&self).subm(&rhs, &m)
            }
            #[inline]
            fn mulm(self, rhs: $T, m: &$T) -> $T {
                (&self).mulm(&rhs, &m)
            }
        }

        // pow
        impl ModularPow<$T, &$T> for &$T {
            type Output = $T;
            #[inline]
            fn powm(self, exp: $T, m: &$T) -> $T {
                self.powm(&exp, &m)
            }
        }
        impl ModularPow<&$T, &$T> for $T {
            type Output = $T;
            #[inline]
            fn powm(self, exp: &$T, m: &$T) -> $T {
                (&self).powm(exp, &m)
            }
        }
        impl ModularPow<$T, &$T> for $T {
            type Output = $T;
            #[inline]
            fn powm(self, exp: $T, m: &$T) -> $T {
                (&self).powm(&exp, &m)
            }
        }

        // unary ops and symbols
        impl ModularUnaryOps<&$T> for $T {
            type Output = $T;
            #[inline]
            fn negm(self, m: &$T) -> $T {
                ModularUnaryOps::<&$T>::negm(&self, m)
            }
            #[inline]
            fn invm(self, m: &$T) -> Option<$T> {
                ModularUnaryOps::<&$T>::invm(&self, m)
            }
        }
    };
}

#[cfg(feature = "num-bigint")]
mod _num_bigint {
    use super::*;
    use num_bigint::BigUint;

    impl ModularCoreOps<&BigUint, &BigUint> for &BigUint {
        type Output = BigUint;

        #[inline]
        fn addm(self, rhs: &BigUint, m: &BigUint) -> BigUint {
            (self + rhs) % m
        }
        fn subm(self, rhs: &BigUint, m: &BigUint) -> BigUint {
            let (lhs, rhs) = (self % m, rhs % m);
            if lhs >= rhs {
                lhs - rhs
            } else {
                m - (rhs - lhs)
            }
        }

        fn mulm(self, rhs: &BigUint, m: &BigUint) -> BigUint {
            let a = self % m;
            let b = rhs % m;

            if let Some(sm) = m.to_u64() {
                let sself = a.to_u64().unwrap();
                let srhs = b.to_u64().unwrap();
                return BigUint::from(sself.mulm(srhs, &sm));
            }

            (a * b) % m
        }
    }

    impl ModularUnaryOps<&BigUint> for &BigUint {
        type Output = BigUint;
        #[inline]
        fn negm(self, m: &BigUint) -> BigUint {
            let x = self % m;
            if x.is_zero() {
                BigUint::zero()
            } else {
                m - x
            }
        }

        fn invm(self, m: &BigUint) -> Option<Self::Output> {
            let x = if self >= m { self % m } else { self.clone() };

            let (mut last_r, mut r) = (m.clone(), x);
            let (mut last_t, mut t) = (BigUint::zero(), BigUint::one());

            while r > BigUint::zero() {
                let (quo, rem) = last_r.div_rem(&r);
                last_r = r;
                r = rem;

                let new_t = last_t.subm(&quo.mulm(&t, m), m);
                last_t = t;
                t = new_t;
            }

            // if r = gcd(self, m) > 1, then inverse doesn't exist
            if last_r > BigUint::one() {
                None
            } else {
                Some(last_t)
            }
        }
    }

    impl ModularPow<&BigUint, &BigUint> for &BigUint {
        type Output = BigUint;
        #[inline]
        fn powm(self, exp: &BigUint, m: &BigUint) -> BigUint {
            self.modpow(&exp, m)
        }
    }

    impl ModularSymbols<&BigUint> for BigUint {
        #[inline]
        fn checked_legendre(&self, n: &BigUint) -> Option<i8> {
            let r = self.powm((n - 1u8) >> 1u8, &n);
            if r.is_zero() {
                Some(0)
            } else if r.is_one() {
                Some(1)
            } else if &(r + 1u8) == n {
                Some(-1)
            } else {
                None
            }
        }

        fn checked_jacobi(&self, n: &BigUint) -> Option<i8> {
            if n.is_even() {
                return None;
            }
            if self.is_zero() {
                return Some(0);
            }
            if self.is_one() {
                return Some(1);
            }

            let three = BigUint::from(3u8);
            let five = BigUint::from(5u8);
            let seven = BigUint::from(7u8);

            let mut a = self % n;
            let mut n = n.clone();
            let mut t = 1;
            while a > BigUint::zero() {
                while a.is_even() {
                    a >>= 1;
                    if &n & &seven == three || &n & &seven == five {
                        t *= -1;
                    }
                }
                core::mem::swap(&mut a, &mut n);
                if (&a & &three) == three && (&n & &three) == three {
                    t *= -1;
                }
                a %= &n;
            }
            Some(if n.is_one() {
                t
            } else {
                0
            })
        }

        #[inline]
        fn kronecker(&self, n: &BigUint) -> i8 {
            if n.is_zero() {
                return if self.is_one() { 1 } else { 0 };
            }
            if n.is_one() {
                return 1;
            }
            if n == &BigUint::from(2u8) {
                return if self.is_even() {
                    0
                } else {
                    let seven = BigUint::from(7u8);
                    if (self & &seven).is_one() || self & &seven == seven {
                        return 1;
                    } else {
                        return -1;
                    }
                };
            }

            let f = n.trailing_zeros().unwrap_or(0);
            let n = n >> f;
            let t1 = self.kronecker(&BigUint::from(2u8));
            let t2 = self.jacobi(&n);
            t1.pow(f.try_into().unwrap()) * t2
        }
    }

    impl_mod_ops_by_ref!(BigUint);

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::random;

        #[test]
        fn biguint_basic_mod_test() {
            let a = random::<u128>();
            let ra = &BigUint::from(a);
            let m = random::<u128>();
            let rm = &BigUint::from(m);
            assert_eq!(ra.addm(ra, rm), (ra + ra) % rm);
            assert_eq!(ra.mulm(ra, rm), (ra * ra) % rm);
            assert_eq!(ra.powm(BigUint::from(3u8), rm), ra.pow(3) % rm);
        }
    }
}
