use leptos::mount::mount_to_body;
use leptos::prelude::*;
use picrs_lib::table::Table;
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Grid);
}
#[component]
fn Grid() -> impl IntoView {
    let (rowcount, set_rowcount) = signal(5);
    let (colcount, set_colcount) = signal(5);

    view! {
        <div>
            <Slider read=rowcount write=set_rowcount></Slider>
        </div>
        <div>
            <Slider read=colcount write=set_colcount></Slider>
        </div>

        <div>
            <For
                each=move || 0..rowcount.get()
                key=|index| *index
                children=move |_| {
                    view! { <RowList n=colcount></RowList> }
                }
            />
        </div>
    }
}

#[component]
fn Slider(read: ReadSignal<usize>, write: WriteSignal<usize>) -> impl IntoView {
    view! {
        <label for="slider">"number of rows: " {read}</label>
        <input
            type="range"
            id="slider"
            min="1"
            max="20"
            value=read
            on:input=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap_or(0);
                write.set(value);
            }
        />
    }
}

#[component]
fn RowList(n: ReadSignal<usize>) -> impl IntoView {
    view! {
        <div>
            <For
                each=move || 0..n.get()
                key=|index| *index
                children=move |index| {
                    view! { <button>{format!("Button {}", index + 1)}</button> }
                }
            />
        </div>
    }
}
