use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Get `output_path`, specified by `-o`
pub fn get_output_path(args: &[String]) -> crate::Result<&String> {
    args.windows(2)
        .find_map(|x| (x[0] == "-o").then(|| &x[1]))
        .ok_or_else(|| "(BUG?) `-o` flag not found".into())
}

/// Get `search_paths`, specified by `-L`
pub fn get_search_paths(args: &[String]) -> Vec<PathBuf> {
    args.windows(2)
        .filter(|&x| (x[0] == "-L"))
        .map(|x| PathBuf::from(&x[1]))
        .inspect(|path| log::trace!("new search path: {}", path.display()))
        .collect()
}

/// Get `search_targets`, the names of the linker scripts, specified by `-T`
pub fn get_search_targets(args: &[String]) -> Vec<Cow<str>> {
    args.iter()
        .filter_map(|arg| arg.strip_prefix("-T").map(Cow::Borrowed))
        .collect()
}

pub fn load_args_from_path(args: &Vec<String>) -> Vec<String> {
    let mut ret = args.clone();
    if args[args.len() - 1].starts_with('@') {
        let file = File::open(ret.pop().unwrap().strip_prefix('@').unwrap()).unwrap();
        let lines = BufReader::new(file).lines().map(|r| {
            r.unwrap()
                .strip_prefix('\"')
                .unwrap()
                .strip_suffix('\"')
                .unwrap()
                .to_string()
        });
        ret.extend(lines);
    }
    ret
}
