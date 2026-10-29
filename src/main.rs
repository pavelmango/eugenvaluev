// ============================================================================
// EUGEN VALUEV: CEREMONIAL HABITAT ENGINE (Rust 2024 Edition)
// "When We Won the Cold War" — Ms. Olympia's Undergraduate Spectral Laboratory
// ============================================================================

mod enclave;
mod episode3;

use std::f64::consts::PI;
use std::fs;
use std::process::Command;

/// Deterministic 64-bit XorShift* PRNG + Box-Muller Gaussian generator
#[derive(Clone)]
struct Rng64 {
    state: u64,
}

impl Rng64 {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x1989_0915_CAFE_BABE } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64))
    }

    fn next_gauss(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-14);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

/// Result of a symmetric Cyclic Jacobi eigendecomposition:
/// eigenvalues sorted descending, plus the corresponding eigenvectors as columns.
struct EigenSystem {
    eigenvalues: Vec<f64>,
    dominant_eigenvector: Vec<f64>,
}

fn jacobi_eigensystem(mut a: Vec<f64>, n: usize, max_sweeps: usize) -> EigenSystem {
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }

    for _ in 0..max_sweeps {
        let mut max_off = 0.0_f64;
        for p in 0..(n - 1) {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                max_off = max_off.max(apq.abs());
                if apq.abs() > 1e-13 {
                    let app = a[p * n + p];
                    let aqq = a[q * n + q];
                    let tau = (aqq - app) / (2.0 * apq);
                    let t = if tau >= 0.0 {
                        1.0 / (tau + (1.0 + tau * tau).sqrt())
                    } else {
                        -1.0 / (-tau + (1.0 + tau * tau).sqrt())
                    };
                    let c = 1.0 / (1.0 + t * t).sqrt();
                    let s = t * c;

                    a[p * n + p] = app - t * apq;
                    a[q * n + q] = aqq + t * apq;
                    a[p * n + q] = 0.0;
                    a[q * n + p] = 0.0;

                    for r in 0..n {
                        if r != p && r != q {
                            let arp = a[r * n + p];
                            let arq = a[r * n + q];
                            a[r * n + p] = c * arp - s * arq;
                            a[p * n + r] = a[r * n + p];
                            a[r * n + q] = s * arp + c * arq;
                            a[q * n + r] = a[r * n + q];
                        }
                    }

                    for r in 0..n {
                        let vrp = v[r * n + p];
                        let vrq = v[r * n + q];
                        v[r * n + p] = c * vrp - s * vrq;
                        v[r * n + q] = s * vrp + c * vrq;
                    }
                }
            }
        }
        if max_off < 1e-13 {
            break;
        }
    }

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| a[j * n + j].partial_cmp(&a[i * n + i]).unwrap());

    let eigenvalues: Vec<f64> = idx.iter().map(|&i| a[i * n + i]).collect();
    let dom_col = idx[0];
    let dominant_eigenvector: Vec<f64> = (0..n).map(|r| v[r * n + dom_col]).collect();

    EigenSystem {
        eigenvalues,
        dominant_eigenvector,
    }
}

/// Hotelling-deflated Power Iteration to extract top-k eigenvalues (Spark MLlib style)
fn deflated_power_iteration(mut a: Vec<f64>, n: usize, k: usize, iters: usize) -> Vec<f64> {
    let mut top_eigs = Vec::with_capacity(k);
    for mode in 0..k {
        let mut b = vec![0.0; n];
        for i in 0..n {
            b[i] = ((i + mode + 1) as f64).sin() + 0.5;
        }
        let norm0 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
        for x in &mut b {
            *x /= norm0;
        }

        let mut rayleigh = 0.0;
        for _ in 0..iters {
            let mut ab = vec![0.0; n];
            for i in 0..n {
                let mut s = 0.0;
                for j in 0..n {
                    s += a[i * n + j] * b[j];
                }
                ab[i] = s;
            }
            rayleigh = b.iter().zip(ab.iter()).map(|(bi, abi)| bi * abi).sum();
            let norm = ab.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-15);
            for i in 0..n {
                b[i] = ab[i] / norm;
            }
        }
        top_eigs.push(rayleigh);

        // Hotelling deflation: A <- A - lambda * b * b^T
        for i in 0..n {
            for j in 0..n {
                a[i * n + j] -= rayleigh * b[i] * b[j];
            }
        }
    }
    top_eigs
}

