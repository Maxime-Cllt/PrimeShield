mod utils;

mod fast_exponentiation;
mod inverse_modular;
mod prime_gen;
#[cfg(test)]
mod tests;

use std::ops::{Rem, RemAssign};
use crate::utils::e_is_prime_with;
use iced::widget::container::Style;
use iced::widget::{button, column, container, horizontal_space, row, text, text_input, vertical_space as spacer, Column};
use iced::{Border, Center, Color, Fill, Pixels, Shadow, Size, Theme};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use crate::fast_exponentiation::fast_exponentiation;

struct App {
    p: u64, // bob prime number
    q: u64, // bob prime number
    modu: u64, // bob modulo
    e: u128, // alice public key
    d: u128, // bob private key
    n: u128, // bob public key
    message: u64, // alice message
    encrypted_message: u64, // alice encrypted message
    decrypted_message: u64, // bob decrypted message
    range_min: u16, // range prime gen
    range_max: u16, // range prime gen
}

impl Default for App {
    fn default() -> Self {
        Self {
            p: 0,
            q: 0,
            modu: 0,
            e: 5,
            d: 5,
            n: 21,
            message: 0,
            encrypted_message: 0,
            decrypted_message: 0,
            range_min: 2,
            range_max: i16::MAX as u16,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GenP,
    GenQ,
    GenE,
    CalculateD,
    RangeMin(String),
    RangeMax(String),
    Message(String),
    Encrypt,
    Decrypt,
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
                                    container(text("e : ").size(20)),
                                    container(text(self.e.to_string()).size(20)).width(Fill),
                                    container(button("generate").on_press(Message::GenE)),
                                ].spacing(20),

                                 row![
                                    container(text("d : ").size(20)),
                                    container(text(self.d.to_string()).size(20)).width(Fill),
                                    container(button("calculate").on_press(Message::CalculateD)),
                                ].spacing(20),

                                row![
                                    container(text("decrypted message : ").size(20)),
                                    container(text(self.decrypted_message.to_string()).size(20)).width(Fill),
                                    container(button("decrypt").on_press(Message::Decrypt)),
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

    pub fn update(&mut self, message: Message) {
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
                loop{
                    if e_is_prime_with(e, u128::from(self.p), u128::from(self.q)) {
                        self.e = e;
                        break;
                    }
                    e += 1;
                }
            }
            Message::CalculateD => {
                self.d = inverse_modular::inverse_modular(u64::try_from(self.e).unwrap(), u64::try_from(self.p).unwrap(), u64::try_from(self.q).unwrap());
            }
            Message::RangeMin(range) => {
                let nb = range.parse().unwrap_or(2);
                if nb < 2 || nb >= self.range_max {
                    return;
                }
                self.range_min = nb;
            }
            Message::RangeMax(range) => {
                let nb = range.parse().unwrap_or(i16::MAX);
                if nb < 2 || nb <= self.range_min as i16 {
                    return;
                }
                self.range_max = nb as u16;
            }
            Message::Message(msg) => {
                let nb = msg.parse().unwrap_or(0);
                if nb >= self.n {
                    return;
                }
                self.message = nb as u64;
            }
            Message::Encrypt => {
                let mut exp = fast_exponentiation(u128::try_from(self.message).unwrap(), self.e as u16);
                exp.rem_assign(BigInt::from(self.n));
                self.encrypted_message = exp.to_u64().unwrap();
            }
            Message::Decrypt => {
                let mut exp = fast_exponentiation(u128::try_from(self.encrypted_message).unwrap(), self.d as u16);
                exp.rem_assign(BigInt::from(self.n));
                self.decrypted_message = exp.to_u64().unwrap();
            }
        }
    }

    fn calculate_n(&mut self) {
        self.n = u128::from(self.p) * u128::from(self.q);
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

