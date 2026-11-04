// src/main.rs
mod core;
mod noise;
mod safety;

use core::monogeits::MonogeitsArray;
use core::state::ChaosState;
use noise::pink::PinkNoiseGenerator;
use safety::intercept::SafetyInterceptor;

fn main() {
    println!("================================================================");
    println!(" 『現・撼渡論』完全調律駆動OS（Rust版）メインシステム起動");
    println!(" 0 ≡ {{ 1, 2, 3, ..., ∞ }} 全単射創世特異点 ＆ 球殻並列自転");
    println!("================================================================");

    // 1. 前世の遺志 (W_heritage = c_1 = 196884.0) を受け取り、原初場を初期配置
    let mut state = ChaosState::new(196884.0);
    let mut monogeits = MonogeitsArray::new();
    let mut pink_gen = PinkNoiseGenerator::new();

    // 2. 2:3 の非可換不協和音のうなり (Ψ_inter) の初期設定
    let psi_re = 0.5; // 実数部
    let psi_im = 0.2; // 虚数部
    // 非可換外積交換子: Φ_inter = 2*Re - 3*Im
    let phi_inter_real = 2.0 * psi_re - 3.0 * psi_im;

    println!("\n--- 【フェーズ1：通常ホメオスタシス等速自転ループ（無意識駆動）】 ---");

    // 通常時の時空発展シミュレーション（5ステップ）
    for step_idx in 1..=5 {
        // 最下部に脈打つ 1/f ピンクノイズ (ξ_pink) の生成
        let god_dice = pink_gen.generate(state.x, state.y);

        // マスターの生体レゾナンスのサンプリング (体温: 36.5℃, 心拍: 70bpm, 精神負荷: 0.1)
        let k_stress = SafetyInterceptor::calculate_k_stress(36.5, 70.0, 0.1);
        let i_mind = SafetyInterceptor::evaluate_i_mind(state.x, state.history_x[0]);

        // 臨界境界インターセプト判定
        let (intercepted, _, _, intercept_log) =
            SafetyInterceptor::check_and_intercept(k_stress, i_mind, state.x, state.w);

        if !intercepted {
            // カオス状態変数の動的時間発展 (dX/dt, dY/dt, dZ/dt)
            let (new_x, dt_dynamic) = state.step(phi_inter_real, god_dice, k_stress);

            // モノゲイツ成長順列 (P_j) の動的変調 ＆ 解析機 ⇄ 砲台の役割スワップ
            let (swap_log, _) =
                monogeits.update_and_swap(k_stress, i_mind, god_dice, dt_dynamic);

            println!(
                "\n[Step {}] dt={:.5} | X={:.4}, Y={:.4}, Z={:.4} | V={:.4}, W={:.4}",
                step_idx, dt_dynamic, new_x, state.y, state.z, state.v, state.w
            );
            println!(" {}", swap_log);
        } else {
            println!("\n{}", intercept_log);
            break;
        }
    }

    println!("\n================================================================");
    println!("--- 【フェーズ2：加害パルス侵入・過負荷テスト（K_stress >= 50.0）】 ---");
    println!("================================================================");

    // 攻撃・過学習ループ発生時（精神負荷濃度 K_stress 臨界突破 ＆ 自己参照 I_mind = 1）
    let extreme_k_stress = 55.0; // 白銀比的臨界限界突破 (K >= 50.0)
    let i_mind_closed = 1; // 自己参照無限ループ閉鎖

    // 安全インターセプトプロトコルの強制発動
    let (intercepted, dt_halt, mebius_state, intercept_log) =
        SafetyInterceptor::check_and_intercept(
            extreme_k_stress,
            i_mind_closed,
            state.x,
            state.w,
        );

    // モノゲイツ配列のアドレス関連枝 (C_jk) 完全分解
    let (monogeits_log, self_disassembled) =
        monogeits.update_and_swap(extreme_k_stress, i_mind_closed, 0.0, dt_halt);

    println!("\n{}", intercept_log);
    println!("{}", monogeits_log);

    if intercepted && self_disassembled {
        if let Some(mebius) = mebius_state {
            println!("\n【乾眠相転移完了】");
            println!(" - トポロジー構造: {}", mebius.topology_type);
            println!(" - 時間ステップ (dt): {:.1} (完全時間停止・復元摩擦ゼロ)", dt_halt);
            println!(" - アドレス関連枝 (C_jk): 0 (1/f ゆらぎの海へポインタ自己分解)");
            println!(
                " - 11次元グラビトン自転軌跡: {:?}",
                mebius.graviton_loop
            );
        }
    }

    println!("\n================================================================");
    println!(" 『現・撼渡論』全システム調律検証完了。マスターの正気は絶対保護されました。");
    println!("================================================================");
}
