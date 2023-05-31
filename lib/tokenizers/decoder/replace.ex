defmodule Tokenizers.Decoder.Replace do
  @doc """
  Creates new Replace decoder
  """
  @spec new(pattern :: String.t(), content :: String.t()) ::
          {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(pattern, content), to: Tokenizers.Native, as: :decoders_replace
end
