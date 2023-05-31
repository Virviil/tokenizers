defmodule Tokenizers.Decoder.ByteFallback do
    @doc """
  Creates new ByteFallback decoder
  """
  @spec new() :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(), to: Tokenizers.Native, as: :decoders_byte_fallback
end
