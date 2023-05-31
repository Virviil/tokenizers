defmodule Tokenizers.DecoderTest do
  use ExUnit.Case, async: true
  doctest Tokenizers.Decoder

  describe "WordPiece Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.WordPiece.new()
    end

    test "accepts only first parameter" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.WordPiece.new("test")
    end

    test "can decode array of strings" do
      assert Tokenizers.Decoder.WordPiece.new()
             |> elem(1)
             |> Tokenizers.Decoder.decode(["Hel", "##lo", "there", "my", "fr", "##iend"]) ==
               {:ok, "Hello there my friend"}
    end
  end

  describe "ByteFallback Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.WordPiece.new()
    end

    test "can decode array of strings" do
      {:ok, decoder} = Tokenizers.Decoder.ByteFallback.new()

      [
        {["Hel", "lo"], "Hello"},
        {["<0x61>"], "a"},
        {["<0x61>"], "a"},
        {["My", " na", "me"], "My name"},
        {["<0x61>"], "a"},
        {["<0xE5>"], "�"},
        {["<0xE5>", "<0x8f>"], "��"},
        {["<0xE5>", "<0x8f>", "<0xab>"], "叫"},
        {["<0xE5>", "<0x8f>", "a"], "��a"},
        {["<0xE5>", "<0x8f>", "<0xab>", "a"], "叫a"}
      ]
      |> Enum.each(fn {tokens, result} ->
        assert Tokenizers.Decoder.decode(decoder, tokens) == {:ok, result}
      end)
    end
  end

  describe "Replace Decoder" do
    test "can decode array of strings" do
      {:ok, decoder} = Tokenizers.Decoder.Replace.new("_", " ")

      assert Tokenizers.Decoder.decode(decoder, ["Hello", "_Hello"]) == {:ok, "Hello Hello"}
    end
  end

  describe "Fuse Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.Fuse.new()
    end

    test "can decode array of strings" do
      assert Tokenizers.Decoder.Fuse.new()
             |> elem(1)
             |> Tokenizers.Decoder.decode(["Hel", "lo"]) ==
               {:ok, "Hello"}
    end
  end

  describe "Strip Decoder" do
    test "can be initialized" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.Strip.new(?_, 0, 0)
    end

    test "cant be initialized with invalid char" do
      assert {:error, _} = Tokenizers.Decoder.Strip.new(61_126_999, 0, 0)
    end

    test "can decode array of strings" do
      assert Tokenizers.Decoder.Strip.new(?_, 1, 0)
             |> elem(1)
             |> Tokenizers.Decoder.decode(["_Hel", "lo", "__there"]) ==
               {:ok, "Hello_there"}
    end
  end

  describe "Metaspace Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.Metaspace.new()
    end

    test "accepts only first parameter" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.Metaspace.new(?t)
    end
  end

  describe "BPE Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.BPE.new()
    end
  end

  describe "CTC Decoder" do
    test "accepts no parameters" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.CTC.new()
    end

    test "can decode array of strings" do
      assert Tokenizers.Decoder.CTC.new()
             |> elem(1)
             |> Tokenizers.Decoder.decode([
               "<pad>",
               "h",
               "h",
               "e",
               "e",
               "l",
               "l",
               "<pad>",
               "l",
               "l",
               "o"
             ]) ==
               {:ok, "hello"}
    end
  end

  describe "Sequence Decoder" do
    test "accepts empty list as parameter" do
      assert {:ok, %Tokenizers.Decoder{}} = Tokenizers.Decoder.Sequence.new([])
    end

    test "can decode array of strings correctly" do
      {:ok, ctc} = Tokenizers.Decoder.CTC.new()
      {:ok, metaspace} = Tokenizers.Decoder.Metaspace.new()

      assert Tokenizers.Decoder.Sequence.new([ctc, metaspace])
             |> elem(1)
             |> Tokenizers.Decoder.decode(["▁", "▁", "H", "H", "i", "i", "▁", "y", "o", "u"]) ==
               {:ok, "Hi you"}
    end
  end
end
