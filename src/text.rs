use pumpkin_plugin_api::command::CommandSender;
use pumpkin_plugin_api::common::{NamedColor, RgbColor};
use pumpkin_plugin_api::text::TextComponent;

const PREFIX: &str = "[RookTAB] ";
const COLOR_CHAR: char = '&';
const HEX_LENGTH: usize = 6;

#[derive(Clone, Copy, Default)]
struct Style {
    named: Option<NamedColor>,
    rgb: Option<RgbColor>,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
}

impl Style {
    fn apply(self, component: &TextComponent) {
        if let Some(color) = self.rgb {
            component.color_rgb(color);
        } else if let Some(color) = self.named {
            component.color_named(color);
        }

        component.bold(self.bold);
        component.italic(self.italic);
        component.underlined(self.underlined);
        component.strikethrough(self.strikethrough);
        component.obfuscated(self.obfuscated);
    }

    fn with_color(self, color: NamedColor) -> Self {
        Self {
            named: Some(color),
            rgb: None,
            ..Self::default()
        }
    }

    fn with_rgb(self, color: RgbColor) -> Self {
        Self {
            named: None,
            rgb: Some(color),
            ..Self::default()
        }
    }
}

pub fn parse(input: &str) -> TextComponent {
    let root = TextComponent::text("");
    let mut style = Style::default();
    let mut buffer = String::new();
    let characters: Vec<char> = input.chars().collect();
    let mut index = 0;

    while index < characters.len() {
        let current = characters[index];
        if current != COLOR_CHAR || index + 1 >= characters.len() {
            buffer.push(current);
            index += 1;
            continue;
        }

        let marker = characters[index + 1];
        if marker == COLOR_CHAR {
            buffer.push(COLOR_CHAR);
            index += 2;
            continue;
        }

        if marker == '#' {
            match read_hex(&characters, index + 2) {
                Some(color) => {
                    flush(&root, &mut buffer, style);
                    style = style.with_rgb(color);
                    index += 2 + HEX_LENGTH;
                }
                None => {
                    buffer.push(current);
                    index += 1;
                }
            }
            continue;
        }

        match resolve(marker, style) {
            Some(next) => {
                flush(&root, &mut buffer, style);
                style = next;
                index += 2;
            }
            None => {
                buffer.push(current);
                index += 1;
            }
        }
    }

    flush(&root, &mut buffer, style);
    root
}

pub fn heading(title: &str) -> TextComponent {
    parse(&format!("&b&l{title}"))
}

pub fn entry(label: &str, value: &str) -> TextComponent {
    parse(&format!("&7  {label}: &f{value}"))
}

pub fn usage(command: &str, description: &str) -> TextComponent {
    let component = parse(&format!("&e  {command}"));
    component.click_suggest_command(command);
    component.add_child(parse(&format!("&7 - {description}")));
    component
}

pub fn success(message: &str) -> TextComponent {
    prefixed(message, "&a")
}

pub fn info(message: &str) -> TextComponent {
    prefixed(message, "&7")
}

pub fn failure(message: &str) -> TextComponent {
    prefixed(message, "&c")
}

pub fn send(sender: &CommandSender, component: TextComponent) {
    sender.send_message(component);
}

pub fn send_success(sender: &CommandSender, message: &str) {
    send(sender, success(message));
}

pub fn send_info(sender: &CommandSender, message: &str) {
    send(sender, info(message));
}

fn prefixed(message: &str, color: &str) -> TextComponent {
    parse(&format!("&3{PREFIX}{color}{message}"))
}

fn flush(root: &TextComponent, buffer: &mut String, style: Style) {
    if buffer.is_empty() {
        return;
    }

    let child = TextComponent::text(buffer);
    style.apply(&child);
    root.add_child(child);
    buffer.clear();
}

fn resolve(marker: char, style: Style) -> Option<Style> {
    match marker.to_ascii_lowercase() {
        'r' => Some(Style::default()),
        'k' => Some(Style {
            obfuscated: true,
            ..style
        }),
        'l' => Some(Style {
            bold: true,
            ..style
        }),
        'm' => Some(Style {
            strikethrough: true,
            ..style
        }),
        'n' => Some(Style {
            underlined: true,
            ..style
        }),
        'o' => Some(Style {
            italic: true,
            ..style
        }),
        code => named_color(code).map(|color| style.with_color(color)),
    }
}

fn named_color(code: char) -> Option<NamedColor> {
    let color = match code {
        '0' => NamedColor::Black,
        '1' => NamedColor::DarkBlue,
        '2' => NamedColor::DarkGreen,
        '3' => NamedColor::DarkAqua,
        '4' => NamedColor::DarkRed,
        '5' => NamedColor::DarkPurple,
        '6' => NamedColor::Gold,
        '7' => NamedColor::Gray,
        '8' => NamedColor::DarkGray,
        '9' => NamedColor::Blue,
        'a' => NamedColor::Green,
        'b' => NamedColor::Aqua,
        'c' => NamedColor::Red,
        'd' => NamedColor::LightPurple,
        'e' => NamedColor::Yellow,
        'f' => NamedColor::White,
        _ => return None,
    };
    Some(color)
}

fn read_hex(characters: &[char], start: usize) -> Option<RgbColor> {
    let end = start.checked_add(HEX_LENGTH)?;
    let digits: String = characters.get(start..end)?.iter().collect();
    if !digits.chars().all(|digit| digit.is_ascii_hexdigit()) {
        return None;
    }

    let value = u32::from_str_radix(&digits, 16).ok()?;
    Some(RgbColor {
        r: ((value >> 16) & 0xff) as u8,
        g: ((value >> 8) & 0xff) as u8,
        b: (value & 0xff) as u8,
    })
}
