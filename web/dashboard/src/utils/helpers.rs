use chrono::{DateTime, Utc};

/// Format a date to a human-readable string
pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%B %d, %Y").to_string()
}

/// Format a datetime to a human-readable string with time
pub fn format_datetime(date: DateTime<Utc>) -> String {
    date.format("%B %d, %Y at %H:%M").to_string()
}

/// Generate a random color for avatars
pub fn get_avatar_color(username: &str) -> &'static str {
    let colors = [
        "bg-red-500",
        "bg-orange-500",
        "bg-yellow-500",
        "bg-green-500",
        "bg-teal-500",
        "bg-blue-500",
        "bg-indigo-500",
        "bg-purple-500",
        "bg-pink-500",
    ];

    let hash: usize = username.chars().map(|c| c as usize).sum();
    colors[hash % colors.len()]
}
