use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};

/// Convert a Markdown string into Ratatui's Text.
pub fn markdown_to_text(markdown: &str) -> Text {
    let parser = Parser::new(markdown);

    // We'll accumulate lines (Line) into a Text object.
    let mut all_lines: Vec<Line> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();

    // Track the current style (bold, italic, etc.)
    let mut current_style = Style::default();

    for event in parser {
        match event {
            // Start of a Markdown tag (e.g., **bold**, *italic*, etc.)
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Item => {}
                Tag::List(_) => {
                    all_lines.push(Line::from(current_line));
                    current_line = Vec::new();
                }
                Tag::Emphasis => {
                    current_style = current_style.add_modifier(Modifier::ITALIC);
                }
                Tag::Strong => {
                    current_style = current_style.add_modifier(Modifier::BOLD);
                }
                Tag::CodeBlock(_lang) => {
                    // You could switch to a "code block style" here if desired
                }
                Tag::Heading { .. } => {
                    all_lines.push(Line::from(""));
                    current_line = Vec::new();
                    // Example: make headings bold + underlined
                    current_style = current_style
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED);
                }
                _ => {}
            },

            // End of a Markdown tag
            Event::End(tag) => match tag {
                TagEnd::Paragraph => {
                    all_lines.push(Line::from(current_line));
                    current_line = Vec::new();
                }
                TagEnd::Item => {
                    all_lines.push(Line::from(current_line));
                    current_line = Vec::new();
                }
                TagEnd::List(_) => {
                    all_lines.push(Line::from(current_line));
                    current_line = Vec::new();
                }
                TagEnd::Emphasis => {
                    current_style = current_style.remove_modifier(Modifier::ITALIC);
                }
                TagEnd::Strong => {
                    current_style = current_style.remove_modifier(Modifier::BOLD);
                }
                TagEnd::CodeBlock => {
                    // End code block styling if you started it above
                }
                TagEnd::Heading(_level) => {
                    all_lines.push(Line::from(current_line));
                    current_line = Vec::new();

                    current_style = current_style
                        .remove_modifier(Modifier::BOLD)
                        .remove_modifier(Modifier::UNDERLINED);
                }
                _ => {}
            },

            // Actual text to display
            Event::Text(text_content) => {
                // Add a new span with the current style
                current_line.push(Span::styled(text_content, current_style));
            }

            // Inline code (e.g. `foo`)
            Event::Code(code_content) => {
                // Make inline code slightly distinct, e.g., dim or italic
                let code_style = current_style.add_modifier(Modifier::DIM);
                current_line.push(Span::styled(code_content, code_style));
            }

            // A hard line break: push the current line into all_lines, reset current_line
            Event::HardBreak => {
                all_lines.push(Line::from(current_line));
                current_line = Vec::new();
            }

            // A soft line break (usually just newline in Markdown)
            Event::SoftBreak => {
                // You could treat soft breaks as new lines or as spaces
                // Here, let's treat them as new lines:
                all_lines.push(Line::from(current_line));
                current_line = Vec::new();
            }

            // We ignore other events (html, footnotes, etc.) in this minimal example
            _ => {}
        }
    }

    // Push the last line if it's not empty
    if !current_line.is_empty() {
        all_lines.push(Line::from(current_line));
    }

    Text::from(all_lines)
}
