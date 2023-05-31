defmodule Tokenizers.Decoder.WordPiece do
  @doc """
  Creates new WordPiece decoder
  """
  @spec new(prefix :: String.t(), cleanup :: boolean()) :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(prefix \\ "##", cleanup \\ true), to: Tokenizers.Native, as: :decoders_wordpiece
end
