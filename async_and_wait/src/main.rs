use std::thread;
use std::{
    future::Future,
    pin::{pin, Pin},
    time::Duration,
};
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
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });

    // Async performs multiple tasks USING THE SAME THREAD by context switching
    // blocked tasks (cooperative)
    //   - concurrency only
    //   - no inherint parallelism
    // Threading creates new threads (in kernel-space) that could be picked up by other CPUs
    //   - Pre-emptive concurrency within the same CPU
    //   - Parallelism across CPUs
    // Fibers are executed by user-space threads
    //   - Cooperative concurrency within the same CPU
    //   - Parallelism across CPUs
    // Cooperative concurrency shines most when the workload is mostly blocking (ie. IO)

    trpl::run(async {
        // Handling multiple futures dynamically
        // Use `pin` to get dynamic type inference without having to use Box
        // which does heap allocation.
        let fut1 = pin!(async move {
            println!("hello");
            trpl::sleep(Duration::from_secs(1)).await;
        });
        let fut2 = pin!(async move {
            println!("world");
            trpl::sleep(Duration::from_secs(1)).await;
        });
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut1, fut2];
        trpl::join_all(futures).await;

        // Handling mutliple futures of different return types but statically
        let a = async { 1u32 };
        let b = async { "hello" };
        let c = async { true };
        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");

        // Racing futures to only require some futures to finish.
        // trpl::race used `select` under the hood.
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };
        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };
        // Order matters here since the implementation executes in order.
        // Other impls are fair in that they may get chosen randomly.
        trpl::race(slow, fast).await;

        // Yielding cooperatively for non-Futures-aware code to run interleaved.
        fn slow_fn(name: &str, ms: u64) {
            thread::sleep(Duration::from_millis(ms));
            println!("'{name}' ran for {ms}ms");
        }

        let a = async {
            // Expensive sleep that won't context switch automatically
            slow_fn("a", 30);
            // Yield to make progress with other task
            trpl::yield_now().await;
            slow_fn("a", 50);
        };
        let b = async {
            // Expensive sleep that won't context switch automatically
            slow_fn("b", 75);
            // Yield to make progress with other task
            trpl::yield_now().await;
            slow_fn("b", 10);
        };
        trpl::race(a, b).await;
    });
}
