#include "lexer.hpp"


namespace lexer
{

char Lexer::get_prox_char()
{
    char prox = _scanner.get_next();
    while(std::isspace(prox) && prox != '\n')
    {
        prox = _scanner.get_next();
    }
    if (prox == '/' && _scanner.peek_next() == '/')
    {
        while(_scanner.peek_next() != '\n' && prox != 0)
        {
            prox = _scanner.get_next();
        }
        return get_prox_char();
    }
    return prox;
}

Token Lexer::get_prox_token()
{
    char prox = this -> get_prox_char();
    if (prox == '=')
    {
        return Token { Atribuicao { } };
    }
    else if (prox == '+')
    {
        return Token { Soma { } };
    }
    else if (prox == '-')
    {
        return Token { Subtracao { } };
    }
    else if (prox == '/')
    {
        return Token { Divisao { } };
    }
    else if (prox == '*')
    {
        return Token { Multiplicacao { } };
    }
    else if (prox == ';')
    {
        return Token { Delimitador { } };
    }
    else if (std::isalpha(prox) || prox == '_')
    {
        std::string nome_id;
        nome_id += prox;
        while(std::isalpha(_scanner.peek_next()) || _scanner.peek_next() == '_')
        {
            nome_id += _scanner.get_next();
        }
        return Token { Id { std::move(nome_id) } };
    }
    else if (std::isdigit(prox))
    {
        std::string numeros;
        numeros += prox;
        while(std::isdigit(_scanner.peek_next()))
        {
            numeros += _scanner.get_next();
        }
        if (!eh_separador(_scanner.peek_next()))
        {
            std::string invalido = std::move(numeros);
            while (!eh_separador(_scanner.peek_next()))
            {
                invalido += _scanner.get_next();
            }
            return Token { ErroLexico { std::move(invalido) } };
        }
        return Token { Int { std::stoull(numeros) } };
    }
    else if (prox == 0)
    {
        return Token { Eof { } };
    }
    else if (prox == '\n')
    {
        return Token { QuebraLinha { } };
    }
    return Token { ErroLexico { std::string { prox } } };
}
}