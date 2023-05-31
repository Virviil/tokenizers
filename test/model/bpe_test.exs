defmodule Tokenizers.Model.BPETest do
  use ExUnit.Case, async: true
  doctest Tokenizers.Model.BPE

  describe "Initialized from memory" do
    test "returns loaded model" do
      assert {:ok, %Tokenizers.Model{}} =
               Tokenizers.Model.BPE.init(%{"a" => 0, "b" => 1, "ab" => 2}, [{"a", "b"}])
    end

    test "accepts keyword params" do
      assert {:ok, %Tokenizers.Model{}} =
               Tokenizers.Model.BPE.init(%{"a" => 0, "b" => 1, "ab" => 2}, [{"a", "b"}],
                 dropout: 0.3
               )
    end

    test "rejects bad keyword params" do
      assert_raise ErlangError, fn ->
        Tokenizers.Model.BPE.init(%{"a" => 0, "b" => 1, "ab" => 2}, [{"a", "b"}],
          weird_value: :something
        )
      end
    end
  end

  describe "Loaded from file" do
    test "Good initialized with valid pathes" do
      assert {:ok, %Tokenizers.Model{}} =
               Tokenizers.Model.BPE.from_file(
                 "test/fixtures/vocab.json",
                 "test/fixtures/merges.txt"
               )
    end

    test "Bad initialized with invalid pathes" do
      assert {:error, _} =
               Tokenizers.Model.BPE.from_file(
                 "test/fixtures/not_found_vocab.json",
                 "test/fixtures/merges.txt"
               )
    end

    test "Bad initialized with good pathes but invalid data" do
      assert {:error, _} =
               Tokenizers.Model.BPE.from_file(
                 "test/fixtures/vocab.txt",
                 "test/fixtures/merges.txt"
               )
    end
  end
end
