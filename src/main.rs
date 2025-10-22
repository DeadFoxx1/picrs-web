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
    let table: NodeRef<Div> = NodeRef::new();

    let cell_size = Signal::derive(move || {
        min(
            table.get().unwrap().client_height() as usize / rowcount.get(),
            table.get().unwrap().client_width() as usize / colcount.get(),
        )
    });

    view! {
        <div>
            <Slider name="number of rows: ".to_string() read=rowcount write=set_rowcount></Slider>
        </div>
        <div>
            <Slider name="number of cols: ".to_string() read=colcount write=set_colcount></Slider>
        </div>

        <div class="table" node_ref=table>
            <For
                each=move || 0..rowcount.get()
                key=|index| *index
                children=move |_| {
                    view! { <Row style:height=move || format!("{}px", cell_size.get()) size=cell_size n=colcount></Row> }
                }
            />
        </div>
    }
}

#[component]
fn Slider(read: ReadSignal<usize>, write: WriteSignal<usize>, name: String) -> impl IntoView {
    view! {
        <label for="slider">{name} {read}</label>
        <input
            type="range"
            id="slider"
            min="1"
            max="50"
            value=read
            on:input=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap_or(0);
                write.set(value);
            }
        />
    }
}

#[component]
fn Row(n: ReadSignal<usize>, #[prop(into)] size: Signal<usize>) -> impl IntoView {
    view! {
        <div class="row">
            <For
                each=move || 0..n.get()
                key=|index| *index
                children=move |_| {
                    view! {
                        <div style:width=move || format!("{}px", size.get()) class="cell"></div>
                    }
                }
            />
        </div>
    }
}
