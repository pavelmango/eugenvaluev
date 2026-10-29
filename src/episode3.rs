// ============================================================================
// EPISODE 3: КАТАСТРОФА С «МАТРИЧНОЙ ОРГАНИЗАЦИЕЙ» (THE MATRIX ORGANIZATION)
// 1. VP's PowerPoint Asymmetric Matrix -> Complex-Conjugate Managers &
//    Marketing Dept. Friday Hopf Bifurcation!
// 2. Eugen's Symmetric Positive-Definite Diagonalization of the Excel On-Call
//    Roster -> Zero Cross-Team Dependencies & O(1) Spectral VLOOKUP!
// ============================================================================

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ComplexPair {
    pub department_pair: (&'static str, &'static str),
    pub real: f64,
    pub imag: f64,
}

#[allow(dead_code)]
pub struct Episode3Report {
    pub asym_norm: f64,
    pub is_symmetric: bool,
    pub is_positive_definite: bool,
    pub complex_managers: Vec<ComplexPair>,
    pub marketing_hopf_trajectory: Vec<(f64, f64, f64)>, // (day_t, campaign_hype, budget_burn)
    pub diagonalized_eigenvalues: Vec<f64>,
    pub off_diagonal_energy_before: f64,
    pub off_diagonal_energy_after: f64,
    pub vlookup_demos: Vec<VlookupComparison>,
    pub open_csv_artifact: String,
}

pub struct VlookupComparison {
    pub key: &'static str,
    pub assigned_engineer: &'static str,
    pub eigen_lambda: f64,
    pub legacy_steps: usize,
    pub spectral_o1_steps: usize,
}

