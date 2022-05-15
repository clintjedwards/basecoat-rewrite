use yew::prelude::*;

const BUILD_SEMVER: &str = env!("FRONTEND_BUILD_SEMVER");
const BUILD_COMMIT: &str = env!("FRONTEND_BUILD_COMMIT");

pub struct Footer {
    version: String,
    commit: String,
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            version: BUILD_SEMVER.to_string(),
            commit: BUILD_COMMIT.to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut short_hash = self.commit.clone();
        short_hash.truncate(6);

        html! {
            <div>
                <hr/>
                <div>{format!("Version {} | Commit {}", self.version, short_hash)}</div>
            </div>
        }
    }
}
