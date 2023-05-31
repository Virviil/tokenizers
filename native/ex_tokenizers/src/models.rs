use std::collections::HashMap;
use std::path::Path;

use rustler::NifTaggedEnum;
use tokenizers::models::bpe::BpeBuilder;
use tokenizers::models::wordlevel::WordLevelBuilder;
use tokenizers::models::wordpiece::WordPieceBuilder;
use tokenizers::{Model, ModelWrapper};

use crate::error::ExTokenizersError;
use crate::util::DetailValue;

pub struct ExTokenizersModelRef(pub ModelWrapper);

#[derive(rustler::NifStruct)]
#[module = "Tokenizers.Model"]
pub struct ExTokenizersModel {
    pub resource: rustler::resource::ResourceArc<ExTokenizersModelRef>,
}

impl ExTokenizersModelRef {
    pub fn new<T>(data: T) -> Self
    where
        T: Into<ModelWrapper>,
    {
        Self(data.into())
    }
}

impl ExTokenizersModel {
    pub fn new<T>(data: T) -> Self
    where
        T: Into<ModelWrapper>,
    {
        Self {
            resource: rustler::resource::ResourceArc::new(ExTokenizersModelRef::new(data)),
        }
    }
}

#[rustler::nif]
pub fn models_save(
    model: ExTokenizersModel,
    folder: String,
    prefix: String,
) -> Result<Vec<String>, ExTokenizersError> {
    Ok(model
        .resource
        .0
        .save(Path::new(&folder), Some(&prefix))?
        .iter()
        .map(|path| {
            path.to_str()
                // Unwraping here, because we are sure that pathes are valid
                .unwrap()
                .to_owned()
        })
        .collect())
}

///////////////////////////////////////////////////////////////////////////////
/// Inspection
///////////////////////////////////////////////////////////////////////////////

#[rustler::nif]
pub fn models_info(
    model: ExTokenizersModel,
) -> Result<HashMap<String, DetailValue>, ExTokenizersError> {
    Ok(match &model.resource.0 {
        ModelWrapper::BPE(model) => HashMap::from([
            (
                String::from("model_type"),
                DetailValue::String(String::from("bpe")),
            ),
            (
                String::from("dropout"),
                DetailValue::OptionNumber(model.dropout),
            ),
            (
                String::from("unk_token"),
                DetailValue::OptionString(model.unk_token.clone()),
            ),
            (
                String::from("continuing_subword_prefix"),
                DetailValue::OptionString(model.continuing_subword_prefix.clone()),
            ),
            (
                String::from("end_of_word_suffix"),
                DetailValue::OptionString(model.end_of_word_suffix.clone()),
            ),
            (String::from("fuse_unk"), DetailValue::Bool(model.fuse_unk)),
            (
                String::from("byte_fallback"),
                DetailValue::Bool(model.byte_fallback),
            ),
        ]),
        ModelWrapper::WordPiece(model) => HashMap::from([
            (
                String::from("model_type"),
                DetailValue::String(String::from("wordpiece")),
            ),
            (
                String::from("unk_token"),
                DetailValue::String(model.unk_token.clone()),
            ),
            (
                String::from("continuing_subword_prefix"),
                DetailValue::String(model.continuing_subword_prefix.clone()),
            ),
            (
                String::from("max_input_chars_per_word"),
                DetailValue::USize(model.max_input_chars_per_word),
            ),
        ]),
        ModelWrapper::WordLevel(model) => HashMap::from([
            (
                String::from("model_type"),
                DetailValue::String(String::from("wordlevel")),
            ),
            (
                String::from("unk_token"),
                DetailValue::String(model.unk_token.clone()),
            ),
        ]),
        ModelWrapper::Unigram(model) => HashMap::from([
            (
                String::from("model_type"),
                DetailValue::String(String::from("unigram")),
            ),
            (String::from("min_score"), DetailValue::F64(model.min_score)),
        ]),
    })
}

///////////////////////////////////////////////////////////////////////////////
/// BPE
///////////////////////////////////////////////////////////////////////////////

#[derive(NifTaggedEnum)]
pub enum BPEOption {
    CacheCapacity(usize),
    Dropout(f32),
    UnkToken(String),
    ContinuingSubwordPrefix(String),
    EndOfWordSuffix(String),
    FuseUnk(bool),
    ByteFallback(bool),
}

fn populate_bpe_options_to_builder(builder: BpeBuilder, options: Vec<BPEOption>) -> BpeBuilder {
    options
        .iter()
        .fold(builder, |builder, option| match option {
            BPEOption::CacheCapacity(capacity) => builder.cache_capacity(*capacity),
            BPEOption::Dropout(dropout) => builder.dropout(*dropout),
            BPEOption::UnkToken(unk_token) => builder.unk_token(unk_token.clone()),
            BPEOption::ContinuingSubwordPrefix(prefix) => {
                builder.continuing_subword_prefix(prefix.clone())
            }
            BPEOption::EndOfWordSuffix(prefix) => builder.end_of_word_suffix(prefix.clone()),
            BPEOption::FuseUnk(fuse_unk) => builder.fuse_unk(*fuse_unk),
            BPEOption::ByteFallback(byte_fallback) => builder.byte_fallback(*byte_fallback),
        })
}

