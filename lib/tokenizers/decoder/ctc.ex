defmodule Tokenizers.Decoder.CTC do
  @doc """
  Creates new CTC decoder
  """
  @spec new(pad_token :: String.t(), word_delimiter_token :: String.t(), cleanup :: boolean()) ::
          {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(pad_token \\ "<pad>", word_delimiter_token \\ "|", cleanup \\ true),
    to: Tokenizers.Native,
    as: :decoders_ctc
end
