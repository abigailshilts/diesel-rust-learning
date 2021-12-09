table! {
    activities (id) {
        id -> Int4,
        topic -> Varchar,
        dy -> Varchar,
        start_at -> Varchar,
        end_at -> Varchar,
    }
}

table! {
    events (id) {
        id -> Int4,
        topic -> Varchar,
        dy -> Nullable<Date>,
        start_at -> Time,
        end_at -> Time,
    }
}

allow_tables_to_appear_in_same_query!(
    activities,
    events,
);
