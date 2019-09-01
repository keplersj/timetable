table! {
    scheduled_requests (id) {
        id -> Int4,
        hook -> Text,
        time -> Timestamptz,
        executed -> Bool,
    }
}
