use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};

pub struct MdRender<'a> {
    all_lines: Vec<Line<'a>>,
    current_line: Vec<Span<'a>>,
    current_style: Style,
}

impl<'a> MdRender<'a> {
    pub fn new() -> Self {
        Self {
            all_lines: Vec::new(),
            current_line: Vec::new(),
            current_style: Style::default(),
        }
    }

    fn commit_line(&mut self) {
        let mut line = Vec::new();
        std::mem::swap(&mut self.current_line, &mut line);
        self.all_lines.push(Line::from(line));
    }

    pub fn render(mut self, markdown: &'a str) -> Text {
        // Track the current style (bold, italic, etc.)
        self.current_style = Style::default();

        let parser = Parser::new(markdown);

        for event in parser {
            match event {
                // Start of a Markdown tag (e.g., **bold**, *italic*, etc.)
                Event::Start(tag) => match tag {
                    Tag::Paragraph => {}
                    Tag::Item => {}
                    Tag::List(_) => {
                        self.commit_line();
                    }
                    Tag::Emphasis => {
                        self.current_style = self.current_style.add_modifier(Modifier::ITALIC);
                    }
                    Tag::Strong => {
                        self.current_style = self.current_style.add_modifier(Modifier::BOLD);
                    }
                    Tag::CodeBlock(_lang) => {
                        // You could switch to a "code block style" here if desired
                    }
                    Tag::Heading { .. } => {
                        self.commit_line(); // an extra empty line
                                            // Example: make headings bold + underlined
                        self.current_style = self
                            .current_style
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::UNDERLINED);
                    }
                    _ => {}
                },

                // End of a Markdown tag
                Event::End(tag) => match tag {
                    TagEnd::Paragraph => {
                        self.commit_line();
                    }
                    TagEnd::Item => {
                        self.commit_line();
                    }
                    TagEnd::List(_) => {
                        self.commit_line();
                    }
                    TagEnd::Emphasis => {
                        self.current_style = self.current_style.remove_modifier(Modifier::ITALIC);
                    }
                    TagEnd::Strong => {
                        self.current_style = self.current_style.remove_modifier(Modifier::BOLD);
                    }
                    TagEnd::CodeBlock => {
                        // End code block styling if you started it above
                    }
                    TagEnd::Heading(_level) => {
                        self.current_style = self
                            .current_style
                            .remove_modifier(Modifier::BOLD)
                            .remove_modifier(Modifier::UNDERLINED);
                        self.commit_line();
                    }
                    _ => {}
                },

                // Actual text to display
                Event::Text(text_content) => {
                    // Add a new span with the current style
                    self.current_line
                        .push(Span::styled(text_content, self.current_style));
                }

                // Inline code (e.g. `foo`)
                Event::Code(code_content) => {
                    // Make inline code slightly distinct, e.g., dim or italic
                    let code_style = self.current_style.add_modifier(Modifier::DIM);
                    self.current_line
                        .push(Span::styled(code_content, code_style));
                }

                // A hard line break: push the current line into all_lines, reset current_line
                Event::HardBreak => {
                    self.commit_line();
                }

                // A soft line break (usually just newline in Markdown)
                Event::SoftBreak => {
                    // You could treat soft breaks as new lines or as spaces
                    // Here, let's treat them as new lines:
                    self.commit_line();
                }

                // We ignore other events (html, footnotes, etc.) in this minimal example
                _ => {}
            }
        }

        self.commit_line();

        Text::from(self.all_lines)
    }
}
