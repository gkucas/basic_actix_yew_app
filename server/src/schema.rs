// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}
