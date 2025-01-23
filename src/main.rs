mod utils;
mod fast_exponentiation;
mod inverse_modular;
mod prime_gen;
#[cfg(test)]
mod tests;

use crate::fast_exponentiation::exponential_fast_mod;
use crate::inverse_modular::inverse_modular_fast;
use crate::prime_gen::prime_gen_probably_and_coprime;
use iced::widget::container::Style;
use iced::widget::{
    button, column, container, horizontal_space, row, text, text_input, vertical_space as spacer,
    Column,
};
use iced::{Border, Center, Color, Fill, Pixels, Shadow, Size, Task, Theme};
use num_format::{Buffer, CustomFormat, Grouping, ToFormattedStr};
use num_traits::ToPrimitive;
use rand::{thread_rng, Rng};
use std::u128;

struct App {
    p: u64,                  // bob prime number
    q: u64,                  // bob prime number
    phi_n: u128,             // bob modulo
    e: u128,                 // alice public key
    d: u128,                 // bob private key
    n: u128,                 // bob public key
    message: u128,           // alice message
    encrypted_message: u128, // alice encrypted message
    decrypted_message: u128, // bob decrypted message
    range_min: u32,          // range prime gen
    range_max: u32,          // range prime gen,
    progress_d: bool,
    progress_decrypt: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            p: 0,
            q: 0,
            phi_n: 0,
            e: 0,
            d: 0,
            n: 0,
            message: 0,
            encrypted_message: 0,
            decrypted_message: 0,
            range_min: 2,
            range_max: u32::MAX,
            progress_d: false,
            progress_decrypt: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GenP,
    GenQ,
    GenE,
    CalculateD,
    CalculateDFinished(u128),
    RangeMin(String),
    RangeMax(String),
    Message(String),
    Encrypt,
    Decrypt,
    DecryptedMessage(u128),
    FakeIt,
}

fn main() -> iced::Result {
    iced::application("RSA", App::update, App::view)
        .transparent(true)
        .window_size(Size::new(1000.0, 800.0))
        .theme(App::theme)
        .run()
}

impl App {
    const SEPARATOR: &'static str = "  ";

    // format a number type T
    fn format<T>(&self, number: &T) -> String
    where
        T: ToPrimitive + ToFormattedStr,
    {
        let format = CustomFormat::builder()
            .grouping(Grouping::Standard)
            .separator(Self::SEPARATOR)
            .build()
            .expect("Invalid format");

        let mut buf: Buffer = Buffer::new();
        buf.write_formatted(number, &format);
        buf.as_str().to_string()
    }

