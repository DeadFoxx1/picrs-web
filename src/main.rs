use leptos::html::Div;
use leptos::mount::mount_to_body;
use leptos::{ev, prelude::*};
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
#[component]
fn App() -> impl IntoView {
    let (rowcount, set_rowcount) = signal(5 as usize);
    let (colcount, set_colcount) = signal(5 as usize);
    let grid: NodeRef<Div> = NodeRef::new();

    let handle = window_event_listener(ev::resize, move |_| {
        set_rowcount.set(rowcount.get())
    });
    on_cleanup(move || handle.remove());

    //this should be updated as soon as the grid is mounted
    let (cell_size, set_cell_size) = signal(0.0);

    Effect::new(move || {
        if let Some(element) = grid.get() {
            let col_size = element.client_width() as f64 / colcount.get() as f64;
            let row_size = element.client_height() as f64 / rowcount.get() as f64;

            if col_size < row_size {
                set_cell_size.set(col_size)
            } else {
                set_cell_size.set(row_size)
            }
        }
    });

    view! {
        <div>
            <Slider name="number of rows: ".to_string() read=rowcount write=set_rowcount></Slider>
        </div>
        <div>
            <Slider name="number of cols: ".to_string() read=colcount write=set_colcount></Slider>
        </div>

        <div class="grid" node_ref=grid>

            <For
                each=move || 0..rowcount.get()
                key=|index| *index
                children=move |y| {
                    view! {
                        <div
                            class="row"
                            style:top=move || format!("{}px", cell_size.get() * y as f64)
                            style:width=move || {
                                format!("{}px", cell_size.get() * colcount.get() as f64)
                            }
                            style:height=move || format!("{}px", cell_size.get())
                        >
                            <For
                                each=move || 0..colcount.get()
                                key=|index| *index
                                children=move |_| {
                                    view! { <div class="cell"></div> }
                                }
                            />
                        </div>
                    }
                }
            />
        </div>
    }
}
#[component]
fn Slider(read: ReadSignal<usize>, write: WriteSignal<usize>, name: String) -> impl IntoView {
    view! {
        <label for="slider">{name}{read}<div></div></label>
        <input
            type="range"
            id="slider"
            min="1"
            max="100"
            value=read
            on:input=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap_or(1);
                write.set(value);
            }
        />
    }
}
