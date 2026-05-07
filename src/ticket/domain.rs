use chrono::{DateTime, Utc};
use nanoid::nanoid;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef};

#[derive(Debug)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub priority: TicketPriority,
    pub closed: bool,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub last_updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum TicketPriority {
    Standard,
    Prioritized,
    Urgent,
}

impl ToSql for TicketPriority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val: i64 = match self {
            Self::Standard => 0,
            Self::Prioritized => 1,
            Self::Urgent => 2,
        };

        Ok(ToSqlOutput::Owned(Value::from(val)))
    }
}

impl FromSql for TicketPriority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let val: Option<i64> = value.as_i64_or_null()?;

        match val.unwrap_or_default() {
            0 => Ok(TicketPriority::Standard),
            1 => Ok(TicketPriority::Prioritized),
            2 => Ok(TicketPriority::Urgent),
            _ => Err(FromSqlError::OutOfRange(3)),
        }
    }
}

impl Ticket {
    pub fn new(title: &str, subject: &str, priority: TicketPriority) -> Self {
        Self {
            id: nanoid!(),
            title: String::from(title),
            subject: String::from(subject),
            priority,
            closed: false,
            created_at: Utc::now(),
            closed_at: None,
            last_updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_new() {
        let title = "Test Title";
        let subject = "Test Subject Context";
        let ticket = Ticket::new(title, subject, TicketPriority::Standard);

        assert_eq!(ticket.title, title);
        assert_eq!(ticket.subject, subject);
        assert!(matches!(ticket.priority, TicketPriority::Standard));
        assert!(!ticket.closed);
        assert!(ticket.closed_at.is_none());
        assert!(ticket.last_updated_at.is_none());
    }

    #[test]
    fn test_ticket_priority_values() {
        assert!(matches!(TicketPriority::Standard, TicketPriority::Standard));
        assert!(matches!(
            TicketPriority::Prioritized,
            TicketPriority::Prioritized
        ));
        assert!(matches!(TicketPriority::Urgent, TicketPriority::Urgent));
    }
}
