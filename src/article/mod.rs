use chrono::NaiveDateTime;

struct Article {
    id: String,
    title: String,
    headline: String,
    content: String,
    author_id: String,
    slug: String,
    added_at: NaiveDateTime,
}
