use yew::prelude::*;

enum Msg {
    AddMultipliedAmount,
    BuyMulitplier,
}

struct Model {
    value: usize,
    base_amount: usize,
    cost: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            base_amount: 1,
            cost: 10,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddMultipliedAmount => {
                self.value += self.base_amount;
                true
            }
            Msg::BuyMulitplier => {
                self.base_amount *= 2;
                self.value -= self.cost;
                self.cost += 20;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <p>{ "Numbers r fun! Click to get more numbers and buy a multiplier to make numbers go up faster!" }</p>
                <p>{"Total: "}{ self.value }</p>
                <button onclick={link.callback(|_| Msg::AddMultipliedAmount)}>{ "+ " }{self.base_amount}</button>
                <button onclick={link.callback(|_| Msg::BuyMulitplier)} disabled={self.value < self.cost}>{ "Buy Multiplier: " }{self.cost}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
