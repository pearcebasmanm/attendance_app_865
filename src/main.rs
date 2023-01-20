use leptos::*;
mod counter;
mod gsheets;
use counter::{Counter, CounterProps};
mod name_input;
use google_sheets4 as sheets4;
use name_input::{NameInput, NameInputProps};
use sheets4::{
    hyper::{client::connect::HttpConnector, *},
    hyper_rustls::*,
    *,
};
fn main() {
    leptos::mount_to_body(attendance_app);
    async {
        
        
    let mut hub = google_sheets_setup().await;
    hub.spreadsheets().get();
    }
}

fn attendance_app(cx: Scope) -> Element {
    view! {
        cx,
        <div>
        <h1>"Attendance App"</h1>
        <Counter/>
        <NameInput/>
        </div>
    }
}

async fn google_sheets_setup() -> Sheets<HttpsConnector<HttpConnector>> {
    let client = Client::builder().build(
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build(),
    );
    let authenticator = oauth2::InstalledFlowAuthenticator::builder(
        Default::default(),
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    Sheets::new(client, authenticator)
