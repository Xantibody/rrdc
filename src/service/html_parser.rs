use crate::domain::ReleaseDate;
use crate::error::{AppError, Result};
use chrono::NaiveDate;
use regex::Regex;
use scraper::{Html, Selector};

pub struct HtmlParser {
    date_pattern: Regex,
    json_pattern: Regex,
}

impl HtmlParser {
    pub fn new() -> Self {
        Self {
            // `2024年12月25日(水)` -> `2024年12月25日`
            date_pattern: Regex::new(r"\([^)]+\)$").unwrap(),
            // `p[0]={...};` pattern
            json_pattern: Regex::new(r"p\[\d+\]=(\{[^}]+\});").unwrap(),
        }
    }

    pub fn parse(&self, html: &str) -> Result<Vec<ReleaseDate>> {
        tracing::info!("Parsing HTML for release dates");

        let document = Html::parse_document(html);
        let mut results = Vec::new();

        // Parse static elements
        results.extend(self.parse_static_elements(&document)?);

        // Parse script data
        results.extend(self.parse_script_data(&document)?);

        tracing::info!("Found {} release dates", results.len());
        Ok(results)
    }

    fn parse_static_elements(&self, document: &Html) -> Result<Vec<ReleaseDate>> {
        let info_selector = Selector::parse(".information").map_err(|e| AppError::HtmlParse {
            message: format!("Invalid selector: {e}"),
        })?;
        let date_selector = Selector::parse(".date").map_err(|e| AppError::HtmlParse {
            message: format!("Invalid selector: {e}"),
        })?;
        let subject_selector = Selector::parse(".subject").map_err(|e| AppError::HtmlParse {
            message: format!("Invalid selector: {e}"),
        })?;
        let theme_selector = Selector::parse(".theme").map_err(|e| AppError::HtmlParse {
            message: format!("Invalid selector: {e}"),
        })?;

        let mut results = Vec::new();

        for elem in document.select(&info_selector) {
            // Extract date
            let date = elem
                .select(&date_selector)
                .next()
                .and_then(|e| e.text().next())
                .and_then(|text| self.parse_japanese_date(text));

            // Extract title
            let subject = elem
                .select(&subject_selector)
                .next()
                .and_then(|e| e.text().next())
                .map(|s| s.trim().to_string());

            let theme = elem
                .select(&theme_selector)
                .next()
                .and_then(|e| e.text().next())
                .map(|s| s.trim().to_string());

            let title = match (subject, theme) {
                (Some(s), Some(t)) => format!("{} {}", s, t),
                (Some(s), None) => s,
                _ => continue,
            };

            results.push(ReleaseDate::new(title, date));
        }

        Ok(results)
    }

    fn parse_script_data(&self, document: &Html) -> Result<Vec<ReleaseDate>> {
        let script_selector = Selector::parse("script").map_err(|e| AppError::HtmlParse {
            message: format!("Invalid selector: {e}"),
        })?;

        let mut results = Vec::new();

        for script in document.select(&script_selector) {
            let script_content: String = script.text().collect();

            if !script_content.contains("var p =") {
                continue;
            }

            for cap in self.json_pattern.captures_iter(&script_content) {
                if let Some(json_match) = cap.get(1) {
                    let json_str = json_match.as_str().replace('\'', "\"");

                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        let title = data.get("title").and_then(|v| v.as_str());
                        let date_str = data.get("release-date").and_then(|v| v.as_str());

                        if let Some(title) = title {
                            let date = date_str.and_then(|s| self.parse_japanese_date(s));
                            results.push(ReleaseDate::new(title, date));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn parse_japanese_date(&self, date_str: &str) -> Option<NaiveDate> {
        let cleaned = self.date_pattern.replace(date_str.trim(), "");
        NaiveDate::parse_from_str(&cleaned, "%Y年%m月%d日").ok()
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_japanese_date() {
        let parser = HtmlParser::new();

        let date = parser.parse_japanese_date("2024年12月25日(水)");
        assert_eq!(date, Some(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()));

        let date = parser.parse_japanese_date("2024年1月1日(月)");
        assert_eq!(date, Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));

        let date = parser.parse_japanese_date("未定");
        assert_eq!(date, None);
    }

    #[test]
    fn test_parse_static_elements() {
        let html = r#"
            <html>
            <body>
                <div class="information">
                    <span class="date">2024年12月25日(水)</span>
                    <span class="subject">Test Product</span>
                    <span class="theme">Theme Name</span>
                </div>
            </body>
            </html>
        "#;

        let parser = HtmlParser::new();
        let results = parser.parse(html).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Test Product Theme Name");
        assert_eq!(
            results[0].date,
            Some(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap())
        );
    }

    #[test]
    fn test_parse_without_theme() {
        let html = r#"
            <html>
            <body>
                <div class="information">
                    <span class="date">2024年12月25日(水)</span>
                    <span class="subject">Test Product Only</span>
                </div>
            </body>
            </html>
        "#;

        let parser = HtmlParser::new();
        let results = parser.parse(html).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Test Product Only");
    }

    #[test]
    fn test_parse_undetermined_date() {
        let html = r#"
            <html>
            <body>
                <div class="information">
                    <span class="date">未定</span>
                    <span class="subject">Future Product</span>
                </div>
            </body>
            </html>
        "#;

        let parser = HtmlParser::new();
        let results = parser.parse(html).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Future Product");
        assert!(results[0].is_undetermined());
    }
}
