// @generated automatically by Diesel CLI.

diesel::table! {
    investment_groups (id) {
        id -> Int4,
        group_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
        owner_id -> Int4,
    }
}

diesel::table! {
    investment_types (id) {
        id -> Int4,
        type_name -> Varchar,
    }
}

diesel::table! {
    investment_users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    investments (id) {
        id -> Int4,
        investment_name -> Varchar,
        code -> Nullable<Varchar>,
        initial_value -> Numeric,
        current_value -> Numeric,
        investment_datetime -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        closed -> Bool,
        deleted -> Bool,
        group_id -> Int4,
        creator_id -> Int4,
        investment_type_id -> Nullable<Int4>,
    }
}

diesel::joinable!(investment_groups -> investment_users (owner_id));
diesel::joinable!(investments -> investment_groups (group_id));
diesel::joinable!(investments -> investment_types (investment_type_id));
diesel::joinable!(investments -> investment_users (creator_id));

diesel::allow_tables_to_appear_in_same_query!(
    investment_groups,
    investment_types,
    investment_users,
    investments,
);