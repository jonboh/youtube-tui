use crate::global::{functions::*, structs::*};

#[cfg(feature = "clipboard")]
use crate::config::*;

use crossterm::event::{KeyCode, KeyEvent};
use tui_additions::framework::FrameworkClean;

/// handles key input when the user is entering commands
pub fn command_capture(framework: &mut FrameworkClean, key: KeyEvent) -> bool {
    #[cfg(feature = "clipboard")]
    match framework
        .data
        .global
        .get::<KeyBindingsConfig>()
        .unwrap()
        .get(key)
    {
        Some(KeyAction::Paste) => {
            let textfield = framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture
                .as_mut()
                .unwrap();
            // a textfield must have a width set, because we don't know the width yet just set it to the
            // maximum value possible, so that the cursor will not be moved because of the small value
            textfield.set_width(u16::MAX);
            let content = get_clipboard();
            if content.is_empty() {
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Error(String::from("Clipboard empty"));
                return true;
            }

            content.chars().for_each(|c| {
                let _ = textfield.push(c);
            });
            return true;
        }
        Some(KeyAction::RemoveWord) => {
            remove_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            return true;
        }
        Some(KeyAction::ClearLine) => {
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture
                .as_mut()
                .unwrap()
                .content
                .clear();
            return true;
        }
        Some(KeyAction::PreviousWord) => {
            previous_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            return true;
        }
        Some(KeyAction::NextWord) => {
            next_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            return true;
        }
        _ => {}
    }

    let textfield = framework
        .data
        .global
        .get_mut::<Status>()
        .unwrap()
        .command_capture
        .as_mut()
        .unwrap();
    // a textfield must have a width set, because we don't know the width yet just set it to the
    // maximum value possible, so that the cursor will not be moved because of the small value
    textfield.set_width(u16::MAX);

    match key.code {
        KeyCode::Char(c) => textfield.push(c).is_ok(),
        KeyCode::Up => textfield.first().is_ok(),
        KeyCode::Down => textfield.last().is_ok(),
        KeyCode::Left => textfield.left().is_ok(),
        KeyCode::Right => textfield.right().is_ok(),
        KeyCode::Backspace => textfield.pop().is_ok(),
        KeyCode::Enter => {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Command(apply_envs(textfield.content.clone())));
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture = None;
            true
        }
        _ => false,
    }
}
