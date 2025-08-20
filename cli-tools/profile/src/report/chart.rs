// Chart visualization module
// Future: ASCII charts for sentence length distribution, tempo visualization, etc.

use crate::models::TextProfile;

pub struct ChartGenerator;

impl ChartGenerator {
    pub fn generate_tempo_chart(_profile: &TextProfile) -> String {
        // TODO: 実装
        // 文の長さの変化を視覚化
        // 例: ▁▃▇▃▁▇▃▁
        String::from("▁▃▇▃▁▇▃▁")
    }
    
    pub fn generate_distribution_chart(_profile: &TextProfile) -> String {
        // TODO: 実装
        // 文長の分布を棒グラフで表示
        String::new()
    }
}