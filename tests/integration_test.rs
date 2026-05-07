use ticketer::app_state::{AppMode, AppOutput, AppState};
use ticketer::ticket::application::TicketService;
use ticketer::ticket::command::{CreateTicketCommand, DisplayTicketCommand, EditTicketCommand, ListTicketCommand};
use ticketer::command::Command;
use rusqlite::Connection;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

fn setup_state() -> Arc<AppState> {
    let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
    let ticket_service = Arc::new(TicketService::new(conn.clone()));

    Arc::new(AppState {
        conn,
        running: AtomicBool::new(true),
        ticket_service,
        output: Mutex::new(AppOutput::None),
        mode: Mutex::new(AppMode::Normal),
    })
}

#[test]
fn test_ticket_commands_integration() {
    let state = setup_state();

    let create_cmd = CreateTicketCommand {};
    create_cmd
        .execute(
            state.clone(),
            vec!["TestTitle".to_string(), "Test Subject".to_string()],
        )
        .unwrap();

    let id = {
        let output = state.output.lock().unwrap();
        if let AppOutput::Text(t) = &*output {
            assert!(t.starts_with("Created ticket "));
            t.replace("Created ticket ", "")
        } else {
            panic!("Expected Text output");
        }
    };

    let list_cmd = ListTicketCommand {};
    list_cmd.execute(state.clone(), vec![]).unwrap();

    {
        let output = state.output.lock().unwrap();
        if let AppOutput::TicketList(list) = &*output {
            assert_eq!(list.len(), 1);
            assert_eq!(list[0].id, id);
            assert_eq!(list[0].title, "TestTitle");
        } else {
            panic!("Expected TicketList output");
        }
    }

    let get_cmd = DisplayTicketCommand {};
    get_cmd.execute(state.clone(), vec![id.clone()]).unwrap();

    {
        let output = state.output.lock().unwrap();
        if let AppOutput::Ticket(t) = &*output {
            assert_eq!(t.id, id);
            assert_eq!(t.title, "TestTitle");
        } else {
            panic!("Expected Ticket output");
        }
    }

    let edit_cmd = EditTicketCommand {};
    edit_cmd.execute(state.clone(), vec![id.clone()]).unwrap();

    {
        let mode = state.mode.lock().unwrap();
        match &*mode {
            AppMode::ChoosingEditField(edited_id) => {
                assert_eq!(edited_id, &id);
            }
            _ => panic!("Expected ChoosingEditField mode"),
        }
    }
}