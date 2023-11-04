use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::Result;
use crate::{
    ctx::Ctx,
    models::{CreateTicket, ModelController, Ticket},
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route(
            "/tickets",
            post(create_ticket_handler).get(list_ticket_handler),
        )
        .route("/tickets/:id", delete(delete_ticket_handler))
        .with_state(mc)
}

async fn create_ticket_handler(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket): Json<CreateTicket>,
) -> Result<Json<Ticket>> {
    let created_ticket = mc.create_ticket(ctx, ticket).await?;
    Ok(Json(created_ticket))
}

async fn list_ticket_handler(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket_handler(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<usize>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delete(ctx, id).await?;
    Ok(Json(ticket))
}
