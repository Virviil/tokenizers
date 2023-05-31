defmodule Tokenizers.Decoder.ByteLevel do
  @doc """
  Creates new ByteLevel decoder
  """
  @spec new() :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(), to: Tokenizers.Native, as: :decoders_byte_level
end
