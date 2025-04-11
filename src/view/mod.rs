use yew::prelude::*;
pub enum WgslDocMsg {
    AddOne,
}

pub struct WgslDocApp {
    value: i64,
}

impl Component for WgslDocApp {
    type Message = WgslDocMsg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WgslDocMsg::AddOne => {
                self.value += 1;
                true
            },
            _ => false,
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| WgslDocMsg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}