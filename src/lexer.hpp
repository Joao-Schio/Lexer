#pragma once

#include "scanner.hpp"
#include "token.hpp"
#include <vector>

namespace lexer
{

class Lexer
{
    private:
        Scanner _scanner;
        size_t _numero_linha;
    private:
        static inline bool eh_separador(char c)
        {
            return std::isspace(static_cast<unsigned char>(c))
                || c == 0
                || c == ';'
                || c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '=';
        }

        char get_prox_char();

    public:
        Lexer(Scanner&& scanner) : _scanner(std::move(scanner)), _numero_linha(1) { }
        Token get_prox_token();

};
}