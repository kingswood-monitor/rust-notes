use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Note {
    id: String,
    title: String,
    desc: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    text: String,
}

impl Note {
    fn from_str(raw_text: &str) -> Result<Self> {
        // id
        let re = Regex::new(r"id:\s*(?<id>.*)")?;
        let id = match re.captures(raw_text) {
            Some(caps) => String::from(&caps["id"]),
            None => return Err(anyhow::anyhow!("invalid id")),
        };

        // title
        let re = Regex::new(r"title:\s*(?<title>.*)")?;
        let title = match re.captures(raw_text) {
            Some(caps) => String::from(&caps["title"]),
            None => return Err(anyhow::anyhow!("invalid title")),
        };

        // desc
        let re = Regex::new(r"desc:\s*(?<desc>.*)")?;
        let desc = match re.captures(raw_text) {
            Some(caps) => String::from(&caps["desc"]),
            None => return Err(anyhow::anyhow!("invalid description")),
        };

        // created
        let re = Regex::new(r"created:\s*(?<created>.*)")?;
        let created = match re.captures(raw_text) {
            Some(caps) => {
                let created_string = String::from(&caps["created"]);
                time_from_string(created_string)
            }
            None => return Err(anyhow::anyhow!("invalid creation date")),
        };

        // updated
        let re = Regex::new(r"updated:\s*(?<updated>.*)")?;
        let updated = match re.captures(raw_text) {
            Some(caps) => {
                let updated_string = String::from(&caps["updated"]);
                time_from_string(updated_string)
            }
            None => return Err(anyhow::anyhow!("invalid updated date")),
        };

        // text
        let re = Regex::new(r"^---[\r\n](?<frontmatter>[\S\s]*)---(?<text>[\S\s]*)$")?;
        let text = match re.captures(raw_text) {
            Some(caps) => String::from(&caps["text"]),
            None => return Err(anyhow::anyhow!("invalid frontmatter")),
        };

        Ok(Note {
            id,
            title,
            desc,
            updated,
            created,
            text,
        })
    }
}

fn time_from_string(time_string: String) -> DateTime<Utc> {
    let timestamp = time_string.parse::<i64>().unwrap();
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp / 1000, 0).unwrap(),
        Utc,
    )
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_from_string() {
        let text = "---
id: 3q7bb40syjsd9jkijd1hy8j
title: Test Title
desc: Test Description
updated: 1689341301642
created: 1689341301642
---

This is the first line
This is the second line
";

        let expected_text = "

This is the first line
This is the second line
";
        let expected_datetime = "2023-07-14T13:28:21Z".parse::<DateTime<Utc>>().unwrap();

        let note = Note::from_str(text).expect("failed");

        assert_eq!(note.id, "3q7bb40syjsd9jkijd1hy8j");
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.desc, "Test Description");
        assert_eq!(note.created, expected_datetime);
        assert_eq!(note.updated, expected_datetime);
        assert_eq!(note.text, expected_text);

        println!("created: {}", note.created);
    }
}
