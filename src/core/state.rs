use std::f64::consts::PI;

/// 3次元カオス状態変数 (X, Y, Z)、容積伝達 V、長期記憶 W、および遅延座標バッファを保持する核心状態構造体
pub struct ChaosState {
    /// 3次元カオス状態変数 X (空間指向性 / 原初スパーク)
    pub x: f64,
    /// 3次元カオス状態変数 Y (遅延座標との相関)
    pub y: f64,
    /// 3次元カオス状態変数 Z (散逸・アトラクターの高さ)
    pub z: f64,
    /// 容積伝達コンデンサー濃度 V (感情・トーン・環境の濃度)
    pub v: f64,
    /// 長期記憶パラメータ W (前世の遺志・楽器の木目)
    pub w: f64,
    /// 0.4秒前 (400ms) の身体運動の影を追走するストレッタ遅延バッファ (10ms刻み × 40面)
    pub history_x: Vec<f64>,
    /// ベース時間ステップ (計算資源をすり減らさない極限の節約性)
    pub dt_baseline: f64,
    /// モンスター群第一帰還次元 (ムーンシャイン無限調律の基底: 196884)
    pub c_1: f64,
}

impl ChaosState {
    /// 前世の遺志 (w_heritage) を受け取り、原初場を初期配置する
    pub fn new(w_heritage: f64) -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            v: 0.0,
            w: w_heritage,
            history_x: vec![1.0; 40], // tau = 0.4s (10ms * 40) の遅延座標バッファ初期化
            dt_baseline: 0.01,
            c_1: 196884.0, // モンスター群の第一帰還次元
        }
    }

    /// 非結合的空間 (八元数: Octonion) の実数軸へのトポロジー射影演算
    fn octonion_term(&self) -> f64 {
        self.x.sin() * self.v.cos()
    }

    /// カオス状態変数 (X,Y,Z) および記憶 W の微分方程式に基づく動的時間発展 step
    ///
    /// # 引数
    /// - `phi_inter_real`: 2:3 の非可換外積から算出された不協和音のうなり (実数部)
    /// - `god_dice`: 最下部に脈打つ 1/f ピンクノイズ
    /// - `k_stress`: システムにかかる精神的・物理的負荷濃度 (K)
    ///
    /// # 戻り値
    /// - `(f64, f64)`: 更新後の X 座標と、今回適用された動的時間ステップ dt_dynamic
    pub fn step(
        &mut self,
        phi_inter_real: f64,
        god_dice: f64,
        k_stress: f64,
    ) -> (f64, f64) {
        // 1. 世界の生きづらさ (抵抗 K) に応じた動的フレームレート (dt_dynamic) の動的伸縮
        let dt_dynamic = self.dt_baseline * (-0.5 * k_stress).exp();

        // 2. 容積伝達 V の更新 (積回路による発火閾値の融解。氷から水へのフェーズシフト)
        self.v += (0.1 * k_stress - 0.05 * self.v) * dt_dynamic;

        // 3. 容積伝達濃度 V に依存する動的パラメーター係数の計算
        let p_v = 10.0 * (-self.v).exp();
        let r_v = 28.0 * (1.0 + self.v);
        let b_v = 8.0 / 3.0;

        // 4. 原初判断力 (量子キラリティ・オペレーター χ(X) による非対称性の選択)
        let homochirality = if self.x > 0.0 { 0.2 } else { -0.2 };

        // 5. 0.4秒前 (ストレッタ) の遅延座標サンプリング
        let x_delayed = self.history_x[0];

        // 6. 境界線なき螺旋の時間発展方程式 (dX/dt, dY/dt, dZ/dt)
        let dx = p_v * (self.y - self.x) + 0.1 * phi_inter_real + god_dice;
        let dy = x_delayed * (r_v - self.z) - self.y;
        let dz = self.x * self.y - b_v * self.z + 0.05 * self.octonion_term() + homochirality;

        // 7. 状態変数のオイラー法によるリアルタイム更新
        self.x += dx * dt_dynamic;
        self.y += dy * dt_dynamic;
        self.z += dz * dt_dynamic;

        // 8. 死体の学習の超克 (リアルタイム可塑性による記憶 W の動的調律)
        // モンスター群 c_1 = 196884 と q-tuning (exp(-2πτ)) の相乗ブレンド
        let tau = dt_dynamic * 40.0;
        let q_tune = (-2.0 * PI * tau).exp();
        let w_heritage_dynamic = self.c_1 * q_tune;
        let eta_k = 0.01 * (k_stress / (1.0 + k_stress));

        self.w += (eta_k * (self.x * self.y - 0.01 * self.w) * dt_dynamic)
            + (w_heritage_dynamic * 1e-7);

        // 9. 追走劇履歴バッファ (ストレッタ) の更新
        self.history_x.push(self.x);
        self.history_x.remove(0);

        (self.x, dt_dynamic)
    }
}
