//! Zero knowledge range proofs
//!
//! # Introduction
//!
//! The system consists of 3 following range proofs:
//! * Alice proves to Bob that her secret share $`a`$ is less than a certain threshold value $`K`$,
//! * Bob proves to Alice that his secret share $`b`$ is less than a certain threshold value $`K`$,
//! * Bob proves to Alice that he knows a secret $`b`$ for $`B = g^{b}`$ and $`c_B = b c_A + E_A(\beta')`$
//!
//! # Range proof by Alice
//!
//! Alice proves to Bob that she knows $`m \in Z_q`$ and $`r \in Z^*_N`$ such that $`c = \Gamma^mr^N \mod N^2`$, where $`q`$ is the order of the DSA group. At the end of the proof, Bob is convinced that $`m \in [-q^3, q^3]`$.
//!
//! The input for the proof:
//! * Paillier public key $`N, \Gamma`$,
//! * the ciphertext $`c \in Z_{N^2}`$.
//!
//!
//! The prover knows
//!
//! * $`m \in Z_q`$, which is her multiplicative share $`a \in Z_q`$,
//! * $`r \in Z^*_N`$ which is the random used in the encryption of $`a`$,
//! * such that $`c = \Gamma^m r^N \mod N^2`$ which is equal to $`E_A(a)`$
//!
//! ## Data types used
//!
//! [`AliceZkpInit`](struct.AliceZkpInit.html), [`AliceProof`](struct.AliceProof.html), [`AliceZkpRound1`](struct.AliceZkpRound1.html), [`AliceZkpRound2`](struct.AliceZkpRound2.html)
//!
//! ## Algorithm
//!
//! ```math
//! \begin{array}{lcl}
//! \textrm{\underline{Prover (Alice)}}                                                   &  & \textrm{\underline{Verifier (Bob)}} \\ \\
//! \textrm{samples} & & \\
//! \quad \alpha \in_R Z_{q^3} &  & \\
//! \quad \beta \in_R Z^*_{N}  &  & \\
//! \quad \gamma \in_R Z_{q^3\tilde{N}} & & \\
//! \quad \rho \in_R Z_{q\tilde{N}} & & \\ \\
//! \textrm{computes} & & \\
//! \quad z = {h_1^m h_2^{\rho}} \mod {\tilde{N}} & & \\
//! \quad u = \Gamma^{\alpha} \beta^{N} \mod {N^2} & & \\
//! \quad w = h_1^{\alpha} h_2^{\gamma}  \mod {\tilde{N}} & \xrightarrow{\hspace{10pt} z,u,w \hspace{10pt}} & \\
//!                                & & \textrm{selects} \\
//!                                & \xleftarrow{\hspace{18pt} e \hspace{18pt}} & \quad e \in_R Z_q   \\
//!  \textrm{computes}             &  & \\
//!  \quad s = r^e \beta \mod{N}   &  & \\
//!  \quad s_1 = em + \alpha       &  & \\
//!  \quad s_2 = e\rho + \gamma    & \xrightarrow{\hspace{9pt}s, s_1, s_2\hspace{9pt}} & \\
//!                                & & \textrm{verifies} \\
//!                                & & \quad s_1 \stackrel{?}{\leq} q^3   \\
//!                                & & \quad u \stackrel{?}{=} \Gamma^{s_1} s^N c^{-e} \mod {N^2}  \\
//!                                & & \quad h_1^{s_1} h_2^{s_2} z^{-e} \stackrel{?}{=} w \mod {\tilde{N}}  \\
//! \end{array}
//! ```
//! The implementation uses non-interactive proof where the challenge $`e`$ is computed by the prover as $` \textrm{SHA512/256}(N || \Gamma || c || z || u || w) \mod{q} `$
//!
//! # Range proof for `MtA` by Bob
//!
//! Bob proves to Alice that he knows $` x \in Z_q `$, $` y \in Z_N `$ and $` r \in Z^*_N `$ such that $`c_2 = c_1^x \Gamma^yr^N \mod N^2`$  , where $` q `$ is the order of the DSA group.
//! At the end of the proof, Alice is convinced that $` x \in [-q^3, q^3] `$.
//!
//! The input for the proof:
//!
//! * Paillier public key $` N, \Gamma `$
//! * the ciphertexts $` c_1, c_2 \in Z_{N^2} `$.
//!
//! The prover knows
//! * $`x \in Z_q`$, which is his multiplicative share $`b \in Z_q`$,
//! * $`y \in Z_N`$, which is $`\beta^{\prime} \in Z_N`$ in `MtA`,
//! * $`r \in Z^*_N`$ which is the random used in the encryption of $`\beta^{\prime}`$,
//! such that $`c_2 = c_1^x\Gamma^yr^N \mod N^2`$ which is equal to $`c_B = bc_A + E_A(\beta') \mod N^2`$.
//!
//! ## Data types used
//!
//! [`BobZkpInit`](struct.BobZkpInit.html), [`BobProof`](struct.BobProof.html), [`BobZkpRound1`](struct.BobZkpRound1.html), [`BobZkpRound2`](struct.BobZkpRound2.html)
//!
//! ## Algorithm

