use my_http_server::macros::*;
use my_http_server::*;

#[http_route(
    method: "GET",
    route: "/",
)]
pub struct IndexAction;

async fn handle_request(
    _action: &IndexAction,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = ctx.request.get_ip().get_real_ip_as_string();
    return HttpOutput::as_text(result).into_ok_result(true).into();
}