#[rustler::nif]
pub fn models_bpe_init(
    vocab: HashMap<String, u32>,
    merges: Vec<(String, String)>,
    options: Vec<BPEOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    let model = populate_bpe_options_to_builder(
        tokenizers::models::bpe::BPE::builder().vocab_and_merges(vocab, merges),
        options,
    )
    .build()?;
    Ok(ExTokenizersModel::new(model))
}

#[rustler::nif]
pub fn models_bpe_empty() -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        tokenizers::models::bpe::BPE::default(),
    ))
}

#[rustler::nif(schedule = "DirtyIo")]
pub fn models_bpe_from_file(
    vocab: String,
    merges: String,
    options: Vec<BPEOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    let model = populate_bpe_options_to_builder(
        tokenizers::models::bpe::BPE::from_file(&vocab, &merges),
        options,
    )
    .build()?;
    Ok(ExTokenizersModel::new(model))
}

///////////////////////////////////////////////////////////////////////////////
/// WordPiece
///////////////////////////////////////////////////////////////////////////////

#[derive(NifTaggedEnum)]
pub enum WordPieceOption {
    UnkToken(String),
    ContinuingSubwordPrefix(String),
    MaxInputCharsPerWord(usize),
}

fn populate_wordpiece_options_to_builder(
    builder: WordPieceBuilder,
    options: Vec<WordPieceOption>,
) -> WordPieceBuilder {
    options
        .iter()
        .fold(builder, |builder, option| match option {
            WordPieceOption::UnkToken(unk_token) => builder.unk_token(unk_token.clone()),
            WordPieceOption::ContinuingSubwordPrefix(continuing_subword_prefix) => {
                builder.continuing_subword_prefix(continuing_subword_prefix.clone())
            }
            WordPieceOption::MaxInputCharsPerWord(max_input_chars_per_word) => {
                builder.max_input_chars_per_word(*max_input_chars_per_word)
            }
        })
}

#[rustler::nif]
pub fn models_wordpiece_init(
    vocab: HashMap<String, u32>,
    options: Vec<WordPieceOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        populate_wordpiece_options_to_builder(
            tokenizers::models::wordpiece::WordPiece::builder().vocab(vocab),
            options,
        )
        .build()?,
    ))
}

#[rustler::nif]
pub fn models_wordpiece_empty() -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        tokenizers::models::wordpiece::WordPiece::default(),
    ))
}

#[rustler::nif(schedule = "DirtyIo")]
pub fn models_wordpiece_from_file(
    vocab: String,
    options: Vec<WordPieceOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    let model = populate_wordpiece_options_to_builder(
        tokenizers::models::wordpiece::WordPiece::from_file(&vocab),
        options,
    )
    .build()?;
    Ok(ExTokenizersModel::new(model))
}

///////////////////////////////////////////////////////////////////////////////
/// WordLevel
///////////////////////////////////////////////////////////////////////////////

#[derive(NifTaggedEnum)]
pub enum WordLevelOption {
    UnkToken(String),
}

fn populate_wordlevel_options_to_builder(
    builder: WordLevelBuilder,
    options: Vec<WordLevelOption>,
) -> WordLevelBuilder {
    options
        .iter()
        .fold(builder, |builder, option| match option {
            WordLevelOption::UnkToken(unk_token) => builder.unk_token(unk_token.clone()),
        })
}

#[rustler::nif]
pub fn models_wordlevel_init(
    vocab: HashMap<String, u32>,
    options: Vec<WordLevelOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        populate_wordlevel_options_to_builder(
            tokenizers::models::wordlevel::WordLevel::builder().vocab(vocab),
            options,
        )
        .build()?,
    ))
}

#[rustler::nif]
pub fn models_wordlevel_empty() -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        tokenizers::models::wordlevel::WordLevel::default(),
    ))
}

#[rustler::nif(schedule = "DirtyIo")]
pub fn models_wordlevel_from_file(
    vocab: String,
    options: Vec<WordLevelOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    let model = populate_wordlevel_options_to_builder(
        tokenizers::models::wordlevel::WordLevel::builder().files(vocab),
        options,
    )
    .build()?;
    Ok(ExTokenizersModel::new(model))
}

///////////////////////////////////////////////////////////////////////////////
/// Unigram
///////////////////////////////////////////////////////////////////////////////

#[derive(NifTaggedEnum)]
pub enum UnigramOption {
    UnkId(usize),
}

#[rustler::nif]
pub fn models_unigram_init(
    vocab: Vec<(String, f64)>,
    options: Vec<UnigramOption>,
) -> Result<ExTokenizersModel, ExTokenizersError> {
    let unk_id = if !options.is_empty() {
        match options[0] {
            UnigramOption::UnkId(unk_id) => Some(unk_id),
        }
    } else {
        None
    };

    Ok(ExTokenizersModel::new(
        tokenizers::models::unigram::Unigram::from(vocab, unk_id)?,
    ))
}

#[rustler::nif]
pub fn models_unigram_empty() -> Result<ExTokenizersModel, ExTokenizersError> {
    Ok(ExTokenizersModel::new(
        tokenizers::models::unigram::Unigram::default(),
    ))
}
