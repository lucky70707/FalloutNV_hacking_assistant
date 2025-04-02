use leptos::{
    ev::{MouseEvent, SubmitEvent},
    html::{self, h1, html},
    logging,
    prelude::*,
};

pub fn run_gui() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
}

#[component]
fn App() -> impl IntoView {  
    let (toggled, set_toggled) = signal(false);
    let help_text = move || {
        if toggled.get() {
            Some("This assistant will help you solve the hacking minigame from fallout 3, new vegas, 4 and 76. I highly recommend first clicking 1 word in the minigame before continuing just in case it's immediately correct. If it is not, proceed by filling in all the words displayed in the terminal you're hacking. You can first fill in the wordt length otherwise it will be filled in automatically upon filling in your first word. All the filled in words will be displayed under \"current words:\". Upon filling in all the words on the terminal, press the [FINISHED] button and proceed to the Guessing form. Fill in the word you guessed and the amount correct. Press the [SUBMIT] button afterwards and the list will shrink accordingly. You can then pick a word from the remaining words list to guess in-game. Repeat this until the password is correct or you're locked out. Press the [RESTART] button to begin the process all over again."
       )
        }else {
            None
        }
    };
    view! {
        <Header render_prop=|| view! { <h1>"Fallout Hacking Assistant"</h1>  }>
        // these get passed to `children`
        <ToggleButton setter=set_toggled id="btnHelp".to_string() text="HELP".to_string()> </ToggleButton>

        
    </Header>
    <section>

    </section>
    <article id="articleHelp">
        <p>
        {help_text}
         </p>        
    </article>
    }
}

#[component]
fn Header<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` can take one of several different types, each of which
    /// is a function that returns some view type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <header class="flex-container">
        {render_prop()}
        {children()}
        </header>
    }
}

#[component]
fn ToggleButton(setter: WriteSignal<bool>, id: String, text: String) -> impl IntoView {
    view! {
        <button id=id
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            {text}
        </button>
    }
}

#[component]
pub fn Button(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}
