use yew::prelude::*;

fn main() {
    yew::start_app::<Counter>();
}

#[derive(PartialEq, Properties)]
struct Counter {
    count: i64,
}

enum CounterMessage {
    Add,
    Sub,
}

impl Component for Counter {
    type Message = CounterMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Counter { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CounterMessage::Add => self.count += 1,
            CounterMessage::Sub => {
                if self.count >= 1 {
                    self.count -= 1
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="d-flex align-items-center justify-content-center min-100vw bg-secondary" style="min-height: 100vh">
                <div class="card shadow">
                    <div class="d-flex align-items-center justify-content-center card-body">
                        <button onclick={link.callback(|_| CounterMessage::Add)} class="btn btn-primary">{"+"}</button>
                        <span class="font-bold mx-3">{&format!(" {} ", self.count)}</span>
                        <button onclick={link.callback(|_| CounterMessage::Sub)} class="btn btn-danger">{"-"}</button>
                    </div>
                </div>
            </div>
        }
    }
}
