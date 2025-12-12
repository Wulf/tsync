#[tsync]
struct TestThingey {
    #[serde(rename = "im_correct   ")]
    not_me: String,
    how_bout_me: String,
    #[serde(skip)]
    skip_me: SuperSecretStructThatsInternal,
}
