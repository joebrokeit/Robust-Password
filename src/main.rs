use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Default)]

struct Password{
    password:String,
}

#[derive(Debug, Copy, Clone)]
pub enum Message{
    GeneratePass,
    SaveToFile,
    Exit,
}

impl Sandbox for Password{
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }
    fn title(&self) -> String {
        String::from("Password Generator 1.0")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::GeneratePass=>{
                self.password = generate_password(12);
            },
            Message::SaveToFile=>{
                save_to_file(&self.password)
            },
            Message::Exit =>{
                std::process::exit(0)
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {
        column![
            text("Welcome.. Generate your password").size(30),
            button("Generate Password").on_press(Message::GeneratePass),
            text(&self.password).size(30),
            button("Save").on_press(Message::SaveToFile),
            button("Exit").on_press(Message::Exit)
        ]
        .align_items(iced::Alignment::Center)
        .spacing(20)
        .into()
    }
}

fn generate_password(length:usize)->String{
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
    let mut rng = thread_rng();
    (0..length)
    .map(|_|charset.chars().nth(rng.gen_range(0..charset.len())).unwrap())
    .collect()
}

fn save_to_file(password:&str){
    let path = Path::new("password.txt");
    let mut file = match File::create(&path) {
        Ok(file)=>file,
        Err(e)=>{
            eprintln!("Failed to create file {} ",e);
            return;
        },
    };

    if let Err(e)=writeln!(file, "{}",password){
        eprintln!("failed to write to file {}",e)
    }
    else {
        println!("password saved to file {:?}",path.display());
    }
}

fn main()->iced::Result{
    Password::run(Settings::default())
}

