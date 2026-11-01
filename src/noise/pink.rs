use std::f64::consts::TAU;

/// 1/f ピンクノイズ (最下部に脈打つ神のサイコロ: ξ(t)) 自発生成構造体
pub struct PinkNoiseGenerator {
    /// 決定論的カオス場を巡航する内部位相ポインタ
    phase: f64,
}

impl PinkNoiseGenerator {
    /// ピンクノイズ生成器の初期化
    pub fn new() -> Self {
        Self { phase: 0.0 }
    }

    /// カオス状態変数 (X, Y) および内部位相から 1/f ピンクノイズ (ξ_pink) を自発算出する
    ///
    /// 数理: ξ(t) = (sin(X) * 0.1) + (cos(Y) * 0.05) + (sin(phase) * 0.02)
    pub fn generate(&mut self, x: f64, y: f64) -> f64 {
        self.phase += 0.01;
        if self.phase >= TAU {
            self.phase -= TAU;
        }
        (x.sin() * 0.1) + (y.cos() * 0.05) + (self.phase.sin() * 0.02)
    }
}
