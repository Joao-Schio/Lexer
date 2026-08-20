#pragma once

#include "scanner.hpp"
#include "token.hpp"


namespace lexer
{

class Lexer
{
    private:
        Scanner _scanner;


    public:
        Lexer(Scanner&& scanner) : _scanner(std::move(scanner)) { }

        Token get_prox_token()
        {
            auto linha = _scanner.get_line();
            for(char i : linha)
            {
                if (i == '=')
                {
                    return Token { Atribuicao{ } };
                }
            }

            return Token { ErroLexico {'a'}};
        }
};
}