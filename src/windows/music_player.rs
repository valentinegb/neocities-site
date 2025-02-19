use std::{io::BufReader, ops::Deref, sync::Arc, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use eframe::CreationContext;
use egui::{
    cache::{ComputerMut, FrameCache},
    mutex::Mutex,
    Align2, Image, InnerResponse, ProgressBar, RichText,
};
use ehttp::fetch;
use log::error;
use rand::{rng, seq::SliceRandom};
use rodio::{OutputStream, Source};

use crate::fonts::RichTextExt;

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

pub struct MusicPlayerWindow<'a> {
    queue: Vec<&'a str>,
    stream: Option<OutputStream>,
    sink: Option<rodio::Sink>,
    data: Vec<Arc<Mutex<Option<Vec<u8>>>>>,
    data_appended: Vec<bool>,
    prev_sink_len: usize,
    position: usize,
    origin: String,
    total_durations: Vec<Option<Duration>>,
    is_mobile: bool,
}

impl<'a> MusicPlayerWindow<'a> {
    pub fn new(cc: &CreationContext<'_>) -> anyhow::Result<MusicPlayerWindow<'a>> {
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
                ehttp::Request::get(format!("/assets/music/{song}/audio.txt")),
                {
                    let data = data.clone();

                    move |response| match response {
                        Ok(response) => match BASE64_STANDARD.decode(response.bytes) {
                            Ok(bytes) => *data[index].lock() = Some(bytes),
                            Err(err) => error!("Could not decode audio for song {song:?}: {err}"),
                        },
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
            origin: cc.integration_info.web_info.location.origin.clone(),
            total_durations: vec![None; data.len()],
            is_mobile: cc.integration_info.web_info.user_agent.contains("Mobile"),
        })
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<InnerResponse<Option<()>>> {
        if let Some(sink) = self.sink.as_ref() {
            if sink.len() != self.prev_sink_len {
                self.position += 1;

                if self.position >= self.queue.len() {
                    self.position -= self.queue.len();
                    self.data_appended = vec![false; self.data.len()];
                }

                self.prev_sink_len = sink.len();
            }

            if self.data_appended.contains(&false) {
                for (i, data) in self.data.clone().into_iter().enumerate() {
                    if self.data_appended[i] {
                        continue;
                    } else if let Some(data) = data.lock().deref() {
                        let reader = BufReader::new(std::io::Cursor::new(data.clone()));
                        let source = rodio::Decoder::new(reader);

                        match source {
                            Ok(source) => {
                                self.total_durations[i] = source.total_duration();

                                sink.append(source);

                                self.data_appended[i] = true;
                                self.prev_sink_len = sink.len();
                            }
                            Err(err) => {
                                error!("Failed to decode audio data: {err}");

                                return None;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        egui::Window::new("🎶 Music Player")
            .max_width(200.0)
            .pivot(Align2::RIGHT_TOP)
            .default_pos((ctx.screen_rect().width() - 20.0, 45.0))
            .default_open(!self.is_mobile)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let current_song = self.queue[self.position];

                    ui.add(
                        Image::new(format!(
                            "{}/assets/music/{current_song}/artwork.jpg",
                            self.origin,
                        ))
                        .corner_radius(3),
                    );

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

                    ui.add(
                        egui::Label::new(RichText::new(song_info.name.clone()).proportional_bold())
                            .truncate(),
                    );
                    ui.add(
                        egui::Label::new(format!("{} — {}", song_info.artist, song_info.album))
                            .truncate(),
                    );
                    ui.add(
                        ProgressBar::new(
                            self.sink
                                .as_ref()
                                .and_then(|sink| {
                                    self.total_durations[self.position].map(|total_duration| {
                                        sink.get_pos().div_duration_f32(total_duration)
                                    })
                                })
                                .unwrap_or(0.0),
                        )
                        .desired_height(5.0),
                    );
                    ctx.request_repaint();
                    ui.columns_const(|[col_1, col_2]| {
                        col_1.vertical_centered_justified(|ui| {
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
                        col_2.vertical_centered_justified(|ui| {
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