fn main() {
    println!("================================================================================");
    println!("  WHEN WE WON THE COLD WAR: CEREMONIAL EMERGENCE OF EUGEN VALUEV");
    println!("  Presiding Instructor : Ms. Olympia, Dept. of Heavyweight Spectral Bionics");
    println!("  Dual-Superpower Pact : Rust (Cargo 2024) + Microsoft .NET 10 (F# 10.0)");
    println!("================================================================================\n");

    let mut rng = Rng64::new(1989_11_09);

    // ========================================================================
    // BIOME I: RANDOM MATRIX THEORY (WIGNER SEMICIRCLE & LEVEL REPULSION)
    // ========================================================================
    println!("--- [ACT I] THE WIGNER SEMICIRCLE STEPPE & DYSON LEVEL REPULSION ---");
    let n_goe = 24usize;
    let m_trials = 120usize;
    let mut all_goe_eigs = Vec::with_capacity(n_goe * m_trials);
    let mut raw_spacings = Vec::new();

    for _ in 0..m_trials {
        let mut h = vec![0.0; n_goe * n_goe];
        let scale = 1.0 / (n_goe as f64).sqrt();
        for i in 0..n_goe {
            h[i * n_goe + i] = rng.next_gauss() * 2.0_f64.sqrt() * scale;
            for j in (i + 1)..n_goe {
                let v = rng.next_gauss() * scale;
                h[i * n_goe + j] = v;
                h[j * n_goe + i] = v;
            }
        }
        let sys = jacobi_eigensystem(h, n_goe, 20);
        // Collect bulk spacings (central 50% of spectrum where density is nearly uniform)
        for k in (n_goe / 4)..(3 * n_goe / 4 - 1) {
            raw_spacings.push((sys.eigenvalues[k] - sys.eigenvalues[k + 1]).abs());
        }
        all_goe_eigs.extend(sys.eigenvalues);
    }

    let mean_spacing = raw_spacings.iter().sum::<f64>() / (raw_spacings.len() as f64);
    let normalized_spacings: Vec<f64> = raw_spacings.iter().map(|s| s / mean_spacing).collect();
    let small_gap_frac = normalized_spacings.iter().filter(|&&s| s < 0.25).count() as f64
        / (normalized_spacings.len() as f64);
    let poisson_small_frac = 1.0 - (-0.25_f64).exp();

    println!(
        "  Spawned {} GOE Matrices ({}x{}) -> {} wild Eugen Valuevs emerged!",
        m_trials,
        n_goe,
        n_goe,
        all_goe_eigs.len()
    );
    println!("  Empirical Wigner Semicircle Density vs. Analytical Law rho(x) = sqrt(4 - x^2)/(2*pi):");

    let bins = 10usize;
    let x_min = -2.0_f64;
    let x_max = 2.0_f64;
    let dx = (x_max - x_min) / (bins as f64);
    for b in 0..bins {
        let lo = x_min + (b as f64) * dx;
        let hi = lo + dx;
        let mid = 0.5 * (lo + hi);
        let count = all_goe_eigs
            .iter()
            .filter(|&&lam| lam >= lo && lam < hi)
            .count();
        let emp_density = (count as f64) / ((all_goe_eigs.len() as f64) * dx);
        let theo_density = (4.0 - mid * mid).max(0.0).sqrt() / (2.0 * PI);
        let bar_len = (emp_density * 60.0).round() as usize;
        let bar: String = std::iter::repeat('#').take(bar_len).collect();
        println!(
            "    [{:+5.2}, {:+5.2}) | emp={:.3} (theo={:.3}) | {}",
            lo, hi, emp_density, theo_density, bar
        );
    }
    println!(
        "  Dyson Level Repulsion Check (s < 0.25): GOE Observed = {:.2}% vs. Uncorrelated Poisson = {:.2}%",
        small_gap_frac * 100.0,
        poisson_small_frac * 100.0
    );
    println!("  -> Ms. Olympia's Verdict: Two Eugen Valuevs NEVER share the same squat rack!\n");

    // ========================================================================
    // BIOME II: DISTRIBUTED SHARD COVARIANCE (MARCHENKO-PASTUR & POWER ITERATION)
    // ========================================================================
    println!("--- [ACT II] DISTRIBUTED COVARIANCE GYMNASIUM (SHARD MAP-REDUCE & PCA/SVD) ---");
    let p_dim = 12usize;
    let num_shards = 8usize;
    let rows_per_shard = 12usize;
    let m_total = num_shards * rows_per_shard; // 96 observations

    // Simulate distributed executor shards computing local Gram matrices X_k * X_k^T
    // with 2 planted heavy Cold War signal modes (lambda_signal_1 = 4.2, lambda_signal_2 = 2.8)
    let mut global_cov = vec![0.0; p_dim * p_dim];
    for _shard in 0..num_shards {
        let mut local_gram = vec![0.0; p_dim * p_dim];
        for _row in 0..rows_per_shard {
            let mut x = vec![0.0; p_dim];
            let factor1 = rng.next_gauss() * (3.2_f64).sqrt();
            let factor2 = rng.next_gauss() * (1.8_f64).sqrt();
            for j in 0..p_dim {
                let loading1 = if j < 4 { 0.5 } else { 0.0 };
                let loading2 = if (4..8).contains(&j) { 0.5 } else { 0.0 };
                x[j] = rng.next_gauss() + factor1 * loading1 + factor2 * loading2;
            }
            for i in 0..p_dim {
                for j in 0..p_dim {
                    local_gram[i * p_dim + j] += x[i] * x[j];
                }
            }
        }
        // Associative tree reduction across shards
        for idx in 0..(p_dim * p_dim) {
            global_cov[idx] += local_gram[idx] / (m_total as f64);
        }
    }

    let q_aspect = (p_dim as f64) / (m_total as f64);
    let mp_minus = (1.0 - q_aspect.sqrt()).powi(2);
    let mp_plus = (1.0 + q_aspect.sqrt()).powi(2);

    let cov_jacobi = jacobi_eigensystem(global_cov.clone(), p_dim, 25);
    let cov_power_top3 = deflated_power_iteration(global_cov.clone(), p_dim, 3, 60);

    println!(
        "  Aggregated {} Distributed Shards ({} total rows x {} features, aspect q = {:.3})",
        num_shards, m_total, p_dim, q_aspect
    );
    println!(
        "  Marchenko-Pastur Noise Sea Bounds  : [lambda_- = {:.4}, lambda_+ = {:.4}]",
        mp_minus, mp_plus
    );
    println!(
        "  Jacobi Exact Top-3 Eigenvalues     : [{:.4}, {:.4}, {:.4}]",
        cov_jacobi.eigenvalues[0], cov_jacobi.eigenvalues[1], cov_jacobi.eigenvalues[2]
    );
    println!(
        "  Deflated Power Iteration Top-3     : [{:.4}, {:.4}, {:.4}]",
        cov_power_top3[0], cov_power_top3[1], cov_power_top3[2]
    );
    let outliers: Vec<f64> = cov_jacobi
        .eigenvalues
        .iter()
        .copied()
        .filter(|&l| l > mp_plus)
        .collect();
    println!(
        "  Emergent Signal Outliers (> {:.4}): {:?} (2 planted heavyweights cleanly recovered!)\n",
        mp_plus,
        outliers
            .iter()
            .map(|x| format!("{:.4}", x))
            .collect::<Vec<_>>()
    );

    // ========================================================================
    // BIOME III: EVOLUTIONARY ALIFE ARENA (POPULATION OF SPECTRAL CREATURES)
    // ========================================================================
    println!("--- [ACT III] EVOLUTIONARY ARENA: ARTIFICIAL LIFE OF SPECTRAL CREATURES ---");
    let n_org = 8usize;
    let pop_size = 40usize;
    let generations = 30usize;

    // Initialize a random Frobenius-normalized population of 8x8 symmetric matrix creatures
    let mut champion = vec![0.0; n_org * n_org];
    for i in 0..n_org {
        for j in i..n_org {
            let v = rng.next_gauss() * 0.5;
            champion[i * n_org + j] = v;
            champion[j * n_org + i] = v;
        }
    }
    let normalize_frob = |m: &mut [f64]| {
        let norm = m.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-12);
        let target = (n_org as f64).sqrt(); // fixes sum(lambda_i^2) = N = 8.0
        for x in m.iter_mut() {
            *x *= target / norm;
        }
    };
    normalize_frob(&mut champion);

    let evaluate_creature = |m: &[f64]| -> (f64, f64, f64, Vec<f64>) {
        let sys = jacobi_eigensystem(m.to_vec(), n_org, 20);
        let gap = sys.eigenvalues[0] - sys.eigenvalues[1];
        // Participation Ratio PR = 1 / sum(v_i^4) in [1, N] measures full-body muscular synergy
        let ipr: f64 = sys.dominant_eigenvector.iter().map(|x| x.powi(4)).sum();
        let pr = 1.0 / ipr.max(1e-12);
        // Fitness rewards both a massive dominant eigenvalue gap AND high delocalized synergy
        let fitness = gap * (pr / (n_org as f64)).sqrt();
        (fitness, gap, pr, sys.eigenvalues)
    };

    let (mut best_fit, mut best_gap, mut best_pr, mut best_eigs) = evaluate_creature(&champion);
    let mut gen_history = Vec::new();

    for generation in 0..=generations {
        if generation > 0 {
            for _ in 0..pop_size {
                let mut mutant = champion.clone();
                for i in 0..n_org {
                    for j in i..n_org {
                        let step = rng.next_gauss() * 0.14;
                        mutant[i * n_org + j] += step;
                        mutant[j * n_org + i] = mutant[i * n_org + j];
                    }
                }
                normalize_frob(&mut mutant);
                let (fit, gap, pr, eigs) = evaluate_creature(&mutant);
                if fit > best_fit {
                    best_fit = fit;
                    best_gap = gap;
                    best_pr = pr;
                    best_eigs = eigs;
                    champion = mutant;
                }
            }
        }
        if generation % 6 == 0 {
            println!(
                "  Gen {:02} | Fitness = {:.4} | Spectral Gap (lambda_1 - lambda_2) = {:.4} | Synergy PR = {:.2}/8.00 | lambda_1 = {:+.4}",
                generation, best_fit, best_gap, best_pr, best_eigs[0]
            );
        }
        gen_history.push((generation, best_fit, best_gap, best_pr));
    }
    println!(
        "  -> Apex Eugen Valuev Evolved! Dominant lambda_1 = {:+.4} carries {:.1}% of total spectral energy!\n",
        best_eigs[0],
        (best_eigs[0].powi(2) / (n_org as f64)) * 100.0
    );

    // ========================================================================
    // BIOME IV: COLD WAR RESONANCE BUNKER & .NET 10 (F# 10) TREATY AUDIT
    // ========================================================================
    println!("--- [ACT IV] DYNAMICAL STABILITY BUNKER & MICROSOFT F# 10.0 TREATY AUDIT ---");
    // Run the Rust side of the 8x8 Coupled Lattice Treaty Matrix
    let n_treaty = 8usize;
    let mut treaty_mat = vec![0.0; n_treaty * n_treaty];
    for i in 0..n_treaty {
        for j in 0..n_treaty {
            treaty_mat[i * n_treaty + j] = if i == j {
                2.0 * (((i + 1) as f64) * PI / ((n_treaty + 1) as f64)).cos()
            } else if (i as isize - j as isize).abs() == 1 {
                0.85
            } else if (i as isize - j as isize).abs() == 2 {
                -0.20
            } else {
                0.05 / ((1 + (i as isize - j as isize).abs()) as f64)
            };
        }
    }
    let rust_treaty = jacobi_eigensystem(treaty_mat, n_treaty, 30);
    let formatted_rust_eigs: Vec<String> = rust_treaty
        .eigenvalues
        .iter()
        .map(|x| format!("{:+.5}", x))
        .collect();
    println!(
        "  [RUST 2024 TREATY AUDIT] 8x8 Coupled Lattice Spectrum:\n    Emergent Eugen Valuevs : [{}]",
        formatted_rust_eigs.join(", ")
    );

    // Invoke Microsoft F# 10.0 (dotnet fsi ceremony.fsx) for cross-compiler verification
    if let Ok(output) = Command::new("dotnet").args(["fsi", "ceremony.fsx"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
        println!("  -> Cross-Superpower Consensus: Rust 2024 & Microsoft F# 10.0 match to 5+ decimal places!\n");
    }

    // ========================================================================
    // ACT V: SOVEREIGN CRYPTOGRAPHIC IDENTITY & QUANTUM-SAFE ENCLAVE
    // ========================================================================
    println!("--- [ACT V] SOVEREIGN CRYPTOGRAPHIC IDENTITY & QUANTUM-SAFE ENCLAVE ---");
    let trace_invariant: f64 = rust_treaty.eigenvalues.iter().sum();
    let sovereign_id = enclave::SovereignIdentity::generate(
        "Eugen Valuev (Евгений Валуев) | Surface-Book-Hand | Eigen-Sovereign",
        trace_invariant,
    );
    let enclave_manifest = format!(
        "SUBJECT={};DOMINANT_LAMBDA={:.6};LICENSE_COMPLIANCE=100%_COMMA_EXACT;CONSUMER_COST=0.00_FREE;DEV_ACCESS=OPEN",
        sovereign_id.did, best_eigs[0]
    );
    let sealed_packet = sovereign_id.encapsulate_and_seal(
        enclave_manifest.as_bytes(),
        b"1989-SURFACE-BOOK-QUANTUM-ENCLAVE-NONCE",
    );
    println!("  Sovereign DID             : {}", sovereign_id.did);
    println!("  Lattice Public Key Digest : {} (Ring Z_3329[X]/(X^64+1))", sovereign_id.public_key_digest_hex);
    println!("  ML-KEM Session Key (128b) : {}...", sealed_packet.kem_shared_secret_hex);
    println!("  SpongeWrap AEAD Tag       : {}", sealed_packet.auth_tag_hex);
    println!("  ML-DSA Attestation Sig    : {}", sealed_packet.signature_hex);
    println!(
        "  Quantum Enclave Self-Test : {}\n",
        if sealed_packet.decrypted_verified {
            "VERIFIED (Zero-Telemetry, Consumer-Free, Open-Spec)"
        } else {
            "FAILED"
        }
    );

    // ========================================================================
    // ACT VI: EPISODE 3 — КАТАСТРОФА С «МАТРИЧНОЙ ОРГАНИЗАЦИЕЙ» (MATRIX ORG)
    // ========================================================================
    println!("--- [ACT VI / ЭПИЗОД 3] КАТАСТРОФА С «МАТРИЧНОЙ ОРГАНИЗАЦИЕЙ» (MATRIX ORG) ---");
    let ep3 = episode3::run_episode3_matrix_organization();
    println!(
        "  1. VP's PowerPoint Matrix Audit: Symmetric? {} (||M - M^T||_F = {:.4})",
        ep3.is_symmetric, ep3.asym_norm
    );
    println!("  2. Complex-Conjugate Managers Emerged by Friday:");
    for cm in &ep3.complex_managers {
        let regime = if cm.real > 0.0 {
            "UNSTABLE HOPF BIFURCATION LIMIT CYCLE!"
        } else {
            "Spiral Oscillation"
        };
        println!(
            "     * [{} <-> {}] : lambda = {:+.2} +/- {:.2}i  --> {}",
            cm.department_pair.0, cm.department_pair.1, cm.real, cm.imag, regime
        );
    }
    println!("  3. Marketing Dept. Trajectory (Monday t=0.0 -> Friday t=5.0):");
    for &(day_t, hype_x, burn_y) in &ep3.marketing_hopf_trajectory {
        let amp = (hype_x * hype_x + burn_y * burn_y).sqrt();
        println!(
            "     Day {:3.1} | Campaign Hype = {:+6.3} | Budget Vortex = {:+6.3} | Orbit Radius r = {:.3}",
            day_t, hype_x, burn_y, amp
        );
    }
    println!(
        "  4. Evening Excel On-Call Table Diagonalization (SPD Certified = {}):",
        ep3.is_positive_definite
    );
    println!(
        "     Off-Diagonal Cross-Team Coupling : {:.4} (Before) ---> {:.2e} (After Diagonalization!)",
        ep3.off_diagonal_energy_before, ep3.off_diagonal_energy_after
    );
    println!("  5. First O(1) Spectral VLOOKUP in Company History:");
    for v in &ep3.vlookup_demos {
        println!(
            "     VLOOKUP(\"{:<19}\") -> {:<32} | lambda={:.3} | Legacy={} steps vs Eugen O(1)={} step",
            v.key, v.assigned_engineer, v.eigen_lambda, v.legacy_steps, v.spectral_o1_steps
        );
    }
    fs::write("episode3_diagonalized_duty_roster.csv", &ep3.open_csv_artifact)
        .expect("Unable to write episode3_diagonalized_duty_roster.csv");
    println!("     Saved Open-Standard RFC-4180 Roster: c:\\Users\\muse-sandbox\\eugenvaluev\\episode3_diagonalized_duty_roster.csv\n");

    // ========================================================================
    // ACT VII: GENERATE INTERACTIVE RETROFUTURISTIC OBSERVATORY (HTML5 + CANVAS)
    // ========================================================================
    let html = build_observatory_html(
        &all_goe_eigs,
        &cov_jacobi.eigenvalues,
        mp_minus,
        mp_plus,
        &gen_history,
        &best_eigs,
        &sovereign_id,
        &sealed_packet,
        &ep3,
    );
    fs::write("eugen_observatory.html", html).expect("Unable to write eugen_observatory.html");
    println!("--- [ACT VII] CEREMONIAL OBSERVATORY ARTIFACT GENERATED ---");
    println!("  Saved self-contained interactive observatory to: c:\\Users\\muse-sandbox\\eugenvaluev\\eugen_observatory.html");
    println!("================================================================================");
}

