use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: usize,
    pub user_id: u64,
    pub title: String,
}

#[derive(Clone, Deserialize)]
pub struct CreateTicket {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    pub ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket: CreateTicket) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len();
        let created_ticket = Ticket {
            id,
            user_id: ctx.user_id(),
            title: ticket.title,
        };
        store.push(Some(created_ticket.clone()));
        info!(?created_ticket, "In Creating Tickets");
        Ok(created_ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        info!(?tickets, "In List Tickets");
        Ok(tickets)
    }

    pub async fn delete(&self, _ctx: Ctx, id: usize) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let deleted_ticket = store.get_mut(id).and_then(|t| t.take());
        info!(?deleted_ticket, ?id, "In delete");
        deleted_ticket.ok_or(Error::NoTicketID { id })
    }
}