/// Analyze the VP's Onboarding PowerPoint Matrix (Horizontal Products x Vertical Platform Disciplines)
/// and then perform Eugen Valuev's Evening Excel Diagonalization & O(1) VLOOKUP upgrade.
pub fn run_episode3_matrix_organization() -> Episode3Report {
    let nodes = [
        "Surface-Book-Core",
        "Excel-Calc-Engine",
        "PowerPoint-SlideVM",
        "NT-Kernel-Guard",
        "Marketing-Campaign",
        "Brand-Synergy-PR",
    ];
    let n = nodes.len();

    // 1. VP's Onboarding Matrix M_vp (Asymmetric & Non-Positive-Definite!)
    // Notice blocks (0,1), (2,3), and especially (4,5) [Marketing <-> Brand-Synergy]
    // where skew-symmetric cross-reporting (b = -c = 1.95) and positive feedback (mu = +0.28)
    // spawn complex-conjugate managers and drive Marketing into a Hopf bifurcation by Friday!
    #[rustfmt::skip]
    let m_vp: Vec<f64> = vec![
        // Surface  Excel   PPT     Kernel  Mktg    Brand
         1.20,   -0.85,   0.15,   0.00,   0.10,   0.00, // Surface-Book-Core
         0.85,    1.20,   0.00,   0.12,   0.00,   0.05, // Excel-Calc-Engine
        -0.15,    0.00,   0.95,  -0.60,   0.08,   0.00, // PowerPoint-SlideVM
         0.00,   -0.12,   0.60,   0.95,   0.00,   0.08, // NT-Kernel-Guard
        -0.10,    0.00,  -0.08,   0.00,   0.28,  -1.95, // Marketing-Campaign (mu = +0.28 > 0!)
         0.00,   -0.05,   0.00,  -0.08,   1.95,   0.28, // Brand-Synergy-PR   (omega = 1.95 rad/day!)
    ];

    // Check Symmetry ||M - M^T||_F
    let mut asym_sq = 0.0;
    for i in 0..n {
        for j in 0..n {
            let diff = m_vp[i * n + j] - m_vp[j * n + i];
            asym_sq += diff * diff;
        }
    }
    let asym_norm = asym_sq.sqrt();
    let is_symmetric = asym_norm < 1e-12;

    // Extract 2x2 Real Schur / Complex-Conjugate Manager blocks along the organizational hierarchy
    // For a 2x2 block [[a, -w], [w, d]], trace = a+d, det = a*d + w^2 -> lambda = (a+d)/2 +/- i*sqrt(det - trace^2/4)
    let mut complex_managers = Vec::new();
    for pair_idx in [(0, 1), (2, 3), (4, 5)] {
        let (i, j) = pair_idx;
        let a = m_vp[i * n + i];
        let b = m_vp[i * n + j];
        let c = m_vp[j * n + i];
        let d = m_vp[j * n + j];
        let tr = a + d;
        let det = a * d - b * c;
        let disc = tr * tr - 4.0 * det;
        if disc < 0.0 {
            complex_managers.push(ComplexPair {
                department_pair: (nodes[i], nodes[j]),
                real: 0.5 * tr,
                imag: 0.5 * (-disc).sqrt(),
            });
        }
    }

    // 2. Simulate Marketing Department (Monday t=0.0 to Friday t=5.0)
    // Stuart-Landau / Hopf normal form driven by the (4,5) complex-conjugate eigenvalue pair:
    // dx/dt = mu(t)*x - omega*y - (x^2 + y^2)*x
    // dy/dt = omega*x + mu(t)*y - (x^2 + y^2)*y
    // where mu(t) ramps from -0.15 on Monday morning past the Hopf threshold mu=0 on Wednesday to +0.36 by Friday!
    let mut x = 0.12_f64;
    let mut y = 0.04_f64;
    let omega = 1.95_f64;
    let dt = 0.05_f64;
    let mut marketing_hopf_trajectory = Vec::new();
    for step in 0..=100 {
        let day_t = (step as f64) * dt; // 0.0 (Mon) .. 5.0 (Fri)
        let mu_t = -0.15 + 0.102 * day_t; // Crosses 0.0 on Tuesday afternoon, hits +0.36 on Friday!
        if step % 10 == 0 {
            marketing_hopf_trajectory.push((day_t, x, y));
        }
        let r2 = x * x + y * y;
        let dx = mu_t * x - omega * y - r2 * x;
        let dy = omega * x + mu_t * y - r2 * y;
        x += dx * dt;
        y += dy * dt;
    }

    // 3. Evening Excel On-Call Roster Diagonalization:
    // Eugen constructs the Symmetric Positive-Definite (SPD) On-Call Coupling Matrix
    // S_duty = (M_vp^T * M_vp) + 0.5 * I, then diagonalizes it via Orthogonal Jacobi:
    // S_duty = Q * Lambda * Q^T  ==>  In the eigenbasis Q, all off-diagonal dependencies vanish!
    let mut s_duty = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += m_vp[k * n + i] * m_vp[k * n + j];
            }
            if i == j {
                sum += 0.50 + 0.17 * (i as f64); // Distinct spectral slots for O(1) lookup
            }
            s_duty[i * n + j] = sum;
        }
    }

    let off_diagonal_energy_before: f64 = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&(i, j)| i != j)
        .map(|(i, j)| s_duty[i * n + j].powi(2))
        .sum::<f64>()
        .sqrt();

    // Cyclic Jacobi diagonalization of S_duty
    let mut a = s_duty.clone();
    for _sweep in 0..30 {
        for p in 0..(n - 1) {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                if apq.abs() > 1e-14 {
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
                }
            }
        }
    }

    let off_diagonal_energy_after: f64 = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&(i, j)| i != j)
        .map(|(i, j)| a[i * n + j].powi(2))
        .sum::<f64>()
        .sqrt();

    let mut diagonalized_eigenvalues: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    diagonalized_eigenvalues.sort_by(|x, y| y.partial_cmp(x).unwrap());
    let is_positive_definite = diagonalized_eigenvalues.iter().all(|&lam| lam > 0.0);

    // 4. O(1) Spectral VLOOKUP Engine vs Legacy O(N) Linear Scan
    // In Eugen's diagonalized table, every eigen-channel i is orthogonal (Lambda_{i,j} = 0 for i != j)
    // and indexed directly in O(1) via spectral eigen-slot addressing!
    let engineers = [
        "Евгений Валуев (Eigen-Lead)",
        "Ада Лавлейс (Calc-Architect)",
        "Джеймс Уилкинсон (JH-Pivot)",
        "Карл Якоби (Rotation-SRE)",
        "Эмми Нётер (Invariant-OnCall)",
        "Андрей Колмогоров (Measure-Ops)",
    ];

    let mut o1_spectral_index: HashMap<&'static str, usize> = HashMap::with_capacity(n);
    for (idx, &dept) in nodes.iter().enumerate() {
        o1_spectral_index.insert(dept, idx);
    }

    let mut vlookup_demos = Vec::new();
    for (idx, &query_key) in nodes.iter().enumerate() {
        // Legacy VLOOKUP(..., FALSE) scans row-by-row from 0..=idx: (idx + 1) comparisons
        let legacy_steps = idx + 1;
        // Eugen's Diagonalized O(1) VLOOKUP resolves directly in 1 step via orthogonal projector P_k
        let slot = *o1_spectral_index.get(query_key).unwrap();
        vlookup_demos.push(VlookupComparison {
            key: query_key,
            assigned_engineer: engineers[slot],
            eigen_lambda: diagonalized_eigenvalues[slot],
            legacy_steps,
            spectral_o1_steps: 1,
        });
    }

    // 5. Generate Open-Format RFC-4180 CSV Roster (Free for Consumers & Open for Developers!)
    let mut csv = String::from(
        "eigen_slot_id,department_unit,diagonal_eigenvalue_lambda,cross_team_coupling_off_diag,on_call_guardian,vlookup_complexity\n",
    );
    for (i, demo) in vlookup_demos.iter().enumerate() {
        csv.push_str(&format!(
            "{},{},{:.8},{:.2e},\"{}\",O(1)\n",
            i, demo.key, demo.eigen_lambda, off_diagonal_energy_after, demo.assigned_engineer
        ));
    }

    Episode3Report {
        asym_norm,
        is_symmetric,
        is_positive_definite,
        complex_managers,
        marketing_hopf_trajectory,
        diagonalized_eigenvalues,
        off_diagonal_energy_before,
        off_diagonal_energy_after,
        vlookup_demos,
        open_csv_artifact: csv,
    }
}
