defmodule Tokenizers.Decoder.BPE do
  @doc """
  Creates new BPE decoder
  """
  @spec new(suffix :: String.t()) :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(suffix \\ "</w>"), to: Tokenizers.Native, as: :decoders_bpe
end
