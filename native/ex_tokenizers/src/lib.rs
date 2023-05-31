mod decoders;
mod encoding;
mod error;
mod models;
mod tokenizer;
mod util;

use decoders::*;
use encoding::*;
use models::*;
use rustler::{Env, Term};
use tokenizer::*;

pub use error::ExTokenizersError;

use crate::decoders::ExTokenizersDecoderRef;

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExTokenizersDecoderRef, env);

    rustler::resource!(ExTokenizersTokenizerRef, env);
    rustler::resource!(ExTokenizersEncodingRef, env);
    rustler::resource!(ExTokenizersModelRef, env);
    true
}

rustler::init!(
    "Elixir.Tokenizers.Native",
    [
        // Decoders
        decoders_decode,
        //
        decoders_info,
        //
        decoders_byte_level,
        decoders_replace,
        decoders_wordpiece,
        decoders_byte_fallback,
        decoders_fuse,
        decoders_strip,
        decoders_metaspace,
        decoders_bpe,
        decoders_ctc,
        decoders_sequence,
        // Models
        models_save,
        //
        models_info,
        //
        models_bpe_init,
        models_bpe_empty,
        models_bpe_from_file,
        //
        models_wordpiece_init,
        models_wordpiece_empty,
        models_wordpiece_from_file,
        //
        models_wordlevel_init,
        models_wordlevel_empty,
        models_wordlevel_from_file,
        //
        models_unigram_init,
        models_unigram_empty,
        // Misc
        decode,
        decode_batch,
        encode,
        encode_batch,
        from_file,
        get_attention_mask,
        get_u32_attention_mask,
        get_type_ids,
        get_u32_type_ids,
        get_ids,
        get_u32_ids,
        get_special_tokens_mask,
        get_u32_special_tokens_mask,
        get_offsets,
        get_model,
        get_tokens,
        get_vocab,
        get_vocab_size,
        id_to_token,
        n_tokens,
        pad,
        save,
        token_to_id,
        truncate,
    ],
    load = on_load
);
