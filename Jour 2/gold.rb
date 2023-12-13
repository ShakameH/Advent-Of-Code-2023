class Game
    attr_reader :id, :bag, :minimal_bag
    attr_writer :bag, :minimal_bag

    def initialize(id)
        @id = id
        @bag = Bag.new
        @minimal_bag = Bag.new
    end
end

class Bag
    attr_reader :red_cube, :green_cube, :blue_cube
    attr_writer :red_cube, :green_cube, :blue_cube
    def initialize()
        @red_cube = 0
        @green_cube = 0
        @blue_cube = 0
    end

    def clear_bag()
        @red_cube = 0
        @green_cube = 0
        @blue_cube = 0
    end
end

class FileReader
    attr_reader :sum_power
    attr_writer :sum_power
    def initialize(file_path)
        @file_path = file_path
        @sum_power = 0
    end

    def read_file
        File.foreach(@file_path) do |line|
            tirage_array =  line.split(/[:]/)
            game = Game.new(tirage_array[0].split(" ")[1])

            tirage_array[1].split(";").each { |x|
                x.split(",").each { |y|
                    number, color = y.split(" ")
                    game.bag.send("#{color}_cube=", game.bag.send("#{color}_cube") + number.to_i)
                }
                if game.bag.red_cube > game.minimal_bag.red_cube
                    game.minimal_bag.red_cube = game.bag.red_cube
                end
                if game.bag.green_cube > game.minimal_bag.green_cube
                    game.minimal_bag.green_cube = game.bag.green_cube
                end
                if game.bag.blue_cube > game.minimal_bag.blue_cube
                    game.minimal_bag.blue_cube = game.bag.blue_cube
                end
                game.bag.clear_bag
            }
        @sum_power += game.minimal_bag.red_cube * game.minimal_bag.green_cube * game.minimal_bag.blue_cube
        end
        @sum_power
    end
end

file = FileReader.new("./input.txt")
puts file.read_file