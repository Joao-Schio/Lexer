#pragma once

#include <fstream>
#include <string>
#include <tuple>

namespace lexer
{
class Scanner
{
    private:
        std::ifstream _arquivo;
        bool _acabou;
        std::string _linha;
        size_t _pos;
    public:
        Scanner(std::ifstream&& arquivo) : 
        _arquivo(std::move(arquivo)), 
        _acabou(false),
        _linha(""),
        _pos(0)
        {
            if (!_arquivo)
            {
                throw std::runtime_error("Arquivo foi não pode ser lido");
            }
        }

        std::string get_line();
        char get_next();
        char peek_next() const;
        bool get_termino() const
        {
            return _acabou;
        }
};
}