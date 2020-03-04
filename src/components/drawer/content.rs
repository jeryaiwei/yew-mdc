use yew::prelude::*;

pub struct Content {
    id: String,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
}

impl Component for Content {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("drawer-content-{}", crate::next_id()));
        Self { id, props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-drawer__content" id=self.id>
                { self.props.children.render() }
            </div>
        }
    }
}
