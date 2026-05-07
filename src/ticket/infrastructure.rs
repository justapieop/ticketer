use std::{
    error::Error,
    sync::{Arc, Mutex},
    vec,
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
              id TEXT PRIMARY KEY,
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

    fn get_ticket(&self, id: &str) -> Option<Ticket> {
        self.conn
            .lock()
            .unwrap()
            .query_row("SELECT * FROM tickets WHERE id = ?1", (id,), |row| {
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
            })
            .ok()
    }

    fn set_priority(&self, id: &str, priority: TicketPriority) -> Result<(), Box<dyn Error + '_>> {
        self.conn.lock()?.execute(
            r#"
            UPDATE tickets SET priority = ?1, last_updated_at = CURRENT_TIMESTAMP WHERE id = ?2;
        "#,
            (priority, id),
        )?;

        Ok(())
    }

    fn list_ticket(&self) -> Vec<Ticket> {
        let conn = match self.conn.lock() {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        let mut stmt = match conn.prepare("SELECT * FROM tickets ORDER BY created_at DESC") {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        let ticket_iter = match stmt.query_map([], |row| {
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
        }) {
            Ok(iter) => iter,
            Err(_) => return vec![],
        };

        ticket_iter.filter_map(Result::ok).collect()
    }

    fn close_ticket(&self, id: &str) -> Result<(), Box<dyn Error + '_>> {
        self.conn.lock()?.execute(
            r#"
            UPDATE tickets SET closed = true, closed_at = CURRENT_TIMESTAMP, last_updated_at = CURRENT_TIMESTAMP WHERE id = ?1
        "#,
            (id,) ,
        )?;
        Ok(())
    }

    fn set_title(&self, id: &str, title: &str) -> Result<(), Box<dyn Error + '_>> {
        self.conn
            .lock()?
            .execute("UPDATE tickets SET title = ?1 WHERE id = ?2", (title, id))?;

        Ok(())
    }

    fn set_subject(&self, id: &str, subject: &str) -> Result<(), Box<dyn Error + '_>> {
        self.conn.lock()?.execute(
            "UPDATE tickets SET subject = ?1 WHERE id = ?2",
            (subject, id),
        )?;

        Ok(())
    }
}
