pub mod date;
pub mod dump;

use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok(date::get_date())
}
