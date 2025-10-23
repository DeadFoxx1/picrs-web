use std::cmp::min;

use leptos::html::Div;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Grid);
}
#[component]
fn Grid() -> impl IntoView {
    let (rowcount, set_rowcount) = signal(5 as usize);
    let (colcount, set_colcount) = signal(5 as usize);
    // init at 0 but this should never be seen because the grid is mounted right away
    let (cellsize, set_cellsize) = signal(0 as f64);
    let table: NodeRef<Div> = NodeRef::new();

    table.on_load(move |_| {
        Effect::new(move |_| {
            if rowcount.get() > colcount.get() || rowcount.get() == colcount.get() {
                set_cellsize
                    .set(table.get().unwrap().client_height() as f64 / rowcount.get() as f64);
            } else {
                set_cellsize
                    .set(table.get().unwrap().client_width() as f64 / colcount.get() as f64);
            }
        });
    });

    view! {
        <div>
            <Slider name="number of rows: ".to_string() read=rowcount write=set_rowcount></Slider>
        </div>
        <div>
            <Slider name="number of cols: ".to_string() read=colcount write=set_colcount></Slider>
        </div>

        <div class="grid" node_ref=table>
            <For
                each=move || 0..rowcount.get()
                key=|index| *index
                children=move |index| {
                    view! {
                        <Row
                            style:height=move || format!("{}px", cellsize.get())
                            y=index
                            size=cellsize
                            n=colcount
                        ></Row>
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
            max="90"
            value=read
            on:input=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap_or(1);
                write.set(value);
            }
        />
    }
}

#[component]
fn Row(n: ReadSignal<usize>, #[prop(into)] size: Signal<f64>, y: usize) -> impl IntoView {
    view! {
        <div
            style:top=move || format!("{}px", size.get())
            class="row"
            style:height=move || format!("{}px", size.get())
        >
            <For
                each=move || 0..n.get()
                key=|index| *index
                children=move |index| {
                    view! {
                        <div
                            style:margin-bottom=move || format!("{}px", size.get() * y as f64)
                            style:left=move || format!("{}px", size.get() * index as f64)
                            style:width=move || format!("{}px", size.get())
                            class="cell"
                        ></div>
                    }
                }
            />
        </div>
    }
}
