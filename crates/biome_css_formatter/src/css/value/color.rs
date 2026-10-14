use crate::prelude::*;
use crate::utils::custom_property::is_raw_custom_property_component;
use biome_css_syntax::{CssColor, CssColorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColor;
impl FormatNodeRule<CssColor> for FormatCssColor {
    fn fmt_fields(&self, node: &CssColor, f: &mut CssFormatter) -> FormatResult<()> {
        let CssColorFields {
            hash_token,
            value_token,
        } = node.as_fields();

        // A raw custom-property value is kept as written.
        if is_raw_custom_property_component(node) {
            return write!(f, [hash_token.format(), value_token.format()]);
        }

        write!(
            f,
            [
                hash_token.format(),
                value_token.format()?.with_text_case(CssCase::Lowercase)
            ]
        )
    }
}
