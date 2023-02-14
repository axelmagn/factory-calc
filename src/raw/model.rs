use core::fmt;

use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(tag = "NativeClass", content = "Classes")]
pub enum ClassGroup {
    #[serde(rename = "Class'/Script/FactoryGame.FGItemDescriptor'")]
    ItemDescriptors(Vec<ItemDescriptor>),
    #[serde(rename = "Class'/Script/FactoryGame.FGRecipe'")]
    Recipes(Vec<Recipe>),
    #[serde(other, deserialize_with = "ignore_contents")]
    IgnoredVariant,
}

// TODO(axelmagn): more fields
#[derive(Deserialize)]
pub struct ItemDescriptor {
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "mDisplayName")]
    pub display_name: String,
}

// TODO(axelmagn): parse ingredients and product
#[derive(Deserialize)]
pub struct Recipe {
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "mDisplayName")]
    pub display_name: String,
    #[serde(rename = "mIngredients")]
    pub ingredients_raw: String,
    #[serde(rename = "mProduct")]
    pub product_raw: String,
    #[serde(
        rename = "mManufactoringDuration",
        deserialize_with = "parse_stringed_float"
    )]
    pub manufacturing_duration: f64,
}

/// Ignore the contents of a value
fn ignore_contents<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    // Ignore any content at this part of the json structure
    deserializer
        .deserialize_ignored_any(serde::de::IgnoredAny)
        .unwrap();

    // Return unit as our 'Unknown' variant has no args
    Ok(())
}

/// For some reason the float values in the raw file are serialized as quoted strings, so we have to use a custom deserializer.
struct StringedF64Visitor;

impl<'de> de::Visitor<'de> for StringedF64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a quoted string describing a floating point number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let r = v.parse::<f64>();
        if let Ok(f) = r {
            Ok(f)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}

/// Parse a string that is expected to contain a float
fn parse_stringed_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let stringed_float = deserializer.deserialize_str(StringedF64Visitor)?;
    Ok(stringed_float)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_item_descriptor() {
        let fixture = include_str!("test_fixtures/item_descriptor.json");
        let val: ItemDescriptor = serde_json::from_str(fixture).unwrap();
        assert_eq!(val.class_name, "Desc_CircuitBoard_C");
        assert_eq!(val.display_name, "Circuit Board");
    }

    #[test]
    fn can_parse_recipe() {
        let fixture = include_str!("test_fixtures/recipe.json");
        let val: Recipe = serde_json::from_str(fixture).unwrap();
        assert_eq!(val.class_name, "Recipe_SteelBeam_C");
        assert_eq!(val.display_name, "Steel Beam");
        assert_eq!(val.manufacturing_duration, 4.);
    }

    #[test]
    fn can_parse_class_group() {
        let fixture = include_str!("test_fixtures/class_groups.json");
        let val: Vec<ClassGroup> = serde_json::from_str(fixture).unwrap();
        assert_eq!(val.len(), 3);
    }
}
