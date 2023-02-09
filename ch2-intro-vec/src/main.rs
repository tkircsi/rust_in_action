fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    It is the same with books.
    What do we seek
    through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line.trim());
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line);
        }
        println!("-----");
    }

    println!("=======");

    my_solution();
}

fn my_solution() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    It is the same with books.
    What do we seek
    through millions of pages?";

    for (i, line) in haystack.lines().enumerate() {
        let lower_bound = i.saturating_sub(ctx_lines);
        let upper_bound = if i + ctx_lines + 1 >= haystack.lines().count() {
            haystack.lines().count()
        } else {
            i + ctx_lines + 1
        };

        if line.contains(needle) {
            let v: Vec<String> = haystack.lines().map(String::from).collect();
            for (i, line) in v[lower_bound..upper_bound].into_iter().enumerate() {
                println!("{}: {}", i + 1, line.trim());
            }
            println!("------");
        }
    }
}
