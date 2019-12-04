defmodule Day4 do
  def check_ascendency(number) do
    new_number = number
    |> Integer.to_string
    |> String.split("")
    |> Enum.sort
    |> Enum.join("")
    |> String.to_integer
    number == new_number
  end
  def check_unique(number) do
    l = number
    |> Integer.to_string
    |> String.split("", trim: true)
    |> Enum.uniq
    |> length
    l <= 5
  end
end

range = 265275..781584
Stream.scan(range, 0, fn (curr, acc) -> 
  a = Day4.check_ascendency(curr)
  b = Day4.check_unique(curr)
  if a && b do
    acc + 1
  else
    acc
  end
end)
|> Enum.to_list
|> Enum.reverse
|> hd
|> IO.inspect
