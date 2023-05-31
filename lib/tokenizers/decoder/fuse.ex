defmodule Tokenizers.Decoder.Fuse do
  @doc """
  Creates new Fuse decoder
  """
  @spec new :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(), to: Tokenizers.Native, as: :decoders_fuse
end
