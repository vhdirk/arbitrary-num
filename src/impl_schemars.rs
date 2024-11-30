use schemars::JsonSchema;

use crate::{traits::BitsSpec, Number};
use core::fmt::Debug;
use super::{AInt, AIntContainer};

impl<T, Bits> JsonSchema for AInt<T, Bits>
where
    Self: Number<Container = T, Bits = Bits>,
    T: AIntContainer + TryInto<f64>,
    Bits: BitsSpec,
    <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>
{
    fn schema_name() -> String {
        [if Self::SIGNED { "int" } else { "uint" }, &Bits::U8.to_string()].concat()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{NumberValidation, Schema, SchemaObject};
        let schema_object = SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Integer.into()),
            format: Some(Self::schema_name()),
            number: Some(Box::new(NumberValidation {
                // can be done with https://github.com/rust-lang/rfcs/pull/2484
                minimum: Some(Self::MIN.value().try_into().ok().unwrap()),
                maximum: Some(Self::MAX.value().try_into().ok().unwrap()),
                ..Default::default()
            })),
            ..Default::default()
        };
        Schema::Object(schema_object)
    }
}

#[cfg(test)]
mod tests {
    use crate::aliases::*;

    #[test]
    fn schemars() {
        use schemars::schema_for;
        let mut u8 = schema_for!(u8);
        let u9 = schema_for!(u9);
        assert_eq!(
            u8.schema.format.clone().unwrap().replace("8", "9"),
            u9.schema.format.clone().unwrap()
        );
        u8.schema.format = u9.schema.format.clone();
        assert_eq!(
            u8.schema
                .metadata
                .clone()
                .unwrap()
                .title
                .unwrap()
                .replace("8", "9"),
            u9.schema.metadata.clone().unwrap().title.unwrap()
        );
        u8.schema.metadata = u9.schema.metadata.clone();
        u8.schema.number = u9.schema.number.clone();
        assert_eq!(u8, u9);
    }
}