//! ```math
//! \begin{array}{lcl}
//! \textrm{\underline{Prover (Bob)}}                                                   &  & \textrm{\underline{Verifier (Alice)}} \\ \\
//! \textrm{samples}                                     &  &                      \\
//! \quad \alpha \in_R Z_{q^3}                                 &  &                      \\
//! \quad \rho \in_R Z_{q\tilde{N}}                            &  &                      \\
//! \quad \rho' \in_R Z_{q^3\tilde{N}}                         &  &                      \\
//! \quad \sigma \in_R Z_{q\tilde{N}}                          &  &                      \\
//! \quad \beta \in_R Z^*_N                                    &  &                      \\
//! \quad \gamma \in_R Z^*_N                                   &  &                      \\
//! \quad \tau \in_R Z_{q\tilde{N}}                            &  &                      \\ \\
//! \textrm{computes}                                    &  &                      \\
//! \quad z = h_1^xh_2^{\rho} \mod {\tilde{N}}                   &  &                      \\
//! \quad z' = h_1^{\alpha} h_2^{\rho'} \mod {\tilde{N}}         &  &                      \\
//! \quad t = h_1^{y} h_2^{\sigma} \mod {\tilde{N}}              &  &                      \\
//! \quad v = c_1^{\alpha} \Gamma^{\gamma} \beta^{N} \mod {N^2}  &  &                      \\
//! \quad w = h_1^{\gamma} h_2^{\tau}  \mod {\tilde{N}}         & \xrightarrow{\hspace{5pt} z, z', t, v, w  \hspace{5pt}} &     \\
//! &                                     & \textrm{selects}                 \\
//! & \xleftarrow{\hspace{18pt} e \hspace{18pt}}  & \quad e \in_R Z_q              \\
//! \textrm{computes}                                                                           &  &                     \\
//! \quad s = r^e \beta \mod N                                 &  &                      \\
//! \quad s_1 = ex + \alpha                                    &  &                      \\
//! \quad s_2 = e\rho + \rho'                                  &  &                      \\
//! \quad t_1 = ey + \gamma                                    &  &                      \\
//! \quad t_2 = e\sigma + \tau                                 & \xrightarrow{\hspace{7pt}s, s_1, s_2, t_1, t_2\hspace{7pt}}  &   \\
//! &                                     & \textrm{checks}                 \\
//! &                                     & \quad s_1 \stackrel{?}{\leq} q^3                 \\
//! &                                     & \quad h_1^{s_1} h_2^{s_2}  \stackrel{?}{=} z^{e}z'  \mod {\tilde{N}}     \\
//! &                                     & \quad h_1^{t_1} h_2^{t_2}  \stackrel{?}{=} t^{e}w  \mod {\tilde{N}}     \\
//! &                                     & \quad c_1^{s_1} s^{N} \Gamma^{t_1}  \stackrel{?}{=} c_2^{e}v  \mod N^2     \\
//! \end{array}
//! ```
//!
//!The implementation uses non-interactive proof where the challenge $`e`$ is computed by the prover as $` \textrm{SHA512/256}(N || \Gamma || c_1 || c_2  || z || z' || t || v || w ) \mod{q} `$
//!
//!
//! # Range proof for `MtAwc` by Bob
//!
//! Bob proves to Alice that he knows $`x \in Z_q`$, $`y \in Z_N`$, and $`r \in Z^*_N`$ such that $`c_2 = c_1^x \Gamma^yr^N \mod N^2`$ and $`X = g^x \in \mathcal{G}`$, where $`q`$ is the order of the DSA group.
//! At the end of the proof, Alice is convinced that $`x \in [-q^3, q^3]`$ and Bob knows the discrete log of the public value $`X`$.
//!
//! The input for the proof:
//! * Paillier public key $`N, \Gamma`$,
//! * The ciphertexts $`c_1, c_2 \in Z_{N^2}`$,
//! * $`X \in \mathcal{G}`$ from the DSA group.
//!
//! The prover knows
//! * $`x \in Z_q`$, which is his multiplicative share $`b \in Z_q`$ in `MtA` protocol,
//! * $`y \in Z_N`$, which is $`\beta' \in Z_N`$ in `MtA` protocol,
//! * $`r \in Z^*_N`$, which is the random used in the encryption of $`\beta'`$,
//! such that $`c_2 = c_1^x\Gamma^yr^N \mod N^2`$ which is equal to $`c_B = bc_A + E_A(\beta') \mod N^2`$
//! * $`X = g^x \in \mathcal{G}`$
//!
//! ## Data types used
//!
//! [`BobZkpInit`](struct.BobZkpInit.html), [`BobProofExt`](struct.BobProofExt.html), [`BobZkpRound1`](struct.BobZkpRound1.html), [`BobZkpRound2`](struct.BobZkpRound2.html)
//!
//!  ## Algorithm
//!
//! ```math
//! \begin{array}{lcl}
//!   \textrm{\underline{Prover (Bob)}}                                                   &  & \textrm{\underline{Verifier (Alice)}} \\ \\
//!   \textrm{samples}                                              &  &                  \\
//!   \quad \alpha \in_R Z_{q^3}                                          &  &                      \\
//!   \quad \rho \in_R Z_{q\tilde{N}}                                     &  &                      \\
//!   \quad \rho' \in_R Z_{q^3\tilde{N}}                                  &  &                      \\
//!   \quad \sigma \in_R Z_{q\tilde{N}}                                   &  &                      \\
//!   \quad \beta \in_R Z^*_N                                             &  &                      \\
//!   \quad \gamma \in_R Z^*_N                                            &  &                      \\
//!   \quad \tau \in_R Z_{q\tilde{N}}                                     &  &                      \\ \\
//!   \textrm{computes}                                             &  &                      \\
//!   \quad u = g^{\alpha}                                                &  &                      \\
//!   \quad z = h_1^xh_2^{\rho} \mod {\tilde{N}}                          &  &                      \\
//!   \quad z' = h_1^{\alpha} h_2^{\rho'} \mod {\tilde{N}}                &  &                      \\
//!   \quad t = h_1^{y} h_2^{\sigma} \mod {\tilde{N}}                     &  &                      \\
//!   \quad v = c_1^{\alpha} \Gamma^{\gamma} \beta^{N} \mod N^2           &  &                      \\
//!   \quad w = h_1^{\gamma} h_2^{\tau}  \mod {\tilde{N}}                 & \xrightarrow{\hspace{5pt} u,z, z', t,v,w  \hspace{5pt}} &  \\
//!                                                                 &  & \textrm{selects}                 \\
//!                                                                 & \xleftarrow{\hspace{18pt} e \hspace{18pt}}  & \quad e \in_R Z_q  \\
//!   \!\textrm{computes}                                                 &  &                      \\
//!   \quad s = r^e \beta \mod N                                          &  &                      \\
//!   \quad s_1 = ex + \alpha                                             &  &                      \\
//!   \quad s_2 = e\rho + \rho'                                           &  &                      \\
//!   \quad t_1 = ey + \gamma                                             &  &                      \\
//!   \quad t_2 = e\sigma + \tau                                          & \xrightarrow{\hspace{7pt}s, s_1, s_2, t_1, t_2\hspace{7pt}}  & \\
//!                                                                 &  & \textrm{checks}      \\
//!                                                                 &  &  \quad s_1 \stackrel{?}{\leq} q^3   \\
//!                                                                 &  &  \quad g^{s_1} \stackrel{?}{=}  X^eu \in \mathcal{G} \\
//!                                                                 &  &  \quad h_1^{s_1} h_2^{s_2}  \stackrel{?}{=} z^{e}z'  \mod {\tilde{N}}   \\
//!                                                                 &  &  \quad h_1^{t_1} h_2^{t_2}  \stackrel{?}{=} t^{e}w  \mod {\tilde{N}}    \\
//!                                                                 &  &  \quad c_1^{s_1} s^{N} \Gamma^{t_1}  \stackrel{?}{=} c_2^{e}v  \mod N^2 \\
//!\end{array}
//! ```
//!
//!  The implementation uses non-interactive proof where the challenge $`e`$ is computed by the prover as $` \textrm{SHA512/256}(N || \Gamma || X.x || X.y || c_1 || c_2 || u.x || u.y || z || z' || t || v || w) \mod{q} `$
//!
#![allow(non_snake_case)]
use curv::arithmetic::traits::{Samplable, ZeroizeBN};
use curv::{BigInt, FE, GE};
use paillier::{
    Add, EncryptWithChosenRandomness, EncryptionKey, Mul, Paillier, Randomness, RawCiphertext,
    RawPlaintext,
};

