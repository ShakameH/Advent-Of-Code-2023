class Game
    attr_reader :id, :bag
    attr_writer :bag

    def initialize(id)
        @id = id
        @bag = Bag.new
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
    attr_reader :sum_id
    attr_writer :sum_id
    def initialize(file_path)
        @file_path = file_path
        @sum_id = 0
    end

    def read_file
        File.foreach(@file_path) do |line|
            is_valid = true
            tirage_array =  line.split(/[:]/)
            game = Game.new(tirage_array[0].split(" ")[1])

            tirage_array[1].split(";").each { |x|
                boule_array = x.split(",").each { |y|
                    number, color = y.split(" ")
                    game.bag.send("#{color}_cube=", game.bag.send("#{color}_cube") + number.to_i)
                }
                if game.bag.red_cube > 12 || game.bag.green_cube > 13 || game.bag.blue_cube > 14
                    is_valid = false
                    break
                end
                game.bag.clear_bag
                if !is_valid
                    break
                end
            }
            if is_valid
                @sum_id += game.id.to_i
            end
        end
        @sum_id
    end
end

file = FileReader.new("./input.txt")
puts file.read_file