defmodule Tokenizers.Decoder do
  defstruct [:resource]
  @type t() :: %__MODULE__{resource: reference()}

  @spec decode(t(), [String.t()]) :: {:ok, String.t()} | {:error, any()}
  defdelegate decode(decoder, tokens), to: Tokenizers.Native, as: :decoders_decode
end

defimpl Inspect, for: Tokenizers.Decoder do
  import Inspect.Algebra

  def inspect(decoder, opts) do
    attrs =
      decoder
      |> Tokenizers.Native.decoders_info()
      |> Tokenizers.Shared.unwrap()
      |> Keyword.new(fn {k, v} -> {String.to_atom(k), v} end)

    concat(["#Tokenizers.Decoder<", to_doc(attrs, opts), ">"])
  end
end
