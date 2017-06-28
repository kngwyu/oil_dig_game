// 定数

// デバッグ
pub const DEBUG: bool = true;

// 窓関連
pub const WINDOW_SIZE: u32 = 600;
pub const WINDOW_TITLE: &'static str = "Oil Dig Game";
pub const ORANGE: [f32; 4] = [0.95, 0.31, 0.18, 1.0];
pub const GREEN: [f32; 4] = [0.35, 0.9, 0.21, 1.0];
pub const CYAN: [f32; 4] = [0.7, 0.21, 0.9, 1.0];
pub const BLUE: [f32; 4] = [0.21, 0.81, 0.9, 1.0];
pub const USER_COLOR: [[f32; 4]; 4] = [ORANGE, GREEN, CYAN, BLUE];

// ゲーム内定数
pub const FIELD_SIZE_MIN: usize = 20;
pub const FIELD_SIZE_MAX: usize = 30;
pub const MIN_OIL: i32 = 10;
pub const MAX_OIL: i32 = 100;
pub const MIN_BOM: i32 = 1;
pub const MAX_BOM: i32 = 3;
