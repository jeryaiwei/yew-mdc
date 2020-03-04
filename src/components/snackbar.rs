use crate::mdc_sys::MDCSnackbar;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use crate::components::Button;

pub struct Snackbar {
    id: String,
    link: ComponentLink<Self>,
    inner: Option<MDCSnackbar>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub text: String,
    pub action_text: String,
    pub onactionclicked: Option<Callback<()>>,
    pub onclose: Option<Callback<()>>,
    pub timeout_ms: Option<u16>,
    #[props(required)]
    pub open: bool,
}

pub enum Msg {
    ActionClicked,
    Closed,
}

impl Component for Snackbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("snackbar-{}", crate::next_id()));
        let close_callback = {
            let callback = link.callback(|_| Msg::Closed);
            Closure::wrap(Box::new(move |e: web_sys::Event| {
                e.stop_propagation();
                callback.emit(());
            }) as Box<dyn FnMut(web_sys::Event)>)
        };
        Self {
            id,
            link,
            inner: None,
            close_callback,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(elem) = crate::get_element_by_id(&self.id) {
            let inner = MDCSnackbar::new(elem);
            if let Some(timeout_ms) = &self.props.timeout_ms {
                inner.set_timeout_ms(*timeout_ms);
            }
            if self.props.open {
                inner.open();
            }
            inner.listen("MDCSnackbar:closed", &self.close_callback);
            self.inner = Some(inner);
        }
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            if let Some(ref inner) = self.inner {
                if self.props.open {
                    inner.open();
                } else {
                    inner.close(None);
                }
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ActionClicked => {
                if let Some(ref callback) = self.props.onactionclicked {
                    callback.emit(());
                }
            }
            Msg::Closed => {
                if let Some(ref callback) = self.props.onclose {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let actions = if !self.props.action_text.is_empty() {
            let emit_action = Some(self.link.callback(|_| Msg::ActionClicked));
            html! {
                <div class="mdc-snackbar__actions">
                    <Button text=&self.props.action_text
                            classes="mdc-snackbar__action"
                            onclick=emit_action
                            />
                </div>
            }
        } else {
            html! {}
        };
        html! {
            <div class="mdc-snackbar" id=&self.id>
                <div class="mdc-snackbar__surface">
                    <div class="mdc-snackbar__label">
                        { &self.props.text }
                    </div>
                    { actions }
                </div>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.unlisten("MDCSnackbar:closed", &self.close_callback);
            inner.destroy();
        }
    }
}
