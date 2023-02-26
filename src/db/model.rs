// use diesel::{deserialize::{FromSql, self}, backend::RawValue};

// use {
//     diesel::{
//         serialize::{self, Output, ToSql},
//         sql_types::Text,
//         sqlite::Sqlite,
//     },
//     serde::{Deserialize, Serialize},
// };

// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
// struct WrapperMicheline(tezos_michelson::micheline::Micheline);

// impl ToSql<Text, Sqlite> for WrapperMicheline {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
//         let s = serde_json::to_string(&self.0)?.clone();
//         out.set_value(s);
//         Ok(serialize::IsNull::No)
//     }
// }

// impl FromSql<Text, Sqlite> for WrapperMicheline {
//     fn from_sql(bytes: RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
//         let micheline = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
//         Ok(WrapperMicheline(serde_json::from_str(&micheline)?))
//     }
// }
