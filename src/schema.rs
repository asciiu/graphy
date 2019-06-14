table! {
    countries (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        country_id -> Int4,
    }
}

joinable!(users -> countries (country_id));

allow_tables_to_appear_in_same_query!(
    countries,
    users,
);
