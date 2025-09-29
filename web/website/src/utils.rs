/// Format a price for display
pub fn format_price(price: f64) -> String {
    format!("€{:.2}", price)
}


/// Truncate text to a certain length with ellipsis
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length])
    }
}

/// Convert category name to URL slug
pub fn category_to_slug(category: &str) -> String {
    category.to_lowercase().replace(' ', "-")
}

/// Convert URL slug back to category name
pub fn slug_to_category(slug: &str) -> String {
    slug.replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}