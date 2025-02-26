#[derive(Serialize, Deserialize)]
#[tsync]
pub struct CameraControl {
    pub camera_uuid: String,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "action", content = "json")]
#[tsync]
pub enum Action {
    GetVideoParameterSettings(VideoParameterSettings),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[tsync]
pub struct VideoParameterSettings {
    pub frame_rate: Option<u16>,
}