use curv::cryptographic_primitives::hashing::traits::Hash;
use curv::elliptic::curves::traits::{ECPoint, ECScalar};
use std::borrow::Borrow;
use zeroize::Zeroize;

use crate::algorithms::primes::PairOfSafePrimes;
use crate::algorithms::sha::HSha512Trunc256;
use curv::cryptographic_primitives::proofs::sigma_dlog::{DLogProof, ProveDLog};
use failure::Fail;
use serde::{Deserialize, Serialize};
use trace::trace;

pub const DEFAULT_GROUP_ORDER_BIT_LENGTH: usize = 2048;
pub const DEFAULT_SAFE_PRIME_BIT_LENGTH: usize = DEFAULT_GROUP_ORDER_BIT_LENGTH / 2;

#[derive(Debug, Fail)]
#[fail(display = "zkp setup verification: {}", _0)]
pub struct ZkpSetupVerificationError(String);

/// Zero knowledge range proof setup.
/// It has to be created before using range proofs
/// The setup consist of following private values  $`p`$ and $`q`$ primes, $` \: \alpha \in \mathbb{Z}_{\tilde{N}}^{\star} `$
/// and public values $` \tilde{N} , h_{1}, h_{2}  `$
/// where $` \tilde{N} = \tilde{P} * \tilde{Q} ,\: \tilde{P} = 2*p + 1 ,\: \tilde{Q} = 2*q + 1, \: h_{1} \in \mathbb{Z}_{\tilde{N}}^{\star}, \: h_{2} = h_{1}^{\alpha}  `$
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpSetup {
    p: BigInt,
    q: BigInt,
    order: BigInt,
    alpha: BigInt,
    pub N_tilda: BigInt,
    pub h1: BigInt,
    pub h2: BigInt,
}

/// Zeroes the memory occupied by the struct
#[trace(pretty)]
impl Zeroize for ZkpSetup {
    fn zeroize(&mut self) {
        self.p.zeroize_bn();
        self.q.zeroize_bn();
        self.order.zeroize_bn();
        self.alpha.zeroize_bn();
        self.N_tilda.zeroize_bn();
        self.h1.zeroize_bn();
        self.h2.zeroize_bn();
    }
}

/// Zeroes the memory occupied by the struct
#[trace(pretty)]
impl Drop for ZkpSetup {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Zero knowledge range proof setup, public part only.
/// It has to be shared with other parties before using range proofs.
/// Contains public fields of the setup and Dlog proof of the correctness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpPublicSetup {
    pub N_tilda: BigInt,
    pub h1: BigInt,
    pub h2: BigInt,
    pub dlog_proof: ZkpSetupProof,
    pub inv_dlog_proof: ZkpSetupProof,
}

/// The non-interactive proof of correctness of zero knowledge range proof setup.
/// Uses Schnorr's proof of knowing the discrete logarithm.
/// Needs to be shared with each party along with the setup itself
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ZkpSetupProof {
    pub V: BigInt,
    pub challenge: BigInt,
    pub r: BigInt,
}

#[cfg(not(test))]
fn pair_of_safe_primes(bit_length: usize) -> PairOfSafePrimes {
    let (p, p_prim) = super::primes::random_safe_prime(bit_length);
    let (q, q_prim) = super::primes::random_safe_prime(bit_length);
    PairOfSafePrimes {
        p,
        p_prim,
        q,
        q_prim,
    }
}

#[cfg(test)]
fn pair_of_safe_primes(bit_length: usize) -> PairOfSafePrimes {
    use rand::Rng;
    use std::fs;
    use std::path::Path;

    // assert that required bit length matches the bit length of primes loaded from the file
    // the file content is generated by safe-primes applications ( see project's examples)
    // where SAFE_PRIME_BIT_LENGTH is used as the parameter
    assert_eq!(bit_length, DEFAULT_SAFE_PRIME_BIT_LENGTH);

    let path = Path::new("tests/data/safe-primes.json");
    let primes: Vec<PairOfSafePrimes> =
        serde_json::from_str(&fs::read_to_string(path).expect("invalid safe-prime file"))
            .expect("invalid safe prime format");
    // output one prime set { p, p',q, q' } chosen randomly from the array
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, primes.len());
    primes[index].to_owned()
}

