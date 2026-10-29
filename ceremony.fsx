// ============================================================================
// CEREMONIAL TREATY VERIFIER: Microsoft F# 10.0 (.NET 10.0.403)
// "When We Won the Cold War" — Ms. Olympia's Co-Signatory Spectral Audit
// ============================================================================
open System
open System.Numerics

[<Measure>] type rad
[<Measure>] type sec

type TreatyRegime =
    | DampedDetente of decayRate: float * freq: float<rad/sec>
    | CriticalHopfOrbit of freq: float<rad/sec>
    | RunawayResonance of growthRate: float * freq: float<rad/sec>

let classifyEigenvalue (eig: Complex) : TreatyRegime =
    let w = abs eig.Imaginary * 1.0<rad/sec>
    if abs eig.Real < 1e-9 then CriticalHopfOrbit w
    elif eig.Real < 0.0 then DampedDetente (eig.Real, w)
    else RunawayResonance (eig.Real, w)

// Cyclic Jacobi solver in immutable-first F# 10
let jacobiSymmetric (matrix: float[,]) (sweeps: int) : float[] =
    let n = Array2D.length1 matrix
    let a = Array2D.copy matrix
    for _ in 1 .. sweeps do
        for p in 0 .. n - 2 do
            for q in p + 1 .. n - 1 do
                let apq = a.[p, q]
                if abs apq > 1e-13 then
                    let app = a.[p, p]
                    let aqq = a.[q, q]
                    let tau = (aqq - app) / (2.0 * apq)
                    let t =
                        if tau >= 0.0 then 1.0 / (tau + sqrt(1.0 + tau * tau))
                        else -1.0 / (-tau + sqrt(1.0 + tau * tau))
                    let c = 1.0 / sqrt(1.0 + t * t)
                    let s = t * c
                    a.[p, p] <- app - t * apq
                    a.[q, q] <- aqq + t * apq
                    a.[p, q] <- 0.0
                    a.[q, p] <- 0.0
                    for r in 0 .. n - 1 do
                        if r <> p && r <> q then
                            let arp = a.[r, p]
                            let arq = a.[r, q]
                            a.[r, p] <- c * arp - s * arq
                            a.[p, r] <- a.[r, p]
                            a.[r, q] <- s * arp + c * arq
                            a.[q, r] <- a.[r, q]
    [| for i in 0 .. n - 1 -> a.[i, i] |] |> Array.sortDescending

// Deterministic treaty matrix H_treaty (8x8 coupled oscillator lattice + Cold War perturbation)
let n = 8
let treatyMatrix =
    Array2D.init n n (fun i j ->
        if i = j then 2.0 * cos(float (i + 1) * Math.PI / float (n + 1))
        elif abs (i - j) = 1 then 0.85
        elif abs (i - j) = 2 then -0.20
        else 0.05 / float (1 + abs (i - j)))

let eigs = jacobiSymmetric treatyMatrix 30
let traceMat = [| 0 .. n - 1 |] |> Array.sumBy (fun i -> treatyMatrix.[i, i])
let sumEigs = Array.sum eigs
let frobMat =
    seq { for i in 0 .. n - 1 do for j in 0 .. n - 1 -> treatyMatrix.[i, j] * treatyMatrix.[i, j] }
    |> Seq.sum
let sumSqEigs = eigs |> Array.sumBy (fun l -> l * l)

printfn "  [F# 10.0 TREATY AUDIT] 8x8 Coupled Lattice Spectrum:"
printfn "    Emergent Eugen Valuevs : [%s]" (String.Join(", ", eigs |> Array.map (fun x -> sprintf "%+.5f" x)))
printfn "    Trace Invariant Check  : Tr(H) = %+.6f | Sum(lambda)   = %+.6f (delta = %.2e)" traceMat sumEigs (abs (traceMat - sumEigs))
printfn "    Frobenius Check        : ||H||^2 = %.6f | Sum(lambda^2) = %.6f (delta = %.2e)" frobMat sumSqEigs (abs (frobMat - sumSqEigs))

for mu in [ -0.40; 0.00; +0.35 ] do
    let c = Complex(mu, 1.89)
    match classifyEigenvalue c with
    | DampedDetente (r, w) ->
        printfn "    Hopf Probe (mu=%+.2f)  : lambda = %+.2f +/- %.2fi -> DAMPED DETENTE (decay=%.2f, omega=%.2f rad/s)" mu r (float w) r (float w)
    | CriticalHopfOrbit w ->
        printfn "    Hopf Probe (mu=%+.2f)  : lambda = +0.00 +/- %.2fi -> MS. OLYMPIA LIMIT CYCLE (omega=%.2f rad/s)" mu (float w) (float w)
    | RunawayResonance (g, w) ->
        printfn "    Hopf Probe (mu=%+.2f)  : lambda = %+.2f +/- %.2fi -> RUNAWAY RESONANCE (growth=%+.2f, omega=%.2f rad/s)" mu g (float w) g (float w)

// Episode 3 Audit in F# 10.0: Marketing Dept. Complex-Conjugate Manager Pair (mu = +0.28, omega = 1.95 rad/day)
let ep3Marketing = Complex(0.28, 1.95)
match classifyEigenvalue ep3Marketing with
| RunawayResonance (g, w) ->
    printfn "    Episode 3 VP Matrix    : lambda = %+.2f +/- %.2fi -> FRIDAY MARKETING HOPF BIFURCATION CONFIRMED!" g (float w)
| _ -> ()


