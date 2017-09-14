#[macro_use]
extern crate neon;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod commands;
mod heroku_api;

use std::error::Error;
use std::io::prelude::*;
use neon::js::{JsNull, JsString, Value};
use neon::mem::Handle;
use neon::vm::{Call, JsResult};

fn init(mut call: Call) -> JsResult<JsNull> {
    let name = fetch_arg::<JsString>(&mut call, 0)?.value();
    let cmd = commands::Init {
        name: name
    };

    cmd.execute().unwrap_or_else(|err| {
        let mut stderr = std::io::stderr();
        writeln!(
            &mut stderr,
            "I/O Error: {}",
            err.description()
        ).expect("Could not write to stderr");
    });

    Ok(JsNull::new())
}

fn register(mut call: Call) -> JsResult<JsNull> {
    let repo = fetch_arg::<JsString>(&mut call, 0)?.value();
    let namespace = fetch_arg::<JsString>(&mut call, 1)?.value();
    let name = fetch_arg::<JsString>(&mut call, 2)?.value();
    // coerce into string if team flag is passed
    let team = match check_arg::<JsString>(&mut call, 3) {
        Some(handle) => Some(handle.value()),
        None => None,
    };

    let cmd = commands::Register {
        repo: repo,
        namespace: namespace,
        name: name,
        team: team,
    };

    cmd.execute();

    Ok(JsNull::new())
}

fn search(mut call: Call) -> JsResult<JsNull> {
    let cmd = commands::Search {
        name: "".to_owned(),
    };

    cmd.execute();

    Ok(JsNull::new())
}

fn publish(mut call: Call) -> JsResult<JsNull> {
    let namespace = fetch_arg::<JsString>(&mut call, 0)?.value();
    let name = fetch_arg::<JsString>(&mut call, 1)?.value();
    let tag = fetch_arg::<JsString>(&mut call, 2)?.value();

    let cmd = commands::Publish {
        namespace: namespace,
        name: name,
        tag: tag,
    };

    cmd.execute();

    Ok(JsNull::new())
}

fn fetch_arg<'a, T: Value>(call: &mut Call<'a>, index: i32) -> JsResult<'a, T> {
    call.arguments.require(call.scope, index)?.check::<T>()
}

fn check_arg<'a, T: Value>(call: &mut Call<'a>, index: i32) -> Option<Handle<'a, T>> {
    call.arguments.get(call.scope, index).and_then(|handle| handle.downcast::<T>())
}

register_module!(m, {
    m.export("register", register)?;
    m.export("init", init)?;
    m.export("search", search)?;
    m.export("publish", publish)?;
    Ok(())
});