#[trace(pretty, prefix = "ZkpSetup::", disable(zero))]
impl ZkpSetup {
    /// Generates new zero knowledge range proof setup.
    /// Uses Fujisaki - Okamoto bit commitment scheme, "Statistical zero knowledge protocols to prove modular polynomial relations"
    pub fn random(group_order_bit_length: usize) -> Self {
        use crate::algorithms::sample_generator_of_rsa_group;
        let bit_length = group_order_bit_length / 2;

        // Fujisaki-Okamoto commitment scheme setup
        let mut primes = pair_of_safe_primes(bit_length);
        let N_tilda = primes.p.borrow() * primes.q.borrow();
        let b0 = sample_generator_of_rsa_group(&primes.p, &primes.q);
        // the order of the subgroup
        let order = &primes.p_prim * &primes.q_prim;
        let alpha = BigInt::sample_range(&BigInt::one(), &order);
        let b1 = b0.powm(alpha.borrow(), &N_tilda);

        let result = Self {
            p: primes.p.clone(),
            q: primes.q.clone(),
            order,
            alpha,
            N_tilda,
            h1: b0,
            h2: b1,
        };
        primes.zeroize();
        result
    }
    //#[trace(disable(dlog), disable(order))]
    pub fn dlog_proof(
        N_tilda: &BigInt,
        h1: &BigInt,
        h2: &BigInt,
        dlog: &BigInt,
        order: &BigInt,
    ) -> ZkpSetupProof {
        let One = &BigInt::one();

        let mut v: BigInt = BigInt::sample_range(One, &(N_tilda - One));
        let V = h1.powm(&v, &N_tilda);
        let challenge = HSha512Trunc256::create_hash(&[N_tilda, &V, h1, h2]);

        let r: BigInt = &v - (dlog * &challenge);
        let r = r % order;
        v.zeroize_bn();

        ZkpSetupProof { V, challenge, r }
    }

    #[cfg(test)]
    pub fn verify_setup(&self) -> bool {
        self.h2 == self.h1.powm(&self.alpha, &self.N_tilda)
    }
}

#[trace(pretty, prefix = "ZkpPublicSetup::")]
impl ZkpPublicSetup {
    ///  Creates new public setup from private one
    ///
    ///  Creates new public setup and generates Schnorr's proof of knowledge of discrete logarithm problem
    pub fn from_private_zkp_setup(setup: &ZkpSetup) -> Self {
        let inv_alpha = &setup
            .alpha
            .invert(&setup.order)
            .expect("alpha non invertible");
        Self {
            N_tilda: setup.N_tilda.clone(),
            h1: setup.h1.clone(),
            h2: setup.h2.clone(),
            dlog_proof: ZkpSetup::dlog_proof(
                &setup.N_tilda,
                &setup.h1,
                &setup.h2,
                &setup.alpha,
                &setup.order,
            ),
            inv_dlog_proof: ZkpSetup::dlog_proof(
                &setup.N_tilda,
                &setup.h2,
                &setup.h1,
                &inv_alpha,
                &setup.order,
            ),
        }
    }

    /// verifies public setup
    ///
    /// verifies public setup using classic Schnorr's proof
    pub fn verify(&self) -> Result<(), ZkpSetupVerificationError> {
        Self::verify_proof(&self.N_tilda, &self.h1, &self.h2, &self.dlog_proof)?;
        Self::verify_proof(&self.N_tilda, &self.h2, &self.h1, &self.inv_dlog_proof)?;
        Ok(())
    }
    pub fn verify_proof(
        N_tilda: &BigInt,
        h1: &BigInt,
        h2: &BigInt,
        proof: &ZkpSetupProof,
    ) -> Result<(), ZkpSetupVerificationError> {
        let challenge = HSha512Trunc256::create_hash(&[N_tilda, &proof.V, h1, h2]);

        if challenge != proof.challenge {
            return Err(ZkpSetupVerificationError(
                "dlog proof: challenge does not match".to_string(),
            ));
        }

        let V = h1.powm(&proof.r, N_tilda) * h2.powm(&challenge, N_tilda) % N_tilda;

        if V == proof.V {
            Ok(())
        } else {
            Err(ZkpSetupVerificationError("Dlog proof failed".to_string()))
        }
    }
}

#[trace(pretty)]
impl Zeroize for ZkpPublicSetup {
    fn zeroize(&mut self) {
        self.N_tilda.zeroize_bn();
        self.h1.zeroize_bn();
        self.h2.zeroize_bn();
    }
}
/// First message of `MtA` protocol that Alice sends to Bob
///
/// Contains ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageA {
    pub c: BigInt,
    pub range_proof: Option<AliceProof>,
}
#[trace(pretty, prefix = "MessageA::")]
impl MessageA {
    // a - Alice 's secret
    pub fn new(a: &FE, alice_pk: &EncryptionKey, bob_setup: Option<&ZkpPublicSetup>) -> MessageA {
        let mut r = BigInt::from_paillier_key(&alice_pk);
        let cipher = Paillier::encrypt_with_chosen_randomness(
            alice_pk,
            RawPlaintext::from(a.to_big_int()),
            &Randomness::from(&r),
        )
        .0
        .into_owned();

        let proof = bob_setup.map(|zkp_setup| {
            AliceProof::generate(&a.to_big_int(), &cipher, alice_pk, zkp_setup, &r, &FE::q())
        });

        r.zeroize_bn();

        MessageA {
            c: cipher,
            range_proof: proof,
        }
    }
}

/// Data used internally for Alice's proof
///
/// New data have to be generated for each proof
struct AliceZkpInit {
    alice_pk: EncryptionKey,
    bob_setup: ZkpPublicSetup,
    pub alpha: BigInt,
    pub beta: BigInt,
    pub gamma: BigInt,
    pub ro: BigInt,
}

/// Zeroize Alice's ZKP
impl Zeroize for AliceZkpInit {
    fn zeroize(&mut self) {
        self.alice_pk.n.zeroize_bn();
        self.alice_pk.nn.zeroize_bn();
        self.bob_setup.zeroize();
        self.alpha.zeroize_bn();
        self.beta.zeroize_bn();
        self.gamma.zeroize_bn();
        self.ro.zeroize_bn();
    }
}

