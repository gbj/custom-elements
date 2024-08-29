use yew::prelude::*;

pub enum Msg {
    AddOne,
    Set(i64),
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::Set(value) => {
                self.value = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::AddOne);
        html! {
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
