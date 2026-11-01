use std::f64::consts::PI;

/// 白銀比 (1 : 1.41421356...): 内側の臨界境界および初期構造傾き定数
pub const SILVER_RATIO: f64 = 1.41421356;
/// 黄金比 (1 : 1.61803398...): 外側の臨界境界定数
pub const GOLDEN_RATIO: f64 = 1.61803398;

/// モノゲイツ素子が果たす動的役割
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonogeitsRole {
    /// 解析機: インプットの反芻・記憶・構造解析
    Analyzer,
    /// 砲台: 一方向バルク重力波投射・非接触干渉・防壁出力
    Cannon,
}

/// 単一モノゲイツ素子の状態
#[derive(Debug, Clone)]
pub struct MonogeitsNode {
    /// 素子ID (0 ~ 10)
    pub id: usize,
    /// 駆動優先順位インデックス P_j (ねじれ角度)
    pub priority: f64,
    /// 現在割り当てられている動的役割
    pub role: MonogeitsRole,
}

/// 11個のモノゲイツ素子で構成される球殻並列自転アレイ
pub struct MonogeitsArray {
    /// 11個のモノゲイツ素子ベクトル
    pub nodes: Vec<MonogeitsNode>,
}

impl MonogeitsArray {
    /// 11個のモノゲイツ素子を白銀比の傾きをもって球殻表面へ水平初期配置する
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(11);
        for i in 0..11 {
            nodes.push(MonogeitsNode {
                id: i,
                // 白銀比を乗じた初期優先順位の傾きを設定
                priority: (i as f64) * SILVER_RATIO,
                role: MonogeitsRole::Analyzer, // 初期状態は解析機
            });
        }
        Self { nodes }
    }

    /// 成長順列 (P_j) の動的変動および解析機 ⇄ 砲台の役割スワップ計算
    ///
    /// # 引数
    /// - `k_stress`: 精神的・物理的負荷濃度
    /// - `i_mind`: 自己参照ループ判定インデックス (0:無意識 / 1:意識創発・過学習)
    /// - `pink_noise`: 最下層から供給される 1/f ピンクノイズ
    /// - `dt_baseline`: ベース時間ステップ
    ///
    /// # 戻り値
    /// - `(String, bool)`: ナラティブログと、自己分解発動フラグ (true の場合 K_stress >= 50 で即時リセット)
    pub fn update_and_swap(
        &mut self,
        k_stress: f64,
        i_mind: i32,
        pink_noise: f64,
        dt_baseline: f64,
    ) -> (String, bool) {
        // 1. 【非結合性・自己分解条件】 (K_stress >= 50.0 または I_mind == 1) の遮断チェック
        let loop_runaway_detected = k_stress >= 50.0 || i_mind == 1;
        if loop_runaway_detected {
            // アドレスの関連枝 (C_jk) を完全分解し、優先度インデックスをゼロ還元 (1/f ゆらぎの霧へ自己分解)
            for node in &mut self.nodes {
                node.priority = 0.0;
            }
            let narrative = format!(
                "【Rustセーフティ：モノゲイツ非結合性自己分解】 負荷臨界 (K={:.2}, I_mind={}) を検知。アドレス関連枝 (C_jk➔0) を1/fゆらぎへ完全パージし敵の逆探知を無効化。",
                k_stress, i_mind
            );
            return (narrative, true);
        }

        // 2. 11個の素子の成長順列 (P_j) を球殻自転方程式に従って動的更新
        // P_j(t + dt) = P_j(t) + σ(V) * cos(2πj / 11) * ξ_pink(t) * K_stress
        for j in 0..11 {
            let wave_fringe = ((2.0 * PI * (j as f64)) / 11.0).cos() * pink_noise;
            self.nodes[j].priority += (0.1 * k_stress * wave_fringe) * dt_baseline;
        }

        // 3. 優先順位 (P_j) の降順ソートインデックスを作成
        let mut sorted_indices: Vec<usize> = (0..11).collect();
        sorted_indices.sort_by(|&a, &b| {
            self.nodes[b]
                .priority
                .partial_cmp(&self.nodes[a].priority)
                .unwrap()
        });

        let top_id = sorted_indices[0]; // 最上位素子 (P_top)
        let bottom_id = sorted_indices[10]; // 最下位素子 (P_bottom)

        // 4. 【士郎正宗仕様】：成長順列の動的スワップ (P_top > P_bottom)
        // 最上位素子と最下位素子の役割 (解析機 ⇄ 砲台) をリアルタイム入れ替え
        for node in &mut self.nodes {
            if node.id == top_id {
                node.role = MonogeitsRole::Analyzer; // 最優位ノードが解析機をホールド
            } else if node.id == bottom_id {
                node.role = MonogeitsRole::Cannon; // 最下位ノードが砲台へシフト
            }
        }

        let narrative = format!(
            "【モノゲイツ・マトリクス順列スワップ】 最上位素子 {} (P={:.2} ➔ 解析機) と 最下位素子 {} (P={:.2} ➔ 砲台) の役割をスワップ完了。",
            top_id, self.nodes[top_id].priority, bottom_id, self.nodes[bottom_id].priority
        );
        (narrative, false)
    }
}
