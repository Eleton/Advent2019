defmodule Day1 do
  def read do
    input = File.read!("lib/signal.txt")
    input
    |> String.split("\n")
    |> Enum.map(&(String.to_integer(&1)))
    |> Enum.map(&(additional(&1) - &1))
    |> Enum.sum
  end
  def additional(fuel) do
    if fuel > 0 do
      fuel + additional(div(fuel, 3) - 2)
    else
      0
    end
  end
end
