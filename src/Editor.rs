use iced::{
    widget::{
        button, column, row, text ,text_input, text_editor
    }, Element, Renderer, Task, Theme
};
use std::fs;

#[derive(Debug, Default)]
pub struct Editor{
    path: String,
    file_content: text_editor::Content,
    statue: String,
}

#[derive(Debug, Clone)]
pub enum Message{
    Save,
    FilePathInput(String),
    SubmitFile,
    Editing(text_editor::Action),
    Delete,
}

impl Editor{
    pub fn run(title: &'static str) -> iced::Result{
        iced::application(title, Editor::update, view)
            .theme(|_s| iced::Theme::Dark)
            .run()?;
        Ok(())
    }

    fn get_file(path: &String) -> Result<text_editor::Content, std::io::Error>{
        let content:String = fs::read_to_string(path)?;

        Ok(text_editor::Content::with_text(&content.to_owned()))
    }

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Save =>{ 
                match Editor::save_file(self, true){
                    Ok(_) => self.statue = "Saved!".to_string(),
                    Err(e) => self.statue = format!("Failed to save file: {}. With error: {:?}", 
                    self.path, e),
                }
            }.into(),

            Message::FilePathInput(path) => (self.path = path).into(),

            Message::SubmitFile => {
                match Editor::get_file(&self.path) {
                    Ok(content) => {
                        self.file_content = content;
                        self.statue = format!("{} loaded.", self.path);
                    }

                    Err(e) => {
                        match Editor::save_file(self, false){
                            Ok(_) => self.statue = "Saved!".to_string(),

                            Err(e) => self.statue = format!("Failed to create file at: {}, 
                            with error: {:?}", self.path, e),
                        };

                        self.statue = format!("Failed to load file. New file created at '{}'. 
                        Error message: {:?},", self.path, e);
                    }
                }
            }.into(),

            Message::Delete => {
                match Editor::delete_file(&self.path){
                    Ok(_) => {
                        self.statue = format!("{} has been deleted.", self.path);
                    },
                    Err(e) => {
                        self.statue = format!("Failed to delete file: {} with error: {:?}", self.path, e);
                    }
                }
            }.into(),

            Message::Editing(action) => {
                self.file_content.perform(action);
            }.into()
        }
    }

    fn save_file(&mut self, condition: bool) -> Result<(), std::io::Error>{
        if condition == false {
            fs::write(&self.path,  "Write something now!")?;
        }else{
            fs::write(&self.path, &self.file_content.text())?;
        }
        Ok(())
    }

    fn delete_file(path: &String) -> Result<(), std::io::Error>{
        fs::remove_file(path)?;
        Ok(())
    }
}

fn view(state: &Editor) -> Element<'_, Message>{
    column![
        row![
            text_input::<Message, Theme, Renderer>("type the path to load", &state.path)
            .on_input(Message::FilePathInput),
            button(text("Submit")).on_press(Message::SubmitFile)
        ],
        text(&state.statue),
        row![
            button(text("Save")).on_press(Message::Save),
            button(text("delete")).on_press(Message::Delete),
        ].spacing(10),
        text_editor(&state.file_content)
        .on_action(Message::Editing),
    ].into() 
}
