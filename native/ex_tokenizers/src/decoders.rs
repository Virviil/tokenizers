use std::collections::HashMap;

use tokenizers::{Decoder, DecoderWrapper};

use crate::{util::DetailValue, ExTokenizersError};

pub struct ExTokenizersDecoderRef(pub DecoderWrapper);

#[derive(rustler::NifStruct)]
#[module = "Tokenizers.Decoder"]
pub struct ExTokenizersDecoder {
    pub resource: rustler::resource::ResourceArc<ExTokenizersDecoderRef>,
}

impl ExTokenizersDecoderRef {
    pub fn new<T>(data: T) -> Self
    where
        T: Into<DecoderWrapper>,
    {
        Self(data.into())
    }
}

impl ExTokenizersDecoder {
    pub fn new<T>(data: T) -> Self
    where
        T: Into<DecoderWrapper>,
    {
        Self {
            resource: rustler::resource::ResourceArc::new(ExTokenizersDecoderRef::new(data)),
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decoders_decode(
    decoder: ExTokenizersDecoder,
    tokens: Vec<String>,
) -> Result<String, ExTokenizersError> {
    decoder
        .resource
        .0
        .decode(tokens)
        .map_err(|e| ExTokenizersError::Tokenizer(e))
}

///////////////////////////////////////////////////////////////////////////////
/// Inspection
///////////////////////////////////////////////////////////////////////////////

#[rustler::nif]
fn decoders_info(
    decoder: ExTokenizersDecoder,
) -> Result<HashMap<String, DetailValue>, ExTokenizersError> {
    Ok(match &decoder.resource.0 {
        tokenizers::DecoderWrapper::BPE(decoder) => HashMap::from([
            ("decoder_type".into(), DetailValue::String("BPE".into())),
            ("suffix".into(), DetailValue::String(decoder.suffix.clone())),
        ]),
        DecoderWrapper::ByteLevel(decoder) => HashMap::from([
            (
                "decoder_type".into(),
                DetailValue::String("ByteLevel".into()),
            ),
            (
                "add_prefix_space".into(),
                DetailValue::Bool(decoder.add_prefix_space),
            ),
            (
                "trim_offsets".into(),
                DetailValue::Bool(decoder.trim_offsets),
            ),
            ("use_regex".into(), DetailValue::Bool(decoder.use_regex)),
        ]),
        DecoderWrapper::WordPiece(decoder) => HashMap::from([
            (
                "decoder_type".into(),
                DetailValue::String("WordPiece".into()),
            ),
            ("prefix".into(), DetailValue::String(decoder.prefix.clone())),
            ("cleanup".into(), DetailValue::Bool(decoder.cleanup)),
        ]),
        DecoderWrapper::Metaspace(decoder) => HashMap::from([
            (
                "decoder_type".into(),
                DetailValue::String("Metaspace".into()),
            ),
            (
                "add_prefix_space".into(),
                DetailValue::Bool(decoder.add_prefix_space),
            ),
        ]),
        DecoderWrapper::CTC(decoder) => HashMap::from([
            ("decoder_type".into(), DetailValue::String("CTC".into())),
            (
                "pad_token".into(),
                DetailValue::String(decoder.pad_token.clone()),
            ),
            (
                "word_delimiter_token".into(),
                DetailValue::String(decoder.word_delimiter_token.clone()),
            ),
            ("cleanup".into(), DetailValue::Bool(decoder.cleanup)),
        ]),
        DecoderWrapper::Sequence(_decoder) => HashMap::from([(
            "decoder_type".into(),
            DetailValue::String("Sequence".into()),
        )]),
        DecoderWrapper::Replace(_decoder) => {
            HashMap::from([("decoder_type".into(), DetailValue::String("Replace".into()))])
        }

        DecoderWrapper::Fuse(_decoder) => {
            HashMap::from([("decoder_type".into(), DetailValue::String("Fuse".into()))])
        }
        DecoderWrapper::Strip(decoder) => HashMap::from([
            ("decoder_type".into(), DetailValue::String("Strip".into())),
            (
                "content".into(),
                DetailValue::String(decoder.content.into()),
            ),
            ("start".into(), DetailValue::USize(decoder.start)),
            ("stop".into(), DetailValue::USize(decoder.stop)),
        ]),
        DecoderWrapper::ByteFallback(_decoder) => HashMap::from([(
            "decoder_type".into(),
            DetailValue::String("ByteFallback".into()),
        )]),
    })
}

///////////////////////////////////////////////////////////////////////////////
/// Builders
///////////////////////////////////////////////////////////////////////////////

#[rustler::nif]
fn decoders_byte_level() -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::byte_level::ByteLevel::default(),
    ))
}

#[rustler::nif]
fn decoders_replace(
    pattern: String,
    content: String,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::normalizers::Replace::new(pattern, content)?,
    ))
}

#[rustler::nif]
fn decoders_wordpiece(
    prefix: String,
    cleanup: bool,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::wordpiece::WordPiece::new(prefix, cleanup),
    ))
}

#[rustler::nif]
fn decoders_byte_fallback() -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::byte_fallback::ByteFallback::new(),
    ))
}

#[rustler::nif]
fn decoders_fuse() -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::fuse::Fuse::new(),
    ))
}

#[rustler::nif]
fn decoders_strip(
    content: u32,
    left: usize,
    right: usize,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    let content = std::char::from_u32(content).ok_or(ExTokenizersError::InvalidChar)?;
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::strip::Strip::new(content, left, right),
    ))
}

#[rustler::nif]
fn decoders_metaspace(
    replacement: u32,
    add_prefix_space: bool,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    let replacement = std::char::from_u32(replacement).ok_or(ExTokenizersError::InvalidChar)?;
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::metaspace::Metaspace::new(replacement, add_prefix_space),
    ))
}

#[rustler::nif]
fn decoders_bpe(suffix: String) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::bpe::BPEDecoder::new(suffix),
    ))
}

#[rustler::nif]
fn decoders_ctc(
    pad_token: String,
    word_delimiter_token: String,
    cleanup: bool,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::ctc::CTC::new(pad_token, word_delimiter_token, cleanup),
    ))
}

#[rustler::nif]
fn decoders_sequence(
    decoders: Vec<ExTokenizersDecoder>,
) -> Result<ExTokenizersDecoder, ExTokenizersError> {
    let sequence = decoders
        .iter()
        .map(|decoder| decoder.resource.clone())
        .fold(Vec::with_capacity(decoders.len()), |mut acc, decoder| {
            acc.push(decoder.0.clone());
            acc
        });

    Ok(ExTokenizersDecoder::new(
        tokenizers::decoders::sequence::Sequence::new(sequence),
    ))
}
