use super::session_gate::use_session;
use crate::hooks::use_memo::use_memo;
use std::rc::Rc;
use surf::Client;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[derive(Clone, Debug)]
struct HttpClient {
    client: Rc<Client>,
}

impl PartialEq for HttpClient {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.client, &other.client)
    }
}

#[function_component(HttpClientProvider)]
pub fn http_client_provider(props: &Props) -> Html {
    let session = use_session();
    let client = use_memo(
        |session| {
            surf::Config::new()
                .set_base_url("http://localhost:8080/".parse().unwrap())
                .add_header("Authorization", format!("Bearer {}", session.name))
                .unwrap()
                .try_into()
                .unwrap()
        },
        session,
    );

    html! {
        <ContextProvider<HttpClient> context={HttpClient { client }}>
            {for props.children.iter()}
        </ContextProvider<HttpClient>>
    }
}

pub fn use_http_client() -> Rc<Client> {
    use_context::<HttpClient>().unwrap().client
}