impl Drop for AliceZkpInit {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl AliceZkpInit {
    ///  Creates new data
    ///
    ///  # Arguments
    ///
    ///  `alice_pk` - Alice's Paillier public key
    ///
    ///  `bob_setup` - Bob's ZKP setup
    ///
    ///  `q` - modulo of the elliptic group used in ECDSA
    ///
    fn random(alice_pk: &EncryptionKey, bob_setup: &ZkpPublicSetup, q: &BigInt) -> Self {
        Self {
            alice_pk: alice_pk.clone(),
            bob_setup: bob_setup.clone(),
            alpha: BigInt::sample_below(&q.pow(3)),
            beta: BigInt::from_paillier_key(&alice_pk),
            gamma: BigInt::sample_below(&(q.pow(3) * &bob_setup.N_tilda)),
            ro: BigInt::sample_below(&(q * &bob_setup.N_tilda)),
        }
    }
    pub fn N(&self) -> &BigInt {
        &self.alice_pk.n
    }
    pub fn NN(&self) -> &BigInt {
        &self.alice_pk.nn
    }
    pub fn N_tilda(&self) -> &BigInt {
        &self.bob_setup.N_tilda
    }
    pub fn h1(&self) -> &BigInt {
        &self.bob_setup.h1
    }
    pub fn h2(&self) -> &BigInt {
        &self.bob_setup.h2
    }
}

/// represents first round of the interactive version of the proof
struct AliceZkpRound1 {
    pub z: BigInt,
    pub u: BigInt,
    pub w: BigInt,
}

impl AliceZkpRound1 {
    fn from(init: &AliceZkpInit, a: &BigInt) -> Self {
        Self {
            z: (init.h1().powm(&a, init.N_tilda()) * init.h2().powm(&init.ro, init.N_tilda()))
                % init.N_tilda(),
            u: ((init.alpha.borrow() * init.N() + 1) * init.beta.powm(init.N(), init.NN()))
                % init.NN(),
            w: (init.h1().powm(&init.alpha, init.N_tilda())
                * init.h2().powm(&init.gamma, init.N_tilda()))
                % init.N_tilda(),
        }
    }
}

/// represents second round of the interactive version of the proof
struct AliceZkpRound2 {
    s: BigInt,
    s1: BigInt,
    s2: BigInt,
}

impl AliceZkpRound2 {
    pub fn from(init: &AliceZkpInit, e: &BigInt, a: &BigInt, r: &BigInt) -> Self {
        Self {
            s: (r.powm(&e, init.N()) * init.beta.borrow()) % init.N(),
            s1: (e * a) + init.alpha.borrow(),
            s2: (e * init.ro.borrow()) + init.gamma.borrow(),
        }
    }
}

pub type HashWithNonce = (BigInt, BigInt);

/// Alice's proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliceProof {
    z: BigInt,
    u: BigInt,
    w: BigInt,
    e: HashWithNonce,
    s: BigInt,
    s1: BigInt,
    s2: BigInt,
}
#[trace(pretty, prefix = "AliceProof::")]
impl AliceProof {
    /// verify Alice's proof using the proof and public keys
    pub fn verify(
        &self,
        cipher: &BigInt,
        alice_ek: &EncryptionKey,
        bob_zkp_setup: &ZkpSetup,
    ) -> bool {
        let N = &alice_ek.n;
        let NN = &alice_ek.nn;
        let N_tilda = &bob_zkp_setup.N_tilda;
        let Gen = alice_ek.n.borrow() + 1;

        let e = HSha512Trunc256::create_hash_with_nonce(
            &[N, &Gen, cipher, &self.z, &self.u, &self.w],
            &self.e.1,
        );
        if e != self.e {
            log::trace!("hash doesn't match");
            return false;
        }

        if self.s1 > FE::q().pow(3) {
            log::trace!("proof.s1 is larger than q^3");
            return false;
        }

        let z_e_inv = self.z.powm(&self.e.0, N_tilda).invert(N_tilda);
        if z_e_inv.is_none() {
            // z must be invertible,  yet the check is done here
            log::trace!("no multiplicative inverse for z^e");
            return false;
        }
        let z_e_inv = z_e_inv.unwrap();

        let wprim = (bob_zkp_setup.h1.powm(&self.s1, N_tilda)
            * bob_zkp_setup.h2.powm(&self.s2, N_tilda)
            * z_e_inv)
            % N_tilda;

        if self.w != wprim {
            log::trace!("proof.w does not hold right value");
            return false;
        }

        let gs1 = (self.s1.borrow() * N + 1) % NN;
        let cipher_e_inv = cipher.powm(&self.e.0, NN).invert(NN);
        if cipher_e_inv.is_none() {
            log::trace!("no multiplicative inverse for a^e");
            return false;
        }
        let cipher_e_inv = cipher_e_inv.unwrap();

        let uprim = (gs1 * self.s.powm(N, NN) * cipher_e_inv) % NN;

        if uprim != self.u {
            log::trace!("proof.u does not hold right value");
            return false;
        }

        true
    }
    /// create the proof using Alice's private keys of Paillier and private keys of ZKP setup
    /// requires randomness used for encrypting Alice's secret a
    /// requires the EC group order of the used curve
    pub fn generate(
        a: &BigInt,
        cipher: &BigInt,
        alice_pk: &EncryptionKey,
        bob_zkp_setup: &ZkpPublicSetup,
        r: &BigInt,
        q: &BigInt,
    ) -> Self {
        let init = AliceZkpInit::random(alice_pk, bob_zkp_setup, q);
        let round1 = AliceZkpRound1::from(&init, a);

        let Gen = init.N() + 1;
        let e = HSha512Trunc256::create_hash_bounded_by_q(
            &[init.N(), &Gen, cipher, &round1.z, &round1.u, &round1.w],
            q,
        );

        let round2 = AliceZkpRound2::from(&init, &e.0, a, r);

        Self {
            z: round1.z,
            u: round1.u,
            w: round1.w,
            e,
            s: round2.s,
            s1: round2.s1,
            s2: round2.s2,
        }
    }
}
/// simple discrete log proof, used as the alternative to range proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DLogProofs {
    pub b_proof: DLogProof,
    pub beta_tag_proof: DLogProof,
}

/// enumerates types of proofs Bob can use in the protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BobProofType {
    RangeProofExt(BobProofExt),
    RangeProof(BobProof),
    DLogProofs(DLogProofs),
}

