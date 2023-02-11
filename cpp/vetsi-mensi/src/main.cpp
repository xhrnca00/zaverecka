#include <iostream>
#include <iomanip>
#include <string>
#include <random>

// #define CIN

// CIN implementation is shorter, but it doesn't handle I/O errors (such as stdin not being available in Compiler Explorer)

constexpr int MIN = 1;
constexpr int MAX = 1000;

int randomNumber() {
    std::default_random_engine generator(std::random_device{}());
    return std::uniform_int_distribution<>{ MIN, MAX }(generator);
}

#ifdef CIN
void resetCin() {
    std::cin.clear();
    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
}
#endif

int main() {
    int guessCount = 0;
    //* Generate a random number from 1 to 1000
    const int number = randomNumber();
    // Print the game instructions
    std::cout << "I am thinking a number from " << MIN << " to " << MAX << ".\n";
    std::cout << "Your goal is to guess it. I will tell you if your guess is too high or too low.\n";
    while (true) {
        std::cout << std::setfill('-') << std::setw(20) << "" << std::endl;
        int guess;
#ifdef CIN
        //* getting a guess + error handling
        std::cout << "Enter your guess: ";
        if (!(std::cin >> guess) || std::cin.fail()) {
            resetCin();
            std::cout << "Please enter a whole number between " << MIN << " and " << MAX << ".\n";
            continue;
        }
        resetCin(); // handle the case where the stream is not empty (e.g. "1 2" or "1a")
#else
        std::string input;
        //* getting a guess
        std::cout << "Enter your guess: ";
        if (!std::getline(std::cin, input) || std::cin.fail()) {
            std::cout << "Something went terribly wrong.\n";
            return 1;
        }
        //* error handling + conversion
        try {
            guess = std::stoi(input);
            // stoi can throw two exceptions, both extend std::logic_error, so we just use it as well
            if (guess < MIN || guess > MAX) throw std::logic_error("Out of range"); // not a logic_error, but less typing
        }
        catch (std::logic_error) {
            std::cout << "Please enter a whole number between " << MIN << " and " << MAX << ".\n";
            continue;
        }
#endif
        guessCount++;
        //* comparing the guess to the number
        if (guess == number) {
            std::cout << "You guessed it! The number was " << number << ".\n";
            std::cout << "It took you " << guessCount << " guesses.\n";
            break;
        } else if (guess < number) {
            std::cout << "Too low!\n";
        } else {
            std::cout << "Too high!\n";
        }
    }
    return 0;
}
