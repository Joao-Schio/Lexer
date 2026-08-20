#include <iostream>
#include "token.hpp"
#include "lexer.hpp"
#include "scanner.hpp"

lexer::Lexer criar_lexer(std::string&& caminho)
{
    using lexer::Scanner, lexer::Lexer;
    Scanner scanner = Scanner { std::ifstream { caminho } };
    Lexer lexer = Lexer { std::move(scanner) };
    return lexer;
}


int main(int argc, char **argv)
{
    using namespace lexer;
    if (argc == 1)
    {
        std::println("Um arquivo fonte não foi especificado");
        return 1;
    }

    Lexer lexer = criar_lexer(std::move(argv[1]));
    auto token = lexer.get_prox_token();
    while(!std::holds_alternative<Eof>(token.get_token()))
    {
        std::println("{}", token);
        token = lexer.get_prox_token();
    }
    std::println("{}", token);
}