    pub fn view(&self) -> Column<Message> {
        column![
            container(text("RSA").size(50)).center_x(Fill),
            spacer().height(Pixels(20.0)),
            row![
                container(column![
                    container(text("Alice").size(20)).center_x(Fill).width(Fill),
                    container(
                        column![
                            row![
                                container(text("message : ").size(20)).align_y(Center),
                                container(
                                    text_input("message", &self.message.to_string())
                                        .on_input(Message::Message)
                                        .size(20)
                                )
                                .width(Fill)
                                .align_y(Center),
                                container(button("encrypt").on_press(Message::Encrypt))
                                    .align_y(Center),
                            ]
                            .spacing(20),
                            row![
                                container(text("encrypted message : ").size(20)).align_y(Center),
                                container(text(self.format(&self.encrypted_message)).size(20))
                                    .width(Fill)
                                    .align_y(Center),
                                container(button("fake it").on_press(Message::FakeIt)),
                            ]
                            .spacing(20),
                        ]
                        .spacing(20)
                    )
                    .center_y(Fill)
                    .width(Fill)
                    .padding(10)
                ],)
                .center_x(Fill)
                .width(Fill)
                .height(Fill)
                .padding(10)
                .style(|_theme| Style {
                    background: Option::default(),
                    text_color: Option::from(Color::WHITE),
                    border: Border::default()
                        .rounded(10)
                        .color(Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        })
                        .width(1.0),
                    shadow: Shadow::default(),
                }), // Pass a closure here,
                horizontal_space().width(Pixels(20.0)),
                container(column![
                    container(text("Bob").size(20)).center_x(Fill).width(Fill),
                    container(
                        column![
                            row![
                                container(text("range prime gen : ").size(20)).align_y(Center),
                                container(
                                    text_input("start", &self.range_min.to_string())
                                        .on_input(Message::RangeMin)
                                        .size(20)
                                )
                                .width(Fill)
                                .align_y(Center),
                                container(
                                    text_input("end", &self.range_max.to_string())
                                        .on_input(Message::RangeMax)
                                        .size(20)
                                )
                                .width(Fill)
                                .align_y(Center),
                            ]
                            .spacing(20),
                            row![
                                container(text("p : ").size(20)),
                                container(text(self.format(&self.p)).size(20)).width(Fill),
                                container(button("generate").on_press(Message::GenP)),
                            ]
                            .spacing(20),
                            row![
                                container(text("q : ").size(20)),
                                container(text(self.format(&self.q)).size(20)).width(Fill),
                                container(button("generate").on_press(Message::GenQ)),
                            ]
                            .spacing(20),
                            row![
                                container(text("(p-1)(q-1) : ").size(20)),
                                container(text(self.format(&self.phi_n)).size(20)).width(Fill),
                            ]
                            .spacing(20),
                            row![
                                container(text("e : ").size(20)),
                                container(text(self.format(&self.e)).size(20)).width(Fill),
                                container(button("generate").on_press(Message::GenE)),
                            ]
                            .spacing(20),
                            row![
                                container(text("d : ").size(20)),
                                container(text(self.format(&self.d)).size(20)).width(Fill),
                                container(
                                    button(if self.progress_d {
                                        "calculating..."
                                    } else {
                                        "calculate"
                                    })
                                    .on_press_maybe(
                                        if self.progress_d {
                                            None
                                        } else {
                                            Some(Message::CalculateD)
                                        }
                                    )
                                ),
                            ]
                            .spacing(20),
                            row![
                                container(text("decrypted message : ").size(20)),
                                container(text(self.format(&self.decrypted_message)).size(20))
                                    .width(Fill),
                                container(
                                    button(if self.progress_decrypt {
                                        "decrypting..."
                                    } else {
                                        "decrypt"
                                    })
                                    .on_press_maybe(
                                        if self.progress_decrypt {
                                            None
                                        } else {
                                            Some(Message::Decrypt)
                                        }
                                    )
                                ),
                            ]
                            .spacing(20),
                        ]
                        .spacing(20)
                    )
                    .center_y(Fill)
                    .width(Fill)
                    .padding(10),
                    container(if self.decrypted_message != 0 {
                        if self.decrypted_message != self.message {
                            text("Decryption failed".to_string())
                                .color(Color::new(1.0, 0.0, 0.0, 1.0))
                        } else {
                            text("Decryption success".to_string())
                                .color(Color::new(0.0, 1.0, 0.0, 1.0))
                        }
                    } else {
                        text(String::new())
                    })
                ],)
                .center_x(Fill)
                .width(Fill)
                .height(Fill)
                .padding(10)
                .style(|_theme| Style {
                    background: Option::default(),
                    text_color: Option::from(Color::WHITE),
                    border: Border::default()
                        .rounded(10)
                        .color(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 1.0,
                            a: 1.0,
                        })
                        .width(1.0),
                    shadow: Shadow::default(),
                }), // Pass a closure here,
            ],
            spacer().height(Pixels(20.0)),
            container(column![
                container(text("Public Infos").size(20))
                    .center_x(Fill)
                    .width(Fill),
                container(
                    column![
                        row![
                            container(text("e : ").size(20)),
                            container(text(self.format(&self.e)).size(20)),
                        ]
                        .spacing(20),
                        row![
                            container(text("n : ").size(20)),
                            container(text(self.format(&self.n)).size(20)),
                        ]
                        .spacing(20),
                        row![
                            container(text("encrypted message : ").size(20)),
                            container(text(self.format(&self.encrypted_message)).size(20)),
                        ]
                        .spacing(20),
                    ]
                    .spacing(20),
                )
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill),
            ])
            .height(Pixels(200.0))
            .width(Fill)
            .padding(10)
            .style(|_theme| Style {
                background: Option::default(),
                text_color: Option::from(Color::WHITE),
                border: Border::default()
                    .rounded(10)
                    .color(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    })
                    .width(1.0),
                shadow: Shadow::default(),
            }), // Pass a closure here
        ]
            .padding(20)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GenP => {
                self.p = prime_gen::prime_gen(u64::from(self.range_min), u64::from(self.range_max));
                self.calculate_n();
            }
            Message::GenQ => {
                self.q = prime_gen::prime_gen(u64::from(self.range_min), u64::from(self.range_max));
                self.calculate_n();
            }
            Message::GenE => {
                if self.q == self.p && self.q == 0 {
                    return Task::none();
                }
                self.e = prime_gen_probably_and_coprime(2, u64::MAX, self.phi_n)
            }
            Message::CalculateD => {
                if self.e == 0 || self.phi_n == 0 {
                    return Task::none();
                }
                let e: u128 = self.e;
                let phi_n: u128 = self.phi_n;
                self.progress_d = true;
                return Task::future(async move {
                    let d: u128 = tokio::task::spawn_blocking(move || {
                        println!("Calculating d...");
                        let res: Option<u128> = inverse_modular_fast(e, phi_n);
                        match res {
                            Some(d) => d,
                            None => panic!("No modular inverse found"),
                        }
                    })
                        .await
                        .unwrap();

                    // Send the information back to the update function
                    Message::CalculateDFinished(d)
                });
            }
            Message::RangeMin(range) => {
                let range: String = range.replace(Self::SEPARATOR, "");
                let nb: u32 = range.parse().unwrap_or(2);
                if nb < 2 || nb >= self.range_max {
                    return Task::none();
                }
                self.range_min = nb;
            }
            Message::RangeMax(range) => {
                let range: String = range.replace(Self::SEPARATOR, "");
                let nb: u32 = range.parse().unwrap_or(u32::MAX);
                if nb < 2 || nb <= self.range_min {
                    return Task::none();
                }
                self.range_max = nb;
            }
            Message::Message(msg) => {
                let msg: String = msg.replace(Self::SEPARATOR, "");
                let nb: u128 = msg.parse().unwrap_or(0);
                if nb >= self.n {
                    return Task::none();
                }
                self.message = nb;
            }
            Message::Encrypt => {
                self.encrypted_message =
                    exponential_fast_mod(self.message, self.e, self.n);
                return Task::none();
            }
            Message::Decrypt => {
                let encrypted_message: u128 = self.encrypted_message.clone();
                let d: u128 = self.d;
                let n: u128 = self.n.clone();
                self.progress_decrypt = true;

                return Task::future(async move {
                    let information: u128 = tokio::task::spawn_blocking(move || {
                        println!("Decrypting message...");
                        exponential_fast_mod(encrypted_message, d, n)
                    })
                        .await
                        .unwrap();

                    // Send the information back to the update function
                    Message::DecryptedMessage(information)
                });
            }
            Message::DecryptedMessage(msg) => {
                self.decrypted_message = msg;
                self.progress_decrypt = false;
                return Task::none();
            }
            Message::CalculateDFinished(d) => {
                self.d = d;
                self.progress_d = false;
                return Task::none();
            }
            Message::FakeIt => {
                self.encrypted_message += u128::from(thread_rng().gen_range(1..u32::MAX));
            }
        }
        Task::none()
    }

    fn calculate_n(&mut self) {
        self.n = u128::from(self.p) * u128::from(self.q);
        if self.p > 0 && self.q > 0 {
            self.phi_n = (u128::from(self.p) - 1) * (u128::from(self.q) - 1);
        }
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
}
