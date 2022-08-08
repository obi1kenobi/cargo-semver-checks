use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;

// a helper joins all values
handlebars_helper!(join: |sep: str, args: Value| {
    if let Value::Array(arr) = args {
        arr.iter().map(|x| x.as_str().expect("non-string included")).collect::<Vec<&str>>().join(sep)
    } else {
        unreachable!("non-array value provided: {args:?}")
    }
});

pub(crate) fn make_handlebars_registry() -> Handlebars<'static> {
    let mut registry = Handlebars::new();
    registry.set_strict_mode(true);
    registry.register_helper("join", Box::new(join));
    registry
}