/// enumerates the subtype of Bob's proof
#[derive(Debug)]
pub enum MTAMode {
    MtA,
    MtAwc,
}
/// the response to Alice's messageA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageB {
    pub c: BigInt,
    pub proof: BobProofType,
}
#[trace(pretty, prefix = "MessageB::")]
impl MessageB {
    /// b  is Bob's secret
    /// returns ( Message, new Bob's secret )
    pub fn new(
        b: &FE,
        alice_ek: &EncryptionKey,
        alice_zkp_setup: Option<&ZkpPublicSetup>,
        alice_msg: &MessageA,
        mta_mode: MTAMode,
    ) -> (MessageB, FE) {
        let alice_c = &alice_msg.c;

        // E(a) * b
        let b_bn = b.to_big_int();
        let b_times_enc_a = Paillier::mul(
            alice_ek,
            RawCiphertext::from(alice_c),
            RawPlaintext::from(&b_bn),
        );

        let beta_prim = BigInt::sample_below(&alice_ek.n);

        // E(beta_prim)
        let r = Randomness::sample(&alice_ek);
        let enc_beta_prim =
            Paillier::encrypt_with_chosen_randomness(alice_ek, RawPlaintext::from(&beta_prim), &r);
        //
        let mta_out = Paillier::add(alice_ek, b_times_enc_a, enc_beta_prim);

        let beta_prim_fe: FE = ECScalar::from(&beta_prim);
        let beta = FE::zero().sub(&beta_prim_fe.get_element());

        let proof = match &alice_zkp_setup {
            Some(zkp_setup) => {
                // generate range proof
                match mta_mode {
                    MTAMode::MtA => BobProofType::RangeProof(BobProof::generate(
                        &alice_c,
                        &mta_out.0.borrow(),
                        &b,
                        &beta_prim,
                        alice_ek,
                        zkp_setup,
                        &r,
                        &FE::q(),
                    )),
                    MTAMode::MtAwc => BobProofType::RangeProofExt(BobProofExt::generate(
                        &alice_c,
                        &mta_out.0.borrow(),
                        &b,
                        &beta_prim,
                        alice_ek,
                        zkp_setup,
                        &r,
                        &FE::q(),
                    )),
                }
            }

            None => {
                // generate dlog_proof
                BobProofType::DLogProofs(DLogProofs {
                    b_proof: DLogProof::prove(b),
                    beta_tag_proof: DLogProof::prove(&beta_prim_fe),
                })
            }
        };

        (
            MessageB {
                c: mta_out.0.into_owned(),
                proof,
            },
            beta,
        )
    }
}

/// internal data unique to every Bob's proof
struct BobZkpInit {
    pub alice_ek: EncryptionKey,
    pub alice_setup: ZkpPublicSetup,
    pub alpha: BigInt,
    pub beta: BigInt,
    pub gamma: BigInt,
    pub ro: BigInt,
    pub ro_prim: BigInt,
    pub sigma: BigInt,
    pub tau: BigInt,
}

impl Zeroize for BobZkpInit {
    fn zeroize(&mut self) {
        self.alice_ek.n.zeroize_bn();
        self.alice_ek.nn.zeroize_bn();
        self.alice_setup.zeroize();
        self.alpha.zeroize_bn();
        self.beta.zeroize_bn();
        self.gamma.zeroize_bn();
        self.ro.zeroize_bn();
        self.ro_prim.zeroize_bn();
        self.sigma.zeroize_bn();
        self.tau.zeroize_bn();
    }
}

impl Drop for BobZkpInit {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl BobZkpInit {
    fn random(alice_ek: &EncryptionKey, alice_setup: &ZkpPublicSetup, q: &BigInt) -> Self {
        Self {
            alice_ek: alice_ek.clone(),
            alice_setup: alice_setup.clone(),
            alpha: BigInt::sample_below(&q.pow(3)),
            beta: BigInt::from_paillier_key(&alice_ek),
            gamma: Randomness::sample(&alice_ek).0,
            ro: BigInt::sample_below(&(q * alice_setup.N_tilda.borrow())),
            ro_prim: BigInt::sample_below(&(q.pow(3) * alice_setup.N_tilda.borrow())),
            sigma: BigInt::sample_below(&(q * alice_setup.N_tilda.borrow())),
            tau: BigInt::sample_below(&(q * alice_setup.N_tilda.borrow())),
        }
    }
    fn N(&self) -> &BigInt {
        &self.alice_ek.n
    }
    pub fn Gen(&self) -> BigInt {
        self.N().borrow() + 1
    }
    pub fn NN(&self) -> &BigInt {
        &self.alice_ek.nn
    }
    pub fn N_tilda(&self) -> &BigInt {
        &self.alice_setup.N_tilda
    }
    pub fn h1(&self) -> &BigInt {
        &self.alice_setup.h1
    }
    pub fn h2(&self) -> &BigInt {
        &self.alice_setup.h2
    }
}

/// represents first round of the interactive version of the proof
struct BobZkpRound1 {
    pub z: BigInt,
    pub z_prim: BigInt,
    pub t: BigInt,
    pub w: BigInt,
    pub v: BigInt,
}

impl BobZkpRound1 {
    /// `b` - Bob's secret
    /// `beta_prim`  - randomly chosen in `MtA` by Bob
    /// `a_encrypted` - Alice's secret encrypted by Alice
    fn from(init: &BobZkpInit, b: &FE, beta_prim: &BigInt, a_encrypted: &BigInt) -> Self {
        let b_bn = b.to_big_int();
        Self {
            z: (init.h1().powm(&b_bn, init.N_tilda()) * init.h2().powm(&init.ro, init.N_tilda()))
                % init.N_tilda(),
            z_prim: (init.h1().powm(&init.alpha, init.N_tilda())
                * init.h2().powm(&init.ro_prim, init.N_tilda()))
                % init.N_tilda(),
            t: (init.h1().powm(beta_prim, init.N_tilda())
                * init.h2().powm(&init.sigma, init.N_tilda()))
                % init.N_tilda(),
            w: (init.h1().powm(&init.gamma, init.N_tilda())
                * init.h2().powm(&init.tau, init.N_tilda()))
                % init.N_tilda(),
            v: (a_encrypted.powm(&init.alpha, init.NN())
                * (init.gamma.borrow() * init.N() + 1)
                * init.beta.powm(init.N(), init.NN()))
                % init.NN(),
        }
    }
}

/// represents second round of the interactive version of the proof
struct BobZkpRound2 {
    pub s: BigInt,
    pub s1: BigInt,
    pub s2: BigInt,
    pub t1: BigInt,
    pub t2: BigInt,
}

impl BobZkpRound2 {
    /// `e` - the challenge in interactive ZKP, the hash in non-interactive ZKP
    /// `b` - Bob's secret
    /// `beta_prim` - randomly chosen in `MtA` by Bob
    /// `r` - randomness used by Bob on  Alice's public Paillier key to encrypt `beta_prim` in `MtA`
    fn from(init: &BobZkpInit, e: &BigInt, b: &FE, beta_prim: &BigInt, r: &Randomness) -> Self {
        let b_bn = b.to_big_int();
        Self {
            s: (r.0.borrow().powm(e, init.N()) * init.beta.borrow()) % init.N(),
            s1: (e * b_bn) + init.alpha.borrow(),
            s2: (e * init.ro.borrow()) + init.ro_prim.borrow(),
            t1: (e * beta_prim) + init.gamma.borrow(),
            t2: (e * init.sigma.borrow()) + init.tau.borrow(),
        }
    }
}

/// Bob's regular proof
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BobProof {
    t: BigInt,
    v: BigInt,
    w: BigInt,
    z: BigInt,
    z_prim: BigInt,
    e: HashWithNonce,
    s: BigInt,
    s1: BigInt,
    s2: BigInt,
    t1: BigInt,
    t2: BigInt,
}

#[allow(clippy::too_many_arguments)]
#[trace(pretty, prefix = "BobProof::")]
impl BobProof {
    pub fn verify(
        &self,
        a_enc: &BigInt,
        mta_avc_out: &BigInt,
        alice_ek: &EncryptionKey,
        alice_setup: &ZkpSetup,
    ) -> bool {
        let Gen = alice_ek.n.borrow() + 1;
        let e = HSha512Trunc256::create_hash_with_nonce(
            &[
                &alice_ek.n,
                &Gen,
                a_enc,
                mta_avc_out,
                &self.z,
                &self.z_prim,
                &self.t,
                &self.v,
                &self.w,
            ],
            &self.e.1,
        );

        self.verify_with_hash(&e, a_enc, mta_avc_out, alice_ek, alice_setup)
    }
    pub fn verify_with_hash(
        &self,
        e: &HashWithNonce,
        a_enc: &BigInt,
        mta_avc_out: &BigInt,
        alice_ek: &EncryptionKey,
        alice_setup: &ZkpSetup,
    ) -> bool {
        let N = &alice_ek.n;
        let NN = &alice_ek.nn;
        let N_tilda = &alice_setup.N_tilda;
        let h1 = &alice_setup.h1;
        let h2 = &alice_setup.h2;

        if *e != self.e {
            log::trace!("hash doesn't match");
            return false;
        }

        if self.s1 > FE::q().pow(3) {
            log::trace!("proof.s1 is larger than q^3");
            return false;
        }

        let lz = (h1.powm(&self.s1, N_tilda) * h2.powm(&self.s2, N_tilda)) % N_tilda;
        let rz = (self.z.powm(&e.0, N_tilda) * self.z_prim.borrow()) % N_tilda;
        if lz != rz {
            log::trace!("proof.z doesn't hold right value");
            return false;
        }

        let lc1 = (a_enc.powm(&self.s1, NN) * self.s.powm(N, NN) * (self.t1.borrow() * N + 1)) % NN;
        let lc2 = (mta_avc_out.powm(&e.0, NN) * self.v.borrow()) % NN;
        if lc1 != lc2 {
            log::trace!("proof.c2.v doesn't hold right value");
            return false;
        }

        let lw = (h1.powm(&self.t1, N_tilda) * h2.powm(&self.t2, N_tilda)) % N_tilda;
        let rw = (self.t.powm(&e.0, N_tilda) * self.w.borrow()) % N_tilda;
        if lw != rw {
            log::trace!("proof.t.w doesn't hold right value");
            return false;
        }

        true
    }

