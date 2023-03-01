// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (id) {
        id -> Integer,
        hash -> Text,
    }
}

diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        source -> Text,
        tag -> Text,
        nonce -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        payload -> Text,
        operation_result_status -> Nullable<Text>,
        operation_result_consumed_milligas -> Nullable<Text>,
        block_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    events,
);
