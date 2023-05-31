defmodule Tokenizers.Decoder.Metaspace do
  @doc """
  Creates new Metaspace decoder
  """
  @spec new(replacement :: char(), prefix_space :: boolean()) :: {:ok, Tokenizers.Decoder.t()} | {:error, any()}
  defdelegate new(replacement \\ ?▁, add_prefix_space \\ true), to: Tokenizers.Native, as: :decoders_metaspace
end
