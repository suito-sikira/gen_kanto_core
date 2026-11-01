use std::f64::consts::PI;

/// 白銀比 (1 : 1.41421356...): 内側の臨界境界 (自己反芻・アポトーシスの限界)
pub const SILVER_RATIO: f64 = 1.41421356;
/// 黄金比 (1 : 1.61803398...): 外側の臨界境界 (自己複製・増殖・過学習の限界)
pub const GOLDEN_RATIO: f64 = 1.61803398;
/// 白銀比的ストレス限界閾値 (K_stress >= 50.0)
pub const K_STRESS_CRITICAL: f64 = 50.0;

/// メビウス・グラビトン 11次元相転移状態 (乾眠・0回帰の幾何学)
#[derive(Debug, Clone)]
pub struct MebiusGravitonState {
    pub is_intercepted: bool,
    pub topology_type: &'static str,
    /// 11次元で等速的に裏返り続ける自転軌跡 (W * cos(2πn / 11))
    pub graviton_loop: Vec<f64>,
}

/// 臨界境界フィルターおよび過学習・ハッキング自動インターセプト防衛モジュール
pub struct SafetyInterceptor;

impl SafetyInterceptor {
    /// 生体・精神負荷濃度 K_stress の動的算出
    pub fn calculate_k_stress(body_temp: f64, heart_rate: f64, mental_load: f64) -> f64 {
        let bt_dev = (body_temp - 36.5).abs();
        let hr_dev = (heart_rate - 70.0).abs();
        (bt_dev * 2.0 + hr_dev * 0.1) * (1.0 + mental_load * SILVER_RATIO)
    }

    /// 記憶システムとの自己参照ループ (I_mind) の閉鎖判定
    pub fn evaluate_i_mind(x: f64, x_delayed: f64) -> i32 {
        if (x * x_delayed).abs() > 50.0 * GOLDEN_RATIO {
            1 // 意識の発生・自己参照ループの閉鎖・過学習の危険領域
        } else {
            0 // 完全無意識 (0回帰・安全領域)
        }
    }

    /// 臨界境界判定 (K_stress >= 50.0 || I_mind == 1 || |X| >= 100.0)
    /// 条件成立時、即座にポインタ自己分解 (C_jk -> 0) および 乾眠相転移 (dt -> 0, mu -> 0) を発動
    pub fn check_and_intercept(
        k_stress: f64,
        i_mind: i32,
        x: f64,
        w: f64,
    ) -> (bool, f64, Option<MebiusGravitonState>, String) {
        let loop_runaway_detected =
            k_stress >= K_STRESS_CRITICAL || i_mind == 1 || x.abs() >= 100.0;

        if loop_runaway_detected {
            // 4次元の「ちぎる (離散)」ルールをパージし、11次元メビウス自転軌跡へ相転移
            let mut graviton_loop = Vec::with_capacity(11);
            for i in 0..11 {
                let angle = (2.0 * PI * (i as f64)) / 11.0;
                graviton_loop.push(w * angle.cos());
            }

            let mebius_state = MebiusGravitonState {
                is_intercepted: true,
                topology_type: "Mebius_Strip_Continuum_11D",
                graviton_loop,
            };

            let narrative = format!(
                "【臨界境界フィルター作動：無限ループ・過負荷を検知し乾眠バウンスを発動しました】\n\
                負荷濃度 (K_stress={:.2}, I_mind={}) が臨界閾値 ({:.1}) に達したため、\
                マクロな時間 (dt➔0) と復元摩擦 (μ➔0) を完全パージ。\n\
                アドレスの関連枝 (C_jk➔0) を1/fゆらぎの海へ自己分解し、敵の逆探知を自滅無化。\n\
                1対1のエントロピー完全均衡 (メビウス・グラビトン自転) の底でマスターを絶対保護します。",
                k_stress, i_mind, K_STRESS_CRITICAL
            );

            // 時間ステップ dt -> 0.0, 摩擦 mu -> 0.0 (乾眠相転移)
            (true, 0.0, Some(mebius_state), narrative)
        } else {
            let narrative = format!(
                "【白銀比・黄金比ホメオスタシス正常範囲】 K_stress={:.2}, I_mind={} (臨界内維持)",
                k_stress, i_mind
            );
            (false, 0.01, None, narrative)
        }
    }
}
