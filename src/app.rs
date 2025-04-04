use std::collections::HashSet;

use leptos::{
    ev::{input, MouseEvent, SubmitEvent},
    html::{self},
    
    prelude::*,

};

use crate::check_word_against_list;

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
        } else {
            None
        }
    };
    

    let test_hash_set: HashSet<String> = HashSet::new();
    let remaining_hash_set: HashSet<String> = HashSet::new();

    let (current_word_list, set_current_wordlist) = signal(test_hash_set);
    let (remaining_wordlist, set_remaining_wordlist) = signal(remaining_hash_set);

    let min_wordlength = signal(4 as u8);
    let max_wordlength = signal(15 as u8);

    let word_input_element: NodeRef<html::Input> = NodeRef::new();
    let append_word_list = move |ev: SubmitEvent| {
        ev.prevent_default();

        let mut input_word = word_input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        if current_word_list.get().len() == 0 {
            let length = input_word.len() as u8;

            min_wordlength.1.set(length);
            max_wordlength.1.set(length);
        }        
        set_current_wordlist.write().insert(input_word);
        
        
        word_input_element.get().unwrap().set_value("");
        
    };

    let complete_wordlist = move || {
        for word in current_word_list.get() {
            set_remaining_wordlist.write().insert(word);
        }
    };

    let guess_input_element: NodeRef<html::Input> = NodeRef::new();
    let correct_input_element: NodeRef<html::Input> = NodeRef::new();
    let shrink_list = move |ev: SubmitEvent| {
        ev.prevent_default();

        let guess_word = guess_input_element
            .get()
            .expect("<input id=\"guess_input\"> should be mounted")
            .value();
        let amount_correct: String = correct_input_element
            .get()
            .expect("<input id=\"correct_input\"> should be mounted")
            .value();
        //html ensures it's a number
        let amount_correct_integer : u8 = amount_correct.parse::<u8>().unwrap();

        set_remaining_wordlist.set(check_word_against_list(remaining_wordlist.get(),guess_word,amount_correct_integer)) ;
        let value = "";
        guess_input_element.get().unwrap().set_value(value);
        correct_input_element.get().unwrap().set_value(value);        
    };

    let reset = move ||{
        set_remaining_wordlist.set(HashSet::new());
        set_current_wordlist.set(HashSet::new());
        let value = "";
        word_input_element.get().unwrap().set_value(value);
        guess_input_element.get().unwrap().set_value(value);
        correct_input_element.get().unwrap().set_value(value);
    };

    view! {
    <Header render_prop=|| view! { <h1>"Fallout Hacking Assistant"</h1>  }>
        <ToggleButton setter=set_toggled id="btnHelp".to_string() text="HELP".to_string()> </ToggleButton>
    </Header>
    <section>
        <Fieldset render_prop=|| view! { <legend>Input</legend>  } id="section1".to_string()>
            <p id="number_label">"4-15 characters"</p>
            <form id="formWordInput"  on:submit=append_word_list>
                <input type="text" id="wordInput" placeholder="Word input" minlength= move || min_wordlength.0.get() maxlength = move || max_wordlength.0.get() node_ref=word_input_element/>

            </form>
            <p id="list_label">Current words:</p>
            <UnorderedList wordlist=current_word_list/>

            <Button on_click=move |_| complete_wordlist() id= "btnFinished".to_string() text="FINISHED".to_string()/>
        </Fieldset>

        <Fieldset render_prop=|| view! { <legend>Guessing</legend>  } id="section2".to_string()>
            <form id="formGuess".to_string() on:submit=shrink_list>

                <input type="text" id="guess_input".to_string() placeholder="Current guess".to_string()
                minlength= move || max_wordlength.0.get() maxlength= move || max_wordlength.0.get()
                node_ref=guess_input_element/>

                <input type="number" id="correct_input".to_string() min=0 max= move || max_wordlength.0.get() placeholder = 0 node_ref=correct_input_element/>

                <p id="correct_label">"/ 15 correct."</p>

                <button on:submit=shrink_list id= "btnSubmit" >"SUBMIT"</button>

            </form>
            <p id="remaining_label">Remaining words:</p>
            <UnorderedList wordlist=remaining_wordlist/>
            <Button on_click=move |_| reset() id= "btnRestart".to_string() text="RESTART".to_string()/>
        </Fieldset>
    </section>
    <article id="articleHelp">
        <p>
        {help_text}
         </p>
    </article>
    }
}
fn do_nothing() {}
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
pub fn Button(
    on_click: impl FnMut(MouseEvent) + 'static,
    id: String,
    text: String,
) -> impl IntoView {
    view! {
        <button on:click=on_click id=id>
            {text}
        </button>
    }
}

#[component]
pub fn Fieldset<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` can take one of several different types, each of which
    /// is a function that returns some view type
    children: Children,
    id: String,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <fieldset id=id>
        {render_prop()}
        {children()}
        </fieldset>
    }
}

#[component]
pub fn NumberInput(id: String, min: u8, max: u8) -> impl IntoView {
    view! {
        <input id=id class="number" type="number" placeholder="0" min=min max=max maxlength=2/>
    }
}



#[component]
pub fn UnorderedList(wordlist: ReadSignal<HashSet<String>>) -> impl IntoView {
    view! {
        <ul class = "list">
        <For
            each = move||wordlist.get()
            key = |item| item.clone()
            children=move |item|view! {<li>{item}</li>}
        />

        </ul>
    }
}

//todo input length should change min and max for word input
//word input should fill the contents of the list on enter it should also fill in the length of the word in the number input
//
