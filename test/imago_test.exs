defmodule ImagoTest do
  use ExUnit.Case
  doctest Imago

  test "greets the world" do
    assert Imago.hello() == :world
  end
end
