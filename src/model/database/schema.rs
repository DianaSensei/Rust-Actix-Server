use diesel::sql_types::*;
// use super::language::Language;


table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        role -> Text,
        // language -> Language,
        created_by -> Text,
        created_time_dt -> Timestamp,
        updated_by -> Text,
        updated_time_dt -> Timestamp,
    }
}
//
// joinable!(comments -> posts (post_id));
// joinable!(comments -> users (user_id));
// joinable!(posts -> users (user_id));
//
// allow_tables_to_appear_in_same_query!(comments, posts, users,);
