// @generated automatically by Diesel CLI.

diesel::table! {
    emails (id) {
        id -> Int4,
        #[max_length = 320]
        sender -> Varchar,
        #[max_length = 320]
        recipient -> Varchar,
        subject -> Nullable<Text>,
        body -> Nullable<Text>,
        is_read -> Nullable<Bool>,
        received_at -> Nullable<Timestamp>,
    }
}
