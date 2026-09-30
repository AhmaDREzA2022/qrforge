pub enum PrintMode {
    Ascii,
    Png,
}

pub struct QrOptions {
    pub data: String,
    pub output: Option<String>,
    pub print_mode: Option<PrintMode>
}

