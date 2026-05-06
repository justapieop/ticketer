use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;

use crate::ticket::{
    application::TicketRepository,
    domain::{Ticket, TicketPriority},
};

pub struct SqliteTicketRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteTicketRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        conn.lock()
            .unwrap()
            .execute(
                r#"
            CREATE TABLE IF NOT EXISTS tickets (
              id BLOB PRIMARY KEY,
              title TEXT NOT NULL,
              subject TEXT NOT NULL,
              priority INTEGER NOT NULL DEFAULT 0,
              closed INTEGER NOT NULL DEFAULT 0,
              created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
              closed_at INTEGER,
              last_updated_at INTEGER
            );
        "#,
                (),
            )
            .unwrap_or_default();

        Self { conn }
    }
}

impl TicketRepository for SqliteTicketRepository {
    fn save_ticket(&self, ticket: Ticket) -> Result<(), Box<dyn Error + '_>> {
        self.conn.lock()?.execute(
            r#"
        INSERT INTO tickets (id, title, subject, priority, closed, created_at, last_updated_at, closed_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);
        "#,
            (&ticket.id, &ticket.title, &ticket.subject, &ticket.priority, &ticket.closed, &ticket.created_at, &ticket.last_updated_at, &ticket.closed_at),
        )?;
        Ok(())
    }

    fn get_ticket(&self, id: &str) -> Result<Ticket, Box<dyn Error + '_>> {
        self.conn
            .lock()?
            .query_one("SELECT * FROM tickets WHERE id = ?1", (id,), |row| {
                Ok(Ticket {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    subject: row.get(2)?,
                    priority: row.get(3)?,
                    closed: row.get(4)?,
                    created_at: row.get(5)?,
                    closed_at: row.get(6)?,
                    last_updated_at: row.get(7)?,
                })
            })?;

        Err("Ticket not found".into())
    }

    fn set_priority(&self, id: &str, priority: TicketPriority) -> Result<(), Box<dyn Error + '_>> {
        self.conn.lock()?.execute(
            r#"
            UPDATE tickets SET priority = ?1 WHERE id = ?2;
        "#,
            (priority, id),
        )?;

        Ok(())
    }
}
