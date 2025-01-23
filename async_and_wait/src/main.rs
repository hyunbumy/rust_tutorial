use trpl::{Either, Html};

// Mark page_title as an async function.
// When a code block is marked with `async`, Rust compiles it into a unique,
// anonymous data type that implements the `Future` trait.
// When a function is marked with `async`, it complies it into a non-async
// function whose body is an async block. Thus the return type is the type of
// anonymous data type that the compiler creates for the async block.
// This is like:
/*
fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
*/
async fn page_title(url: &str) -> (&str, Option<String>) {
    // Annotate with `await` to block on the response of the methods.
    // Futures are _lazy_ otherwise.
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    // We could also write `let response_text = trpl::get(url).await.text().await`.
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    // Any types that implement the `Future` trait are considered futures.

    // `async` keyword can be applied to blocks and functions to specify that
    // they can be interrupted and resumed.
    // Within an async block or async function, one can use `await` keyword to
    // wait for a future to become ready.

    let args: Vec<String> = std::env::args().collect();
    // The only place we can use the `await` keyword is in async functions or
    // block so we cannot use it in main directly. This is because async code
    // needs a runtime that instructs how to run async code.
    // Every Rust async program needs at least one place where it sets up a
    // runtime and executes the futures.
    // What's a runtime???
    // Similar to executors?
    /*
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title"),
        None => println!("{urls} had no title"),
    }
    */
    // Instead we use trpl::run that sets up a runtime and blocks on the
    // execution of the future / async block.
    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        // Take the first returning Future's values
        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        
        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
