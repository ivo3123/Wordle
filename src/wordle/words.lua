FILE_NAME = "src/wordle/five_letter_words"
NUMBER_OF_LETTERS_IN_wORD = 5;


function get_all_words () 
    local words = io.open (FILE_NAME);

    if words == nil then
        error("Could not open the file with the words");
    end

    local my_table = {};

    local ctr = 1;

    while true do 
        curr_word = words:read(NUMBER_OF_LETTERS_IN_wORD);
        my_table[ctr] = curr_word;
        ctr = ctr + 1;
        while true do   
            check = words:read(1);
            if check == "\n" or check == nil then
                break
            end
        end
        if check == nil then
            break;
        end
    end

    words:close();

    return my_table, ctr - 1;
end

math.randomseed(os.time())

function get_random_word ()
    local words, max_words = get_all_words();
    random_num = math.random(1, max_words);
    return words[random_num];
end
