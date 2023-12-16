class FileReader
    def initialize(file_path)
        @file_path = file_path
        @part_sum = 0
    end

    def read_file
        content = File.readlines(@file_path).map(&:chomp)
        matrix = content.map { |line| line.chars }
        start_number_index = 0
        end_number_index = 0
        symbol_is_number = false
        number_array = []
        part_sum = 0 
        matrix.each_with_index do |line, index_line|
            line.each_with_index do |char, index_char|
                if char.match(/[[:digit:]]/)
                    number_array = number_array << char
                    if symbol_is_number == false
                        start_number_index = index_char 
                        symbol_is_number = true
                    end
                    if matrix[index_line][index_char+1] != nil && matrix[index_line][index_char+1].match(/[[:digit:]]/)
                        next
                    else
                        end_number_index = index_char
                        symbol_is_number = false
                        result = check_part(matrix, start_number_index, end_number_index, index_line)
                        if result == true
                            part_sum += (number_array * "").to_i  
                        end
                        number_array.clear
                    end
                end
            end
        end
        part_sum
    end

    def check_part(matrix, start_index, end_index, index_line)
        new_start_index = start_index
        new_end_index = end_index
        if start_index - 1 != -1
            new_start_index -= 1
        end
        if end_index + 1 != matrix[index_line].length
            new_end_index += 1
        end
        for index in new_start_index..new_end_index do
            if index == start_index - 1 || index == end_index + 1
                if index_line != 0 && matrix[index_line - 1][index] != "." || 
                    matrix[index_line][index] != "." || 
                    index_line != matrix.length - 1 && matrix[index_line + 1][index] != "."
                    return true
                end
            else
                if index_line != 0 && matrix[index_line - 1][index] != "." || 
                    (index_line != matrix.length - 1 && matrix[index_line + 1][index] != ".")
                    return true  
                end
            end
        end
    end
    false
end

file = FileReader.new('./input.txt')
puts file.read_file