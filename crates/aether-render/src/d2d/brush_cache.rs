use std::collections::HashMap;
use windows::core::Result;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::ID2D1HwndRenderTarget;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWriteCreateFactory, IDWriteFactory,
    DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STRETCH_NORMAL,
    DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_ALIGNMENT_TRAILING, DWRITE_TEXT_ALIGNMENT_CENTER,
    DWRITE_PARAGRAPH_ALIGNMENT_NEAR, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
};

/// 画刷缓存 - 避免每帧创建 COM 对象
pub struct BrushCache {
    brushes: HashMap<u32, ID2D1SolidColorBrush>,
}

impl BrushCache {
    pub fn new() -> Self {
        Self {
            brushes: HashMap::new(),
        }
    }

    /// 获取或创建指定颜色的画刷
    pub fn get_brush(
        &mut self,
        target: &ID2D1HwndRenderTarget,
        color: &D2D1_COLOR_F,
    ) -> Result<ID2D1SolidColorBrush> {
        let key = color_key(color);
        if let Some(brush) = self.brushes.get(&key) {
            return Ok(brush.clone());
        }
        let brush = unsafe { target.CreateSolidColorBrush(color, None)? };
        let result = brush.clone();
        self.brushes.insert(key, brush);
        Ok(result)
    }

    /// 清空缓存（设备丢失时调用）
    pub fn clear(&mut self) {
        self.brushes.clear();
    }
}

/// 文本格式缓存 - 避免每帧创建 DirectWrite 格式对象
pub struct TextFormatCache {
    dwrite_factory: IDWriteFactory,
    formats: HashMap<TextFormatKey, IDWriteTextFormat>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TextFormatKey {
    font_size: u32,  // 缩放为整数避免浮点精度问题
    font_weight: u32,
    alignment: u8,
    paragraph_alignment: u8,
}

impl TextFormatCache {
    pub fn new() -> Result<Self> {
        unsafe {
            let dwrite_factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
            Ok(Self {
                dwrite_factory,
                formats: HashMap::new(),
            })
        }
    }

    /// 获取或创建文本格式
    pub fn get_format(
        &mut self,
        font_size: f32,
        font_weight: u32,
        text_alignment: u32,
        paragraph_alignment: u32,
    ) -> Result<IDWriteTextFormat> {
        let key = TextFormatKey {
            font_size: (font_size * 10.0) as u32,
            font_weight,
            alignment: text_alignment as u8,
            paragraph_alignment: paragraph_alignment as u8,
        };

        if let Some(format) = self.formats.get(&key) {
            return Ok(format.clone());
        }

        unsafe {
            let format = self.dwrite_factory.CreateTextFormat(
                windows::core::w!("Consolas"),
                None,
                windows::Win32::Graphics::DirectWrite::DWRITE_FONT_WEIGHT(font_weight as i32),
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                font_size,
                windows::core::w!("zh-CN"),
            )?;
            let _ = format.SetTextAlignment(
                std::mem::transmute::<u32, windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_ALIGNMENT>(text_alignment)
            );
            let _ = format.SetParagraphAlignment(
                std::mem::transmute::<u32, windows::Win32::Graphics::DirectWrite::DWRITE_PARAGRAPH_ALIGNMENT>(paragraph_alignment)
            );
            let result = format.clone();
            self.formats.insert(key, format);
            Ok(result)
        }
    }

    /// 获取代码文本格式（左对齐，顶部）
    pub fn get_code_format(&mut self, font_size: f32) -> Result<IDWriteTextFormat> {
        self.get_format(
            font_size,
            DWRITE_FONT_WEIGHT_NORMAL.0 as u32,
            DWRITE_TEXT_ALIGNMENT_LEADING.0 as u32,
            DWRITE_PARAGRAPH_ALIGNMENT_NEAR.0 as u32,
        )
    }

    /// 获取行号格式（右对齐，顶部）
    pub fn get_line_number_format(&mut self, font_size: f32) -> Result<IDWriteTextFormat> {
        self.get_format(
            font_size,
            DWRITE_FONT_WEIGHT_NORMAL.0 as u32,
            DWRITE_TEXT_ALIGNMENT_TRAILING.0 as u32,
            DWRITE_PARAGRAPH_ALIGNMENT_NEAR.0 as u32,
        )
    }

    /// 获取居中格式
    pub fn get_center_format(&mut self, font_size: f32, font_weight: u32) -> Result<IDWriteTextFormat> {
        self.get_format(
            font_size,
            font_weight,
            DWRITE_TEXT_ALIGNMENT_CENTER.0 as u32,
            DWRITE_PARAGRAPH_ALIGNMENT_CENTER.0 as u32,
        )
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.formats.clear();
    }
}

/// 将颜色转换为缓存键
fn color_key(color: &D2D1_COLOR_F) -> u32 {
    let r = (color.r * 255.0) as u32;
    let g = (color.g * 255.0) as u32;
    let b = (color.b * 255.0) as u32;
    let a = (color.a * 255.0) as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}
