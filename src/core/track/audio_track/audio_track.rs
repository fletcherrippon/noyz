use jack;
use nanoid::nanoid;

pub struct AudioTrack {
    id: String,
    name: String,
    file_path: Option<String>,
}

impl Default for AudioTrack {
    fn default() -> Self {
        AudioTrack {
            id: nanoid!(),
            name: "Audio Track".to_string(),
            file_path: None,
        }
    }
}

impl AudioTrack {
    fn new<N: Option<&str>, P: Option<&str>>(name: N, path: P) -> Self {
        if (!path) {
            // Set path to name
            path = None;
        }

        AudioTrack {
            id: nanoid!(),
            name: name.to_string(),
            file_path: path.to_string(),
        }
    }
}
