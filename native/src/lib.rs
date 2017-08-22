#[macro_use]
extern crate neon;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod commands;
mod options;
mod heroku_api;

use neon::vm::{Call, JsResult};
use neon::js::{JsNull, JsString};

fn register(call: Call) -> JsResult<JsNull> {
    let scope = call.scope;
    let repo = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
    let namespace = call.arguments.require(scope, 1)?.check::<JsString>()?.value();
    let name = call.arguments.require(scope, 2)?.check::<JsString>()?.value();

    let cmd = commands::Register {
        repo: repo,
        namespace: namespace,
        name: name,
    };

    cmd.execute();

    Ok(JsNull::new())
}

register_module!(m, {
    m.export("register", register);
    Ok(())
});
