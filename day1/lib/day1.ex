defmodule Day1 do
  @moduledoc """
  Documentation for Day1.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Day1.hello()
      :world

  """
  def hello do
    :world
  end
  def read do
    input = File.read!("lib/signal.txt")
    input
    |> String.split("\n")
    |> Enum.map(&(String.to_integer(&1)))
    |> Enum.map(&(div(&1, 3) - 2))
    |> Enum.sum
    |> IO.inspect
  end
end
