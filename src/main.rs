mod utils;

mod fast_exponentiation;
mod inverse_modular;
mod prime_gen;
#[cfg(test)]
mod tests;

use crate::fast_exponentiation::{fast_exponentiation, parallel_fast_exponentiation};
use crate::utils::are_coprime;
use iced::widget::container::Style;
use iced::widget::{button, column, container, horizontal_space, row, text, text_input, vertical_space as spacer, Column};
use iced::{Border, Center, Color, Fill, Pixels, Shadow, Size, Task, Theme};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::ops::{Rem, RemAssign};
use std::u128;

struct App {
    p: u64, // bob prime number
    q: u64, // bob prime number
    phi_n: u128, // bob modulo
    e: u128, // alice public key
    d: u128, // bob private key
    n: u128, // bob public key
    message: u64, // alice message
    encrypted_message: u64, // alice encrypted message
    decrypted_message: u64, // bob decrypted message
    range_min: u16, // range prime gen
    range_max: u16, // range prime gen,
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
            range_max: i16::MAX as u16,
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
    DecryptedMessage(u64),
}

fn main() -> iced::Result {

    // Initialiser le runtime tokio
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = runtime.enter();

    iced::application("RSA", App::update, App::view)
        .transparent(true)
        .window_size(Size::new(800.0, 600.0))
        // .theme(App::theme)
        .run()
}


