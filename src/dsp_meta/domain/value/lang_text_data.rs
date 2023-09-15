use std::collections::HashMap;

use crate::domain::value::iso_code::IsoCode;
use crate::errors::DspMetaError;

#[derive(Debug, PartialEq)]
pub struct LangTextData(pub HashMap<IsoCode, String>);

/// Try to create the text description of the discipline
/// FIXME: Move to the API layer where the service adapter is implemented
impl TryFrom<Vec<&hcl::Attribute>> for LangTextData {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut text_data: HashMap<IsoCode, String> = HashMap::new();

        for attribute in attributes {
            let iso_code = IsoCode::try_from(attribute.key())?;
            let text = match attribute.expr() {
                hcl::Expression::String(value) => Ok(value.to_owned()),
                _ => Err(DspMetaError::CreateValueObject(
                    "The passed discipline block description attribute is not of String type."
                        .to_string(),
                )),
            }?;

            text_data.insert(iso_code, text);
        }

        Ok(LangTextData(text_data))
    }
}
