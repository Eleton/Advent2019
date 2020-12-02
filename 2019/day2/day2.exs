defmodule Day2 do
  def parse do
    File.read!("./signal.txt")
    |> String.split(",")
    |> Enum.map(&(String.to_integer(&1)))
  end

  def operate(list, index) do
    [command, index1, index2, index3] = IO.inspect(Enum.at(Enum.chunk_every(list, 4), index))
    
    case command do
      1 ->
        value1 = Enum.at(list, index1)
        value2 = Enum.at(list, index2)
        List.replace_at(list, index3, value1 + value2)
        |> operate(index + 1)
      2 ->
        value1 = Enum.at(list, index1)
        value2 = Enum.at(list, index2)
        List.replace_at(list, index3, value1 * value2)
        |> operate(index + 1)
      _ ->
        list
    end
  end
end

Day2.parse
|> Day2.operate(0)
|> IO.inspect