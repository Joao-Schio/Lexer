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
}