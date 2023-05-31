defmodule Tokenizers.Decoder.Strip do
  @doc """
  Creates new Strip decoder
  """
  @spec new(content :: char(), left :: non_neg_integer(), right :: non_neg_integer()) :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(content, left, right), to: Tokenizers.Native, as: :decoders_strip
end
