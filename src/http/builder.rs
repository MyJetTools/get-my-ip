use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

pub fn build_controllers() -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(super::controllers::home::IndexAction));

    result
}
