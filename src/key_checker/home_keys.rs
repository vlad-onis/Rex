use crate::key_checker::InputKeyHandler;
use crate::outputs::HandlingOutput;
use crate::page_handler::PopupState;
use crossterm::event::KeyCode;

/// Tracks the keys of the Home page and calls relevant function based on it
#[cfg(not(tarpaulin_include))]
pub fn home_keys(handler: &mut InputKeyHandler) -> Option<HandlingOutput> {
    match handler.popup {
        PopupState::Nothing => match handler.key.code {
            KeyCode::Char('q') => return Some(HandlingOutput::QuitUi),
            KeyCode::Char('a') => handler.go_add_tx(),
            KeyCode::Char('r') => handler.go_chart(),
            KeyCode::Char('j') => return Some(HandlingOutput::TakeUserInput),
            KeyCode::Char('h') => handler.do_help_popup(),
            KeyCode::Char('z') => handler.go_summary(),
            KeyCode::Char('w') => handler.go_search(),
            KeyCode::Char('e') => handler.edit_tx(),
            KeyCode::Char('d') => handler.do_deletion_popup(),
            KeyCode::Right => handler.handle_right_arrow(),
            KeyCode::Left => handler.handle_left_arrow(),
            KeyCode::Up => handler.handle_up_arrow(),
            KeyCode::Down => handler.handle_down_arrow(),
            _ => {}
        },
        PopupState::TxDeletion => match handler.key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Enter => handler.handle_deletion_popup(),
            _ => {}
        },
        _ => handler.do_empty_popup(),
    }
    None
}
