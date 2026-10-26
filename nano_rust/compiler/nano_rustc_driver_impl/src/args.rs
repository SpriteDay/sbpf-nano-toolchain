use std::env;

/// Gets the raw unprocessed command-line arguments as Unicode strings, without doing any further
/// processing
///
/// This function is identical to [`env::args()`] except that it emits an error when it encounters
/// non-Unicode arguments instead of panicking.
pub fn raw_args() -> Vec<String> {
    let mut args = Vec::new();
    let mut guar = Ok(());
    for (i, arg) in env::args_os().enumerate() {
        match arg.into_string() {
            Ok(arg) => args.push(arg),
            Err(arg) => {
                guar = Err(format!("Argument {i} is not valid Unicode: {arg:?}"));
            }
        };
    }
    if let Err(guar) = guar {
        panic!("{guar}");
    }
    args
}
