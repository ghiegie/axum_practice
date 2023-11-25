use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{error_mod::*, ctx_mod::Ctx};
use crate::model_mod::*;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets)) // post for /ticket and get for /ticket
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc) // so that all handlers can have access to the state
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    Path(id): Path<u64>,
    ctx: Ctx,
    State(mc): State<ModelController>,
) -> Result<Json<Ticket>> {
    println!(">>> {:<15} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
