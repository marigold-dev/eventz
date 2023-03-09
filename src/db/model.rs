// use diesel::{deserialize::{FromSql, self}, backend::RawValue};

// use {
//     diesel::{
//         serialize::{self, Output, ToSql},
//         sql_types::String,
//         sqlite::Sqlite,
//     },
//     serde::{Deserialize, Serialize},
// };

use {
    diesel::Queryable,
    serde::{Deserialize, Serialize},
    tezos_michelson::micheline::Micheline,
};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = blocks)]
pub struct BlockModel {
    pub id: i32,
    pub hash: String,
    pub timestamp: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(BlockModel))]
#[diesel(table_name = events)]
pub struct EventModel {
    // pub id: u64,
    pub source: String,
    pub type_: Micheline,
    pub tag: String,
    pub nonce: u16,
    pub payload: Micheline,
    pub operation_result_status: Option<String>,
    pub operation_result_consumed_milligas: Option<String>,
    pub block_id: i32,
}

impl EventModel {
    pub fn new(
        // id: u64,
        source: String,
        type_: Micheline,
        tag: String,
        nonce: u16,
        payload: Micheline,
        operation_result_status: Option<String>,
        operation_result_consumed_milligas: Option<String>,
        block_id: i32,
    ) -> Self {
        Self {
            // id,
            source,
            type_,
            tag,
            nonce,
            payload,
            operation_result_status,
            operation_result_consumed_milligas,
            block_id,
        }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
// struct WrapperMicheline(tezos_michelson::micheline::Micheline);

// impl ToSql<String, Sqlite> for WrapperMicheline {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) : serialize::Result {
//         let s = serde_json::to_string(&self.0)?.clone();
//         out.set_value(s);
//         Ok(serialize::IsNull::No)
//     }
// }

// impl FromSql<String, Sqlite> for WrapperMicheline {
//     fn from_sql(bytes: RawValue<'_, Sqlite>) : deserialize::Result<Self> {
//         let micheline = <String as FromSql<String, Sqlite>>::from_sql(bytes)?;
//         Ok(WrapperMicheline(serde_json::from_str(&micheline)?))
//     }
// }
