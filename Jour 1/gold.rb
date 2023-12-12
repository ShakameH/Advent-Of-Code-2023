class String
    def is_integer
        self.to_i.to_s == self
    end
end

class FileReader
    def initialize(file_path)
        @file_path = file_path
        @dictionary = {
            "one" => 1, "two" => 2, "three" => 3, 
            "four" => 4, "five" => 5, "six" => 6, 
            "seven" => 7, "eight" => 8, "nine" => 9, 
        }
        @dictionnary_combination = {}

        @dictionary.each do |keyi, valuei|
            @dictionary.each do |keyj, valuej| 
                if keyi != keyj
                    if keyi[0] == keyj[-1, 1]
                        key_total = keyj.chop + keyi
                        @dictionnary_combination[key_total] = valuej.to_s + valuei.to_s
                    end
                end
            end
        end
        @dictionary = @dictionnary_combination.merge(@dictionary)
        puts @dictionary
    end

    def read_data
        sum = 0

        out_file = File.new("out.txt", "w")
        File.foreach(@file_path) do |line|
            @dictionary.each do |key, value|
                if line.include? key
                    line.gsub!(key.to_s, value.to_s)
                end
            end
            out_file.puts(line)
            first_number = nil
            last_number = nil
            line.each_char do |c|
                if c.is_integer
                    if first_number.nil?
                        first_number = c
                    else
                        last_number = c
                    end
                end
            end
            if first_number && last_number
                sum += (first_number + last_number).to_i
            elsif first_number
                sum += (first_number * 2).to_i
            end
        end
        out_file.close
        sum
    end
end

file = FileReader.new("./input")
sum = file.read_data
puts sum
