#include "scanner.hpp"


namespace lexer
{
std::string Scanner::get_line()
{
    if (_acabou)
    {
        throw std::runtime_error("Arquivo ja terminado");
    }

    std::string linha;

    if (!std::getline(_arquivo, linha))
    {
        _acabou = true;
        throw std::runtime_error("Arquivo ja terminado");
    }
    return linha;
}

char Scanner::peek_next() const
{
    if (_linha.size() > _pos)
    {
        char saida = _linha.at(_pos);
        return saida;
    }
    return 0;
}

char Scanner::get_next()
{
    if (_linha == "")
    {
        _linha = get_line();
    }
    auto saida = peek_next();
    _pos++;
    return saida;
}
}