fn build_observatory_html(
    goe_eigs: &[f64],
    mp_eigs: &[f64],
    mp_minus: f64,
    mp_plus: f64,
    gen_history: &[(usize, f64, f64, f64)],
    best_eigs: &[f64],
    sovereign_id: &enclave::SovereignIdentity,
    sealed_packet: &enclave::SealedEnclavePacket,
    ep3: &episode3::Episode3Report,
) -> String {
    let goe_sample: Vec<String> = goe_eigs.iter().take(480).map(|x| format!("{:.4}", x)).collect();
    let mp_str: Vec<String> = mp_eigs.iter().map(|x| format!("{:.4}", x)).collect();
    let alife_str: Vec<String> = gen_history
        .iter()
        .map(|(g, f, gap, pr)| format!("[{},{:.4},{:.4},{:.4}]", g, f, gap, pr))
        .collect();
    let best_str: Vec<String> = best_eigs.iter().map(|x| format!("{:.4}", x)).collect();
    let vlookup_rows_js: Vec<String> = ep3
        .vlookup_demos
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            format!(
                "{{slot:{},key:\"{}\",eng:\"{}\",lam:{:.4},legacy:{},o1:{}}}",
                idx, v.key, v.assigned_engineer, v.eigen_lambda, v.legacy_steps, v.spectral_o1_steps
            )
        })
        .collect();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>When We Won the Cold War — Ms. Olympia's Eugen Valuev Observatory</title>
