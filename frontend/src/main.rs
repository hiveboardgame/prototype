#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use frontend::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
