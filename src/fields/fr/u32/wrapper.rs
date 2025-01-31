use super::fiat;

const B: usize = 251;
const N_64: usize = (B + 63) / 64;
const N_8: usize = (B + 7) / 8;
const N: usize = (B + 31) / 32;

#[derive(Copy, Clone)]
pub struct Fr(pub fiat::FrMontgomeryDomainFieldElement);

impl PartialEq for Fr {
    fn eq(&self, other: &Self) -> bool {
        let sub = self.sub(other);
        let mut check_word = 0;
        fiat::fr_nonzero(&mut check_word, &sub.0 .0);
        check_word == 0
    }
}

impl Eq for Fr {}

impl zeroize::Zeroize for Fr {
    fn zeroize(&mut self) {
        self.0 .0.zeroize()
    }
}

impl Fr {
    pub fn from_le_limbs(limbs: [u64; N_64]) -> Fr {
        let limbs = {
            let mut out = [0u32; N];
            for i in 0..N_64 {
                out[2 * i] = (limbs[i] & 0xFFFF_FFFF_FFFF_FFFF) as u32;
                out[2 * i + 1] = (limbs[i] >> 32) as u32;
            }
            out
        };
        let x_non_monty = fiat::FrNonMontgomeryDomainFieldElement(limbs);
        let mut x = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_to_montgomery(&mut x, &x_non_monty);
        Self(x)
    }

    pub fn from_raw_bytes(bytes: &[u8; N_8]) -> Fr {
        let mut x_non_montgomery = fiat::FrNonMontgomeryDomainFieldElement([0; N]);
        let mut x = fiat::FrMontgomeryDomainFieldElement([0; N]);

        fiat::fr_from_bytes(&mut x_non_montgomery.0, &bytes);
        fiat::fr_to_montgomery(&mut x, &x_non_montgomery);

        Self(x)
    }

    pub fn to_le_limbs(&self) -> [u64; N_64] {
        let mut x_non_montgomery = fiat::FrNonMontgomeryDomainFieldElement([0; N]);
        fiat::fr_from_montgomery(&mut x_non_montgomery, &self.0);
        let limbs = x_non_montgomery.0;
        let mut out = [0u64; N_64];
        for i in 0..N_64 {
            out[i] = (limbs[2 * i] as u64) | ((limbs[2 * i + 1] as u64) << 32);
        }
        out
    }

    pub fn to_bytes_le(&self) -> [u8; N_8] {
        let mut bytes = [0u8; N_8];
        let mut x_non_montgomery = fiat::FrNonMontgomeryDomainFieldElement([0; N]);
        fiat::fr_from_montgomery(&mut x_non_montgomery, &self.0);
        fiat::fr_to_bytes(&mut bytes, &x_non_montgomery.0);
        bytes
    }

    pub const fn from_montgomery_limbs(limbs: [u32; N]) -> Fr {
        Self(fiat::FrMontgomeryDomainFieldElement(limbs))
    }

    pub const fn zero() -> Fr {
        Self(fiat::FrMontgomeryDomainFieldElement([0; N]))
    }

    pub const fn one() -> Self {
        Self(fiat::FrMontgomeryDomainFieldElement([
            3498574902, 3872500570, 2604314180, 2497411308, 588265454, 3867012838, 3735373809,
            66463618,
        ]))
    }

    pub fn square(&self) -> Fr {
        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_square(&mut result, &self.0);
        Self(result)
    }

    pub fn inverse(&self) -> Option<Fr> {
        if self == &Fr::zero() {
            return None;
        }

        const I: usize = (49 * B + 57) / 17;

        let mut a = fiat::FrNonMontgomeryDomainFieldElement([0; N]);
        fiat::fr_from_montgomery(&mut a, &self.0);
        let mut d = 1;
        let mut f: [u32; N + 1] = [0u32; N + 1];
        fiat::fr_msat(&mut f);
        let mut g: [u32; N + 1] = [0u32; N + 1];
        let mut v: [u32; N] = [0u32; N];
        let mut r: [u32; N] = Fr::one().0 .0;
        let mut i = 0;
        let mut j = 0;

        while j < N {
            g[j] = a[j];
            j += 1;
        }

        let mut out1: u32 = 0;
        let mut out2: [u32; N + 1] = [0; N + 1];
        let mut out3: [u32; N + 1] = [0; N + 1];
        let mut out4: [u32; N] = [0; N];
        let mut out5: [u32; N] = [0; N];
        let mut out6: u32 = 0;
        let mut out7: [u32; N + 1] = [0; N + 1];
        let mut out8: [u32; N + 1] = [0; N + 1];
        let mut out9: [u32; N] = [0; N];
        let mut out10: [u32; N] = [0; N];

        while i < I - I % 2 {
            fiat::fr_divstep(
                &mut out1, &mut out2, &mut out3, &mut out4, &mut out5, d, &f, &g, &v, &r,
            );
            fiat::fr_divstep(
                &mut out6, &mut out7, &mut out8, &mut out9, &mut out10, out1, &out2, &out3, &out4,
                &out5,
            );
            d = out6;
            f = out7;
            g = out8;
            v = out9;
            r = out10;
            i += 2;
        }

        if I % 2 != 0 {
            fiat::fr_divstep(
                &mut out1, &mut out2, &mut out3, &mut out4, &mut out5, d, &f, &g, &v, &r,
            );
            v = out4;
            f = out2;
        }

        let s = ((f[f.len() - 1] >> (32 - 1)) & 1) as u8;
        let mut neg = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_opp(&mut neg, &fiat::FrMontgomeryDomainFieldElement(v));

        let mut v_prime: [u32; N] = [0u32; N];
        fiat::fr_selectznz(&mut v_prime, s, &v, &neg.0);

        let mut pre_comp: [u32; N] = [0u32; N];
        fiat::fr_divstep_precomp(&mut pre_comp);

        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_mul(
            &mut result,
            &fiat::FrMontgomeryDomainFieldElement(v_prime),
            &fiat::FrMontgomeryDomainFieldElement(pre_comp),
        );

        Some(Fr(result))
    }

    pub fn add(self, other: &Fr) -> Fr {
        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_add(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn sub(self, other: &Fr) -> Fr {
        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_sub(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn mul(self, other: &Fr) -> Fr {
        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_mul(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn neg(self) -> Fr {
        let mut result = fiat::FrMontgomeryDomainFieldElement([0; N]);
        fiat::fr_opp(&mut result, &self.0);
        Fr(result)
    }
}
