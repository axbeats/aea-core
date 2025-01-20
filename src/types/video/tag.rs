// Import all items from the current crate.
use crate::*;

/// Represents a tag that can be associated with various entities on the platform.
///
/// A tag consists of a type (e.g., hashtag, account) and its content (e.g., "nature" for a hashtag).
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub tag_type: TagType, // Type of the tag, defined by the TagType enum.
    pub content: String,   // The actual tag content, e.g., "nature" for a hashtag or "user123" for an account.
}

impl Tag {
    /// Creates a new tag based on its type and content.
    pub fn new(tag_type: TagType, content: String) -> Self {
        Tag { tag_type, content }
    }

    /// Converts the tag to its string representation, e.g., "#nature" or "@user123".
    pub fn to_string(&self) -> String {
        format!("{}{}", self.tag_type.prefix(), self.content)
    }

    /// Parses a string to produce a `Tag`.
    /// This function assumes that the input string has a valid prefix.
    pub fn from_string(tag_str: &str) -> Self {
        let prefix = tag_str.chars().next().unwrap(); // Get the first character of the string.
        let content = tag_str[1..].to_string();       // Extract the content after the prefix.
        
        // Determine the tag type based on the prefix.
        let tag_type = match prefix {
            '#' => TagType::Hashtag,
            '@' => TagType::Account,
            '%' => TagType::Video,
            '&' => TagType::Audio,
            '^' => TagType::Proposal,
            '~' => TagType::Choice,
            '+' => TagType::Calibration,
            '!' => TagType::Rule,
            '=' => TagType::Review,
            '*' => TagType::Product,
            '$' => TagType::Token,
            _ => panic!("Invalid tag prefix!"), // Panic if the prefix is invalid (could be handled differently).
        };

        Tag::new(tag_type, content) // Create a new Tag with the determined type and content.
    }
}

/// Enum representing the available types of tags and their corresponding prefixes.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum TagType {
    Hashtag,   // #
    Account,   // @
    Video,     // %
    Audio,     // &
    Proposal,  // ^
    Choice,    // ~
    Calibration, // +
    Rule,       // !
    Review,    // =
    Product,   // *
    Token,     // $
}

impl TagType {
    /// Returns the prefix character associated with the tag type.
    pub fn prefix(&self) -> char {
        match self {
            TagType::Hashtag => '#',
            TagType::Account => '@',
            TagType::Video => '%',
            TagType::Audio => '&',
            TagType::Proposal => '^',
            TagType::Choice => '~',
            TagType::Calibration => '+',
            TagType::Rule => '!',
            TagType::Review => '=',
            TagType::Product => '*',
            TagType::Token => '$',
        }
    }
}
