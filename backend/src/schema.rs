// @generated automatically by Diesel CLI.

diesel::table! {
    investment_groups (id) {
        id -> Uuid,
        #[max_length = 255]
        group_name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted -> Bool,
        owner_id -> Uuid,
    }
}

diesel::table! {
    investment_types (id) {
        id -> Uuid,
        #[max_length = 255]
        type_name -> Varchar,
    }
}

diesel::table! {
    investment_users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        superuser -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    investments (id) {
        id -> Uuid,
        #[max_length = 255]
        investment_name -> Varchar,
        #[max_length = 30]
        code -> Nullable<Varchar>,
        initial_value -> Numeric,
        current_value -> Numeric,
        investment_datetime -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        closed -> Bool,
        deleted -> Bool,
        group_id -> Uuid,
        creator_id -> Uuid,
        investment_type_id -> Nullable<Uuid>,
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
