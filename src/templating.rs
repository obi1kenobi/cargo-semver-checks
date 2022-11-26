use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;

// a helper to join all values
handlebars_helper!(join: |sep: str, args: Value| {
    if let Value::Array(arr) = args {
        arr.iter().map(|x| x.as_str().expect("non-string included")).collect::<Vec<&str>>().join(sep)
    } else {
        unreachable!("non-array value provided: {args:?}")
    }
});

// a helper to unpack a one-element list to get the single element, or leave the list as-is otherwise
handlebars_helper!(unpack_if_singleton: |arg: Value| {
    if let Value::Array(arr) = &arg {
        if arr.len() == 1 {
            arr.first().unwrap().clone()
        } else {
            arg
        }
    } else {
        unreachable!("non-array value provided: {arg:?}")
    }
});

// a helper to pretty-print multiple spans, submitted as parallel lists of file names and line numbers
handlebars_helper!(multiple_spans: |files: Value, begin_line_numbers: Value| {
    match (&files, &begin_line_numbers) {
        (Value::Array(files), Value::Array(begin_line_numbers)) if files.len() == begin_line_numbers.len() => {
            let formatted_values: Vec<_> = files.iter().zip(begin_line_numbers).filter_map(|(file, begin_line)| {
                if file.is_null() {
                    assert!(begin_line.is_null());
                    None
                } else {
                    let file = file.as_str().expect("file was not a string");
                    let begin_line = begin_line.as_u64().expect("begin line number was not a u64");
                    Some(format!("{file}:{begin_line}"))
                }
            }).collect();
            if formatted_values.len() == 1 {
                formatted_values.into_iter().next().unwrap()
            } else {
                format!("[ {} ]", formatted_values.join(" , "))
            }
        }
        (Value::Array(files), Value::Array(begin_line_numbers)) => {
            unreachable!("the arrays did not have the same length: {files:?} {begin_line_numbers:?}")
        }
        _ => unreachable!("non-array values provided: {files:?} {begin_line_numbers:?}"),
    }
});

pub(crate) fn make_handlebars_registry() -> Handlebars<'static> {
    let mut registry = Handlebars::new();
    registry.set_strict_mode(true);
    registry.register_helper("join", Box::new(join));
    registry.register_helper("unpack_if_singleton", Box::new(unpack_if_singleton));
    registry.register_helper("multiple_spans", Box::new(multiple_spans));
    registry
}
