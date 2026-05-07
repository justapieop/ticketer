use ticketer::ticket::domain::{Ticket, TicketPriority};
use ticketer::ticket::infrastructure::SqliteTicketRepository;
use ticketer::ticket::application::TicketRepository;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn setup_repository() -> SqliteTicketRepository {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
    let shared_conn = Arc::new(Mutex::new(conn));
    SqliteTicketRepository::new(shared_conn)
}

#[test]
fn test_save_and_list_ticket() {
    let repo = setup_repository();

    assert!(repo.list_ticket().is_empty());

    let ticket1 = Ticket::new("Issue 1", "App crashes", TicketPriority::Urgent);
    let ticket2 = Ticket::new("Issue 2", "Typo in UI", TicketPriority::Standard);

    repo.save_ticket(ticket1).unwrap();
    repo.save_ticket(ticket2).unwrap();

    let tickets = repo.list_ticket();
    assert_eq!(tickets.len(), 2);
}

#[test]
fn test_get_ticket_not_found() {
    let repo = setup_repository();
    let ticket = repo.get_ticket("nonexistent_id");
    assert!(ticket.is_none());
}

#[test]
fn test_update_ticket_fields() {
    let repo = setup_repository();
    let ticket = Ticket::new("Initial Title", "Initial Subject", TicketPriority::Standard);
    let target_id = ticket.id.clone();
    
    repo.save_ticket(ticket).unwrap();

    repo.set_title(&target_id, "Updated Title").unwrap();
    repo.set_subject(&target_id, "Updated Subject").unwrap();
    repo.set_priority(&target_id, TicketPriority::Urgent).unwrap();

    let updated_ticket = repo.get_ticket(&target_id).unwrap();
    
    assert_eq!(updated_ticket.title, "Updated Title");
    assert_eq!(updated_ticket.subject, "Updated Subject");
    assert!(matches!(updated_ticket.priority, TicketPriority::Urgent));
}

#[test]
fn test_close_ticket() {
    let repo = setup_repository();
    let ticket = Ticket::new("Close Me", "Subject", TicketPriority::Standard);
    let target_id = ticket.id.clone();
    
    repo.save_ticket(ticket).unwrap();
    
    let initial_ticket = repo.get_ticket(&target_id).unwrap();
    assert!(!initial_ticket.closed);

    repo.close_ticket(&target_id).unwrap();

    let closed_ticket = repo.get_ticket(&target_id).unwrap();
    assert!(closed_ticket.closed);
    assert!(closed_ticket.closed_at.is_some());
}