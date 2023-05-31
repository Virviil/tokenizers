use rustler::NifUntaggedEnum;

#[derive(NifUntaggedEnum)]
pub enum DetailValue {
    String(String),
    OptionString(Option<String>),
    OptionNumber(Option<f32>),
    Bool(bool),
    F64(f64),
    USize(usize),
}
