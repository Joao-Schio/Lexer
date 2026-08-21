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


void print_tokens(const std::vector<lexer::Token> &tokens)
{
    bool primeiro = true;
    for (const auto& i : tokens)
    {
        if (primeiro == true)
        {
            std::print("{}", i);
            primeiro = false;
        }
        else if (std::holds_alternative<lexer::QuebraLinha>(i.get_token()))
        {
            std::print("{}", i);
            primeiro = true;
        }
        else
        {
            std::print(" {}", i);
        }
    }
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
    std::vector<lexer::Token> tokens;
    auto tok = lexer.get_prox_token();
    while(!std::holds_alternative<Eof>(tok.get_token()))
    {
        tokens.push_back(std::move(tok));
        tok = lexer.get_prox_token();
    }
    tokens.push_back(std::move(tok));
    tokens.emplace_back(QuebraLinha { });
    print_tokens(tokens);
}