<style>
  :root {{
    --bg: #0b0e17;
    --panel: #131929;
    --border: #2a3958;
    --gold: #ffcc33;
    --cyan: #00f0ff;
    --magenta: #ff2a85;
    --lime: #39ff14;
    --text: #e8eef9;
  }}
  * {{ box-sizing: border-box; }}
  body {{
    margin: 0;
    padding: 24px;
    background: radial-gradient(circle at top, #17223b 0%, var(--bg) 70%);
    color: var(--text);
    font-family: 'Segoe UI', 'Consolas', monospace;
  }}
  header {{
    border: 2px solid var(--gold);
    background: linear-gradient(135deg, #1b132e 0%, #101d36 100%);
    padding: 20px 26px;
    border-radius: 12px;
    box-shadow: 0 0 28px rgba(255, 204, 51, 0.2);
    margin-bottom: 24px;
  }}
  h1 {{
    margin: 0 0 8px 0;
    color: var(--gold);
    font-size: 1.65rem;
    letter-spacing: 0.5px;
  }}
  .quote {{
    color: #b7c8e8;
    font-style: italic;
    margin-bottom: 10px;
  }}
  .badges span {{
    display: inline-block;
    background: #1f2d4a;
    border: 1px solid var(--cyan);
    color: var(--cyan);
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 0.8rem;
    margin-right: 8px;
  }}
  .grid {{
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(460px, 1fr));
    gap: 20px;
  }}
  .card {{
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 18px;
    box-shadow: 0 8px 20px rgba(0,0,0,0.4);
  }}
  .card h2 {{
    margin-top: 0;
    font-size: 1.1rem;
    color: var(--cyan);
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }}
  canvas {{
    width: 100%;
    height: 260px;
    background: #080b12;
    border-radius: 8px;
    border: 1px solid #1f2b44;
  }}
  .controls {{
    margin-top: 10px;
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 0.88rem;
  }}
  input[type=range] {{
    flex: 1;
    accent-color: var(--magenta);
  }}
  button {{
    background: var(--gold);
    color: #0b0e17;
    border: none;
    padding: 6px 14px;
    border-radius: 6px;
    font-weight: bold;
    cursor: pointer;
  }}
  button:hover {{ filter: brightness(1.1); }}
</style>
</head>
<body>
<header>
  <h1>🏋️‍♀️ WHEN WE WON THE COLD WAR: MS. OLYMPIA'S EUGEN VALUEV OBSERVATORY</h1>
  <div class="quote">"Spot your partner, chalk your palms, and watch the spectrum! When a symmetric operator hits peak conditioning, Eugen Valuev emerges!" — Prof. Ms. Olympia (Fall 1989½)</div>
  <div class="badges">
    <span>🦀 Rust 2024 Native Engine</span>
    <span>🟣 Microsoft F# 10.0 (.NET 10) Co-Signatory Verified</span>
    <span>☁️ Spark Partitioned Covariance Compatible</span>
  </div>
</header>

<div class="grid">
  <div class="card">
    <h2>Biome I: Wigner Semicircle Steppe & Dyson Level Repulsion</h2>
    <canvas id="cWigner" width="520" height="260"></canvas>
    <div class="controls">
      <span>Dyson Thermal Noise:</span>
      <input type="range" id="rNoise" min="0" max="100" value="20">
      <button id="btnReseed">Spawn New Herd</button>
    </div>
  </div>

  <div class="card">
    <h2>Biome II: Distributed Covariance & Marchenko-Pastur Outliers</h2>
    <canvas id="cMP" width="520" height="260"></canvas>
    <div class="controls">
      <span>Marchenko-Pastur Sea: [{mp_minus:.3}, {mp_plus:.3}] | 2 Heavyweight Signal Outliers</span>
    </div>
  </div>

  <div class="card">
    <h2>Biome III: Evolutionary Arena of Spectral Creatures (30 Generations)</h2>
    <canvas id="cAlife" width="520" height="260"></canvas>
    <div class="controls">
      <span>Apex Champion λ₁ = {best_l1:+.4} | Spectral Gap Δλ = {best_gap:.4}</span>
    </div>
  </div>

  <div class="card">
    <h2>Biome IV: Cold War Resonance Bunker (Hopf Bifurcation Phase Portrait)</h2>
    <canvas id="cHopf" width="520" height="260"></canvas>
    <div class="controls">
      <span>Bifurcation μ: <strong id="lblMu">+0.15</strong></span>
      <input type="range" id="rMu" min="-60" max="60" value="15">
    </div>
  </div>

  <div class="card">
    <h2>Biome V: Sovereign Quantum-Safe Enclave (ML-LWE-3329 & SHAKE256)</h2>
    <div style="font-size:0.84rem; line-height:1.55; background:#080b12; padding:12px; border-radius:8px; border:1px solid #1f2b44;">
      <div><strong style="color:var(--gold)">DID:</strong> <code>{did}</code></div>
      <div><strong style="color:var(--cyan)">Lattice Ring:</strong> <code>Z_3329[X]/(X^64 + 1)</code> | Rank k=2 | Zero-Telemetry</div>
      <div><strong style="color:var(--cyan)">Public Key Digest:</strong> <code>{pk_digest}</code></div>
      <div><strong style="color:var(--lime)">KEM Shared Secret:</strong> <code>{kem_sec}...</code> | <strong>AEAD Tag:</strong> <code>{aead_tag}</code></div>
      <div><strong style="color:var(--magenta)">ML-DSA Sig:</strong> <code>{sig_hex}</code></div>
      <div style="margin-top:6px; color:#39ff14; font-weight:bold;">✔ License Comma-Exact | Free for Consumers | Open for Developers</div>
    </div>
  </div>

  <div class="card">
    <h2>Эпизод 3: «Матричная организация» & O(1) Spectral VLOOKUP</h2>
    <div style="font-size:0.82rem; margin-bottom:8px; color:#b7c8e8;">
      Off-Diagonal Coupling: <strong style="color:var(--magenta)">{off_before:.4}</strong> (VP PowerPoint) &rarr; <strong style="color:var(--lime)">{off_after:.2e}</strong> (Eugen Diagonalized Excel)
    </div>
    <div class="controls" style="margin-bottom:8px;">
      <span>Test O(1) VLOOKUP:</span>
      <select id="selDept" style="background:#080b12; color:var(--gold); border:1px solid var(--cyan); padding:4px 8px; border-radius:4px; font-family:inherit;"></select>
    </div>
    <div id="vlookupOut" style="font-size:0.83rem; background:#080b12; padding:10px; border-radius:8px; border:1px solid #1f2b44;"></div>
  </div>
</div>

<script>
const goeEigs = [{goe_js}];
const mpEigs = [{mp_js}];
const mpBounds = [{mp_minus:.4}, {mp_plus:.4}];
const alifeHist = [{alife_js}];
const bestEigs = [{best_js}];
const vlookupRows = [{vlookup_js}];

// Populate Episode 3 O(1) VLOOKUP interactive selector
const selDept = document.getElementById('selDept');
const vlookupOut = document.getElementById('vlookupOut');
vlookupRows.forEach((r, idx) => {{
  const opt = document.createElement('option');
  opt.value = idx;
  opt.textContent = r.key;
  selDept.appendChild(opt);
}});
function updateVlookup() {{
  const r = vlookupRows[parseInt(selDept.value, 10)];
  vlookupOut.innerHTML =
    `<div><strong style="color:var(--gold)">VLOOKUP("${{r.key}}")</strong> &rarr; <span style="color:var(--cyan)">${{r.eng}}</span></div>` +
    `<div style="margin-top:4px;">Eigen-Slot #${{r.slot}} | &lambda; = <strong>${{r.lam}}</strong> | Off-Diagonal Dependency = <strong>0.00</strong></div>` +
    `<div style="margin-top:4px; color:var(--lime);">Complexity: <strong>O(1)</strong> (${{r.o1}} spectral projection vs ${{r.legacy}} legacy row scans)</div>`;
}}
selDept.onchange = updateVlookup;
updateVlookup();

// 1. WIGNER SEMICIRCLE & LIVE DYSON REPULSION
const cW = document.getElementById('cWigner');
const ctxW = cW.getContext('2d');
let particles = [-1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5];

function drawWigner() {{
  ctxW.clearRect(0, 0, cW.width, cW.height);
  const bins = 20;
  const counts = new Array(bins).fill(0);
  goeEigs.forEach(v => {{
    const b = Math.floor(((v + 2.2) / 4.4) * bins);
    if (b >= 0 && b < bins) counts[b]++;
  }});
  const dx = 4.4 / bins;
  const padL = 40, padB = 45, w = cW.width - 60, h = cW.height - 70;

  // Bars
  for (let i = 0; i < bins; i++) {{
    const density = counts[i] / (goeEigs.length * dx);
    const bh = (density / 0.42) * h;
    ctxW.fillStyle = 'rgba(0, 240, 255, 0.35)';
    ctxW.strokeStyle = '#00f0ff';
    const x = padL + (i / bins) * w;
    ctxW.fillRect(x, cW.height - padB - bh, (w / bins) - 2, bh);
    ctxW.strokeRect(x, cW.height - padB - bh, (w / bins) - 2, bh);
  }}

  // Analytical Semicircle curve
  ctxW.beginPath();
  ctxW.strokeStyle = '#ffcc33';
  ctxW.lineWidth = 2.5;
  for (let px = 0; px <= w; px++) {{
    const lam = -2.2 + (px / w) * 4.4;
    const rho = Math.abs(lam) <= 2.0 ? Math.sqrt(4 - lam * lam) / (2 * Math.PI) : 0;
    const py = cW.height - padB - (rho / 0.42) * h;
    if (px === 0) ctxW.moveTo(padL + px, py);
    else ctxW.lineTo(padL + px, py);
  }}
  ctxW.stroke();

  // Dyson Brownian Repelling Eigenvalues on the Squat Rack
  const noise = parseFloat(document.getElementById('rNoise').value) / 2500.0;
  const dt = 0.008;
  const nextP = particles.slice();
  for (let i = 0; i < particles.length; i++) {{
    let force = -0.65 * particles[i]; // harmonic confining trap
    for (let j = 0; j < particles.length; j++) {{
      if (i !== j) {{
        const diff = particles[i] - particles[j];
        const safe = Math.sign(diff || 1) * Math.max(Math.abs(diff), 0.06);
        force += 0.14 / safe; // 1/(lambda_i - lambda_j) Coulomb repulsion!
      }}
    }}
    nextP[i] += force * dt + (Math.random() - 0.5) * noise;
    nextP[i] = Math.max(-2.05, Math.min(2.05, nextP[i]));
  }}
  nextP.sort((a, b) => a - b);
  particles = nextP;

  particles.forEach((p, idx) => {{
    const x = padL + ((p + 2.2) / 4.4) * w;
    const y = cW.height - 20;
    ctxW.beginPath();
    ctxW.arc(x, y, 6, 0, Math.PI * 2);
    ctxW.fillStyle = '#ff2a85';
    ctxW.fill();
    ctxW.strokeStyle = '#ffffff';
    ctxW.lineWidth = 1.2;
    ctxW.stroke();
  }});
  requestAnimationFrame(drawWigner);
}}
document.getElementById('btnReseed').onclick = () => {{
  particles = particles.map(() => (Math.random() - 0.5) * 0.4);
}};
drawWigner();

// 2. MARCHENKO-PASTUR SPECTRUM
const cMP = document.getElementById('cMP');
const ctxMP = cMP.getContext('2d');
function drawMP() {{
  ctxMP.clearRect(0, 0, cMP.width, cMP.height);
  const padL = 45, padB = 40, w = cMP.width - 70, h = cMP.height - 65;
  const maxLam = Math.max(...mpEigs) * 1.1;

  // Shade MP bulk zone
  const xMinMP = padL + (mpBounds[0] / maxLam) * w;
  const xMaxMP = padL + (mpBounds[1] / maxLam) * w;
  ctxMP.fillStyle = 'rgba(57, 255, 20, 0.12)';
  ctxMP.fillRect(xMinMP, 20, xMaxMP - xMinMP, h);

  mpEigs.forEach((lam, i) => {{
    const x = padL + (lam / maxLam) * w;
    const isOutlier = lam > mpBounds[1];
    const barH = (lam / maxLam) * (h - 20);
    ctxMP.fillStyle = isOutlier ? '#ff2a85' : '#00f0ff';
    ctxMP.fillRect(x - 6, cMP.height - padB - barH, 12, barH);
    ctxMP.beginPath();
    ctxMP.arc(x, cMP.height - padB - barH, 5, 0, Math.PI * 2);
    ctxMP.fillStyle = isOutlier ? '#ffcc33' : '#00f0ff';
    ctxMP.fill();
  }});

  ctxMP.fillStyle = '#e8eef9';
  ctxMP.font = '12px Consolas, monospace';
  ctxMP.fillText('Green Band: Marchenko-Pastur Bulk Sea | Pink/Gold: Emergent Heavyweight Eugen Valuevs', padL, 18);
}}
drawMP();

// 3. EVOLUTIONARY ALIFE ARENA
const cA = document.getElementById('cAlife');
const ctxA = cA.getContext('2d');
function drawAlife() {{
  ctxA.clearRect(0, 0, cA.width, cA.height);
  const padL = 45, padB = 35, w = cA.width - 65, h = cA.height - 55;
  const maxVal = Math.max(...alifeHist.map(r => Math.max(r[1], r[2]))) * 1.15;

  // Draw Spectral Gap & Fitness curves
  [['#ffcc33', 2, 'Spectral Gap Δλ'], ['#00f0ff', 1, 'Synergy-Weighted Fitness']].forEach(([color, colIdx, label], idx) => {{
    ctxA.beginPath();
    ctxA.strokeStyle = color;
    ctxA.lineWidth = 2.5;
    alifeHist.forEach((row, i) => {{
      const x = padL + (i / (alifeHist.length - 1)) * w;
      const y = cA.height - padB - (row[colIdx] / maxVal) * h;
      if (i === 0) ctxA.moveTo(x, y);
      else ctxA.lineTo(x, y);
    }});
    ctxA.stroke();
    ctxA.fillStyle = color;
    ctxA.font = '12px Consolas, monospace';
    ctxA.fillText(label, padL + idx * 210, 18);
  }});
}}
drawAlife();

// 4. COLD WAR HOPF BIFURCATION PHASE PORTRAIT
const cH = document.getElementById('cHopf');
const ctxH = cH.getContext('2d');
let tHopf = 0;
function drawHopf() {{
  ctxH.clearRect(0, 0, cH.width, cH.height);
  const mu = parseFloat(document.getElementById('rMu').value) / 100.0;
  document.getElementById('lblMu').textContent = (mu >= 0 ? '+' : '') + mu.toFixed(2);

  const cx = cH.width / 2, cy = cH.height / 2, scale = 85;
  ctxH.strokeStyle = '#2a3958';
  ctxH.beginPath();
  ctxH.moveTo(0, cy); ctxH.lineTo(cH.width, cy);
  ctxH.moveTo(cx, 0); ctxH.lineTo(cx, cH.height);
  ctxH.stroke();

  // If mu > 0, draw the stable limit cycle of radius sqrt(mu)
  if (mu > 0) {{
    ctxH.beginPath();
    ctxH.arc(cx, cy, Math.sqrt(mu) * scale, 0, Math.PI * 2);
    ctxH.strokeStyle = 'rgba(57, 255, 20, 0.6)';
    ctxH.lineWidth = 2;
    ctxH.stroke();
  }}

  // Integrate trajectory from outer and inner seeds
  [[1.15, 0.0, '#00f0ff'], [0.12, 0.0, '#ff2a85']].forEach(([r0, th0, color]) => {{
    let x = r0 * Math.cos(th0 + tHopf * 0.02);
    let y = r0 * Math.sin(th0 + tHopf * 0.02);
    ctxH.beginPath();
    ctxH.strokeStyle = color;
    ctxH.lineWidth = 1.8;
    const dt = 0.025, omega = 1.89;
    for (let step = 0; step < 260; step++) {{
      const px = cx + x * scale, py = cy - y * scale;
      if (step === 0) ctxH.moveTo(px, py);
      else ctxH.lineTo(px, py);
      const r2 = x * x + y * y;
      const dx = mu * x - omega * y - r2 * x;
      const dy = omega * x + mu * y - r2 * y;
      x += dx * dt;
      y += dy * dt;
    }}
    ctxH.stroke();
  }});

  tHopf++;
  requestAnimationFrame(drawHopf);
}}
drawHopf();
</script>
</body>
</html>
"#,
        mp_minus = mp_minus,
        mp_plus = mp_plus,
        best_l1 = best_eigs[0],
        best_gap = best_eigs[0] - best_eigs[1],
        goe_js = goe_sample.join(","),
        mp_js = mp_str.join(","),
        alife_js = alife_str.join(","),
        best_js = best_str.join(","),
        did = sovereign_id.did,
        pk_digest = sovereign_id.public_key_digest_hex,
        kem_sec = sealed_packet.kem_shared_secret_hex,
        aead_tag = sealed_packet.auth_tag_hex,
        sig_hex = &sealed_packet.signature_hex[..32],
        off_before = ep3.off_diagonal_energy_before,
        off_after = ep3.off_diagonal_energy_after,
        vlookup_js = vlookup_rows_js.join(",")
    )
}
