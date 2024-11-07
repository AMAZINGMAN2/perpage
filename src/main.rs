// create a demo sycamore page that says hello

use sycamore::prelude::*;

fn main() {
sycamore::render(|| {
    view! {
        h1 { "W rust, this compiles so fast idek how bruh, and the code says 'unoptimised' when its compiling localy, I wonder how fast it will be when compiling deployed to cloudflare lol, such a gigachad thing lol, surley I could find some good rust packages to use" }
    }
});
}
