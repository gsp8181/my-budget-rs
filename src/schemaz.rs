// @generated automatically by Diesel CLI.

diesel::table! {
    item (id) {
        id -> Nullable<Integer>,
        oldId -> Nullable<Integer>,
        category -> Text,
        name -> Text,
        day -> Nullable<Integer>,
        amount -> Text,
        cardid -> Nullable<Integer>,
        dbName -> Nullable<Integer>,
    }
}

diesel::table! {
    settings (id) {
        id -> Nullable<Integer>,
        name -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    item,
    settings,
);
