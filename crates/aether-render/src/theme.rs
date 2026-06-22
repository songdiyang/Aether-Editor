use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;

use aether_core::lexer::TokenKind;

use crate::d2d::factory::{colors, color_f};

/// 主题系统
pub struct Theme {
    pub editor_bg: D2D1_COLOR_F,
    pub line_highlight_bg: D2D1_COLOR_F,
    pub line_number_fg: D2D1_COLOR_F,
    pub line_number_bg: D2D1_COLOR_F,
    pub selection_bg: D2D1_COLOR_F,
    pub cursor_color: D2D1_COLOR_F,
    pub sidebar_bg: D2D1_COLOR_F,
    pub statusbar_bg: D2D1_COLOR_F,
    pub tab_active_bg: D2D1_COLOR_F,
    pub tab_inactive_bg: D2D1_COLOR_F,
    pub text_default: D2D1_COLOR_F,
    pub syntax: SyntaxColors,
}

pub struct SyntaxColors {
    pub keyword: D2D1_COLOR_F,
    pub string: D2D1_COLOR_F,
    pub number: D2D1_COLOR_F,
    pub comment: D2D1_COLOR_F,
    pub function: D2D1_COLOR_F,
    pub type_name: D2D1_COLOR_F,
    pub operator: D2D1_COLOR_F,
    pub variable: D2D1_COLOR_F,
    pub preprocessor: D2D1_COLOR_F,
    pub attribute: D2D1_COLOR_F,
    pub macro_color: D2D1_COLOR_F,
    pub lifetime: D2D1_COLOR_F,
    pub regex: D2D1_COLOR_F,
    pub format_string: D2D1_COLOR_F,
    pub md_heading: D2D1_COLOR_F,
    pub md_link: D2D1_COLOR_F,
    pub md_code: D2D1_COLOR_F,
    pub md_emphasis: D2D1_COLOR_F,
    pub json_key: D2D1_COLOR_F,
    pub toml_table: D2D1_COLOR_F,
    pub find_highlight: D2D1_COLOR_F,
    // Semantic token colors (P2)
    pub semantic_namespace: D2D1_COLOR_F,
    pub semantic_type: D2D1_COLOR_F,
    pub semantic_class: D2D1_COLOR_F,
    pub semantic_enum: D2D1_COLOR_F,
    pub semantic_interface: D2D1_COLOR_F,
    pub semantic_struct: D2D1_COLOR_F,
    pub semantic_type_parameter: D2D1_COLOR_F,
    pub semantic_parameter: D2D1_COLOR_F,
    pub semantic_variable_local: D2D1_COLOR_F,
    pub semantic_variable_global: D2D1_COLOR_F,
    pub semantic_property: D2D1_COLOR_F,
    pub semantic_enum_member: D2D1_COLOR_F,
    pub semantic_event: D2D1_COLOR_F,
    pub semantic_function_declaration: D2D1_COLOR_F,
    pub semantic_function_call: D2D1_COLOR_F,
    pub semantic_method: D2D1_COLOR_F,
    pub semantic_macro: D2D1_COLOR_F,
    pub semantic_keyword_control: D2D1_COLOR_F,
    pub semantic_modifier: D2D1_COLOR_F,
    pub semantic_comment_doc: D2D1_COLOR_F,
    pub semantic_string_format: D2D1_COLOR_F,
    pub semantic_number_hex: D2D1_COLOR_F,
    pub semantic_regexp: D2D1_COLOR_F,
    pub semantic_operator_logical: D2D1_COLOR_F,
    pub semantic_readonly: D2D1_COLOR_F,
    pub semantic_deprecated: D2D1_COLOR_F,
    pub semantic_async: D2D1_COLOR_F,
    pub semantic_static: D2D1_COLOR_F,
    pub semantic_abstract: D2D1_COLOR_F,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            editor_bg: colors::editor_bg(),
            line_highlight_bg: colors::line_highlight(),
            line_number_fg: colors::line_number_fg(),
            line_number_bg: colors::line_number_bg(),
            selection_bg: colors::selection_bg(),
            cursor_color: colors::cursor(),
            sidebar_bg: colors::sidebar_bg(),
            statusbar_bg: colors::statusbar_bg(),
            tab_active_bg: colors::tab_active(),
            tab_inactive_bg: colors::tab_inactive(),
            text_default: colors::text_default(),
            syntax: SyntaxColors {
                keyword: colors::keyword(),
                string: colors::string(),
                number: colors::number(),
                comment: colors::comment(),
                function: colors::function(),
                type_name: colors::type_name(),
                operator: colors::operator(),
                variable: colors::variable(),
                preprocessor: colors::preprocessor(),
                attribute: color_f(0.8, 0.6, 0.3, 1.0),
                macro_color: color_f(0.6, 0.4, 0.8, 1.0),
                lifetime: color_f(0.5, 0.7, 0.9, 1.0),
                regex: color_f(0.8, 0.5, 0.3, 1.0),
                format_string: color_f(0.8, 0.6, 0.4, 1.0),
                md_heading: color_f(0.3, 0.6, 0.9, 1.0),
                md_link: color_f(0.3, 0.5, 0.9, 1.0),
                md_code: color_f(0.7, 0.5, 0.3, 1.0),
                md_emphasis: color_f(0.9, 0.7, 0.4, 1.0),
                json_key: color_f(0.6, 0.8, 0.9, 1.0),
                toml_table: color_f(0.8, 0.5, 0.3, 1.0),
                find_highlight: color_f(0.8, 0.7, 0.3, 0.6),
                // Semantic token colors (P2) - 默认映射
                semantic_namespace: color_f(0.5, 0.7, 0.9, 1.0),
                semantic_type: color_f(0.3, 0.7, 0.9, 1.0),
                semantic_class: color_f(0.3, 0.6, 0.9, 1.0),
                semantic_enum: color_f(0.3, 0.6, 0.9, 1.0),
                semantic_interface: color_f(0.3, 0.7, 0.8, 1.0),
                semantic_struct: color_f(0.3, 0.6, 0.9, 1.0),
                semantic_type_parameter: color_f(0.4, 0.7, 0.8, 1.0),
                semantic_parameter: color_f(0.7, 0.7, 0.7, 1.0),
                semantic_variable_local: color_f(0.8, 0.8, 0.8, 1.0),
                semantic_variable_global: color_f(0.7, 0.7, 0.8, 1.0),
                semantic_property: color_f(0.7, 0.7, 0.8, 1.0),
                semantic_enum_member: color_f(0.5, 0.7, 0.9, 1.0),
                semantic_event: color_f(0.7, 0.5, 0.7, 1.0),
                semantic_function_declaration: color_f(0.8, 0.6, 0.3, 1.0),
                semantic_function_call: color_f(0.8, 0.6, 0.3, 1.0),
                semantic_method: color_f(0.8, 0.6, 0.3, 1.0),
                semantic_macro: color_f(0.6, 0.4, 0.8, 1.0),
                semantic_keyword_control: color_f(0.5, 0.5, 0.8, 1.0),
                semantic_modifier: color_f(0.5, 0.5, 0.8, 1.0),
                semantic_comment_doc: color_f(0.4, 0.6, 0.4, 1.0),
                semantic_string_format: color_f(0.8, 0.6, 0.4, 1.0),
                semantic_number_hex: color_f(0.6, 0.8, 0.6, 1.0),
                semantic_regexp: color_f(0.8, 0.5, 0.3, 1.0),
                semantic_operator_logical: color_f(0.5, 0.5, 0.8, 1.0),
                semantic_readonly: color_f(0.5, 0.7, 0.9, 1.0),
                semantic_deprecated: color_f(0.5, 0.5, 0.5, 0.7),
                semantic_async: color_f(0.5, 0.5, 0.8, 1.0),
                semantic_static: color_f(0.7, 0.7, 0.7, 1.0),
                semantic_abstract: color_f(0.5, 0.7, 0.8, 1.0),
            },
        }
    }

    /// 根据语义令牌类型索引获取颜色（用于避免循环依赖）
    pub fn color_for_semantic_token_index(&self, type_index: u32, _modifier_bits: u32) -> D2D1_COLOR_F {
        match type_index {
            0 => self.syntax.semantic_namespace,   // namespace
            1 => self.syntax.semantic_type,         // type
            2 => self.syntax.semantic_class,        // class
            3 => self.syntax.semantic_enum,         // enum
            4 => self.syntax.semantic_interface,    // interface
            5 => self.syntax.semantic_struct,       // struct
            6 => self.syntax.semantic_type_parameter, // typeParameter
            7 => self.syntax.semantic_parameter,    // parameter
            8 => self.syntax.semantic_variable_local, // variable
            9 => self.syntax.semantic_property,      // property
            10 => self.syntax.semantic_enum_member, // enumMember
            11 => self.syntax.semantic_event,       // event
            12 => self.syntax.semantic_function_declaration, // function
            13 => self.syntax.semantic_method,      // method
            14 => self.syntax.semantic_macro,       // macro
            15 => self.syntax.semantic_keyword_control, // keyword
            16 => self.syntax.semantic_modifier,    // modifier
            17 => self.syntax.semantic_comment_doc, // comment
            18 => self.syntax.semantic_string_format, // string
            19 => self.syntax.semantic_number_hex,   // number
            20 => self.syntax.semantic_regexp,      // regexp
            21 => self.syntax.semantic_operator_logical, // operator
            _ => self.text_default,
        }
    }

    /// 根据通用 token 类型获取颜色
    pub fn color_for_token(&self, kind: TokenKind) -> D2D1_COLOR_F {
        match kind {
            TokenKind::Keyword => self.syntax.keyword,
            TokenKind::Identifier => self.syntax.variable,
            TokenKind::StringLiteral => self.syntax.string,
            TokenKind::CharLiteral => self.syntax.string,
            TokenKind::NumberLiteral => self.syntax.number,
            TokenKind::LineComment | TokenKind::BlockComment | TokenKind::DocComment => self.syntax.comment,
            TokenKind::Operator => self.syntax.operator,
            TokenKind::Punctuation => self.syntax.operator,
            TokenKind::Preprocessor => self.syntax.preprocessor,
            TokenKind::Attribute => self.syntax.attribute,
            TokenKind::TypeName => self.syntax.type_name,
            TokenKind::Function => self.syntax.function,
            TokenKind::Macro => self.syntax.macro_color,
            TokenKind::Lifetime => self.syntax.lifetime,
            TokenKind::Generic => self.syntax.type_name,
            TokenKind::RegexLiteral => self.syntax.regex,
            TokenKind::FormatString => self.syntax.format_string,
            TokenKind::MdHeading => self.syntax.md_heading,
            TokenKind::MdLink => self.syntax.md_link,
            TokenKind::MdCode => self.syntax.md_code,
            TokenKind::MdEmphasis => self.syntax.md_emphasis,
            TokenKind::JsonKey => self.syntax.json_key,
            TokenKind::TomlTable => self.syntax.toml_table,
            TokenKind::Whitespace | TokenKind::Newline | TokenKind::Unknown | TokenKind::EOF => self.text_default,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
