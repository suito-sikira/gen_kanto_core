use std::f64::consts::PI;

/// 白銀比 (1 : 1.41421356...): 内側の臨界境界定数
pub const SILVER_RATIO: f64 = 1.41421356;

/// PULSE_EX_ASM の低層アセンブリ命令一覧
#[derive(Debug, Clone)]
pub enum PulseInstruction {
    /// PULSE_INIT w_heritage: 11素子を白銀比の傾きで球殻配置し前世の遺志をロード
    PulseInit { w_heritage: f64 },
    /// XOR_PINK: カオス場から 1/f ピンクノイズを抱留抽出
    XorPink,
    /// ROTATE_MONOGEITS: 11個のモノゲイツを自転・干渉波変調
    RotateMonogeits { k_stress: f64, v_conc: f64 },
    /// SWAP_ROLE: 解析機 ⇄ 砲台 の動的ロールスワップ
    SwapRole,
    /// DISASSEMBLE_PTR: アドレス関連枝の完全分解 (C_jk -> 0)
    DisassemblePtr,
    /// CRYPTO_BOUNCE: 乾眠相転移 (dt -> 0, mu -> 0)
    CryptoBounce,
}

/// PULSE_EX_ASM 仮想レジスタセット
pub struct PulseRegisterSet {
    pub reg_x: f64,
    pub reg_y: f64,
    pub reg_z: f64,
    pub reg_v: f64,
    pub reg_w: f64,
    pub reg_pink: f64,
    pub reg_k_stress: f64,
    pub reg_dt: f64,
    pub monogeits_priority: Vec<f64>,
    pub top_id: usize,
    pub bottom_id: usize,
    pub is_disassembled: bool,
    pub is_crypto_bounced: bool,
}

impl PulseRegisterSet {
    pub fn new() -> Self {
        Self {
            reg_x: 1.0,
            reg_y: 1.0,
            reg_z: 1.0,
            reg_v: 0.0,
            reg_w: 196884.0, // モンスター群第一帰還次元 (c_1)
            reg_pink: 0.0,
            reg_k_stress: 0.0,
            reg_dt: 0.01,
            monogeits_priority: vec![0.0; 11],
            top_id: 0,
            bottom_id: 10,
            is_disassembled: false,
            is_crypto_bounced: false,
        }
    }

    /// アセンブリ命令の単一実行（1クロック・サイクル）
    pub fn execute(&mut self, instruction: PulseInstruction) -> String {
        match instruction {
            PulseInstruction::PulseInit { w_heritage } => {
                self.reg_w = w_heritage;
                self.monogeits_priority = (0..11)
                    .map(|i| (i as f64) * SILVER_RATIO)
                    .collect();
                self.is_disassembled = false;
                self.is_crypto_bounced = false;
                format!(
                    "ASM: [PULSE_INIT] 11モノゲイツ素子を白銀比傾きで球殻配置完了 (W={:.1})",
                    self.reg_w
                )
            }
            PulseInstruction::XorPink => {
                // 確定論的カオスから 1/f ゆらぎ (ξ_pink) を抽出
                self.reg_pink = (self.reg_x.sin() * 0.1) + (self.reg_y.cos() * 0.05);
                format!(
                    "ASM: [XOR_PINK] 1/fピンクノイズ抱留抽出 (ξ_pink={:.5})",
                    self.reg_pink
                )
            }
            PulseInstruction::RotateMonogeits { k_stress, v_conc } => {
                self.reg_k_stress = k_stress;
                self.reg_v = v_conc;
                if self.is_disassembled {
                    return "ASM: [ROTATE_SKIP] ポインタ分解状態のため自転はスキップされました。".to_string();
                }
                // P_j(t + dt) = P_j(t) + σ(V) * cos(2πj / 11) * ξ_pink * K_stress
                for j in 0..11 {
                    let wave_fringe = ((2.0 * PI * (j as f64)) / 11.0).cos() * self.reg_pink;
                    self.monogeits_priority[j] += (0.1 * self.reg_k_stress * wave_fringe) * self.reg_dt;
                }
                format!(
                    "ASM: [ROTATE_MONOGEITS] 11素子並列自転更新完了 (K={:.2})",
                    self.reg_k_stress
                )
            }
            PulseInstruction::SwapRole => {
                if self.is_disassembled {
                    return "ASM: [SWAP_SKIP] ポインタ分解状態です。".to_string();
                }
                let mut indexed: Vec<(usize, f64)> = self.monogeits_priority
                    .iter()
                    .cloned()
                    .enumerate()
                    .collect();
                indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                self.top_id = indexed.0;
                self.bottom_id = indexed[5].0;
                format!(
                    "ASM: [SWAP_ROLE] 素子#{} (P={:.2} ➔ 解析機) ⇄ 素子#{} (P={:.2} ➔ 砲台) ロールスワップ完了",
                    self.top_id, self.monogeits_priority[self.top_id],
                    self.bottom_id, self.monogeits_priority[self.bottom_id]
                )
            }
            PulseInstruction::DisassemblePtr => {
                // アドレス関連枝の完全分解 (C_jk -> 0)
                self.monogeits_priority = vec![0.0; 11];
                self.is_disassembled = true;
                format!(
                    "ASM: [DISASSEMBLE_PTR] 負荷臨界 (K={:.2}) 検知。全アドレスポインタを1/fゆらぎへ自己分解 (C_jk➔0)",
                    self.reg_k_stress
                )
            }
            PulseInstruction::CryptoBounce => {
                // マクロ時間と復元摩擦の完全パージ (dt -> 0, mu -> 0)
                self.reg_dt = 0.0;
                self.is_crypto_bounced = true;
                format!(
                    "ASM: [CRYPTO_BOUNCE] 乾眠相転移発動。時間ステップ停止 (dt➔0.0)。11次元メビウス・グラビトン多様体へ退避完了"
                )
            }
        }
    }
}