impl App {
    pub fn view(&self) -> Column<Message> {
        column![
            container( text("RSA").size(50)).center_x(Fill),
            spacer().height(Pixels(20.0)),
            row![
                container(
                    column![

                        container(text("Alice").size(20)).center_x(Fill).width(Fill),

                        container(
                            column![

                                row![
                                    container(text("message : ").size(20)).align_y(Center),
                                    container(text_input("message", &*self.message.to_string()).on_input(Message::Message).size(20)).width(Fill).align_y(Center),
                                    container(button("encrypt").on_press(Message::Encrypt)).align_y(Center),
                                ].spacing(20),

                                row![
                                    container(text("encrypted message : ").size(20)).align_y(Center),
                                    container(text(&self.encrypted_message).size(20)).width(Fill).align_y(Center),
                                ].spacing(20),

                            ].spacing(20)
                        ).center_y(Fill).width(Fill).padding(10)

                    ],
                ).center_x(Fill).width(Fill).height(Fill).padding(10)
                .style(|_theme| Style {
                background: Default::default(),
                text_color: Option::from(Color::WHITE),
                border: Border::default().rounded(10).color(Color{r: 1.0 , g: 0.0 , b: 0.0 , a: 1.0 , }).width(1.0),
                shadow: Shadow::default(),}), // Pass a closure here,

                horizontal_space().width(Pixels(20.0)),

                container(
                    column![

                        container(text("Bob").size(20)).center_x(Fill).width(Fill),

                        container(
                            column![

                                  row![
                                    container(text("range prime gen : ").size(20)).align_y(Center),
                                    container(text_input("start", &*self.range_min.to_string()).on_input(Message::RangeMin).size(20)).width(Fill).align_y(Center),
                                    container(text_input("end", &*self.range_max.to_string()).on_input(Message::RangeMax).size(20)).width(Fill).align_y(Center),
                                ].spacing(20),

                                row![
                                    container(text("p : ").size(20)),
                                    container(text(self.p.to_string()).size(20)).width(Fill),
                                    container(button("generate").on_press(Message::GenP)),
                                ].spacing(20),

                                row![
                                    container(text("q : ").size(20)),
                                    container(text(self.q.to_string()).size(20)).width(Fill),
                                    container(button("generate").on_press(Message::GenQ)),
                                ].spacing(20),

                                   row![
                                    container(text("(p-1)(q-1) : ").size(20)),
                                    container(text(self.phi_n.to_string()).size(20)).width(Fill),
                                ].spacing(20),

                                row![
                                    container(text("e : ").size(20)),
                                    container(text(self.e.to_string()).size(20)).width(Fill),
                                    container(button("generate").on_press(Message::GenE)),
                                ].spacing(20),

                                 row![
                                    container(text("d : ").size(20)),
                                    container(text(self.d.to_string()).size(20)).width(Fill),
                                    container(button(if self.progress_d {"calculating..."} else {"calculate"}).on_press_maybe(
                                        if self.progress_d {None} else {Some(Message::CalculateD)}
                                    )),
                                ].spacing(20),

                                row![
                                    container(text("decrypted message : ").size(20)),
                                    container(text(self.decrypted_message.to_string()).size(20)).width(Fill),
                                    container(button(if self.progress_decrypt {"decrypting..."} else {"decrypt"}).on_press_maybe(
                                        if self.progress_decrypt {None} else {Some(Message::Decrypt)}
                                    )),
                                ].spacing(20),

                            ].spacing(20)
                        ).center_y(Fill).width(Fill).padding(10)

                    ],
                ).center_x(Fill).width(Fill).height(Fill).padding(10)
                .style(|_theme| Style {
                background: Default::default(),
                text_color: Option::from(Color::WHITE),
                border: Border::default().rounded(10).color(Color{r: 0.0 , g: 0.0 , b: 1.0 , a: 1.0 , }).width(1.0),
                shadow: Shadow::default(),}), // Pass a closure here,
            ],

            spacer().height(Pixels(20.0)),

            container(column![
                container(text("Public Infos").size(20)).center_x(Fill).width(Fill),
                container(
                    column![

                        row![
                            container(text("e : ").size(20)),
                            container(text(self.e.to_string()).size(20)),
                        ].spacing(20),
                        row![
                            container(text("n : ").size(20)),
                            container(text(self.n.to_string()).size(20)),
                        ].spacing(20),

                        row![
                            container(text("encrypted message : ").size(20)),
                            container(text(&self.encrypted_message).size(20)),
                        ].spacing(20),

                    ].spacing(20),
                ).center_x(Fill).center_y(Fill).width(Fill),
            ]).height(Pixels(200.0)).width(Fill).padding(10)
            .style(|_theme| Style {
            background: Default::default(),
            text_color: Option::from(Color::WHITE),
            border: Border::default().rounded(10).color(Color{r: 0.0 , g: 1.0 , b: 0.0 , a: 1.0 , }).width(1.0),
            shadow: Shadow::default(),}), // Pass a closure here
        ].padding(20)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GenP => {
                self.p = prime_gen::prime_gen(self.range_min as u64, self.range_max as u64);
                self.calculate_n();
            }
            Message::GenQ => {
                self.q = prime_gen::prime_gen(self.range_min as u64, self.range_max as u64);
                self.calculate_n();
            }
            Message::GenE => {
                let mut e = 2u128;
                loop {
                    if are_coprime(e, self.phi_n) {
                        self.e = e;
                        break;
                    }
                    e += 1;
                }
            }
            Message::CalculateD => {
                let e = self.e;
                let phi_n = self.phi_n;
                self.progress_d = true;
                return Task::future(async move {
                    let d = tokio::task::spawn_blocking(move || {
                        println!("Calculating d...");
                        inverse_modular::inverse_modular(u64::try_from(e).unwrap(), phi_n)
                    })
                        .await
                        .unwrap();

                    // Send the information back to the update function
                    Message::CalculateDFinished(d)
                });
            }
            Message::RangeMin(range) => {
                let nb = range.parse().unwrap_or(2);
                if nb < 2 || nb >= self.range_max {
                    return Task::none();
                }
                self.range_min = nb;
            }
            Message::RangeMax(range) => {
                let nb = range.parse().unwrap_or(i16::MAX);
                if nb < 2 || nb <= self.range_min as i16 {
                    return Task::none();
                }
                self.range_max = nb as u16;
            }
            Message::Message(msg) => {
                let nb = msg.parse().unwrap_or(0);
                if nb >= self.n {
                    return Task::none();
                }
                self.message = nb as u64;
            }
            Message::Encrypt => {
                let mut exp = fast_exponentiation(u128::try_from(self.message).unwrap(), self.e as u32);
                exp.rem_assign(BigInt::from(self.n));
                self.encrypted_message = exp.to_u64().unwrap();
                return Task::none();
            }
            Message::Decrypt => {
                let encrypted_message = self.encrypted_message.clone();
                let d = self.d;
                let n = self.n.clone();
                self.progress_decrypt = true;

                return Task::future(async move {
                    let information = tokio::task::spawn_blocking(move || {
                        println!("Decrypting message...");
                        let mut exp = parallel_fast_exponentiation(u128::try_from(encrypted_message).unwrap(), d as u32);
                        exp.rem_assign(BigInt::from(n));
                        exp.to_u64().unwrap_or(0)
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
        Theme::Dark
    }
}