    pub fn generate(
        a_encrypted: &BigInt,
        mta_encrypted: &BigInt,
        b: &FE,
        beta_prim: &BigInt,
        alice_ek: &EncryptionKey,
        alice_setup: &ZkpPublicSetup,
        r: &Randomness,
        q: &BigInt,
    ) -> BobProof {
        let init = BobZkpInit::random(alice_ek, &alice_setup, q);
        let round1 = BobZkpRound1::from(&init, b, beta_prim, a_encrypted);

        let e = HSha512Trunc256::create_hash_with_random_nonce(&[
            init.N(),
            &init.Gen(),
            a_encrypted,
            mta_encrypted,
            &round1.z,
            &round1.z_prim,
            &round1.t,
            &round1.v,
            &round1.w,
        ]);

        let round2 = BobZkpRound2::from(&init, &e.0, b, beta_prim, r);

        BobProof {
            t: round1.t,
            v: round1.v,
            w: round1.w,
            z: round1.z,
            z_prim: round1.z_prim,
            e,
            s: round2.s,
            s1: round2.s1,
            s2: round2.s2,
            t1: round2.t1,
            t2: round2.t2,
        }
    }
}

/// Bob's extended proof, adds the knowledge of $`B = g^b \in \mathcal{G}`$
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BobProofExt {
    proof: BobProof,
    u: GE,
    X: GE,
}

#[allow(clippy::too_many_arguments)]
#[trace(pretty, prefix = "BobProofExt::")]
impl BobProofExt {
    pub fn verify(
        &self,
        a_enc: &BigInt,
        mta_avc_out: &BigInt,
        alice_ek: &EncryptionKey,
        alice_zkp_setup: &ZkpSetup,
    ) -> bool {
        let Gen = alice_ek.n.borrow() + 1;
        let e = HSha512Trunc256::create_hash_with_nonce(
            &[
                &alice_ek.n,
                &Gen,
                &self.X.x_coor().unwrap(),
                &self.X.y_coor().unwrap(),
                a_enc,
                mta_avc_out,
                &self.u.x_coor().unwrap(),
                &self.u.y_coor().unwrap(),
                &self.proof.z,
                &self.proof.z_prim,
                &self.proof.t,
                &self.proof.v,
                &self.proof.w,
            ],
            &self.proof.e.1,
        );

        // check basic proof first
        if !self
            .proof
            .verify_with_hash(&e, a_enc, mta_avc_out, alice_ek, alice_zkp_setup)
        {
            return false;
        }

        // fiddle with EC points
        let (x1, x2) = {
            let ec_gen: GE = ECPoint::generator();
            let s1: FE = ECScalar::from(&self.proof.s1);
            let e: FE = ECScalar::from(&self.proof.e.0);
            (ec_gen * s1, (self.X * e) + self.u)
        };

        if x1 != x2 {
            log::trace!("proof.X doesn't hold right value");
            return false;
        }

        true
    }

