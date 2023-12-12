class String
    def is_integer
        self.to_i.to_s == self
    end
end

class FileReader
    def initialize(file_path)
        @file_path = file_path
    end
    def read_data
        sum = 0
        File.foreach(@file_path) do |line| 
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
                sum += (first_number + first_number).to_i
            end
        end
        sum
    end
end

file = FileReader.new("./input")
sum = file.read_data
puts sum