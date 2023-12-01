use linera_sdk::views::{RegisterView, ViewStorageContext};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Activity {
    pub value: RegisterView<u64>,
    // Add fields here.
}