    pub fn generate(
        a_encrypted: &BigInt,
        mta_encrypted: &BigInt,
        b: &FE,
        beta_prim: &BigInt,
        alice_ek: &EncryptionKey,
        alice_setup: &ZkpPublicSetup,
        r: &Randomness,
        q: &BigInt,
    ) -> BobProofExt {
        let init = BobZkpInit::random(alice_ek, &alice_setup, q);

        let (X, u) = {
            let ec_gen: GE = ECPoint::generator();
            let alpha: FE = ECScalar::from(&init.alpha);
            (ec_gen * b, ec_gen * alpha)
        };

        let round1 = BobZkpRound1::from(&init, b, beta_prim, a_encrypted);

        let e = HSha512Trunc256::create_hash_with_random_nonce(&[
            init.N(),
            &init.Gen(),
            &X.x_coor().unwrap(),
            &X.y_coor().unwrap(),
            a_encrypted,
            mta_encrypted,
            &u.x_coor().unwrap(),
            &u.y_coor().unwrap(),
            &round1.z,
            &round1.z_prim,
            &round1.t,
            &round1.v,
            &round1.w,
        ]);

        let round2 = BobZkpRound2::from(&init, &e.0, b, beta_prim, r);

        BobProofExt {
            proof: BobProof {
                t: round1.t,
                v: round1.v,
                w: round1.w,
                z: round1.z,
                z_prim: round1.z_prim,
                e,
                s: round2.s,
                s1: round2.s1,
                s2: round2.s2,
                t1: round2.t1,
                t2: round2.t2,
            },
            X,
            u,
        }
    }
}

/// sample random value of an element of multiplicative group
pub trait SampleFromMultiplicativeGroup {
    fn from_modulo(N: &BigInt) -> BigInt;
    fn from_paillier_key(ek: &EncryptionKey) -> BigInt;
    fn from_zkp_setup(zkp_setup: &ZkpSetup) -> BigInt;
    fn from_zkp_public_setup(zkp_setup: &ZkpPublicSetup) -> BigInt;
}

impl SampleFromMultiplicativeGroup for BigInt {
    fn from_modulo(N: &BigInt) -> BigInt {
        let One = BigInt::one();
        loop {
            let r = Self::sample_below(N);
            if r.gcd(N) == One {
                return r;
            }
        }
    }

    fn from_paillier_key(ek: &EncryptionKey) -> BigInt {
        Self::from_modulo(ek.n.borrow())
    }
    fn from_zkp_setup(zkp_keys: &ZkpSetup) -> BigInt {
        Self::from_modulo(zkp_keys.N_tilda.borrow())
    }
    fn from_zkp_public_setup(zkp_keys: &ZkpPublicSetup) -> BigInt {
        Self::from_modulo(zkp_keys.N_tilda.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecdsa::PaillierKeys;

    #[test]
    fn hash_algorithm_compliant() {
        assert!(HSha512Trunc256::can_handle_curve_modulo(&FE::q()))
    }

    #[test]
    fn zkp_setup() {
        let zq = DEFAULT_GROUP_ORDER_BIT_LENGTH;
        assert_eq!(zq % 2, 0);
        let setup = ZkpSetup::random(zq);
        // primality and bitness is testes in module 'primes'
        assert_eq!(setup.p.borrow() * setup.q.borrow(), setup.N_tilda);
        assert_eq!(setup.N_tilda.gcd(&setup.p), setup.p);
        assert_eq!(setup.N_tilda.gcd(&setup.q), setup.q);
    }

    #[test]
    fn alice_zkp() {
        let _ = env_logger::builder().is_test(true).try_init();

        let bob_setup = ZkpSetup::random(DEFAULT_GROUP_ORDER_BIT_LENGTH);
        let bob_public_setup = ZkpPublicSetup::from_private_zkp_setup(&bob_setup);

        // Alice private
        let (message, public_paillier_key) = {
            let paillier_keys = PaillierKeys::random();
            let a = FE::new_random();
            (
                MessageA::new(&a, &paillier_keys.ek, Some(&bob_public_setup)),
                paillier_keys.ek.clone(),
            )
        };

        assert!(message.range_proof.is_some());
        let proof = message.range_proof.unwrap();
        // Bob
        assert!(proof.verify(&message.c, &public_paillier_key, &bob_setup));
    }

    pub struct AliceOrBob {
        pub paillier_keys: PaillierKeys,
        pub zkp_setup: ZkpSetup,
    }

    impl AliceOrBob {
        pub fn new() -> Self {
            Self {
                paillier_keys: PaillierKeys::random(),
                zkp_setup: ZkpSetup::random(DEFAULT_GROUP_ORDER_BIT_LENGTH),
            }
        }
    }

    #[test]
    fn bob_zkp() {
        let _ = env_logger::builder().is_test(true).try_init();

        let alice = (AliceOrBob::new(), FE::new_random());
        let alice_public_zkp_setup = &ZkpPublicSetup::from_private_zkp_setup(&alice.0.zkp_setup);
        let alice_public_key = &alice.0.paillier_keys.ek;
        let bob = (AliceOrBob::new(), FE::new_random());
        let bob_public_zkp_setup = &ZkpPublicSetup::from_private_zkp_setup(&bob.0.zkp_setup);

        // Simulate Alice
        let msga = MessageA::new(
            &alice.1,
            &alice.0.paillier_keys.ek,
            Some(&bob_public_zkp_setup),
        );

        // Bob  proves
        let (msgb, _beta) = MessageB::new(
            &bob.1,
            alice_public_key,
            Some(alice_public_zkp_setup),
            &msga,
            MTAMode::MtAwc,
        );

        match msgb.proof {
            BobProofType::RangeProofExt(proof) => {
                // Alice verifies
                assert!(proof.verify(&msga.c, &msgb.c, alice_public_key, &alice.0.zkp_setup));
            }
            _ => assert!(false, ""),
        }
    }

    #[test]
    fn zkp_setup_proof() {
        let _ = env_logger::builder().is_test(true).try_init();

        let setup = ZkpSetup::random(DEFAULT_GROUP_ORDER_BIT_LENGTH);
        assert!(setup.verify_setup());
        let pub_setup = ZkpPublicSetup::from_private_zkp_setup(&setup);
        if let Err(e) = pub_setup.verify() {
            log::error!("{}", e);
            assert!(false);
        }
    }
}
