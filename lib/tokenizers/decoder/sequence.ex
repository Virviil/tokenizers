defmodule Tokenizers.Decoder.Sequence do
  @doc """
  Creates new Sequence decoder
  """
  @spec new(decoders :: [Tokenizers.Decoder.t()]) ::
          {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(decoders), to: Tokenizers.Native, as: :decoders_sequence
end
