#include "scanner.hpp"


namespace lexer
{
char Scanner::peek_next() const
{
    if (_arquivo_str.size() > _pos)
    {
        char saida = _arquivo_str.at(_pos);
        return saida;
    }
    return 0;
}

char Scanner::get_next()
{
    auto saida = peek_next();
    _pos++;
    if (saida == 0)
    {
        _acabou = true;
    }
    return saida;
}
}