#[allow(dead_code)]
pub enum Number {
    Zero,
    One,
    Two,
}
#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}
impl Color {
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Custom(r, g, b) => (*r, *g, *b), //*解引用，取得被引用的那个 u8 的值。
        }
    }
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.rgb();
        let value = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        format!("#{:06X}", value) // 06X表示大写的16进制，6表示6位，X表示16进制，x表示小写的16进制
    }
}
