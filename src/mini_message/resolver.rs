use crate::mini_message::parse_hex_color;
use crate::text::{NamedColor, RgbColor, TextComponent};

use crate::mini_message::parser::{Node, Parser};

#[derive(Clone, Default)]
struct ResolvedStyle {
    color: Option<Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    font: Option<String>,
    insertion: Option<String>,
    click: Option<ClickEvent>,
}

#[derive(Clone)]
enum Color {
    Named(NamedColor),
    Rgb(RgbColor),
}

#[derive(Clone)]
enum ClickEvent {
    OpenUrl(String),
    RunCommand(String),
    SuggestCommand(String),
    CopyToClipboard(String),
}

impl ResolvedStyle {
    fn apply_to(&self, component: &TextComponent) {
        match &self.color {
            Some(Color::Named(c)) => {
                component.color_named(*c);
            }
            Some(Color::Rgb(c)) => {
                component.color_rgb(*c);
            }
            None => {}
        }
        if let Some(v) = self.bold {
            component.bold(v);
        }
        if let Some(v) = self.italic {
            component.italic(v);
        }
        if let Some(v) = self.underlined {
            component.underlined(v);
        }
        if let Some(v) = self.strikethrough {
            component.strikethrough(v);
        }
        if let Some(v) = self.obfuscated {
            component.obfuscated(v);
        }
        if let Some(f) = &self.font {
            component.font(f);
        }
        if let Some(s) = &self.insertion {
            component.insertion(s);
        }
        match &self.click {
            Some(ClickEvent::OpenUrl(u)) => {
                component.click_open_url(u);
            }
            Some(ClickEvent::RunCommand(c)) => {
                component.click_run_command(c);
            }
            Some(ClickEvent::SuggestCommand(c)) => {
                component.click_suggest_command(c);
            }
            Some(ClickEvent::CopyToClipboard(t)) => {
                component.click_copy_to_clipboard(t);
            }
            None => {}
        }
    }
}

fn resolve_tag<'s>(name: &'s str, args: &[&'s str], parent: &ResolvedStyle) -> ResolvedStyle {
    let mut style = parent.clone();

    match name {
        "bold" => style.bold = Some(true),
        "italic" => style.italic = Some(true),
        "underlined" => style.underlined = Some(true),
        "strikethrough" => style.strikethrough = Some(true),
        "obfuscated" => style.obfuscated = Some(true),

        "black" => style.color = Some(Color::Named(NamedColor::Black)),
        "dark_blue" => style.color = Some(Color::Named(NamedColor::DarkBlue)),
        "dark_green" => style.color = Some(Color::Named(NamedColor::DarkGreen)),
        "dark_aqua" => style.color = Some(Color::Named(NamedColor::DarkAqua)),
        "dark_red" => style.color = Some(Color::Named(NamedColor::DarkRed)),
        "dark_purple" => style.color = Some(Color::Named(NamedColor::DarkPurple)),
        "gold" => style.color = Some(Color::Named(NamedColor::Gold)),
        "gray" => style.color = Some(Color::Named(NamedColor::Gray)),
        "dark_gray" => style.color = Some(Color::Named(NamedColor::DarkGray)),
        "blue" => style.color = Some(Color::Named(NamedColor::Blue)),
        "green" => style.color = Some(Color::Named(NamedColor::Green)),
        "aqua" => style.color = Some(Color::Named(NamedColor::Aqua)),
        "red" => style.color = Some(Color::Named(NamedColor::Red)),
        "light_purple" => style.color = Some(Color::Named(NamedColor::LightPurple)),
        "yellow" => style.color = Some(Color::Named(NamedColor::Yellow)),
        "white" => style.color = Some(Color::Named(NamedColor::White)),

        hex if hex.starts_with('#') => {
            if let Some(rgb) = parse_hex_color(hex) {
                style.color = Some(Color::Rgb(rgb));
            }
        }

        "color" | "colour" => {
            if let Some(&arg) = args.first() {
                return resolve_tag(arg, &[], parent);
            }
        }

        "font" => {
            if let Some(&f) = args.first() {
                style.font = Some(f.to_string());
            }
        }

        "insertion" => {
            if let Some(&s) = args.first() {
                style.insertion = Some(s.to_string());
            }
        }

        "click" => match args.first().copied() {
            Some("open_url") => {
                style.click = args.get(1).map(|u| ClickEvent::OpenUrl(u.to_string()))
            }
            Some("run_command") => {
                style.click = args.get(1).map(|c| ClickEvent::RunCommand(c.to_string()))
            }
            Some("suggest_command") => {
                style.click = args
                    .get(1)
                    .map(|c| ClickEvent::SuggestCommand(c.to_string()))
            }
            Some("copy_to_clipboard") => {
                style.click = args
                    .get(1)
                    .map(|t| ClickEvent::CopyToClipboard(t.to_string()))
            }
            _ => {}
        },

        "reset" => return ResolvedStyle::default(),

        _ => {}
    }

    style
}

fn resolve_into(parent: &TextComponent, nodes: &[Node<'_>], style: &ResolvedStyle) {
    for node in nodes {
        match node {
            Node::Text(s) => {
                let child = TextComponent::text(s);
                style.apply_to(&child);
                parent.add_child(child);
            }

            Node::LiteralOpen => {
                let child = TextComponent::text("<");
                style.apply_to(&child);
                parent.add_child(child);
            }

            Node::Tag {
                name,
                args,
                children,
            } => {
                let child_style = resolve_tag(name, args, style);
                let wrapper = TextComponent::text("");
                child_style.apply_to(&wrapper);
                resolve_into(&wrapper, children, &child_style);
                parent.add_child(wrapper);
            }
        }
    }
}

pub fn parse_mini_message(input: &str) -> TextComponent {
    let nodes = Parser::new(input).parse().unwrap_or_default();
    let root = TextComponent::text("");
    resolve_into(&root, &nodes, &ResolvedStyle::default());
    root
}
