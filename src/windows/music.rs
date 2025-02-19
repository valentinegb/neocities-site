use std::{io::BufReader, ops::Deref, sync::Arc};

use egui::{
    cache::{ComputerMut, FrameCache},
    mutex::Mutex,
    Align, Align2, InnerResponse,
};
use ehttp::fetch;
use log::{debug, error};
use rand::{rng, seq::SliceRandom};
use rodio::OutputStream;

use crate::fonts::UiExt as _;

#[derive(Default)]
struct SongInfo {
    name: String,
    album: String,
    artist: String,
}

impl ComputerMut<&str, Arc<Mutex<Option<SongInfo>>>> for SongInfo {
    fn compute(&mut self, key: &str) -> Arc<Mutex<Option<SongInfo>>> {
        let song_info = Arc::new(Mutex::new(None));

        fetch(
            ehttp::Request::get(format!("/assets/music/{key}/info.txt")),
            {
                let key = key.to_string();
                let song_info = song_info.clone();

                move |response| match response {
                    Ok(response) => {
                        match response.text() {
                            Some(text) => {
                                let mut lines = text.lines();

                                match lines.next() {
                                Some(name) => match lines.next() {
                                    Some(album) => match lines.next() {
                                        Some(artist) => {
                                            *song_info.lock() = Some(SongInfo {
                                                name: name.to_string(),
                                                album: album.to_string(),
                                                artist: artist.to_string(),
                                            });
                                        }
                                        None => error!("Song info response for {key:?} did not have artist"),
                                    },
                                    None => error!("Song info response for {key:?} did not have album"),
                                },
                                None => error!("Song info response for {key:?} did not have name"),
                            }
                            }
                            None => error!("Song info response for {key:?} did not have text"),
                        }
                    }
                    Err(err) => error!("Failed to fetch song info for {key:?}: {err}"),
                }
            },
        );

        song_info
    }
}

pub struct MusicWindow<'a> {
    queue: Vec<&'a str>,
    stream: Option<OutputStream>,
    sink: Option<rodio::Sink>,
    data: Vec<Arc<Mutex<Option<Vec<u8>>>>>,
    data_appended: Vec<bool>,
    prev_sink_len: usize,
    position: usize,
    origin: String,
}

impl<'a> MusicWindow<'a> {
    pub fn new(origin: String) -> anyhow::Result<MusicWindow<'a>> {
        let mut queue = vec![
            "20190724",
            "about-10-hours-of-making-breakcore",
            "felt-good",
            "ghost-choir",
            "ghost-duet",
            "harlequin",
            "i-might-b3-sick",
            "lovesick-cannibal",
            "oh-its-you",
            "ruffneck-killa",
        ];
        let mut rng = rng();

        queue.shuffle(&mut rng);

        let mut data = Vec::with_capacity(queue.len());

        for _ in 0..data.capacity() {
            data.push(Arc::new(Mutex::new(None)));
        }

        for (index, song) in queue.clone().into_iter().enumerate() {
            fetch(
                ehttp::Request::get(format!("/assets/music/{song}/audio")),
                {
                    let data = data.clone();

                    move |response| match response {
                        Ok(response) => *data[index].lock() = Some(response.bytes),
                        Err(err) => error!("Failed to fetch audio for song {song:?}: {err}"),
                    }
                },
            );
        }

        Ok(Self {
            queue,
            stream: None,
            sink: None,
            data: data.clone(),
            data_appended: vec![false; data.len()],
            prev_sink_len: 0,
            position: 0,
            origin,
        })
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<InnerResponse<Option<()>>> {
        if let Some(sink) = self.sink.as_ref() {
            if sink.len() != self.prev_sink_len {
                debug!("Sink length changed!");

                self.position += 1;

                if self.position >= self.queue.len() {
                    self.position -= self.queue.len();
                    self.data_appended = vec![false; self.data.len()];

                    debug!("Reached end of queue, signaled to re-append queue");
                }

                self.prev_sink_len = sink.len();
            }

            if self.data_appended.contains(&false) {
                debug!("There is data that needs to be appended");

                for (i, data) in self.data.clone().into_iter().enumerate() {
                    if self.data_appended[i] {
                        debug!("Data already appended for index {i}");

                        continue;
                    } else if let Some(data) = data.lock().deref() {
                        let reader = BufReader::new(std::io::Cursor::new(data.clone()));
                        let source = rodio::Decoder::new(reader);

                        match source {
                            Ok(source) => {
                                sink.append(source);

                                self.data_appended[i] = true;
                                self.prev_sink_len = sink.len();

                                debug!("Appended data with index of {i}");
                            }
                            Err(err) => {
                                error!("Failed to decode audio data: {err}");

                                return None;
                            }
                        }
                    } else {
                        debug!("Data not ready for index {i}");

                        break;
                    }
                }
            }
        }

        egui::Window::new("Music")
            .default_width(200.0)
            .max_width(600.0)
            .pivot(Align2::RIGHT_TOP)
            .default_pos((ctx.screen_rect().width() - 20.0, 45.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let current_song = self.queue[self.position];

                    ui.image(format!(
                        "{}/assets/music/{current_song}/artwork.jpg",
                        self.origin,
                    ));

                    let default_song_info = SongInfo {
                        name: "Unknown Song".to_string(),
                        album: "Unknown Album".to_string(),
                        artist: "Unknown Artist".to_string(),
                    };
                    let song_info = ui.memory_mut(|memory| {
                        memory
                            .caches
                            .cache::<FrameCache<Arc<Mutex<Option<SongInfo>>>, SongInfo>>()
                            .get(current_song)
                    });
                    let song_info = song_info.lock();
                    let song_info = song_info.as_ref().unwrap_or(&default_song_info);

                    ui.proportional_bold(song_info.name.clone());
                    ui.add(
                        egui::Label::new(format!("{} — {}", song_info.artist, song_info.album))
                            .truncate(),
                    );

                    ui.columns_const(|[col_1, col_2]| {
                        col_1.with_layout(egui::Layout::top_down(Align::RIGHT), |ui| {
                            if self.sink.as_ref().map_or(true, |sink| sink.is_paused()) {
                                if ui.button("⏵").clicked() {
                                    match self.sink.as_ref() {
                                        Some(sink) => sink.play(),
                                        None => {
                                            if let Ok((stream, stream_handle)) =
                                                OutputStream::try_default()
                                            {
                                                self.stream = Some(stream);

                                                if let Ok(sink) =
                                                    rodio::Sink::try_new(&stream_handle)
                                                {
                                                    self.sink = Some(sink);
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if ui.button("⏸").clicked() {
                                if let Some(sink) = self.sink.as_ref() {
                                    sink.pause();
                                }
                            }
                        });
                        col_2.with_layout(egui::Layout::top_down(Align::LEFT), |ui| {
                            if self.sink.is_none() {
                                ui.disable();
                            }

                            if ui.button("⏩").clicked() {
                                if let Some(sink) = self.sink.as_ref() {
                                    sink.skip_one();
                                }
                            }
                        });
                    });
                });
            })
    }
}
