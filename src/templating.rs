use handlebars::{
    BlockContext, Context, Handlebars, Helper, Output, RenderContext, RenderError,
    RenderErrorReason, Renderable, handlebars_helper, to_json,
};
use serde_json::Value;

// a helper to lowercase a string
handlebars_helper!(lowercase: |arg: Value| {
    if let Value::String(arg) = arg {
        arg.to_ascii_lowercase()
    } else {
        unreachable!("non-string value provided: {arg:?}")
    }
});

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

// a helper to convert arg to a string, submitted as a number, string, or bool
handlebars_helper!(to_string: |arg: Value| {
    match arg {
        Value::Number(num) => num.to_string(),
        Value::String(already_string) => already_string.to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        _ => unreachable!("non-stringifiable value provided: {arg:?}")
    }
});

// a helper to loop n number of times. similar to #each, has @index, @first, and @last. does not set `this`
// written without the handlebars_helper! for block access
// this is largely based on the logic within #each itself
fn repeat<'reg, 'rc>(
    helper: &Helper<'rc>,
    registry: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    render_ctx: &mut RenderContext<'reg, 'rc>,
    output: &mut dyn Output,
) -> Result<(), RenderError> {
    let value = helper
        .param(0)
        .ok_or(RenderErrorReason::ParamNotFoundForIndex("repeat", 0))?;

    let template = helper.template();

    match template {
        Some(template) => match *value.value() {
            // If number encountered
            Value::Number(ref count) => {
                // Create block
                let mut block = BlockContext::new();

                if let Some(new_path) = value.context_path() {
                    block.base_path_mut().clone_from(new_path);
                } else {
                    block.set_base_value(Value::Number(count.clone()));
                }

                render_ctx.push_block(block);

                // Get range
                let range = count
                    .as_u64()
                    .ok_or(RenderErrorReason::InvalidParamType("Uint64"))?;

                // Loop over range
                for index in 0..range {
                    if let Some(ref mut block) = render_ctx.block_mut() {
                        let is_first = index == 0u64;
                        let is_last = index == range - 1;

                        // Set local variables
                        block.set_local_var("first", Value::Bool(is_first));
                        block.set_local_var("last", Value::Bool(is_last));
                        block.set_local_var("index", to_json(index));
                    }

                    // Render with current context
                    template.render(registry, ctx, render_ctx, output)?;
                }

                render_ctx.pop_block();

                Ok(())
            }

            // If any other type encountered
            _ => {
                if registry.strict_mode() {
                    Err(RenderError::strict_error(value.relative_path()))
                } else {
                    Ok(())
                }
            }
        },
        None => Ok(()),
    }
}

pub(crate) fn make_handlebars_registry() -> Handlebars<'static> {
    let mut registry = Handlebars::new();
    registry.set_strict_mode(true);
    registry.register_escape_fn(handlebars::no_escape);
    registry.register_helper("lowercase", Box::new(lowercase));
    registry.register_helper("join", Box::new(join));
    registry.register_helper("unpack_if_singleton", Box::new(unpack_if_singleton));
    registry.register_helper("multiple_spans", Box::new(multiple_spans));
    registry.register_helper("to_string", Box::new(to_string));
    registry.register_helper("repeat", Box::new(repeat));
    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_helper_does_not_leak_block_context() {
        let registry = make_handlebars_registry();

        // Basic rendering works as expected.
        let rendered = registry
            .render_template("{{#repeat 2}}{{@index}}{{/repeat}}", &serde_json::json!({}))
            .expect("render failed");
        assert_eq!(rendered, "01");

        // Referencing `@index` outside the helper should be an error.
        registry
            .render_template(
                "{{#repeat 1}}{{@index}}{{/repeat}}{{@index}}",
                &serde_json::json!({}),
            )
            .expect_err("block context leaked outside of repeat helper");
    }
}
